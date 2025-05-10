use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Point in a frequency-watt curve for V2X.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct V2XFreqWattPointType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Frequency in Hz.
    pub freq: f64,

    /// Required. Power in percent of maximum power. Negative values indicate power being discharged from the EV.
    pub power: f64,
}

impl V2XFreqWattPointType {
    /// Creates a new `V2XFreqWattPointType` with the required fields.
    ///
    /// # Arguments
    ///
    /// * `freq` - Frequency in Hz
    /// * `power` - Power in percent of maximum power (negative values indicate power being discharged from the EV)
    ///
    /// # Returns
    ///
    /// A new `V2XFreqWattPointType` instance with optional fields set to `None`
    pub fn new(freq: f64, power: f64) -> Self {
        Self {
            custom_data: None,
            freq,
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
    /// The modified `V2XFreqWattPointType` instance
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
    /// The modified `V2XFreqWattPointType` instance
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the frequency.
    ///
    /// # Returns
    ///
    /// The frequency in Hz
    pub fn freq(&self) -> f64 {
        self.freq
    }

    /// Sets the frequency.
    ///
    /// # Arguments
    ///
    /// * `freq` - Frequency in Hz
    ///
    /// # Returns
    ///
    /// The modified `V2XFreqWattPointType` instance
    pub fn set_freq(&mut self, freq: f64) -> &mut Self {
        self.freq = freq;
        self
    }

    /// Gets the power.
    ///
    /// # Returns
    ///
    /// The power in percent of maximum power
    pub fn power(&self) -> f64 {
        self.power
    }

    /// Sets the power.
    ///
    /// # Arguments
    ///
    /// * `power` - Power in percent of maximum power (negative values indicate power being discharged from the EV)
    ///
    /// # Returns
    ///
    /// The modified `V2XFreqWattPointType` instance
    pub fn set_power(&mut self, power: f64) -> &mut Self {
        self.power = power;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2x_freq_watt_point_new() {
        let point = V2XFreqWattPointType::new(50.0, -30.0);

        assert_eq!(point.freq(), 50.0);
        assert_eq!(point.power(), -30.0);
        assert_eq!(point.custom_data(), None);
    }

    #[test]
    fn test_v2x_freq_watt_point_with_methods() {
        let custom_data = CustomDataType::new("Vendor".to_string());
        let point = V2XFreqWattPointType::new(51.5, -40.0).with_custom_data(custom_data.clone());

        assert_eq!(point.freq(), 51.5);
        assert_eq!(point.power(), -40.0);
        assert_eq!(point.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_v2x_freq_watt_point_setters() {
        let mut point = V2XFreqWattPointType::new(50.0, -30.0);

        point.set_freq(49.5).set_power(-25.0);

        assert_eq!(point.freq(), 49.5);
        assert_eq!(point.power(), -25.0);
    }
}
