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
