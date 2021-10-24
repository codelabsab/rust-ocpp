use crate::v2_0_1::core::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::core::enumerations::certificate_action_enum_type::CertificateActionEnumType;
use crate::v2_0_1::core::enumerations::iso15118ev_certificate_status_enum_type::Iso15118EVCertificateStatusEnumType;

/// This message is sent by the Charging Station to the CSMS if an ISO 15118 vehicle selects the service Certificate installation. NOTE:
/// This message is based on CertificateInstallationReq Res from ISO 15118 2.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Get15118EVCertificateRequest {
    #[serde(rename = "iso15118SchemaVersion")]
    pub iso_15118_schema_version: String,
    pub action: CertificateActionEnumType,
    pub exit_request: String,
}

/// Response message from CSMS to Charging Station containing the status and optionally new certificate. NOTE: This message is based on CertificateInstallationReq Res from ISO 15118-2.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Get15118EVCertificateResponse {
    pub status: Iso15118EVCertificateStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_response: Option<String>,
    pub status_info: StatusInfoType,
}
