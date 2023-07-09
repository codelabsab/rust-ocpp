use chrono::DateTime;
use chrono::Utc;

use super::component_type::ComponentType;
use super::message_content_type::MessageContentType;
use crate::enumerations::message_priority_enum_type::MessagePriorityEnumType;
use crate::enumerations::message_state_enum_type::MessageStateEnumType;

/// Contains message details, for a message to be displayed on a Charging Station.
/// MessageInfoType is used by: SetDisplayMessageRequest , NotifyDisplayMessagesRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
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
