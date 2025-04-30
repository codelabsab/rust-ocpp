use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    custom_data::CustomDataType, tariff_conditions::TariffConditionsType,
    tariff_energy::TariffEnergyType, tariff_fixed::TariffFixedType, tariff_time::TariffTimeType,
};

/// A tariff defines price and price rules for charging sessions.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Unique identifier used to identify one or more tariffs.
    #[validate(length(max = 36))]
    pub id: String,

    /// Optional. Currency of the price in ISO 4217 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 3))]
    pub currency: Option<String>,

    /// Optional. Language identifier of the language used in the description fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 8))]
    pub language: Option<String>,

    /// Optional. Display text of the tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 100))]
    pub display_text: Option<String>,

    /// Required. Conditions under which this tariff applies.
    pub conditions: TariffConditionsType,

    /// Optional. Fixed costs of the tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<TariffFixedType>,

    /// Optional. Energy costs of the tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy: Option<TariffEnergyType>,

    /// Optional. Time costs of the tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<TariffTimeType>,
}

impl TariffType {
    /// Creates a new `TariffType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier used to identify one or more tariffs
    /// * `conditions` - Conditions under which this tariff applies
    ///
    /// # Returns
    ///
    /// A new instance of `TariffType` with optional fields set to `None`
    pub fn new(id: String, conditions: TariffConditionsType) -> Self {
        Self {
            id,
            conditions,
            custom_data: None,
            currency: None,
            language: None,
            display_text: None,
            fixed: None,
            energy: None,
            time: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the currency.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency of the price in ISO 4217 format
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    /// Sets the language.
    ///
    /// # Arguments
    ///
    /// * `language` - Language identifier of the language used in the description fields
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }

    /// Sets the display text.
    ///
    /// # Arguments
    ///
    /// * `display_text` - Display text of the tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_display_text(mut self, display_text: String) -> Self {
        self.display_text = Some(display_text);
        self
    }

    /// Sets the fixed costs.
    ///
    /// # Arguments
    ///
    /// * `fixed` - Fixed costs of the tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_fixed(mut self, fixed: TariffFixedType) -> Self {
        self.fixed = Some(fixed);
        self
    }

    /// Sets the energy costs.
    ///
    /// # Arguments
    ///
    /// * `energy` - Energy costs of the tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_energy(mut self, energy: TariffEnergyType) -> Self {
        self.energy = Some(energy);
        self
    }

    /// Sets the time costs.
    ///
    /// # Arguments
    ///
    /// * `time` - Time costs of the tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_time(mut self, time: TariffTimeType) -> Self {
        self.time = Some(time);
        self
    }

    /// Gets the tariff identifier.
    ///
    /// # Returns
    ///
    /// The unique identifier used to identify one or more tariffs
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Sets the tariff identifier.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier used to identify one or more tariffs
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: String) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets the conditions.
    ///
    /// # Returns
    ///
    /// A reference to the conditions under which this tariff applies
    pub fn conditions(&self) -> &TariffConditionsType {
        &self.conditions
    }

    /// Sets the conditions.
    ///
    /// # Arguments
    ///
    /// * `conditions` - Conditions under which this tariff applies
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_conditions(&mut self, conditions: TariffConditionsType) -> &mut Self {
        self.conditions = conditions;
        self
    }

    /// Gets the currency.
    ///
    /// # Returns
    ///
    /// An optional reference to the currency of the price in ISO 4217 format
    pub fn currency(&self) -> Option<&str> {
        self.currency.as_deref()
    }

    /// Sets the currency.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency of the price in ISO 4217 format, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_currency(&mut self, currency: Option<String>) -> &mut Self {
        self.currency = currency;
        self
    }

    /// Gets the language.
    ///
    /// # Returns
    ///
    /// An optional reference to the language identifier of the language used in the description fields
    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    /// Sets the language.
    ///
    /// # Arguments
    ///
    /// * `language` - Language identifier of the language used in the description fields, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_language(&mut self, language: Option<String>) -> &mut Self {
        self.language = language;
        self
    }

    /// Gets the display text.
    ///
    /// # Returns
    ///
    /// An optional reference to the display text of the tariff
    pub fn display_text(&self) -> Option<&str> {
        self.display_text.as_deref()
    }

    /// Sets the display text.
    ///
    /// # Arguments
    ///
    /// * `display_text` - Display text of the tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_display_text(&mut self, display_text: Option<String>) -> &mut Self {
        self.display_text = display_text;
        self
    }

    /// Gets the fixed costs.
    ///
    /// # Returns
    ///
    /// An optional reference to the fixed costs of the tariff
    pub fn fixed(&self) -> Option<&TariffFixedType> {
        self.fixed.as_ref()
    }

    /// Sets the fixed costs.
    ///
    /// # Arguments
    ///
    /// * `fixed` - Fixed costs of the tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_fixed(&mut self, fixed: Option<TariffFixedType>) -> &mut Self {
        self.fixed = fixed;
        self
    }

    /// Gets the energy costs.
    ///
    /// # Returns
    ///
    /// An optional reference to the energy costs of the tariff
    pub fn energy(&self) -> Option<&TariffEnergyType> {
        self.energy.as_ref()
    }

    /// Sets the energy costs.
    ///
    /// # Arguments
    ///
    /// * `energy` - Energy costs of the tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_energy(&mut self, energy: Option<TariffEnergyType>) -> &mut Self {
        self.energy = energy;
        self
    }

    /// Gets the time costs.
    ///
    /// # Returns
    ///
    /// An optional reference to the time costs of the tariff
    pub fn time(&self) -> Option<&TariffTimeType> {
        self.time.as_ref()
    }

    /// Sets the time costs.
    ///
    /// # Arguments
    ///
    /// * `time` - Time costs of the tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_time(&mut self, time: Option<TariffTimeType>) -> &mut Self {
        self.time = time;
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
    /// * `custom_data` - Custom data for this tariff, or None to clear
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
    use crate::v2_1::{
        datatypes::{
            tariff_energy_price::TariffEnergyPriceType,
            tariff_fixed_price::TariffFixedPriceType,
            tariff_time_price::TariffTimePriceType,
        },
        enumerations::EnergyTransferModeEnumType,
    };

    #[test]
    fn test_new_tariff() {
        let id = "tariff-123".to_string();
        let conditions = TariffConditionsType::new(EnergyTransferModeEnumType::DC);
        let tariff = TariffType::new(id.clone(), conditions.clone());

        assert_eq!(tariff.id(), id);
        assert_eq!(tariff.conditions(), &conditions);
        assert_eq!(tariff.currency(), None);
        assert_eq!(tariff.language(), None);
        assert_eq!(tariff.display_text(), None);
        assert_eq!(tariff.fixed(), None);
        assert_eq!(tariff.energy(), None);
        assert_eq!(tariff.time(), None);
        assert_eq!(tariff.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let id = "tariff-123".to_string();
        let conditions = TariffConditionsType::new(EnergyTransferModeEnumType::DC);
        let currency = "EUR".to_string();
        let language = "en".to_string();
        let display_text = "Standard Tariff".to_string();
        let fixed = TariffFixedType::new(TariffFixedPriceType::new(10.0));
        let energy = TariffEnergyType::new(TariffEnergyPriceType::new(0.25));
        let time = TariffTimeType::new(TariffTimePriceType::new(5.0));
        let custom_data = CustomDataType::new("VendorX".to_string());

        let tariff = TariffType::new(id.clone(), conditions.clone())
            .with_currency(currency.clone())
            .with_language(language.clone())
            .with_display_text(display_text.clone())
            .with_fixed(fixed.clone())
            .with_energy(energy.clone())
            .with_time(time.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(tariff.id(), id);
        assert_eq!(tariff.conditions(), &conditions);
        assert_eq!(tariff.currency(), Some(currency.as_str()));
        assert_eq!(tariff.language(), Some(language.as_str()));
        assert_eq!(tariff.display_text(), Some(display_text.as_str()));
        assert_eq!(tariff.fixed(), Some(&fixed));
        assert_eq!(tariff.energy(), Some(&energy));
        assert_eq!(tariff.time(), Some(&time));
        assert_eq!(tariff.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let id1 = "tariff-123".to_string();
        let conditions1 = TariffConditionsType::new(EnergyTransferModeEnumType::DC);
        let id2 = "tariff-456".to_string();
        let conditions2 = TariffConditionsType::new(EnergyTransferModeEnumType::ACBPT);
        let currency = "EUR".to_string();
        let language = "en".to_string();
        let display_text = "Standard Tariff".to_string();
        let fixed = TariffFixedType::new(TariffFixedPriceType::new(10.0));
        let energy = TariffEnergyType::new(TariffEnergyPriceType::new(0.25));
        let time = TariffTimeType::new(TariffTimePriceType::new(5.0));
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut tariff = TariffType::new(id1, conditions1);

        tariff
            .set_id(id2.clone())
            .set_conditions(conditions2.clone())
            .set_currency(Some(currency.clone()))
            .set_language(Some(language.clone()))
            .set_display_text(Some(display_text.clone()))
            .set_fixed(Some(fixed.clone()))
            .set_energy(Some(energy.clone()))
            .set_time(Some(time.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(tariff.id(), id2);
        assert_eq!(tariff.conditions(), &conditions2);
        assert_eq!(tariff.currency(), Some(currency.as_str()));
        assert_eq!(tariff.language(), Some(language.as_str()));
        assert_eq!(tariff.display_text(), Some(display_text.as_str()));
        assert_eq!(tariff.fixed(), Some(&fixed));
        assert_eq!(tariff.energy(), Some(&energy));
        assert_eq!(tariff.time(), Some(&time));
        assert_eq!(tariff.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        tariff
            .set_currency(None)
            .set_language(None)
            .set_display_text(None)
            .set_fixed(None)
            .set_energy(None)
            .set_time(None)
            .set_custom_data(None);

        assert_eq!(tariff.currency(), None);
        assert_eq!(tariff.language(), None);
        assert_eq!(tariff.display_text(), None);
        assert_eq!(tariff.fixed(), None);
        assert_eq!(tariff.energy(), None);
        assert_eq!(tariff.time(), None);
        assert_eq!(tariff.custom_data(), None);
    }
}
