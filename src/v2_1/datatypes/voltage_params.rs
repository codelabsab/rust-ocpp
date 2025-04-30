use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Parameters for voltage-based control.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VoltageParamsType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Voltage at which to start charging, in Volts.
    pub voltage_start: f64,

    /// Required. Voltage at which to stop charging, in Volts.
    pub voltage_stop: f64,

    /// Required. Voltage at which to start discharging, in Volts.
    pub voltage_discharge_start: f64,

    /// Required. Voltage at which to stop discharging, in Volts.
    pub voltage_discharge_stop: f64,
}

impl VoltageParamsType {
    /// Creates a new `VoltageParamsType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `voltage_start` - Voltage at which to start charging, in Volts
    /// * `voltage_stop` - Voltage at which to stop charging, in Volts
    /// * `voltage_discharge_start` - Voltage at which to start discharging, in Volts
    /// * `voltage_discharge_stop` - Voltage at which to stop discharging, in Volts
    ///
    /// # Returns
    ///
    /// A new instance of `VoltageParamsType` with optional fields set to `None`
    pub fn new(
        voltage_start: f64,
        voltage_stop: f64,
        voltage_discharge_start: f64,
        voltage_discharge_stop: f64,
    ) -> Self {
        Self {
            voltage_start,
            voltage_stop,
            voltage_discharge_start,
            voltage_discharge_stop,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these voltage parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the voltage at which to start charging.
    ///
    /// # Returns
    ///
    /// The voltage at which to start charging, in Volts
    pub fn voltage_start(&self) -> f64 {
        self.voltage_start
    }

    /// Sets the voltage at which to start charging.
    ///
    /// # Arguments
    ///
    /// * `voltage_start` - Voltage at which to start charging, in Volts
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_voltage_start(&mut self, voltage_start: f64) -> &mut Self {
        self.voltage_start = voltage_start;
        self
    }

    /// Gets the voltage at which to stop charging.
    ///
    /// # Returns
    ///
    /// The voltage at which to stop charging, in Volts
    pub fn voltage_stop(&self) -> f64 {
        self.voltage_stop
    }

    /// Sets the voltage at which to stop charging.
    ///
    /// # Arguments
    ///
    /// * `voltage_stop` - Voltage at which to stop charging, in Volts
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_voltage_stop(&mut self, voltage_stop: f64) -> &mut Self {
        self.voltage_stop = voltage_stop;
        self
    }

    /// Gets the voltage at which to start discharging.
    ///
    /// # Returns
    ///
    /// The voltage at which to start discharging, in Volts
    pub fn voltage_discharge_start(&self) -> f64 {
        self.voltage_discharge_start
    }

    /// Sets the voltage at which to start discharging.
    ///
    /// # Arguments
    ///
    /// * `voltage_discharge_start` - Voltage at which to start discharging, in Volts
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_voltage_discharge_start(&mut self, voltage_discharge_start: f64) -> &mut Self {
        self.voltage_discharge_start = voltage_discharge_start;
        self
    }

    /// Gets the voltage at which to stop discharging.
    ///
    /// # Returns
    ///
    /// The voltage at which to stop discharging, in Volts
    pub fn voltage_discharge_stop(&self) -> f64 {
        self.voltage_discharge_stop
    }

    /// Sets the voltage at which to stop discharging.
    ///
    /// # Arguments
    ///
    /// * `voltage_discharge_stop` - Voltage at which to stop discharging, in Volts
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_voltage_discharge_stop(&mut self, voltage_discharge_stop: f64) -> &mut Self {
        self.voltage_discharge_stop = voltage_discharge_stop;
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
    /// * `custom_data` - Custom data for these voltage parameters, or None to clear
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
    fn test_new_voltage_params() {
        let voltage_start = 220.0;
        let voltage_stop = 240.0;
        let voltage_discharge_start = 250.0;
        let voltage_discharge_stop = 230.0;

        let params = VoltageParamsType::new(
            voltage_start,
            voltage_stop,
            voltage_discharge_start,
            voltage_discharge_stop,
        );

        assert_eq!(params.voltage_start(), voltage_start);
        assert_eq!(params.voltage_stop(), voltage_stop);
        assert_eq!(params.voltage_discharge_start(), voltage_discharge_start);
        assert_eq!(params.voltage_discharge_stop(), voltage_discharge_stop);
        assert_eq!(params.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let voltage_start = 220.0;
        let voltage_stop = 240.0;
        let voltage_discharge_start = 250.0;
        let voltage_discharge_stop = 230.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let params = VoltageParamsType::new(
            voltage_start,
            voltage_stop,
            voltage_discharge_start,
            voltage_discharge_stop,
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(params.voltage_start(), voltage_start);
        assert_eq!(params.voltage_stop(), voltage_stop);
        assert_eq!(params.voltage_discharge_start(), voltage_discharge_start);
        assert_eq!(params.voltage_discharge_stop(), voltage_discharge_stop);
        assert_eq!(params.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let voltage_start1 = 220.0;
        let voltage_stop1 = 240.0;
        let voltage_discharge_start1 = 250.0;
        let voltage_discharge_stop1 = 230.0;

        let voltage_start2 = 225.0;
        let voltage_stop2 = 245.0;
        let voltage_discharge_start2 = 255.0;
        let voltage_discharge_stop2 = 235.0;

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut params = VoltageParamsType::new(
            voltage_start1,
            voltage_stop1,
            voltage_discharge_start1,
            voltage_discharge_stop1,
        );

        params
            .set_voltage_start(voltage_start2)
            .set_voltage_stop(voltage_stop2)
            .set_voltage_discharge_start(voltage_discharge_start2)
            .set_voltage_discharge_stop(voltage_discharge_stop2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(params.voltage_start(), voltage_start2);
        assert_eq!(params.voltage_stop(), voltage_stop2);
        assert_eq!(params.voltage_discharge_start(), voltage_discharge_start2);
        assert_eq!(params.voltage_discharge_stop(), voltage_discharge_stop2);
        assert_eq!(params.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        params.set_custom_data(None);
        assert_eq!(params.custom_data(), None);
    }
}
