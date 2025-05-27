use crate::v2_1::datatypes::CustomDataType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the ClosePeriodicEventStream request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClosePeriodicEventStreamRequest {
    /// Id of stream to close.
    #[validate(range(min = 0))]
    pub id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClosePeriodicEventStreamRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `id` - Id of stream to close.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(id: i32) -> Self {
        Self {
            id,
            custom_data: None,
        }
    }

    /// Sets the id field.
    ///
    /// * `id` - Id of stream to close.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
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

    /// Gets a reference to the id field.
    ///
    /// # Returns
    ///
    /// Id of stream to close.
    pub fn get_id(&self) -> &i32 {
        &self.id
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

/// Response body for the ClosePeriodicEventStream response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClosePeriodicEventStreamResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClosePeriodicEventStreamResponse {
    /// Creates a new instance of the struct.
    ///
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new() -> Self {
        Self {
            custom_data: None,
        }
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
    use serde_json;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    #[test]
    fn test_close_periodic_event_stream_request_new() {
        let id = 123;

        let request = ClosePeriodicEventStreamRequest::new(id);

        assert_eq!(request.id, id);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_close_periodic_event_stream_request_with_custom_data() {
        let id = 456;
        let custom_data = create_test_custom_data();

        let request = ClosePeriodicEventStreamRequest::new(id)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.id, id);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_close_periodic_event_stream_request_setters() {
        let mut request = ClosePeriodicEventStreamRequest::new(1);

        let new_id = 789;
        let custom_data = create_test_custom_data();

        request.set_id(new_id)
               .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.id, new_id);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_close_periodic_event_stream_request_getters() {
        let id = 101;
        let custom_data = create_test_custom_data();

        let request = ClosePeriodicEventStreamRequest::new(id)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_id(), &id);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_close_periodic_event_stream_request_serialization() {
        let id = 202;

        let request = ClosePeriodicEventStreamRequest::new(id);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ClosePeriodicEventStreamRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_close_periodic_event_stream_request_validation_negative_id() {
        let request = ClosePeriodicEventStreamRequest::new(-1);

        let validation_result = request.validate();
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_close_periodic_event_stream_request_validation_valid() {
        let id = 0; // 0 is valid (min = 0)

        let request = ClosePeriodicEventStreamRequest::new(id);

        let validation_result = request.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_close_periodic_event_stream_request_edge_cases() {
        // Test with zero ID
        let request_zero = ClosePeriodicEventStreamRequest::new(0);
        assert_eq!(request_zero.id, 0);
        assert!(request_zero.validate().is_ok());

        // Test with large ID
        let request_large = ClosePeriodicEventStreamRequest::new(999999);
        assert_eq!(request_large.id, 999999);
        assert!(request_large.validate().is_ok());

        // Test serialization with large ID
        let json = serde_json::to_string(&request_large).expect("Failed to serialize");
        let deserialized: ClosePeriodicEventStreamRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request_large, deserialized);
    }

    #[test]
    fn test_close_periodic_event_stream_response_new() {
        let response = ClosePeriodicEventStreamResponse::new();

        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_close_periodic_event_stream_response_with_custom_data() {
        let custom_data = create_test_custom_data();

        let response = ClosePeriodicEventStreamResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_close_periodic_event_stream_response_setters() {
        let mut response = ClosePeriodicEventStreamResponse::new();

        let custom_data = create_test_custom_data();

        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_close_periodic_event_stream_response_getters() {
        let custom_data = create_test_custom_data();

        let response = ClosePeriodicEventStreamResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_close_periodic_event_stream_response_serialization() {
        let response = ClosePeriodicEventStreamResponse::new();

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ClosePeriodicEventStreamResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_close_periodic_event_stream_response_validation() {
        let response = ClosePeriodicEventStreamResponse::new();

        let validation_result = response.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_close_periodic_event_stream_request_builder_pattern() {
        let id = 303;
        let custom_data = create_test_custom_data();

        let request = ClosePeriodicEventStreamRequest::new(id)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.id, id);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_close_periodic_event_stream_response_builder_pattern() {
        let custom_data = create_test_custom_data();

        let response = ClosePeriodicEventStreamResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_close_periodic_event_stream_request_with_and_without_custom_data() {
        // Test request without custom data
        let request_empty = ClosePeriodicEventStreamRequest::new(404);
        assert_eq!(request_empty.custom_data, None);

        let json_empty = serde_json::to_string(&request_empty).expect("Failed to serialize");
        let deserialized_empty: ClosePeriodicEventStreamRequest =
            serde_json::from_str(&json_empty).expect("Failed to deserialize");
        assert_eq!(request_empty, deserialized_empty);

        // Test request with custom data
        let custom_data = create_test_custom_data();
        let request_with_data = ClosePeriodicEventStreamRequest::new(505)
            .with_custom_data(custom_data.clone());
        assert_eq!(request_with_data.custom_data, Some(custom_data));

        let json_with_data = serde_json::to_string(&request_with_data).expect("Failed to serialize");
        let deserialized_with_data: ClosePeriodicEventStreamRequest =
            serde_json::from_str(&json_with_data).expect("Failed to deserialize");
        assert_eq!(request_with_data, deserialized_with_data);
    }

    #[test]
    fn test_close_periodic_event_stream_response_with_and_without_custom_data() {
        // Test response without custom data
        let response_empty = ClosePeriodicEventStreamResponse::new();
        assert_eq!(response_empty.custom_data, None);

        let json_empty = serde_json::to_string(&response_empty).expect("Failed to serialize");
        let deserialized_empty: ClosePeriodicEventStreamResponse =
            serde_json::from_str(&json_empty).expect("Failed to deserialize");
        assert_eq!(response_empty, deserialized_empty);

        // Test response with custom data
        let custom_data = create_test_custom_data();
        let response_with_data = ClosePeriodicEventStreamResponse::new()
            .with_custom_data(custom_data.clone());
        assert_eq!(response_with_data.custom_data, Some(custom_data));

        let json_with_data = serde_json::to_string(&response_with_data).expect("Failed to serialize");
        let deserialized_with_data: ClosePeriodicEventStreamResponse =
            serde_json::from_str(&json_with_data).expect("Failed to deserialize");
        assert_eq!(response_with_data, deserialized_with_data);
    }

    #[test]
    fn test_close_periodic_event_stream_request_multiple_ids() {
        let test_ids = vec![0, 1, 100, 999, 123456];

        for id in test_ids {
            let request = ClosePeriodicEventStreamRequest::new(id);
            assert_eq!(request.id, id);
            assert!(request.validate().is_ok());

            // Test serialization for each ID
            let json = serde_json::to_string(&request).expect("Failed to serialize");
            let deserialized: ClosePeriodicEventStreamRequest =
                serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(request, deserialized);
        }
    }
}