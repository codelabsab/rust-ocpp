use crate::v2_0_1::enumerations::vpn_enum_type::VPNEnumType;

/// VPN Configuration settings
/// VPNType is used by: SetNetworkProfileRequest.NetworkConnectionProfileType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VPNType<'a> {
    pub server: &'a str,
    pub user: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<&'a str>,
    pub password: &'a str,
    pub key: &'a str,
    #[serde(rename = "type")]
    pub kind: VPNEnumType,
}
