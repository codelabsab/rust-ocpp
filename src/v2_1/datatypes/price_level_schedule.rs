use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, price_level_schedule_entry::PriceLevelScheduleEntryType};

/// Price level schedule structure defines a list of time periods during which a specific price level applies.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PriceLevelScheduleType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Point in time at which the schedule becomes active.
    pub time_anchor: DateTime<Utc>,

    /// Required. List of price level schedule entries.
    #[validate(length(min = 1, max = 1024))]
    pub price_level_schedule_entries: Vec<PriceLevelScheduleEntryType>,
}

impl PriceLevelScheduleType {
    /// Creates a new `PriceLevelScheduleType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `time_anchor` - Point in time at which the schedule becomes active
    /// * `price_level_schedule_entries` - List of price level schedule entries
    ///
    /// # Returns
    ///
    /// A new instance of `PriceLevelScheduleType` with optional fields set to `None`
    pub fn new(
        time_anchor: DateTime<Utc>,
        price_level_schedule_entries: Vec<PriceLevelScheduleEntryType>,
    ) -> Self {
        Self {
            time_anchor,
            price_level_schedule_entries,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this price level schedule
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
    /// A reference to the point in time at which the schedule becomes active
    pub fn time_anchor(&self) -> &DateTime<Utc> {
        &self.time_anchor
    }

    /// Sets the time anchor.
    ///
    /// # Arguments
    ///
    /// * `time_anchor` - Point in time at which the schedule becomes active
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_time_anchor(&mut self, time_anchor: DateTime<Utc>) -> &mut Self {
        self.time_anchor = time_anchor;
        self
    }

    /// Gets the price level schedule entries.
    ///
    /// # Returns
    ///
    /// A reference to the list of price level schedule entries
    pub fn price_level_schedule_entries(&self) -> &Vec<PriceLevelScheduleEntryType> {
        &self.price_level_schedule_entries
    }

    /// Sets the price level schedule entries.
    ///
    /// # Arguments
    ///
    /// * `price_level_schedule_entries` - List of price level schedule entries
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price_level_schedule_entries(
        &mut self,
        price_level_schedule_entries: Vec<PriceLevelScheduleEntryType>,
    ) -> &mut Self {
        self.price_level_schedule_entries = price_level_schedule_entries;
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
    /// * `custom_data` - Custom data for this price level schedule, or None to clear
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
    fn test_new_price_level_schedule() {
        let time_anchor = Utc::now();
        let entries = vec![
            PriceLevelScheduleEntryType::new(3600, 1),
            PriceLevelScheduleEntryType::new(7200, 2),
        ];

        let schedule = PriceLevelScheduleType::new(time_anchor.clone(), entries.clone());

        assert_eq!(schedule.time_anchor(), &time_anchor);
        assert_eq!(schedule.price_level_schedule_entries(), &entries);
        assert_eq!(schedule.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let time_anchor = Utc::now();
        let entries = vec![
            PriceLevelScheduleEntryType::new(3600, 1),
            PriceLevelScheduleEntryType::new(7200, 2),
        ];
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let schedule = PriceLevelScheduleType::new(time_anchor.clone(), entries.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(schedule.time_anchor(), &time_anchor);
        assert_eq!(schedule.price_level_schedule_entries(), &entries);
        assert_eq!(schedule.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let time_anchor1 = Utc::now();
        let entries1 = vec![
            PriceLevelScheduleEntryType::new(3600, 1),
            PriceLevelScheduleEntryType::new(7200, 2),
        ];

        let time_anchor2 = Utc::now();
        let entries2 = vec![
            PriceLevelScheduleEntryType::new(1800, 3),
            PriceLevelScheduleEntryType::new(3600, 2),
            PriceLevelScheduleEntryType::new(5400, 1),
        ];
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut schedule = PriceLevelScheduleType::new(time_anchor1.clone(), entries1.clone());

        schedule
            .set_time_anchor(time_anchor2.clone())
            .set_price_level_schedule_entries(entries2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(schedule.time_anchor(), &time_anchor2);
        assert_eq!(schedule.price_level_schedule_entries(), &entries2);
        assert_eq!(schedule.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        schedule.set_custom_data(None);
        assert_eq!(schedule.custom_data(), None);
    }
}
