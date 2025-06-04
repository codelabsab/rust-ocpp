use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::ClearMessageStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the ClearDisplayMessage request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearDisplayMessageRequest {
    /// Id of the message that SHALL be removed from the Charging Station.
    #[validate(range(min = 0))]
    pub id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClearDisplayMessageRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `id` - Id of the message that SHALL be removed from the Charging Station.
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
    /// * `id` - Id of the message that SHALL be removed from the Charging Station.
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
    /// Id of the message that SHALL be removed from the Charging Station.
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

/// Response body for the ClearDisplayMessage response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearDisplayMessageResponse {
    pub status: ClearMessageStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClearDisplayMessageResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: ClearMessageStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: ClearMessageStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &ClearMessageStatusEnumType {
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

    #[test]
    fn test_clear_display_message_request_new() {
        let request = ClearDisplayMessageRequest::new(123);
        assert_eq!(request.get_id(), &123);
        assert_eq!(request.get_custom_data(), None);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_clear_display_message_request_validation_invalid_id() {
        let request = ClearDisplayMessageRequest::new(-1); // Invalid: must be >= 0
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_clear_display_message_request_serialization() {
        let request = ClearDisplayMessageRequest::new(456);
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ClearDisplayMessageRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_clear_display_message_response_new() {
        let status = ClearMessageStatusEnumType::Accepted;
        let response = ClearDisplayMessageResponse::new(status.clone());
        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_clear_display_message_response_all_status_values() {
        let status_values = vec![
            ClearMessageStatusEnumType::Accepted,
            ClearMessageStatusEnumType::Unknown,
        ];

        for status in status_values {
            let response = ClearDisplayMessageResponse::new(status.clone());
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_clear_display_message_request_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = ClearDisplayMessageRequest::new(789)
            .with_custom_data(custom_data.clone());
        assert_eq!(request.get_custom_data(), Some(&custom_data));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_clear_display_message_response_with_status_info() {
        let status = ClearMessageStatusEnumType::Unknown;
        let status_info = StatusInfoType::new("NotFound".to_string());
        let response = ClearDisplayMessageResponse::new(status)
            .with_status_info(status_info.clone());
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_clear_display_message_json_round_trip() {
        let request = ClearDisplayMessageRequest::new(999)
            .with_custom_data(CustomDataType::new("TestVendor".to_string()));

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ClearDisplayMessageRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());

        let response = ClearDisplayMessageResponse::new(ClearMessageStatusEnumType::Accepted)
            .with_status_info(StatusInfoType::new("Success".to_string()));

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ClearDisplayMessageResponse = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_clear_display_message_request_all_setters() {
        let mut request = ClearDisplayMessageRequest::new(123);
        let custom_data = CustomDataType::new("TestVendor".to_string());

        // Test setter methods
        request.set_id(456);
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_id(), &456);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_clear_display_message_response_all_setters() {
        let mut response = ClearDisplayMessageResponse::new(ClearMessageStatusEnumType::Accepted);
        let status_info = StatusInfoType::new("Success".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        // Test setter methods
        response.set_status(ClearMessageStatusEnumType::Unknown);
        response.set_status_info(Some(status_info.clone()));
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_status(), &ClearMessageStatusEnumType::Unknown);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_clear_display_message_request_validation_zero_id() {
        let request = ClearDisplayMessageRequest::new(0); // Valid: exactly 0
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_clear_display_message_request_clear_custom_data() {
        let mut request = ClearDisplayMessageRequest::new(789)
            .with_custom_data(CustomDataType::new("TestVendor".to_string()));

        // Verify custom data is set
        assert!(request.get_custom_data().is_some());

        // Clear custom data
        request.set_custom_data(None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_clear_display_message_response_clear_fields() {
        let mut response = ClearDisplayMessageResponse::new(ClearMessageStatusEnumType::Accepted)
            .with_status_info(StatusInfoType::new("Success".to_string()))
            .with_custom_data(CustomDataType::new("TestVendor".to_string()));

        // Clear optional fields
        response.set_status_info(None);
        response.set_custom_data(None);

        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_clear_display_message_request_method_chaining() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        
        let mut request = ClearDisplayMessageRequest::new(100);
        let result = request
            .set_id(200)
            .set_custom_data(Some(custom_data.clone()));

        // Verify chaining returns self
        assert_eq!(result.get_id(), &200);
        assert_eq!(result.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_clear_display_message_response_method_chaining() {
        let status_info = StatusInfoType::new("TestInfo".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());
        
        let mut response = ClearDisplayMessageResponse::new(ClearMessageStatusEnumType::Accepted);
        let result = response
            .set_status(ClearMessageStatusEnumType::Unknown)
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        // Verify chaining returns self
        assert_eq!(result.get_status(), &ClearMessageStatusEnumType::Unknown);
        assert_eq!(result.get_status_info(), Some(&status_info));
        assert_eq!(result.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_clear_display_message_request_edge_cases() {
        // Test with very large ID
        let large_id = i32::MAX;
        let request = ClearDisplayMessageRequest::new(large_id);
        assert_eq!(request.get_id(), &large_id);
        assert!(request.validate().is_ok());

        // Test with maximum id boundary
        let request = ClearDisplayMessageRequest::new(999999);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_clear_display_message_response_with_all_custom_data() {
        let status_info = StatusInfoType::new("DetailedInfo".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());
        
        let response = ClearDisplayMessageResponse::new(ClearMessageStatusEnumType::Unknown)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &ClearMessageStatusEnumType::Unknown);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_clear_display_message_partial_json_deserialization() {
        // Test request with only required fields
        let json = r#"{"id":42}"#;
        let deserialized: ClearDisplayMessageRequest = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(deserialized.get_id(), &42);
        assert_eq!(deserialized.get_custom_data(), None);

        // Test response with only required fields
        let json = r#"{"status":"Accepted"}"#;
        let deserialized: ClearDisplayMessageResponse = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(deserialized.get_status(), &ClearMessageStatusEnumType::Accepted);
        assert_eq!(deserialized.get_status_info(), None);
        assert_eq!(deserialized.get_custom_data(), None);
    }
}