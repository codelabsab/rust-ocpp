use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, id_token::IdTokenType};
use crate::v2_1::enumerations::{
    DisplayMessageStatusEnumType, MessagePriorityEnumType, MessageStateEnumType,
};

/// Contains message details, for a message to be displayed on a Charging Station.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MessageInfoType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The identifier that identifies this message.
    #[validate(length(max = 50))]
    pub id: String,

    /// Required. Priority with which this message should be shown.
    pub priority: MessagePriorityEnumType,

    /// Required. Current state of this message.
    pub state: MessageStateEnumType,

    /// Required. Date and time at which this message was received.
    pub start_timestamp: DateTime<Utc>,

    /// Optional. Date and time at which this message should be removed from the display.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<DateTime<Utc>>,

    /// Optional. Transaction Id for which this message is intended.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub transaction_id: Option<String>,

    /// Optional. Message details for a specific user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Optional. Display style of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<DisplayMessageStatusEnumType>,

    /// Optional. Identification of the token for which this message is intended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdTokenType>,
}
