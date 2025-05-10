use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Rule that describes the pricing of overstaying.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OverstayRuleType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Duration in seconds after which the overstay rule becomes active.
    #[validate(range(min = 0, max = 86400))]
    pub overstay_time_threshold: i32,

    /// Required. Factor by which the price is multiplied when the overstay rule is active.
    #[validate(range(min = 0.0))]
    pub overstay_fee_threshold: f32,
}

impl OverstayRuleType {
    /// Creates a new `OverstayRuleType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `overstay_time_threshold` - Duration in seconds after which the overstay rule becomes active
    /// * `overstay_fee_threshold` - Factor by which the price is multiplied when the overstay rule is active
    ///
    /// # Returns
    ///
    /// A new instance of `OverstayRuleType` with optional fields set to `None`
    pub fn new(overstay_time_threshold: i32, overstay_fee_threshold: f32) -> Self {
        Self {
            overstay_time_threshold,
            overstay_fee_threshold,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this overstay rule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the overstay time threshold.
    ///
    /// # Returns
    ///
    /// The duration in seconds after which the overstay rule becomes active
    pub fn overstay_time_threshold(&self) -> i32 {
        self.overstay_time_threshold
    }

    /// Sets the overstay time threshold.
    ///
    /// # Arguments
    ///
    /// * `overstay_time_threshold` - Duration in seconds after which the overstay rule becomes active
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_overstay_time_threshold(&mut self, overstay_time_threshold: i32) -> &mut Self {
        self.overstay_time_threshold = overstay_time_threshold;
        self
    }

    /// Gets the overstay fee threshold.
    ///
    /// # Returns
    ///
    /// The factor by which the price is multiplied when the overstay rule is active
    pub fn overstay_fee_threshold(&self) -> f32 {
        self.overstay_fee_threshold
    }

    /// Sets the overstay fee threshold.
    ///
    /// # Arguments
    ///
    /// * `overstay_fee_threshold` - Factor by which the price is multiplied when the overstay rule is active
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_overstay_fee_threshold(&mut self, overstay_fee_threshold: f32) -> &mut Self {
        self.overstay_fee_threshold = overstay_fee_threshold;
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
    /// * `custom_data` - Custom data for this overstay rule, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_overstay_rule() {
        let time_threshold = 3600;
        let fee_threshold = 2.5;

        let rule = OverstayRuleType::new(time_threshold, fee_threshold);

        assert_eq!(rule.overstay_time_threshold(), time_threshold);
        assert_eq!(rule.overstay_fee_threshold(), fee_threshold);
        assert_eq!(rule.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let time_threshold = 3600;
        let fee_threshold = 2.5;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let rule = OverstayRuleType::new(time_threshold, fee_threshold)
            .with_custom_data(custom_data.clone());

        assert_eq!(rule.overstay_time_threshold(), time_threshold);
        assert_eq!(rule.overstay_fee_threshold(), fee_threshold);
        assert_eq!(rule.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let time_threshold1 = 3600;
        let fee_threshold1 = 2.5;
        let time_threshold2 = 7200;
        let fee_threshold2 = 3.0;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut rule = OverstayRuleType::new(time_threshold1, fee_threshold1);

        rule.set_overstay_time_threshold(time_threshold2)
            .set_overstay_fee_threshold(fee_threshold2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(rule.overstay_time_threshold(), time_threshold2);
        assert_eq!(rule.overstay_fee_threshold(), fee_threshold2);
        assert_eq!(rule.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        rule.set_custom_data(None);
        assert_eq!(rule.custom_data(), None);
    }
}
