use crate::v2_0_1::core::{
    datatypes::{ocsp_request_data_type::OCSPRequestDataType, status_info_type::StatusInfoType},
    enumerations::get_certificate_status_enum_type::GetCertificateStatusEnumType,
};

/// This contains the field definition of the GetCertificateStatusRequest PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateStatusRequest {
    pub ocsp_request_data: OCSPRequestDataType,
}

/// This contains the field definition of the GetCertificateStatusResponse PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateStatusResponse {
    pub status: GetCertificateStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
