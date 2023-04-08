use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{
    Error,
    config::AsArgs
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MachineConfig {
    r#type: String,

    #[serde(flatten)]
    props: Option<HashMap<String, String>>,
}

impl AsArgs for MachineConfig {
  
    fn as_args(&self) -> Result<Vec<String>, Error> {
        if let Some(props) = &self.props {
            let mut options = props.into_iter()
                .map(|(key, value)| format!("{key}={value}"))
                .collect::<Vec<_>>();
            options.insert(0, self.r#type.clone());
            let options = options.join(",");
            Ok(vec![format!("-machine"), options])
        } else {
            Ok(vec![format!("-machine"), self.r#type.clone()])
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn args() {
        let config = 
            MachineConfig {
                r#type: format!("virt"),
                props: None
            };
        let args = config.as_args().unwrap();
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], "-machine");
        assert_eq!(args[1], "virt");
    }

    #[test]
    fn args_with_props() {
        let mut config = 
            MachineConfig {
                r#type: format!("virt"),
                props: Some(HashMap::new())
            };
        config.props.as_mut().unwrap().insert(format!("highmem"), format!("on"));
        let args = config.as_args().unwrap();
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], "-machine");
        assert_eq!(args[1], "virt,highmem=on");
    }
}
