//! The drive configuration
use crate::{
    config::AsArgs,
    Error
};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DriveConfig {

    /// The interface of the drive
    #[serde(skip_serializing_if = "Option::is_none")]
    r#if: Option<String>,

    /// The file backing the drive
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,

    /// The format of the file backing the drive
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,

    /// Media format (e.g. cdrom)
    #[serde(skip_serializing_if = "Option::is_none")]
    media: Option<String>
}

impl AsArgs for DriveConfig {

    fn as_args(&self) -> Result<Vec<String>, Error> {

        let mut options = Vec::new();

        if let Some(r#if) = &self.r#if {
            options.push(format!("if={}", r#if));
        }

        if let Some(file) = &self.file {
            options.push(format!("file={}", file));
        }

        if let Some(file) = &self.file {
            options.push(format!("file={}", file));
        }


        if !options.is_empty() {
            Ok(vec![String::from("-drive"), options.join(",")])
        } else {
            Ok(Vec::new())
        }
    }
}

