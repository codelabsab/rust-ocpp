#[cfg(feature = "std")]
use validator::Validate;

use crate::v1_6::types::KeyValue;
use crate::Vec;

/// This contains the field definition of the GetConfiguration.req PDU sent by the Central System to the Charge Point. See also Get Configuration
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationRequest<'a, const N_KEYS: usize = { crate::N_KEYS }> {
    /// Optional. List of keys for which the configuration value is requested.
    #[serde(skip_serializing_if = "Option::is_none",borrow)]
    #[cfg_attr(feature="std", validate(length(min = 1, max = 50)))]
    pub key: Option<Vec<&'a str, N_KEYS>>,
}

/// This contains the field definition of the GetConfiguration.conf PDU sent by Charge Point the to the Central System in response to a GetConfiguration.req. See also Get Configuration
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationResponse<'a, const N_CONF_KEYS: usize = { crate::N_CONF_KEYS }, const N_UNKNOWN_KEYS: usize = { crate::N_UNKNOWN_KEYS }> {
    /// Optional. List of requested or known keys
    #[serde(skip_serializing_if = "Option::is_none",borrow)]
    pub configuration_key: Option<Vec<KeyValue<'a>, N_CONF_KEYS>>,
    /// Optional. Requested keys that are unknown
    #[cfg_attr(feature="std", validate(length(min = 1, max = 50)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_key: Option<Vec<&'a str, N_UNKNOWN_KEYS>>,
}
