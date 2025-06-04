use crate::v2_1::datatypes::CustomDataType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the NotifyWebPaymentStarted request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyWebPaymentStartedRequest {
    /// EVSE id for which transaction is requested.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    /// Timeout value in seconds after which no result of web payment process (e.g. QR code scanning) is to be expected anymore.
    pub timeout: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyWebPaymentStartedRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `evse_id` - EVSE id for which transaction is requested.
    /// * `timeout` - Timeout value in seconds after which no result of web payment process (e.g. QR code scanning) is to be expected anymore.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(evse_id: i32, timeout: i32) -> Self {
        Self {
            evse_id,
            timeout,
            custom_data: None,
        }
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - EVSE id for which transaction is requested.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: i32) -> &mut Self {
        self.evse_id = evse_id;
        self
    }

    /// Sets the timeout field.
    ///
    /// * `timeout` - Timeout value in seconds after which no result of web payment process (e.g. QR code scanning) is to be expected anymore.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_timeout(&mut self, timeout: i32) -> &mut Self {
        self.timeout = timeout;
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

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// EVSE id for which transaction is requested.
    pub fn get_evse_id(&self) -> &i32 {
        &self.evse_id
    }

    /// Gets a reference to the timeout field.
    ///
    /// # Returns
    ///
    /// Timeout value in seconds after which no result of web payment process (e.g. QR code scanning) is to be expected anymore.
    pub fn get_timeout(&self) -> &i32 {
        &self.timeout
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

/// Response body for the NotifyWebPaymentStarted response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyWebPaymentStartedResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyWebPaymentStartedResponse {
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

    fn create_test_notify_web_payment_started_request() -> NotifyWebPaymentStartedRequest {
        NotifyWebPaymentStartedRequest::new(1, 300)
    }

    fn create_test_notify_web_payment_started_response() -> NotifyWebPaymentStartedResponse {
        NotifyWebPaymentStartedResponse::new()
    }

    #[test]
    fn test_notify_web_payment_started_request_new() {
        let evse_id = 5;
        let timeout = 600;

        let request = NotifyWebPaymentStartedRequest::new(evse_id, timeout);

        assert_eq!(request.evse_id, evse_id);
        assert_eq!(request.timeout, timeout);
        assert!(request.custom_data.is_none());
    }

    #[test]
    fn test_notify_web_payment_started_request_serialization() {
        let request = create_test_notify_web_payment_started_request();

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: NotifyWebPaymentStartedRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_notify_web_payment_started_request_validation_valid() {
        let request = create_test_notify_web_payment_started_request();
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_web_payment_started_request_validation_negative_evse_id() {
        let request = NotifyWebPaymentStartedRequest::new(-1, 300);

        let validation_result = request.validate();
        assert!(validation_result.is_err());
        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("evse_id"));
    }

    #[test]
    fn test_notify_web_payment_started_request_set_methods() {
        let mut request = create_test_notify_web_payment_started_request();
        let new_evse_id = 10;
        let new_timeout = 900;

        request.set_evse_id(new_evse_id)
               .set_timeout(new_timeout);

        assert_eq!(request.evse_id, new_evse_id);
        assert_eq!(request.timeout, new_timeout);
    }

    #[test]
    fn test_notify_web_payment_started_request_get_methods() {
        let request = create_test_notify_web_payment_started_request();

        assert_eq!(request.get_evse_id(), &request.evse_id);
        assert_eq!(request.get_timeout(), &request.timeout);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_notify_web_payment_started_request_with_custom_data() {
        let custom_data = create_test_custom_data();
        let request = create_test_notify_web_payment_started_request()
            .with_custom_data(custom_data.clone());

        assert_eq!(request.custom_data, Some(custom_data));
        assert_eq!(request.get_custom_data(), request.custom_data.as_ref());
    }

    #[test]
    fn test_notify_web_payment_started_request_set_custom_data() {
        let mut request = create_test_notify_web_payment_started_request();
        let custom_data = create_test_custom_data();

        request.set_custom_data(Some(custom_data.clone()));
        assert_eq!(request.custom_data, Some(custom_data));

        request.set_custom_data(None);
        assert!(request.custom_data.is_none());
    }

    #[test]
    fn test_notify_web_payment_started_request_boundary_values() {
        // Test minimum valid EVSE ID
        let request_min = NotifyWebPaymentStartedRequest::new(0, 1);
        assert!(request_min.validate().is_ok());
        assert_eq!(request_min.evse_id, 0);

        // Test large valid values
        let request_max = NotifyWebPaymentStartedRequest::new(i32::MAX, i32::MAX);
        assert!(request_max.validate().is_ok());
        assert_eq!(request_max.evse_id, i32::MAX);
        assert_eq!(request_max.timeout, i32::MAX);
    }

    #[test]
    fn test_notify_web_payment_started_request_timeout_scenarios() {
        // Test various timeout values
        let timeout_values = vec![1, 30, 60, 300, 600, 1800, 3600];

        for timeout in timeout_values {
            let request = NotifyWebPaymentStartedRequest::new(1, timeout);
            assert_eq!(request.timeout, timeout);
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_notify_web_payment_started_request_json_format() {
        let request = create_test_notify_web_payment_started_request();
        let json = serde_json::to_value(&request).expect("Failed to serialize to JSON");

        assert!(json.get("evseId").is_some());
        assert!(json.get("timeout").is_some());

        // Custom data should not be present if None
        if request.custom_data.is_none() {
            assert!(json.get("customData").is_none());
        }
    }

    #[test]
    fn test_notify_web_payment_started_response_new() {
        let response = NotifyWebPaymentStartedResponse::new();
        assert!(response.custom_data.is_none());
    }

    #[test]
    fn test_notify_web_payment_started_response_serialization() {
        let response = create_test_notify_web_payment_started_response();

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: NotifyWebPaymentStartedResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_notify_web_payment_started_response_validation_valid() {
        let response = create_test_notify_web_payment_started_response();
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_notify_web_payment_started_response_set_methods() {
        let mut response = create_test_notify_web_payment_started_response();
        let custom_data = create_test_custom_data();

        response.set_custom_data(Some(custom_data.clone()));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_notify_web_payment_started_response_get_methods() {
        let response = create_test_notify_web_payment_started_response();
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_notify_web_payment_started_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = create_test_notify_web_payment_started_response()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.custom_data, Some(custom_data));
        assert_eq!(response.get_custom_data(), response.custom_data.as_ref());
    }

    #[test]
    fn test_notify_web_payment_started_response_json_format() {
        let response = create_test_notify_web_payment_started_response();
        let json = serde_json::to_value(&response).expect("Failed to serialize to JSON");

        // Custom data should not be present if None
        if response.custom_data.is_none() {
            assert!(json.get("customData").is_none());
        }
    }

    #[test]
    fn test_notify_web_payment_started_round_trip_with_custom_data() {
        let custom_data = create_test_custom_data();
        let request = create_test_notify_web_payment_started_request()
            .with_custom_data(custom_data.clone());
        let response = create_test_notify_web_payment_started_response()
            .with_custom_data(custom_data);

        // Test request round trip
        let request_json = serde_json::to_string(&request).expect("Failed to serialize request");
        let request_deserialized: NotifyWebPaymentStartedRequest = serde_json::from_str(&request_json).expect("Failed to deserialize request");
        assert_eq!(request, request_deserialized);

        // Test response round trip
        let response_json = serde_json::to_string(&response).expect("Failed to serialize response");
        let response_deserialized: NotifyWebPaymentStartedResponse = serde_json::from_str(&response_json).expect("Failed to deserialize response");
        assert_eq!(response, response_deserialized);
    }

    #[test]
    fn test_notify_web_payment_started_evse_id_scenarios() {
        // Test various EVSE ID scenarios
        let evse_ids = vec![0, 1, 2, 10, 100, 999];

        for evse_id in evse_ids {
            let request = NotifyWebPaymentStartedRequest::new(evse_id, 300);
            assert_eq!(request.evse_id, evse_id);
            assert!(request.validate().is_ok());

            // Test serialization preserves EVSE ID
            let json = serde_json::to_string(&request).expect("Failed to serialize");
            let deserialized: NotifyWebPaymentStartedRequest = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(request.evse_id, deserialized.evse_id);
        }
    }
}