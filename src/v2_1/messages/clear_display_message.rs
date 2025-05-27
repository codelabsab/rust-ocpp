use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::ClearMessageStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the ClearDisplayMessage request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearDisplayMessageRequest {
    /// Id of the message that SHALL be removed from the Charging Station.
    #[validate(range(min = 0))]
    pub id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClearDisplayMessageRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `id` - Id of the message that SHALL be removed from the Charging Station.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(id: i32) -> Self {
        Self {
            id,
            custom_data: None,
        }
    }

    /// Sets the id field.
    ///
    /// * `id` - Id of the message that SHALL be removed from the Charging Station.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
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
    /// Id of the message that SHALL be removed from the Charging Station.
    pub fn get_id(&self) -> &i32 {
        &self.id
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

/// Response body for the ClearDisplayMessage response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearDisplayMessageResponse {
    pub status: ClearMessageStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClearDisplayMessageResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: ClearMessageStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: ClearMessageStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &ClearMessageStatusEnumType {
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
    use validator::Validate;

    #[test]
    fn test_clear_display_message_request_new() {
        let request = ClearDisplayMessageRequest::new(123);
        assert_eq!(request.get_id(), &123);
        assert_eq!(request.get_custom_data(), None);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_clear_display_message_request_validation_invalid_id() {
        let request = ClearDisplayMessageRequest::new(-1); // Invalid: must be >= 0
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_clear_display_message_request_serialization() {
        let request = ClearDisplayMessageRequest::new(456);
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ClearDisplayMessageRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_clear_display_message_response_new() {
        let status = ClearMessageStatusEnumType::Accepted;
        let response = ClearDisplayMessageResponse::new(status.clone());
        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_clear_display_message_response_all_status_values() {
        let status_values = vec![
            ClearMessageStatusEnumType::Accepted,
            ClearMessageStatusEnumType::Unknown,
        ];

        for status in status_values {
            let response = ClearDisplayMessageResponse::new(status.clone());
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_clear_display_message_request_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = ClearDisplayMessageRequest::new(789)
            .with_custom_data(custom_data.clone());
        assert_eq!(request.get_custom_data(), Some(&custom_data));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_clear_display_message_response_with_status_info() {
        let status = ClearMessageStatusEnumType::Unknown;
        let status_info = StatusInfoType::new("NotFound".to_string());
        let response = ClearDisplayMessageResponse::new(status)
            .with_status_info(status_info.clone());
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_clear_display_message_json_round_trip() {
        let request = ClearDisplayMessageRequest::new(999)
            .with_custom_data(CustomDataType::new("TestVendor".to_string()));

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ClearDisplayMessageRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());

        let response = ClearDisplayMessageResponse::new(ClearMessageStatusEnumType::Accepted)
            .with_status_info(StatusInfoType::new("Success".to_string()));

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ClearDisplayMessageResponse = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }
}