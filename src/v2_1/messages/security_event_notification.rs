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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::datatypes::CustomDataType;
    use chrono::{DateTime, Utc};

    #[test]
    fn test_security_event_notification_request_new() {
        let timestamp = Utc::now();
        let request = SecurityEventNotificationRequest::new("FirmwareMismatch".to_string(), timestamp);

        assert_eq!(request.type_, "FirmwareMismatch");
        assert_eq!(request.timestamp, timestamp);
        assert_eq!(request.tech_info, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_security_event_notification_request_serialization() {
        let timestamp = DateTime::parse_from_rfc3339("2023-01-01T12:00:00Z").unwrap().with_timezone(&Utc);
        let request = SecurityEventNotificationRequest::new("InvalidCertificate".to_string(), timestamp)
            .with_tech_info("Certificate validation failed".to_string());

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: SecurityEventNotificationRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(json.contains("\"type\":\"InvalidCertificate\""));
        assert!(json.contains("\"techInfo\":\"Certificate validation failed\""));
    }

    #[test]
    fn test_security_event_notification_request_validation() {
        let timestamp = Utc::now();

        // Test valid request
        let valid_request = SecurityEventNotificationRequest::new("ValidEvent".to_string(), timestamp);
        assert!(valid_request.validate().is_ok());

        // Test invalid type (too long)
        let mut invalid_request = SecurityEventNotificationRequest::new("a".repeat(51), timestamp);
        assert!(invalid_request.validate().is_err());

        // Test invalid tech_info (too long)
        invalid_request = SecurityEventNotificationRequest::new("ValidEvent".to_string(), timestamp);
        invalid_request.tech_info = Some("a".repeat(256));
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_security_event_notification_request_builder_pattern() {
        let timestamp = Utc::now();
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let request = SecurityEventNotificationRequest::new("SecurityLog".to_string(), timestamp)
            .with_tech_info("Unauthorized access attempt".to_string())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.type_, "SecurityLog");
        assert_eq!(request.tech_info, Some("Unauthorized access attempt".to_string()));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_security_event_notification_request_setters() {
        let timestamp1 = Utc::now();
        let timestamp2 = timestamp1 + chrono::Duration::hours(1);
        let mut request = SecurityEventNotificationRequest::new("InitialEvent".to_string(), timestamp1);
        let custom_data = CustomDataType::new("test_vendor".to_string());

        request.set_type_("UpdatedEvent".to_string())
               .set_timestamp(timestamp2)
               .set_tech_info(Some("Updated info".to_string()))
               .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.type_, "UpdatedEvent");
        assert_eq!(request.timestamp, timestamp2);
        assert_eq!(request.tech_info, Some("Updated info".to_string()));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_security_event_notification_request_getters() {
        let timestamp = Utc::now();
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let request = SecurityEventNotificationRequest::new("TestEvent".to_string(), timestamp)
            .with_tech_info("Test info".to_string())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_type_(), &"TestEvent".to_string());
        assert_eq!(request.get_timestamp(), &timestamp);
        assert_eq!(request.get_tech_info(), Some(&"Test info".to_string()));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_security_event_notification_response_new() {
        let response = SecurityEventNotificationResponse::new();
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_security_event_notification_response_serialization() {
        let response = SecurityEventNotificationResponse::new();

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: SecurityEventNotificationResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_security_event_notification_response_builder_pattern() {
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let response = SecurityEventNotificationResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_security_event_notification_response_setters() {
        let mut response = SecurityEventNotificationResponse::new();
        let custom_data = CustomDataType::new("test_vendor".to_string());

        response.set_custom_data(Some(custom_data.clone()));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_security_event_notification_response_getters() {
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let response = SecurityEventNotificationResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_security_event_notification_edge_cases() {
        let timestamp = Utc::now();

        // Test with maximum allowed string lengths
        let max_type = "a".repeat(50);
        let max_tech_info = "b".repeat(255);

        let request = SecurityEventNotificationRequest::new(max_type.clone(), timestamp)
            .with_tech_info(max_tech_info.clone());

        assert!(request.validate().is_ok());
        assert_eq!(request.type_, max_type);
        assert_eq!(request.tech_info, Some(max_tech_info));
    }

    #[test]
    fn test_security_event_notification_response_validation() {
        let response = SecurityEventNotificationResponse::new();
        assert!(response.validate().is_ok());
    }
}