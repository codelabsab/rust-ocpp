use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{cost::CostType, custom_data::CustomDataType};

/// Consumption cost type for consumption blocks.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
pub struct ConsumptionCostType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// The lowest level of consumption that defines the starting point of this consumption block.
    /// The block interval extends to the start of the next interval.
    pub start_value: f64,

    /// List of costs associated with this consumption block.
    #[validate(length(min = 1, max = 3))]
    pub cost: Vec<CostType>,
}
