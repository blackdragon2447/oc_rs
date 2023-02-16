use std::io;
use std::sync::Mutex;
use serde_json::Value;
use crate::device_bus::{BusCall, BusReturn, RPCBus};
use crate::rpc_device::RPCDevice;

pub struct SoundCard {
    pub device: RPCDevice,
    pub bus: Mutex<RPCBus>,
}

impl SoundCard {
    pub fn find_sound(&mut self, sound: String) -> io::Result<Vec<String>> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "findSound".to_string(),
            parameters: vec![Value::String(sound)],
        })?;
        let result: BusReturn<Vec<String>> = bus.read()?;
        if let BusReturn::Result(v) = result {
            return Ok(v);
        } else if let BusReturn::Error(e) = result {
            eprintln!("an error occurred calling the method: {}", e);
        }
        Err(io::ErrorKind::InvalidData.into())
    }

    pub fn play_sound (&mut self, sound: String) -> io::Result<()> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "playSound".to_string(),
            parameters: vec![Value::String(sound)],
        })?;
        Ok(())
    }
}
