use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{GetDisplayMessagesStatusEnumType, MessagePriorityEnumType, MessageStateEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetDisplayMessages request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetDisplayMessagesRequest {
    /// If provided the Charging Station shall return Display Messages of the given ids. This field SHALL NOT contain more ids than set in &lt;&lt;configkey-number-of-display-messages,NumberOfDisplayMessages.maxLimit&gt;&gt;
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub id: Option<Vec<i32>>,

    /// The Id of this request.
    #[validate(range(min = 0))]
    pub request_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<MessagePriorityEnumType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<MessageStateEnumType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetDisplayMessagesRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `request_id` - The Id of this request.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(request_id: i32) -> Self {
        Self {
            id: None,
            request_id,
            priority: None,
            state: None,
            custom_data: None,
        }
    }

    /// Sets the id field.
    ///
    /// * `id` - If provided the Charging Station shall return Display Messages of the given ids. This field SHALL NOT contain more ids than set in &lt;&lt;configkey-number-of-display-messages,NumberOfDisplayMessages.maxLimit&gt;&gt;
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_id(&mut self, id: Option<Vec<i32>>) -> &mut Self {
        self.id = id;
        self
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - The Id of this request.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Sets the priority field.
    ///
    /// * `priority` - The priority field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_priority(&mut self, priority: Option<MessagePriorityEnumType>) -> &mut Self {
        self.priority = priority;
        self
    }

    /// Sets the state field.
    ///
    /// * `state` - The state field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_state(&mut self, state: Option<MessageStateEnumType>) -> &mut Self {
        self.state = state;
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
    /// If provided the Charging Station shall return Display Messages of the given ids. This field SHALL NOT contain more ids than set in &lt;&lt;configkey-number-of-display-messages,NumberOfDisplayMessages.maxLimit&gt;&gt;
    pub fn get_id(&self) -> Option<&Vec<i32>> {
        self.id.as_ref()
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// The Id of this request.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the priority field.
    ///
    /// # Returns
    ///
    /// The priority field
    pub fn get_priority(&self) -> Option<&MessagePriorityEnumType> {
        self.priority.as_ref()
    }

    /// Gets a reference to the state field.
    ///
    /// # Returns
    ///
    /// The state field
    pub fn get_state(&self) -> Option<&MessageStateEnumType> {
        self.state.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the id field and returns self for builder pattern.
    ///
    /// * `id` - If provided the Charging Station shall return Display Messages of the given ids. This field SHALL NOT contain more ids than set in &lt;&lt;configkey-number-of-display-messages,NumberOfDisplayMessages.maxLimit&gt;&gt;
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_id(mut self, id: Vec<i32>) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the priority field and returns self for builder pattern.
    ///
    /// * `priority` - The priority field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_priority(mut self, priority: MessagePriorityEnumType) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Sets the state field and returns self for builder pattern.
    ///
    /// * `state` - The state field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_state(mut self, state: MessageStateEnumType) -> Self {
        self.state = Some(state);
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

/// Response body for the GetDisplayMessages response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetDisplayMessagesResponse {
    pub status: GetDisplayMessagesStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetDisplayMessagesResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: GetDisplayMessagesStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: GetDisplayMessagesStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &GetDisplayMessagesStatusEnumType {
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

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("TestReason".to_string())
    }

    // Tests for GetDisplayMessagesRequest

    #[test]
    fn test_get_display_messages_request_new() {
        let request = GetDisplayMessagesRequest::new(123);

        assert_eq!(request.id, None);
        assert_eq!(request.request_id, 123);
        assert_eq!(request.priority, None);
        assert_eq!(request.state, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_display_messages_request_with_id() {
        let ids = vec![1, 2, 3];
        let request = GetDisplayMessagesRequest::new(456)
            .with_id(ids.clone());

        assert_eq!(request.id, Some(ids));
        assert_eq!(request.request_id, 456);
        assert_eq!(request.priority, None);
        assert_eq!(request.state, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_display_messages_request_with_priority() {
        let request = GetDisplayMessagesRequest::new(789)
            .with_priority(MessagePriorityEnumType::AlwaysFront);

        assert_eq!(request.id, None);
        assert_eq!(request.request_id, 789);
        assert_eq!(request.priority, Some(MessagePriorityEnumType::AlwaysFront));
        assert_eq!(request.state, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_display_messages_request_with_state() {
        let request = GetDisplayMessagesRequest::new(999)
            .with_state(MessageStateEnumType::Charging);

        assert_eq!(request.id, None);
        assert_eq!(request.request_id, 999);
        assert_eq!(request.priority, None);
        assert_eq!(request.state, Some(MessageStateEnumType::Charging));
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_display_messages_request_with_custom_data() {
        let custom_data = create_test_custom_data();
        let request = GetDisplayMessagesRequest::new(111)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.id, None);
        assert_eq!(request.request_id, 111);
        assert_eq!(request.priority, None);
        assert_eq!(request.state, None);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_display_messages_request_setters() {
        let ids = vec![4, 5, 6];
        let custom_data = create_test_custom_data();

        let mut request = GetDisplayMessagesRequest::new(100);
        request.set_id(Some(ids.clone()));
        request.set_request_id(200);
        request.set_priority(Some(MessagePriorityEnumType::InFront));
        request.set_state(Some(MessageStateEnumType::Faulted));
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.id, Some(ids));
        assert_eq!(request.request_id, 200);
        assert_eq!(request.priority, Some(MessagePriorityEnumType::InFront));
        assert_eq!(request.state, Some(MessageStateEnumType::Faulted));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_display_messages_request_getters() {
        let ids = vec![7, 8, 9];
        let custom_data = create_test_custom_data();
        let request = GetDisplayMessagesRequest::new(555)
            .with_id(ids.clone())
            .with_priority(MessagePriorityEnumType::NormalCycle)
            .with_state(MessageStateEnumType::Idle)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_id(), Some(&ids));
        assert_eq!(request.get_request_id(), &555);
        assert_eq!(request.get_priority(), Some(&MessagePriorityEnumType::NormalCycle));
        assert_eq!(request.get_state(), Some(&MessageStateEnumType::Idle));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_display_messages_request_serialization() {
        let request = GetDisplayMessagesRequest::new(123);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetDisplayMessagesRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
    }

    #[test]
    fn test_get_display_messages_request_validation() {
        let request = GetDisplayMessagesRequest::new(100);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_get_display_messages_request_validation_negative_request_id() {
        let mut request = GetDisplayMessagesRequest::new(100);
        request.set_request_id(-1);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_display_messages_request_validation_empty_id_list() {
        let mut request = GetDisplayMessagesRequest::new(100);
        request.set_id(Some(vec![])); // Empty list should fail validation

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_display_messages_request_all_priority_types() {
        let priorities = vec![
            MessagePriorityEnumType::AlwaysFront,
            MessagePriorityEnumType::InFront,
            MessagePriorityEnumType::NormalCycle,
        ];

        for priority in priorities {
            let request = GetDisplayMessagesRequest::new(123)
                .with_priority(priority.clone());
            assert_eq!(request.priority, Some(priority));
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_get_display_messages_request_all_state_types() {
        let states = vec![
            MessageStateEnumType::Charging,
            MessageStateEnumType::Faulted,
            MessageStateEnumType::Idle,
            MessageStateEnumType::Unavailable,
            MessageStateEnumType::Suspended,
            MessageStateEnumType::Discharging,
        ];

        for state in states {
            let request = GetDisplayMessagesRequest::new(123)
                .with_state(state.clone());
            assert_eq!(request.state, Some(state));
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_get_display_messages_request_json_round_trip() {
        let ids = vec![10, 20, 30];
        let custom_data = create_test_custom_data();
        let request = GetDisplayMessagesRequest::new(777)
            .with_id(ids)
            .with_priority(MessagePriorityEnumType::AlwaysFront)
            .with_state(MessageStateEnumType::Charging)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetDisplayMessagesRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
        assert!(parsed.validate().is_ok());
    }

    // Tests for GetDisplayMessagesResponse

    #[test]
    fn test_get_display_messages_response_new() {
        let response = GetDisplayMessagesResponse::new(GetDisplayMessagesStatusEnumType::Accepted);

        assert_eq!(response.status, GetDisplayMessagesStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_display_messages_response_with_status_info() {
        let status_info = create_test_status_info();
        let response = GetDisplayMessagesResponse::new(GetDisplayMessagesStatusEnumType::Unknown)
            .with_status_info(status_info.clone());

        assert_eq!(response.status, GetDisplayMessagesStatusEnumType::Unknown);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_display_messages_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = GetDisplayMessagesResponse::new(GetDisplayMessagesStatusEnumType::Accepted)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, GetDisplayMessagesStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_display_messages_response_setters() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let mut response = GetDisplayMessagesResponse::new(GetDisplayMessagesStatusEnumType::Accepted);
        response.set_status(GetDisplayMessagesStatusEnumType::Unknown);
        response.set_status_info(Some(status_info.clone()));
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, GetDisplayMessagesStatusEnumType::Unknown);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_display_messages_response_getters() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();
        let response = GetDisplayMessagesResponse::new(GetDisplayMessagesStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &GetDisplayMessagesStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_display_messages_response_serialization() {
        let response = GetDisplayMessagesResponse::new(GetDisplayMessagesStatusEnumType::Accepted);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetDisplayMessagesResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
    }

    #[test]
    fn test_get_display_messages_response_validation() {
        let response = GetDisplayMessagesResponse::new(GetDisplayMessagesStatusEnumType::Accepted);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_get_display_messages_response_all_status_types() {
        let statuses = vec![
            GetDisplayMessagesStatusEnumType::Accepted,
            GetDisplayMessagesStatusEnumType::Unknown,
        ];

        for status in statuses {
            let response = GetDisplayMessagesResponse::new(status.clone());
            assert_eq!(response.status, status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_get_display_messages_response_json_round_trip() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();
        let response = GetDisplayMessagesResponse::new(GetDisplayMessagesStatusEnumType::Unknown)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetDisplayMessagesResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
        assert!(parsed.validate().is_ok());
    }
}