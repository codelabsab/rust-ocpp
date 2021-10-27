use super::evse_type::EVSEType;
use validator::Validate;
/// A physical or logical component
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ComponentType {
    /// Required. Name of the component. Name should be taken from the list of standardized component names whenever possible. Case Insensitive. strongly advised to use Camel Case
    #[validate(length(min = 0, max = 50))]
    pub name: String,
    /// Optional. Name of instance in case the component exists as multiple instances. Case Insensitive. strongly advised to use Camel Case
    #[validate(length(min = 0, max = 50))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// Optional. Specifies the EVSE when component is located at EVSE level, also specifies the connector when component is located at Connector leve
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
}
