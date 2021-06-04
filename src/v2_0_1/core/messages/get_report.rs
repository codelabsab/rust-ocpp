use crate::v2_0_1::core::{
    datatypes::{component_variable_type::ComponentVariableType, status_info_type::StatusInfoType},
    enumerations::{
        component_criterion_enum_type::ComponentCriterionEnumType,
        generic_device_model_status_enum_type::GenericDeviceModelStatusEnumType,
    },
};

/// This contains the field definition of the GetReportRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetReportRequest {
    pub request_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_criteria: Option<ComponentCriterionEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_variable: Option<ComponentVariableType>,
}

/// This contains the field definition of the GetReportRequest, PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetReportResponse {
    pub status: GenericDeviceModelStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
