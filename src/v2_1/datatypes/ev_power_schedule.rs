use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, ev_power_schedule_entry::EVPowerScheduleEntryType};

/// Power schedule of EV energy offer.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EVPowerScheduleType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Starting point in time of the EVEnergyOffer.
    pub time_anchor: DateTime<Utc>,

    /// List of power schedule entries.
    #[validate(length(min = 1, max = 1024))]
    pub ev_power_schedule_entries: Vec<EVPowerScheduleEntryType>,
}

impl EVPowerScheduleType {
    /// Creates a new `EVPowerScheduleType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `time_anchor` - Starting point in time of the EVEnergyOffer
    /// * `ev_power_schedule_entries` - List of power schedule entries
    ///
    /// # Returns
    ///
    /// A new instance of `EVPowerScheduleType` with optional fields set to `None`
    pub fn new(
        time_anchor: DateTime<Utc>,
        ev_power_schedule_entries: Vec<EVPowerScheduleEntryType>,
    ) -> Self {
        Self {
            time_anchor,
            ev_power_schedule_entries,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this power schedule
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
    /// A reference to the starting point in time of the EVEnergyOffer
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

    /// Gets the power schedule entries.
    ///
    /// # Returns
    ///
    /// A reference to the list of power schedule entries
    pub fn ev_power_schedule_entries(&self) -> &Vec<EVPowerScheduleEntryType> {
        &self.ev_power_schedule_entries
    }

    /// Sets the power schedule entries.
    ///
    /// # Arguments
    ///
    /// * `ev_power_schedule_entries` - List of power schedule entries
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_power_schedule_entries(
        &mut self,
        ev_power_schedule_entries: Vec<EVPowerScheduleEntryType>,
    ) -> &mut Self {
        self.ev_power_schedule_entries = ev_power_schedule_entries;
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
    /// * `custom_data` - Custom data for this power schedule, or None to clear
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
    fn test_new_ev_power_schedule() {
        let time_anchor = Utc::now();
        let entries = vec![
            EVPowerScheduleEntryType::new(3600, 11000.0),
            EVPowerScheduleEntryType::new(7200, 7500.0),
        ];

        let schedule = EVPowerScheduleType::new(time_anchor.clone(), entries.clone());

        assert_eq!(schedule.time_anchor(), &time_anchor);
        assert_eq!(schedule.ev_power_schedule_entries(), &entries);
        assert_eq!(schedule.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let time_anchor = Utc::now();
        let entries = vec![
            EVPowerScheduleEntryType::new(3600, 11000.0),
            EVPowerScheduleEntryType::new(7200, 7500.0),
        ];
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let schedule = EVPowerScheduleType::new(time_anchor.clone(), entries.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(schedule.time_anchor(), &time_anchor);
        assert_eq!(schedule.ev_power_schedule_entries(), &entries);
        assert_eq!(schedule.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let time_anchor1 = Utc::now();
        let entries1 = vec![
            EVPowerScheduleEntryType::new(3600, 11000.0),
            EVPowerScheduleEntryType::new(7200, 7500.0),
        ];

        let time_anchor2 = Utc::now();
        let entries2 = vec![
            EVPowerScheduleEntryType::new(1800, 22000.0),
            EVPowerScheduleEntryType::new(3600, 11000.0),
            EVPowerScheduleEntryType::new(5400, 5500.0),
        ];
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut schedule = EVPowerScheduleType::new(time_anchor1.clone(), entries1.clone());

        schedule
            .set_time_anchor(time_anchor2.clone())
            .set_ev_power_schedule_entries(entries2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(schedule.time_anchor(), &time_anchor2);
        assert_eq!(schedule.ev_power_schedule_entries(), &entries2);
        assert_eq!(schedule.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        schedule.set_custom_data(None);
        assert_eq!(schedule.custom_data(), None);
    }
}
