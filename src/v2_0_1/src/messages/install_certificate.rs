//! InstallCertificate

use crate::datatypes::status_info_type::StatusInfoType;
use crate::enumerations::install_certificate_status_enum_type::InstallCertificateStatusEnumType;
use crate::enumerations::install_certificate_use_enum_type::InstallCertificateUseEnumType;

use validator::Validate;

/// Used by the CSMS to request installation of a certificate on a Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificateRequest {
    /// Indicates the certificate type that is sent.
    pub certificate_type: InstallCertificateUseEnumType,
    /// A PEM encoded X.509 certificate.
    #[validate(length(min = 0, max = 5500))]
    pub certificate: String,
}

/// The response to a InstallCertificateRequest, sent by the Charging Station to the CSMS
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificateResponse {
    /// Charging Station indicates if installation wassuccessful.
    pub status: InstallCertificateStatusEnumType,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
