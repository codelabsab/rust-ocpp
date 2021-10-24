use crate::v2_0_1::core::datatypes::component_variable_type::ComponentVariableType;
use crate::v2_0_1::core::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::core::enumerations::generic_device_model_status_enum_type::GenericDeviceModelStatusEnumType;
use crate::v2_0_1::core::enumerations::monitoring_criterion_enum_type::MonitoringCriterionEnumType;

/// This contains the field definition of the GetMonitoringReportRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetMonitoringReportRequest {
    pub request_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_criteria: Option<MonitoringCriterionEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_variable: Option<ComponentVariableType>,
}

/// This contains the field definition of the GetMonitoringReportResponse PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetMonitoringReportResponse {
    pub status: GenericDeviceModelStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
