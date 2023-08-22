/// Contains information about a specific configuration key. It is returned in GetConfigurationResponse
use validator::Validate;

#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct KeyValue<'a> {
    /// Required.
    #[validate(length(min = 1, max = 50))]
    pub key: &'a str,
    /// Required. False if the value can be set with the ChangeConfiguration message.
    pub readonly: bool,
    /// Optional. If key is known but not set, this field may be absent.
    #[validate(length(min = 1, max = 500))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<&'a str>,
}
