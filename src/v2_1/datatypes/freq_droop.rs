use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Frequency droop settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FreqDroopType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Priority of setting (0=highest)
    #[validate(range(min = 0))]
    pub priority: i32,

    /// Frequency droop slope in percent active power per Hz.
    pub droop: f64,

    /// Frequency offset in Hz.
    pub offset: f64,

    /// Frequency deadband in Hz.
    pub deadband: f64,

    /// Response time in seconds.
    pub response_time: f64,
}

impl FreqDroopType {
    /// Creates a new `FreqDroopType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of setting (0=highest)
    /// * `droop` - Frequency droop slope in percent active power per Hz
    /// * `offset` - Frequency offset in Hz
    /// * `deadband` - Frequency deadband in Hz
    /// * `response_time` - Response time in seconds
    ///
    /// # Returns
    ///
    /// A new instance of `FreqDroopType` with optional fields set to `None`
    pub fn new(priority: i32, droop: f64, offset: f64, deadband: f64, response_time: f64) -> Self {
        Self {
            priority,
            droop,
            offset,
            deadband,
            response_time,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these frequency droop settings
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

    /// Gets the frequency droop slope.
    ///
    /// # Returns
    ///
    /// The frequency droop slope in percent active power per Hz
    pub fn droop(&self) -> f64 {
        self.droop
    }

    /// Sets the frequency droop slope.
    ///
    /// # Arguments
    ///
    /// * `droop` - Frequency droop slope in percent active power per Hz
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_droop(&mut self, droop: f64) -> &mut Self {
        self.droop = droop;
        self
    }

    /// Gets the frequency offset.
    ///
    /// # Returns
    ///
    /// The frequency offset in Hz
    pub fn offset(&self) -> f64 {
        self.offset
    }

    /// Sets the frequency offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - Frequency offset in Hz
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_offset(&mut self, offset: f64) -> &mut Self {
        self.offset = offset;
        self
    }

    /// Gets the frequency deadband.
    ///
    /// # Returns
    ///
    /// The frequency deadband in Hz
    pub fn deadband(&self) -> f64 {
        self.deadband
    }

    /// Sets the frequency deadband.
    ///
    /// # Arguments
    ///
    /// * `deadband` - Frequency deadband in Hz
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_deadband(&mut self, deadband: f64) -> &mut Self {
        self.deadband = deadband;
        self
    }

    /// Gets the response time.
    ///
    /// # Returns
    ///
    /// The response time in seconds
    pub fn response_time(&self) -> f64 {
        self.response_time
    }

    /// Sets the response time.
    ///
    /// # Arguments
    ///
    /// * `response_time` - Response time in seconds
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_response_time(&mut self, response_time: f64) -> &mut Self {
        self.response_time = response_time;
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
    /// * `custom_data` - Custom data for these frequency droop settings, or None to clear
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
    fn test_new_freq_droop() {
        let priority = 1;
        let droop = 5.0;
        let offset = 0.1;
        let deadband = 0.05;
        let response_time = 2.0;

        let freq_droop = FreqDroopType::new(priority, droop, offset, deadband, response_time);

        assert_eq!(freq_droop.priority(), priority);
        assert_eq!(freq_droop.droop(), droop);
        assert_eq!(freq_droop.offset(), offset);
        assert_eq!(freq_droop.deadband(), deadband);
        assert_eq!(freq_droop.response_time(), response_time);
        assert_eq!(freq_droop.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let priority = 1;
        let droop = 5.0;
        let offset = 0.1;
        let deadband = 0.05;
        let response_time = 2.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let freq_droop = FreqDroopType::new(priority, droop, offset, deadband, response_time)
            .with_custom_data(custom_data.clone());

        assert_eq!(freq_droop.priority(), priority);
        assert_eq!(freq_droop.droop(), droop);
        assert_eq!(freq_droop.offset(), offset);
        assert_eq!(freq_droop.deadband(), deadband);
        assert_eq!(freq_droop.response_time(), response_time);
        assert_eq!(freq_droop.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let priority1 = 1;
        let droop1 = 5.0;
        let offset1 = 0.1;
        let deadband1 = 0.05;
        let response_time1 = 2.0;

        let priority2 = 2;
        let droop2 = 6.0;
        let offset2 = 0.2;
        let deadband2 = 0.1;
        let response_time2 = 3.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut freq_droop = FreqDroopType::new(priority1, droop1, offset1, deadband1, response_time1);

        freq_droop
            .set_priority(priority2)
            .set_droop(droop2)
            .set_offset(offset2)
            .set_deadband(deadband2)
            .set_response_time(response_time2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(freq_droop.priority(), priority2);
        assert_eq!(freq_droop.droop(), droop2);
        assert_eq!(freq_droop.offset(), offset2);
        assert_eq!(freq_droop.deadband(), deadband2);
        assert_eq!(freq_droop.response_time(), response_time2);
        assert_eq!(freq_droop.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        freq_droop.set_custom_data(None);
        assert_eq!(freq_droop.custom_data(), None);
    }
}
