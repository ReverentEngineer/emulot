use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use crate::config::Args;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MachineConfig {
    r#type: String,

    #[serde(flatten)]
    props: Option<HashMap<String, String>>,
}

impl Args for MachineConfig {
   
    fn fmt_args<'a>(&'a self, cmd: &'a mut Command) -> &mut Command {
        cmd.arg("-machine");
        let mut argvalue = format!("{}", self.r#type);
        if let Some(props) = &self.props {
            for (key, value) in props {
                argvalue.push_str(&format!(",{key}={value}"));
            }
        }
        cmd.arg(argvalue);
        cmd
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn args() {
        let mut config = 
            MachineConfig {
                r#type: format!("virt"),
                props: Some(HashMap::new())
            };
        config.props.as_mut().unwrap().insert(format!("highmem"), format!("on"));
        let mut cmd = Command::new("");
        config.fmt_args(&mut cmd);
        assert_eq!(cmd.get_args().count(), 2);
    }
}
