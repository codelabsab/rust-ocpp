use crate::v2_0_1::core::{
    datatypes::status_info_type::StatusInfoType,
    enumerations::{
        install_certificate_status_enum_type::InstallCertificateStatusEnumType,
        install_certificate_use_enum_type::InstallCertificateUseEnumType,
    },
};

/// Used by the CSMS to request installation of a certificate on a Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificateRequest {
    pub certificate_type: InstallCertificateUseEnumType,
    pub certificate: String,
}

/// This contains the field definition of the InstallCertificateRequest, PDU sent by the Charging Station to the CSMS. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificateResponse {
    pub status: InstallCertificateStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
