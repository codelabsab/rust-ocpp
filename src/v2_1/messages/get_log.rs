use crate::v2_1::datatypes::{CustomDataType, LogParametersType, StatusInfoType};
use crate::v2_1::enumerations::{LogEnumType, LogStatusEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetLog request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetLogRequest {
    #[validate(nested)]
    pub log: LogParametersType,

    pub log_type: LogEnumType,

    /// The Id of this request
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// This specifies how many times the Charging Station must retry to upload the log before giving up. If this field is not present, it is left to Charging Station to decide how many times it wants to retry. If the value is 0, it means: no retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub retries: Option<i32>,

    /// The interval in seconds after which a retry may be attempted. If this field is not present, it is left to Charging Station to decide how long to wait between attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetLogRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `log` - The log field
    /// * `log_type` - The log_type field
    /// * `request_id` - The Id of this request
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(log: LogParametersType, log_type: LogEnumType, request_id: i32) -> Self {
        Self {
            log,
            log_type,
            request_id,
            retries: None,
            retry_interval: None,
            custom_data: None,
        }
    }

    /// Sets the log field.
    ///
    /// * `log` - The log field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_log(&mut self, log: LogParametersType) -> &mut Self {
        self.log = log;
        self
    }

    /// Sets the log_type field.
    ///
    /// * `log_type` - The log_type field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_log_type(&mut self, log_type: LogEnumType) -> &mut Self {
        self.log_type = log_type;
        self
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - The Id of this request
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Sets the retries field.
    ///
    /// * `retries` - This specifies how many times the Charging Station must retry to upload the log before giving up. If this field is not present, it is left to Charging Station to decide how many times it wants to retry. If the value is 0, it means: no retries.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_retries(&mut self, retries: Option<i32>) -> &mut Self {
        self.retries = retries;
        self
    }

    /// Sets the retry_interval field.
    ///
    /// * `retry_interval` - The interval in seconds after which a retry may be attempted. If this field is not present, it is left to Charging Station to decide how long to wait between attempts.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_retry_interval(&mut self, retry_interval: Option<i32>) -> &mut Self {
        self.retry_interval = retry_interval;
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

    /// Gets a reference to the log field.
    ///
    /// # Returns
    ///
    /// The log field
    pub fn get_log(&self) -> &LogParametersType {
        &self.log
    }

    /// Gets a reference to the log_type field.
    ///
    /// # Returns
    ///
    /// The log_type field
    pub fn get_log_type(&self) -> &LogEnumType {
        &self.log_type
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// The Id of this request
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the retries field.
    ///
    /// # Returns
    ///
    /// This specifies how many times the Charging Station must retry to upload the log before giving up. If this field is not present, it is left to Charging Station to decide how many times it wants to retry. If the value is 0, it means: no retries.
    pub fn get_retries(&self) -> Option<&i32> {
        self.retries.as_ref()
    }

    /// Gets a reference to the retry_interval field.
    ///
    /// # Returns
    ///
    /// The interval in seconds after which a retry may be attempted. If this field is not present, it is left to Charging Station to decide how long to wait between attempts.
    pub fn get_retry_interval(&self) -> Option<&i32> {
        self.retry_interval.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the retries field and returns self for builder pattern.
    ///
    /// * `retries` - This specifies how many times the Charging Station must retry to upload the log before giving up. If this field is not present, it is left to Charging Station to decide how many times it wants to retry. If the value is 0, it means: no retries.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_retries(mut self, retries: i32) -> Self {
        self.retries = Some(retries);
        self
    }

    /// Sets the retry_interval field and returns self for builder pattern.
    ///
    /// * `retry_interval` - The interval in seconds after which a retry may be attempted. If this field is not present, it is left to Charging Station to decide how long to wait between attempts.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_retry_interval(mut self, retry_interval: i32) -> Self {
        self.retry_interval = Some(retry_interval);
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

/// Response body for the GetLog response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetLogResponse {
    pub status: LogStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// This contains the name of the log file that will be uploaded. This field is not present when no logging information is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub filename: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetLogResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: LogStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
            filename: None,
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
    pub fn set_status(&mut self, status: LogStatusEnumType) -> &mut Self {
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

    /// Sets the filename field.
    ///
    /// * `filename` - This contains the name of the log file that will be uploaded. This field is not present when no logging information is available.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_filename(&mut self, filename: Option<String>) -> &mut Self {
        self.filename = filename;
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
    pub fn get_status(&self) -> &LogStatusEnumType {
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

    /// Gets a reference to the filename field.
    ///
    /// # Returns
    ///
    /// This contains the name of the log file that will be uploaded. This field is not present when no logging information is available.
    pub fn get_filename(&self) -> Option<&String> {
        self.filename.as_ref()
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

    /// Sets the filename field and returns self for builder pattern.
    ///
    /// * `filename` - This contains the name of the log file that will be uploaded. This field is not present when no logging information is available.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_filename(mut self, filename: String) -> Self {
        self.filename = Some(filename);
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
    use chrono::{TimeZone, Utc};
    use serde_json;

    // Helper function to create test LogParametersType
    fn create_test_log_parameters() -> LogParametersType {
        LogParametersType::new("https://example.com/logs".to_string())
            .with_oldest_timestamp(Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap())
            .with_latest_timestamp(Utc.with_ymd_and_hms(2023, 12, 31, 23, 59, 59).unwrap())
    }

    // Tests for GetLogRequest
    
    #[test]
    fn test_get_log_request_new() {
        let log_params = create_test_log_parameters();
        let request = GetLogRequest::new(
            log_params.clone(),
            LogEnumType::DiagnosticsLog,
            42
        );
        
        assert_eq!(request.log.remote_location, "https://example.com/logs");
        assert_eq!(request.log_type, LogEnumType::DiagnosticsLog);
        assert_eq!(request.request_id, 42);
        assert_eq!(request.retries, None);
        assert_eq!(request.retry_interval, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_log_request_with_optional_fields() {
        let log_params = create_test_log_parameters();
        let request = GetLogRequest::new(
            log_params,
            LogEnumType::SecurityLog,
            123
        )
        .with_retries(3)
        .with_retry_interval(60)
        .with_custom_data(CustomDataType::new("TestVendor".to_string()));
        
        assert_eq!(request.retries, Some(3));
        assert_eq!(request.retry_interval, Some(60));
        assert!(request.custom_data.is_some());
        assert_eq!(request.custom_data.as_ref().unwrap().vendor_id, "TestVendor");
    }

    #[test]
    fn test_get_log_request_setters() {
        let mut request = GetLogRequest::new(
            create_test_log_parameters(),
            LogEnumType::DiagnosticsLog,
            1
        );
        
        let new_log_params = LogParametersType::new("https://newlocation.com/logs".to_string());
        request.set_log(new_log_params);
        request.set_log_type(LogEnumType::SecurityLog);
        request.set_request_id(999);
        request.set_retries(Some(5));
        request.set_retry_interval(Some(120));
        request.set_custom_data(Some(CustomDataType::new("NewVendor".to_string())));
        
        assert_eq!(request.log.remote_location, "https://newlocation.com/logs");
        assert_eq!(request.log_type, LogEnumType::SecurityLog);
        assert_eq!(request.request_id, 999);
        assert_eq!(request.retries, Some(5));
        assert_eq!(request.retry_interval, Some(120));
        assert_eq!(request.custom_data.as_ref().unwrap().vendor_id, "NewVendor");
    }

    #[test]
    fn test_get_log_request_getters() {
        let log_params = create_test_log_parameters();
        let request = GetLogRequest::new(
            log_params.clone(),
            LogEnumType::DiagnosticsLog,
            42
        )
        .with_retries(3)
        .with_retry_interval(60);
        
        assert_eq!(request.get_log().remote_location, log_params.remote_location);
        assert_eq!(*request.get_log_type(), LogEnumType::DiagnosticsLog);
        assert_eq!(*request.get_request_id(), 42);
        assert_eq!(request.get_retries(), Some(&3));
        assert_eq!(request.get_retry_interval(), Some(&60));
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_get_log_request_serialization() {
        let log_params = create_test_log_parameters();
        let request = GetLogRequest::new(
            log_params,
            LogEnumType::SecurityLog,
            123
        );
        
        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetLogRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(request, parsed);
    }

    #[test]
    fn test_get_log_request_deserialization() {
        let json = r#"{
            "log": {
                "remoteLocation": "https://example.com/logs",
                "oldestTimestamp": "2023-01-01T00:00:00Z",
                "latestTimestamp": "2023-12-31T23:59:59Z"
            },
            "logType": "DiagnosticsLog",
            "requestId": 42,
            "retries": 3,
            "retryInterval": 60
        }"#;
        
        let request: GetLogRequest = serde_json::from_str(json).unwrap();
        
        assert_eq!(request.log.remote_location, "https://example.com/logs");
        assert_eq!(request.log_type, LogEnumType::DiagnosticsLog);
        assert_eq!(request.request_id, 42);
        assert_eq!(request.retries, Some(3));
        assert_eq!(request.retry_interval, Some(60));
    }

    #[test]
    fn test_get_log_request_validation_negative_request_id() {
        let request = GetLogRequest::new(
            create_test_log_parameters(),
            LogEnumType::DiagnosticsLog,
            -1  // Negative request ID
        );
        
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_log_request_validation_negative_retries() {
        let mut request = GetLogRequest::new(
            create_test_log_parameters(),
            LogEnumType::DiagnosticsLog,
            42
        );
        request.set_retries(Some(-1));
        
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_log_request_zero_retries() {
        let request = GetLogRequest::new(
            create_test_log_parameters(),
            LogEnumType::DiagnosticsLog,
            42
        )
        .with_retries(0);  // Zero retries is valid (means no retries)
        
        assert!(request.validate().is_ok());
        assert_eq!(request.retries, Some(0));
    }

    // Tests for GetLogResponse
    
    #[test]
    fn test_get_log_response_new() {
        let response = GetLogResponse::new(LogStatusEnumType::Accepted);
        
        assert_eq!(response.status, LogStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.filename, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_log_response_with_optional_fields() {
        let response = GetLogResponse::new(LogStatusEnumType::Accepted)
            .with_status_info(StatusInfoType::new("Success".to_string())
                .with_additional_info("Log file ready for upload".to_string()))
            .with_filename("chargepoint_20231231_235959.log".to_string())
            .with_custom_data(CustomDataType::new("TestVendor".to_string()));
        
        assert!(response.status_info.is_some());
        assert_eq!(response.status_info.as_ref().unwrap().reason_code, "Success");
        assert_eq!(response.filename, Some("chargepoint_20231231_235959.log".to_string()));
        assert!(response.custom_data.is_some());
    }

    #[test]
    fn test_get_log_response_setters() {
        let mut response = GetLogResponse::new(LogStatusEnumType::Accepted);
        
        response.set_status(LogStatusEnumType::Rejected);
        response.set_status_info(Some(StatusInfoType::new("NotSupported".to_string())));
        response.set_filename(Some("system.log".to_string()));
        response.set_custom_data(Some(CustomDataType::new("NewVendor".to_string())));
        
        assert_eq!(response.status, LogStatusEnumType::Rejected);
        assert!(response.status_info.is_some());
        assert_eq!(response.filename, Some("system.log".to_string()));
        assert_eq!(response.custom_data.as_ref().unwrap().vendor_id, "NewVendor");
    }

    #[test]
    fn test_get_log_response_getters() {
        let response = GetLogResponse::new(LogStatusEnumType::Accepted)
            .with_filename("test.log".to_string());
        
        assert_eq!(*response.get_status(), LogStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_filename(), Some(&"test.log".to_string()));
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_get_log_response_serialization() {
        let response = GetLogResponse::new(LogStatusEnumType::Accepted)
            .with_filename("diagnostics.log".to_string());
        
        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetLogResponse = serde_json::from_str(&json).unwrap();
        
        assert_eq!(response, parsed);
    }

    #[test]
    fn test_get_log_response_deserialization() {
        let json = r#"{
            "status": "Accepted",
            "filename": "security_20231231.log"
        }"#;
        
        let response: GetLogResponse = serde_json::from_str(json).unwrap();
        
        assert_eq!(response.status, LogStatusEnumType::Accepted);
        assert_eq!(response.filename, Some("security_20231231.log".to_string()));
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_log_response_validation_filename_too_long() {
        let response = GetLogResponse::new(LogStatusEnumType::Accepted)
            .with_filename("a".repeat(256));  // 256 characters, exceeds max length of 255
        
        assert!(response.validate().is_err());
    }

    #[test]
    fn test_get_log_response_all_status_types() {
        // Test with different status types
        let statuses = vec![
            LogStatusEnumType::Accepted,
            LogStatusEnumType::Rejected,
            LogStatusEnumType::AcceptedCanceled,
        ];
        
        for status in statuses {
            let response = GetLogResponse::new(status.clone());
            assert_eq!(response.status, status);
        }
    }

    #[test]
    fn test_get_log_request_all_log_types() {
        // Test with different log types
        let log_types = vec![
            LogEnumType::DiagnosticsLog,
            LogEnumType::SecurityLog,
        ];
        
        for log_type in log_types {
            let request = GetLogRequest::new(
                create_test_log_parameters(),
                log_type.clone(),
                42
            );
            assert_eq!(request.log_type, log_type);
        }
    }

    #[test]
    fn test_get_log_request_json_round_trip_with_all_fields() {
        let log_params = LogParametersType::new("https://example.com/logs".to_string())
            .with_oldest_timestamp(Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap())
            .with_latest_timestamp(Utc.with_ymd_and_hms(2023, 12, 31, 23, 59, 59).unwrap())
            .with_custom_data(CustomDataType::new("LogVendor".to_string()));
            
        let request = GetLogRequest::new(
            log_params,
            LogEnumType::SecurityLog,
            999
        )
        .with_retries(5)
        .with_retry_interval(300)
        .with_custom_data(CustomDataType::new("RequestVendor".to_string()));
        
        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetLogRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(request, parsed);
        assert_eq!(parsed.retries, Some(5));
        assert_eq!(parsed.retry_interval, Some(300));
        assert_eq!(parsed.log.custom_data.as_ref().unwrap().vendor_id, "LogVendor");
        assert_eq!(parsed.custom_data.as_ref().unwrap().vendor_id, "RequestVendor");
    }

    #[test]
    fn test_get_log_response_json_round_trip_with_all_fields() {
        let response = GetLogResponse::new(LogStatusEnumType::Accepted)
            .with_status_info(StatusInfoType::new("LogReady".to_string())
                .with_additional_info("Log file has been prepared and is ready for upload".to_string()))
            .with_filename("charging_station_diagnostics_20231231.log".to_string())
            .with_custom_data(CustomDataType::new("ResponseVendor".to_string()));
        
        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetLogResponse = serde_json::from_str(&json).unwrap();
        
        assert_eq!(response, parsed);
        assert_eq!(parsed.status_info.as_ref().unwrap().reason_code, "LogReady");
        assert_eq!(parsed.status_info.as_ref().unwrap().additional_info, 
                   Some("Log file has been prepared and is ready for upload".to_string()));
        assert_eq!(parsed.filename, Some("charging_station_diagnostics_20231231.log".to_string()));
        assert_eq!(parsed.custom_data.as_ref().unwrap().vendor_id, "ResponseVendor");
    }

    #[test]
    fn test_get_log_request_with_minimal_log_parameters() {
        let log_params = LogParametersType::new("ftp://logs.example.com/upload".to_string());
        let request = GetLogRequest::new(
            log_params,
            LogEnumType::DiagnosticsLog,
            1
        );
        
        assert_eq!(request.log.remote_location, "ftp://logs.example.com/upload");
        assert_eq!(request.log.oldest_timestamp, None);
        assert_eq!(request.log.latest_timestamp, None);
        assert!(request.validate().is_ok());
    }
}
