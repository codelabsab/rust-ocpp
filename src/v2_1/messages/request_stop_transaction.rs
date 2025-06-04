use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::RequestStartStopStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the StopTransaction request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestStopTransactionRequest {
    /// The identifier of the transaction which the Charging Station is requested to stop.
    #[validate(length(max = 36))]
    pub transaction_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl RequestStopTransactionRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `transaction_id` - The identifier of the transaction which the Charging Station is requested to stop.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(transaction_id: String) -> Self {
        Self {
            transaction_id,
            custom_data: None,
        }
    }

    /// Sets the transaction_id field.
    ///
    /// * `transaction_id` - The identifier of the transaction which the Charging Station is requested to stop.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_transaction_id(&mut self, transaction_id: String) -> &mut Self {
        self.transaction_id = transaction_id;
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

    /// Gets a reference to the transaction_id field.
    ///
    /// # Returns
    ///
    /// The identifier of the transaction which the Charging Station is requested to stop.
    pub fn get_transaction_id(&self) -> &String {
        &self.transaction_id
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

/// Response body for the StopTransaction response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestStopTransactionResponse {
    pub status: RequestStartStopStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl RequestStopTransactionResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: RequestStartStopStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: RequestStartStopStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &RequestStartStopStatusEnumType {
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
    use crate::v2_1::datatypes::StatusInfoType;
    use crate::v2_1::enumerations::RequestStartStopStatusEnumType;
    use serde_json;
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("TestReason".to_string())
    }

    #[test]
    fn test_request_stop_transaction_request_new() {
        let request = RequestStopTransactionRequest::new("tx-12345".to_string());

        assert_eq!(request.transaction_id, "tx-12345");
        assert!(request.custom_data.is_none());
    }

    #[test]
    fn test_request_stop_transaction_request_serialization() {
        let request = RequestStopTransactionRequest::new("tx-67890".to_string())
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: RequestStopTransactionRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert_eq!(deserialized.transaction_id, "tx-67890");
        assert!(deserialized.custom_data.is_some());
    }

    #[test]
    fn test_request_stop_transaction_request_validation() {
        // Valid request
        let valid_request = RequestStopTransactionRequest::new("tx-valid".to_string());
        assert!(valid_request.validate().is_ok());

        // Test transaction_id length validation (max 36 characters)
        let long_transaction_id = "a".repeat(37);
        let invalid_request = RequestStopTransactionRequest {
            transaction_id: long_transaction_id,
            custom_data: None,
        };
        assert!(invalid_request.validate().is_err());

        // Test maximum valid transaction_id length
        let max_transaction_id = "a".repeat(36);
        let max_request = RequestStopTransactionRequest::new(max_transaction_id);
        assert!(max_request.validate().is_ok());
    }

    #[test]
    fn test_request_stop_transaction_request_builder_pattern() {
        let custom_data = create_test_custom_data();
        let request = RequestStopTransactionRequest::new("tx-builder".to_string())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.transaction_id, "tx-builder");
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_request_stop_transaction_request_setters_getters() {
        let mut request = RequestStopTransactionRequest::new("tx-original".to_string());
        let custom_data = create_test_custom_data();

        // Test setters
        request.set_transaction_id("tx-modified".to_string());
        request.set_custom_data(Some(custom_data.clone()));

        // Test getters
        assert_eq!(request.get_transaction_id(), "tx-modified");
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_request_stop_transaction_response_new() {
        let response = RequestStopTransactionResponse::new(RequestStartStopStatusEnumType::Accepted);

        assert_eq!(response.status, RequestStartStopStatusEnumType::Accepted);
        assert!(response.status_info.is_none());
        assert!(response.custom_data.is_none());
    }

    #[test]
    fn test_request_stop_transaction_response_serialization() {
        let response = RequestStopTransactionResponse::new(RequestStartStopStatusEnumType::Rejected)
            .with_status_info(create_test_status_info())
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: RequestStopTransactionResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert_eq!(deserialized.status, RequestStartStopStatusEnumType::Rejected);
        assert!(deserialized.status_info.is_some());
        assert!(deserialized.custom_data.is_some());
    }

    #[test]
    fn test_request_stop_transaction_response_validation() {
        let valid_response = RequestStopTransactionResponse::new(RequestStartStopStatusEnumType::Accepted);
        assert!(valid_response.validate().is_ok());

        let response_with_all_fields = RequestStopTransactionResponse::new(RequestStartStopStatusEnumType::Rejected)
            .with_status_info(create_test_status_info())
            .with_custom_data(create_test_custom_data());
        assert!(response_with_all_fields.validate().is_ok());
    }

    #[test]
    fn test_request_stop_transaction_response_builder_pattern() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let response = RequestStopTransactionResponse::new(RequestStartStopStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, RequestStartStopStatusEnumType::Accepted);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_request_stop_transaction_response_setters_getters() {
        let mut response = RequestStopTransactionResponse::new(RequestStartStopStatusEnumType::Accepted);
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        // Test setters
        response.set_status(RequestStartStopStatusEnumType::Rejected);
        response.set_status_info(Some(status_info.clone()));
        response.set_custom_data(Some(custom_data.clone()));

        // Test getters
        assert_eq!(*response.get_status(), RequestStartStopStatusEnumType::Rejected);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_request_stop_transaction_response_enum_variants() {
        let accepted_response = RequestStopTransactionResponse::new(RequestStartStopStatusEnumType::Accepted);
        assert_eq!(accepted_response.status, RequestStartStopStatusEnumType::Accepted);

        let rejected_response = RequestStopTransactionResponse::new(RequestStartStopStatusEnumType::Rejected);
        assert_eq!(rejected_response.status, RequestStartStopStatusEnumType::Rejected);
    }

    #[test]
    fn test_request_stop_transaction_json_round_trip() {
        let original_request = RequestStopTransactionRequest::new("tx-round-trip".to_string())
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&original_request).expect("Failed to serialize request");
        let parsed_request: RequestStopTransactionRequest =
            serde_json::from_str(&json).expect("Failed to deserialize request");

        assert_eq!(original_request, parsed_request);

        let original_response = RequestStopTransactionResponse::new(RequestStartStopStatusEnumType::Accepted)
            .with_status_info(create_test_status_info())
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&original_response).expect("Failed to serialize response");
        let parsed_response: RequestStopTransactionResponse =
            serde_json::from_str(&json).expect("Failed to deserialize response");

        assert_eq!(original_response, parsed_response);
    }

    #[test]
    fn test_request_stop_transaction_edge_cases() {
        // Test empty transaction_id
        let empty_request = RequestStopTransactionRequest::new("".to_string());
        assert!(empty_request.validate().is_ok());

        // Test transaction_id with special characters
        let special_request = RequestStopTransactionRequest::new("tx-123_ABC-def".to_string());
        assert!(special_request.validate().is_ok());
    }
}