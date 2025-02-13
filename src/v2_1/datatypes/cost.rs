use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::CostKindEnumType;

/// Cost type for consumption costs.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
pub struct CostType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// The kind of cost referred to in the message element amount.
    pub cost_kind: CostKindEnumType,

    /// The estimated or actual cost per kWh.
    pub amount: i32,

    /// Values: -3..3, The amountMultiplier defines the exponent to base 10 (dec).
    /// The final value is determined by: amount * 10 ^ amountMultiplier.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = -3, max = 3))]
    pub amount_multiplier: Option<i8>,
}
