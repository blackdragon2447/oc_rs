use std::io;
use std::sync::Mutex;
use serde_json::{Number, Value};
use crate::device_bus::{BusCall, BusReturn, RPCBus};
use crate::rpc_device::RPCDevice;
use crate::util::Item;

pub struct Computer {
    pub device: RPCDevice,
    pub bus: Mutex<RPCBus>,
}

impl Computer {
    pub fn get_item_slot_count(&mut self) -> io::Result<usize> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "getItemSlotCount".to_string(),
            parameters: vec![],
        })?;
        let result: BusReturn<usize> = bus.read()?;
        if let BusReturn::Result(v) = result {
            return Ok(v);
        } else if let BusReturn::Error(e) = result {
            eprintln!("an error occurred calling the method: {}", e);
        }
        Err(io::ErrorKind::InvalidData.into())
    }


    pub fn get_item_stack_in_slot(&mut self, slot: usize) -> io::Result<Item> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "getItemStackInSlot".to_string(),
            parameters: vec![Value::Number(Number::from(slot))],
        })?;
        let result: BusReturn<Item> = bus.read()?;
        if let BusReturn::Result(v) = result {
            return Ok(v);
        } else if let BusReturn::Error(e) = result {
            eprintln!("an error occurred calling the method: {}", e);
        }
        Err(io::ErrorKind::InvalidData.into())
    }

    pub fn get_item_slot_limit(&mut self, slot: usize) -> io::Result<usize> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "getItemSlotLimit".to_string(),
            parameters: vec![Value::Number(Number::from(slot))],
        })?;
        let result: BusReturn<usize> = bus.read()?;
        if let BusReturn::Result(v) = result {
            return Ok(v);
        } else if let BusReturn::Error(e) = result {
            eprintln!("an error occurred calling the method: {}", e);
        }
        Err(io::ErrorKind::InvalidData.into())
    }

    pub fn get_energy_stored(&mut self) -> io::Result<usize> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "getEnergyStored".to_string(),
            parameters: vec![],
        })?;
        let result: BusReturn<usize> = bus.read()?;
        if let BusReturn::Result(v) = result {
            return Ok(v);
        } else if let BusReturn::Error(e) = result {
            eprintln!("an error occurred calling the method: {}", e);
        }
        Err(io::ErrorKind::InvalidData.into())
    }

    pub fn get_max_energy_stored(&mut self) -> io::Result<usize> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "getMaxEnergyStored".to_string(),
            parameters: vec![],
        })?;
        let result: BusReturn<usize> = bus.read()?;
        if let BusReturn::Result(v) = result {
            return Ok(v);
        } else if let BusReturn::Error(e) = result {
            eprintln!("an error occurred calling the method: {}", e);
        }
        Err(io::ErrorKind::InvalidData.into())
    }

    pub fn can_extract_energy(&mut self) -> io::Result<bool> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "canExtractEnergy".to_string(),
            parameters: vec![],
        })?;
        let result: BusReturn<bool> = bus.read()?;
        if let BusReturn::Result(v) = result {
            return Ok(v);
        } else if let BusReturn::Error(e) = result {
            eprintln!("an error occurred calling the method: {}", e);
        }
        Err(io::ErrorKind::InvalidData.into())
    }
    pub fn can_receive_energy(&mut self) -> io::Result<bool> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "canReceiveEnergy".to_string(),
            parameters: vec![],
        })?;
        let result: BusReturn<bool> = bus.read()?;
        if let BusReturn::Result(v) = result {
            return Ok(v);
        } else if let BusReturn::Error(e) = result {
            eprintln!("an error occurred calling the method: {}", e);
        }
        Err(io::ErrorKind::InvalidData.into())
    }


}
