use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// V2X charging parameters for an ISO 15118-20 session.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct V2XChargingParametersType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Maximum discharge power in W (DC) or VA (AC).
    pub ev_max_discharge_power: f64,

    /// Required. Maximum discharge current in A.
    pub ev_max_discharge_current: f64,

    /// Required. Maximum voltage at which the EV can discharge.
    pub ev_max_voltage: f64,

    /// Optional. Minimum voltage at which the EV can discharge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_min_voltage: Option<f64>,

    /// Optional. Minimum discharge current that the EV needs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_min_discharge_current: Option<f64>,

    /// Optional. Minimum discharge power that the EV needs, in W (DC) or VA (AC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_min_discharge_power: Option<f64>,
}

impl V2XChargingParametersType {
    /// Creates a new `V2XChargingParametersType` with the required fields.
    ///
    /// # Arguments
    ///
    /// * `ev_max_discharge_power` - Maximum discharge power in W (DC) or VA (AC)
    /// * `ev_max_discharge_current` - Maximum discharge current in A
    /// * `ev_max_voltage` - Maximum voltage at which the EV can discharge
    ///
    /// # Returns
    ///
    /// A new `V2XChargingParametersType` instance with optional fields set to `None`
    pub fn new(ev_max_discharge_power: f64, ev_max_discharge_current: f64, ev_max_voltage: f64) -> Self {
        Self {
            custom_data: None,
            ev_max_discharge_power,
            ev_max_discharge_current,
            ev_max_voltage,
            ev_min_voltage: None,
            ev_min_discharge_current: None,
            ev_min_discharge_power: None,
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
    /// The modified `V2XChargingParametersType` instance
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the minimum voltage field.
    ///
    /// # Arguments
    ///
    /// * `ev_min_voltage` - Minimum voltage at which the EV can discharge
    ///
    /// # Returns
    ///
    /// The modified `V2XChargingParametersType` instance
    pub fn with_ev_min_voltage(mut self, ev_min_voltage: f64) -> Self {
        self.ev_min_voltage = Some(ev_min_voltage);
        self
    }

    /// Sets the minimum discharge current field.
    ///
    /// # Arguments
    ///
    /// * `ev_min_discharge_current` - Minimum discharge current that the EV needs
    ///
    /// # Returns
    ///
    /// The modified `V2XChargingParametersType` instance
    pub fn with_ev_min_discharge_current(mut self, ev_min_discharge_current: f64) -> Self {
        self.ev_min_discharge_current = Some(ev_min_discharge_current);
        self
    }

    /// Sets the minimum discharge power field.
    ///
    /// # Arguments
    ///
    /// * `ev_min_discharge_power` - Minimum discharge power that the EV needs, in W (DC) or VA (AC)
    ///
    /// # Returns
    ///
    /// The modified `V2XChargingParametersType` instance
    pub fn with_ev_min_discharge_power(mut self, ev_min_discharge_power: f64) -> Self {
        self.ev_min_discharge_power = Some(ev_min_discharge_power);
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
    /// The modified `V2XChargingParametersType` instance
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the maximum discharge power.
    ///
    /// # Returns
    ///
    /// The maximum discharge power in W (DC) or VA (AC)
    pub fn ev_max_discharge_power(&self) -> f64 {
        self.ev_max_discharge_power
    }

    /// Sets the maximum discharge power.
    ///
    /// # Arguments
    ///
    /// * `ev_max_discharge_power` - Maximum discharge power in W (DC) or VA (AC)
    ///
    /// # Returns
    ///
    /// The modified `V2XChargingParametersType` instance
    pub fn set_ev_max_discharge_power(&mut self, ev_max_discharge_power: f64) -> &mut Self {
        self.ev_max_discharge_power = ev_max_discharge_power;
        self
    }

    /// Gets the maximum discharge current.
    ///
    /// # Returns
    ///
    /// The maximum discharge current in A
    pub fn ev_max_discharge_current(&self) -> f64 {
        self.ev_max_discharge_current
    }

    /// Sets the maximum discharge current.
    ///
    /// # Arguments
    ///
    /// * `ev_max_discharge_current` - Maximum discharge current in A
    ///
    /// # Returns
    ///
    /// The modified `V2XChargingParametersType` instance
    pub fn set_ev_max_discharge_current(&mut self, ev_max_discharge_current: f64) -> &mut Self {
        self.ev_max_discharge_current = ev_max_discharge_current;
        self
    }

    /// Gets the maximum voltage.
    ///
    /// # Returns
    ///
    /// The maximum voltage at which the EV can discharge
    pub fn ev_max_voltage(&self) -> f64 {
        self.ev_max_voltage
    }

    /// Sets the maximum voltage.
    ///
    /// # Arguments
    ///
    /// * `ev_max_voltage` - Maximum voltage at which the EV can discharge
    ///
    /// # Returns
    ///
    /// The modified `V2XChargingParametersType` instance
    pub fn set_ev_max_voltage(&mut self, ev_max_voltage: f64) -> &mut Self {
        self.ev_max_voltage = ev_max_voltage;
        self
    }

    /// Gets the minimum voltage.
    ///
    /// # Returns
    ///
    /// The minimum voltage at which the EV can discharge, if specified
    pub fn ev_min_voltage(&self) -> Option<f64> {
        self.ev_min_voltage
    }

    /// Sets the minimum voltage.
    ///
    /// # Arguments
    ///
    /// * `ev_min_voltage` - Minimum voltage at which the EV can discharge
    ///
    /// # Returns
    ///
    /// The modified `V2XChargingParametersType` instance
    pub fn set_ev_min_voltage(&mut self, ev_min_voltage: Option<f64>) -> &mut Self {
        self.ev_min_voltage = ev_min_voltage;
        self
    }

    /// Gets the minimum discharge current.
    ///
    /// # Returns
    ///
    /// The minimum discharge current that the EV needs, if specified
    pub fn ev_min_discharge_current(&self) -> Option<f64> {
        self.ev_min_discharge_current
    }

    /// Sets the minimum discharge current.
    ///
    /// # Arguments
    ///
    /// * `ev_min_discharge_current` - Minimum discharge current that the EV needs
    ///
    /// # Returns
    ///
    /// The modified `V2XChargingParametersType` instance
    pub fn set_ev_min_discharge_current(&mut self, ev_min_discharge_current: Option<f64>) -> &mut Self {
        self.ev_min_discharge_current = ev_min_discharge_current;
        self
    }

    /// Gets the minimum discharge power.
    ///
    /// # Returns
    ///
    /// The minimum discharge power that the EV needs, in W (DC) or VA (AC), if specified
    pub fn ev_min_discharge_power(&self) -> Option<f64> {
        self.ev_min_discharge_power
    }

    /// Sets the minimum discharge power.
    ///
    /// # Arguments
    ///
    /// * `ev_min_discharge_power` - Minimum discharge power that the EV needs, in W (DC) or VA (AC)
    ///
    /// # Returns
    ///
    /// The modified `V2XChargingParametersType` instance
    pub fn set_ev_min_discharge_power(&mut self, ev_min_discharge_power: Option<f64>) -> &mut Self {
        self.ev_min_discharge_power = ev_min_discharge_power;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2x_charging_parameters_new() {
        let params = V2XChargingParametersType::new(10000.0, 32.0, 400.0);
        
        assert_eq!(params.ev_max_discharge_power(), 10000.0);
        assert_eq!(params.ev_max_discharge_current(), 32.0);
        assert_eq!(params.ev_max_voltage(), 400.0);
        assert_eq!(params.ev_min_voltage(), None);
        assert_eq!(params.ev_min_discharge_current(), None);
        assert_eq!(params.ev_min_discharge_power(), None);
        assert_eq!(params.custom_data(), None);
    }

    #[test]
    fn test_v2x_charging_parameters_with_methods() {
        let custom_data = CustomDataType::new("Vendor".to_string());
        let params = V2XChargingParametersType::new(10000.0, 32.0, 400.0)
            .with_custom_data(custom_data.clone())
            .with_ev_min_voltage(200.0)
            .with_ev_min_discharge_current(10.0)
            .with_ev_min_discharge_power(2000.0);
        
        assert_eq!(params.ev_max_discharge_power(), 10000.0);
        assert_eq!(params.ev_max_discharge_current(), 32.0);
        assert_eq!(params.ev_max_voltage(), 400.0);
        assert_eq!(params.ev_min_voltage(), Some(200.0));
        assert_eq!(params.ev_min_discharge_current(), Some(10.0));
        assert_eq!(params.ev_min_discharge_power(), Some(2000.0));
        assert_eq!(params.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_v2x_charging_parameters_setters() {
        let mut params = V2XChargingParametersType::new(10000.0, 32.0, 400.0);
        
        params
            .set_ev_max_discharge_power(12000.0)
            .set_ev_max_discharge_current(40.0)
            .set_ev_max_voltage(415.0)
            .set_ev_min_voltage(Some(210.0))
            .set_ev_min_discharge_current(Some(12.0))
            .set_ev_min_discharge_power(Some(2400.0));
        
        assert_eq!(params.ev_max_discharge_power(), 12000.0);
        assert_eq!(params.ev_max_discharge_current(), 40.0);
        assert_eq!(params.ev_max_voltage(), 415.0);
        assert_eq!(params.ev_min_voltage(), Some(210.0));
        assert_eq!(params.ev_min_discharge_current(), Some(12.0));
        assert_eq!(params.ev_min_discharge_power(), Some(2400.0));
    }
}
