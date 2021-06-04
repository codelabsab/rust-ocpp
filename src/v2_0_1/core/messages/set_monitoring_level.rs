use crate::v2_0_1::core::{
    datatypes::status_info_type::StatusInfoType,
    enumerations::generic_status_enum_type::GenericStatusEnumType,
};
/// This contains the field definition of the SetMonitoringLevelRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringLevelRequest {
    pub severity: u8,
}

/// This contains the field definition of the SetMonitoringLevelResponse PDU sent by the Charging Station to the CSMS in response to a SetMonitoringLevelRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringLevelResponse {
    pub status: GenericStatusEnumType,
    pub status_info: Option<StatusInfoType>,
}
