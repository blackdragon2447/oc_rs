use std::io;
use std::sync::Mutex;
use serde_json::{Number, Value};
use crate::device_bus::{BusCall, BusReturn, RPCBus};
use crate::rpc_device::RPCDevice;

pub struct RedstoneInterface {
    pub device: RPCDevice,
    pub bus: Mutex<RPCBus>,
}


impl RedstoneInterface {
    pub fn get_redstone_output(&mut self, side: String) -> io::Result<usize> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "getRedstoneOutput".to_string(),
            parameters: vec![Value::String(side)],
        })?;
        let result: BusReturn<usize> = bus.read()?;
        if let BusReturn::Result(v) = result {
            return Ok(v);
        } else if let BusReturn::Error(e) = result {
            eprintln!("an error occurred calling the method: {}", e);
        }
        Err(io::ErrorKind::InvalidData.into())
    }

    pub fn get_redstone_input(&mut self, side: String) -> io::Result<usize> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "getRedstoneInput".to_string(),
            parameters: vec![Value::String(side)],
        })?;
        let result: BusReturn<usize> = bus.read()?;
        if let BusReturn::Result(v) = result {
            return Ok(v);
        } else if let BusReturn::Error(e) = result {
            eprintln!("an error occurred calling the method: {}", e);
        }
        Err(io::ErrorKind::InvalidData.into())
    }

    pub fn set_redstone_output(&mut self, side: String, power: usize) -> io::Result<()> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "setRedstoneOutput".to_string(),
            parameters: vec![Value::String(side), Value::Number(Number::from(power))],
        })?;
        Ok(())
    }
}
