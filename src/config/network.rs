use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use crate::{
    Error,
    config::AsArgs
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NetworkDeviceConfig {

    /// Type of device
    r#type: String,

    #[serde(flatten)]
    props: Option<BTreeMap<String, String>>,

}

impl AsArgs for NetworkDeviceConfig{

    fn as_args(&self) -> Result<Vec<String>, Error> {
        let mut args = vec![format!("-netdev")];
        if let Some(ref props) = self.props {
            if !props.is_empty() {
                let options = props.iter().map(|(k, v)| format!("{k}={v}")).
                    collect::<Vec<_>>().join(",");
                args.push(format!("{},{options}", self.r#type.clone()));
            } else {
                args.push(self.r#type.clone()); 
            }
        } else {
            args.push(self.r#type.clone()); 
        }
        Ok(args)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn as_args_no_props() {
        let device = NetworkDeviceConfig {
            r#type: "user".to_string(),
            props: Some(BTreeMap::new())
        };
        assert_eq!(device.as_args().unwrap(), vec![format!("-netdev"), format!("user")]);
    }
    
    #[test]
    fn as_args_with_props() {
        let mut props = BTreeMap::new();
        props.insert(format!("id"), format!("n1"));
        props.insert(format!("net"), format!("192.168.0.1/24"));
        let device = NetworkDeviceConfig {
            r#type: "user".to_string(),
            props: Some(props)
        };
        assert_eq!(device.as_args().unwrap(), vec![format!("-netdev"), format!("user,id=n1,net=192.168.0.1/24")]);
    }
}
