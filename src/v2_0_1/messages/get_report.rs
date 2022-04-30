//! GetReport
use crate::v2_0_1::datatypes::component_variable_type::ComponentVariableType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::component_criterion_enum_type::ComponentCriterionEnumType;
use crate::v2_0_1::enumerations::generic_device_model_status_enum_type::GenericDeviceModelStatusEnumType;

/// GetReportRequest, sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetReportRequest {
    /// The Id of the request.
    pub request_id: i64,
    /// This field contains criteria for components forwhich a report is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_criteria: Option<ComponentCriterionEnumType>,
    /// This field specifies the components andvariables for which a report is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_variable: Option<ComponentVariableType>,
}

/// GetReportRequest, sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetReportResponse {
    /// This field indicates whether the ChargingStation was able to accept the request.
    pub status: GenericDeviceModelStatusEnumType,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
