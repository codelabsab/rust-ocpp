use crate::v2_1::datatypes::{ConstantStreamDataType, CustomDataType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetPeriodicEventStream request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetPeriodicEventStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetPeriodicEventStreamRequest {
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

/// Response body for the GetPeriodicEventStream response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetPeriodicEventStreamResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub constant_stream_data: Option<Vec<ConstantStreamDataType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetPeriodicEventStreamResponse {
    /// Creates a new instance of the struct.
    ///
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new() -> Self {
        Self {
            constant_stream_data: None,
            custom_data: None,
        }
    }

    /// Sets the constant_stream_data field.
    ///
    /// * `constant_stream_data` - The constant_stream_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_constant_stream_data(&mut self, constant_stream_data: Option<Vec<ConstantStreamDataType>>) -> &mut Self {
        self.constant_stream_data = constant_stream_data;
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

    /// Gets a reference to the constant_stream_data field.
    ///
    /// # Returns
    ///
    /// The constant_stream_data field
    pub fn get_constant_stream_data(&self) -> Option<&Vec<ConstantStreamDataType>> {
        self.constant_stream_data.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the constant_stream_data field and returns self for builder pattern.
    ///
    /// * `constant_stream_data` - The constant_stream_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_constant_stream_data(mut self, constant_stream_data: Vec<ConstantStreamDataType>) -> Self {
        self.constant_stream_data = Some(constant_stream_data);
        self
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

    fn create_test_constant_stream_data() -> ConstantStreamDataType {
        use crate::v2_1::datatypes::PeriodicEventStreamParamsType;
        let params = PeriodicEventStreamParamsType::new(60, 10);
        ConstantStreamDataType::new(
            1,
            params,
            100,
        )
    }

    // Tests for GetPeriodicEventStreamRequest

    #[test]
    fn test_get_periodic_event_stream_request_new() {
        let request = GetPeriodicEventStreamRequest::new();

        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_periodic_event_stream_request_with_custom_data() {
        let custom_data = create_test_custom_data();
        let request = GetPeriodicEventStreamRequest::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_periodic_event_stream_request_setters() {
        let custom_data = create_test_custom_data();

        let mut request = GetPeriodicEventStreamRequest::new();
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_periodic_event_stream_request_getters() {
        let custom_data = create_test_custom_data();
        let request = GetPeriodicEventStreamRequest::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_periodic_event_stream_request_serialization() {
        let request = GetPeriodicEventStreamRequest::new();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetPeriodicEventStreamRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
    }

    #[test]
    fn test_get_periodic_event_stream_request_validation() {
        let request = GetPeriodicEventStreamRequest::new();

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_get_periodic_event_stream_request_json_round_trip() {
        let custom_data = create_test_custom_data();
        let request = GetPeriodicEventStreamRequest::new()
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetPeriodicEventStreamRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
        assert!(parsed.validate().is_ok());
    }

    // Tests for GetPeriodicEventStreamResponse

    #[test]
    fn test_get_periodic_event_stream_response_new() {
        let response = GetPeriodicEventStreamResponse::new();

        assert_eq!(response.constant_stream_data, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_periodic_event_stream_response_with_constant_stream_data() {
        let stream_data = vec![create_test_constant_stream_data()];
        let response = GetPeriodicEventStreamResponse::new()
            .with_constant_stream_data(stream_data.clone());

        assert_eq!(response.constant_stream_data, Some(stream_data));
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_periodic_event_stream_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = GetPeriodicEventStreamResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.constant_stream_data, None);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_periodic_event_stream_response_setters() {
        let stream_data = vec![create_test_constant_stream_data()];
        let custom_data = create_test_custom_data();

        let mut response = GetPeriodicEventStreamResponse::new();
        response.set_constant_stream_data(Some(stream_data.clone()));
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.constant_stream_data, Some(stream_data));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_periodic_event_stream_response_getters() {
        let stream_data = vec![create_test_constant_stream_data()];
        let custom_data = create_test_custom_data();
        let response = GetPeriodicEventStreamResponse::new()
            .with_constant_stream_data(stream_data.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_constant_stream_data(), Some(&stream_data));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_periodic_event_stream_response_serialization() {
        let response = GetPeriodicEventStreamResponse::new();

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetPeriodicEventStreamResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
    }

    #[test]
    fn test_get_periodic_event_stream_response_validation() {
        let response = GetPeriodicEventStreamResponse::new();

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_get_periodic_event_stream_response_validation_empty_constant_stream_data() {
        let mut response = GetPeriodicEventStreamResponse::new();
        response.set_constant_stream_data(Some(vec![])); // Empty list should fail validation

        assert!(response.validate().is_err());
    }

    #[test]
    fn test_get_periodic_event_stream_response_multiple_stream_data() {
        use crate::v2_1::datatypes::PeriodicEventStreamParamsType;
        let params1 = PeriodicEventStreamParamsType::new(60, 10);
        let params2 = PeriodicEventStreamParamsType::new(120, 20);
        let stream_data1 = ConstantStreamDataType::new(1, params1, 101);
        let stream_data2 = ConstantStreamDataType::new(2, params2, 102);
        let stream_data = vec![stream_data1, stream_data2];

        let response = GetPeriodicEventStreamResponse::new()
            .with_constant_stream_data(stream_data.clone());

        assert_eq!(response.constant_stream_data, Some(stream_data));
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_get_periodic_event_stream_response_json_round_trip() {
        let stream_data = vec![create_test_constant_stream_data()];
        let custom_data = create_test_custom_data();
        let response = GetPeriodicEventStreamResponse::new()
            .with_constant_stream_data(stream_data)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetPeriodicEventStreamResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
        assert!(parsed.validate().is_ok());
    }
}