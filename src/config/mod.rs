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
    accel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine: Option<MachineConfig>,

    #[serde(default = "default_display")]
    display: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    drive: Option<Vec<DriveConfig>>
}

impl GuestConfig {
    
    pub fn new(arch: String, memory: u64) -> Self {
        Self {
            arch,
            memory,
            boot: None,
            cpu: None,
            smp: None,
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

        self.machine.fmt_args(&mut command);
        self.boot.fmt_args(&mut command);
        self.smp.fmt_args(&mut command);

        command.arg("-m").arg(format!("{}", self.memory));
        command.arg("-display").arg(&self.display);
        command.args(["-chardev", "stdio,id=mon0", "-mon", "chardev=mon0,mode=control"]);
        command
    }
}

fn default_display() -> String {
    format!("none")
}
