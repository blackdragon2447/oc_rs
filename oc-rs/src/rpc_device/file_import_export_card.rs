use anyhow::anyhow;
use serde_json::Value;

use crate::device_bus::{BusCall, BusReturn, RPCBus};
use crate::rpc_device::RPCDevice;
use crate::util::ImportedFileInfo;

use std::sync::Mutex;

pub struct FileImportExportCard {
    pub device: RPCDevice,
    pub bus: Mutex<RPCBus>,
}

impl FileImportExportCard {
    pub fn begin_export_file(&mut self, name: String) -> anyhow::Result<()> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "beginExportFile".to_string(),
            parameters: vec![Value::String(name)],
        })?;
        Ok(())
    }

    pub fn write_export_file(&mut self, data: Vec<u8>) -> anyhow::Result<()> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "writeExportFile".to_string(),
            parameters: vec![Value::Array(
                data.into_iter().map(|b| Value::Number(b.into())).collect(),
            )],
        })?;
        Ok(())
    }

    pub fn finish_export_file(&mut self) -> anyhow::Result<()> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "finishExportFile".to_string(),
            parameters: vec![],
        })?;
        Ok(())
    }

    pub fn request_import_file(&mut self) -> anyhow::Result<()> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "requestImportFile".to_string(),
            parameters: vec![],
        })?;
        let result: BusReturn<bool> = bus.read_debug()?;
        if let BusReturn::Result(true) = result {
            Ok(())
        } else if let BusReturn::Result(false) = result {
            Err(anyhow!("Request denied"))
        } else if let BusReturn::Error(e) = result {
            eprintln!("an error occurred calling the method: {}", e);
            Err(anyhow!(e))
        } else {
            Err(anyhow!("Invalid Data"))
        }
    }

    pub fn begin_import_file(&mut self) -> anyhow::Result<ImportedFileInfo> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "beginImportFile".to_string(),
            parameters: vec![],
        })?;
        let result: BusReturn<ImportedFileInfo> = bus.read()?;
        if let BusReturn::Result(v) = result {
            Ok(v)
        } else if let BusReturn::Error(e) = result {
            eprintln!("an error occurred calling the method: {}", e);
            Err(anyhow!(e))
        } else {
            Err(anyhow!("Invalid Data"))
        }
    }

    pub fn read_import_file(&mut self) -> anyhow::Result<Vec<u8>> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "readImportFile".to_string(),
            parameters: vec![],
        })?;
        let result: BusReturn<Vec<u8>> = bus.read()?;
        if let BusReturn::Result(v) = result {
            Ok(v)
        } else if let BusReturn::Error(e) = result {
            eprintln!("an error occurred calling the method: {}", e);
            Err(anyhow!(e))
        } else {
            Err(anyhow!("Invalid Data"))
        }
    }

    pub fn reset(&mut self) -> anyhow::Result<()> {
        let mut bus = self.bus.lock().unwrap();
        bus.write(&BusCall::Invoke {
            device_id: self.device,
            method_name: "reset".to_string(),
            parameters: vec![],
        })?;
        Ok(())
    }
}
