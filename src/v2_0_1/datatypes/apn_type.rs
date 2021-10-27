use crate::v2_0_1::enumerations::apn_authentication_enum_type::APNAuthenticationEnumType;
use validator::Validate;

/// Collection of configuration data needed to make a data-connection over a cellular network
// NOTE
// When asking a GSM modem to dial in, it is possible to specify which mobile operator should be used.
// This can be done with the mobile country code (MCC) in combination with a mobile network code (MNC).
// Example: If your preferred network is Vodafone Netherlands, the MCC=204 and the MNC=04 which means the key PreferredNetwork = 20404 Some modems allows to specify a preferred network, which means, if this network is not available, a different network is used.
// If you specify UseOnlyPreferredNetwork and this network is not available, the modem will not dial in
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct APNType {
    /// Required. The Access Point Name as an URL
    #[validate(length(min = 0, max = 512))]
    pub apn: String,
    /// Optional. APN username
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apn_user_name: Option<String>,
    /// Optional. APN Password.
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apn_password: Option<String>,
    /// Optional. SIM card pin code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_pin: Option<i64>,
    /// Optional. Preferred network, written as MCC and MNC concatenated. See note.
    #[validate(length(min = 0, max = 6))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_network: Option<String>,
    /// Optional. Default: false. Use only the preferred Network, do not dial in when not available. See Note
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_only_preferred_network: Option<bool>,
    /// Required. Authentication method.
    pub apn_authentication: APNAuthenticationEnumType,
}
