use crate::v2_0_1::datatypes::evse_type::EVSEType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::message_trigger_enum_type::MessageTriggerEnumType;
use crate::v2_0_1::enumerations::trigger_message_status_enum_type::TriggerMessageStatusEnumType;

/// This contains the field definition of the TriggerMessageRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessageRequest {
    pub requested_message: MessageTriggerEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
}

/// This contains the field definition of the TriggerMessageResponse PDU sent by the Charging Station to the CSMS in response to TriggerMessageResponse.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessageResponse<'a> {
    pub status: TriggerMessageStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none",borrow)]
    pub status_info: Option<StatusInfoType<'a>>,
}
