use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, price_rule::PriceRuleType};

/// Stack of price rules, defining the price of charging.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PriceRuleStackType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Duration in seconds after which the price rule becomes active.
    #[validate(range(min = 0, max = 86400))]
    pub duration: i32,

    /// Required. List of price rules that are part of the stack.
    #[validate(length(min = 1))]
    pub price_rules: Vec<PriceRuleType>,
}
