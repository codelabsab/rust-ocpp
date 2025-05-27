use crate::v2_1::datatypes::CustomDataType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetLocalListVersion request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetLocalListVersionRequest {
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

/// Response body for the GetLocalListVersion response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionResponse {
    /// This contains the current version number of the local authorization list in the Charging Station.
    pub version_number: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetLocalListVersionResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `version_number` - This contains the current version number of the local authorization list in the Charging Station.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(version_number: i32) -> Self {
        Self {
            version_number,
            custom_data: None,
        }
    }

    /// Sets the version_number field.
    ///
    /// * `version_number` - This contains the current version number of the local authorization list in the Charging Station.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_version_number(&mut self, version_number: i32) -> &mut Self {
        self.version_number = version_number;
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

    /// Gets a reference to the version_number field.
    ///
    /// # Returns
    ///
    /// This contains the current version number of the local authorization list in the Charging Station.
    pub fn get_version_number(&self) -> &i32 {
        &self.version_number
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

    // Tests for GetLocalListVersionRequest

    #[test]
    fn test_get_local_list_version_request_new() {
        let request = GetLocalListVersionRequest::new();

        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_local_list_version_request_with_custom_data() {
        let custom_data = create_test_custom_data();
        let request = GetLocalListVersionRequest::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_local_list_version_request_setters() {
        let custom_data = create_test_custom_data();

        let mut request = GetLocalListVersionRequest::new();
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_local_list_version_request_getters() {
        let custom_data = create_test_custom_data();
        let request = GetLocalListVersionRequest::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_local_list_version_request_serialization() {
        let request = GetLocalListVersionRequest::new();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetLocalListVersionRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
    }

    #[test]
    fn test_get_local_list_version_request_validation() {
        let request = GetLocalListVersionRequest::new();

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_get_local_list_version_request_json_round_trip() {
        let custom_data = create_test_custom_data();
        let request = GetLocalListVersionRequest::new()
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetLocalListVersionRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
        assert!(parsed.validate().is_ok());
    }

    // Tests for GetLocalListVersionResponse

    #[test]
    fn test_get_local_list_version_response_new() {
        let response = GetLocalListVersionResponse::new(42);

        assert_eq!(response.version_number, 42);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_local_list_version_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = GetLocalListVersionResponse::new(123)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.version_number, 123);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_local_list_version_response_setters() {
        let custom_data = create_test_custom_data();

        let mut response = GetLocalListVersionResponse::new(100);
        response.set_version_number(200);
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.version_number, 200);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_local_list_version_response_getters() {
        let custom_data = create_test_custom_data();
        let response = GetLocalListVersionResponse::new(555)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_version_number(), &555);
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_local_list_version_response_serialization() {
        let response = GetLocalListVersionResponse::new(999);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetLocalListVersionResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
    }

    #[test]
    fn test_get_local_list_version_response_validation() {
        let response = GetLocalListVersionResponse::new(0);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_get_local_list_version_response_negative_version() {
        let response = GetLocalListVersionResponse::new(-1);

        assert!(response.validate().is_ok()); // No validation constraints on version_number
    }

    #[test]
    fn test_get_local_list_version_response_json_round_trip() {
        let custom_data = create_test_custom_data();
        let response = GetLocalListVersionResponse::new(777)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetLocalListVersionResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
        assert!(parsed.validate().is_ok());
    }
}
