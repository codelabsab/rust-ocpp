use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::generic_device_model_status_enum_type::GenericDeviceModelStatusEnumType;
use crate::v2_0_1::enumerations::monitoring_base_enum_type::MonitoringBaseEnumType;

/// This contains the field definition of the SetMonitoringLevelRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringBaseRequest {
    pub monitoring_base: MonitoringBaseEnumType,
}

/// This contains the field definition of the SetMonitoringBaseResponse PDU sent by the Charging Station to the CSMS in response to a SetMonitoringBaseRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringBaseResponse<'a> {
    pub status: GenericDeviceModelStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none",borrow)]
    pub status_info: Option<StatusInfoType<'a>>,
}
