//! DeleteCertificate
use crate::v2_0_1::datatypes::certificate_hash_data_type::CertificateHashDataType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::delete_certificate_status_enum_type::DeleteCertificateStatusEnumType;

/// Used by the CSMS to request deletion of an installed certificate on a Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificateRequest {
    /// Indicates the certificate of which deletion isrequested.
    pub certificate_hash_data: CertificateHashDataType,
}

/// Response to a DeleteCertificateRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificateResponse {
    /// Charging Station indicates if it can process the request.
    pub status: DeleteCertificateStatusEnumType,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
