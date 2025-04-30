use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Point in a signal-watt curve for V2X.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct V2XSignalWattPointType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Signal value between 0 and 100 percent.
    #[validate(range(min = 0.0, max = 100.0))]
    pub signal: f64,

    /// Required. Power in percent of maximum power. Negative values indicate power being discharged from the EV.
    pub power: f64,
}

impl V2XSignalWattPointType {
    /// Creates a new `V2XSignalWattPointType` with the required fields.
    ///
    /// # Arguments
    ///
    /// * `signal` - Signal value between 0 and 100 percent
    /// * `power` - Power in percent of maximum power (negative values indicate power being discharged from the EV)
    ///
    /// # Returns
    ///
    /// A new `V2XSignalWattPointType` instance with optional fields set to `None`
    pub fn new(signal: f64, power: f64) -> Self {
        Self {
            custom_data: None,
            signal,
            power,
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
    /// The modified `V2XSignalWattPointType` instance
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
    /// * `custom_data` - Custom data from the Charging Station
    ///
    /// # Returns
    ///
    /// The modified `V2XSignalWattPointType` instance
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the signal value.
    ///
    /// # Returns
    ///
    /// The signal value between 0 and 100 percent
    pub fn signal(&self) -> f64 {
        self.signal
    }

    /// Sets the signal value.
    ///
    /// # Arguments
    ///
    /// * `signal` - Signal value between 0 and 100 percent
    ///
    /// # Returns
    ///
    /// The modified `V2XSignalWattPointType` instance
    pub fn set_signal(&mut self, signal: f64) -> &mut Self {
        self.signal = signal;
        self
    }

    /// Gets the power value.
    ///
    /// # Returns
    ///
    /// The power in percent of maximum power
    pub fn power(&self) -> f64 {
        self.power
    }

    /// Sets the power value.
    ///
    /// # Arguments
    ///
    /// * `power` - Power in percent of maximum power (negative values indicate power being discharged from the EV)
    ///
    /// # Returns
    ///
    /// The modified `V2XSignalWattPointType` instance
    pub fn set_power(&mut self, power: f64) -> &mut Self {
        self.power = power;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2x_signal_watt_point_new() {
        let point = V2XSignalWattPointType::new(75.0, -30.0);
        
        assert_eq!(point.signal(), 75.0);
        assert_eq!(point.power(), -30.0);
        assert_eq!(point.custom_data(), None);
    }

    #[test]
    fn test_v2x_signal_watt_point_with_methods() {
        let custom_data = CustomDataType::new("Vendor".to_string());
        let point = V2XSignalWattPointType::new(50.0, -40.0)
            .with_custom_data(custom_data.clone());
        
        assert_eq!(point.signal(), 50.0);
        assert_eq!(point.power(), -40.0);
        assert_eq!(point.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_v2x_signal_watt_point_setters() {
        let mut point = V2XSignalWattPointType::new(75.0, -30.0);
        
        point
            .set_signal(80.0)
            .set_power(-25.0);
        
        assert_eq!(point.signal(), 80.0);
        assert_eq!(point.power(), -25.0);
    }
}
