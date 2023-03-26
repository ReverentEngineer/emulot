//! The drive configuration
use crate::config::Args;
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
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
    format: Option<String>
}

impl Args for DriveConfig {
    fn fmt_args<'a>(&'a self, command: &'a mut std::process::Command) -> &mut std::process::Command {

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
            command.arg("-drive");
            command.arg(options.join(","));
        }

        command
    }
}

