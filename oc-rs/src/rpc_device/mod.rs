pub mod device_bus;
#[cfg(feature = "redstone_interface")]
pub mod redstone_interface;
#[cfg(feature = "sound_card")]
pub mod sound_card;
#[cfg(feature = "computer")]
pub mod computer;
#[cfg(feature = "file_transfer")]
pub mod file_import_export_card;

#[cfg(feature = "redstone_interface")]
pub use redstone_interface::RedstoneInterface;
#[cfg(feature = "sound_card")]
pub use sound_card::SoundCard;
#[cfg(feature = "computer")]
pub use computer::Computer;
#[cfg(feature = "file_transfer")]
pub use file_import_export_card::FileImportExportCard;

#[cfg(feature = "pretty_print")]
use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub type RPCDevice = Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RPCDeviceDescriptor {
    pub device_id: RPCDevice,
    #[serde(rename = "typeNames")]
    pub components: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RPCDeviceMethod {
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<RPCParamType>,
    pub return_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value_description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RPCParamType {
    #[serde(rename = "type")]
    data: String,
}

#[cfg(feature = "pretty_print")]
impl Display for RPCDeviceDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ID: {}", self.device_id)?;
        writeln!(f, "Components: ")?;
        for c in &self.components {
            writeln!(f, "\t {}", c)?;
        }
        Ok(())
    }
}

#[cfg(feature = "pretty_print")]
impl Display for RPCDeviceMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn {}(", self.name)?;
        let mut params = self.parameters.iter().peekable();
        while let Some(p) = params.next() {
            if params.peek().is_some() {
                write!(f, "{}, ", p.data)?;
            } else {
                write!(f, "{}", p.data)?;
            }
        }
        writeln!(f, ") -> {}", self.return_type)?;

        writeln!(f)?;
        writeln!(f, "\tDescription: {}", self.description.clone().unwrap_or_else(|| "no description provided".to_string()))?;
        writeln!(f)?;
        writeln!(f, "\tReturn Type({}): {}", self.return_type, self.return_value_description.clone().unwrap_or_else(|| "no description provided".to_string()))?;

        Ok(())
    }
}
