use crate::v2_1::datatypes::CustomDataType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the NotifyPriorityCharging request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyPriorityChargingRequest {
    /// The transaction for which priority charging is requested.
    #[validate(length(max = 36))]
    pub transaction_id: String,

    /// True if priority charging was activated. False if it has stopped using the priority charging profile.
    pub activated: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyPriorityChargingRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `transaction_id` - The transaction for which priority charging is requested.
    /// * `activated` - True if priority charging was activated. False if it has stopped using the priority charging profile.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(transaction_id: String, activated: bool) -> Self {
        Self {
            transaction_id,
            activated,
            custom_data: None,
        }
    }

    /// Sets the transaction_id field.
    ///
    /// * `transaction_id` - The transaction for which priority charging is requested.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_transaction_id(&mut self, transaction_id: String) -> &mut Self {
        self.transaction_id = transaction_id;
        self
    }

    /// Sets the activated field.
    ///
    /// * `activated` - True if priority charging was activated. False if it has stopped using the priority charging profile.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_activated(&mut self, activated: bool) -> &mut Self {
        self.activated = activated;
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
    /// The transaction for which priority charging is requested.
    pub fn get_transaction_id(&self) -> &String {
        &self.transaction_id
    }

    /// Gets a reference to the activated field.
    ///
    /// # Returns
    ///
    /// True if priority charging was activated. False if it has stopped using the priority charging profile.
    pub fn get_activated(&self) -> &bool {
        &self.activated
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

/// Response body for the NotifyPriorityCharging response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyPriorityChargingResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyPriorityChargingResponse {
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
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_notify_priority_charging_request() -> NotifyPriorityChargingRequest {
        NotifyPriorityChargingRequest::new("test_transaction_123".to_string(), true)
    }

    fn create_test_notify_priority_charging_response() -> NotifyPriorityChargingResponse {
        NotifyPriorityChargingResponse::new()
    }

    #[test]
    fn test_notify_priority_charging_request_new() {
        let transaction_id = "test_transaction_456".to_string();
        let activated = false;

        let request = NotifyPriorityChargingRequest::new(transaction_id.clone(), activated);

        assert_eq!(request.transaction_id, transaction_id);
        assert_eq!(request.activated, activated);
        assert!(request.custom_data.is_none());
    }

    #[test]
    fn test_notify_priority_charging_request_serialization() {
        let request = create_test_notify_priority_charging_request();

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: NotifyPriorityChargingRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_notify_priority_charging_request_validation_valid() {
        let request = create_test_notify_priority_charging_request();
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_priority_charging_request_validation_transaction_id_too_long() {
        let long_transaction_id = "a".repeat(37); // Max is 36
        let request = NotifyPriorityChargingRequest::new(long_transaction_id, true);

        let validation_result = request.validate();
        assert!(validation_result.is_err());
        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("transaction_id"));
    }

    #[test]
    fn test_notify_priority_charging_request_set_methods() {
        let mut request = create_test_notify_priority_charging_request();
        let new_transaction_id = "new_transaction_789".to_string();
        let new_activated = false;

        request.set_transaction_id(new_transaction_id.clone())
               .set_activated(new_activated);

        assert_eq!(request.transaction_id, new_transaction_id);
        assert_eq!(request.activated, new_activated);
    }

    #[test]
    fn test_notify_priority_charging_request_get_methods() {
        let request = create_test_notify_priority_charging_request();

        assert_eq!(request.get_transaction_id(), &request.transaction_id);
        assert_eq!(request.get_activated(), &request.activated);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_notify_priority_charging_request_with_custom_data() {
        let custom_data = create_test_custom_data();
        let request = create_test_notify_priority_charging_request()
            .with_custom_data(custom_data.clone());

        assert_eq!(request.custom_data, Some(custom_data));
        assert_eq!(request.get_custom_data(), request.custom_data.as_ref());
    }

    #[test]
    fn test_notify_priority_charging_request_set_custom_data() {
        let mut request = create_test_notify_priority_charging_request();
        let custom_data = create_test_custom_data();

        request.set_custom_data(Some(custom_data.clone()));
        assert_eq!(request.custom_data, Some(custom_data));

        request.set_custom_data(None);
        assert!(request.custom_data.is_none());
    }

    #[test]
    fn test_notify_priority_charging_request_boundary_values() {
        // Test maximum length transaction ID
        let max_transaction_id = "a".repeat(36);
        let request = NotifyPriorityChargingRequest::new(max_transaction_id.clone(), true);
        assert!(request.validate().is_ok());
        assert_eq!(request.transaction_id, max_transaction_id);

        // Test minimum length transaction ID
        let min_transaction_id = "a".to_string();
        let request_min = NotifyPriorityChargingRequest::new(min_transaction_id.clone(), false);
        assert!(request_min.validate().is_ok());
        assert_eq!(request_min.transaction_id, min_transaction_id);
    }

    #[test]
    fn test_notify_priority_charging_request_json_format() {
        let request = create_test_notify_priority_charging_request();
        let json = serde_json::to_value(&request).expect("Failed to serialize to JSON");

        assert!(json.get("transactionId").is_some());
        assert!(json.get("activated").is_some());

        // Custom data should not be present if None
        if request.custom_data.is_none() {
            assert!(json.get("customData").is_none());
        }
    }

    #[test]
    fn test_notify_priority_charging_response_new() {
        let response = NotifyPriorityChargingResponse::new();
        assert!(response.custom_data.is_none());
    }

    #[test]
    fn test_notify_priority_charging_response_serialization() {
        let response = create_test_notify_priority_charging_response();

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: NotifyPriorityChargingResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_notify_priority_charging_response_validation_valid() {
        let response = create_test_notify_priority_charging_response();
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_notify_priority_charging_response_set_methods() {
        let mut response = create_test_notify_priority_charging_response();
        let custom_data = create_test_custom_data();

        response.set_custom_data(Some(custom_data.clone()));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_notify_priority_charging_response_get_methods() {
        let response = create_test_notify_priority_charging_response();
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_notify_priority_charging_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = create_test_notify_priority_charging_response()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.custom_data, Some(custom_data));
        assert_eq!(response.get_custom_data(), response.custom_data.as_ref());
    }

    #[test]
    fn test_notify_priority_charging_response_json_format() {
        let response = create_test_notify_priority_charging_response();
        let json = serde_json::to_value(&response).expect("Failed to serialize to JSON");

        // Custom data should not be present if None
        if response.custom_data.is_none() {
            assert!(json.get("customData").is_none());
        }
    }

    #[test]
    fn test_notify_priority_charging_both_activated_states() {
        // Test activated = true
        let request_activated = NotifyPriorityChargingRequest::new("test_123".to_string(), true);
        assert_eq!(request_activated.activated, true);
        assert!(request_activated.validate().is_ok());

        // Test activated = false
        let request_deactivated = NotifyPriorityChargingRequest::new("test_456".to_string(), false);
        assert_eq!(request_deactivated.activated, false);
        assert!(request_deactivated.validate().is_ok());
    }

    #[test]
    fn test_notify_priority_charging_round_trip_with_custom_data() {
        let custom_data = create_test_custom_data();
        let request = create_test_notify_priority_charging_request()
            .with_custom_data(custom_data.clone());
        let response = create_test_notify_priority_charging_response()
            .with_custom_data(custom_data);

        // Test request round trip
        let request_json = serde_json::to_string(&request).expect("Failed to serialize request");
        let request_deserialized: NotifyPriorityChargingRequest = serde_json::from_str(&request_json).expect("Failed to deserialize request");
        assert_eq!(request, request_deserialized);

        // Test response round trip
        let response_json = serde_json::to_string(&response).expect("Failed to serialize response");
        let response_deserialized: NotifyPriorityChargingResponse = serde_json::from_str(&response_json).expect("Failed to deserialize response");
        assert_eq!(response, response_deserialized);
    }
}