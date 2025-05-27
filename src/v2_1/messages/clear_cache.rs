use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::ClearCacheStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the ClearCache request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClearCacheRequest {
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

/// Response body for the ClearCache response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheResponse {
    pub status: ClearCacheStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClearCacheResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: ClearCacheStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: ClearCacheStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &ClearCacheStatusEnumType {
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

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("200".to_string())
    }

    #[test]
    fn test_clear_cache_request_new() {
        let request = ClearCacheRequest::new();

        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_clear_cache_request_validation() {
        let request = ClearCacheRequest::new();

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_clear_cache_request_serialization() {
        let request = ClearCacheRequest::new();

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ClearCacheRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_clear_cache_request_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = ClearCacheRequest::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_clear_cache_request_set_methods() {
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = ClearCacheRequest::new();

        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_clear_cache_response_new() {
        let status = ClearCacheStatusEnumType::Accepted;
        let response = ClearCacheResponse::new(status.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_clear_cache_response_validation() {
        let status = ClearCacheStatusEnumType::Accepted;
        let response = ClearCacheResponse::new(status);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_clear_cache_response_serialization() {
        let status = ClearCacheStatusEnumType::Accepted;
        let response = ClearCacheResponse::new(status);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ClearCacheResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_clear_cache_response_with_status_info() {
        let status = ClearCacheStatusEnumType::Accepted;
        let status_info = create_test_status_info();

        let response = ClearCacheResponse::new(status)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
    }

    #[test]
    fn test_clear_cache_response_with_custom_data() {
        let status = ClearCacheStatusEnumType::Accepted;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = ClearCacheResponse::new(status)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_clear_cache_response_set_methods() {
        let status = ClearCacheStatusEnumType::Accepted;
        let new_status = ClearCacheStatusEnumType::Rejected;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = ClearCacheResponse::new(status);

        response
            .set_status(new_status.clone())
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_status(), &new_status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_clear_cache_response_all_status_values() {
        let status_values = vec![
            ClearCacheStatusEnumType::Accepted,
            ClearCacheStatusEnumType::Rejected,
        ];

        for status in status_values {
            let response = ClearCacheResponse::new(status.clone());
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_clear_cache_response_builder_pattern() {
        let status = ClearCacheStatusEnumType::Accepted;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = ClearCacheResponse::new(status.clone())
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_clear_cache_request_json_round_trip() {
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = ClearCacheRequest::new()
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ClearCacheRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_clear_cache_response_json_round_trip() {
        let status = ClearCacheStatusEnumType::Rejected;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = ClearCacheResponse::new(status)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ClearCacheResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_clear_cache_response_with_detailed_status_info() {
        let status = ClearCacheStatusEnumType::Rejected;
        let status_info = StatusInfoType::new("CacheNotCleared".to_string())
            .with_additional_info("Cache could not be cleared due to ongoing transactions".to_string());

        let response = ClearCacheResponse::new(status)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
        assert!(response.validate().is_ok());

        // Test serialization
        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ClearCacheResponse = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_clear_cache_request_clear_optional_fields() {
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = ClearCacheRequest::new()
            .with_custom_data(custom_data);

        // Verify custom data is set
        assert!(request.get_custom_data().is_some());

        // Clear custom data
        request.set_custom_data(None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_clear_cache_response_clear_optional_fields() {
        let status = ClearCacheStatusEnumType::Accepted;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = ClearCacheResponse::new(status)
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

    #[test]
    fn test_clear_cache_request_with_complex_custom_data() {
        use serde_json::json;

        let custom_data = CustomDataType::new("CacheVendor".to_string())
            .with_property("cache_type".to_string(), json!("authorization"))
            .with_property("force_clear".to_string(), json!(true))
            .with_property("metadata".to_string(), json!({
                "requested_by": "operator",
                "reason": "security_update"
            }));

        let request = ClearCacheRequest::new()
            .with_custom_data(custom_data);

        assert!(request.validate().is_ok());

        // Test serialization with complex custom data
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ClearCacheRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_clear_cache_response_status_semantics() {
        // Test Accepted status - cache was successfully cleared
        let accepted_response = ClearCacheResponse::new(ClearCacheStatusEnumType::Accepted)
            .with_status_info(StatusInfoType::new("Success".to_string())
                .with_additional_info("Authorization cache cleared successfully".to_string()));

        assert_eq!(accepted_response.get_status(), &ClearCacheStatusEnumType::Accepted);
        assert!(accepted_response.validate().is_ok());

        // Test Rejected status - cache could not be cleared
        let rejected_response = ClearCacheResponse::new(ClearCacheStatusEnumType::Rejected)
            .with_status_info(StatusInfoType::new("CacheInUse".to_string())
                .with_additional_info("Cache cannot be cleared while transactions are active".to_string()));

        assert_eq!(rejected_response.get_status(), &ClearCacheStatusEnumType::Rejected);
        assert!(rejected_response.validate().is_ok());
    }

    #[test]
    fn test_clear_cache_request_minimal_valid() {
        // Test minimal valid request (no fields required)
        let request = ClearCacheRequest::new();

        assert_eq!(request.get_custom_data(), None);
        assert!(request.validate().is_ok());

        // Test serialization of minimal request
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ClearCacheRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_clear_cache_response_minimal_valid() {
        // Test minimal valid response (only required fields)
        let response = ClearCacheResponse::new(ClearCacheStatusEnumType::Accepted);

        assert_eq!(response.get_status(), &ClearCacheStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
        assert!(response.validate().is_ok());

        // Test serialization of minimal response
        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ClearCacheResponse = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_clear_cache_request_empty_json() {
        // Test that an empty JSON object deserializes correctly
        let json = "{}";
        let request: ClearCacheRequest = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(request.get_custom_data(), None);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_clear_cache_response_with_various_status_info() {
        // Test with different status info scenarios
        let scenarios = vec![
            (
                ClearCacheStatusEnumType::Accepted,
                StatusInfoType::new("CacheCleared".to_string())
                    .with_additional_info("All cached authorization data has been removed".to_string())
            ),
            (
                ClearCacheStatusEnumType::Rejected,
                StatusInfoType::new("NotSupported".to_string())
                    .with_additional_info("Cache clearing is not supported in current mode".to_string())
            ),
            (
                ClearCacheStatusEnumType::Rejected,
                StatusInfoType::new("SystemBusy".to_string())
                    .with_additional_info("System is too busy to clear cache at this time".to_string())
            ),
        ];

        for (status, status_info) in scenarios {
            let response = ClearCacheResponse::new(status.clone())
                .with_status_info(status_info.clone());

            assert_eq!(response.get_status(), &status);
            assert_eq!(response.get_status_info(), Some(&status_info));
            assert!(response.validate().is_ok());

            // Test serialization
            let json = serde_json::to_string(&response).expect("Failed to serialize");
            let deserialized: ClearCacheResponse = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(response, deserialized);
        }
    }

    #[test]
    fn test_clear_cache_request_builder_pattern() {
        // Test that builder pattern works correctly
        let custom_data = CustomDataType::new("BuilderVendor".to_string());

        let request = ClearCacheRequest::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_clear_cache_response_builder_pattern_chaining() {
        // Test that builder pattern allows method chaining
        let status = ClearCacheStatusEnumType::Accepted;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("ChainVendor".to_string());

        let response = ClearCacheResponse::new(status.clone())
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_clear_cache_request_default_behavior() {
        // Test that new() creates a request with no optional fields set
        let request = ClearCacheRequest::new();

        assert_eq!(request.get_custom_data(), None);

        // Verify it serializes to minimal JSON
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_clear_cache_response_status_only() {
        // Test response with only status field (minimal case)
        for status in [ClearCacheStatusEnumType::Accepted, ClearCacheStatusEnumType::Rejected] {
            let response = ClearCacheResponse::new(status.clone());

            assert_eq!(response.get_status(), &status);
            assert_eq!(response.get_status_info(), None);
            assert_eq!(response.get_custom_data(), None);
            assert!(response.validate().is_ok());

            // Test JSON serialization includes only status
            let json = serde_json::to_string(&response).expect("Failed to serialize");
            let deserialized: ClearCacheResponse = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(response, deserialized);
        }
    }
}