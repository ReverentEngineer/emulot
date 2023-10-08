use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{
    Error,
    config::AsArgs
};

fn local() -> String {
    "local".to_string()
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SecurityModel {
    Passthrough,
    Mapped,
    MappedXattr,
    MappedFile,
    None
}

impl core::fmt::Display for SecurityModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.serialize(f)
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FilesystemDeviceConfig {

    /// The fsdev driver
     #[serde(default = "local")]
    driver: String,
    
    /// Path to local directory to mount
    path: String,
    
    /// The tag name to be used by guest to mount
    mount_tag: String,

    /// The security model of the mount
    security_model: SecurityModel,

    /// Optional passthrough options
    #[serde(flatten)]
    options: Option<HashMap<String, String>>,

}

impl AsArgs for FilesystemDeviceConfig {
  
    fn as_args(&self) -> Result<Vec<String>, Error> {
        let mut args = Vec::new();
        if let Some(options) = &self.options {
            args.extend(
                options.into_iter()
                .map(|(key, value)| format!("{key}={value}"))
            );
        }
        args.insert(0, self.driver.clone());
        args.insert(1, format!("path={}", self.path.clone()));
        args.insert(2, format!("mount_tag={}", self.mount_tag.clone()));
        args.insert(3, format!("security_model={}", self.security_model));
        Ok(vec![format!("-fsdev"), args.join(",")])
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn args() {
        let config = 
            FilesystemDeviceConfig {
                driver: format!("local"),
                path: format!("/path/to/file"),
                mount_tag: "vol0".to_string(),
                security_model: SecurityModel::None,
                options: None
            };
        let args = config.as_args().unwrap();
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], "-fsdev");
        assert_eq!(args[1], "local,path=/path/to/file,mount_tag=vol0,security_model=none");
    }

    #[test]
    fn args_with_options() {
        let mut config = 
            FilesystemDeviceConfig {
                driver: format!("local"),
                path: format!("/path/to/file"),
                mount_tag: "vol0".to_string(),
                security_model: SecurityModel::None,
                options: Some(HashMap::new())
            };
        config.options.as_mut().unwrap().insert(format!("id"), format!("vol0"));
        let args = config.as_args().unwrap();
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], "-fsdev");
        assert_eq!(args[1], "local,path=/path/to/file,mount_tag=vol0,security_model=none,id=vol0");
    }
}
