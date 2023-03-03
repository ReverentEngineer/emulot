use core::fmt::Write;
use serde::{Deserialize, Serialize};
use crate::config::Args;
use std::process::Command;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SmpConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    cores: Option<u64>,
}

impl Args for SmpConfig {
    fn fmt_args<'a>(&'a self, cmd: &'a mut Command) -> &mut Command{
        let mut smpvalue = String::new();
        if let Some(cores) = self.cores {
            write!(&mut smpvalue, "cores={cores}").unwrap();
        }
        if !smpvalue.is_empty() {
            cmd.arg("-smp");
            cmd.arg(smpvalue);
        }
        cmd
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn args() {
        let mut cmd = Command::new("");
        SmpConfig {
            cores: None 
        }.fmt_args(&mut cmd);
        assert_eq!(cmd.get_args().count(), 0);
        SmpConfig {
            cores: Some(4) 
        }.fmt_args(&mut cmd);
        let mut args = cmd.get_args();
        assert_eq!(args.next().map(|s| s.to_str()).flatten(), Some("-smp"));
        assert_eq!(args.next().map(|s| s.to_str()).flatten(), Some("cores=4"));
    }
}
