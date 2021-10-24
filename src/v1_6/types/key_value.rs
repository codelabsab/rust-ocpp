/// Contains information about a specific configuration key. It is returned in GetConfigurationResponse
use validator::Validate;

#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
pub struct KeyValue {
    /// Required.
    #[validate(length(min = 1, max = 50))]
    key: String,
    /// Required. False if the value can be set with the ChangeConfiguration message.
    readonly: bool,
    /// Optional. If key is known but not set, this field may be absent.
    #[validate(length(min = 1, max = 500))]
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}
