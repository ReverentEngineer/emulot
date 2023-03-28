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
            let options = props.into_iter()
                .map(|(key, value)| format!("{key}={value}"))
                .collect::<Vec<_>>()
                .join(",");
            Ok(vec![format!("-machine"), options])
        } else {
            Ok(Vec::new())
        }
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
        assert_eq!(config.as_args().unwrap().len(), 2);
    }
}
