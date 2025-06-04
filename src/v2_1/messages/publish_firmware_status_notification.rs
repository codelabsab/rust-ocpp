use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::PublishFirmwareStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the PublishFirmwareStatusNotification request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareStatusNotificationRequest {
    pub status: PublishFirmwareStatusEnumType,

    /// Required if status is Published. Can be multiple URI’s, if the Local Controller supports e.g. HTTP, HTTPS, and FTP.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub location: Option<Vec<String>>,

    /// The request id that was provided in the PublishFirmwareRequest which triggered this action.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub request_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl PublishFirmwareStatusNotificationRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: PublishFirmwareStatusEnumType) -> Self {
        Self {
            status,
            location: None,
            request_id: None,
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
    pub fn set_status(&mut self, status: PublishFirmwareStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Sets the location field.
    ///
    /// * `location` - Required if status is Published. Can be multiple URI’s, if the Local Controller supports e.g. HTTP, HTTPS, and FTP.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_location(&mut self, location: Option<Vec<String>>) -> &mut Self {
        self.location = location;
        self
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - The request id that was provided in the PublishFirmwareRequest which triggered this action.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: Option<i32>) -> &mut Self {
        self.request_id = request_id;
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
    pub fn get_status(&self) -> &PublishFirmwareStatusEnumType {
        &self.status
    }

    /// Gets a reference to the location field.
    ///
    /// # Returns
    ///
    /// Required if status is Published. Can be multiple URI’s, if the Local Controller supports e.g. HTTP, HTTPS, and FTP.
    pub fn get_location(&self) -> Option<&Vec<String>> {
        self.location.as_ref()
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// The request id that was provided in the PublishFirmwareRequest which triggered this action.
    pub fn get_request_id(&self) -> Option<&i32> {
        self.request_id.as_ref()
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

    /// Sets the location field and returns self for builder pattern.
    ///
    /// * `location` - Required if status is Published. Can be multiple URI’s, if the Local Controller supports e.g. HTTP, HTTPS, and FTP.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_location(mut self, location: Vec<String>) -> Self {
        self.location = Some(location);
        self
    }

    /// Sets the request_id field and returns self for builder pattern.
    ///
    /// * `request_id` - The request id that was provided in the PublishFirmwareRequest which triggered this action.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_request_id(mut self, request_id: i32) -> Self {
        self.request_id = Some(request_id);
        self
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

/// Response body for the PublishFirmwareStatusNotification response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareStatusNotificationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl PublishFirmwareStatusNotificationResponse {
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
    use serde_json;
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("Test status info".to_string())
    }

    fn create_test_publish_firmware_status_notification_request() -> PublishFirmwareStatusNotificationRequest {
        PublishFirmwareStatusNotificationRequest::new(PublishFirmwareStatusEnumType::Published)
    }

    fn create_test_publish_firmware_status_notification_response() -> PublishFirmwareStatusNotificationResponse {
        PublishFirmwareStatusNotificationResponse::new()
    }

    #[test]
    fn test_publish_firmware_status_notification_request_new() {
        let status = PublishFirmwareStatusEnumType::Published;
        let request = PublishFirmwareStatusNotificationRequest::new(status.clone());

        assert_eq!(request.status, status);
        assert!(request.location.is_none());
        assert!(request.request_id.is_none());
        assert!(request.status_info.is_none());
        assert!(request.custom_data.is_none());
    }

    #[test]
    fn test_publish_firmware_status_notification_request_serialization() {
        let request = create_test_publish_firmware_status_notification_request();

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: PublishFirmwareStatusNotificationRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_publish_firmware_status_notification_request_validation_valid() {
        let request = create_test_publish_firmware_status_notification_request();
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_publish_firmware_status_notification_request_validation_empty_location() {
        let mut request = create_test_publish_firmware_status_notification_request();
        request.location = Some(vec![]); // Empty vector should fail validation

        let validation_result = request.validate();
        assert!(validation_result.is_err());
        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("location"));
    }

    #[test]
    fn test_publish_firmware_status_notification_request_validation_negative_request_id() {
        let mut request = create_test_publish_firmware_status_notification_request();
        request.request_id = Some(-1);

        let validation_result = request.validate();
        assert!(validation_result.is_err());
        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("request_id"));
    }

    #[test]
    fn test_publish_firmware_status_notification_request_set_methods() {
        let mut request = create_test_publish_firmware_status_notification_request();
        let new_status = PublishFirmwareStatusEnumType::Published;
        let locations = vec!["https://example.com/fw1.bin".to_string(), "https://example.com/fw2.bin".to_string()];
        let status_info = create_test_status_info();

        request.set_status(new_status.clone())
               .set_location(Some(locations.clone()))
               .set_request_id(Some(123))
               .set_status_info(Some(status_info.clone()));

        assert_eq!(request.status, new_status);
        assert_eq!(request.location, Some(locations));
        assert_eq!(request.request_id, Some(123));
        assert_eq!(request.status_info, Some(status_info));
    }

    #[test]
    fn test_publish_firmware_status_notification_request_get_methods() {
        let request = create_test_publish_firmware_status_notification_request();

        assert_eq!(request.get_status(), &request.status);
        assert_eq!(request.get_location(), None);
        assert_eq!(request.get_request_id(), None);
        assert_eq!(request.get_status_info(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_publish_firmware_status_notification_request_with_methods() {
        let custom_data = create_test_custom_data();
        let status_info = create_test_status_info();
        let locations = vec!["https://example.com/firmware.bin".to_string()];

        let request = create_test_publish_firmware_status_notification_request()
            .with_location(locations.clone())
            .with_request_id(456)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.location, Some(locations));
        assert_eq!(request.request_id, Some(456));
        assert_eq!(request.status_info, Some(status_info));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_publish_firmware_status_notification_request_status_variants() {
        // Test all PublishFirmwareStatusEnumType variants
        let statuses = vec![
            PublishFirmwareStatusEnumType::Published,
            PublishFirmwareStatusEnumType::DownloadScheduled,
            PublishFirmwareStatusEnumType::Downloading,
            PublishFirmwareStatusEnumType::Downloaded,
            PublishFirmwareStatusEnumType::DownloadFailed,
            PublishFirmwareStatusEnumType::InvalidChecksum,
            PublishFirmwareStatusEnumType::NotDownloaded,
        ];

        for status in statuses {
            let request = PublishFirmwareStatusNotificationRequest::new(status.clone());
            assert_eq!(request.status, status);
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_publish_firmware_status_notification_request_boundary_values() {
        // Test minimum valid request ID
        let request_min = create_test_publish_firmware_status_notification_request()
            .with_request_id(0);
        assert!(request_min.validate().is_ok());
        assert_eq!(request_min.request_id, Some(0));

        // Test large valid request ID
        let request_max = create_test_publish_firmware_status_notification_request()
            .with_request_id(i32::MAX);
        assert!(request_max.validate().is_ok());
        assert_eq!(request_max.request_id, Some(i32::MAX));
    }

    #[test]
    fn test_publish_firmware_status_notification_request_json_format() {
        let request = create_test_publish_firmware_status_notification_request();
        let json = serde_json::to_value(&request).expect("Failed to serialize to JSON");

        assert!(json.get("status").is_some());

        // Optional fields should not be present if None
        if request.location.is_none() {
            assert!(json.get("location").is_none());
        }
        if request.request_id.is_none() {
            assert!(json.get("requestId").is_none());
        }
        if request.status_info.is_none() {
            assert!(json.get("statusInfo").is_none());
        }
        if request.custom_data.is_none() {
            assert!(json.get("customData").is_none());
        }
    }

    #[test]
    fn test_publish_firmware_status_notification_request_multiple_locations() {
        let locations = vec![
            "https://example.com/firmware.bin".to_string(),
            "http://backup.example.com/firmware.bin".to_string(),
            "ftp://files.example.com/firmware.bin".to_string(),
        ];

        let request = create_test_publish_firmware_status_notification_request()
            .with_location(locations.clone());

        assert_eq!(request.location, Some(locations));
        assert!(request.validate().is_ok());

        // Test serialization preserves all locations
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: PublishFirmwareStatusNotificationRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_publish_firmware_status_notification_response_new() {
        let response = PublishFirmwareStatusNotificationResponse::new();
        assert!(response.custom_data.is_none());
    }

    #[test]
    fn test_publish_firmware_status_notification_response_serialization() {
        let response = create_test_publish_firmware_status_notification_response();

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: PublishFirmwareStatusNotificationResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_publish_firmware_status_notification_response_validation_valid() {
        let response = create_test_publish_firmware_status_notification_response();
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_publish_firmware_status_notification_response_set_methods() {
        let mut response = create_test_publish_firmware_status_notification_response();
        let custom_data = create_test_custom_data();

        response.set_custom_data(Some(custom_data.clone()));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_publish_firmware_status_notification_response_get_methods() {
        let response = create_test_publish_firmware_status_notification_response();
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_publish_firmware_status_notification_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = create_test_publish_firmware_status_notification_response()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.custom_data, Some(custom_data));
        assert_eq!(response.get_custom_data(), response.custom_data.as_ref());
    }

    #[test]
    fn test_publish_firmware_status_notification_response_json_format() {
        let response = create_test_publish_firmware_status_notification_response();
        let json = serde_json::to_value(&response).expect("Failed to serialize to JSON");

        // Custom data should not be present if None
        if response.custom_data.is_none() {
            assert!(json.get("customData").is_none());
        }
    }
}