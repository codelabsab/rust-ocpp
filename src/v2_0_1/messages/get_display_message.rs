use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::get_display_messages_status_enum_type::GetDisplayMessagesStatusEnumType;
use crate::v2_0_1::enumerations::message_priority_enum_type::MessagePriorityEnumType;
use crate::v2_0_1::enumerations::message_state_enum_type::MessageStateEnumType;

/// This contains the field definition of the GetCompositeScheduleRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetDisplayMessagesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub request_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<MessagePriorityEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<MessageStateEnumType>,
}

/// This contains the field definition of the GetCompositeScheduleResponse PDU sent by the Charging Station to the CSMS in response to a GetCompositeScheduleRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetDisplayMessagesResponse {
    pub status: GetDisplayMessagesStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
