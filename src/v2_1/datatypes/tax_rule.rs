use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, tax_rate::TaxRateType};

/// Tax rule structure defining tax rules for a tariff.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TaxRuleType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Tax rate for this tax rule.
    pub tax_rate: TaxRateType,

    /// Optional. Type of energy for which this tax rule applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub energy_source_type: Option<String>,

    /// Optional. Type of consumption for which this tax rule applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub consumption_type: Option<String>,
}

impl TaxRuleType {
    /// Creates a new `TaxRuleType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `tax_rate` - Tax rate for this tax rule
    ///
    /// # Returns
    ///
    /// A new instance of `TaxRuleType` with optional fields set to `None`
    pub fn new(tax_rate: TaxRateType) -> Self {
        Self {
            tax_rate,
            custom_data: None,
            energy_source_type: None,
            consumption_type: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this tax rule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the energy source type.
    ///
    /// # Arguments
    ///
    /// * `energy_source_type` - Type of energy for which this tax rule applies
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_energy_source_type(mut self, energy_source_type: String) -> Self {
        self.energy_source_type = Some(energy_source_type);
        self
    }

    /// Sets the consumption type.
    ///
    /// # Arguments
    ///
    /// * `consumption_type` - Type of consumption for which this tax rule applies
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_consumption_type(mut self, consumption_type: String) -> Self {
        self.consumption_type = Some(consumption_type);
        self
    }

    /// Gets the tax rate.
    ///
    /// # Returns
    ///
    /// A reference to the tax rate for this tax rule
    pub fn tax_rate(&self) -> &TaxRateType {
        &self.tax_rate
    }

    /// Sets the tax rate.
    ///
    /// # Arguments
    ///
    /// * `tax_rate` - Tax rate for this tax rule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_tax_rate(&mut self, tax_rate: TaxRateType) -> &mut Self {
        self.tax_rate = tax_rate;
        self
    }

    /// Gets the energy source type.
    ///
    /// # Returns
    ///
    /// An optional reference to the type of energy for which this tax rule applies
    pub fn energy_source_type(&self) -> Option<&str> {
        self.energy_source_type.as_deref()
    }

    /// Sets the energy source type.
    ///
    /// # Arguments
    ///
    /// * `energy_source_type` - Type of energy for which this tax rule applies, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_energy_source_type(&mut self, energy_source_type: Option<String>) -> &mut Self {
        self.energy_source_type = energy_source_type;
        self
    }

    /// Gets the consumption type.
    ///
    /// # Returns
    ///
    /// An optional reference to the type of consumption for which this tax rule applies
    pub fn consumption_type(&self) -> Option<&str> {
        self.consumption_type.as_deref()
    }

    /// Sets the consumption type.
    ///
    /// # Arguments
    ///
    /// * `consumption_type` - Type of consumption for which this tax rule applies, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_consumption_type(&mut self, consumption_type: Option<String>) -> &mut Self {
        self.consumption_type = consumption_type;
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
    /// * `custom_data` - Custom data for this tax rule, or None to clear
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
    fn test_new_tax_rule() {
        let tax_rate = TaxRateType::new(Decimal::new(200, 1), "VAT".to_string()); // 20.0
        let tax_rule = TaxRuleType::new(tax_rate.clone());

        assert_eq!(tax_rule.tax_rate(), &tax_rate);
        assert_eq!(tax_rule.energy_source_type(), None);
        assert_eq!(tax_rule.consumption_type(), None);
        assert_eq!(tax_rule.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let tax_rate = TaxRateType::new(Decimal::new(200, 1), "VAT".to_string()); // 20.0
        let energy_source_type = "Solar".to_string();
        let consumption_type = "Residential".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let tax_rule = TaxRuleType::new(tax_rate.clone())
            .with_energy_source_type(energy_source_type.clone())
            .with_consumption_type(consumption_type.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(tax_rule.tax_rate(), &tax_rate);
        assert_eq!(
            tax_rule.energy_source_type(),
            Some(energy_source_type.as_str())
        );
        assert_eq!(tax_rule.consumption_type(), Some(consumption_type.as_str()));
        assert_eq!(tax_rule.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let tax_rate1 = TaxRateType::new(Decimal::new(200, 1), "VAT".to_string()); // 20.0
        let tax_rate2 = TaxRateType::new(Decimal::new(200, 1), "GST".to_string()); // 20.0
        let energy_source_type = "Solar".to_string();
        let consumption_type = "Residential".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut tax_rule = TaxRuleType::new(tax_rate1);

        tax_rule
            .set_tax_rate(tax_rate2.clone())
            .set_energy_source_type(Some(energy_source_type.clone()))
            .set_consumption_type(Some(consumption_type.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(tax_rule.tax_rate(), &tax_rate2);
        assert_eq!(
            tax_rule.energy_source_type(),
            Some(energy_source_type.as_str())
        );
        assert_eq!(tax_rule.consumption_type(), Some(consumption_type.as_str()));
        assert_eq!(tax_rule.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        tax_rule
            .set_energy_source_type(None)
            .set_consumption_type(None)
            .set_custom_data(None);

        assert_eq!(tax_rule.energy_source_type(), None);
        assert_eq!(tax_rule.consumption_type(), None);
        assert_eq!(tax_rule.custom_data(), None);
    }
}
