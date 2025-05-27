use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::GenericStatusEnumType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the AFRRSignal request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AFRRSignalRequest {
    /// Time when signal becomes active.
    pub timestamp: DateTime<Utc>,

    /// Value of signal in _v2xSignalWattCurve_.
    pub signal: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl AFRRSignalRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `timestamp` - Time when signal becomes active.
    /// * `signal` - Value of signal in _v2xSignalWattCurve_.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(timestamp: DateTime<Utc>, signal: i32) -> Self {
        Self {
            timestamp,
            signal,
            custom_data: None,
        }
    }

    /// Sets the timestamp field.
    ///
    /// * `timestamp` - Time when signal becomes active.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_timestamp(&mut self, timestamp: DateTime<Utc>) -> &mut Self {
        self.timestamp = timestamp;
        self
    }

    /// Sets the signal field.
    ///
    /// * `signal` - Value of signal in _v2xSignalWattCurve_.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_signal(&mut self, signal: i32) -> &mut Self {
        self.signal = signal;
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
    /// Time when signal becomes active.
    pub fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    /// Gets a reference to the signal field.
    ///
    /// # Returns
    ///
    /// Value of signal in _v2xSignalWattCurve_.
    pub fn get_signal(&self) -> &i32 {
        &self.signal
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

/// Response body for the AFRRSignal response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AFRRSignalResponse {
    pub status: GenericStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl AFRRSignalResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: GenericStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: GenericStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &GenericStatusEnumType {
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
    use chrono::Utc;
    use serde_json;
    use validator::Validate;

    fn create_test_timestamp() -> DateTime<Utc> {
        Utc::now()
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("200".to_string())
    }

    #[test]
    fn test_afrr_signal_request_new() {
        let timestamp = create_test_timestamp();
        let signal = 1000;
        let request = AFRRSignalRequest::new(timestamp, signal);

        assert_eq!(request.get_timestamp(), &timestamp);
        assert_eq!(request.get_signal(), &signal);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_afrr_signal_request_validation() {
        let timestamp = create_test_timestamp();
        let signal = 1000;
        let request = AFRRSignalRequest::new(timestamp, signal);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_afrr_signal_request_serialization() {
        let timestamp = create_test_timestamp();
        let signal = 1000;
        let request = AFRRSignalRequest::new(timestamp, signal);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: AFRRSignalRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_afrr_signal_request_with_custom_data() {
        let timestamp = create_test_timestamp();
        let signal = 1000;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = AFRRSignalRequest::new(timestamp, signal)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_afrr_signal_request_set_methods() {
        let timestamp = create_test_timestamp();
        let new_timestamp = Utc::now();
        let signal = 1000;
        let new_signal = 2000;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = AFRRSignalRequest::new(timestamp, signal);

        request
            .set_timestamp(new_timestamp)
            .set_signal(new_signal)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_timestamp(), &new_timestamp);
        assert_eq!(request.get_signal(), &new_signal);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_afrr_signal_request_signal_edge_cases() {
        let timestamp = create_test_timestamp();

        // Test with zero signal
        let request = AFRRSignalRequest::new(timestamp, 0);
        assert_eq!(request.get_signal(), &0);
        assert!(request.validate().is_ok());

        // Test with negative signal
        let request = AFRRSignalRequest::new(timestamp, -1000);
        assert_eq!(request.get_signal(), &-1000);
        assert!(request.validate().is_ok());

        // Test with maximum signal
        let request = AFRRSignalRequest::new(timestamp, i32::MAX);
        assert_eq!(request.get_signal(), &i32::MAX);
        assert!(request.validate().is_ok());

        // Test with minimum signal
        let request = AFRRSignalRequest::new(timestamp, i32::MIN);
        assert_eq!(request.get_signal(), &i32::MIN);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_afrr_signal_request_timestamp_precision() {
        use chrono::{TimeZone, Utc};

        // Test with specific timestamp
        let timestamp = Utc.with_ymd_and_hms(2023, 12, 25, 10, 30, 45).unwrap();
        let signal = 1500;
        let request = AFRRSignalRequest::new(timestamp, signal);

        assert_eq!(request.get_timestamp(), &timestamp);
        assert!(request.validate().is_ok());

        // Test serialization preserves timestamp precision
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: AFRRSignalRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request.get_timestamp(), deserialized.get_timestamp());
    }

    #[test]
    fn test_afrr_signal_response_new() {
        let status = GenericStatusEnumType::Accepted;
        let response = AFRRSignalResponse::new(status.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_afrr_signal_response_validation() {
        let status = GenericStatusEnumType::Accepted;
        let response = AFRRSignalResponse::new(status);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_afrr_signal_response_serialization() {
        let status = GenericStatusEnumType::Accepted;
        let response = AFRRSignalResponse::new(status);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: AFRRSignalResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_afrr_signal_response_with_status_info() {
        let status = GenericStatusEnumType::Accepted;
        let status_info = create_test_status_info();

        let response = AFRRSignalResponse::new(status)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
    }

    #[test]
    fn test_afrr_signal_response_with_custom_data() {
        let status = GenericStatusEnumType::Accepted;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = AFRRSignalResponse::new(status)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_afrr_signal_response_set_methods() {
        let status = GenericStatusEnumType::Accepted;
        let new_status = GenericStatusEnumType::Rejected;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = AFRRSignalResponse::new(status);

        response
            .set_status(new_status.clone())
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_status(), &new_status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_afrr_signal_response_all_status_values() {
        let status_values = vec![
            GenericStatusEnumType::Accepted,
            GenericStatusEnumType::Rejected,
        ];

        for status in status_values {
            let response = AFRRSignalResponse::new(status.clone());
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_afrr_signal_response_builder_pattern() {
        let status = GenericStatusEnumType::Accepted;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = AFRRSignalResponse::new(status.clone())
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_afrr_signal_request_json_round_trip() {
        let timestamp = create_test_timestamp();
        let signal = 1000;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = AFRRSignalRequest::new(timestamp, signal)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: AFRRSignalRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_afrr_signal_response_json_round_trip() {
        let status = GenericStatusEnumType::Rejected;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = AFRRSignalResponse::new(status)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: AFRRSignalResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_afrr_signal_response_with_detailed_status_info() {
        let status = GenericStatusEnumType::Rejected;
        let status_info = StatusInfoType::new("InvalidSignal".to_string())
            .with_additional_info("The provided signal value is outside acceptable range".to_string());

        let response = AFRRSignalResponse::new(status)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
        assert!(response.validate().is_ok());

        // Test serialization
        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: AFRRSignalResponse = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_afrr_signal_request_clear_optional_fields() {
        let timestamp = create_test_timestamp();
        let signal = 1000;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = AFRRSignalRequest::new(timestamp, signal)
            .with_custom_data(custom_data);

        // Verify custom data is set
        assert!(request.get_custom_data().is_some());

        // Clear custom data
        request.set_custom_data(None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_afrr_signal_response_clear_optional_fields() {
        let status = GenericStatusEnumType::Accepted;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = AFRRSignalResponse::new(status)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        // Verify fields are set
        assert!(response.get_status_info().is_some());
        assert!(response.get_custom_data().is_some());

        // Clear optional fields
        response.set_status_info(None);
        response.set_custom_data(None);

        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_afrr_signal_request_with_complex_custom_data() {
        use serde_json::json;

        let timestamp = create_test_timestamp();
        let signal = 1500;
        let custom_data = CustomDataType::new("AFRRVendor".to_string())
            .with_property("region".to_string(), json!("EU"))
            .with_property("priority".to_string(), json!(5))
            .with_property("metadata".to_string(), json!({
                "source": "grid_operator",
                "version": "2.1"
            }));

        let request = AFRRSignalRequest::new(timestamp, signal)
            .with_custom_data(custom_data);

        assert!(request.validate().is_ok());

        // Test serialization with complex custom data
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: AFRRSignalRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_afrr_signal_request_signal_ranges() {
        let timestamp = create_test_timestamp();

        // Test typical positive signal values
        let positive_signals = vec![1, 100, 1000, 5000, 10000];
        for signal in positive_signals {
            let request = AFRRSignalRequest::new(timestamp, signal);
            assert_eq!(request.get_signal(), &signal);
            assert!(request.validate().is_ok());
        }

        // Test typical negative signal values
        let negative_signals = vec![-1, -100, -1000, -5000, -10000];
        for signal in negative_signals {
            let request = AFRRSignalRequest::new(timestamp, signal);
            assert_eq!(request.get_signal(), &signal);
            assert!(request.validate().is_ok());
        }
    }
}