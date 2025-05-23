use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{CustomDataType, StreamDataElementType};

/// Request to notify the CSMS about periodic event stream data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyPeriodicEventStreamRequest {
    /// Required. Base timestamp to add to time offset of values.
    pub basetime: DateTime<Utc>,

    /// Required. Array of stream data elements.
    #[validate(length(min = 1))]
    pub data: Vec<StreamDataElementType>,

    /// Required. Id of stream.
    #[validate(range(min = 0))]
    pub id: i32,

    /// Required. Number of data elements still pending to be sent.
    #[validate(range(min = 0))]
    pub pending: i32,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a NotifyPeriodicEventStreamRequest. This message has no fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyPeriodicEventStreamResponse {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
