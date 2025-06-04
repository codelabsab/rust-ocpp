use crate::v2_1::datatypes::CustomDataType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the Heartbeat request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl HeartbeatRequest {
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

/// Response body for the Heartbeat response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatResponse {
    /// Contains the current time of the CSMS.
    pub current_time: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl HeartbeatResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `current_time` - Contains the current time of the CSMS.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(current_time: DateTime<Utc>) -> Self {
        Self {
            current_time,
            custom_data: None,
        }
    }

    /// Sets the current_time field.
    ///
    /// * `current_time` - Contains the current time of the CSMS.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_current_time(&mut self, current_time: DateTime<Utc>) -> &mut Self {
        self.current_time = current_time;
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

    /// Gets a reference to the current_time field.
    ///
    /// # Returns
    ///
    /// Contains the current time of the CSMS.
    pub fn get_current_time(&self) -> &DateTime<Utc> {
        &self.current_time
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
    use chrono::Utc;
    use serde_json;
    use validator::Validate;

    #[test]
    fn test_heartbeat_request_new() {
        let request = HeartbeatRequest::new();

        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_heartbeat_request_serialization() {
        let request = HeartbeatRequest::new();

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: HeartbeatRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_heartbeat_request_validation() {
        let request = HeartbeatRequest::new();

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_heartbeat_request_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = HeartbeatRequest::new().with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_heartbeat_request_set_custom_data() {
        let mut request = HeartbeatRequest::new();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_heartbeat_request_builder_pattern() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = HeartbeatRequest::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_heartbeat_response_new() {
        let current_time = Utc::now();
        let response = HeartbeatResponse::new(current_time);

        assert_eq!(response.get_current_time(), &current_time);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_heartbeat_response_serialization() {
        let current_time = Utc::now();
        let response = HeartbeatResponse::new(current_time);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: HeartbeatResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_heartbeat_response_validation() {
        let current_time = Utc::now();
        let response = HeartbeatResponse::new(current_time);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_heartbeat_response_with_custom_data() {
        let current_time = Utc::now();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = HeartbeatResponse::new(current_time)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_heartbeat_response_set_methods() {
        let current_time = Utc::now();
        let new_time = Utc::now();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = HeartbeatResponse::new(current_time);

        response
            .set_current_time(new_time)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_current_time(), &new_time);
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_heartbeat_response_builder_pattern() {
        let current_time = Utc::now();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = HeartbeatResponse::new(current_time)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_heartbeat_request_json_round_trip() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = HeartbeatRequest::new().with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: HeartbeatRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_heartbeat_response_json_round_trip() {
        let current_time = Utc::now();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = HeartbeatResponse::new(current_time)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: HeartbeatResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_heartbeat_request_empty_json() {
        let json = "{}";
        let request: HeartbeatRequest = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(request.get_custom_data(), None);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_heartbeat_response_with_custom_data_validation() {
        let current_time = Utc::now();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = HeartbeatResponse::new(current_time)
            .with_custom_data(custom_data);

        assert!(response.validate().is_ok());
    }
}
