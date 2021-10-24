use crate::v2_0_1::datatypes::certificate_hash_data_type::CertificateHashDataType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::delete_certificate_status_enum_type::DeleteCertificateStatusEnumType;

/// Used by the CSMS to request deletion of an installed certificate on a Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificateRequest {
    pub certificate_hash_data: CertificateHashDataType,
}

/// Response to a DeleteCertificateRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificateResponse {
    pub status: DeleteCertificateStatusEnumType,
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
