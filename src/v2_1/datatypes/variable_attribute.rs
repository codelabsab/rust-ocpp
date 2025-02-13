use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::MutabilityEnumType;

/// Attribute data of a variable.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VariableAttributeType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Type of attribute: Actual, Target, MinSet, MaxSet, etc.
    /// This value refers to the attribute name from the list of attributes provided in the Variable class from the OCPP 2.0 Part 2 Appendices.
    #[validate(length(max = 50))]
    pub r#type: String,

    /// Required. Value of the attribute. May only be omitted when mutability is set to 'WriteOnly'.
    /// The Configuration Variable ConfigurationValueSize can be used to limit the size of this field.
    #[validate(length(max = 1000))]
    pub value: String,

    /// Required. Defines the mutability of this attribute.
    /// Default = ReadWrite when omitted.
    pub mutability: MutabilityEnumType,

    /// Optional. Boolean indicating if this variable is persistent between sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,

    /// Optional. Boolean indicating if this variable is constant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<bool>,
}
