use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Structure to report total usage of a resource.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TotalUsageType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The total usage of the resource.
    pub usage: f64,

    /// Optional. The total usage of the resource, excluding taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_excl_tax: Option<f64>,

    /// Optional. The total usage of the resource, including taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_incl_tax: Option<f64>,
}

impl TotalUsageType {
    /// Creates a new `TotalUsageType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `usage` - The total usage of the resource
    ///
    /// # Returns
    ///
    /// A new instance of `TotalUsageType` with optional fields set to `None`
    pub fn new(usage: f64) -> Self {
        Self {
            usage,
            custom_data: None,
            usage_excl_tax: None,
            usage_incl_tax: None,
        }
    }

    /// Gets the total usage.
    ///
    /// # Returns
    ///
    /// The total usage of the resource
    pub fn usage(&self) -> f64 {
        self.usage
    }

    /// Gets the total usage excluding tax.
    ///
    /// # Returns
    ///
    /// An optional total usage of the resource, excluding taxes
    pub fn usage_excl_tax(&self) -> Option<f64> {
        self.usage_excl_tax
    }

    /// Gets the total usage including tax.
    ///
    /// # Returns
    ///
    /// An optional total usage of the resource, including taxes
    pub fn usage_incl_tax(&self) -> Option<f64> {
        self.usage_incl_tax
    }

    /// Gets the custom data.
    ///
    /// # Returns
    ///
    /// An optional reference to the custom data
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the total usage.
    ///
    /// # Arguments
    ///
    /// * `usage` - The total usage of the resource
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_usage(&mut self, usage: f64) -> &mut Self {
        self.usage = usage;
        self
    }

    /// Sets the total usage excluding tax.
    ///
    /// # Arguments
    ///
    /// * `usage_excl_tax` - The total usage of the resource, excluding taxes, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_usage_excl_tax(&mut self, usage_excl_tax: Option<f64>) -> &mut Self {
        self.usage_excl_tax = usage_excl_tax;
        self
    }

    /// Sets the total usage including tax.
    ///
    /// # Arguments
    ///
    /// * `usage_incl_tax` - The total usage of the resource, including taxes, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_usage_incl_tax(&mut self, usage_incl_tax: Option<f64>) -> &mut Self {
        self.usage_incl_tax = usage_incl_tax;
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this total usage, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Sets the total usage excluding tax.
    ///
    /// # Arguments
    ///
    /// * `usage_excl_tax` - The total usage of the resource, excluding taxes
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_usage_excl_tax(mut self, usage_excl_tax: f64) -> Self {
        self.usage_excl_tax = Some(usage_excl_tax);
        self
    }

    /// Sets the total usage including tax.
    ///
    /// # Arguments
    ///
    /// * `usage_incl_tax` - The total usage of the resource, including taxes
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_usage_incl_tax(mut self, usage_incl_tax: f64) -> Self {
        self.usage_incl_tax = Some(usage_incl_tax);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this total usage
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_total_usage() {
        let usage = 100.0;
        let total_usage = TotalUsageType::new(usage);

        assert_eq!(total_usage.usage(), usage);
        assert_eq!(total_usage.usage_excl_tax(), None);
        assert_eq!(total_usage.usage_incl_tax(), None);
        assert_eq!(total_usage.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let usage = 100.0;
        let usage_excl_tax = 80.0;
        let usage_incl_tax = 100.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let total_usage = TotalUsageType::new(usage)
            .with_usage_excl_tax(usage_excl_tax)
            .with_usage_incl_tax(usage_incl_tax)
            .with_custom_data(custom_data.clone());

        assert_eq!(total_usage.usage(), usage);
        assert_eq!(total_usage.usage_excl_tax(), Some(usage_excl_tax));
        assert_eq!(total_usage.usage_incl_tax(), Some(usage_incl_tax));
        assert_eq!(total_usage.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let mut total_usage = TotalUsageType::new(100.0);
        let custom_data = CustomDataType::new("VendorX".to_string());

        total_usage
            .set_usage(120.0)
            .set_usage_excl_tax(Some(100.0))
            .set_usage_incl_tax(Some(120.0))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(total_usage.usage(), 120.0);
        assert_eq!(total_usage.usage_excl_tax(), Some(100.0));
        assert_eq!(total_usage.usage_incl_tax(), Some(120.0));
        assert_eq!(total_usage.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        total_usage
            .set_usage_excl_tax(None)
            .set_usage_incl_tax(None)
            .set_custom_data(None);

        assert_eq!(total_usage.usage(), 120.0);
        assert_eq!(total_usage.usage_excl_tax(), None);
        assert_eq!(total_usage.usage_incl_tax(), None);
        assert_eq!(total_usage.custom_data(), None);
    }
}
