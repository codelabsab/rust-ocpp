use crate::v2_1::datatypes::{CustomDataType, MessageInfoType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the NotifyDisplayMessages request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDisplayMessagesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub message_info: Option<Vec<MessageInfoType>>,

    /// The id of the &lt;&lt;getdisplaymessagesrequest,GetDisplayMessagesRequest&gt;&gt; that requested this message.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// "to be continued" indicator. Indicates whether another part of the report follows in an upcoming NotifyDisplayMessagesRequest message. Default value when omitted is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyDisplayMessagesRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `request_id` - The id of the &lt;&lt;getdisplaymessagesrequest,GetDisplayMessagesRequest&gt;&gt; that requested this message.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(request_id: i32) -> Self {
        Self {
            message_info: None,
            request_id,
            tbc: None,
            custom_data: None,
        }
    }

    /// Sets the message_info field.
    ///
    /// * `message_info` - The message_info field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_message_info(&mut self, message_info: Option<Vec<MessageInfoType>>) -> &mut Self {
        self.message_info = message_info;
        self
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - The id of the &lt;&lt;getdisplaymessagesrequest,GetDisplayMessagesRequest&gt;&gt; that requested this message.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Sets the tbc field.
    ///
    /// * `tbc` - "to be continued" indicator. Indicates whether another part of the report follows in an upcoming NotifyDisplayMessagesRequest message. Default value when omitted is false.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_tbc(&mut self, tbc: Option<bool>) -> &mut Self {
        self.tbc = tbc;
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

    /// Gets a reference to the message_info field.
    ///
    /// # Returns
    ///
    /// The message_info field
    pub fn get_message_info(&self) -> Option<&Vec<MessageInfoType>> {
        self.message_info.as_ref()
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// The id of the &lt;&lt;getdisplaymessagesrequest,GetDisplayMessagesRequest&gt;&gt; that requested this message.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the tbc field.
    ///
    /// # Returns
    ///
    /// "to be continued" indicator. Indicates whether another part of the report follows in an upcoming NotifyDisplayMessagesRequest message. Default value when omitted is false.
    pub fn get_tbc(&self) -> Option<&bool> {
        self.tbc.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the message_info field and returns self for builder pattern.
    ///
    /// * `message_info` - The message_info field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_message_info(mut self, message_info: Vec<MessageInfoType>) -> Self {
        self.message_info = Some(message_info);
        self
    }

    /// Sets the tbc field and returns self for builder pattern.
    ///
    /// * `tbc` - "to be continued" indicator. Indicates whether another part of the report follows in an upcoming NotifyDisplayMessagesRequest message. Default value when omitted is false.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_tbc(mut self, tbc: bool) -> Self {
        self.tbc = Some(tbc);
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

/// Response body for the NotifyDisplayMessages response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDisplayMessagesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyDisplayMessagesResponse {
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
    use crate::v2_1::enumerations::{MessagePriorityEnumType, MessageStateEnumType};
    use chrono::Utc;
    use serde_json;
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_message_info() -> MessageInfoType {
        MessageInfoType::new(
            1,
            MessagePriorityEnumType::AlwaysFront,
            MessageStateEnumType::Idle,
            Utc::now(),
        )
    }

    #[test]
    fn test_notify_display_messages_request_new() {
        let request_id = 123;

        let request = NotifyDisplayMessagesRequest::new(request_id);

        assert_eq!(request.get_request_id(), &request_id);
        assert_eq!(request.get_message_info(), None);
        assert_eq!(request.get_tbc(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_notify_display_messages_request_serialization() {
        let request_id = 123;

        let request = NotifyDisplayMessagesRequest::new(request_id);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: NotifyDisplayMessagesRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_notify_display_messages_request_validation() {
        let request_id = 123;

        let request = NotifyDisplayMessagesRequest::new(request_id);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_display_messages_request_validation_negative_request_id() {
        let request_id = -1; // Invalid negative value

        let request = NotifyDisplayMessagesRequest::new(request_id);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_display_messages_request_validation_empty_message_info() {
        let request_id = 123;
        let message_info = vec![]; // Empty vector violates min length of 1

        let request = NotifyDisplayMessagesRequest::new(request_id)
            .with_message_info(message_info);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_display_messages_request_with_message_info() {
        let request_id = 123;
        let message_info = vec![create_test_message_info()];
        let custom_data = create_test_custom_data();

        let request = NotifyDisplayMessagesRequest::new(request_id)
            .with_message_info(message_info.clone())
            .with_tbc(true)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_request_id(), &request_id);
        assert_eq!(request.get_message_info(), Some(&message_info));
        assert_eq!(request.get_tbc(), Some(&true));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_display_messages_request_set_methods() {
        let request_id = 123;

        let mut request = NotifyDisplayMessagesRequest::new(request_id);

        let new_request_id = 456;
        let message_info = vec![create_test_message_info()];
        let custom_data = create_test_custom_data();

        request
            .set_request_id(new_request_id)
            .set_message_info(Some(message_info.clone()))
            .set_tbc(Some(true))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_request_id(), &new_request_id);
        assert_eq!(request.get_message_info(), Some(&message_info));
        assert_eq!(request.get_tbc(), Some(&true));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_display_messages_request_builder_pattern() {
        let request_id = 123;
        let message_info = vec![create_test_message_info()];
        let custom_data = create_test_custom_data();

        let request = NotifyDisplayMessagesRequest::new(request_id)
            .with_message_info(message_info.clone())
            .with_tbc(false)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_request_id(), &request_id);
        assert_eq!(request.get_message_info(), Some(&message_info));
        assert_eq!(request.get_tbc(), Some(&false));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_display_messages_request_json_round_trip() {
        let request_id = 123;
        let message_info = vec![create_test_message_info()];
        let custom_data = create_test_custom_data();

        let request = NotifyDisplayMessagesRequest::new(request_id)
            .with_message_info(message_info)
            .with_tbc(true)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: NotifyDisplayMessagesRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_notify_display_messages_request_multiple_message_info() {
        let request_id = 123;
        let message_info = vec![
            create_test_message_info(),
            MessageInfoType::new(
                2,
                MessagePriorityEnumType::InFront,
                MessageStateEnumType::Charging,
                Utc::now(),
            ),
        ];

        let request = NotifyDisplayMessagesRequest::new(request_id)
            .with_message_info(message_info.clone());

        assert_eq!(request.get_message_info(), Some(&message_info));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_display_messages_request_boundary_values() {
        // Test with minimum valid request_id
        let request_id = 0; // Minimum valid value

        let request = NotifyDisplayMessagesRequest::new(request_id);

        assert_eq!(request.get_request_id(), &request_id);
        assert!(request.validate().is_ok());

        // Test with minimum valid message_info length
        let message_info = vec![create_test_message_info()]; // Minimum length of 1
        let request = NotifyDisplayMessagesRequest::new(request_id)
            .with_message_info(message_info.clone());

        assert_eq!(request.get_message_info(), Some(&message_info));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_display_messages_response_new() {
        let response = NotifyDisplayMessagesResponse::new();

        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_notify_display_messages_response_serialization() {
        let response = NotifyDisplayMessagesResponse::new();

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: NotifyDisplayMessagesResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_notify_display_messages_response_validation() {
        let response = NotifyDisplayMessagesResponse::new();

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_notify_display_messages_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = NotifyDisplayMessagesResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_display_messages_response_set_custom_data() {
        let mut response = NotifyDisplayMessagesResponse::new();
        let custom_data = create_test_custom_data();

        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_display_messages_response_builder_pattern() {
        let custom_data = create_test_custom_data();

        let response = NotifyDisplayMessagesResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_display_messages_response_json_round_trip() {
        let custom_data = create_test_custom_data();
        let response = NotifyDisplayMessagesResponse::new()
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: NotifyDisplayMessagesResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_notify_display_messages_response_empty_json() {
        let json = "{}";
        let response: NotifyDisplayMessagesResponse =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(response.get_custom_data(), None);
        assert!(response.validate().is_ok());
    }
}