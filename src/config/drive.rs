//! The drive configuration
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
