use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Log parameters for GetLog request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LogParametersType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The Id of this request.
    #[validate(length(max = 36))]
    pub remote_location: String,

    /// Required. The oldest log entry date/time to include in the response.
    pub oldest_timestamp: String,

    /// Optional. The latest log entry date/time to include in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_timestamp: Option<String>,
}
