use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Structure to report total price.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TotalPriceType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The total price (including taxes) in the specified currency.
    pub price: f64,

    /// Optional. The total price (excluding taxes) in the specified currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_excl_tax: Option<f64>,

    /// Optional. The total price (including taxes) in the specified currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_incl_tax: Option<f64>,
}

impl TotalPriceType {
    /// Creates a new `TotalPriceType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `price` - The total price (including taxes) in the specified currency
    ///
    /// # Returns
    ///
    /// A new instance of `TotalPriceType` with optional fields set to `None`
    pub fn new(price: f64) -> Self {
        Self {
            price,
            custom_data: None,
            price_excl_tax: None,
            price_incl_tax: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this total price
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the price excluding tax.
    ///
    /// # Arguments
    ///
    /// * `price_excl_tax` - The total price (excluding taxes) in the specified currency
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_price_excl_tax(mut self, price_excl_tax: f64) -> Self {
        self.price_excl_tax = Some(price_excl_tax);
        self
    }

    /// Sets the price including tax.
    ///
    /// # Arguments
    ///
    /// * `price_incl_tax` - The total price (including taxes) in the specified currency
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_price_incl_tax(mut self, price_incl_tax: f64) -> Self {
        self.price_incl_tax = Some(price_incl_tax);
        self
    }

    /// Gets the price.
    ///
    /// # Returns
    ///
    /// The total price (including taxes) in the specified currency
    pub fn price(&self) -> f64 {
        self.price
    }

    /// Sets the price.
    ///
    /// # Arguments
    ///
    /// * `price` - The total price (including taxes) in the specified currency
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price(&mut self, price: f64) -> &mut Self {
        self.price = price;
        self
    }

    /// Gets the price excluding tax.
    ///
    /// # Returns
    ///
    /// An optional total price (excluding taxes) in the specified currency
    pub fn price_excl_tax(&self) -> Option<f64> {
        self.price_excl_tax
    }

    /// Sets the price excluding tax.
    ///
    /// # Arguments
    ///
    /// * `price_excl_tax` - The total price (excluding taxes) in the specified currency, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price_excl_tax(&mut self, price_excl_tax: Option<f64>) -> &mut Self {
        self.price_excl_tax = price_excl_tax;
        self
    }

    /// Gets the price including tax.
    ///
    /// # Returns
    ///
    /// An optional total price (including taxes) in the specified currency
    pub fn price_incl_tax(&self) -> Option<f64> {
        self.price_incl_tax
    }

    /// Sets the price including tax.
    ///
    /// # Arguments
    ///
    /// * `price_incl_tax` - The total price (including taxes) in the specified currency, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price_incl_tax(&mut self, price_incl_tax: Option<f64>) -> &mut Self {
        self.price_incl_tax = price_incl_tax;
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
    /// * `custom_data` - Custom data for this total price, or None to clear
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
    fn test_new_total_price() {
        let price = 100.0;
        let total_price = TotalPriceType::new(price);

        assert_eq!(total_price.price(), price);
        assert_eq!(total_price.price_excl_tax(), None);
        assert_eq!(total_price.price_incl_tax(), None);
        assert_eq!(total_price.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let price = 100.0;
        let price_excl_tax = 80.0;
        let price_incl_tax = 100.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let total_price = TotalPriceType::new(price)
            .with_price_excl_tax(price_excl_tax)
            .with_price_incl_tax(price_incl_tax)
            .with_custom_data(custom_data.clone());

        assert_eq!(total_price.price(), price);
        assert_eq!(total_price.price_excl_tax(), Some(price_excl_tax));
        assert_eq!(total_price.price_incl_tax(), Some(price_incl_tax));
        assert_eq!(total_price.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let price1 = 100.0;
        let price2 = 120.0;
        let price_excl_tax = 80.0;
        let price_incl_tax = 100.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut total_price = TotalPriceType::new(price1);

        total_price
            .set_price(price2)
            .set_price_excl_tax(Some(price_excl_tax))
            .set_price_incl_tax(Some(price_incl_tax))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(total_price.price(), price2);
        assert_eq!(total_price.price_excl_tax(), Some(price_excl_tax));
        assert_eq!(total_price.price_incl_tax(), Some(price_incl_tax));
        assert_eq!(total_price.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        total_price
            .set_price_excl_tax(None)
            .set_price_incl_tax(None)
            .set_custom_data(None);

        assert_eq!(total_price.price_excl_tax(), None);
        assert_eq!(total_price.price_incl_tax(), None);
        assert_eq!(total_price.custom_data(), None);
    }
}
