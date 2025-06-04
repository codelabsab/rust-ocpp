use crate::v2_1::datatypes::{CustomDataType, EVSEType, StatusInfoType};
use crate::v2_1::enumerations::{MessageTriggerEnumType, TriggerMessageStatusEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the TriggerMessage request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub evse: Option<EVSEType>,

    pub requested_message: MessageTriggerEnumType,

    /// *(2.1)* When _requestedMessage_ = `CustomTrigger` this will trigger sending the corresponding message in field _customTrigger_, if supported by Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub custom_trigger: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl TriggerMessageRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `requested_message` - The requested_message field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(requested_message: MessageTriggerEnumType) -> Self {
        Self {
            evse: None,
            requested_message,
            custom_trigger: None,
            custom_data: None,
        }
    }

    /// Sets the evse field.
    ///
    /// * `evse` - The evse field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse(&mut self, evse: Option<EVSEType>) -> &mut Self {
        self.evse = evse;
        self
    }

    /// Sets the requested_message field.
    ///
    /// * `requested_message` - The requested_message field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_requested_message(&mut self, requested_message: MessageTriggerEnumType) -> &mut Self {
        self.requested_message = requested_message;
        self
    }

    /// Sets the custom_trigger field.
    ///
    /// * `custom_trigger` - *(2.1)* When _requestedMessage_ = `CustomTrigger` this will trigger sending the corresponding message in field _customTrigger_, if supported by Charging Station.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_custom_trigger(&mut self, custom_trigger: Option<String>) -> &mut Self {
        self.custom_trigger = custom_trigger;
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

    /// Gets a reference to the evse field.
    ///
    /// # Returns
    ///
    /// The evse field
    pub fn get_evse(&self) -> Option<&EVSEType> {
        self.evse.as_ref()
    }

    /// Gets a reference to the requested_message field.
    ///
    /// # Returns
    ///
    /// The requested_message field
    pub fn get_requested_message(&self) -> &MessageTriggerEnumType {
        &self.requested_message
    }

    /// Gets a reference to the custom_trigger field.
    ///
    /// # Returns
    ///
    /// *(2.1)* When _requestedMessage_ = `CustomTrigger` this will trigger sending the corresponding message in field _customTrigger_, if supported by Charging Station.
    pub fn get_custom_trigger(&self) -> Option<&String> {
        self.custom_trigger.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the evse field and returns self for builder pattern.
    ///
    /// * `evse` - The evse field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_evse(mut self, evse: EVSEType) -> Self {
        self.evse = Some(evse);
        self
    }

    /// Sets the custom_trigger field and returns self for builder pattern.
    ///
    /// * `custom_trigger` - *(2.1)* When _requestedMessage_ = `CustomTrigger` this will trigger sending the corresponding message in field _customTrigger_, if supported by Charging Station.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_custom_trigger(mut self, custom_trigger: String) -> Self {
        self.custom_trigger = Some(custom_trigger);
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

/// Response body for the TriggerMessage response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessageResponse {
    pub status: TriggerMessageStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl TriggerMessageResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: TriggerMessageStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: TriggerMessageStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &TriggerMessageStatusEnumType {
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
    use crate::v2_1::datatypes::{CustomDataType, EVSEType, StatusInfoType};
    use crate::v2_1::enumerations::{MessageTriggerEnumType, TriggerMessageStatusEnumType};
    use serde_json;
    use validator::Validate;

    fn create_test_evse() -> EVSEType {
        EVSEType::new(1)
    }

    // Tests for TriggerMessageRequest

    #[test]
    fn test_trigger_message_request_new() {
        let requested_message = MessageTriggerEnumType::Heartbeat;
        let request = TriggerMessageRequest::new(requested_message.clone());

        assert_eq!(request.get_requested_message(), &requested_message);
        assert_eq!(request.get_evse(), None);
        assert_eq!(request.get_custom_trigger(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_trigger_message_request_serialization() {
        let request = TriggerMessageRequest::new(MessageTriggerEnumType::StatusNotification);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: TriggerMessageRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_trigger_message_request_validation() {
        let request = TriggerMessageRequest::new(MessageTriggerEnumType::MeterValues);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_trigger_message_request_with_evse() {
        let evse = create_test_evse();
        let request = TriggerMessageRequest::new(MessageTriggerEnumType::StatusNotification)
            .with_evse(evse.clone());

        assert_eq!(request.get_evse(), Some(&evse));
    }

    #[test]
    fn test_trigger_message_request_with_custom_trigger() {
        let custom_trigger = "VendorSpecificMessage".to_string();
        let request = TriggerMessageRequest::new(MessageTriggerEnumType::CustomTrigger)
            .with_custom_trigger(custom_trigger.clone());

        assert_eq!(request.get_custom_trigger(), Some(&custom_trigger));
    }

    #[test]
    fn test_trigger_message_request_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = TriggerMessageRequest::new(MessageTriggerEnumType::BootNotification)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_trigger_message_request_set_methods() {
        let evse = create_test_evse();
        let custom_trigger = "TestMessage".to_string();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = TriggerMessageRequest::new(MessageTriggerEnumType::Heartbeat);

        request
            .set_requested_message(MessageTriggerEnumType::TransactionEvent)
            .set_evse(Some(evse.clone()))
            .set_custom_trigger(Some(custom_trigger.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_requested_message(), &MessageTriggerEnumType::TransactionEvent);
        assert_eq!(request.get_evse(), Some(&evse));
        assert_eq!(request.get_custom_trigger(), Some(&custom_trigger));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_trigger_message_request_builder_pattern() {
        let evse = create_test_evse();
        let custom_trigger = "CustomMessage".to_string();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = TriggerMessageRequest::new(MessageTriggerEnumType::CustomTrigger)
            .with_evse(evse.clone())
            .with_custom_trigger(custom_trigger.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_evse(), Some(&evse));
        assert_eq!(request.get_custom_trigger(), Some(&custom_trigger));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_trigger_message_request_custom_trigger_validation() {
        let long_trigger = "x".repeat(50); // Max allowed length
        let request = TriggerMessageRequest::new(MessageTriggerEnumType::CustomTrigger)
            .with_custom_trigger(long_trigger);

        assert!(request.validate().is_ok());

        let too_long_trigger = "x".repeat(51); // Exceeds max length
        let invalid_request = TriggerMessageRequest::new(MessageTriggerEnumType::CustomTrigger)
            .with_custom_trigger(too_long_trigger);

        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_trigger_message_request_all_message_types() {
        let message_types = vec![
            MessageTriggerEnumType::BootNotification,
            MessageTriggerEnumType::LogStatusNotification,
            MessageTriggerEnumType::FirmwareStatusNotification,
            MessageTriggerEnumType::Heartbeat,
            MessageTriggerEnumType::MeterValues,
            MessageTriggerEnumType::SignChargingStationCertificate,
            MessageTriggerEnumType::SignV2GCertificate,
            MessageTriggerEnumType::SignV2G20Certificate,
            MessageTriggerEnumType::StatusNotification,
            MessageTriggerEnumType::TransactionEvent,
            MessageTriggerEnumType::SignCombinedCertificate,
            MessageTriggerEnumType::PublishFirmwareStatusNotification,
            MessageTriggerEnumType::CustomTrigger,
        ];

        for message_type in message_types {
            let request = TriggerMessageRequest::new(message_type.clone());
            
            assert_eq!(request.get_requested_message(), &message_type);
            assert!(request.validate().is_ok());

            let json = serde_json::to_string(&request).expect("Failed to serialize");
            let deserialized: TriggerMessageRequest = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(request, deserialized);
        }
    }

    // Tests for TriggerMessageResponse

    #[test]
    fn test_trigger_message_response_new() {
        let status = TriggerMessageStatusEnumType::Accepted;
        let response = TriggerMessageResponse::new(status.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_trigger_message_response_serialization() {
        let response = TriggerMessageResponse::new(TriggerMessageStatusEnumType::Rejected);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: TriggerMessageResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_trigger_message_response_validation() {
        let response = TriggerMessageResponse::new(TriggerMessageStatusEnumType::NotImplemented);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_trigger_message_response_with_status_info() {
        let status_info = StatusInfoType::new("Success".to_string());
        let response = TriggerMessageResponse::new(TriggerMessageStatusEnumType::Accepted)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
    }

    #[test]
    fn test_trigger_message_response_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = TriggerMessageResponse::new(TriggerMessageStatusEnumType::Rejected)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_trigger_message_response_set_methods() {
        let status_info = StatusInfoType::new("Error".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = TriggerMessageResponse::new(TriggerMessageStatusEnumType::Accepted);

        response
            .set_status(TriggerMessageStatusEnumType::NotImplemented)
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_status(), &TriggerMessageStatusEnumType::NotImplemented);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_trigger_message_response_builder_pattern() {
        let status_info = StatusInfoType::new("Info".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = TriggerMessageResponse::new(TriggerMessageStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_trigger_message_response_all_status_types() {
        let status_types = vec![
            TriggerMessageStatusEnumType::Accepted,
            TriggerMessageStatusEnumType::Rejected,
            TriggerMessageStatusEnumType::NotImplemented,
        ];

        for status in status_types {
            let response = TriggerMessageResponse::new(status.clone());
            
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());

            let json = serde_json::to_string(&response).expect("Failed to serialize");
            let deserialized: TriggerMessageResponse = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(response, deserialized);
        }
    }

    #[test]
    fn test_trigger_message_request_json_round_trip() {
        let evse = create_test_evse();
        let custom_trigger = "VendorMessage".to_string();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = TriggerMessageRequest::new(MessageTriggerEnumType::CustomTrigger)
            .with_evse(evse)
            .with_custom_trigger(custom_trigger)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: TriggerMessageRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_trigger_message_response_json_round_trip() {
        let status_info = StatusInfoType::new("Details".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = TriggerMessageResponse::new(TriggerMessageStatusEnumType::Rejected)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: TriggerMessageResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_trigger_message_request_with_meter_values_and_evse() {
        let evse = create_test_evse();
        let request = TriggerMessageRequest::new(MessageTriggerEnumType::MeterValues)
            .with_evse(evse.clone());

        assert_eq!(request.get_requested_message(), &MessageTriggerEnumType::MeterValues);
        assert_eq!(request.get_evse(), Some(&evse));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_trigger_message_response_with_all_optional_fields() {
        let status_info = StatusInfoType::new("Complete".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = TriggerMessageResponse::new(TriggerMessageStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &TriggerMessageStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
        assert!(response.validate().is_ok());
    }
}