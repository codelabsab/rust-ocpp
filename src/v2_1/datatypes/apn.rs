use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::APNAuthenticationEnumType;

/// Collection of configuration data needed to make a data-connection over a cellular network.
///
/// NOTE: When asking a GSM modem to dial in, it is possible to specify which mobile operator should be used.
/// This can be done with the mobile country code (MCC) in combination with a mobile network code (MNC).
/// Example: If your preferred network is Vodafone Netherlands, the MCC=204 and the MNC=04 which means
/// the key PreferredNetwork = 20404 Some modems allows to specify a preferred network, which means,
/// if this network is not available, a different network is used. If you specify UseOnlyPreferredNetwork
/// and this network is not available, the modem will not dial in.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct APNType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The Access Point Name as an URL.
    #[validate(length(max = 2000))]
    pub apn: String,

    /// APN username.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub apn_user_name: Option<String>,

    /// APN Password.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 64))]
    pub apn_password: Option<String>,

    /// SIM card pin code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_pin: Option<i32>,

    /// Preferred network, written as MCC and MNC concatenated. See note.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 6))]
    pub preferred_network: Option<String>,

    /// Default: false. Use only the preferred Network, do not dial in when not available. See Note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_only_preferred_network: Option<bool>,

    /// Required. Authentication method.
    pub apn_authentication: APNAuthenticationEnumType,
}
