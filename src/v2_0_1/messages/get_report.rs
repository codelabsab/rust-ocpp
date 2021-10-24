use crate::v2_0_1::datatypes::component_variable_type::ComponentVariableType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::component_criterion_enum_type::ComponentCriterionEnumType;
use crate::v2_0_1::enumerations::generic_device_model_status_enum_type::GenericDeviceModelStatusEnumType;

/// This contains the field definition of the GetReportRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetReportRequest {
    pub request_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_criteria: Option<ComponentCriterionEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_variable: Option<ComponentVariableType>,
}

/// This contains the field definition of the GetReportRequest, PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetReportResponse {
    pub status: GenericDeviceModelStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
