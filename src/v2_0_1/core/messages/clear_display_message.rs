use crate::v2_0_1::core::{
    datatypes::status_info_type::StatusInfoType,
    enumerations::clear_message_status_enum_type::ClearMessageStatusEnumType,
};

/// This contains the field definition of the ClearDisplayMessageRequest PDU sent by the CSMS to the Charging Station. The CSMS asks the Charging Station to clear a display message that has been configured in the Charging Station to be cleared/removed. See also O05 - Clear a Display Message.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearDisplayMessageRequest {
    pub id: i64,
}

/// This contains the field definition of the ClearDisplayMessageResponse PDU sent by the Charging Station to the CSMS in a response to a ClearDisplayMessageRequest. See also O05 - Clear a Display Message.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearDisplayMessageResponse {
    pub status: ClearMessageStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
