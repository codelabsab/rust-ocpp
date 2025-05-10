use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Price rule for a power range.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EVPriceRuleType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Energy fee in the currency specified in EVAbsolutePriceSchedule.
    pub energy_fee: f64,

    /// Start of the power range in Watts (W).
    pub power_range_start: f64,
}

impl EVPriceRuleType {
    /// Creates a new `EVPriceRuleType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `energy_fee` - Energy fee in the currency specified in EVAbsolutePriceSchedule
    /// * `power_range_start` - Start of the power range in Watts (W)
    ///
    /// # Returns
    ///
    /// A new instance of `EVPriceRuleType` with optional fields set to `None`
    pub fn new(energy_fee: f64, power_range_start: f64) -> Self {
        Self {
            energy_fee,
            power_range_start,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this price rule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the energy fee.
    ///
    /// # Returns
    ///
    /// The energy fee in the currency specified in EVAbsolutePriceSchedule
    pub fn energy_fee(&self) -> f64 {
        self.energy_fee
    }

    /// Sets the energy fee.
    ///
    /// # Arguments
    ///
    /// * `energy_fee` - Energy fee in the currency specified in EVAbsolutePriceSchedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_energy_fee(&mut self, energy_fee: f64) -> &mut Self {
        self.energy_fee = energy_fee;
        self
    }

    /// Gets the power range start.
    ///
    /// # Returns
    ///
    /// The start of the power range in Watts (W)
    pub fn power_range_start(&self) -> f64 {
        self.power_range_start
    }

    /// Sets the power range start.
    ///
    /// # Arguments
    ///
    /// * `power_range_start` - Start of the power range in Watts (W)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_power_range_start(&mut self, power_range_start: f64) -> &mut Self {
        self.power_range_start = power_range_start;
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
    /// * `custom_data` - Custom data for this price rule, or None to clear
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
    fn test_new_ev_price_rule() {
        let energy_fee = 0.25;
        let power_range_start = 5000.0;

        let price_rule = EVPriceRuleType::new(energy_fee, power_range_start);

        assert_eq!(price_rule.energy_fee(), energy_fee);
        assert_eq!(price_rule.power_range_start(), power_range_start);
        assert_eq!(price_rule.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let energy_fee = 0.25;
        let power_range_start = 5000.0;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let price_rule = EVPriceRuleType::new(energy_fee, power_range_start)
            .with_custom_data(custom_data.clone());

        assert_eq!(price_rule.energy_fee(), energy_fee);
        assert_eq!(price_rule.power_range_start(), power_range_start);
        assert_eq!(price_rule.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let energy_fee1 = 0.25;
        let power_range_start1 = 5000.0;
        let energy_fee2 = 0.30;
        let power_range_start2 = 7500.0;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut price_rule = EVPriceRuleType::new(energy_fee1, power_range_start1);

        price_rule
            .set_energy_fee(energy_fee2)
            .set_power_range_start(power_range_start2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(price_rule.energy_fee(), energy_fee2);
        assert_eq!(price_rule.power_range_start(), power_range_start2);
        assert_eq!(price_rule.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        price_rule.set_custom_data(None);
        assert_eq!(price_rule.custom_data(), None);
    }
}
