use crate::v2_1::datatypes::CustomDataType;
use crate::v2_1::enumerations::ConnectorStatusEnumType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the StatusNotification request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationRequest {
    /// The time for which the status is reported.
    pub timestamp: DateTime<Utc>,

    pub connector_status: ConnectorStatusEnumType,

    /// The id of the EVSE to which the connector belongs for which the the status is reported.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    /// The id of the connector within the EVSE for which the status is reported.
    #[validate(range(min = 0))]
    pub connector_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl StatusNotificationRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `timestamp` - The time for which the status is reported.
    /// * `connector_status` - The connector_status field
    /// * `evse_id` - The id of the EVSE to which the connector belongs for which the the status is reported.
    /// * `connector_id` - The id of the connector within the EVSE for which the status is reported.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(timestamp: DateTime<Utc>, connector_status: ConnectorStatusEnumType, evse_id: i32, connector_id: i32) -> Self {
        Self {
            timestamp,
            connector_status,
            evse_id,
            connector_id,
            custom_data: None,
        }
    }

    /// Sets the timestamp field.
    ///
    /// * `timestamp` - The time for which the status is reported.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_timestamp(&mut self, timestamp: DateTime<Utc>) -> &mut Self {
        self.timestamp = timestamp;
        self
    }

    /// Sets the connector_status field.
    ///
    /// * `connector_status` - The connector_status field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_connector_status(&mut self, connector_status: ConnectorStatusEnumType) -> &mut Self {
        self.connector_status = connector_status;
        self
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - The id of the EVSE to which the connector belongs for which the the status is reported.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: i32) -> &mut Self {
        self.evse_id = evse_id;
        self
    }

    /// Sets the connector_id field.
    ///
    /// * `connector_id` - The id of the connector within the EVSE for which the status is reported.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_connector_id(&mut self, connector_id: i32) -> &mut Self {
        self.connector_id = connector_id;
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

    /// Gets a reference to the timestamp field.
    ///
    /// # Returns
    ///
    /// The time for which the status is reported.
    pub fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    /// Gets a reference to the connector_status field.
    ///
    /// # Returns
    ///
    /// The connector_status field
    pub fn get_connector_status(&self) -> &ConnectorStatusEnumType {
        &self.connector_status
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// The id of the EVSE to which the connector belongs for which the the status is reported.
    pub fn get_evse_id(&self) -> &i32 {
        &self.evse_id
    }

    /// Gets a reference to the connector_id field.
    ///
    /// # Returns
    ///
    /// The id of the connector within the EVSE for which the status is reported.
    pub fn get_connector_id(&self) -> &i32 {
        &self.connector_id
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

/// Response body for the StatusNotification response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl StatusNotificationResponse {
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
    use crate::v2_1::enumerations::ConnectorStatusEnumType;
    use chrono::Utc;
    use serde_json;
    use validator::Validate;

    // Tests for StatusNotificationRequest

    #[test]
    fn test_status_notification_request_new() {
        let timestamp = Utc::now();
        let connector_status = ConnectorStatusEnumType::Available;
        let evse_id = 1;
        let connector_id = 1;
        let request = StatusNotificationRequest::new(timestamp, connector_status.clone(), evse_id, connector_id);

        assert_eq!(request.get_timestamp(), &timestamp);
        assert_eq!(request.get_connector_status(), &connector_status);
        assert_eq!(request.get_evse_id(), &evse_id);
        assert_eq!(request.get_connector_id(), &connector_id);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_status_notification_request_serialization() {
        let timestamp = Utc::now();
        let request = StatusNotificationRequest::new(timestamp, ConnectorStatusEnumType::Occupied, 1, 1);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: StatusNotificationRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_status_notification_request_validation() {
        let timestamp = Utc::now();
        let request = StatusNotificationRequest::new(timestamp, ConnectorStatusEnumType::Available, 1, 1);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_status_notification_request_with_custom_data() {
        let timestamp = Utc::now();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = StatusNotificationRequest::new(timestamp, ConnectorStatusEnumType::Reserved, 2, 3)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_status_notification_request_set_methods() {
        let timestamp = Utc::now();
        let new_timestamp = Utc::now();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = StatusNotificationRequest::new(timestamp, ConnectorStatusEnumType::Available, 1, 1);

        request
            .set_timestamp(new_timestamp)
            .set_connector_status(ConnectorStatusEnumType::Faulted)
            .set_evse_id(5)
            .set_connector_id(10)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_timestamp(), &new_timestamp);
        assert_eq!(request.get_connector_status(), &ConnectorStatusEnumType::Faulted);
        assert_eq!(request.get_evse_id(), &5);
        assert_eq!(request.get_connector_id(), &10);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_status_notification_request_builder_pattern() {
        let timestamp = Utc::now();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = StatusNotificationRequest::new(timestamp, ConnectorStatusEnumType::Unavailable, 3, 2)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_status_notification_request_negative_evse_id_validation() {
        let timestamp = Utc::now();
        let mut request = StatusNotificationRequest::new(timestamp, ConnectorStatusEnumType::Available, 1, 1);
        request.set_evse_id(-1);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_status_notification_request_negative_connector_id_validation() {
        let timestamp = Utc::now();
        let mut request = StatusNotificationRequest::new(timestamp, ConnectorStatusEnumType::Available, 1, 1);
        request.set_connector_id(-1);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_status_notification_request_zero_ids_validation() {
        let timestamp = Utc::now();
        let request = StatusNotificationRequest::new(timestamp, ConnectorStatusEnumType::Available, 0, 0);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_status_notification_request_all_connector_statuses() {
        let timestamp = Utc::now();
        let statuses = vec![
            ConnectorStatusEnumType::Available,
            ConnectorStatusEnumType::Occupied,
            ConnectorStatusEnumType::Reserved,
            ConnectorStatusEnumType::Unavailable,
            ConnectorStatusEnumType::Faulted,
        ];

        for status in statuses {
            let request = StatusNotificationRequest::new(timestamp, status.clone(), 1, 1);
            
            assert_eq!(request.get_connector_status(), &status);
            assert!(request.validate().is_ok());

            let json = serde_json::to_string(&request).expect("Failed to serialize");
            let deserialized: StatusNotificationRequest = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(request, deserialized);
        }
    }

    // Tests for StatusNotificationResponse

    #[test]
    fn test_status_notification_response_new() {
        let response = StatusNotificationResponse::new();

        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_status_notification_response_serialization() {
        let response = StatusNotificationResponse::new();

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: StatusNotificationResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_status_notification_response_validation() {
        let response = StatusNotificationResponse::new();

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_status_notification_response_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = StatusNotificationResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_status_notification_response_set_custom_data() {
        let mut response = StatusNotificationResponse::new();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_status_notification_response_builder_pattern() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = StatusNotificationResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_status_notification_request_json_round_trip() {
        let timestamp = Utc::now();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = StatusNotificationRequest::new(timestamp, ConnectorStatusEnumType::Occupied, 2, 1)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: StatusNotificationRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_status_notification_response_json_round_trip() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = StatusNotificationResponse::new()
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: StatusNotificationResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_status_notification_response_empty_json() {
        let json = "{}";
        let response: StatusNotificationResponse = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(response.get_custom_data(), None);
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_status_notification_request_with_large_ids() {
        let timestamp = Utc::now();
        let request = StatusNotificationRequest::new(timestamp, ConnectorStatusEnumType::Available, 999999, 888888);

        assert_eq!(request.get_evse_id(), &999999);
        assert_eq!(request.get_connector_id(), &888888);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_status_notification_response_with_custom_data_validation() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = StatusNotificationResponse::new()
            .with_custom_data(custom_data);

        assert!(response.validate().is_ok());
    }
}