use validator::Validate;

use crate::v1_6::types::KeyValue;

/// This contains the field definition of the GetConfiguration.req PDU sent by the Central System to the Charge Point. See also Get Configuration
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationRequest {
    /// Optional. List of keys for which the configuration value is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 50))]
    pub key: Option<String>,
}

/// This contains the field definition of the GetConfiguration.conf PDU sent by Charge Point the to the Central System in response to a GetConfiguration.req. See also Get Configuration
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationResponse {
    /// Optional. List of requested or known keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_key: Option<KeyValue>,
    /// Optional. Requested keys that are unknown
    #[validate(length(min = 1, max = 50))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_key: Option<String>,
}
