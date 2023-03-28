use core::fmt::Write;
use serde::{Deserialize, Serialize};
use crate::{
    Error,
    config::AsArgs
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SmpConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    cores: Option<u64>,
}

impl AsArgs for SmpConfig {

    fn as_args(&self) -> Result<Vec<String>, Error> {
        let mut smpvalue = String::new();
        if let Some(cores) = self.cores {
            write!(&mut smpvalue, "cores={cores}").unwrap();
        }

        if !smpvalue.is_empty() {
            Ok(vec![format!("-smp"), smpvalue])
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
        let config = SmpConfig {
            cores: None
        };
        assert_eq!(config.as_args().unwrap().len(), 0);
        let config = SmpConfig {
            cores: Some(4)
        };
        let binding = config.as_args().unwrap();
        let mut args = binding.iter();
        assert_eq!(args.next(), Some(&"-smp".to_string()));
        assert_eq!(args.next(), Some(&"cores=4".to_string()));
    }
}
