use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::FirmwareStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the FirmwareStatusNotification request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationRequest {
    pub status: FirmwareStatusEnumType,

    /// The request id that was provided in the UpdateFirmwareRequest that started this firmware update. This field is mandatory, unless the message was triggered by a TriggerMessageRequest AND there is no firmware update ongoing.
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

impl FirmwareStatusNotificationRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: FirmwareStatusEnumType) -> Self {
        Self {
            status,
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
    pub fn set_status(&mut self, status: FirmwareStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - The request id that was provided in the UpdateFirmwareRequest that started this firmware update. This field is mandatory, unless the message was triggered by a TriggerMessageRequest AND there is no firmware update ongoing.
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
    pub fn get_status(&self) -> &FirmwareStatusEnumType {
        &self.status
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// The request id that was provided in the UpdateFirmwareRequest that started this firmware update. This field is mandatory, unless the message was triggered by a TriggerMessageRequest AND there is no firmware update ongoing.
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

    /// Sets the request_id field and returns self for builder pattern.
    ///
    /// * `request_id` - The request id that was provided in the UpdateFirmwareRequest that started this firmware update. This field is mandatory, unless the message was triggered by a TriggerMessageRequest AND there is no firmware update ongoing.
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

/// Response body for the FirmwareStatusNotification response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl FirmwareStatusNotificationResponse {
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

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("TestReason".to_string())
    }

    // Tests for FirmwareStatusNotificationRequest

    #[test]
    fn test_firmware_status_notification_request_new() {
        let request = FirmwareStatusNotificationRequest::new(FirmwareStatusEnumType::Downloading);

        assert_eq!(request.status, FirmwareStatusEnumType::Downloading);
        assert_eq!(request.request_id, None);
        assert_eq!(request.status_info, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_firmware_status_notification_request_with_request_id() {
        let request = FirmwareStatusNotificationRequest::new(FirmwareStatusEnumType::Downloaded)
            .with_request_id(12345);

        assert_eq!(request.status, FirmwareStatusEnumType::Downloaded);
        assert_eq!(request.request_id, Some(12345));
        assert_eq!(request.status_info, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_firmware_status_notification_request_with_status_info() {
        let status_info = create_test_status_info();
        let request = FirmwareStatusNotificationRequest::new(FirmwareStatusEnumType::InstallationFailed)
            .with_status_info(status_info.clone());

        assert_eq!(request.status, FirmwareStatusEnumType::InstallationFailed);
        assert_eq!(request.request_id, None);
        assert_eq!(request.status_info, Some(status_info));
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_firmware_status_notification_request_with_custom_data() {
        let custom_data = create_test_custom_data();
        let request = FirmwareStatusNotificationRequest::new(FirmwareStatusEnumType::Installed)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.status, FirmwareStatusEnumType::Installed);
        assert_eq!(request.request_id, None);
        assert_eq!(request.status_info, None);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_firmware_status_notification_request_setters() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let mut request = FirmwareStatusNotificationRequest::new(FirmwareStatusEnumType::Idle);
        request.set_status(FirmwareStatusEnumType::Installing);
        request.set_request_id(Some(54321));
        request.set_status_info(Some(status_info.clone()));
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.status, FirmwareStatusEnumType::Installing);
        assert_eq!(request.request_id, Some(54321));
        assert_eq!(request.status_info, Some(status_info));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_firmware_status_notification_request_getters() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();
        let request = FirmwareStatusNotificationRequest::new(FirmwareStatusEnumType::SignatureVerified)
            .with_request_id(99999)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_status(), &FirmwareStatusEnumType::SignatureVerified);
        assert_eq!(request.get_request_id(), Some(&99999));
        assert_eq!(request.get_status_info(), Some(&status_info));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_firmware_status_notification_request_serialization() {
        let request = FirmwareStatusNotificationRequest::new(FirmwareStatusEnumType::DownloadFailed)
            .with_request_id(123);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: FirmwareStatusNotificationRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
    }

    #[test]
    fn test_firmware_status_notification_request_deserialization() {
        let json = r#"{
            "status": "Downloading",
            "requestId": 456,
            "statusInfo": {
                "reasonCode": "ProgressUpdate"
            }
        }"#;

        let request: FirmwareStatusNotificationRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.status, FirmwareStatusEnumType::Downloading);
        assert_eq!(request.request_id, Some(456));
        assert!(request.status_info.is_some());
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_firmware_status_notification_request_validation() {
        let request = FirmwareStatusNotificationRequest::new(FirmwareStatusEnumType::Downloaded)
            .with_request_id(100);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_firmware_status_notification_request_validation_negative_request_id() {
        let mut request = FirmwareStatusNotificationRequest::new(FirmwareStatusEnumType::Downloaded);
        request.set_request_id(Some(-1));

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_firmware_status_notification_request_all_status_types() {
        let statuses = vec![
            FirmwareStatusEnumType::Downloaded,
            FirmwareStatusEnumType::DownloadFailed,
            FirmwareStatusEnumType::Downloading,
            FirmwareStatusEnumType::DownloadScheduled,
            FirmwareStatusEnumType::DownloadPaused,
            FirmwareStatusEnumType::Idle,
            FirmwareStatusEnumType::InstallationFailed,
            FirmwareStatusEnumType::Installing,
            FirmwareStatusEnumType::Installed,
            FirmwareStatusEnumType::InstallRebooting,
            FirmwareStatusEnumType::InstallScheduled,
            FirmwareStatusEnumType::InstallVerificationFailed,
            FirmwareStatusEnumType::InvalidSignature,
            FirmwareStatusEnumType::SignatureVerified,
        ];

        for status in statuses {
            let request = FirmwareStatusNotificationRequest::new(status.clone());
            assert_eq!(request.status, status);
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_firmware_status_notification_request_json_round_trip() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();
        let request = FirmwareStatusNotificationRequest::new(FirmwareStatusEnumType::Installing)
            .with_request_id(789)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: FirmwareStatusNotificationRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
        assert!(parsed.validate().is_ok());
    }

    // Tests for FirmwareStatusNotificationResponse

    #[test]
    fn test_firmware_status_notification_response_new() {
        let response = FirmwareStatusNotificationResponse::new();

        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_firmware_status_notification_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = FirmwareStatusNotificationResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_firmware_status_notification_response_setters() {
        let custom_data = create_test_custom_data();

        let mut response = FirmwareStatusNotificationResponse::new();
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_firmware_status_notification_response_getters() {
        let custom_data = create_test_custom_data();
        let response = FirmwareStatusNotificationResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_firmware_status_notification_response_serialization() {
        let response = FirmwareStatusNotificationResponse::new();

        let json = serde_json::to_string(&response).unwrap();
        let parsed: FirmwareStatusNotificationResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
    }

    #[test]
    fn test_firmware_status_notification_response_deserialization() {
        let json = r#"{
            "customData": {
                "vendorId": "TestVendor"
            }
        }"#;

        let response: FirmwareStatusNotificationResponse = serde_json::from_str(json).unwrap();
        assert!(response.custom_data.is_some());
    }

    #[test]
    fn test_firmware_status_notification_response_validation() {
        let response = FirmwareStatusNotificationResponse::new();

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_firmware_status_notification_response_json_round_trip() {
        let custom_data = create_test_custom_data();
        let response = FirmwareStatusNotificationResponse::new()
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: FirmwareStatusNotificationResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
        assert!(parsed.validate().is_ok());
    }

    #[test]
    fn test_firmware_status_notification_response_empty_json() {
        let json = "{}";

        let response: FirmwareStatusNotificationResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.custom_data, None);
        assert!(response.validate().is_ok());
    }
}