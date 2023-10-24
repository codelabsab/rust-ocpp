use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::unlock_status_enum_type::UnlockStatusEnumType;

/// This contains the field definition of the UnlockConnectorRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnectorRequest {
    pub evse_id: i32,
    pub connector_id: i32,
}

/// This contains the field definition of the UnlockConnectorResponse PDU sent by the Charging Station to the CSMS in response to an UnlockConnectorRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnectorResponse {
    pub status: UnlockStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
