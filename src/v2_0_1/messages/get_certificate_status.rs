//! GetCertificateStatus
use crate::v2_0_1::datatypes::ocsp_request_data_type::OCSPRequestDataType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::get_certificate_status_enum_type::GetCertificateStatusEnumType;

/// GetCertificateStatusRequest, sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateStatusRequest<'a> {
    /// Indicates the certificate of which the status isrequested.
    #[serde(borrow)]
    pub ocsp_request_data: OCSPRequestDataType<'a>,
}

/// GetCertificateStatusResponse, sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateStatusResponse<'a> {
    /// This indicates whether the charging stationwas able to retrieve the OCSP certificate status.
    pub status: GetCertificateStatusEnumType,
    /// OCSPResponse class as defined in IETF RFC6960. DER encoded (as defined in IETF RFC 6960), andthen base64 encoded. MAY only be omitted when statusis not Accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp_result: Option<&'a str>,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType<'a>>,
}
