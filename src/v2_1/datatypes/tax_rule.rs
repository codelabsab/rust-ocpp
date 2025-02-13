use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, tax_rate::TaxRateType};

/// Tax rule structure defining tax rules for a tariff.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TaxRuleType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Tax rate for this tax rule.
    pub tax_rate: TaxRateType,

    /// Optional. Type of energy for which this tax rule applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub energy_source_type: Option<String>,

    /// Optional. Type of consumption for which this tax rule applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub consumption_type: Option<String>,
}
