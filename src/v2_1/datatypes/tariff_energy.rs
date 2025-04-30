use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, tariff_energy_price::TariffEnergyPriceType};

/// Energy tariff structure defining energy costs.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffEnergyType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Energy price per kWh.
    pub energy_price: TariffEnergyPriceType,

    /// Optional. Maximum energy in kWh that can be charged under this tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_energy: Option<f64>,

    /// Optional. Minimum energy in kWh that must be charged under this tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_energy: Option<f64>,
}

impl TariffEnergyType {
    /// Creates a new `TariffEnergyType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `energy_price` - Energy price per kWh
    ///
    /// # Returns
    ///
    /// A new instance of `TariffEnergyType` with optional fields set to `None`
    pub fn new(energy_price: TariffEnergyPriceType) -> Self {
        Self {
            energy_price,
            custom_data: None,
            max_energy: None,
            min_energy: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this tariff energy
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the maximum energy.
    ///
    /// # Arguments
    ///
    /// * `max_energy` - Maximum energy in kWh that can be charged under this tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_max_energy(mut self, max_energy: f64) -> Self {
        self.max_energy = Some(max_energy);
        self
    }

    /// Sets the minimum energy.
    ///
    /// # Arguments
    ///
    /// * `min_energy` - Minimum energy in kWh that must be charged under this tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_min_energy(mut self, min_energy: f64) -> Self {
        self.min_energy = Some(min_energy);
        self
    }

    /// Gets the energy price.
    ///
    /// # Returns
    ///
    /// A reference to the energy price per kWh
    pub fn energy_price(&self) -> &TariffEnergyPriceType {
        &self.energy_price
    }

    /// Sets the energy price.
    ///
    /// # Arguments
    ///
    /// * `energy_price` - Energy price per kWh
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_energy_price(&mut self, energy_price: TariffEnergyPriceType) -> &mut Self {
        self.energy_price = energy_price;
        self
    }

    /// Gets the maximum energy.
    ///
    /// # Returns
    ///
    /// An optional maximum energy in kWh that can be charged under this tariff
    pub fn max_energy(&self) -> Option<f64> {
        self.max_energy
    }

    /// Sets the maximum energy.
    ///
    /// # Arguments
    ///
    /// * `max_energy` - Maximum energy in kWh that can be charged under this tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_max_energy(&mut self, max_energy: Option<f64>) -> &mut Self {
        self.max_energy = max_energy;
        self
    }

    /// Gets the minimum energy.
    ///
    /// # Returns
    ///
    /// An optional minimum energy in kWh that must be charged under this tariff
    pub fn min_energy(&self) -> Option<f64> {
        self.min_energy
    }

    /// Sets the minimum energy.
    ///
    /// # Arguments
    ///
    /// * `min_energy` - Minimum energy in kWh that must be charged under this tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_min_energy(&mut self, min_energy: Option<f64>) -> &mut Self {
        self.min_energy = min_energy;
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
    /// * `custom_data` - Custom data for this tariff energy, or None to clear
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
    fn test_new_tariff_energy() {
        let energy_price = TariffEnergyPriceType::new(10.0);
        let tariff_energy = TariffEnergyType::new(energy_price.clone());

        assert_eq!(tariff_energy.energy_price(), &energy_price);
        assert_eq!(tariff_energy.max_energy(), None);
        assert_eq!(tariff_energy.min_energy(), None);
        assert_eq!(tariff_energy.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let energy_price = TariffEnergyPriceType::new(10.0);
        let max_energy = 50.0;
        let min_energy = 5.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let tariff_energy = TariffEnergyType::new(energy_price.clone())
            .with_max_energy(max_energy)
            .with_min_energy(min_energy)
            .with_custom_data(custom_data.clone());

        assert_eq!(tariff_energy.energy_price(), &energy_price);
        assert_eq!(tariff_energy.max_energy(), Some(max_energy));
        assert_eq!(tariff_energy.min_energy(), Some(min_energy));
        assert_eq!(tariff_energy.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let energy_price1 = TariffEnergyPriceType::new(10.0);
        let energy_price2 = TariffEnergyPriceType::new(15.0);
        let max_energy = 50.0;
        let min_energy = 5.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut tariff_energy = TariffEnergyType::new(energy_price1);

        tariff_energy
            .set_energy_price(energy_price2.clone())
            .set_max_energy(Some(max_energy))
            .set_min_energy(Some(min_energy))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(tariff_energy.energy_price(), &energy_price2);
        assert_eq!(tariff_energy.max_energy(), Some(max_energy));
        assert_eq!(tariff_energy.min_energy(), Some(min_energy));
        assert_eq!(tariff_energy.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        tariff_energy
            .set_max_energy(None)
            .set_min_energy(None)
            .set_custom_data(None);

        assert_eq!(tariff_energy.max_energy(), None);
        assert_eq!(tariff_energy.min_energy(), None);
        assert_eq!(tariff_energy.custom_data(), None);
    }
}
