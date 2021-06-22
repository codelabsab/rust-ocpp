use chrono::{DateTime, Utc};

use crate::v2_0_1::core::enumerations::{
    message_priority_enum_type::MessagePriorityEnumType,
    message_state_enum_type::MessageStateEnumType,
};

use super::{component_type::ComponentType, message_content_type::MessageContentType};

/// Contains message details, for a message to be displayed on a Charging Station.
/// MessageInfoType is used by: SetDisplayMessageRequest , NotifyDisplayMessagesRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MessageInfoType {
    pub id: i64,
    pub priority: MessagePriorityEnumType,
    pub state: MessageStateEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    pub message: MessageContentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<ComponentType>,
}
