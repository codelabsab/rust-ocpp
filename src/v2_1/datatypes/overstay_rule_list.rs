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

impl OverstayRuleListType {
    /// Creates a new `OverstayRuleListType` with the required fields.
    ///
    /// # Arguments
    ///
    /// * `overstay_rules` - List of overstay rules
    ///
    /// # Returns
    ///
    /// A new `OverstayRuleListType` instance with optional fields set to `None`
    pub fn new(overstay_rules: Vec<OverstayRuleType>) -> Self {
        Self {
            custom_data: None,
            overstay_rules,
        }
    }

    /// Sets the custom data field.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data from the Charging Station
    ///
    /// # Returns
    ///
    /// The modified `OverstayRuleListType` instance
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the custom data.
    ///
    /// # Returns
    ///
    /// An optional reference to the custom data
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data from the Charging Station, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `OverstayRuleListType` instance
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the overstay rules.
    ///
    /// # Returns
    ///
    /// A reference to the list of overstay rules
    pub fn overstay_rules(&self) -> &Vec<OverstayRuleType> {
        &self.overstay_rules
    }

    /// Sets the overstay rules.
    ///
    /// # Arguments
    ///
    /// * `overstay_rules` - List of overstay rules
    ///
    /// # Returns
    ///
    /// The modified `OverstayRuleListType` instance
    pub fn set_overstay_rules(&mut self, overstay_rules: Vec<OverstayRuleType>) -> &mut Self {
        self.overstay_rules = overstay_rules;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::datatypes::overstay_rule::OverstayRuleType;

    #[test]
    fn test_overstay_rule_list_new() {
        let overstay_rule = OverstayRuleType::new(10, 5.0);
        let overstay_rules = vec![overstay_rule.clone()];
        let overstay_rule_list = OverstayRuleListType::new(overstay_rules.clone());

        assert_eq!(overstay_rule_list.overstay_rules(), &overstay_rules);
        assert_eq!(overstay_rule_list.custom_data(), None);
    }

    #[test]
    fn test_overstay_rule_list_with_methods() {
        let overstay_rule = OverstayRuleType::new(10, 5.0);
        let overstay_rules = vec![overstay_rule.clone()];
        let custom_data = CustomDataType::new("VendorX".to_string());

        let overstay_rule_list = OverstayRuleListType::new(overstay_rules.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(overstay_rule_list.overstay_rules(), &overstay_rules);
        assert_eq!(overstay_rule_list.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_overstay_rule_list_setters() {
        let overstay_rule1 = OverstayRuleType::new(10, 5.0);
        let overstay_rules1 = vec![overstay_rule1.clone()];

        let overstay_rule2 = OverstayRuleType::new(20, 10.0);
        let overstay_rules2 = vec![overstay_rule2.clone()];

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut overstay_rule_list = OverstayRuleListType::new(overstay_rules1.clone());

        overstay_rule_list
            .set_overstay_rules(overstay_rules2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(overstay_rule_list.overstay_rules(), &overstay_rules2);
        assert_eq!(overstay_rule_list.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        overstay_rule_list.set_custom_data(None);
        assert_eq!(overstay_rule_list.custom_data(), None);
    }
}
