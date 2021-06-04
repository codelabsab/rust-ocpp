use crate::v2_0_1::core::{
    datatypes::{
        certificate_hash_data_chain_type::CertificateHashDataChainType,
        status_info_type::StatusInfoType,
    },
    enumerations::{
        get_certificate_id_use_enum_type::GetCertificateIdUseEnumType,
        get_display_messages_status_enum_type::GetDisplayMessagesStatusEnumType,
    },
};

/// Used by the CSMS to request an overview of the installed certificates on a Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIdsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<GetCertificateIdUseEnumType>,
}

/// Response to a GetInstalledCertificateIDsRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIdsResponse {
    pub status: GetDisplayMessagesStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_hash_data_chain: Option<CertificateHashDataChainType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
