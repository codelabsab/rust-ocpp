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
