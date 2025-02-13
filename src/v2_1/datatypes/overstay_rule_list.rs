use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, overstay_rule::OverstayRuleType};

/// List of overstay rules for a charging profile.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OverstayRuleListType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. List of overstay rules.
    #[validate(length(min = 1))]
    pub overstay_rules: Vec<OverstayRuleType>,
}
