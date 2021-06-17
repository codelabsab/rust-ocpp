use crate::v2_0_1::core::{
    datatypes::status_info_type::StatusInfoType,
    enumerations::{
        generic_device_model_status_enum_type::GenericDeviceModelStatusEnumType,
        report_base_enum_type::ReportBaseEnumType,
    },
};

/// This contains the field definition of the GetBaseReportRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetBaseReportRequest {
    pub request_id: i64,
    pub report_base: ReportBaseEnumType,
}

/// Response to a DeleteCertificateRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetBaseReportResponse {
    pub status: GenericDeviceModelStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
