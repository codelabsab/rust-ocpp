use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Limit at State of Charge settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LimitAtSoCType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Priority of setting (0=highest)
    #[validate(range(min = 0))]
    pub priority: i32,

    /// State of Charge at which power limit becomes active.
    pub soc: f64,

    /// Maximum power level when power limit is active.
    pub power_limit: f64,
}

impl LimitAtSoCType {
    /// Creates a new `LimitAtSoCType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of setting (0=highest)
    /// * `soc` - State of Charge at which power limit becomes active
    /// * `power_limit` - Maximum power level when power limit is active
    ///
    /// # Returns
    ///
    /// A new instance of `LimitAtSoCType` with optional fields set to `None`
    pub fn new(priority: i32, soc: f64, power_limit: f64) -> Self {
        Self {
            priority,
            soc,
            power_limit,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this LimitAtSoC
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the priority.
    ///
    /// # Returns
    ///
    /// The priority of setting (0=highest)
    pub fn priority(&self) -> i32 {
        self.priority
    }

    /// Sets the priority.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of setting (0=highest)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_priority(&mut self, priority: i32) -> &mut Self {
        self.priority = priority;
        self
    }

    /// Gets the State of Charge.
    ///
    /// # Returns
    ///
    /// The State of Charge at which power limit becomes active
    pub fn soc(&self) -> f64 {
        self.soc
    }

    /// Sets the State of Charge.
    ///
    /// # Arguments
    ///
    /// * `soc` - State of Charge at which power limit becomes active
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_soc(&mut self, soc: f64) -> &mut Self {
        self.soc = soc;
        self
    }

    /// Gets the power limit.
    ///
    /// # Returns
    ///
    /// The maximum power level when power limit is active
    pub fn power_limit(&self) -> f64 {
        self.power_limit
    }

    /// Sets the power limit.
    ///
    /// # Arguments
    ///
    /// * `power_limit` - Maximum power level when power limit is active
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_power_limit(&mut self, power_limit: f64) -> &mut Self {
        self.power_limit = power_limit;
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
    /// * `custom_data` - Custom data for this LimitAtSoC, or None to clear
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
    fn test_new_limit_at_soc() {
        let priority = 1;
        let soc = 80.0;
        let power_limit = 7500.0;

        let limit = LimitAtSoCType::new(priority, soc, power_limit);

        assert_eq!(limit.priority(), priority);
        assert_eq!(limit.soc(), soc);
        assert_eq!(limit.power_limit(), power_limit);
        assert_eq!(limit.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let priority = 1;
        let soc = 80.0;
        let power_limit = 7500.0;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let limit =
            LimitAtSoCType::new(priority, soc, power_limit).with_custom_data(custom_data.clone());

        assert_eq!(limit.priority(), priority);
        assert_eq!(limit.soc(), soc);
        assert_eq!(limit.power_limit(), power_limit);
        assert_eq!(limit.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let priority1 = 1;
        let soc1 = 80.0;
        let power_limit1 = 7500.0;
        let priority2 = 2;
        let soc2 = 90.0;
        let power_limit2 = 5000.0;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut limit = LimitAtSoCType::new(priority1, soc1, power_limit1);

        limit
            .set_priority(priority2)
            .set_soc(soc2)
            .set_power_limit(power_limit2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(limit.priority(), priority2);
        assert_eq!(limit.soc(), soc2);
        assert_eq!(limit.power_limit(), power_limit2);
        assert_eq!(limit.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        limit.set_custom_data(None);
        assert_eq!(limit.custom_data(), None);
    }
}
