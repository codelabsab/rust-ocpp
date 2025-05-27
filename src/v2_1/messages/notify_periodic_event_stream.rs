use crate::v2_1::datatypes::{CustomDataType, StreamDataElementType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// NotifyPeriodicEventStream message structure.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyPeriodicEventStream {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub data: Vec<StreamDataElementType>,

    /// Id of stream.
    #[validate(range(min = 0))]
    pub id: i32,

    /// Number of data elements still pending to be sent.
    #[validate(range(min = 0))]
    pub pending: i32,

    /// Base timestamp to add to time offset of values.
    pub basetime: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyPeriodicEventStream {
    /// Creates a new instance of the struct.
    ///
    /// * `data` - The data field
    /// * `id` - Id of stream.
    /// * `pending` - Number of data elements still pending to be sent.
    /// * `basetime` - Base timestamp to add to time offset of values.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(data: Vec<StreamDataElementType>, id: i32, pending: i32, basetime: DateTime<Utc>) -> Self {
        Self {
            data,
            id,
            pending,
            basetime,
            custom_data: None,
        }
    }

    /// Sets the data field.
    ///
    /// * `data` - The data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_data(&mut self, data: Vec<StreamDataElementType>) -> &mut Self {
        self.data = data;
        self
    }

    /// Sets the id field.
    ///
    /// * `id` - Id of stream.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    /// Sets the pending field.
    ///
    /// * `pending` - Number of data elements still pending to be sent.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_pending(&mut self, pending: i32) -> &mut Self {
        self.pending = pending;
        self
    }

    /// Sets the basetime field.
    ///
    /// * `basetime` - Base timestamp to add to time offset of values.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_basetime(&mut self, basetime: DateTime<Utc>) -> &mut Self {
        self.basetime = basetime;
        self
    }

    /// Sets the custom_data field.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets a reference to the data field.
    ///
    /// # Returns
    ///
    /// The data field
    pub fn get_data(&self) -> &Vec<StreamDataElementType> {
        &self.data
    }

    /// Gets a reference to the id field.
    ///
    /// # Returns
    ///
    /// Id of stream.
    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    /// Gets a reference to the pending field.
    ///
    /// # Returns
    ///
    /// Number of data elements still pending to be sent.
    pub fn get_pending(&self) -> &i32 {
        &self.pending
    }

    /// Gets a reference to the basetime field.
    ///
    /// # Returns
    ///
    /// Base timestamp to add to time offset of values.
    pub fn get_basetime(&self) -> &DateTime<Utc> {
        &self.basetime
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom_data field and returns self for builder pattern.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rust_decimal::Decimal;
    use std::str::FromStr;
    use serde_json;
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_stream_data_element() -> StreamDataElementType {
        StreamDataElementType::new(Decimal::from_str("123.45").unwrap(), "test_value".to_string())
    }

    fn create_test_notify_periodic_event_stream() -> NotifyPeriodicEventStream {
        let data = vec![create_test_stream_data_element()];
        let basetime = Utc::now();
        NotifyPeriodicEventStream::new(data, 1, 5, basetime)
    }

    #[test]
    fn test_notify_periodic_event_stream_new() {
        let data = vec![create_test_stream_data_element()];
        let id = 42;
        let pending = 10;
        let basetime = Utc::now();

        let stream = NotifyPeriodicEventStream::new(data.clone(), id, pending, basetime);

        assert_eq!(stream.data, data);
        assert_eq!(stream.id, id);
        assert_eq!(stream.pending, pending);
        assert_eq!(stream.basetime, basetime);
        assert!(stream.custom_data.is_none());
    }

    #[test]
    fn test_notify_periodic_event_stream_serialization() {
        let stream = create_test_notify_periodic_event_stream();

        let json = serde_json::to_string(&stream).expect("Failed to serialize");
        let deserialized: NotifyPeriodicEventStream = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(stream, deserialized);
    }

    #[test]
    fn test_notify_periodic_event_stream_validation_valid() {
        let stream = create_test_notify_periodic_event_stream();
        assert!(stream.validate().is_ok());
    }

    #[test]
    fn test_notify_periodic_event_stream_validation_empty_data() {
        let basetime = Utc::now();
        let stream = NotifyPeriodicEventStream::new(vec![], 1, 5, basetime);

        let validation_result = stream.validate();
        assert!(validation_result.is_err());
        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("data"));
    }

    #[test]
    fn test_notify_periodic_event_stream_validation_negative_id() {
        let data = vec![create_test_stream_data_element()];
        let basetime = Utc::now();
        let stream = NotifyPeriodicEventStream::new(data, -1, 5, basetime);

        let validation_result = stream.validate();
        assert!(validation_result.is_err());
        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("id"));
    }

    #[test]
    fn test_notify_periodic_event_stream_validation_negative_pending() {
        let data = vec![create_test_stream_data_element()];
        let basetime = Utc::now();
        let stream = NotifyPeriodicEventStream::new(data, 1, -1, basetime);

        let validation_result = stream.validate();
        assert!(validation_result.is_err());
        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("pending"));
    }

    #[test]
    fn test_notify_periodic_event_stream_set_methods() {
        let mut stream = create_test_notify_periodic_event_stream();
        let new_data = vec![create_test_stream_data_element(), create_test_stream_data_element()];
        let new_id = 99;
        let new_pending = 20;
        let new_basetime = Utc::now();

        stream.set_data(new_data.clone())
              .set_id(new_id)
              .set_pending(new_pending)
              .set_basetime(new_basetime);

        assert_eq!(stream.data, new_data);
        assert_eq!(stream.id, new_id);
        assert_eq!(stream.pending, new_pending);
        assert_eq!(stream.basetime, new_basetime);
    }

    #[test]
    fn test_notify_periodic_event_stream_get_methods() {
        let stream = create_test_notify_periodic_event_stream();

        assert_eq!(stream.get_data(), &stream.data);
        assert_eq!(stream.get_id(), &stream.id);
        assert_eq!(stream.get_pending(), &stream.pending);
        assert_eq!(stream.get_basetime(), &stream.basetime);
        assert_eq!(stream.get_custom_data(), None);
    }

    #[test]
    fn test_notify_periodic_event_stream_with_custom_data() {
        let custom_data = create_test_custom_data();
        let stream = create_test_notify_periodic_event_stream()
            .with_custom_data(custom_data.clone());

        assert_eq!(stream.custom_data, Some(custom_data));
        assert_eq!(stream.get_custom_data(), stream.custom_data.as_ref());
    }

    #[test]
    fn test_notify_periodic_event_stream_set_custom_data() {
        let mut stream = create_test_notify_periodic_event_stream();
        let custom_data = create_test_custom_data();

        stream.set_custom_data(Some(custom_data.clone()));
        assert_eq!(stream.custom_data, Some(custom_data));

        stream.set_custom_data(None);
        assert!(stream.custom_data.is_none());
    }

    #[test]
    fn test_notify_periodic_event_stream_boundary_values() {
        let data = vec![create_test_stream_data_element()];
        let basetime = Utc::now();

        // Test minimum valid values
        let stream_min = NotifyPeriodicEventStream::new(data.clone(), 0, 0, basetime);
        assert!(stream_min.validate().is_ok());

        // Test large valid values
        let stream_max = NotifyPeriodicEventStream::new(data, i32::MAX, i32::MAX, basetime);
        assert!(stream_max.validate().is_ok());
    }

    #[test]
    fn test_notify_periodic_event_stream_json_format() {
        let stream = create_test_notify_periodic_event_stream();
        let json = serde_json::to_value(&stream).expect("Failed to serialize to JSON");

        assert!(json.get("data").is_some());
        assert!(json.get("id").is_some());
        assert!(json.get("pending").is_some());
        assert!(json.get("basetime").is_some());

        // Custom data should not be present if None
        if stream.custom_data.is_none() {
            assert!(json.get("customData").is_none());
        }
    }

    #[test]
    fn test_notify_periodic_event_stream_with_multiple_data_elements() {
        let data = vec![
            create_test_stream_data_element(),
            StreamDataElementType::new(Decimal::from_str("456.78").unwrap(), "another_value".to_string()),
            StreamDataElementType::new(Decimal::from_str("789.01").unwrap(), "third_value".to_string()),
        ];
        let basetime = Utc::now();
        let stream = NotifyPeriodicEventStream::new(data.clone(), 1, 5, basetime);

        assert_eq!(stream.data.len(), 3);
        assert!(stream.validate().is_ok());

        let json = serde_json::to_string(&stream).expect("Failed to serialize");
        let deserialized: NotifyPeriodicEventStream = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(stream, deserialized);
    }
}
