//! GetBaseReport

use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::generic_device_model_status_enum_type::GenericDeviceModelStatusEnumType;
use crate::v2_0_1::enumerations::report_base_enum_type::ReportBaseEnumType;

/// GetBaseReportRequest, sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBaseReportRequest {
    /// The Id of the request.
    pub request_id: i64,
    /// This field specifies the report base.
    pub report_base: ReportBaseEnumType,
}

/// Response to a DeleteCertificateRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBaseReportResponse {
    /// This indicates whether the Charging Station is able to accept this request.
    pub status: GenericDeviceModelStatusEnumType,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
