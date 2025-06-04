use crate::v2_1::datatypes::{CustomDataType, MessageInfoType, StatusInfoType};
use crate::v2_1::enumerations::DisplayMessageStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the SetDisplayMessage request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetDisplayMessageRequest {
    #[validate(nested)]
    pub message: MessageInfoType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetDisplayMessageRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `message` - The message field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(message: MessageInfoType) -> Self {
        Self {
            message,
            custom_data: None,
        }
    }

    /// Sets the message field.
    ///
    /// * `message` - The message field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_message(&mut self, message: MessageInfoType) -> &mut Self {
        self.message = message;
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

    /// Gets a reference to the message field.
    ///
    /// # Returns
    ///
    /// The message field
    pub fn get_message(&self) -> &MessageInfoType {
        &self.message
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

/// Response body for the SetDisplayMessage response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetDisplayMessageResponse {
    pub status: DisplayMessageStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetDisplayMessageResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: DisplayMessageStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: DisplayMessageStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &DisplayMessageStatusEnumType {
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
    use crate::v2_1::datatypes::{CustomDataType, MessageInfoType, StatusInfoType};
    use crate::v2_1::enumerations::{DisplayMessageStatusEnumType, MessagePriorityEnumType, MessageStateEnumType};
    use chrono::Utc;

    fn create_test_message() -> MessageInfoType {
        MessageInfoType::new(1, MessagePriorityEnumType::NormalCycle, MessageStateEnumType::Charging, Utc::now())
    }

    #[test]
    fn test_set_display_message_request_new() {
        let message = create_test_message();
        let request = SetDisplayMessageRequest::new(message.clone());

        assert_eq!(request.message, message);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_set_display_message_request_serialization() {
        let message = create_test_message();
        let request = SetDisplayMessageRequest::new(message);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: SetDisplayMessageRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(json.contains("\"message\""));
    }

    #[test]
    fn test_set_display_message_request_validation() {
        let message = create_test_message();
        let request = SetDisplayMessageRequest::new(message);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_set_display_message_request_builder_pattern() {
        let message = create_test_message();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = SetDisplayMessageRequest::new(message.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.message, message);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_display_message_request_setters() {
        let message1 = create_test_message();
        let message2 = MessageInfoType::new(2, MessagePriorityEnumType::AlwaysFront, MessageStateEnumType::Idle, Utc::now());
        let mut request = SetDisplayMessageRequest::new(message1);
        let custom_data = CustomDataType::new("TestVendor".to_string());

        request.set_message(message2.clone())
               .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.message, message2);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_display_message_request_getters() {
        let message = create_test_message();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = SetDisplayMessageRequest::new(message.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(*request.get_message(), message);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_display_message_response_new() {
        let response = SetDisplayMessageResponse::new(DisplayMessageStatusEnumType::Accepted);
        assert_eq!(response.status, DisplayMessageStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_set_display_message_response_serialization() {
        let response = SetDisplayMessageResponse::new(DisplayMessageStatusEnumType::Rejected);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: SetDisplayMessageResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(json.contains("\"status\":\"Rejected\""));
    }

    #[test]
    fn test_set_display_message_response_builder_pattern() {
        let status_info = StatusInfoType::new("Message conflict".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = SetDisplayMessageResponse::new(DisplayMessageStatusEnumType::NotSupportedMessageFormat)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, DisplayMessageStatusEnumType::NotSupportedMessageFormat);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_display_message_response_setters() {
        let mut response = SetDisplayMessageResponse::new(DisplayMessageStatusEnumType::Accepted);
        let status_info = StatusInfoType::new("Updated status".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        response.set_status(DisplayMessageStatusEnumType::Rejected)
                .set_status_info(Some(status_info.clone()))
                .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, DisplayMessageStatusEnumType::Rejected);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_display_message_response_getters() {
        let status_info = StatusInfoType::new("Test status".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = SetDisplayMessageResponse::new(DisplayMessageStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(*response.get_status(), DisplayMessageStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_display_message_edge_cases() {
        // Test with different message priorities and states
        let high_priority_message = MessageInfoType::new(
            999,
            MessagePriorityEnumType::AlwaysFront,
            MessageStateEnumType::Faulted,
            Utc::now()
        );

        let request = SetDisplayMessageRequest::new(high_priority_message);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_set_display_message_response_validation() {
        let response = SetDisplayMessageResponse::new(DisplayMessageStatusEnumType::Accepted);
        assert!(response.validate().is_ok());
    }
}