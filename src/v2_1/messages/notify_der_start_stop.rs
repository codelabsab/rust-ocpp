use crate::v2_1::datatypes::CustomDataType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the NotifyDERStartStop request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDERStartStopRequest {
    /// Id of the started or stopped DER control. Corresponds to the _controlId_ of the SetDERControlRequest.
    #[validate(length(max = 36))]
    pub control_id: String,

    /// True if DER control has started. False if it has ended.
    pub started: bool,

    /// Time of start or end of event.
    pub timestamp: DateTime<Utc>,

    /// List of controlIds that are superseded as a result of this control starting.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    pub superseded_ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyDERStartStopRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `control_id` - Id of the started or stopped DER control. Corresponds to the _controlId_ of the SetDERControlRequest.
    /// * `started` - True if DER control has started. False if it has ended.
    /// * `timestamp` - Time of start or end of event.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(control_id: String, started: bool, timestamp: DateTime<Utc>) -> Self {
        Self {
            control_id,
            started,
            timestamp,
            superseded_ids: None,
            custom_data: None,
        }
    }

    /// Sets the control_id field.
    ///
    /// * `control_id` - Id of the started or stopped DER control. Corresponds to the _controlId_ of the SetDERControlRequest.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_control_id(&mut self, control_id: String) -> &mut Self {
        self.control_id = control_id;
        self
    }

    /// Sets the started field.
    ///
    /// * `started` - True if DER control has started. False if it has ended.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_started(&mut self, started: bool) -> &mut Self {
        self.started = started;
        self
    }

    /// Sets the timestamp field.
    ///
    /// * `timestamp` - Time of start or end of event.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_timestamp(&mut self, timestamp: DateTime<Utc>) -> &mut Self {
        self.timestamp = timestamp;
        self
    }

    /// Sets the superseded_ids field.
    ///
    /// * `superseded_ids` - List of controlIds that are superseded as a result of this control starting.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_superseded_ids(&mut self, superseded_ids: Option<Vec<String>>) -> &mut Self {
        self.superseded_ids = superseded_ids;
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

    /// Gets a reference to the control_id field.
    ///
    /// # Returns
    ///
    /// Id of the started or stopped DER control. Corresponds to the _controlId_ of the SetDERControlRequest.
    pub fn get_control_id(&self) -> &String {
        &self.control_id
    }

    /// Gets a reference to the started field.
    ///
    /// # Returns
    ///
    /// True if DER control has started. False if it has ended.
    pub fn get_started(&self) -> &bool {
        &self.started
    }

    /// Gets a reference to the timestamp field.
    ///
    /// # Returns
    ///
    /// Time of start or end of event.
    pub fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    /// Gets a reference to the superseded_ids field.
    ///
    /// # Returns
    ///
    /// List of controlIds that are superseded as a result of this control starting.
    pub fn get_superseded_ids(&self) -> Option<&Vec<String>> {
        self.superseded_ids.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the superseded_ids field and returns self for builder pattern.
    ///
    /// * `superseded_ids` - List of controlIds that are superseded as a result of this control starting.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_superseded_ids(mut self, superseded_ids: Vec<String>) -> Self {
        self.superseded_ids = Some(superseded_ids);
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

/// Response body for the NotifyDERStartStop response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDERStartStopResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyDERStartStopResponse {
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
    use serde_json;
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    #[test]
    fn test_notify_der_start_stop_request_new() {
        let control_id = "control123".to_string();
        let started = true;
        let timestamp = Utc::now();

        let request = NotifyDERStartStopRequest::new(
            control_id.clone(),
            started,
            timestamp,
        );

        assert_eq!(request.get_control_id(), &control_id);
        assert_eq!(request.get_started(), &started);
        assert_eq!(request.get_timestamp(), &timestamp);
        assert_eq!(request.get_superseded_ids(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_notify_der_start_stop_request_serialization() {
        let control_id = "control123".to_string();
        let started = true;
        let timestamp = Utc::now();

        let request = NotifyDERStartStopRequest::new(
            control_id,
            started,
            timestamp,
        );

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: NotifyDERStartStopRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_notify_der_start_stop_request_validation() {
        let control_id = "control123".to_string();
        let started = true;
        let timestamp = Utc::now();

        let request = NotifyDERStartStopRequest::new(
            control_id,
            started,
            timestamp,
        );

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_der_start_stop_request_validation_control_id_too_long() {
        let control_id = "x".repeat(37); // Exceeds max length of 36
        let started = true;
        let timestamp = Utc::now();

        let request = NotifyDERStartStopRequest::new(
            control_id,
            started,
            timestamp,
        );

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_der_start_stop_request_validation_superseded_ids_empty() {
        let control_id = "control123".to_string();
        let started = true;
        let timestamp = Utc::now();
        let superseded_ids = vec![]; // Empty vector violates min length of 1

        let request = NotifyDERStartStopRequest::new(
            control_id,
            started,
            timestamp,
        ).with_superseded_ids(superseded_ids);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_der_start_stop_request_validation_superseded_ids_too_many() {
        let control_id = "control123".to_string();
        let started = true;
        let timestamp = Utc::now();
        let superseded_ids = (0..25).map(|i| format!("control{}", i)).collect(); // Exceeds max length of 24

        let request = NotifyDERStartStopRequest::new(
            control_id,
            started,
            timestamp,
        ).with_superseded_ids(superseded_ids);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_der_start_stop_request_with_superseded_ids() {
        let control_id = "control123".to_string();
        let started = true;
        let timestamp = Utc::now();
        let superseded_ids = vec!["control1".to_string(), "control2".to_string()];
        let custom_data = create_test_custom_data();

        let request = NotifyDERStartStopRequest::new(
            control_id.clone(),
            started,
            timestamp,
        )
        .with_superseded_ids(superseded_ids.clone())
        .with_custom_data(custom_data.clone());

        assert_eq!(request.get_control_id(), &control_id);
        assert_eq!(request.get_started(), &started);
        assert_eq!(request.get_timestamp(), &timestamp);
        assert_eq!(request.get_superseded_ids(), Some(&superseded_ids));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_der_start_stop_request_set_methods() {
        let control_id = "control123".to_string();
        let started = true;
        let timestamp = Utc::now();

        let mut request = NotifyDERStartStopRequest::new(
            control_id,
            started,
            timestamp,
        );

        let new_control_id = "newcontrol456".to_string();
        let new_started = false;
        let new_timestamp = Utc::now();
        let superseded_ids = vec!["old1".to_string(), "old2".to_string()];
        let custom_data = create_test_custom_data();

        request
            .set_control_id(new_control_id.clone())
            .set_started(new_started)
            .set_timestamp(new_timestamp)
            .set_superseded_ids(Some(superseded_ids.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_control_id(), &new_control_id);
        assert_eq!(request.get_started(), &new_started);
        assert_eq!(request.get_timestamp(), &new_timestamp);
        assert_eq!(request.get_superseded_ids(), Some(&superseded_ids));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_der_start_stop_request_builder_pattern() {
        let control_id = "control123".to_string();
        let started = true;
        let timestamp = Utc::now();
        let superseded_ids = vec!["control1".to_string()];
        let custom_data = create_test_custom_data();

        let request = NotifyDERStartStopRequest::new(
            control_id.clone(),
            started,
            timestamp,
        )
        .with_superseded_ids(superseded_ids.clone())
        .with_custom_data(custom_data.clone());

        assert_eq!(request.get_control_id(), &control_id);
        assert_eq!(request.get_started(), &started);
        assert_eq!(request.get_superseded_ids(), Some(&superseded_ids));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_der_start_stop_request_json_round_trip() {
        let control_id = "control123".to_string();
        let started = false;
        let timestamp = Utc::now();
        let superseded_ids = vec!["control1".to_string(), "control2".to_string()];
        let custom_data = create_test_custom_data();

        let request = NotifyDERStartStopRequest::new(
            control_id,
            started,
            timestamp,
        )
        .with_superseded_ids(superseded_ids)
        .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: NotifyDERStartStopRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_notify_der_start_stop_request_boundary_values() {
        // Test with maximum valid control_id length
        let control_id = "x".repeat(36); // Maximum allowed length
        let started = true;
        let timestamp = Utc::now();

        let request = NotifyDERStartStopRequest::new(
            control_id.clone(),
            started,
            timestamp,
        );

        assert_eq!(request.get_control_id(), &control_id);
        assert!(request.validate().is_ok());

        // Test with minimum valid superseded_ids length
        let superseded_ids = vec!["control1".to_string()]; // Minimum length of 1
        let request = NotifyDERStartStopRequest::new(
            "control123".to_string(),
            started,
            timestamp,
        ).with_superseded_ids(superseded_ids.clone());

        assert_eq!(request.get_superseded_ids(), Some(&superseded_ids));
        assert!(request.validate().is_ok());

        // Test with maximum valid superseded_ids length
        let superseded_ids = (0..24).map(|i| format!("control{}", i)).collect::<Vec<_>>(); // Maximum length of 24
        let request = NotifyDERStartStopRequest::new(
            "control123".to_string(),
            started,
            timestamp,
        ).with_superseded_ids(superseded_ids.clone());

        assert_eq!(request.get_superseded_ids(), Some(&superseded_ids));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_der_start_stop_response_new() {
        let response = NotifyDERStartStopResponse::new();

        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_notify_der_start_stop_response_serialization() {
        let response = NotifyDERStartStopResponse::new();

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: NotifyDERStartStopResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_notify_der_start_stop_response_validation() {
        let response = NotifyDERStartStopResponse::new();

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_notify_der_start_stop_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = NotifyDERStartStopResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_der_start_stop_response_set_custom_data() {
        let mut response = NotifyDERStartStopResponse::new();
        let custom_data = create_test_custom_data();

        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_der_start_stop_response_builder_pattern() {
        let custom_data = create_test_custom_data();

        let response = NotifyDERStartStopResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_der_start_stop_response_json_round_trip() {
        let custom_data = create_test_custom_data();
        let response = NotifyDERStartStopResponse::new()
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: NotifyDERStartStopResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_notify_der_start_stop_response_empty_json() {
        let json = "{}";
        let response: NotifyDERStartStopResponse =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(response.get_custom_data(), None);
        assert!(response.validate().is_ok());
    }
}