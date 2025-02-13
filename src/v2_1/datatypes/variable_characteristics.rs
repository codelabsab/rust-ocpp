use super::custom_data::CustomDataType;
use serde::{Deserialize, Serialize};

/// Defines the datatype of the variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataEnumType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "decimal")]
    Decimal,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "dateTime")]
    DateTime,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "OptionList")]
    OptionList,
    #[serde(rename = "SequenceList")]
    SequenceList,
    #[serde(rename = "MemberList")]
    MemberList,
}

/// Fixed read-only parameters of a variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableCharacteristicsType {
    /// Unit of the variable. When the transmitted value has a unit, this field SHALL be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    /// Data type of this variable.
    pub data_type: DataEnumType,

    /// Minimum possible value of this variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_limit: Option<f64>,

    /// Maximum possible value of this variable.
    /// When the datatype is String, OptionList, SequenceList or MemberList,
    /// this field defines the maximum length of the (CSV) string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_limit: Option<f64>,

    /// Maximum number of elements from valuesList that are supported as attributeValue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_elements: Option<i32>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
