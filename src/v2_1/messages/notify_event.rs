use crate::v2_1::datatypes::CustomDataType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

/// Request body for the NotifyEvent request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEventRequest {
    /// Timestamp of the moment this message was generated at the Charging Station.
    pub generated_at: DateTime<Utc>,

    /// “to be continued” indicator. Indicates whether another part of the report follows in an upcoming notifyEventRequest message. Default value when omitted is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    /// Sequence number of this message. First message starts at 0.
    #[validate(range(min = 0))]
    pub seq_no: i32,

    #[validate(length(min = 1))]
    pub event_data: Vec<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyEventRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `generated_at` - Timestamp of the moment this message was generated at the Charging Station.
    /// * `seq_no` - Sequence number of this message. First message starts at 0.
    /// * `event_data` - The event_data field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(generated_at: DateTime<Utc>, seq_no: i32, event_data: Vec<Value>) -> Self {
        Self {
            generated_at,
            tbc: None,
            seq_no,
            event_data,
            custom_data: None,
        }
    }

    /// Sets the generated_at field.
    ///
    /// * `generated_at` - Timestamp of the moment this message was generated at the Charging Station.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_generated_at(&mut self, generated_at: DateTime<Utc>) -> &mut Self {
        self.generated_at = generated_at;
        self
    }

    /// Sets the tbc field.
    ///
    /// * `tbc` - “to be continued” indicator. Indicates whether another part of the report follows in an upcoming notifyEventRequest message. Default value when omitted is false.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_tbc(&mut self, tbc: Option<bool>) -> &mut Self {
        self.tbc = tbc;
        self
    }

    /// Sets the seq_no field.
    ///
    /// * `seq_no` - Sequence number of this message. First message starts at 0.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_seq_no(&mut self, seq_no: i32) -> &mut Self {
        self.seq_no = seq_no;
        self
    }

    /// Sets the event_data field.
    ///
    /// * `event_data` - The event_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_event_data(&mut self, event_data: Vec<Value>) -> &mut Self {
        self.event_data = event_data;
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

    /// Gets a reference to the generated_at field.
    ///
    /// # Returns
    ///
    /// Timestamp of the moment this message was generated at the Charging Station.
    pub fn get_generated_at(&self) -> &DateTime<Utc> {
        &self.generated_at
    }

    /// Gets a reference to the tbc field.
    ///
    /// # Returns
    ///
    /// “to be continued” indicator. Indicates whether another part of the report follows in an upcoming notifyEventRequest message. Default value when omitted is false.
    pub fn get_tbc(&self) -> Option<&bool> {
        self.tbc.as_ref()
    }

    /// Gets a reference to the seq_no field.
    ///
    /// # Returns
    ///
    /// Sequence number of this message. First message starts at 0.
    pub fn get_seq_no(&self) -> &i32 {
        &self.seq_no
    }

    /// Gets a reference to the event_data field.
    ///
    /// # Returns
    ///
    /// The event_data field
    pub fn get_event_data(&self) -> &Vec<Value> {
        &self.event_data
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the tbc field and returns self for builder pattern.
    ///
    /// * `tbc` - “to be continued” indicator. Indicates whether another part of the report follows in an upcoming notifyEventRequest message. Default value when omitted is false.
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

/// Response body for the NotifyEvent response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEventResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyEventResponse {
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
    use chrono::Utc;
    use serde_json::{self, json};
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_event_data() -> Vec<Value> {
        vec![
            json!({"eventType": "Alert", "component": "EVSE", "variable": "AvailabilityState"}),
            json!({"eventType": "Warning", "component": "Connector", "variable": "ConnectorType"}),
        ]
    }

    #[test]
    fn test_notify_event_request_new() {
        let generated_at = Utc::now();
        let seq_no = 0;
        let event_data = create_test_event_data();

        let request = NotifyEventRequest::new(
            generated_at,
            seq_no,
            event_data.clone(),
        );

        assert_eq!(request.get_generated_at(), &generated_at);
        assert_eq!(request.get_seq_no(), &seq_no);
        assert_eq!(request.get_event_data(), &event_data);
        assert_eq!(request.get_tbc(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_notify_event_request_validation() {
        let generated_at = Utc::now();
        let seq_no = 0;
        let event_data = create_test_event_data();

        let request = NotifyEventRequest::new(generated_at, seq_no, event_data);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_event_request_validation_negative_seq_no() {
        let generated_at = Utc::now();
        let seq_no = -1; // Invalid negative value
        let event_data = create_test_event_data();

        let request = NotifyEventRequest::new(generated_at, seq_no, event_data);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_event_request_validation_empty_event_data() {
        let generated_at = Utc::now();
        let seq_no = 0;
        let event_data = vec![]; // Empty vector violates min length of 1

        let request = NotifyEventRequest::new(generated_at, seq_no, event_data);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_event_request_with_optional_fields() {
        let generated_at = Utc::now();
        let seq_no = 1;
        let event_data = create_test_event_data();
        let custom_data = create_test_custom_data();

        let request = NotifyEventRequest::new(generated_at, seq_no, event_data.clone())
            .with_tbc(true)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_generated_at(), &generated_at);
        assert_eq!(request.get_seq_no(), &seq_no);
        assert_eq!(request.get_event_data(), &event_data);
        assert_eq!(request.get_tbc(), Some(&true));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_event_request_set_methods() {
        let generated_at = Utc::now();
        let seq_no = 0;
        let event_data = create_test_event_data();

        let mut request = NotifyEventRequest::new(generated_at, seq_no, event_data);

        let new_generated_at = Utc::now();
        let new_seq_no = 2;
        let new_event_data = vec![json!({"eventType": "Error", "component": "ChargingStation"})];
        let custom_data = create_test_custom_data();

        request
            .set_generated_at(new_generated_at)
            .set_seq_no(new_seq_no)
            .set_event_data(new_event_data.clone())
            .set_tbc(Some(false))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_generated_at(), &new_generated_at);
        assert_eq!(request.get_seq_no(), &new_seq_no);
        assert_eq!(request.get_event_data(), &new_event_data);
        assert_eq!(request.get_tbc(), Some(&false));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_event_request_json_round_trip() {
        let generated_at = Utc::now();
        let seq_no = 1;
        let event_data = create_test_event_data();
        let custom_data = create_test_custom_data();

        let request = NotifyEventRequest::new(generated_at, seq_no, event_data)
            .with_tbc(true)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: NotifyEventRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_notify_event_request_boundary_values() {
        let generated_at = Utc::now();

        // Test with minimum valid seq_no
        let seq_no = 0; // Minimum valid value
        let event_data = vec![json!({"eventType": "Info"})]; // Minimum length of 1

        let request = NotifyEventRequest::new(generated_at, seq_no, event_data.clone());

        assert_eq!(request.get_seq_no(), &seq_no);
        assert_eq!(request.get_event_data(), &event_data);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_event_response_new() {
        let response = NotifyEventResponse::new();

        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_notify_event_response_validation() {
        let response = NotifyEventResponse::new();

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_notify_event_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = NotifyEventResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_event_response_set_custom_data() {
        let mut response = NotifyEventResponse::new();
        let custom_data = create_test_custom_data();

        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_event_response_json_round_trip() {
        let custom_data = create_test_custom_data();
        let response = NotifyEventResponse::new()
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: NotifyEventResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_notify_event_response_empty_json() {
        let json = "{}";
        let response: NotifyEventResponse =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(response.get_custom_data(), None);
        assert!(response.validate().is_ok());
    }
}