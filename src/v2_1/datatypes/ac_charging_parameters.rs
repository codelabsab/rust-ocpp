use crate::v2_1::datatypes::custom_data::CustomDataType;
use serde::{Deserialize, Serialize};

/// EV AC charging parameters for ISO 15118-2
///
/// Contains parameters specific to AC charging according to ISO 15118-2.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ACChargingParametersType {
    /// Amount of energy requested (in Wh). This includes energy required for preconditioning.
    /// Relates to:
    /// *ISO 15118-2*: AC_EVChargeParameterType: EAmount
    /// *ISO 15118-20*: Dynamic/Scheduled_SEReqControlModeType: EVTargetEnergyRequest
    pub energy_amount: f64,

    /// Minimum current (amps) supported by the electric vehicle (per phase).
    /// Relates to:
    /// *ISO 15118-2*: AC_EVChargeParameterType: EVMinCurrent
    pub ev_min_current: f64,

    /// Maximum current (amps) supported by the electric vehicle (per phase). Includes cable capacity.
    /// Relates to:
    /// *ISO 15118-2*: AC_EVChargeParameterType: EVMaxCurrent
    pub ev_max_current: f64,

    /// Maximum voltage supported by the electric vehicle.
    /// Relates to:
    /// *ISO 15118-2*: AC_EVChargeParameterType: EVMaxVoltage
    pub ev_max_voltage: f64,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl ACChargingParametersType {
    /// Creates a new `ACChargingParametersType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `energy_amount` - Amount of energy requested (in Wh)
    /// * `ev_min_current` - Minimum current (amps) supported by the electric vehicle (per phase)
    /// * `ev_max_current` - Maximum current (amps) supported by the electric vehicle (per phase)
    /// * `ev_max_voltage` - Maximum voltage supported by the electric vehicle
    ///
    /// # Returns
    ///
    /// A new instance of `ACChargingParametersType` with optional fields set to `None`
    pub fn new(
        energy_amount: f64,
        ev_min_current: f64,
        ev_max_current: f64,
        ev_max_voltage: f64,
    ) -> Self {
        Self {
            energy_amount,
            ev_min_current,
            ev_max_current,
            ev_max_voltage,
            custom_data: None,
        }
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

    /// Gets the energy amount.
    ///
    /// # Returns
    ///
    /// The amount of energy requested (in Wh)
    pub fn energy_amount(&self) -> f64 {
        self.energy_amount
    }

    /// Sets the energy amount.
    ///
    /// # Arguments
    ///
    /// * `energy_amount` - Amount of energy requested (in Wh)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_energy_amount(&mut self, energy_amount: f64) -> &mut Self {
        self.energy_amount = energy_amount;
        self
    }

    /// Gets the minimum current supported by the electric vehicle.
    ///
    /// # Returns
    ///
    /// The minimum current (amps) supported by the electric vehicle (per phase)
    pub fn ev_min_current(&self) -> f64 {
        self.ev_min_current
    }

    /// Sets the minimum current supported by the electric vehicle.
    ///
    /// # Arguments
    ///
    /// * `ev_min_current` - Minimum current (amps) supported by the electric vehicle (per phase)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_min_current(&mut self, ev_min_current: f64) -> &mut Self {
        self.ev_min_current = ev_min_current;
        self
    }

    /// Gets the maximum current supported by the electric vehicle.
    ///
    /// # Returns
    ///
    /// The maximum current (amps) supported by the electric vehicle (per phase)
    pub fn ev_max_current(&self) -> f64 {
        self.ev_max_current
    }

    /// Sets the maximum current supported by the electric vehicle.
    ///
    /// # Arguments
    ///
    /// * `ev_max_current` - Maximum current (amps) supported by the electric vehicle (per phase)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_max_current(&mut self, ev_max_current: f64) -> &mut Self {
        self.ev_max_current = ev_max_current;
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
    fn test_new_ac_charging_parameters() {
        let params = ACChargingParametersType::new(
            10000.0, // energy_amount
            10.0,    // ev_min_current
            32.0,    // ev_max_current
            400.0,   // ev_max_voltage
        );

        assert_eq!(params.energy_amount(), 10000.0);
        assert_eq!(params.ev_min_current(), 10.0);
        assert_eq!(params.ev_max_current(), 32.0);
        assert_eq!(params.ev_max_voltage(), 400.0);
        assert_eq!(params.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let params = ACChargingParametersType::new(
            10000.0, // energy_amount
            10.0,    // ev_min_current
            32.0,    // ev_max_current
            400.0,   // ev_max_voltage
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(params.energy_amount(), 10000.0);
        assert_eq!(params.ev_min_current(), 10.0);
        assert_eq!(params.ev_max_current(), 32.0);
        assert_eq!(params.ev_max_voltage(), 400.0);
        assert_eq!(params.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut params = ACChargingParametersType::new(
            10000.0, // energy_amount
            10.0,    // ev_min_current
            32.0,    // ev_max_current
            400.0,   // ev_max_voltage
        );

        params
            .set_energy_amount(15000.0)
            .set_ev_min_current(15.0)
            .set_ev_max_current(40.0)
            .set_ev_max_voltage(415.0)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(params.energy_amount(), 15000.0);
        assert_eq!(params.ev_min_current(), 15.0);
        assert_eq!(params.ev_max_current(), 40.0);
        assert_eq!(params.ev_max_voltage(), 415.0);
        assert_eq!(params.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        params.set_custom_data(None);
        assert_eq!(params.custom_data(), None);
    }
}
