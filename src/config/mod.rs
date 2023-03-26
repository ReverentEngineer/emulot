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

/// A trait for interpreting into command args
pub(crate) trait Args {

    /// Format into args
    fn fmt_args<'a>(&'a self, command: &'a mut Command) -> &mut Command;

}

impl<T> Args for Option<T> where T: Args {

    fn fmt_args<'a>(&'a self, cmd: &'a mut Command) -> &mut Command {
        match self {
            Some(args) => args.fmt_args(cmd),
            None => cmd
        }
    }

}

impl<T> Args for Vec<T> where T: Args {

    fn fmt_args<'a>(&'a self, cmd: &'a mut Command) -> &mut Command {
        for arg in self {
            arg.fmt_args(cmd);
        }
        cmd
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

        self.machine.fmt_args(&mut command);
        self.boot.fmt_args(&mut command);
        self.smp.fmt_args(&mut command);
        self.drive.fmt_args(&mut command);

        command.arg("-m").arg(format!("{}", self.memory));
        command.arg("-display").arg(&self.display);
        command
    }
}

fn default_display() -> String {
    format!("none")
}
