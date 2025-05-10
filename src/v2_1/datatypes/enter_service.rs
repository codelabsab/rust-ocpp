use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Parameters for the EnterService DER control function.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EnterServiceType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Priority of setting (0=highest)
    #[validate(range(min = 0))]
    pub priority: i32,

    /// Enter service voltage high
    pub high_voltage: f64,

    /// Enter service voltage low
    pub low_voltage: f64,

    /// Enter service frequency high
    pub high_freq: f64,

    /// Enter service frequency low
    pub low_freq: f64,

    /// Enter service delay
    pub delay: f64,

    /// Enter service randomized delay
    pub random_delay: f64,

    /// Enter service ramp rate in seconds
    pub ramp_rate: f64,
}

impl EnterServiceType {
    /// Creates a new `EnterServiceType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of setting (0=highest)
    /// * `high_voltage` - Enter service voltage high
    /// * `low_voltage` - Enter service voltage low
    /// * `high_freq` - Enter service frequency high
    /// * `low_freq` - Enter service frequency low
    /// * `delay` - Enter service delay
    /// * `random_delay` - Enter service randomized delay
    /// * `ramp_rate` - Enter service ramp rate in seconds
    ///
    /// # Returns
    ///
    /// A new instance of `EnterServiceType` with optional fields set to `None`
    pub fn new(
        priority: i32,
        high_voltage: f64,
        low_voltage: f64,
        high_freq: f64,
        low_freq: f64,
        delay: f64,
        random_delay: f64,
        ramp_rate: f64,
    ) -> Self {
        Self {
            priority,
            high_voltage,
            low_voltage,
            high_freq,
            low_freq,
            delay,
            random_delay,
            ramp_rate,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these enter service parameters
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

    /// Gets the high voltage.
    ///
    /// # Returns
    ///
    /// The enter service voltage high
    pub fn high_voltage(&self) -> f64 {
        self.high_voltage
    }

    /// Sets the high voltage.
    ///
    /// # Arguments
    ///
    /// * `high_voltage` - Enter service voltage high
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_high_voltage(&mut self, high_voltage: f64) -> &mut Self {
        self.high_voltage = high_voltage;
        self
    }

    /// Gets the low voltage.
    ///
    /// # Returns
    ///
    /// The enter service voltage low
    pub fn low_voltage(&self) -> f64 {
        self.low_voltage
    }

    /// Sets the low voltage.
    ///
    /// # Arguments
    ///
    /// * `low_voltage` - Enter service voltage low
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_low_voltage(&mut self, low_voltage: f64) -> &mut Self {
        self.low_voltage = low_voltage;
        self
    }

    /// Gets the high frequency.
    ///
    /// # Returns
    ///
    /// The enter service frequency high
    pub fn high_freq(&self) -> f64 {
        self.high_freq
    }

    /// Sets the high frequency.
    ///
    /// # Arguments
    ///
    /// * `high_freq` - Enter service frequency high
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_high_freq(&mut self, high_freq: f64) -> &mut Self {
        self.high_freq = high_freq;
        self
    }

    /// Gets the low frequency.
    ///
    /// # Returns
    ///
    /// The enter service frequency low
    pub fn low_freq(&self) -> f64 {
        self.low_freq
    }

    /// Sets the low frequency.
    ///
    /// # Arguments
    ///
    /// * `low_freq` - Enter service frequency low
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_low_freq(&mut self, low_freq: f64) -> &mut Self {
        self.low_freq = low_freq;
        self
    }

    /// Gets the delay.
    ///
    /// # Returns
    ///
    /// The enter service delay
    pub fn delay(&self) -> f64 {
        self.delay
    }

    /// Sets the delay.
    ///
    /// # Arguments
    ///
    /// * `delay` - Enter service delay
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_delay(&mut self, delay: f64) -> &mut Self {
        self.delay = delay;
        self
    }

    /// Gets the random delay.
    ///
    /// # Returns
    ///
    /// The enter service randomized delay
    pub fn random_delay(&self) -> f64 {
        self.random_delay
    }

    /// Sets the random delay.
    ///
    /// # Arguments
    ///
    /// * `random_delay` - Enter service randomized delay
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_random_delay(&mut self, random_delay: f64) -> &mut Self {
        self.random_delay = random_delay;
        self
    }

    /// Gets the ramp rate.
    ///
    /// # Returns
    ///
    /// The enter service ramp rate in seconds
    pub fn ramp_rate(&self) -> f64 {
        self.ramp_rate
    }

    /// Sets the ramp rate.
    ///
    /// # Arguments
    ///
    /// * `ramp_rate` - Enter service ramp rate in seconds
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ramp_rate(&mut self, ramp_rate: f64) -> &mut Self {
        self.ramp_rate = ramp_rate;
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
    /// * `custom_data` - Custom data for these enter service parameters, or None to clear
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
    fn test_new_enter_service() {
        let priority = 1;
        let high_voltage = 240.0;
        let low_voltage = 220.0;
        let high_freq = 60.5;
        let low_freq = 59.5;
        let delay = 5.0;
        let random_delay = 2.0;
        let ramp_rate = 10.0;

        let enter_service = EnterServiceType::new(
            priority,
            high_voltage,
            low_voltage,
            high_freq,
            low_freq,
            delay,
            random_delay,
            ramp_rate,
        );

        assert_eq!(enter_service.priority(), priority);
        assert_eq!(enter_service.high_voltage(), high_voltage);
        assert_eq!(enter_service.low_voltage(), low_voltage);
        assert_eq!(enter_service.high_freq(), high_freq);
        assert_eq!(enter_service.low_freq(), low_freq);
        assert_eq!(enter_service.delay(), delay);
        assert_eq!(enter_service.random_delay(), random_delay);
        assert_eq!(enter_service.ramp_rate(), ramp_rate);
        assert_eq!(enter_service.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let priority = 1;
        let high_voltage = 240.0;
        let low_voltage = 220.0;
        let high_freq = 60.5;
        let low_freq = 59.5;
        let delay = 5.0;
        let random_delay = 2.0;
        let ramp_rate = 10.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let enter_service = EnterServiceType::new(
            priority,
            high_voltage,
            low_voltage,
            high_freq,
            low_freq,
            delay,
            random_delay,
            ramp_rate,
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(enter_service.priority(), priority);
        assert_eq!(enter_service.high_voltage(), high_voltage);
        assert_eq!(enter_service.low_voltage(), low_voltage);
        assert_eq!(enter_service.high_freq(), high_freq);
        assert_eq!(enter_service.low_freq(), low_freq);
        assert_eq!(enter_service.delay(), delay);
        assert_eq!(enter_service.random_delay(), random_delay);
        assert_eq!(enter_service.ramp_rate(), ramp_rate);
        assert_eq!(enter_service.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let priority1 = 1;
        let high_voltage1 = 240.0;
        let low_voltage1 = 220.0;
        let high_freq1 = 60.5;
        let low_freq1 = 59.5;
        let delay1 = 5.0;
        let random_delay1 = 2.0;
        let ramp_rate1 = 10.0;

        let priority2 = 2;
        let high_voltage2 = 245.0;
        let low_voltage2 = 215.0;
        let high_freq2 = 61.0;
        let low_freq2 = 59.0;
        let delay2 = 6.0;
        let random_delay2 = 3.0;
        let ramp_rate2 = 12.0;

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut enter_service = EnterServiceType::new(
            priority1,
            high_voltage1,
            low_voltage1,
            high_freq1,
            low_freq1,
            delay1,
            random_delay1,
            ramp_rate1,
        );

        enter_service
            .set_priority(priority2)
            .set_high_voltage(high_voltage2)
            .set_low_voltage(low_voltage2)
            .set_high_freq(high_freq2)
            .set_low_freq(low_freq2)
            .set_delay(delay2)
            .set_random_delay(random_delay2)
            .set_ramp_rate(ramp_rate2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(enter_service.priority(), priority2);
        assert_eq!(enter_service.high_voltage(), high_voltage2);
        assert_eq!(enter_service.low_voltage(), low_voltage2);
        assert_eq!(enter_service.high_freq(), high_freq2);
        assert_eq!(enter_service.low_freq(), low_freq2);
        assert_eq!(enter_service.delay(), delay2);
        assert_eq!(enter_service.random_delay(), random_delay2);
        assert_eq!(enter_service.ramp_rate(), ramp_rate2);
        assert_eq!(enter_service.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        enter_service.set_custom_data(None);
        assert_eq!(enter_service.custom_data(), None);
    }
}
