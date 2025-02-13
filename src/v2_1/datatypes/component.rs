use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, evse::EVSEType};

/// A physical or logical component
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
pub struct ComponentType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,

    #[validate(length(max = 50))]
    /// Name of the component. Name should be taken from the list of standardized component names whenever possible.
    /// Case Insensitive. strongly advised to use Camel Case.
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    /// Name of instance in case the component exists as multiple instances. Case Insensitive. strongly advised to use Camel Case.
    pub instance: Option<String>,
}
