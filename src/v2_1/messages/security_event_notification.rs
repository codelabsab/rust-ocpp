use crate::v2_1::datatypes::CustomDataType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the SecurityEventNotification request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SecurityEventNotificationRequest {
    /// Type of the security event. This value should be taken from the Security events list.
    #[serde(rename = "type")]
    #[validate(length(max = 50))]
    pub type_: String,

    /// Date and time at which the event occurred.
    pub timestamp: DateTime<Utc>,

    /// Additional information about the occurred security event.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub tech_info: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SecurityEventNotificationRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `type_` - Type of the security event. This value should be taken from the Security events list.
    /// * `timestamp` - Date and time at which the event occurred.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(type_: String, timestamp: DateTime<Utc>) -> Self {
        Self {
            type_,
            timestamp,
            tech_info: None,
            custom_data: None,
        }
    }

    /// Sets the type_ field.
    ///
    /// * `type_` - Type of the security event. This value should be taken from the Security events list.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_type_(&mut self, type_: String) -> &mut Self {
        self.type_ = type_;
        self
    }

    /// Sets the timestamp field.
    ///
    /// * `timestamp` - Date and time at which the event occurred.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_timestamp(&mut self, timestamp: DateTime<Utc>) -> &mut Self {
        self.timestamp = timestamp;
        self
    }

    /// Sets the tech_info field.
    ///
    /// * `tech_info` - Additional information about the occurred security event.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_tech_info(&mut self, tech_info: Option<String>) -> &mut Self {
        self.tech_info = tech_info;
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

    /// Gets a reference to the type_ field.
    ///
    /// # Returns
    ///
    /// Type of the security event. This value should be taken from the Security events list.
    pub fn get_type_(&self) -> &String {
        &self.type_
    }

    /// Gets a reference to the timestamp field.
    ///
    /// # Returns
    ///
    /// Date and time at which the event occurred.
    pub fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    /// Gets a reference to the tech_info field.
    ///
    /// # Returns
    ///
    /// Additional information about the occurred security event.
    pub fn get_tech_info(&self) -> Option<&String> {
        self.tech_info.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the tech_info field and returns self for builder pattern.
    ///
    /// * `tech_info` - Additional information about the occurred security event.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_tech_info(mut self, tech_info: String) -> Self {
        self.tech_info = Some(tech_info);
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

/// Response body for the SecurityEventNotification response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SecurityEventNotificationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SecurityEventNotificationResponse {
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
