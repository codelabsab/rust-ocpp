use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Represents a UnitOfMeasure with a multiplier.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UnitOfMeasureType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Unit of the value. Default = "Wh" if the (default) measurand is an "Energy" type.
    /// This field SHALL use a value from the list Standardized Units of Measurements in Part 2 Appendices.
    /// If an applicable unit is available in that list, otherwise a "custom" unit might be used.
    #[validate(length(max = 20))]
    pub unit: String,

    /// Optional. Multiplier, this value represents the exponent to base 10. I.e. multiplier 3 means 10 raised to the third power.
    /// Default is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = -9, max = 9))]
    pub multiplier: Option<i8>,
}
