use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use crate::{
    Error,
    config::AsArgs
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceConfig {

    /// Type of device
    r#type: String,

    #[serde(flatten)]
    props: Option<BTreeMap<String, String>>,

}

impl AsArgs for DeviceConfig{

    fn as_args(&self) -> Result<Vec<String>, Error> {
        let mut args = vec![format!("-device")];
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


