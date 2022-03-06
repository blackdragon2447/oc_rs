use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};
use epoll_rs::Epoll;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;

use crate::rpc_device::device_bus::os_stuff::setup_termios;
use crate::rpc_device::{RPCDevice, RPCDeviceDescriptor, RPCDeviceMethod};

#[allow(dead_code)]
#[derive(Debug)]
pub struct RPCBus {
    file: File,
    poll: Epoll,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "data")]
pub enum BusCall {
    List,
    Methods(RPCDevice),
    #[serde(rename_all = "camelCase")]
    Invoke {
        device_id: RPCDevice,
        // hyphenated
        #[serde(rename = "name")]
        method_name: String,
        parameters: Vec<serde_json::Value>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "data")]
pub enum BusReturn<T> {
    List(Vec<RPCDeviceDescriptor>),
    Methods(Vec<RPCDeviceMethod>),
    Error(String),
    Result(#[serde(default)] T), // returned values
}

pub const DELIMITER: &[u8] = b"\0";

impl RPCBus {
    pub fn init(path: &str) -> io::Result<Self> {
        let poll = Epoll::new()?;
        let file: File = poll.add(
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path)?,
            epoll_rs::Opts::IN,
        )?.into_file();

        unsafe { setup_termios(&file) }?;

        Ok(Self {
            poll,
            file,
        })
    }

    pub fn list(&mut self) -> io::Result<Vec<RPCDeviceDescriptor>> {
        self.write(&BusCall::List)?;

        let list: BusReturn<bool> = self.read()?;
        if let BusReturn::List(devices) = list {
            Ok(devices)
        } else {
            Err(io::ErrorKind::InvalidData.into())
        }
    }

    pub fn methods(&mut self, device: RPCDevice) -> io::Result<Vec<RPCDeviceMethod>> {
        self.write(&BusCall::Methods(device))?;
        let list: BusReturn<bool> = self.read()?;
        if let BusReturn::Methods(methods) = list {
            Ok(methods)
        } else { Err(io::ErrorKind::InvalidData.into()) }
    }

    pub fn find(&mut self, name: &str) -> io::Result<RPCDevice> {
        for RPCDeviceDescriptor { device_id, components } in self.list()? {
            if components.into_iter().any(|dev| name == dev) { return Ok(device_id); }
        }
        Err(io::ErrorKind::NotFound.into())
    }

    pub fn write<D: Serialize>(&mut self, data: &D) -> io::Result<()> {
        self.file.write_all(DELIMITER)?;
        serde_json::to_writer(&self.file, data).map_err::<io::Error, _>(|_| io::ErrorKind::InvalidData.into())?;
        self.file.write_all(DELIMITER)?;

        self.file.flush()?;

        Ok(())
    }

    pub fn read<D: DeserializeOwned>(&mut self) -> io::Result<D> {
        self.poll.wait_one()?;

        let mut delim_buf = [0; DELIMITER.len()];
        let read = self.file.read(&mut delim_buf)?;
        if read != DELIMITER.len() || delim_buf != DELIMITER {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        let mut deserializer = serde_json::Deserializer::from_reader(&mut self.file);
        //currently errors (missing field data, l1 c17)
        let data = D::deserialize(&mut deserializer)?;

        let mut delim_buf = [0; DELIMITER.len()];
        let read = self.file.read(&mut delim_buf)?;
        if read != DELIMITER.len() || delim_buf != DELIMITER {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        Ok(data)
    }

    pub fn read_test<D: DeserializeOwned>(&mut self) -> io::Result<()> {
        self.poll.wait_one()?;

        let mut delim_buf = [0; DELIMITER.len()];
        let read = self.file.read(&mut delim_buf)?;
        if read != DELIMITER.len() || delim_buf != DELIMITER {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        // let mut deserializer = serde_json::Deserializer::from_reader(&mut self.file);
        // //currently errors (missing field data, l1 c17)
        // let result = D::deserialize(&mut deserializer);
        //
        // if let Err(e) = result {
        //     eprintln!("reading file failed");
        // }

        let mut delim_buf = [128; DELIMITER.len()];

        while delim_buf != DELIMITER {
            let read = self.file.read(&mut delim_buf)?;
            print!("{}", String::from_utf8_lossy(&delim_buf))
        }

        if read != DELIMITER.len() || delim_buf != DELIMITER {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        Ok(())
    }
}

mod os_stuff {
    use std::fs::File;
    use std::os::raw::{c_int, c_uchar, c_uint};
    use std::os::unix::io::AsRawFd;
    use std::{io, mem};

    #[link(name = "c")]
    extern "C" {
        fn tcgetattr(fd: c_int, termios_p: *mut termios) -> c_int;
        fn cfmakeraw(termios_p: *mut termios);
        fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const termios) -> c_int;
    }

    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    pub(crate) struct termios {
        pub c_iflag: c_uint,
        pub c_oflag: c_uint,
        pub c_cflag: c_uint,
        pub c_lflag: c_uint,
        c_line: c_uchar,
        pub c_cc: [c_uchar; 32],
        c_ispeed: c_uint,
        c_ospeed: c_uint,
    }

    pub(crate) unsafe fn setup_termios(termios: &File) -> Result<(), io::Error> {
        let raw_fd: c_int = termios.as_raw_fd();

        #[allow(clippy::uninit_assumed_init)]
            let mut termios: termios = mem::MaybeUninit::uninit().assume_init();

        match tcgetattr(raw_fd, &mut termios) {
            0 => (),
            _ => { return Err(io::Error::last_os_error()); }
        }

        cfmakeraw(&mut termios);
        termios.c_lflag &= !0o000010;
        tcsetattr(raw_fd, 0, &termios);

        Ok(())
    }
}
