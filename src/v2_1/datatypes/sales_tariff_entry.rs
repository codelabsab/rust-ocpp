use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    consumption_cost::ConsumptionCostType, custom_data::CustomDataType,
    relative_time_interval::RelativeTimeIntervalType,
};

/// Sales tariff entry details.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffEntryType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Time and date at which the tariff becomes valid.
    pub relative_time_interval: RelativeTimeIntervalType,

    /// Optional. Consumption cost per time interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_cost: Option<Vec<ConsumptionCostType>>,

    /// Optional. A human readable tariff description.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub e_price_level: Option<i32>,
}

impl SalesTariffEntryType {
    /// Creates a new `SalesTariffEntryType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `e_price_level` - A human readable tariff description
    ///
    /// # Returns
    ///
    /// A new instance of `SalesTariffEntryType` with optional fields set to `None`
    pub fn new(e_price_level: i32) -> Self {
        Self {
            custom_data: None,
            relative_time_interval: RelativeTimeIntervalType::new_default(),
            consumption_cost: None,
            e_price_level: Some(e_price_level),
        }
    }

    /// Creates a new `SalesTariffEntryType` with a specific relative time interval.
    ///
    /// # Arguments
    ///
    /// * `relative_time_interval` - Time and date at which the tariff becomes valid
    ///
    /// # Returns
    ///
    /// A new instance of `SalesTariffEntryType` with optional fields set to `None`
    pub fn new_with_interval(relative_time_interval: RelativeTimeIntervalType) -> Self {
        Self {
            custom_data: None,
            relative_time_interval,
            consumption_cost: None,
            e_price_level: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this sales tariff entry
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the consumption cost.
    ///
    /// # Arguments
    ///
    /// * `consumption_cost` - Consumption cost per time interval
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_consumption_cost(mut self, consumption_cost: Vec<ConsumptionCostType>) -> Self {
        self.consumption_cost = Some(consumption_cost);
        self
    }

    /// Gets the relative time interval.
    ///
    /// # Returns
    ///
    /// A reference to the time and date at which the tariff becomes valid
    pub fn relative_time_interval(&self) -> &RelativeTimeIntervalType {
        &self.relative_time_interval
    }

    /// Sets the relative time interval.
    ///
    /// # Arguments
    ///
    /// * `relative_time_interval` - Time and date at which the tariff becomes valid
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_relative_time_interval(
        &mut self,
        relative_time_interval: RelativeTimeIntervalType,
    ) -> &mut Self {
        self.relative_time_interval = relative_time_interval;
        self
    }

    /// Gets the consumption cost.
    ///
    /// # Returns
    ///
    /// An optional reference to consumption cost per time interval
    pub fn consumption_cost(&self) -> Option<&Vec<ConsumptionCostType>> {
        self.consumption_cost.as_ref()
    }

    /// Sets the consumption cost.
    ///
    /// # Arguments
    ///
    /// * `consumption_cost` - Consumption cost per time interval, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_consumption_cost(
        &mut self,
        consumption_cost: Option<Vec<ConsumptionCostType>>,
    ) -> &mut Self {
        self.consumption_cost = consumption_cost;
        self
    }

    /// Gets the price level.
    ///
    /// # Returns
    ///
    /// An optional human readable tariff description
    pub fn e_price_level(&self) -> Option<i32> {
        self.e_price_level
    }

    /// Sets the price level.
    ///
    /// # Arguments
    ///
    /// * `e_price_level` - A human readable tariff description, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_e_price_level(&mut self, e_price_level: Option<i32>) -> &mut Self {
        self.e_price_level = e_price_level;
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
    /// * `custom_data` - Custom data for this sales tariff entry, or None to clear
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
    use crate::v2_1::datatypes::cost::CostType;
    use crate::v2_1::enumerations::CostKindEnumType;

    #[test]
    fn test_new_sales_tariff_entry() {
        let price_level = 3;
        let entry = SalesTariffEntryType::new(price_level);

        assert_eq!(entry.e_price_level(), Some(price_level));
        assert_eq!(entry.consumption_cost(), None);
        assert_eq!(entry.custom_data(), None);
    }

    #[test]
    fn test_new_with_interval() {
        let interval = RelativeTimeIntervalType::new_default();
        let entry = SalesTariffEntryType::new_with_interval(interval.clone());

        assert_eq!(entry.relative_time_interval(), &interval);
        assert_eq!(entry.e_price_level(), None);
        assert_eq!(entry.consumption_cost(), None);
        assert_eq!(entry.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let price_level = 3;
        let cost = CostType::new(CostKindEnumType::CarbonDioxideEmission, 100);
        let consumption_cost = vec![ConsumptionCostType::new(10.0, vec![cost])];
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let entry = SalesTariffEntryType::new(price_level)
            .with_consumption_cost(consumption_cost.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(entry.e_price_level(), Some(price_level));
        assert_eq!(entry.consumption_cost().unwrap().len(), 1);
        assert_eq!(entry.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let price_level1 = 3;
        let mut entry = SalesTariffEntryType::new(price_level1);

        let interval = RelativeTimeIntervalType::new_default();
        let price_level2 = 5;
        let cost = CostType::new(CostKindEnumType::CarbonDioxideEmission, 100);
        let consumption_cost = vec![ConsumptionCostType::new(10.0, vec![cost])];
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        entry
            .set_relative_time_interval(interval.clone())
            .set_e_price_level(Some(price_level2))
            .set_consumption_cost(Some(consumption_cost.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(entry.relative_time_interval(), &interval);
        assert_eq!(entry.e_price_level(), Some(price_level2));
        assert_eq!(entry.consumption_cost().unwrap().len(), 1);
        assert_eq!(entry.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        entry
            .set_e_price_level(None)
            .set_consumption_cost(None)
            .set_custom_data(None);

        assert_eq!(entry.e_price_level(), None);
        assert_eq!(entry.consumption_cost(), None);
        assert_eq!(entry.custom_data(), None);
    }
}
