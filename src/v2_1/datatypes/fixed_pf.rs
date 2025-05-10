use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Fixed power factor settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FixedPFType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Priority of setting (0=highest)
    #[validate(range(min = 0))]
    pub priority: i32,

    /// Fixed power factor value.
    /// A positive value means DER is absorbing reactive power (under-excited),
    /// a negative value means DER is injecting reactive power (over-excited).
    pub power_factor: f64,
}

impl FixedPFType {
    /// Creates a new `FixedPFType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of setting (0=highest)
    /// * `power_factor` - Fixed power factor value
    ///
    /// # Returns
    ///
    /// A new instance of `FixedPFType` with optional fields set to `None`
    pub fn new(priority: i32, power_factor: f64) -> Self {
        Self {
            priority,
            power_factor,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these fixed power factor settings
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

    /// Gets the power factor.
    ///
    /// # Returns
    ///
    /// The fixed power factor value
    pub fn power_factor(&self) -> f64 {
        self.power_factor
    }

    /// Sets the power factor.
    ///
    /// # Arguments
    ///
    /// * `power_factor` - Fixed power factor value
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_power_factor(&mut self, power_factor: f64) -> &mut Self {
        self.power_factor = power_factor;
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
    /// * `custom_data` - Custom data for these fixed power factor settings, or None to clear
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
    fn test_new_fixed_pf() {
        let priority = 1;
        let power_factor = 0.95;

        let fixed_pf = FixedPFType::new(priority, power_factor);

        assert_eq!(fixed_pf.priority(), priority);
        assert_eq!(fixed_pf.power_factor(), power_factor);
        assert_eq!(fixed_pf.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let priority = 1;
        let power_factor = 0.95;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let fixed_pf =
            FixedPFType::new(priority, power_factor).with_custom_data(custom_data.clone());

        assert_eq!(fixed_pf.priority(), priority);
        assert_eq!(fixed_pf.power_factor(), power_factor);
        assert_eq!(fixed_pf.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let priority1 = 1;
        let power_factor1 = 0.95;
        let priority2 = 2;
        let power_factor2 = -0.9;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut fixed_pf = FixedPFType::new(priority1, power_factor1);

        fixed_pf
            .set_priority(priority2)
            .set_power_factor(power_factor2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(fixed_pf.priority(), priority2);
        assert_eq!(fixed_pf.power_factor(), power_factor2);
        assert_eq!(fixed_pf.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        fixed_pf.set_custom_data(None);
        assert_eq!(fixed_pf.custom_data(), None);
    }
}
