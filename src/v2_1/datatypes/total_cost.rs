use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, price::PriceType, total_price::TotalPriceType};
use crate::v2_1::enumerations::TariffCostEnumType;

/// Structure to report costs.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TotalCostType {
    /// Currency of the costs in ISO 4217 Code.
    #[validate(length(max = 3))]
    pub currency: String,

    /// Type of cost: normal or the minimum or maximum cost.
    pub type_of_cost: TariffCostEnumType,

    /// Fixed costs per transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<PriceType>,

    /// Energy costs per transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy: Option<PriceType>,

    /// Time cost per transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_time: Option<PriceType>,

    /// Idle time cost per transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_time: Option<PriceType>,

    /// Reservation time cost per transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_time: Option<PriceType>,

    /// Fixed reservation costs per transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_fixed: Option<PriceType>,

    /// Total cost including and/or excluding tax.
    pub total: TotalPriceType,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The total cost (including taxes) of the transaction in the specified currency.
    pub cost: f64,

    /// Optional. The total cost (excluding taxes) of the transaction in the specified currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_excl_tax: Option<f64>,

    /// Optional. The total cost (including taxes) of the transaction in the specified currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_incl_tax: Option<f64>,
}

impl TotalCostType {
    /// Creates a new `TotalCostType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency of the costs in ISO 4217 Code
    /// * `type_of_cost` - Type of cost: normal or the minimum or maximum cost
    /// * `total` - Total cost including and/or excluding tax
    /// * `cost` - The total cost (including taxes) of the transaction in the specified currency
    ///
    /// # Returns
    ///
    /// A new instance of `TotalCostType` with optional fields set to `None`
    pub fn new(
        currency: String,
        type_of_cost: TariffCostEnumType,
        total: TotalPriceType,
        cost: f64,
    ) -> Self {
        Self {
            currency,
            type_of_cost,
            total,
            cost,
            fixed: None,
            energy: None,
            charging_time: None,
            idle_time: None,
            reservation_time: None,
            reservation_fixed: None,
            custom_data: None,
            cost_excl_tax: None,
            cost_incl_tax: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this total cost
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the fixed costs.
    ///
    /// # Arguments
    ///
    /// * `fixed` - Fixed costs per transaction
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_fixed(mut self, fixed: PriceType) -> Self {
        self.fixed = Some(fixed);
        self
    }

    /// Sets the energy costs.
    ///
    /// # Arguments
    ///
    /// * `energy` - Energy costs per transaction
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_energy(mut self, energy: PriceType) -> Self {
        self.energy = Some(energy);
        self
    }

    /// Sets the charging time costs.
    ///
    /// # Arguments
    ///
    /// * `charging_time` - Time cost per transaction
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_charging_time(mut self, charging_time: PriceType) -> Self {
        self.charging_time = Some(charging_time);
        self
    }

    /// Sets the idle time costs.
    ///
    /// # Arguments
    ///
    /// * `idle_time` - Idle time cost per transaction
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_idle_time(mut self, idle_time: PriceType) -> Self {
        self.idle_time = Some(idle_time);
        self
    }

    /// Sets the reservation time costs.
    ///
    /// # Arguments
    ///
    /// * `reservation_time` - Reservation time cost per transaction
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_reservation_time(mut self, reservation_time: PriceType) -> Self {
        self.reservation_time = Some(reservation_time);
        self
    }

    /// Sets the fixed reservation costs.
    ///
    /// # Arguments
    ///
    /// * `reservation_fixed` - Fixed reservation costs per transaction
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_reservation_fixed(mut self, reservation_fixed: PriceType) -> Self {
        self.reservation_fixed = Some(reservation_fixed);
        self
    }

    /// Sets the cost excluding tax.
    ///
    /// # Arguments
    ///
    /// * `cost_excl_tax` - The total cost (excluding taxes) of the transaction in the specified currency
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_cost_excl_tax(mut self, cost_excl_tax: f64) -> Self {
        self.cost_excl_tax = Some(cost_excl_tax);
        self
    }

    /// Sets the cost including tax.
    ///
    /// # Arguments
    ///
    /// * `cost_incl_tax` - The total cost (including taxes) of the transaction in the specified currency
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_cost_incl_tax(mut self, cost_incl_tax: f64) -> Self {
        self.cost_incl_tax = Some(cost_incl_tax);
        self
    }

    /// Gets the currency.
    ///
    /// # Returns
    ///
    /// The currency of the costs in ISO 4217 Code
    pub fn currency(&self) -> &str {
        &self.currency
    }

    /// Sets the currency.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency of the costs in ISO 4217 Code
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_currency(&mut self, currency: String) -> &mut Self {
        self.currency = currency;
        self
    }

    /// Gets the type of cost.
    ///
    /// # Returns
    ///
    /// The type of cost: normal or the minimum or maximum cost
    pub fn type_of_cost(&self) -> &TariffCostEnumType {
        &self.type_of_cost
    }

    /// Sets the type of cost.
    ///
    /// # Arguments
    ///
    /// * `type_of_cost` - Type of cost: normal or the minimum or maximum cost
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_type_of_cost(&mut self, type_of_cost: TariffCostEnumType) -> &mut Self {
        self.type_of_cost = type_of_cost;
        self
    }

    /// Gets the total.
    ///
    /// # Returns
    ///
    /// A reference to the total cost including and/or excluding tax
    pub fn total(&self) -> &TotalPriceType {
        &self.total
    }

    /// Sets the total.
    ///
    /// # Arguments
    ///
    /// * `total` - Total cost including and/or excluding tax
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_total(&mut self, total: TotalPriceType) -> &mut Self {
        self.total = total;
        self
    }

    /// Gets the cost.
    ///
    /// # Returns
    ///
    /// The total cost (including taxes) of the transaction in the specified currency
    pub fn cost(&self) -> f64 {
        self.cost
    }

    /// Sets the cost.
    ///
    /// # Arguments
    ///
    /// * `cost` - The total cost (including taxes) of the transaction in the specified currency
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_cost(&mut self, cost: f64) -> &mut Self {
        self.cost = cost;
        self
    }

    /// Gets the fixed costs.
    ///
    /// # Returns
    ///
    /// An optional reference to the fixed costs per transaction
    pub fn fixed(&self) -> Option<&PriceType> {
        self.fixed.as_ref()
    }

    /// Sets the fixed costs.
    ///
    /// # Arguments
    ///
    /// * `fixed` - Fixed costs per transaction, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_fixed(&mut self, fixed: Option<PriceType>) -> &mut Self {
        self.fixed = fixed;
        self
    }

    /// Gets the energy costs.
    ///
    /// # Returns
    ///
    /// An optional reference to the energy costs per transaction
    pub fn energy(&self) -> Option<&PriceType> {
        self.energy.as_ref()
    }

    /// Sets the energy costs.
    ///
    /// # Arguments
    ///
    /// * `energy` - Energy costs per transaction, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_energy(&mut self, energy: Option<PriceType>) -> &mut Self {
        self.energy = energy;
        self
    }

    /// Gets the charging time costs.
    ///
    /// # Returns
    ///
    /// An optional reference to the time cost per transaction
    pub fn charging_time(&self) -> Option<&PriceType> {
        self.charging_time.as_ref()
    }

    /// Sets the charging time costs.
    ///
    /// # Arguments
    ///
    /// * `charging_time` - Time cost per transaction, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_time(&mut self, charging_time: Option<PriceType>) -> &mut Self {
        self.charging_time = charging_time;
        self
    }

    /// Gets the idle time costs.
    ///
    /// # Returns
    ///
    /// An optional reference to the idle time cost per transaction
    pub fn idle_time(&self) -> Option<&PriceType> {
        self.idle_time.as_ref()
    }

    /// Sets the idle time costs.
    ///
    /// # Arguments
    ///
    /// * `idle_time` - Idle time cost per transaction, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_idle_time(&mut self, idle_time: Option<PriceType>) -> &mut Self {
        self.idle_time = idle_time;
        self
    }

    /// Gets the reservation time costs.
    ///
    /// # Returns
    ///
    /// An optional reference to the reservation time cost per transaction
    pub fn reservation_time(&self) -> Option<&PriceType> {
        self.reservation_time.as_ref()
    }

    /// Sets the reservation time costs.
    ///
    /// # Arguments
    ///
    /// * `reservation_time` - Reservation time cost per transaction, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_reservation_time(&mut self, reservation_time: Option<PriceType>) -> &mut Self {
        self.reservation_time = reservation_time;
        self
    }

    /// Gets the fixed reservation costs.
    ///
    /// # Returns
    ///
    /// An optional reference to the fixed reservation costs per transaction
    pub fn reservation_fixed(&self) -> Option<&PriceType> {
        self.reservation_fixed.as_ref()
    }

    /// Sets the fixed reservation costs.
    ///
    /// # Arguments
    ///
    /// * `reservation_fixed` - Fixed reservation costs per transaction, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_reservation_fixed(&mut self, reservation_fixed: Option<PriceType>) -> &mut Self {
        self.reservation_fixed = reservation_fixed;
        self
    }

    /// Gets the cost excluding tax.
    ///
    /// # Returns
    ///
    /// An optional total cost (excluding taxes) of the transaction in the specified currency
    pub fn cost_excl_tax(&self) -> Option<f64> {
        self.cost_excl_tax
    }

    /// Sets the cost excluding tax.
    ///
    /// # Arguments
    ///
    /// * `cost_excl_tax` - The total cost (excluding taxes) of the transaction in the specified currency, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_cost_excl_tax(&mut self, cost_excl_tax: Option<f64>) -> &mut Self {
        self.cost_excl_tax = cost_excl_tax;
        self
    }

    /// Gets the cost including tax.
    ///
    /// # Returns
    ///
    /// An optional total cost (including taxes) of the transaction in the specified currency
    pub fn cost_incl_tax(&self) -> Option<f64> {
        self.cost_incl_tax
    }

    /// Sets the cost including tax.
    ///
    /// # Arguments
    ///
    /// * `cost_incl_tax` - The total cost (including taxes) of the transaction in the specified currency, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_cost_incl_tax(&mut self, cost_incl_tax: Option<f64>) -> &mut Self {
        self.cost_incl_tax = cost_incl_tax;
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
    /// * `custom_data` - Custom data for this total cost, or None to clear
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
    use rust_decimal::Decimal;
    #[test]
    fn test_new_total_cost() {
        let currency = "EUR".to_string();
        let type_of_cost = TariffCostEnumType::NormalCost;
        let total = TotalPriceType::new(100.0);
        let cost = 100.0;

        let total_cost =
            TotalCostType::new(currency.clone(), type_of_cost.clone(), total.clone(), cost);

        assert_eq!(total_cost.currency(), currency);
        assert_eq!(total_cost.type_of_cost(), &type_of_cost);
        assert_eq!(total_cost.total(), &total);
        assert_eq!(total_cost.cost(), cost);
        assert_eq!(total_cost.fixed(), None);
        assert_eq!(total_cost.energy(), None);
        assert_eq!(total_cost.charging_time(), None);
        assert_eq!(total_cost.idle_time(), None);
        assert_eq!(total_cost.reservation_time(), None);
        assert_eq!(total_cost.reservation_fixed(), None);
        assert_eq!(total_cost.cost_excl_tax(), None);
        assert_eq!(total_cost.cost_incl_tax(), None);
        assert_eq!(total_cost.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let currency = "EUR".to_string();
        let type_of_cost = TariffCostEnumType::NormalCost;
        let total = TotalPriceType::new(100.0);
        let cost = 100.0;
        let fixed = PriceType::new(Decimal::new(100, 1), false); // 10.0
        let energy = PriceType::new(Decimal::new(500, 1), false); // 50.0
        let charging_time = PriceType::new(Decimal::new(200, 1), false); // 20.0
        let idle_time = PriceType::new(Decimal::new(50, 1), false); // 5.0
        let reservation_time = PriceType::new(Decimal::new(50, 1), false); // 5.0
        let reservation_fixed = PriceType::new(Decimal::new(100, 1), false); // 10.0
        let cost_excl_tax = 80.0;
        let cost_incl_tax = 100.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let total_cost =
            TotalCostType::new(currency.clone(), type_of_cost.clone(), total.clone(), cost)
                .with_fixed(fixed.clone())
                .with_energy(energy.clone())
                .with_charging_time(charging_time.clone())
                .with_idle_time(idle_time.clone())
                .with_reservation_time(reservation_time.clone())
                .with_reservation_fixed(reservation_fixed.clone())
                .with_cost_excl_tax(cost_excl_tax)
                .with_cost_incl_tax(cost_incl_tax)
                .with_custom_data(custom_data.clone());

        assert_eq!(total_cost.currency(), currency);
        assert_eq!(total_cost.type_of_cost(), &type_of_cost);
        assert_eq!(total_cost.total(), &total);
        assert_eq!(total_cost.cost(), cost);
        assert_eq!(total_cost.fixed(), Some(&fixed));
        assert_eq!(total_cost.energy(), Some(&energy));
        assert_eq!(total_cost.charging_time(), Some(&charging_time));
        assert_eq!(total_cost.idle_time(), Some(&idle_time));
        assert_eq!(total_cost.reservation_time(), Some(&reservation_time));
        assert_eq!(total_cost.reservation_fixed(), Some(&reservation_fixed));
        assert_eq!(total_cost.cost_excl_tax(), Some(cost_excl_tax));
        assert_eq!(total_cost.cost_incl_tax(), Some(cost_incl_tax));
        assert_eq!(total_cost.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let currency1 = "EUR".to_string();
        let currency2 = "USD".to_string();
        let type_of_cost1 = TariffCostEnumType::NormalCost;
        let type_of_cost2 = TariffCostEnumType::MaxCost;
        let total1 = TotalPriceType::new(100.0);
        let total2 = TotalPriceType::new(120.0);
        let cost1 = 100.0;
        let cost2 = 120.0;
        let fixed = PriceType::new(Decimal::new(100, 1), false); // 10.0
        let energy = PriceType::new(Decimal::new(500, 1), false); // 50.0
        let charging_time = PriceType::new(Decimal::new(200, 1), false); // 20.0
        let idle_time = PriceType::new(Decimal::new(50, 1), false); // 5.0
        let reservation_time = PriceType::new(Decimal::new(50, 1), false); // 5.0
        let reservation_fixed = PriceType::new(Decimal::new(100, 1), false); // 10.0
        let cost_excl_tax = 80.0;
        let cost_incl_tax = 100.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut total_cost = TotalCostType::new(currency1, type_of_cost1, total1, cost1);

        total_cost
            .set_currency(currency2.clone())
            .set_type_of_cost(type_of_cost2.clone())
            .set_total(total2.clone())
            .set_cost(cost2)
            .set_fixed(Some(fixed.clone()))
            .set_energy(Some(energy.clone()))
            .set_charging_time(Some(charging_time.clone()))
            .set_idle_time(Some(idle_time.clone()))
            .set_reservation_time(Some(reservation_time.clone()))
            .set_reservation_fixed(Some(reservation_fixed.clone()))
            .set_cost_excl_tax(Some(cost_excl_tax))
            .set_cost_incl_tax(Some(cost_incl_tax))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(total_cost.currency(), currency2);
        assert_eq!(total_cost.type_of_cost(), &type_of_cost2);
        assert_eq!(total_cost.total(), &total2);
        assert_eq!(total_cost.cost(), cost2);
        assert_eq!(total_cost.fixed(), Some(&fixed));
        assert_eq!(total_cost.energy(), Some(&energy));
        assert_eq!(total_cost.charging_time(), Some(&charging_time));
        assert_eq!(total_cost.idle_time(), Some(&idle_time));
        assert_eq!(total_cost.reservation_time(), Some(&reservation_time));
        assert_eq!(total_cost.reservation_fixed(), Some(&reservation_fixed));
        assert_eq!(total_cost.cost_excl_tax(), Some(cost_excl_tax));
        assert_eq!(total_cost.cost_incl_tax(), Some(cost_incl_tax));
        assert_eq!(total_cost.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        total_cost
            .set_fixed(None)
            .set_energy(None)
            .set_charging_time(None)
            .set_idle_time(None)
            .set_reservation_time(None)
            .set_reservation_fixed(None)
            .set_cost_excl_tax(None)
            .set_cost_incl_tax(None)
            .set_custom_data(None);

        assert_eq!(total_cost.fixed(), None);
        assert_eq!(total_cost.energy(), None);
        assert_eq!(total_cost.charging_time(), None);
        assert_eq!(total_cost.idle_time(), None);
        assert_eq!(total_cost.reservation_time(), None);
        assert_eq!(total_cost.reservation_fixed(), None);
        assert_eq!(total_cost.cost_excl_tax(), None);
        assert_eq!(total_cost.cost_incl_tax(), None);
        assert_eq!(total_cost.custom_data(), None);
    }
}
