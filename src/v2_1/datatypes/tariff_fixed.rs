use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, tariff_fixed_price::TariffFixedPriceType};

/// Fixed tariff structure defining fixed costs.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffFixedType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Fixed price per charging session.
    pub fixed_price: TariffFixedPriceType,
}

impl TariffFixedType {
    /// Creates a new `TariffFixedType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `fixed_price` - Fixed price per charging session
    ///
    /// # Returns
    ///
    /// A new instance of `TariffFixedType` with optional fields set to `None`
    pub fn new(fixed_price: TariffFixedPriceType) -> Self {
        Self {
            fixed_price,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this tariff fixed
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the fixed price.
    ///
    /// # Returns
    ///
    /// A reference to the fixed price per charging session
    pub fn fixed_price(&self) -> &TariffFixedPriceType {
        &self.fixed_price
    }

    /// Sets the fixed price.
    ///
    /// # Arguments
    ///
    /// * `fixed_price` - Fixed price per charging session
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_fixed_price(&mut self, fixed_price: TariffFixedPriceType) -> &mut Self {
        self.fixed_price = fixed_price;
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
    /// * `custom_data` - Custom data for this tariff fixed, or None to clear
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
    fn test_new_tariff_fixed() {
        let fixed_price = TariffFixedPriceType::new(10.0);
        let tariff_fixed = TariffFixedType::new(fixed_price.clone());

        assert_eq!(tariff_fixed.fixed_price(), &fixed_price);
        assert_eq!(tariff_fixed.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let fixed_price = TariffFixedPriceType::new(10.0);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let tariff_fixed =
            TariffFixedType::new(fixed_price.clone()).with_custom_data(custom_data.clone());

        assert_eq!(tariff_fixed.fixed_price(), &fixed_price);
        assert_eq!(tariff_fixed.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let fixed_price1 = TariffFixedPriceType::new(10.0);
        let fixed_price2 = TariffFixedPriceType::new(15.0);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut tariff_fixed = TariffFixedType::new(fixed_price1);

        tariff_fixed
            .set_fixed_price(fixed_price2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(tariff_fixed.fixed_price(), &fixed_price2);
        assert_eq!(tariff_fixed.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        tariff_fixed.set_custom_data(None);

        assert_eq!(tariff_fixed.custom_data(), None);
    }
}
