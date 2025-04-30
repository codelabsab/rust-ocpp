use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    custom_data::CustomDataType, ev_absolute_price_schedule_entry::EVAbsolutePriceScheduleEntryType,
};

/// Price schedule of EV energy offer.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EVAbsolutePriceScheduleType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Starting point in time of the EVEnergyOffer.
    pub time_anchor: DateTime<Utc>,

    /// Currency code according to ISO 4217.
    #[validate(length(max = 3))]
    pub currency: String,

    /// ISO 15118-20 URN of price algorithm: Power, PeakPower, StackedEnergy.
    #[validate(length(max = 2000))]
    pub price_algorithm: String,

    /// List of price schedule entries.
    #[validate(length(min = 1, max = 1024))]
    pub ev_absolute_price_schedule_entries: Vec<EVAbsolutePriceScheduleEntryType>,
}

impl EVAbsolutePriceScheduleType {
    /// Creates a new `EVAbsolutePriceScheduleType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `time_anchor` - Starting point in time of the EVEnergyOffer
    /// * `currency` - Currency code according to ISO 4217
    /// * `price_algorithm` - ISO 15118-20 URN of price algorithm
    /// * `ev_absolute_price_schedule_entries` - List of price schedule entries
    ///
    /// # Returns
    ///
    /// A new instance of `EVAbsolutePriceScheduleType` with optional fields set to `None`
    pub fn new(
        time_anchor: DateTime<Utc>,
        currency: String,
        price_algorithm: String,
        ev_absolute_price_schedule_entries: Vec<EVAbsolutePriceScheduleEntryType>,
    ) -> Self {
        Self {
            time_anchor,
            currency,
            price_algorithm,
            ev_absolute_price_schedule_entries,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this price schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the time anchor.
    ///
    /// # Returns
    ///
    /// The starting point in time of the EVEnergyOffer
    pub fn time_anchor(&self) -> &DateTime<Utc> {
        &self.time_anchor
    }

    /// Sets the time anchor.
    ///
    /// # Arguments
    ///
    /// * `time_anchor` - Starting point in time of the EVEnergyOffer
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_time_anchor(&mut self, time_anchor: DateTime<Utc>) -> &mut Self {
        self.time_anchor = time_anchor;
        self
    }

    /// Gets the currency.
    ///
    /// # Returns
    ///
    /// The currency code according to ISO 4217
    pub fn currency(&self) -> &str {
        &self.currency
    }

    /// Sets the currency.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency code according to ISO 4217
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_currency(&mut self, currency: String) -> &mut Self {
        self.currency = currency;
        self
    }

    /// Gets the price algorithm.
    ///
    /// # Returns
    ///
    /// The ISO 15118-20 URN of price algorithm
    pub fn price_algorithm(&self) -> &str {
        &self.price_algorithm
    }

    /// Sets the price algorithm.
    ///
    /// # Arguments
    ///
    /// * `price_algorithm` - ISO 15118-20 URN of price algorithm
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price_algorithm(&mut self, price_algorithm: String) -> &mut Self {
        self.price_algorithm = price_algorithm;
        self
    }

    /// Gets the price schedule entries.
    ///
    /// # Returns
    ///
    /// A reference to the list of price schedule entries
    pub fn ev_absolute_price_schedule_entries(&self) -> &Vec<EVAbsolutePriceScheduleEntryType> {
        &self.ev_absolute_price_schedule_entries
    }

    /// Sets the price schedule entries.
    ///
    /// # Arguments
    ///
    /// * `ev_absolute_price_schedule_entries` - List of price schedule entries
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_absolute_price_schedule_entries(
        &mut self,
        ev_absolute_price_schedule_entries: Vec<EVAbsolutePriceScheduleEntryType>,
    ) -> &mut Self {
        self.ev_absolute_price_schedule_entries = ev_absolute_price_schedule_entries;
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
    /// * `custom_data` - Custom data for this price schedule, or None to clear
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
    fn test_new_ev_absolute_price_schedule() {
        let time_anchor = Utc::now();
        let currency = "EUR".to_string();
        let price_algorithm = "Power".to_string();
        let entries = vec![
            EVAbsolutePriceScheduleEntryType::new(3600, 0.25),
            EVAbsolutePriceScheduleEntryType::new(7200, 0.30),
        ];

        let schedule = EVAbsolutePriceScheduleType::new(
            time_anchor.clone(),
            currency.clone(),
            price_algorithm.clone(),
            entries.clone(),
        );

        assert_eq!(schedule.time_anchor(), &time_anchor);
        assert_eq!(schedule.currency(), currency);
        assert_eq!(schedule.price_algorithm(), price_algorithm);
        assert_eq!(schedule.ev_absolute_price_schedule_entries(), &entries);
        assert_eq!(schedule.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let time_anchor = Utc::now();
        let currency = "EUR".to_string();
        let price_algorithm = "Power".to_string();
        let entries = vec![
            EVAbsolutePriceScheduleEntryType::new(3600, 0.25),
            EVAbsolutePriceScheduleEntryType::new(7200, 0.30),
        ];
        let custom_data = CustomDataType::new("VendorX".to_string());

        let schedule = EVAbsolutePriceScheduleType::new(
            time_anchor.clone(),
            currency.clone(),
            price_algorithm.clone(),
            entries.clone(),
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(schedule.time_anchor(), &time_anchor);
        assert_eq!(schedule.currency(), currency);
        assert_eq!(schedule.price_algorithm(), price_algorithm);
        assert_eq!(schedule.ev_absolute_price_schedule_entries(), &entries);
        assert_eq!(schedule.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let time_anchor1 = Utc::now();
        let currency1 = "EUR".to_string();
        let price_algorithm1 = "Power".to_string();
        let entries1 = vec![
            EVAbsolutePriceScheduleEntryType::new(3600, 0.25),
            EVAbsolutePriceScheduleEntryType::new(7200, 0.30),
        ];

        let time_anchor2 = Utc::now();
        let currency2 = "USD".to_string();
        let price_algorithm2 = "PeakPower".to_string();
        let entries2 = vec![
            EVAbsolutePriceScheduleEntryType::new(1800, 0.20),
            EVAbsolutePriceScheduleEntryType::new(3600, 0.25),
            EVAbsolutePriceScheduleEntryType::new(5400, 0.35),
        ];
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut schedule = EVAbsolutePriceScheduleType::new(
            time_anchor1.clone(),
            currency1.clone(),
            price_algorithm1.clone(),
            entries1.clone(),
        );

        schedule
            .set_time_anchor(time_anchor2.clone())
            .set_currency(currency2.clone())
            .set_price_algorithm(price_algorithm2.clone())
            .set_ev_absolute_price_schedule_entries(entries2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(schedule.time_anchor(), &time_anchor2);
        assert_eq!(schedule.currency(), currency2);
        assert_eq!(schedule.price_algorithm(), price_algorithm2);
        assert_eq!(schedule.ev_absolute_price_schedule_entries(), &entries2);
        assert_eq!(schedule.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        schedule.set_custom_data(None);
        assert_eq!(schedule.custom_data(), None);
    }
}
