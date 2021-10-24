use crate::v2_0_1::core::datatypes::evse_type::EVSEType;
use crate::v2_0_1::core::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::core::enumerations::message_trigger_enum_type::MessageTriggerEnumType;
use crate::v2_0_1::core::enumerations::trigger_message_status_enum_type::TriggerMessageStatusEnumType;

/// This contains the field definition of the TriggerMessageRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessageRequest {
    pub requested_message: MessageTriggerEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
}

/// This contains the field definition of the TriggerMessageResponse PDU sent by the Charging Station to the CSMS in response to TriggerMessageResponse.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessageResponse {
    pub status: TriggerMessageStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
