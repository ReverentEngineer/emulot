use serde::{Deserialize, Serialize};
use std::process::Command;

mod drive;
pub use drive::DriveConfig;
mod smp;
pub use smp::SmpConfig;
mod machine;
pub use machine::MachineConfig;
mod boot;
pub use boot::BootConfig;
use crate::Error;

/// A trait for interpreting into command args
pub(crate) trait AsArgs {

    /// Format into args
    fn as_args(&self) -> Result<Vec<String>, Error>;

}

impl<T> AsArgs for Option<T> where T: AsArgs {

    fn as_args(&self) -> Result<Vec<String>, Error> {
        match self {
            Some(args) => args.as_args(),
            None => Ok(Vec::new())
        }
    }

}

impl<T> AsArgs for Vec<T> where T: AsArgs {

    fn as_args(&self) -> Result<Vec<String>, Error> {
        Ok(self.into_iter().map(|args| args.as_args()).collect::<Result<Vec<_>, Error>>()?.into_iter().flatten().collect())
    }

}

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
    bios: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    accel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine: Option<MachineConfig>,

    #[serde(default = "default_display")]
    display: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    drive: Option<Vec<DriveConfig>>
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
            drive: None
        }
    }

    pub(crate) fn as_cmd(&self) -> Command {
        let mut command = Command::new(format!("qemu-system-{}", self.arch));

        if let Some(cpu) = &self.cpu {
            command.arg("-cpu").arg(cpu);
        }

        if let Some(accel) = &self.accel {
            command.arg("-accel").arg(accel);
        }

        if let Some(bios) = &self.bios {
            command.arg("-bios").arg(bios);
        }

        command.args(self.machine.as_args().unwrap());
        command.args(self.boot.as_args().unwrap());
        command.args(self.smp.as_args().unwrap());
        command.args(self.drive.as_args().unwrap());

        command.arg("-m").arg(format!("{}", self.memory));
        command.arg("-display").arg(&self.display);
        command
    }
}

fn default_display() -> String {
    format!("none")
}
