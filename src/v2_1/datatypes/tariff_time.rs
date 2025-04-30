use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, tariff_time_price::TariffTimePriceType};

/// Time tariff structure defining time-based costs.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffTimeType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Time-based price per hour.
    pub time_price: TariffTimePriceType,

    /// Optional. Maximum duration in seconds that can be charged under this tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<i32>,

    /// Optional. Minimum duration in seconds that must be charged under this tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_duration: Option<i32>,
}

impl TariffTimeType {
    /// Creates a new `TariffTimeType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `time_price` - Time-based price per hour
    ///
    /// # Returns
    ///
    /// A new instance of `TariffTimeType` with optional fields set to `None`
    pub fn new(time_price: TariffTimePriceType) -> Self {
        Self {
            time_price,
            custom_data: None,
            max_duration: None,
            min_duration: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this tariff time
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the maximum duration.
    ///
    /// # Arguments
    ///
    /// * `max_duration` - Maximum duration in seconds that can be charged under this tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_max_duration(mut self, max_duration: i32) -> Self {
        self.max_duration = Some(max_duration);
        self
    }

    /// Sets the minimum duration.
    ///
    /// # Arguments
    ///
    /// * `min_duration` - Minimum duration in seconds that must be charged under this tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_min_duration(mut self, min_duration: i32) -> Self {
        self.min_duration = Some(min_duration);
        self
    }

    /// Gets the time price.
    ///
    /// # Returns
    ///
    /// A reference to the time-based price per hour
    pub fn time_price(&self) -> &TariffTimePriceType {
        &self.time_price
    }

    /// Sets the time price.
    ///
    /// # Arguments
    ///
    /// * `time_price` - Time-based price per hour
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_time_price(&mut self, time_price: TariffTimePriceType) -> &mut Self {
        self.time_price = time_price;
        self
    }

    /// Gets the maximum duration.
    ///
    /// # Returns
    ///
    /// An optional maximum duration in seconds that can be charged under this tariff
    pub fn max_duration(&self) -> Option<i32> {
        self.max_duration
    }

    /// Sets the maximum duration.
    ///
    /// # Arguments
    ///
    /// * `max_duration` - Maximum duration in seconds that can be charged under this tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_max_duration(&mut self, max_duration: Option<i32>) -> &mut Self {
        self.max_duration = max_duration;
        self
    }

    /// Gets the minimum duration.
    ///
    /// # Returns
    ///
    /// An optional minimum duration in seconds that must be charged under this tariff
    pub fn min_duration(&self) -> Option<i32> {
        self.min_duration
    }

    /// Sets the minimum duration.
    ///
    /// # Arguments
    ///
    /// * `min_duration` - Minimum duration in seconds that must be charged under this tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_min_duration(&mut self, min_duration: Option<i32>) -> &mut Self {
        self.min_duration = min_duration;
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
    /// * `custom_data` - Custom data for this tariff time, or None to clear
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
    fn test_new_tariff_time() {
        let time_price = TariffTimePriceType::new(10.0);
        let tariff_time = TariffTimeType::new(time_price.clone());

        assert_eq!(tariff_time.time_price(), &time_price);
        assert_eq!(tariff_time.max_duration(), None);
        assert_eq!(tariff_time.min_duration(), None);
        assert_eq!(tariff_time.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let time_price = TariffTimePriceType::new(10.0);
        let max_duration = 3600;
        let min_duration = 300;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let tariff_time = TariffTimeType::new(time_price.clone())
            .with_max_duration(max_duration)
            .with_min_duration(min_duration)
            .with_custom_data(custom_data.clone());

        assert_eq!(tariff_time.time_price(), &time_price);
        assert_eq!(tariff_time.max_duration(), Some(max_duration));
        assert_eq!(tariff_time.min_duration(), Some(min_duration));
        assert_eq!(tariff_time.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let time_price1 = TariffTimePriceType::new(10.0);
        let time_price2 = TariffTimePriceType::new(15.0);
        let max_duration = 3600;
        let min_duration = 300;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut tariff_time = TariffTimeType::new(time_price1);

        tariff_time
            .set_time_price(time_price2.clone())
            .set_max_duration(Some(max_duration))
            .set_min_duration(Some(min_duration))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(tariff_time.time_price(), &time_price2);
        assert_eq!(tariff_time.max_duration(), Some(max_duration));
        assert_eq!(tariff_time.min_duration(), Some(min_duration));
        assert_eq!(tariff_time.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        tariff_time
            .set_max_duration(None)
            .set_min_duration(None)
            .set_custom_data(None);

        assert_eq!(tariff_time.max_duration(), None);
        assert_eq!(tariff_time.min_duration(), None);
        assert_eq!(tariff_time.custom_data(), None);
    }
}
