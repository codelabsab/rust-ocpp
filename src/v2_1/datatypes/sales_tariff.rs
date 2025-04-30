use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, sales_tariff_entry::SalesTariffEntryType};

/// Sales tariff structure defining pricing information.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Unique identifier used to identify one or more tariffs.
    #[validate(length(max = 36))]
    pub id: String,

    /// Optional. A human readable title/description of the sales tariff e.g. for HMI display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 32))]
    pub sales_tariff_description: Option<String>,

    /// Optional. The number of time intervals supported for this sales tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_e_price_levels: Option<i32>,

    /// Required. List of sales tariff entries.
    #[validate(length(min = 1, max = 1024))]
    pub sales_tariff_entry: Vec<SalesTariffEntryType>,
}

impl SalesTariffType {
    /// Creates a new `SalesTariffType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier used to identify one or more tariffs
    /// * `sales_tariff_entry` - List of sales tariff entries
    ///
    /// # Returns
    ///
    /// A new instance of `SalesTariffType` with optional fields set to `None`
    pub fn new(id: String, sales_tariff_entry: Vec<SalesTariffEntryType>) -> Self {
        Self {
            custom_data: None,
            id,
            sales_tariff_description: None,
            num_e_price_levels: None,
            sales_tariff_entry,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this sales tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the sales tariff description.
    ///
    /// # Arguments
    ///
    /// * `description` - A human readable title/description of the sales tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_sales_tariff_description(mut self, description: String) -> Self {
        self.sales_tariff_description = Some(description);
        self
    }

    /// Sets the number of time intervals supported for this sales tariff.
    ///
    /// # Arguments
    ///
    /// * `num_e_price_levels` - The number of time intervals supported
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_num_e_price_levels(mut self, num_e_price_levels: i32) -> Self {
        self.num_e_price_levels = Some(num_e_price_levels);
        self
    }

    /// Gets the ID.
    ///
    /// # Returns
    ///
    /// The unique identifier used to identify one or more tariffs
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Sets the ID.
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

    /// Gets the sales tariff description.
    ///
    /// # Returns
    ///
    /// An optional human readable title/description of the sales tariff
    pub fn sales_tariff_description(&self) -> Option<&str> {
        self.sales_tariff_description.as_deref()
    }

    /// Sets the sales tariff description.
    ///
    /// # Arguments
    ///
    /// * `description` - A human readable title/description of the sales tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_sales_tariff_description(&mut self, description: Option<String>) -> &mut Self {
        self.sales_tariff_description = description;
        self
    }

    /// Gets the number of time intervals supported.
    ///
    /// # Returns
    ///
    /// An optional number of time intervals supported for this sales tariff
    pub fn num_e_price_levels(&self) -> Option<i32> {
        self.num_e_price_levels
    }

    /// Sets the number of time intervals supported.
    ///
    /// # Arguments
    ///
    /// * `num_e_price_levels` - The number of time intervals supported, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_num_e_price_levels(&mut self, num_e_price_levels: Option<i32>) -> &mut Self {
        self.num_e_price_levels = num_e_price_levels;
        self
    }

    /// Gets the sales tariff entries.
    ///
    /// # Returns
    ///
    /// A reference to the list of sales tariff entries
    pub fn sales_tariff_entry(&self) -> &[SalesTariffEntryType] {
        &self.sales_tariff_entry
    }

    /// Sets the sales tariff entries.
    ///
    /// # Arguments
    ///
    /// * `sales_tariff_entry` - List of sales tariff entries
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_sales_tariff_entry(&mut self, sales_tariff_entry: Vec<SalesTariffEntryType>) -> &mut Self {
        self.sales_tariff_entry = sales_tariff_entry;
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
    /// * `custom_data` - Custom data for this sales tariff, or None to clear
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
    fn test_new_sales_tariff() {
        let id = "tariff_1".to_string();
        let entry = SalesTariffEntryType::new(1);
        let sales_tariff = SalesTariffType::new(id.clone(), vec![entry.clone()]);

        assert_eq!(sales_tariff.id(), id);
        assert_eq!(sales_tariff.sales_tariff_description(), None);
        assert_eq!(sales_tariff.num_e_price_levels(), None);
        assert_eq!(sales_tariff.sales_tariff_entry().len(), 1);
        assert_eq!(sales_tariff.sales_tariff_entry()[0], entry);
        assert_eq!(sales_tariff.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let id = "tariff_1".to_string();
        let entry = SalesTariffEntryType::new(1);
        let description = "Peak Hours Tariff".to_string();
        let num_levels = 3;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let sales_tariff = SalesTariffType::new(id.clone(), vec![entry.clone()])
            .with_sales_tariff_description(description.clone())
            .with_num_e_price_levels(num_levels)
            .with_custom_data(custom_data.clone());

        assert_eq!(sales_tariff.id(), id);
        assert_eq!(sales_tariff.sales_tariff_description(), Some(description.as_str()));
        assert_eq!(sales_tariff.num_e_price_levels(), Some(num_levels));
        assert_eq!(sales_tariff.sales_tariff_entry().len(), 1);
        assert_eq!(sales_tariff.sales_tariff_entry()[0], entry);
        assert_eq!(sales_tariff.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let id1 = "tariff_1".to_string();
        let entry1 = SalesTariffEntryType::new(1);
        let mut sales_tariff = SalesTariffType::new(id1, vec![entry1]);

        let id2 = "tariff_2".to_string();
        let entry2 = SalesTariffEntryType::new(2);
        let description = "Off-peak Hours Tariff".to_string();
        let num_levels = 5;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        sales_tariff
            .set_id(id2.clone())
            .set_sales_tariff_description(Some(description.clone()))
            .set_num_e_price_levels(Some(num_levels))
            .set_sales_tariff_entry(vec![entry2.clone()])
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(sales_tariff.id(), id2);
        assert_eq!(sales_tariff.sales_tariff_description(), Some(description.as_str()));
        assert_eq!(sales_tariff.num_e_price_levels(), Some(num_levels));
        assert_eq!(sales_tariff.sales_tariff_entry().len(), 1);
        assert_eq!(sales_tariff.sales_tariff_entry()[0], entry2);
        assert_eq!(sales_tariff.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        sales_tariff
            .set_sales_tariff_description(None)
            .set_num_e_price_levels(None)
            .set_custom_data(None);

        assert_eq!(sales_tariff.sales_tariff_description(), None);
        assert_eq!(sales_tariff.num_e_price_levels(), None);
        assert_eq!(sales_tariff.custom_data(), None);
    }
}
