use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, tariff_conditions_fixed::TariffConditionsFixedType};
use crate::v2_1::enumerations::EnergyTransferModeEnumType;

/// Conditions for a tariff.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffConditionsType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Mode of energy transfer for which this tariff applies.
    pub energy_transfer_mode: EnergyTransferModeEnumType,

    /// Optional. Fixed conditions for this tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<TariffConditionsFixedType>,

    /// Optional. Maximum power in kW that can be delivered under this tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_power: Option<f64>,

    /// Optional. Minimum power in kW that must be delivered under this tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_power: Option<f64>,

    /// Optional. Maximum duration in seconds that a charging session can last under this tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<i32>,

    /// Optional. Minimum duration in seconds that a charging session must last under this tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_duration: Option<i32>,
}

impl TariffConditionsType {
    /// Creates a new `TariffConditionsType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `energy_transfer_mode` - Mode of energy transfer for which this tariff applies
    ///
    /// # Returns
    ///
    /// A new instance of `TariffConditionsType` with optional fields set to `None`
    pub fn new(energy_transfer_mode: EnergyTransferModeEnumType) -> Self {
        Self {
            energy_transfer_mode,
            custom_data: None,
            fixed: None,
            max_power: None,
            min_power: None,
            max_duration: None,
            min_duration: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this tariff condition
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the fixed conditions.
    ///
    /// # Arguments
    ///
    /// * `fixed` - Fixed conditions for this tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_fixed(mut self, fixed: TariffConditionsFixedType) -> Self {
        self.fixed = Some(fixed);
        self
    }

    /// Sets the maximum power.
    ///
    /// # Arguments
    ///
    /// * `max_power` - Maximum power in kW that can be delivered under this tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_max_power(mut self, max_power: f64) -> Self {
        self.max_power = Some(max_power);
        self
    }

    /// Sets the minimum power.
    ///
    /// # Arguments
    ///
    /// * `min_power` - Minimum power in kW that must be delivered under this tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_min_power(mut self, min_power: f64) -> Self {
        self.min_power = Some(min_power);
        self
    }

    /// Sets the maximum duration.
    ///
    /// # Arguments
    ///
    /// * `max_duration` - Maximum duration in seconds that a charging session can last under this tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_max_duration(mut self, max_duration: i32) -> Self {
        self.max_duration = Some(max_duration);
        self
    }

    /// Sets the minimum duration.
    ///
    /// # Arguments
    ///
    /// * `min_duration` - Minimum duration in seconds that a charging session must last under this tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_min_duration(mut self, min_duration: i32) -> Self {
        self.min_duration = Some(min_duration);
        self
    }

    /// Gets the energy transfer mode.
    ///
    /// # Returns
    ///
    /// The mode of energy transfer for which this tariff applies
    pub fn energy_transfer_mode(&self) -> &EnergyTransferModeEnumType {
        &self.energy_transfer_mode
    }

    /// Sets the energy transfer mode.
    ///
    /// # Arguments
    ///
    /// * `energy_transfer_mode` - Mode of energy transfer for which this tariff applies
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_energy_transfer_mode(&mut self, energy_transfer_mode: EnergyTransferModeEnumType) -> &mut Self {
        self.energy_transfer_mode = energy_transfer_mode;
        self
    }

    /// Gets the fixed conditions.
    ///
    /// # Returns
    ///
    /// An optional reference to the fixed conditions for this tariff
    pub fn fixed(&self) -> Option<&TariffConditionsFixedType> {
        self.fixed.as_ref()
    }

    /// Sets the fixed conditions.
    ///
    /// # Arguments
    ///
    /// * `fixed` - Fixed conditions for this tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_fixed(&mut self, fixed: Option<TariffConditionsFixedType>) -> &mut Self {
        self.fixed = fixed;
        self
    }

    /// Gets the maximum power.
    ///
    /// # Returns
    ///
    /// An optional maximum power in kW that can be delivered under this tariff
    pub fn max_power(&self) -> Option<f64> {
        self.max_power
    }

    /// Sets the maximum power.
    ///
    /// # Arguments
    ///
    /// * `max_power` - Maximum power in kW that can be delivered under this tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_max_power(&mut self, max_power: Option<f64>) -> &mut Self {
        self.max_power = max_power;
        self
    }

    /// Gets the minimum power.
    ///
    /// # Returns
    ///
    /// An optional minimum power in kW that must be delivered under this tariff
    pub fn min_power(&self) -> Option<f64> {
        self.min_power
    }

    /// Sets the minimum power.
    ///
    /// # Arguments
    ///
    /// * `min_power` - Minimum power in kW that must be delivered under this tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_min_power(&mut self, min_power: Option<f64>) -> &mut Self {
        self.min_power = min_power;
        self
    }

    /// Gets the maximum duration.
    ///
    /// # Returns
    ///
    /// An optional maximum duration in seconds that a charging session can last under this tariff
    pub fn max_duration(&self) -> Option<i32> {
        self.max_duration
    }

    /// Sets the maximum duration.
    ///
    /// # Arguments
    ///
    /// * `max_duration` - Maximum duration in seconds that a charging session can last under this tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_max_duration(&mut self, max_duration: Option<i32>) -> &mut Self {
        self.max_duration = max_duration;
        self
    }

    /// Gets the minimum duration.
    ///
    /// # Returns
    ///
    /// An optional minimum duration in seconds that a charging session must last under this tariff
    pub fn min_duration(&self) -> Option<i32> {
        self.min_duration
    }

    /// Sets the minimum duration.
    ///
    /// # Arguments
    ///
    /// * `min_duration` - Minimum duration in seconds that a charging session must last under this tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_min_duration(&mut self, min_duration: Option<i32>) -> &mut Self {
        self.min_duration = min_duration;
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
    /// * `custom_data` - Custom data for this tariff condition, or None to clear
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
    fn test_new_tariff_conditions() {
        let energy_transfer_mode = EnergyTransferModeEnumType::DC;
        let tariff_conditions = TariffConditionsType::new(energy_transfer_mode.clone());

        assert_eq!(tariff_conditions.energy_transfer_mode(), &energy_transfer_mode);
        assert_eq!(tariff_conditions.fixed(), None);
        assert_eq!(tariff_conditions.max_power(), None);
        assert_eq!(tariff_conditions.min_power(), None);
        assert_eq!(tariff_conditions.max_duration(), None);
        assert_eq!(tariff_conditions.min_duration(), None);
        assert_eq!(tariff_conditions.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let energy_transfer_mode = EnergyTransferModeEnumType::DC;
        let fixed = TariffConditionsFixedType::new(10.0);
        let custom_data = CustomDataType::new("VendorX".to_string());
        let max_power = 22.0;
        let min_power = 3.7;
        let max_duration = 3600;
        let min_duration = 300;

        let tariff_conditions = TariffConditionsType::new(energy_transfer_mode.clone())
            .with_fixed(fixed.clone())
            .with_max_power(max_power)
            .with_min_power(min_power)
            .with_max_duration(max_duration)
            .with_min_duration(min_duration)
            .with_custom_data(custom_data.clone());

        assert_eq!(tariff_conditions.energy_transfer_mode(), &energy_transfer_mode);
        assert_eq!(tariff_conditions.fixed(), Some(&fixed));
        assert_eq!(tariff_conditions.max_power(), Some(max_power));
        assert_eq!(tariff_conditions.min_power(), Some(min_power));
        assert_eq!(tariff_conditions.max_duration(), Some(max_duration));
        assert_eq!(tariff_conditions.min_duration(), Some(min_duration));
        assert_eq!(tariff_conditions.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let energy_transfer_mode1 = EnergyTransferModeEnumType::DC;
        let energy_transfer_mode2 = EnergyTransferModeEnumType::ACBPT;
        let fixed = TariffConditionsFixedType::new(10.0);
        let custom_data = CustomDataType::new("VendorX".to_string());
        let max_power = 22.0;
        let min_power = 3.7;
        let max_duration = 3600;
        let min_duration = 300;

        let mut tariff_conditions = TariffConditionsType::new(energy_transfer_mode1);

        tariff_conditions
            .set_energy_transfer_mode(energy_transfer_mode2.clone())
            .set_fixed(Some(fixed.clone()))
            .set_max_power(Some(max_power))
            .set_min_power(Some(min_power))
            .set_max_duration(Some(max_duration))
            .set_min_duration(Some(min_duration))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(tariff_conditions.energy_transfer_mode(), &energy_transfer_mode2);
        assert_eq!(tariff_conditions.fixed(), Some(&fixed));
        assert_eq!(tariff_conditions.max_power(), Some(max_power));
        assert_eq!(tariff_conditions.min_power(), Some(min_power));
        assert_eq!(tariff_conditions.max_duration(), Some(max_duration));
        assert_eq!(tariff_conditions.min_duration(), Some(min_duration));
        assert_eq!(tariff_conditions.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        tariff_conditions
            .set_fixed(None)
            .set_max_power(None)
            .set_min_power(None)
            .set_max_duration(None)
            .set_min_duration(None)
            .set_custom_data(None);

        assert_eq!(tariff_conditions.fixed(), None);
        assert_eq!(tariff_conditions.max_power(), None);
        assert_eq!(tariff_conditions.min_power(), None);
        assert_eq!(tariff_conditions.max_duration(), None);
        assert_eq!(tariff_conditions.min_duration(), None);
        assert_eq!(tariff_conditions.custom_data(), None);
    }
}
