use crate::v2_1::datatypes::custom_data::CustomDataType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// The AbsolutePriceScheduleType is modeled after the same type that is defined in ISO 15118-20,
/// such that if it is supplied by an EMSP as a signed EXI message, the conversion from EXI to JSON
/// (in OCPP) and back to EXI (for ISO 15118-20) does not change the digest and therefore does not
/// invalidate the signature.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AbsolutePriceScheduleType {
    /// Starting point of price schedule.
    pub time_anchor: String, // date-time format

    /// Unique ID of price schedule
    #[serde(rename = "priceScheduleID")]
    pub price_schedule_id: i32,

    /// Description of the price schedule.
    #[serde(rename = "priceScheduleDescription")]
    pub price_schedule_description: Option<String>,

    /// Currency according to ISO 4217.
    pub currency: Option<String>,

    /// String that indicates what language is used for the human readable strings in the price schedule.
    /// Based on ISO 639.
    pub language: Option<String>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl AbsolutePriceScheduleType {
    /// Creates a new `AbsolutePriceScheduleType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `time_anchor` - Starting point of price schedule in date-time format
    /// * `price_schedule_id` - Unique ID of price schedule
    ///
    /// # Returns
    ///
    /// A new instance of `AbsolutePriceScheduleType` with optional fields set to `None`
    pub fn new(time_anchor: String, price_schedule_id: i32) -> Self {
        Self {
            time_anchor,
            price_schedule_id,
            price_schedule_description: None,
            currency: None,
            language: None,
            custom_data: None,
        }
    }

    /// Sets the price schedule description.
    ///
    /// # Arguments
    ///
    /// * `description` - Description of the price schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_price_schedule_description(mut self, description: String) -> Self {
        self.price_schedule_description = Some(description);
        self
    }

    /// Sets the currency.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency according to ISO 4217
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    /// Sets the language.
    ///
    /// # Arguments
    ///
    /// * `language` - Language code based on ISO 639
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
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
    /// The time anchor as a string in date-time format
    pub fn time_anchor(&self) -> &str {
        &self.time_anchor
    }

    /// Sets the time anchor.
    ///
    /// # Arguments
    ///
    /// * `time_anchor` - Starting point of price schedule in date-time format
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_time_anchor(&mut self, time_anchor: String) -> &mut Self {
        self.time_anchor = time_anchor;
        self
    }

    /// Gets the price schedule ID.
    ///
    /// # Returns
    ///
    /// The unique ID of the price schedule
    pub fn price_schedule_id(&self) -> i32 {
        self.price_schedule_id
    }

    /// Sets the price schedule ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique ID of price schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price_schedule_id(&mut self, id: i32) -> &mut Self {
        self.price_schedule_id = id;
        self
    }

    /// Gets the price schedule description.
    ///
    /// # Returns
    ///
    /// An optional reference to the price schedule description
    pub fn price_schedule_description(&self) -> Option<&String> {
        self.price_schedule_description.as_ref()
    }

    /// Sets the price schedule description.
    ///
    /// # Arguments
    ///
    /// * `description` - Description of the price schedule, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price_schedule_description(&mut self, description: Option<String>) -> &mut Self {
        self.price_schedule_description = description;
        self
    }

    /// Gets the currency.
    ///
    /// # Returns
    ///
    /// An optional reference to the currency code
    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }

    /// Sets the currency.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency according to ISO 4217, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_currency(&mut self, currency: Option<String>) -> &mut Self {
        self.currency = currency;
        self
    }

    /// Gets the language.
    ///
    /// # Returns
    ///
    /// An optional reference to the language code
    pub fn language(&self) -> Option<&String> {
        self.language.as_ref()
    }

    /// Sets the language.
    ///
    /// # Arguments
    ///
    /// * `language` - Language code based on ISO 639, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_language(&mut self, language: Option<String>) -> &mut Self {
        self.language = language;
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
    fn test_new_absolute_price_schedule() {
        let schedule = AbsolutePriceScheduleType::new(
            "2023-01-01T00:00:00Z".to_string(),
            123,
        );

        assert_eq!(schedule.time_anchor(), "2023-01-01T00:00:00Z");
        assert_eq!(schedule.price_schedule_id(), 123);
        assert_eq!(schedule.price_schedule_description(), None);
        assert_eq!(schedule.currency(), None);
        assert_eq!(schedule.language(), None);
        assert_eq!(schedule.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let schedule = AbsolutePriceScheduleType::new(
            "2023-01-01T00:00:00Z".to_string(),
            123,
        )
        .with_price_schedule_description("Test Schedule".to_string())
        .with_currency("USD".to_string())
        .with_language("en".to_string())
        .with_custom_data(custom_data.clone());

        assert_eq!(schedule.time_anchor(), "2023-01-01T00:00:00Z");
        assert_eq!(schedule.price_schedule_id(), 123);
        assert_eq!(schedule.price_schedule_description(), Some(&"Test Schedule".to_string()));
        assert_eq!(schedule.currency(), Some(&"USD".to_string()));
        assert_eq!(schedule.language(), Some(&"en".to_string()));
        assert_eq!(schedule.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut schedule = AbsolutePriceScheduleType::new(
            "2023-01-01T00:00:00Z".to_string(),
            123,
        );

        schedule
            .set_time_anchor("2023-02-01T00:00:00Z".to_string())
            .set_price_schedule_id(456)
            .set_price_schedule_description(Some("Updated Schedule".to_string()))
            .set_currency(Some("EUR".to_string()))
            .set_language(Some("fr".to_string()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(schedule.time_anchor(), "2023-02-01T00:00:00Z");
        assert_eq!(schedule.price_schedule_id(), 456);
        assert_eq!(schedule.price_schedule_description(), Some(&"Updated Schedule".to_string()));
        assert_eq!(schedule.currency(), Some(&"EUR".to_string()));
        assert_eq!(schedule.language(), Some(&"fr".to_string()));
        assert_eq!(schedule.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        schedule
            .set_price_schedule_description(None)
            .set_currency(None)
            .set_language(None)
            .set_custom_data(None);

        assert_eq!(schedule.price_schedule_description(), None);
        assert_eq!(schedule.currency(), None);
        assert_eq!(schedule.language(), None);
        assert_eq!(schedule.custom_data(), None);
    }
}
