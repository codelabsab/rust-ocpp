use crate::v2_0_1::datatypes::message_info_type::MessageInfoType;

/// This contains the field definition of the NotifyDisplayMessagesRequest PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDisplayMessagesRequest {
    pub request_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_info: Option<MessageInfoType>,
}

/// The NotifyDisplayMessagesResponse message is sent by the CSMS to the Charging Station in response to a NotifyDisplayMessagesRequest. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDisplayMessagesResponse {}
