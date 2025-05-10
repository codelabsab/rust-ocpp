use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Entry in the EVAbsolutePriceSchedule.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EVAbsolutePriceScheduleEntryType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Duration of the schedule entry in seconds.
    pub duration: i32,

    /// Price per power unit.
    pub price: f64,
}

impl EVAbsolutePriceScheduleEntryType {
    /// Creates a new `EVAbsolutePriceScheduleEntryType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the schedule entry in seconds
    /// * `price` - Price per power unit
    ///
    /// # Returns
    ///
    /// A new instance of `EVAbsolutePriceScheduleEntryType` with optional fields set to `None`
    pub fn new(duration: i32, price: f64) -> Self {
        Self {
            duration,
            price,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this schedule entry
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the duration.
    ///
    /// # Returns
    ///
    /// The duration of the schedule entry in seconds
    pub fn duration(&self) -> i32 {
        self.duration
    }

    /// Sets the duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the schedule entry in seconds
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_duration(&mut self, duration: i32) -> &mut Self {
        self.duration = duration;
        self
    }

    /// Gets the price.
    ///
    /// # Returns
    ///
    /// The price per power unit
    pub fn price(&self) -> f64 {
        self.price
    }

    /// Sets the price.
    ///
    /// # Arguments
    ///
    /// * `price` - Price per power unit
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price(&mut self, price: f64) -> &mut Self {
        self.price = price;
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
    /// * `custom_data` - Custom data for this schedule entry, or None to clear
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
    fn test_new_ev_absolute_price_schedule_entry() {
        let duration = 3600;
        let price = 0.25;

        let entry = EVAbsolutePriceScheduleEntryType::new(duration, price);

        assert_eq!(entry.duration(), duration);
        assert_eq!(entry.price(), price);
        assert_eq!(entry.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let duration = 3600;
        let price = 0.25;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let entry = EVAbsolutePriceScheduleEntryType::new(duration, price)
            .with_custom_data(custom_data.clone());

        assert_eq!(entry.duration(), duration);
        assert_eq!(entry.price(), price);
        assert_eq!(entry.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let duration1 = 3600;
        let price1 = 0.25;
        let duration2 = 7200;
        let price2 = 0.30;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut entry = EVAbsolutePriceScheduleEntryType::new(duration1, price1);

        entry
            .set_duration(duration2)
            .set_price(price2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(entry.duration(), duration2);
        assert_eq!(entry.price(), price2);
        assert_eq!(entry.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        entry.set_custom_data(None);
        assert_eq!(entry.custom_data(), None);
    }
}
