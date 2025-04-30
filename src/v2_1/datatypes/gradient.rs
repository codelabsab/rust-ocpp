use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Gradient settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GradientType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Priority of setting (0=highest)
    #[validate(range(min = 0))]
    pub priority: i32,

    /// Maximum rate of change in percent per second.
    pub rate: f64,
}

impl GradientType {
    /// Creates a new `GradientType` with the required fields.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of setting (0=highest)
    /// * `rate` - Maximum rate of change in percent per second
    ///
    /// # Returns
    ///
    /// A new `GradientType` instance with optional fields set to `None`
    pub fn new(priority: i32, rate: f64) -> Self {
        Self {
            custom_data: None,
            priority,
            rate,
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
    /// The modified `GradientType` instance
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
    /// The modified `GradientType` instance
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
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
    /// The modified `GradientType` instance
    pub fn set_priority(&mut self, priority: i32) -> &mut Self {
        self.priority = priority;
        self
    }

    /// Gets the rate.
    ///
    /// # Returns
    ///
    /// The maximum rate of change in percent per second
    pub fn rate(&self) -> f64 {
        self.rate
    }

    /// Sets the rate.
    ///
    /// # Arguments
    ///
    /// * `rate` - Maximum rate of change in percent per second
    ///
    /// # Returns
    ///
    /// The modified `GradientType` instance
    pub fn set_rate(&mut self, rate: f64) -> &mut Self {
        self.rate = rate;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_new() {
        let priority = 1;
        let rate = 5.0;

        let gradient = GradientType::new(priority, rate);

        assert_eq!(gradient.priority(), priority);
        assert_eq!(gradient.rate(), rate);
        assert_eq!(gradient.custom_data(), None);
    }

    #[test]
    fn test_gradient_with_methods() {
        let priority = 1;
        let rate = 5.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let gradient = GradientType::new(priority, rate).with_custom_data(custom_data.clone());

        assert_eq!(gradient.priority(), priority);
        assert_eq!(gradient.rate(), rate);
        assert_eq!(gradient.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_gradient_setters() {
        let priority1 = 1;
        let priority2 = 2;
        let rate1 = 5.0;
        let rate2 = 10.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut gradient = GradientType::new(priority1, rate1);

        gradient
            .set_priority(priority2)
            .set_rate(rate2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(gradient.priority(), priority2);
        assert_eq!(gradient.rate(), rate2);
        assert_eq!(gradient.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        gradient.set_custom_data(None);
        assert_eq!(gradient.custom_data(), None);
    }
}
