use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::GenericStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the PublishFirmware request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareRequest {
    /// This contains a string containing a URI pointing to a location from which to retrieve the firmware.
    #[validate(length(max = 2000))]
    pub location: String,

    /// This specifies how many times Charging Station must retry to download the firmware before giving up. If this field is not present, it is left to Charging Station to decide how many times it wants to retry. If the value is 0, it means: no retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub retries: Option<i32>,

    /// The MD5 checksum over the entire firmware file as a hexadecimal string of length 32.
    #[validate(length(max = 32))]
    pub checksum: String,

    /// The Id of the request.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// The interval in seconds after which a retry may be attempted. If this field is not present, it is left to Charging Station to decide how long to wait between attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub retry_interval: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl PublishFirmwareRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `location` - This contains a string containing a URI pointing to a location from which to retrieve the firmware.
    /// * `checksum` - The MD5 checksum over the entire firmware file as a hexadecimal string of length 32.
    /// * `request_id` - The Id of the request.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(location: String, checksum: String, request_id: i32) -> Self {
        Self {
            location,
            retries: None,
            checksum,
            request_id,
            retry_interval: None,
            custom_data: None,
        }
    }

    /// Sets the location field.
    ///
    /// * `location` - This contains a string containing a URI pointing to a location from which to retrieve the firmware.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_location(&mut self, location: String) -> &mut Self {
        self.location = location;
        self
    }

    /// Sets the retries field.
    ///
    /// * `retries` - This specifies how many times Charging Station must retry to download the firmware before giving up. If this field is not present, it is left to Charging Station to decide how many times it wants to retry. If the value is 0, it means: no retries.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_retries(&mut self, retries: Option<i32>) -> &mut Self {
        self.retries = retries;
        self
    }

    /// Sets the checksum field.
    ///
    /// * `checksum` - The MD5 checksum over the entire firmware file as a hexadecimal string of length 32.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_checksum(&mut self, checksum: String) -> &mut Self {
        self.checksum = checksum;
        self
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - The Id of the request.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
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

    /// Gets a reference to the location field.
    ///
    /// # Returns
    ///
    /// This contains a string containing a URI pointing to a location from which to retrieve the firmware.
    pub fn get_location(&self) -> &String {
        &self.location
    }

    /// Gets a reference to the retries field.
    ///
    /// # Returns
    ///
    /// This specifies how many times Charging Station must retry to download the firmware before giving up. If this field is not present, it is left to Charging Station to decide how many times it wants to retry. If the value is 0, it means: no retries.
    pub fn get_retries(&self) -> Option<&i32> {
        self.retries.as_ref()
    }

    /// Gets a reference to the checksum field.
    ///
    /// # Returns
    ///
    /// The MD5 checksum over the entire firmware file as a hexadecimal string of length 32.
    pub fn get_checksum(&self) -> &String {
        &self.checksum
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// The Id of the request.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
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
    /// * `retries` - This specifies how many times Charging Station must retry to download the firmware before giving up. If this field is not present, it is left to Charging Station to decide how many times it wants to retry. If the value is 0, it means: no retries.
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

/// Response body for the PublishFirmware response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareResponse {
    pub status: GenericStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl PublishFirmwareResponse {
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
    use serde_json;
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("Test status info".to_string())
    }

    fn create_test_publish_firmware_request() -> PublishFirmwareRequest {
        PublishFirmwareRequest::new(
            "https://example.com/firmware.bin".to_string(),
            "d41d8cd98f00b204e9800998ecf8427e".to_string(), // Valid MD5 hash
            123,
        )
    }

    fn create_test_publish_firmware_response() -> PublishFirmwareResponse {
        PublishFirmwareResponse::new(GenericStatusEnumType::Accepted)
    }

    #[test]
    fn test_publish_firmware_request_new() {
        let location = "https://firmware.example.com/v1.0.bin".to_string();
        let checksum = "a1b2c3d4e5f6789012345678901234ab".to_string();
        let request_id = 456;

        let request = PublishFirmwareRequest::new(location.clone(), checksum.clone(), request_id);

        assert_eq!(request.location, location);
        assert_eq!(request.checksum, checksum);
        assert_eq!(request.request_id, request_id);
        assert!(request.retries.is_none());
        assert!(request.retry_interval.is_none());
        assert!(request.custom_data.is_none());
    }

    #[test]
    fn test_publish_firmware_request_serialization() {
        let request = create_test_publish_firmware_request();

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: PublishFirmwareRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_publish_firmware_request_validation_valid() {
        let request = create_test_publish_firmware_request();
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_publish_firmware_request_validation_location_too_long() {
        let long_location = "a".repeat(2001); // Max is 2000
        let request = PublishFirmwareRequest::new(
            long_location,
            "d41d8cd98f00b204e9800998ecf8427e".to_string(),
            123,
        );

        let validation_result = request.validate();
        assert!(validation_result.is_err());
        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("location"));
    }

    #[test]
    fn test_publish_firmware_request_validation_checksum_too_long() {
        let long_checksum = "a".repeat(33); // Max is 32
        let request = PublishFirmwareRequest::new(
            "https://example.com/firmware.bin".to_string(),
            long_checksum,
            123,
        );

        let validation_result = request.validate();
        assert!(validation_result.is_err());
        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("checksum"));
    }

    #[test]
    fn test_publish_firmware_request_validation_negative_request_id() {
        let request = PublishFirmwareRequest::new(
            "https://example.com/firmware.bin".to_string(),
            "d41d8cd98f00b204e9800998ecf8427e".to_string(),
            -1,
        );

        let validation_result = request.validate();
        assert!(validation_result.is_err());
        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("request_id"));
    }

    #[test]
    fn test_publish_firmware_request_validation_negative_retries() {
        let mut request = create_test_publish_firmware_request();
        request.retries = Some(-1);

        let validation_result = request.validate();
        assert!(validation_result.is_err());
        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("retries"));
    }

    #[test]
    fn test_publish_firmware_request_validation_negative_retry_interval() {
        let mut request = create_test_publish_firmware_request();
        request.retry_interval = Some(-1);

        let validation_result = request.validate();
        assert!(validation_result.is_err());
        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("retry_interval"));
    }

    #[test]
    fn test_publish_firmware_request_set_methods() {
        let mut request = create_test_publish_firmware_request();
        let new_location = "https://new.example.com/firmware.bin".to_string();
        let new_checksum = "1234567890abcdef1234567890abcdef".to_string();
        let new_request_id = 999;

        request.set_location(new_location.clone())
               .set_checksum(new_checksum.clone())
               .set_request_id(new_request_id)
               .set_retries(Some(5))
               .set_retry_interval(Some(300));

        assert_eq!(request.location, new_location);
        assert_eq!(request.checksum, new_checksum);
        assert_eq!(request.request_id, new_request_id);
        assert_eq!(request.retries, Some(5));
        assert_eq!(request.retry_interval, Some(300));
    }

    #[test]
    fn test_publish_firmware_request_get_methods() {
        let request = create_test_publish_firmware_request();

        assert_eq!(request.get_location(), &request.location);
        assert_eq!(request.get_checksum(), &request.checksum);
        assert_eq!(request.get_request_id(), &request.request_id);
        assert_eq!(request.get_retries(), None);
        assert_eq!(request.get_retry_interval(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_publish_firmware_request_with_methods() {
        let custom_data = create_test_custom_data();

        let request = create_test_publish_firmware_request()
            .with_retries(3)
            .with_retry_interval(600)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.retries, Some(3));
        assert_eq!(request.retry_interval, Some(600));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_publish_firmware_request_boundary_values() {
        // Test maximum length values
        let max_location = "a".repeat(2000);
        let max_checksum = "a".repeat(32);

        let request = PublishFirmwareRequest::new(max_location.clone(), max_checksum.clone(), 0)
            .with_retries(0)
            .with_retry_interval(0);

        assert!(request.validate().is_ok());
        assert_eq!(request.location, max_location);
        assert_eq!(request.checksum, max_checksum);
        assert_eq!(request.request_id, 0);
        assert_eq!(request.retries, Some(0));
        assert_eq!(request.retry_interval, Some(0));

        // Test large valid values
        let request_max = PublishFirmwareRequest::new(
            "https://example.com/firmware.bin".to_string(),
            "d41d8cd98f00b204e9800998ecf8427e".to_string(),
            i32::MAX,
        )
        .with_retries(i32::MAX)
        .with_retry_interval(i32::MAX);

        assert!(request_max.validate().is_ok());
    }

    #[test]
    fn test_publish_firmware_request_json_format() {
        let request = create_test_publish_firmware_request();
        let json = serde_json::to_value(&request).expect("Failed to serialize to JSON");

        assert!(json.get("location").is_some());
        assert!(json.get("checksum").is_some());
        assert!(json.get("requestId").is_some());

        // Optional fields should not be present if None
        if request.retries.is_none() {
            assert!(json.get("retries").is_none());
        }
        if request.retry_interval.is_none() {
            assert!(json.get("retryInterval").is_none());
        }
        if request.custom_data.is_none() {
            assert!(json.get("customData").is_none());
        }
    }

    #[test]
    fn test_publish_firmware_request_md5_checksums() {
        // Test various valid MD5 checksums
        let valid_checksums = vec![
            "d41d8cd98f00b204e9800998ecf8427e", // Empty string MD5
            "5d41402abc4b2a76b9719d911017c592", // "hello" MD5
            "098f6bcd4621d373cade4e832627b4f6", // "test" MD5
            "ABCDEF1234567890ABCDEF1234567890", // Uppercase
            "abcdef1234567890abcdef1234567890", // Lowercase
        ];

        for checksum in valid_checksums {
            let request = PublishFirmwareRequest::new(
                "https://example.com/firmware.bin".to_string(),
                checksum.to_string(),
                123,
            );
            assert!(request.validate().is_ok());
            assert_eq!(request.checksum, checksum);
        }
    }

    #[test]
    fn test_publish_firmware_response_new() {
        let status = GenericStatusEnumType::Rejected;
        let response = PublishFirmwareResponse::new(status.clone());

        assert_eq!(response.status, status);
        assert!(response.status_info.is_none());
        assert!(response.custom_data.is_none());
    }

    #[test]
    fn test_publish_firmware_response_serialization() {
        let response = create_test_publish_firmware_response();

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: PublishFirmwareResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_publish_firmware_response_validation_valid() {
        let response = create_test_publish_firmware_response();
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_publish_firmware_response_set_methods() {
        let mut response = create_test_publish_firmware_response();
        let new_status = GenericStatusEnumType::Rejected;
        let status_info = create_test_status_info();

        response.set_status(new_status.clone())
                .set_status_info(Some(status_info.clone()));

        assert_eq!(response.status, new_status);
        assert_eq!(response.status_info, Some(status_info));
    }

    #[test]
    fn test_publish_firmware_response_get_methods() {
        let response = create_test_publish_firmware_response();

        assert_eq!(response.get_status(), &response.status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_publish_firmware_response_with_methods() {
        let custom_data = create_test_custom_data();
        let status_info = create_test_status_info();

        let response = create_test_publish_firmware_response()
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_publish_firmware_response_status_variants() {
        // Test all GenericStatusEnumType variants
        let statuses = vec![
            GenericStatusEnumType::Accepted,
            GenericStatusEnumType::Rejected,
        ];

        for status in statuses {
            let response = PublishFirmwareResponse::new(status.clone());
            assert_eq!(response.status, status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_publish_firmware_response_json_format() {
        let response = create_test_publish_firmware_response();
        let json = serde_json::to_value(&response).expect("Failed to serialize to JSON");

        assert!(json.get("status").is_some());

        // Optional fields should not be present if None
        if response.status_info.is_none() {
            assert!(json.get("statusInfo").is_none());
        }
        if response.custom_data.is_none() {
            assert!(json.get("customData").is_none());
        }
    }

    #[test]
    fn test_publish_firmware_round_trip_with_all_fields() {
        let custom_data = create_test_custom_data();
        let status_info = create_test_status_info();

        let request = create_test_publish_firmware_request()
            .with_retries(5)
            .with_retry_interval(300)
            .with_custom_data(custom_data.clone());

        let response = create_test_publish_firmware_response()
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        // Test request round trip
        let request_json = serde_json::to_string(&request).expect("Failed to serialize request");
        let request_deserialized: PublishFirmwareRequest = serde_json::from_str(&request_json).expect("Failed to deserialize request");
        assert_eq!(request, request_deserialized);

        // Test response round trip
        let response_json = serde_json::to_string(&response).expect("Failed to serialize response");
        let response_deserialized: PublishFirmwareResponse = serde_json::from_str(&response_json).expect("Failed to deserialize response");
        assert_eq!(response, response_deserialized);
    }

    #[test]
    fn test_publish_firmware_retry_scenarios() {
        // Test no retries (0)
        let request_no_retry = create_test_publish_firmware_request().with_retries(0);
        assert_eq!(request_no_retry.retries, Some(0));
        assert!(request_no_retry.validate().is_ok());

        // Test multiple retries
        let retry_counts = vec![1, 3, 5, 10, 100];
        for retry_count in retry_counts {
            let request = create_test_publish_firmware_request().with_retries(retry_count);
            assert_eq!(request.retries, Some(retry_count));
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_publish_firmware_retry_interval_scenarios() {
        // Test various retry intervals
        let intervals = vec![0, 30, 60, 300, 600, 1800, 3600];

        for interval in intervals {
            let request = create_test_publish_firmware_request().with_retry_interval(interval);
            assert_eq!(request.retry_interval, Some(interval));
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_publish_firmware_url_scenarios() {
        // Test various URL formats
        let urls = vec![
            "https://example.com/firmware.bin",
            "http://firmware.local/update.bin",
            "ftp://files.example.com/fw/v1.0.bin",
            "file:///local/path/firmware.bin",
        ];

        for url in urls {
            let request = PublishFirmwareRequest::new(
                url.to_string(),
                "d41d8cd98f00b204e9800998ecf8427e".to_string(),
                123,
            );
            assert_eq!(request.location, url);
            assert!(request.validate().is_ok());
        }
    }
}