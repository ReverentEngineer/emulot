use core::fmt::Write;
use serde::{Serialize, Deserialize};
use std::process::Command;
use crate::config::Args;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BootConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<String>
}

impl Args for BootConfig {
    fn fmt_args<'a>(&'a self, command: &'a mut Command) -> &mut Command {
        let mut bootvalue = String::new();

        if let Some(order) = &self.order {
            write!(&mut bootvalue, "order={order}").unwrap();
        }

        if !bootvalue.is_empty() {
            command.arg("-boot");
            command.arg(bootvalue);
        }
        
        command
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn args() {
        let mut cmd = Command::new("");
        BootConfig {
            order: None
        }.fmt_args(&mut cmd);

        assert_eq!(cmd.get_args().count(), 0);
    }

}
