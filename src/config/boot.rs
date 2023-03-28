use serde::{Serialize, Deserialize};
use crate::{
    Error,
    config::AsArgs
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BootConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<String>
}

impl AsArgs for BootConfig {
    
    fn as_args(&self) -> Result<Vec<String>, Error> {
        if let Some(order) = &self.order {
            Ok(vec![format!("-boot"), format!("order={order}")])
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
        let config = BootConfig {
            order: None
        };
        assert_eq!(config.as_args().unwrap().len(), 0);
    }

}
