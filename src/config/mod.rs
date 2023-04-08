use serde::{Deserialize, Serialize};
use std::{
    path::Path,
    process::Command
};


mod network;
pub use network::NetworkDeviceConfig;
mod drive;
pub use drive::DriveConfig;
mod smp;
pub use smp::SmpConfig;
mod machine;
pub use machine::MachineConfig;
mod boot;
pub use boot::BootConfig;
pub(crate) use crate::args::AsArgs;
pub(crate) use crate::file::File;
use crate::Error;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GuestConfig {
    arch: String,
    memory: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    boot: Option<BootConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    smp: Option<SmpConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bios: Option<File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    accel: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    machine: Option<MachineConfig>,

    #[serde(default = "default_display")]
    display: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    drive: Option<Vec<DriveConfig>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    netdev: Option<Vec<NetworkDeviceConfig>>
}

impl GuestConfig {
    
    #[allow(dead_code)]
    pub fn new(arch: String, memory: u64) -> Self {
        Self {
            arch,
            memory,
            boot: None,
            cpu: None,
            smp: None,
            bios: None,
            accel: None,
            machine: None,
            display: "none".to_string(),
            drive: None,
            netdev: None
        }
    }

    pub(crate) fn as_cmd<P: AsRef<Path>>(&self, local_storage: P) -> Result<Command, Error> {
        let mut command = Command::new(format!("qemu-system-{}", self.arch));

        if let Some(cpu) = &self.cpu {
            command.arg("-cpu").arg(cpu);
        }

        if let Some(accel) = &self.accel {
            command.arg("-accel").arg(accel);
        }

        if let Some(bios) = &self.bios {
            command.arg("-bios").arg(bios.path(local_storage)?);
        }

        command.args(self.machine.as_args().unwrap());
        command.args(self.boot.as_args().unwrap());
        command.args(self.smp.as_args().unwrap());
        command.args(self.drive.as_args().unwrap());

        command.arg("-m").arg(format!("{}", self.memory));
        command.arg("-display").arg(&self.display);
        Ok(command)
    }
}

fn default_display() -> String {
    format!("none")
}
