use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Tax rate structure defining tax rate details.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TaxRateType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Tax rate in percent.
    pub tax_rate: f64,

    /// Required. Tax rate name.
    #[validate(length(max = 20))]
    pub tax_rate_name: String,
}

impl TaxRateType {
    /// Creates a new `TaxRateType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `tax_rate` - Tax rate in percent
    /// * `tax_rate_name` - Tax rate name
    ///
    /// # Returns
    ///
    /// A new instance of `TaxRateType` with optional fields set to `None`
    pub fn new(tax_rate: f64, tax_rate_name: String) -> Self {
        Self {
            tax_rate,
            tax_rate_name,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this tax rate
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the tax rate.
    ///
    /// # Returns
    ///
    /// The tax rate in percent
    pub fn tax_rate(&self) -> f64 {
        self.tax_rate
    }

    /// Sets the tax rate.
    ///
    /// # Arguments
    ///
    /// * `tax_rate` - Tax rate in percent
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_tax_rate(&mut self, tax_rate: f64) -> &mut Self {
        self.tax_rate = tax_rate;
        self
    }

    /// Gets the tax rate name.
    ///
    /// # Returns
    ///
    /// The tax rate name
    pub fn tax_rate_name(&self) -> &str {
        &self.tax_rate_name
    }

    /// Sets the tax rate name.
    ///
    /// # Arguments
    ///
    /// * `tax_rate_name` - Tax rate name
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_tax_rate_name(&mut self, tax_rate_name: String) -> &mut Self {
        self.tax_rate_name = tax_rate_name;
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
    /// * `custom_data` - Custom data for this tax rate, or None to clear
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
    fn test_new_tax_rate() {
        let tax_rate = 21.0;
        let tax_rate_name = "VAT".to_string();

        let tax_rate_type = TaxRateType::new(tax_rate, tax_rate_name.clone());

        assert_eq!(tax_rate_type.tax_rate(), tax_rate);
        assert_eq!(tax_rate_type.tax_rate_name(), tax_rate_name);
        assert_eq!(tax_rate_type.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let tax_rate = 21.0;
        let tax_rate_name = "VAT".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let tax_rate_type = TaxRateType::new(tax_rate, tax_rate_name.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(tax_rate_type.tax_rate(), tax_rate);
        assert_eq!(tax_rate_type.tax_rate_name(), tax_rate_name);
        assert_eq!(tax_rate_type.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let tax_rate1 = 21.0;
        let tax_rate_name1 = "VAT".to_string();
        let tax_rate2 = 15.0;
        let tax_rate_name2 = "GST".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut tax_rate_type = TaxRateType::new(tax_rate1, tax_rate_name1);

        tax_rate_type
            .set_tax_rate(tax_rate2)
            .set_tax_rate_name(tax_rate_name2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(tax_rate_type.tax_rate(), tax_rate2);
        assert_eq!(tax_rate_type.tax_rate_name(), tax_rate_name2);
        assert_eq!(tax_rate_type.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        tax_rate_type.set_custom_data(None);
        assert_eq!(tax_rate_type.custom_data(), None);
    }
}
