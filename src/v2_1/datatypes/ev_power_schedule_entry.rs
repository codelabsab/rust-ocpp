use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Entry in the EVPowerSchedule.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EVPowerScheduleEntryType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Duration of the schedule entry in seconds.
    pub duration: i32,

    /// Power in Watts (W).
    pub power: f64,
}

impl EVPowerScheduleEntryType {
    /// Creates a new `EVPowerScheduleEntryType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the schedule entry in seconds
    /// * `power` - Power in Watts (W)
    ///
    /// # Returns
    ///
    /// A new instance of `EVPowerScheduleEntryType` with optional fields set to `None`
    pub fn new(duration: i32, power: f64) -> Self {
        Self {
            duration,
            power,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this schedule entry
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the duration.
    ///
    /// # Returns
    ///
    /// The duration of the schedule entry in seconds
    pub fn duration(&self) -> i32 {
        self.duration
    }

    /// Sets the duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the schedule entry in seconds
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_duration(&mut self, duration: i32) -> &mut Self {
        self.duration = duration;
        self
    }

    /// Gets the power.
    ///
    /// # Returns
    ///
    /// The power in Watts (W)
    pub fn power(&self) -> f64 {
        self.power
    }

    /// Sets the power.
    ///
    /// # Arguments
    ///
    /// * `power` - Power in Watts (W)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_power(&mut self, power: f64) -> &mut Self {
        self.power = power;
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
    /// * `custom_data` - Custom data for this schedule entry, or None to clear
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
    fn test_new_ev_power_schedule_entry() {
        let duration = 3600;
        let power = 11000.0;

        let entry = EVPowerScheduleEntryType::new(duration, power);

        assert_eq!(entry.duration(), duration);
        assert_eq!(entry.power(), power);
        assert_eq!(entry.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let duration = 3600;
        let power = 11000.0;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let entry = EVPowerScheduleEntryType::new(duration, power)
            .with_custom_data(custom_data.clone());

        assert_eq!(entry.duration(), duration);
        assert_eq!(entry.power(), power);
        assert_eq!(entry.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let duration1 = 3600;
        let power1 = 11000.0;
        let duration2 = 7200;
        let power2 = 7500.0;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut entry = EVPowerScheduleEntryType::new(duration1, power1);

        entry
            .set_duration(duration2)
            .set_power(power2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(entry.duration(), duration2);
        assert_eq!(entry.power(), power2);
        assert_eq!(entry.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        entry.set_custom_data(None);
        assert_eq!(entry.custom_data(), None);
    }
}
