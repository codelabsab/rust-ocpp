use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Time interval relative to a fixed point in time defined in the message that contains this type.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RelativeTimeIntervalType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Start of the interval, in seconds from NOW.
    pub start: i32,

    /// Required. Duration of the interval, in seconds.
    pub duration: i32,
}
