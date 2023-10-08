//! The drive configuration
use crate::{
    config::AsArgs,
    Error
};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DriveConfig {

    #[serde(flatten)]
    options: HashMap<String, String>,
}

impl AsArgs for DriveConfig {

    fn as_args(&self) -> Result<Vec<String>, Error> {

        let mut args = Vec::new();
        args.extend(
            self.options.iter()
            .map(|(key, value)| format!("{key}={value}"))
        );

        Ok(vec![format!("-drive"), args.join(",")])
    }
}

