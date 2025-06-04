use crate::v2_1::datatypes::{CustomDataType, PeriodicEventStreamParamsType, StatusInfoType};
use crate::v2_1::enumerations::GenericStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the AdjustPeriodicEventStream request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdjustPeriodicEventStreamRequest {
    #[validate(range(min = 0))]
    pub id: i32,

    #[validate(nested)]
    pub params: PeriodicEventStreamParamsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl AdjustPeriodicEventStreamRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `id` - The id field
    /// * `params` - The params field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(id: i32, params: PeriodicEventStreamParamsType) -> Self {
        Self {
            id,
            params,
            custom_data: None,
        }
    }

    /// Sets the id field.
    ///
    /// * `id` - The id field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    /// Sets the params field.
    ///
    /// * `params` - The params field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_params(&mut self, params: PeriodicEventStreamParamsType) -> &mut Self {
        self.params = params;
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
    /// The id field
    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    /// Gets a reference to the params field.
    ///
    /// # Returns
    ///
    /// The params field
    pub fn get_params(&self) -> &PeriodicEventStreamParamsType {
        &self.params
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

/// Response body for the AdjustPeriodicEventStream response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdjustPeriodicEventStreamResponse {
    pub status: GenericStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl AdjustPeriodicEventStreamResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: GenericStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
            custom_data: None,
        }
    }

    /// Sets the status field.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_status(&mut self, status: GenericStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Sets the status_info field.
    ///
    /// * `status_info` - The status_info field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_status_info(&mut self, status_info: Option<StatusInfoType>) -> &mut Self {
        self.status_info = status_info;
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

    /// Gets a reference to the status field.
    ///
    /// # Returns
    ///
    /// The status field
    pub fn get_status(&self) -> &GenericStatusEnumType {
        &self.status
    }

    /// Gets a reference to the status_info field.
    ///
    /// # Returns
    ///
    /// The status_info field
    pub fn get_status_info(&self) -> Option<&StatusInfoType> {
        self.status_info.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the status_info field and returns self for builder pattern.
    ///
    /// * `status_info` - The status_info field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_status_info(mut self, status_info: StatusInfoType) -> Self {
        self.status_info = Some(status_info);
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
    use validator::Validate;

    fn create_test_params() -> PeriodicEventStreamParamsType {
        PeriodicEventStreamParamsType::new(60, 10)
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("200".to_string())
    }

    #[test]
    fn test_adjust_periodic_event_stream_request_new() {
        let id = 123;
        let params = create_test_params();
        let request = AdjustPeriodicEventStreamRequest::new(id, params.clone());

        assert_eq!(request.get_id(), &id);
        assert_eq!(request.get_params(), &params);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_adjust_periodic_event_stream_request_validation() {
        let id = 123;
        let params = create_test_params();
        let request = AdjustPeriodicEventStreamRequest::new(id, params);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_adjust_periodic_event_stream_request_validation_invalid_id() {
        let id = -1; // Invalid: id must be >= 0
        let params = create_test_params();
        let request = AdjustPeriodicEventStreamRequest::new(id, params);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_adjust_periodic_event_stream_request_serialization() {
        let id = 123;
        let params = create_test_params();
        let request = AdjustPeriodicEventStreamRequest::new(id, params);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: AdjustPeriodicEventStreamRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_adjust_periodic_event_stream_request_with_custom_data() {
        let id = 123;
        let params = create_test_params();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = AdjustPeriodicEventStreamRequest::new(id, params)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_adjust_periodic_event_stream_request_set_methods() {
        let id = 123;
        let new_id = 456;
        let params = create_test_params();
        let new_params = PeriodicEventStreamParamsType::new(120, 20);
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = AdjustPeriodicEventStreamRequest::new(id, params);

        request
            .set_id(new_id)
            .set_params(new_params.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_id(), &new_id);
        assert_eq!(request.get_params(), &new_params);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_adjust_periodic_event_stream_request_edge_cases() {
        // Test with minimum valid id
        let id = 0;
        let params = create_test_params();
        let request = AdjustPeriodicEventStreamRequest::new(id, params);

        assert_eq!(request.get_id(), &0);
        assert!(request.validate().is_ok());

        // Test with large id
        let id = i32::MAX;
        let params = create_test_params();
        let request = AdjustPeriodicEventStreamRequest::new(id, params);

        assert_eq!(request.get_id(), &i32::MAX);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_adjust_periodic_event_stream_request_with_complex_params() {
        let id = 123;
        let custom_data = CustomDataType::new("ParamsVendor".to_string());
        let params = PeriodicEventStreamParamsType::new(3600, 100)
            .with_custom_data(custom_data);

        let request = AdjustPeriodicEventStreamRequest::new(id, params);

        assert!(request.validate().is_ok());

        // Test serialization with complex data
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: AdjustPeriodicEventStreamRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_adjust_periodic_event_stream_response_new() {
        let status = GenericStatusEnumType::Accepted;
        let response = AdjustPeriodicEventStreamResponse::new(status.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_adjust_periodic_event_stream_response_validation() {
        let status = GenericStatusEnumType::Accepted;
        let response = AdjustPeriodicEventStreamResponse::new(status);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_adjust_periodic_event_stream_response_serialization() {
        let status = GenericStatusEnumType::Accepted;
        let response = AdjustPeriodicEventStreamResponse::new(status);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: AdjustPeriodicEventStreamResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_adjust_periodic_event_stream_response_with_status_info() {
        let status = GenericStatusEnumType::Accepted;
        let status_info = create_test_status_info();

        let response = AdjustPeriodicEventStreamResponse::new(status)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
    }

    #[test]
    fn test_adjust_periodic_event_stream_response_with_custom_data() {
        let status = GenericStatusEnumType::Accepted;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = AdjustPeriodicEventStreamResponse::new(status)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_adjust_periodic_event_stream_response_set_methods() {
        let status = GenericStatusEnumType::Accepted;
        let new_status = GenericStatusEnumType::Rejected;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = AdjustPeriodicEventStreamResponse::new(status);

        response
            .set_status(new_status.clone())
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_status(), &new_status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_adjust_periodic_event_stream_response_all_status_values() {
        let status_values = vec![
            GenericStatusEnumType::Accepted,
            GenericStatusEnumType::Rejected,
        ];

        for status in status_values {
            let response = AdjustPeriodicEventStreamResponse::new(status.clone());
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_adjust_periodic_event_stream_response_builder_pattern() {
        let status = GenericStatusEnumType::Accepted;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = AdjustPeriodicEventStreamResponse::new(status.clone())
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_adjust_periodic_event_stream_request_json_round_trip() {
        let id = 123;
        let params = create_test_params();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = AdjustPeriodicEventStreamRequest::new(id, params)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: AdjustPeriodicEventStreamRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_adjust_periodic_event_stream_response_json_round_trip() {
        let status = GenericStatusEnumType::Rejected;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = AdjustPeriodicEventStreamResponse::new(status)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: AdjustPeriodicEventStreamResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_adjust_periodic_event_stream_request_params_validation() {
        let id = 123;

        // Test with valid params
        let valid_params = PeriodicEventStreamParamsType::new(60, 10);
        let request = AdjustPeriodicEventStreamRequest::new(id, valid_params);
        assert!(request.validate().is_ok());

        // Test with edge case params (maximum interval)
        let edge_params = PeriodicEventStreamParamsType::new(86400, 0); // Max interval, min values
        let request = AdjustPeriodicEventStreamRequest::new(id, edge_params);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_adjust_periodic_event_stream_response_with_detailed_status_info() {
        let status = GenericStatusEnumType::Rejected;
        let status_info = StatusInfoType::new("InvalidParams".to_string())
            .with_additional_info("The provided parameters are invalid for this stream".to_string());

        let response = AdjustPeriodicEventStreamResponse::new(status)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
        assert!(response.validate().is_ok());

        // Test serialization
        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: AdjustPeriodicEventStreamResponse = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_adjust_periodic_event_stream_request_clear_optional_fields() {
        let id = 123;
        let params = create_test_params();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = AdjustPeriodicEventStreamRequest::new(id, params)
            .with_custom_data(custom_data);

        // Verify custom data is set
        assert!(request.get_custom_data().is_some());

        // Clear custom data
        request.set_custom_data(None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_adjust_periodic_event_stream_response_clear_optional_fields() {
        let status = GenericStatusEnumType::Accepted;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = AdjustPeriodicEventStreamResponse::new(status)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        // Verify fields are set
        assert!(response.get_status_info().is_some());
        assert!(response.get_custom_data().is_some());

        // Clear optional fields
        response.set_status_info(None);
        response.set_custom_data(None);

        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }
}
