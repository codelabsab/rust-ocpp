use crate::v2_1::datatypes::{CustomDataType, IdTokenType, StatusInfoType};
use crate::v2_1::enumerations::GenericStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the BatterySwap request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestBatterySwapRequest {
    #[validate(nested)]
    pub id_token: IdTokenType,

    /// Request id to match with BatterySwapRequest.
    #[validate(range(min = 0))]
    pub request_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl RequestBatterySwapRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `id_token` - The id_token field
    /// * `request_id` - Request id to match with BatterySwapRequest.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(id_token: IdTokenType, request_id: i32) -> Self {
        Self {
            id_token,
            request_id,
            custom_data: None,
        }
    }

    /// Sets the id_token field.
    ///
    /// * `id_token` - The id_token field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_id_token(&mut self, id_token: IdTokenType) -> &mut Self {
        self.id_token = id_token;
        self
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - Request id to match with BatterySwapRequest.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
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

    /// Gets a reference to the id_token field.
    ///
    /// # Returns
    ///
    /// The id_token field
    pub fn get_id_token(&self) -> &IdTokenType {
        &self.id_token
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// Request id to match with BatterySwapRequest.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
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

/// Response body for the BatterySwap response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestBatterySwapResponse {
    pub status: GenericStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl RequestBatterySwapResponse {
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
    use crate::v2_1::datatypes::{IdTokenType, StatusInfoType};
    use crate::v2_1::enumerations::GenericStatusEnumType;
    use serde_json;
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_id_token() -> IdTokenType {
        IdTokenType::new("test_token".to_string(), "Central".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("TestReason".to_string())
    }

    #[test]
    fn test_request_battery_swap_request_new() {
        let id_token = create_test_id_token();
        let request = RequestBatterySwapRequest::new(id_token.clone(), 123);

        assert_eq!(request.id_token, id_token);
        assert_eq!(request.request_id, 123);
        assert!(request.custom_data.is_none());
    }

    #[test]
    fn test_request_battery_swap_request_serialization() {
        let id_token = create_test_id_token();
        let request = RequestBatterySwapRequest::new(id_token.clone(), 456)
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: RequestBatterySwapRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert_eq!(deserialized.id_token, id_token);
        assert_eq!(deserialized.request_id, 456);
        assert!(deserialized.custom_data.is_some());
    }

    #[test]
    fn test_request_battery_swap_request_validation() {
        let id_token = create_test_id_token();

        // Valid request
        let valid_request = RequestBatterySwapRequest::new(id_token.clone(), 0);
        assert!(valid_request.validate().is_ok());

        // Invalid request_id (negative)
        let invalid_request = RequestBatterySwapRequest {
            id_token: id_token.clone(),
            request_id: -1,
            custom_data: None,
        };
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_request_battery_swap_request_builder_pattern() {
        let id_token = create_test_id_token();
        let custom_data = create_test_custom_data();

        let request = RequestBatterySwapRequest::new(id_token.clone(), 789)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.id_token, id_token);
        assert_eq!(request.request_id, 789);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_request_battery_swap_request_setters_getters() {
        let id_token = create_test_id_token();
        let mut request = RequestBatterySwapRequest::new(id_token.clone(), 100);
        let new_id_token = IdTokenType::new("new_token".to_string(), "ISO14443".to_string());
        let custom_data = create_test_custom_data();

        // Test setters
        request.set_id_token(new_id_token.clone());
        request.set_request_id(200);
        request.set_custom_data(Some(custom_data.clone()));

        // Test getters
        assert_eq!(request.get_id_token(), &new_id_token);
        assert_eq!(*request.get_request_id(), 200);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_request_battery_swap_response_new() {
        let response = RequestBatterySwapResponse::new(GenericStatusEnumType::Accepted);

        assert_eq!(response.status, GenericStatusEnumType::Accepted);
        assert!(response.status_info.is_none());
        assert!(response.custom_data.is_none());
    }

    #[test]
    fn test_request_battery_swap_response_serialization() {
        let response = RequestBatterySwapResponse::new(GenericStatusEnumType::Rejected)
            .with_status_info(create_test_status_info())
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: RequestBatterySwapResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert_eq!(deserialized.status, GenericStatusEnumType::Rejected);
        assert!(deserialized.status_info.is_some());
        assert!(deserialized.custom_data.is_some());
    }

    #[test]
    fn test_request_battery_swap_response_validation() {
        let valid_response = RequestBatterySwapResponse::new(GenericStatusEnumType::Accepted);
        assert!(valid_response.validate().is_ok());

        let response_with_all_fields = RequestBatterySwapResponse::new(GenericStatusEnumType::Rejected)
            .with_status_info(create_test_status_info())
            .with_custom_data(create_test_custom_data());
        assert!(response_with_all_fields.validate().is_ok());
    }

    #[test]
    fn test_request_battery_swap_response_builder_pattern() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let response = RequestBatterySwapResponse::new(GenericStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, GenericStatusEnumType::Accepted);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_request_battery_swap_response_setters_getters() {
        let mut response = RequestBatterySwapResponse::new(GenericStatusEnumType::Accepted);
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        // Test setters
        response.set_status(GenericStatusEnumType::Rejected);
        response.set_status_info(Some(status_info.clone()));
        response.set_custom_data(Some(custom_data.clone()));

        // Test getters
        assert_eq!(*response.get_status(), GenericStatusEnumType::Rejected);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_request_battery_swap_response_enum_variants() {
        let accepted_response = RequestBatterySwapResponse::new(GenericStatusEnumType::Accepted);
        assert_eq!(accepted_response.status, GenericStatusEnumType::Accepted);

        let rejected_response = RequestBatterySwapResponse::new(GenericStatusEnumType::Rejected);
        assert_eq!(rejected_response.status, GenericStatusEnumType::Rejected);
    }

    #[test]
    fn test_request_battery_swap_request_edge_cases() {
        let id_token = create_test_id_token();

        // Test minimum valid request_id
        let min_request = RequestBatterySwapRequest::new(id_token.clone(), 0);
        assert!(min_request.validate().is_ok());

        // Test large request_id
        let large_request = RequestBatterySwapRequest::new(id_token, i32::MAX);
        assert!(large_request.validate().is_ok());
    }

    #[test]
    fn test_request_battery_swap_json_round_trip() {
        let id_token = create_test_id_token();
        let original_request = RequestBatterySwapRequest::new(id_token, 12345)
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&original_request).expect("Failed to serialize request");
        let parsed_request: RequestBatterySwapRequest =
            serde_json::from_str(&json).expect("Failed to deserialize request");

        assert_eq!(original_request, parsed_request);

        let original_response = RequestBatterySwapResponse::new(GenericStatusEnumType::Accepted)
            .with_status_info(create_test_status_info())
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&original_response).expect("Failed to serialize response");
        let parsed_response: RequestBatterySwapResponse =
            serde_json::from_str(&json).expect("Failed to deserialize response");

        assert_eq!(original_response, parsed_response);
    }

    #[test]
    fn test_request_battery_swap_id_token_types() {
        // Test different IdToken types
        let token_types = vec![
            "Central",
            "eMAID",
            "ISO14443",
            "ISO15693",
            "KeyCode",
            "Local",
            "MacAddress",
            "NoAuthorization",
        ];

        for token_type in token_types {
            let id_token = IdTokenType::new("test_token".to_string(), token_type.to_string());
            let request = RequestBatterySwapRequest::new(id_token.clone(), 1);
            assert!(request.validate().is_ok());
            assert_eq!(request.id_token, id_token);
        }
    }
}