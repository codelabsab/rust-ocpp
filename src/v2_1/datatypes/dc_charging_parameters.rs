use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::CustomDataType;

/// EV DC charging parameters for ISO 15118-2
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DCChargingParametersType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Maximum current (in A) supported by the electric vehicle. Includes cable capacity.
    /// Relates to:
    /// *ISO 15118-2*: DC_EVChargeParameterType:EVMaximumCurrentLimit
    pub ev_max_current: f64,

    /// Maximum voltage supported by the electric vehicle.
    /// Relates to:
    /// *ISO 15118-2*: DC_EVChargeParameterType: EVMaximumVoltageLimit
    pub ev_max_voltage: f64,

    /// Maximum power (in W) supported by the electric vehicle. Required for DC charging.
    /// Relates to:
    /// *ISO 15118-2*: DC_EVChargeParameterType: EVMaximumPowerLimit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_max_power: Option<f64>,

    /// Capacity of the electric vehicle battery (in Wh).
    /// Relates to:
    /// *ISO 15118-2*: DC_EVChargeParameterType: EVEnergyCapacity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_energy_capacity: Option<f64>,

    /// Amount of energy requested (in Wh). This includes energy required for preconditioning.
    /// Relates to:
    /// *ISO 15118-2*: DC_EVChargeParameterType: EVEnergyRequest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_amount: Option<f64>,
}

impl DCChargingParametersType {
    /// Creates a new `DCChargingParametersType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `ev_max_voltage` - Maximum voltage supported by the electric vehicle
    /// * `ev_max_current` - Maximum current (in A) supported by the electric vehicle
    ///
    /// # Returns
    ///
    /// A new instance of `DCChargingParametersType` with optional fields set to `None`
    pub fn new(ev_max_voltage: f64, ev_max_current: f64) -> Self {
        Self {
            ev_max_voltage,
            ev_max_current,
            ev_max_power: None,
            ev_energy_capacity: None,
            energy_amount: None,
            custom_data: None,
        }
    }

    /// Sets the maximum power supported by the electric vehicle.
    ///
    /// # Arguments
    ///
    /// * `ev_max_power` - Maximum power (in W) supported by the electric vehicle
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_max_power(mut self, ev_max_power: f64) -> Self {
        self.ev_max_power = Some(ev_max_power);
        self
    }

    /// Sets the capacity of the electric vehicle battery.
    ///
    /// # Arguments
    ///
    /// * `ev_energy_capacity` - Capacity of the electric vehicle battery (in Wh)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_energy_capacity(mut self, ev_energy_capacity: f64) -> Self {
        self.ev_energy_capacity = Some(ev_energy_capacity);
        self
    }

    /// Sets the amount of energy requested.
    ///
    /// # Arguments
    ///
    /// * `energy_amount` - Amount of energy requested (in Wh)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_energy_amount(mut self, energy_amount: f64) -> Self {
        self.energy_amount = Some(energy_amount);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these charging parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the maximum voltage supported by the electric vehicle.
    ///
    /// # Returns
    ///
    /// The maximum voltage supported by the electric vehicle
    pub fn ev_max_voltage(&self) -> f64 {
        self.ev_max_voltage
    }

    /// Sets the maximum voltage supported by the electric vehicle.
    ///
    /// # Arguments
    ///
    /// * `ev_max_voltage` - Maximum voltage supported by the electric vehicle
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_max_voltage(&mut self, ev_max_voltage: f64) -> &mut Self {
        self.ev_max_voltage = ev_max_voltage;
        self
    }

    /// Gets the maximum current supported by the electric vehicle.
    ///
    /// # Returns
    ///
    /// The maximum current (in A) supported by the electric vehicle
    pub fn ev_max_current(&self) -> f64 {
        self.ev_max_current
    }

    /// Sets the maximum current supported by the electric vehicle.
    ///
    /// # Arguments
    ///
    /// * `ev_max_current` - Maximum current (in A) supported by the electric vehicle
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_max_current(&mut self, ev_max_current: f64) -> &mut Self {
        self.ev_max_current = ev_max_current;
        self
    }

    /// Gets the maximum power supported by the electric vehicle.
    ///
    /// # Returns
    ///
    /// An optional value representing the maximum power (in W) supported by the electric vehicle
    pub fn ev_max_power(&self) -> Option<f64> {
        self.ev_max_power
    }

    /// Sets the maximum power supported by the electric vehicle.
    ///
    /// # Arguments
    ///
    /// * `ev_max_power` - Maximum power (in W) supported by the electric vehicle, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_max_power(&mut self, ev_max_power: Option<f64>) -> &mut Self {
        self.ev_max_power = ev_max_power;
        self
    }

    /// Gets the capacity of the electric vehicle battery.
    ///
    /// # Returns
    ///
    /// An optional value representing the capacity of the electric vehicle battery (in Wh)
    pub fn ev_energy_capacity(&self) -> Option<f64> {
        self.ev_energy_capacity
    }

    /// Sets the capacity of the electric vehicle battery.
    ///
    /// # Arguments
    ///
    /// * `ev_energy_capacity` - Capacity of the electric vehicle battery (in Wh), or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_energy_capacity(&mut self, ev_energy_capacity: Option<f64>) -> &mut Self {
        self.ev_energy_capacity = ev_energy_capacity;
        self
    }

    /// Gets the amount of energy requested.
    ///
    /// # Returns
    ///
    /// An optional value representing the amount of energy requested (in Wh)
    pub fn energy_amount(&self) -> Option<f64> {
        self.energy_amount
    }

    /// Sets the amount of energy requested.
    ///
    /// # Arguments
    ///
    /// * `energy_amount` - Amount of energy requested (in Wh), or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_energy_amount(&mut self, energy_amount: Option<f64>) -> &mut Self {
        self.energy_amount = energy_amount;
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
    /// * `custom_data` - Custom data for these charging parameters, or None to clear
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
    fn test_new_dc_charging_parameters() {
        let params = DCChargingParametersType::new(500.0, 125.0);

        assert_eq!(params.ev_max_voltage(), 500.0);
        assert_eq!(params.ev_max_current(), 125.0);
        assert_eq!(params.ev_max_power(), None);
        assert_eq!(params.ev_energy_capacity(), None);
        assert_eq!(params.energy_amount(), None);
        assert_eq!(params.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let params = DCChargingParametersType::new(500.0, 125.0)
            .with_max_power(50000.0)
            .with_energy_capacity(75000.0)
            .with_energy_amount(20000.0)
            .with_custom_data(custom_data.clone());

        assert_eq!(params.ev_max_voltage(), 500.0);
        assert_eq!(params.ev_max_current(), 125.0);
        assert_eq!(params.ev_max_power(), Some(50000.0));
        assert_eq!(params.ev_energy_capacity(), Some(75000.0));
        assert_eq!(params.energy_amount(), Some(20000.0));
        assert_eq!(params.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut params = DCChargingParametersType::new(500.0, 125.0);

        params
            .set_ev_max_voltage(550.0)
            .set_ev_max_current(150.0)
            .set_ev_max_power(Some(60000.0))
            .set_ev_energy_capacity(Some(80000.0))
            .set_energy_amount(Some(25000.0))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(params.ev_max_voltage(), 550.0);
        assert_eq!(params.ev_max_current(), 150.0);
        assert_eq!(params.ev_max_power(), Some(60000.0));
        assert_eq!(params.ev_energy_capacity(), Some(80000.0));
        assert_eq!(params.energy_amount(), Some(25000.0));
        assert_eq!(params.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        params
            .set_ev_max_power(None)
            .set_ev_energy_capacity(None)
            .set_energy_amount(None)
            .set_custom_data(None);

        assert_eq!(params.ev_max_power(), None);
        assert_eq!(params.ev_energy_capacity(), None);
        assert_eq!(params.energy_amount(), None);
        assert_eq!(params.custom_data(), None);
    }
}
