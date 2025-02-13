use super::custom_data::CustomDataType;
use serde::{Deserialize, Serialize};
use validator::Validate;

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
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VariableCharacteristicsType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Unit of the variable. When the variable represents a measurand from the measurand enumeration, this field SHALL contain the unit of the measurand as used in the signedMeterValue field defined in Part 2.
    #[validate(length(max = 16))]
    pub unit: String,

    /// Required. Data type of this variable.
    pub data_type: DataEnumType,

    /// Required. Minimum possible value of this variable.
    #[validate(length(max = 1000))]
    pub min_limit: String,

    /// Required. Maximum possible value of this variable.
    #[validate(length(max = 1000))]
    pub max_limit: String,

    /// Required. When true, value from the Charging Station may not be set to null.
    pub supports_monitoring: bool,
}
