use super::custom_data::CustomDataType;
use serde::{Deserialize, Serialize};

/// Element providing more information about the status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusInfoType {
    /// A predefined code for the reason why the status is returned in this response.
    /// The string is case-insensitive.
    pub reason_code: String,

    /// Additional text to provide detailed information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
