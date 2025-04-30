use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Contains EV battery parameters.
///
/// This type represents battery data for an electric vehicle, including state of charge (SoC)
/// at the start and end of charging, battery capacity, and rechargeable energy capacity.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BatteryDataType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Battery level at the start of charging as percentage of the battery capacity.
    ///
    /// Value range: 0-100 (percentage)
    #[validate(range(min = 0.0, max = 100.0))]
    pub charging_start_soc: f64,

    /// Required. Battery level at the end of charging as percentage of the battery capacity.
    ///
    /// Value range: 0-100 (percentage)
    #[validate(range(min = 0.0, max = 100.0))]
    pub charging_end_soc: f64,

    /// Required. Battery capacity in kWh.
    ///
    /// Total energy capacity of the battery
    pub battery_capacity: f64,

    /// Required. Battery energy capacity that can be recharged, in kWh.
    ///
    /// This may be different from the total battery capacity due to battery degradation or reserved capacity
    pub rechargeable_energy_capacity: f64,
}

impl BatteryDataType {
    /// Creates a new `BatteryDataType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `charging_start_soc` - Battery level at the start of charging (0-100%)
    /// * `charging_end_soc` - Battery level at the end of charging (0-100%)
    /// * `battery_capacity` - Total battery capacity in kWh
    /// * `rechargeable_energy_capacity` - Rechargeable energy capacity in kWh
    ///
    /// # Returns
    ///
    /// A new instance of `BatteryDataType` with optional fields set to `None`
    pub fn new(
        charging_start_soc: f64,
        charging_end_soc: f64,
        battery_capacity: f64,
        rechargeable_energy_capacity: f64,
    ) -> Self {
        Self {
            custom_data: None,
            charging_start_soc,
            charging_end_soc,
            battery_capacity,
            rechargeable_energy_capacity,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this battery data
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
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
    /// * `custom_data` - Custom data for this battery data, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the battery level at the start of charging.
    ///
    /// # Returns
    ///
    /// The battery level at the start of charging as a percentage (0-100%)
    pub fn charging_start_soc(&self) -> f64 {
        self.charging_start_soc
    }

    /// Sets the battery level at the start of charging.
    ///
    /// # Arguments
    ///
    /// * `charging_start_soc` - Battery level at the start of charging (0-100%)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_start_soc(&mut self, charging_start_soc: f64) -> &mut Self {
        self.charging_start_soc = charging_start_soc;
        self
    }

    /// Gets the battery level at the end of charging.
    ///
    /// # Returns
    ///
    /// The battery level at the end of charging as a percentage (0-100%)
    pub fn charging_end_soc(&self) -> f64 {
        self.charging_end_soc
    }

    /// Sets the battery level at the end of charging.
    ///
    /// # Arguments
    ///
    /// * `charging_end_soc` - Battery level at the end of charging (0-100%)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_end_soc(&mut self, charging_end_soc: f64) -> &mut Self {
        self.charging_end_soc = charging_end_soc;
        self
    }

    /// Gets the total battery capacity.
    ///
    /// # Returns
    ///
    /// The total battery capacity in kWh
    pub fn battery_capacity(&self) -> f64 {
        self.battery_capacity
    }

    /// Sets the total battery capacity.
    ///
    /// # Arguments
    ///
    /// * `battery_capacity` - Total battery capacity in kWh
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_battery_capacity(&mut self, battery_capacity: f64) -> &mut Self {
        self.battery_capacity = battery_capacity;
        self
    }

    /// Gets the rechargeable energy capacity.
    ///
    /// # Returns
    ///
    /// The rechargeable energy capacity in kWh
    pub fn rechargeable_energy_capacity(&self) -> f64 {
        self.rechargeable_energy_capacity
    }

    /// Sets the rechargeable energy capacity.
    ///
    /// # Arguments
    ///
    /// * `rechargeable_energy_capacity` - Rechargeable energy capacity in kWh
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_rechargeable_energy_capacity(&mut self, rechargeable_energy_capacity: f64) -> &mut Self {
        self.rechargeable_energy_capacity = rechargeable_energy_capacity;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_battery_data() {
        let battery_data = BatteryDataType::new(
            20.0,  // charging_start_soc
            80.0,  // charging_end_soc
            75.0,  // battery_capacity
            70.0,  // rechargeable_energy_capacity
        );

        assert_eq!(battery_data.charging_start_soc(), 20.0);
        assert_eq!(battery_data.charging_end_soc(), 80.0);
        assert_eq!(battery_data.battery_capacity(), 75.0);
        assert_eq!(battery_data.rechargeable_energy_capacity(), 70.0);
        assert_eq!(battery_data.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let battery_data = BatteryDataType::new(
            20.0,  // charging_start_soc
            80.0,  // charging_end_soc
            75.0,  // battery_capacity
            70.0,  // rechargeable_energy_capacity
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(battery_data.charging_start_soc(), 20.0);
        assert_eq!(battery_data.charging_end_soc(), 80.0);
        assert_eq!(battery_data.battery_capacity(), 75.0);
        assert_eq!(battery_data.rechargeable_energy_capacity(), 70.0);
        assert_eq!(battery_data.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut battery_data = BatteryDataType::new(
            20.0,  // charging_start_soc
            80.0,  // charging_end_soc
            75.0,  // battery_capacity
            70.0,  // rechargeable_energy_capacity
        );

        battery_data
            .set_charging_start_soc(25.0)
            .set_charging_end_soc(85.0)
            .set_battery_capacity(80.0)
            .set_rechargeable_energy_capacity(75.0)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(battery_data.charging_start_soc(), 25.0);
        assert_eq!(battery_data.charging_end_soc(), 85.0);
        assert_eq!(battery_data.battery_capacity(), 80.0);
        assert_eq!(battery_data.rechargeable_energy_capacity(), 75.0);
        assert_eq!(battery_data.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        battery_data.set_custom_data(None);
        assert_eq!(battery_data.custom_data(), None);
    }
}
