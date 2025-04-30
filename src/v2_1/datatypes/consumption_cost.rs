use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{cost::CostType, custom_data::CustomDataType};

/// Consumption cost type for consumption blocks.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConsumptionCostType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// The lowest level of consumption that defines the starting point of this consumption block.
    /// The block interval extends to the start of the next interval.
    pub start_value: f64,

    /// List of costs associated with this consumption block.
    #[validate(length(min = 1, max = 3))]
    pub cost: Vec<CostType>,
}

impl ConsumptionCostType {
    /// Creates a new `ConsumptionCostType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `start_value` - The lowest level of consumption that defines the starting point of this consumption block
    /// * `cost` - List of costs associated with this consumption block
    ///
    /// # Returns
    ///
    /// A new instance of `ConsumptionCostType` with optional fields set to `None`
    pub fn new(start_value: f64, cost: Vec<CostType>) -> Self {
        Self {
            start_value,
            cost,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this consumption cost
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the start value.
    ///
    /// # Returns
    ///
    /// The lowest level of consumption that defines the starting point of this consumption block
    pub fn start_value(&self) -> f64 {
        self.start_value
    }

    /// Sets the start value.
    ///
    /// # Arguments
    ///
    /// * `start_value` - The lowest level of consumption that defines the starting point of this consumption block
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_start_value(&mut self, start_value: f64) -> &mut Self {
        self.start_value = start_value;
        self
    }

    /// Gets the costs.
    ///
    /// # Returns
    ///
    /// A reference to the list of costs associated with this consumption block
    pub fn cost(&self) -> &[CostType] {
        &self.cost
    }

    /// Sets the costs.
    ///
    /// # Arguments
    ///
    /// * `cost` - List of costs associated with this consumption block
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_cost(&mut self, cost: Vec<CostType>) -> &mut Self {
        self.cost = cost;
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
    /// * `custom_data` - Custom data for this consumption cost, or None to clear
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
    use crate::v2_1::enumerations::CostKindEnumType;

    #[test]
    fn test_new_consumption_cost() {
        let cost = CostType::new(CostKindEnumType::CarbonDioxideEmission, 100);
        let consumption_cost = ConsumptionCostType::new(10.0, vec![cost.clone()]);

        assert_eq!(consumption_cost.start_value(), 10.0);
        assert_eq!(consumption_cost.cost().len(), 1);
        assert_eq!(consumption_cost.cost()[0].cost_kind(), &CostKindEnumType::CarbonDioxideEmission);
        assert_eq!(consumption_cost.cost()[0].amount(), 100);
        assert_eq!(consumption_cost.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let cost = CostType::new(CostKindEnumType::CarbonDioxideEmission, 100);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let consumption_cost = ConsumptionCostType::new(10.0, vec![cost.clone()])
            .with_custom_data(custom_data.clone());

        assert_eq!(consumption_cost.start_value(), 10.0);
        assert_eq!(consumption_cost.cost().len(), 1);
        assert_eq!(consumption_cost.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let cost1 = CostType::new(CostKindEnumType::CarbonDioxideEmission, 100);
        let cost2 = CostType::new(CostKindEnumType::RelativePricePercentage, 200);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut consumption_cost = ConsumptionCostType::new(10.0, vec![cost1.clone()]);

        consumption_cost
            .set_start_value(20.0)
            .set_cost(vec![cost1.clone(), cost2.clone()])
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(consumption_cost.start_value(), 20.0);
        assert_eq!(consumption_cost.cost().len(), 2);
        assert_eq!(consumption_cost.cost()[0].cost_kind(), &CostKindEnumType::CarbonDioxideEmission);
        assert_eq!(consumption_cost.cost()[1].cost_kind(), &CostKindEnumType::RelativePricePercentage);
        assert_eq!(consumption_cost.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        consumption_cost.set_custom_data(None);
        assert_eq!(consumption_cost.custom_data(), None);
    }
}
