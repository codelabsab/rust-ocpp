use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Contains information about a specific firmware version.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// URL from which the firmware can be downloaded.
    #[validate(length(max = 512))]
    pub location: String,

    /// Date and time at which the firmware shall be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_date: Option<String>,

    /// Date and time at which the firmware shall be installed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_date: Option<String>,

    /// Firmware version.
    #[validate(length(max = 50))]
    pub signature: String,

    /// MD5 checksum over the entire firmware file as a hexadecimal string of length 32.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(equal = 32))]
    pub signing_certificate: Option<String>,
}
