use crate::v2_0_1::core::enumerations::apn_authentication_enum_type::APNAuthenticationEnumType;

/// Collection of configuration data needed to make a data-connection over a cellular network
/*
    NOTE
    When asking a GSM modem to dial in, it is possible to specify which mobile operator should be used.
    This can be done with the mobile country code (MCC) in combination with a mobile network code (MNC).
    Example: If your preferred network is Vodafone Netherlands, the MCC=204 and the MNC=04 which means the key PreferredNetwork = 20404 Some modems allows to specify a preferred network, which means, if this network is not available, a different network is used.
    If you specify UseOnlyPreferredNetwork and this network is not available, the modem will not dial in
*/
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct APNType {
    pub apn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apn_user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apn_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_pin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_only_preferred_network: Option<bool>,
    pub apn_authentication: APNAuthenticationEnumType,
}
