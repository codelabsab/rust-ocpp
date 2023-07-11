use crate::datatypes::status_info_type::StatusInfoType;
use crate::enumerations::reset_enum_type::ResetEnumType;
use crate::enumerations::reset_status_enum_type::ResetStatusEnumType;

/// This contains the field definition of the ResetRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResetRequest {
    #[serde(rename = "type")]
    pub request_type: ResetEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i64>,
}

/// This contains the field definition of the ResetResponse PDU sent by the Charging Station to the CSMS in response to ResetRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResetResponse {
    pub status: ResetStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
