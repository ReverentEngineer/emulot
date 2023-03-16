use std::collections::BTreeMap;
use serde::{
    Serialize,
    Deserialize,
    ser::SerializeMap,
};

/// QMP Command
pub enum Command {
    Capabilities
}

impl Serialize for Command {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut map = serializer.serialize_map(Some(1))?;
            match self {
                Self::Capabilities => {
                    map.serialize_entry("execute", "qmp_capabilities")?;
                }
            }
            map.end()
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct QemuVersion {
    major: usize,
    minor: usize,
    micro: usize,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Version {
    qemu: QemuVersion,
    package: String
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Greeting {
    version: Version,
    capabilities: Vec<String>
}

#[derive(Deserialize)]
pub enum Response {
    #[serde(rename = "QMP")]
    Greeting(Greeting),

    #[serde(rename = "return")]
    Return(BTreeMap<String, String>),

    #[serde(rename = "error")]
    Error(BTreeMap<String, String>)
}

mod send;
pub use send::AsyncSend;
mod receive;
pub use receive::AsyncReceive;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn deserialize_empty_response() {
        let _: Response = serde_json::from_str(r#"{"return": {}}"#).unwrap();
    }
    
    #[test]
    fn deserialize_non_empty_response() {
        let response: Response = serde_json::from_str(r#"{"return": {"key": "value"}}"#).unwrap();
        match response {
            Response::Return(map) => {
                assert_eq!(map.get("key"), Some(&String::from("value")));
            }
            _ => panic!("Unexpect message")
        }
    }
}
