use crate::v2_0_1::core::enumerations::{
    ocpp_interface_enum_type::OCPPInterfaceEnumType,
    ocpp_transport_enum_type::OCPPTransportEnumType, ocpp_version_enum_type::OCPPVersionEnumType,
};

use super::{apn_type::APNType, vpn_type::VPNType};

/// The NetworkConnectionProfile defines the functional and technical parameters of a communication link.
/// NetworkConnectionProfileType is used by: SetNetworkProfileRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConnectionProfileType {
    pub ocpp_version: OCPPVersionEnumType,
    pub ocpp_transport: OCPPTransportEnumType,
    pub ocpp_csms_url: String,
    pub message_timeout: i64,
    pub security_profile: i64,
    pub ocpp_interface: OCPPInterfaceEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn: Option<VPNType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apn: Option<APNType>,
}
