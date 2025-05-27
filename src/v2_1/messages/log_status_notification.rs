use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::UploadLogStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the LogStatusNotification request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LogStatusNotificationRequest {
    pub status: UploadLogStatusEnumType,

    /// The request id that was provided in GetLogRequest that started this log upload. This field is mandatory, unless the message was triggered by a TriggerMessageRequest AND there is no log upload ongoing.
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

impl LogStatusNotificationRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: UploadLogStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: UploadLogStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - The request id that was provided in GetLogRequest that started this log upload. This field is mandatory, unless the message was triggered by a TriggerMessageRequest AND there is no log upload ongoing.
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
    pub fn get_status(&self) -> &UploadLogStatusEnumType {
        &self.status
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// The request id that was provided in GetLogRequest that started this log upload. This field is mandatory, unless the message was triggered by a TriggerMessageRequest AND there is no log upload ongoing.
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
    /// * `request_id` - The request id that was provided in GetLogRequest that started this log upload. This field is mandatory, unless the message was triggered by a TriggerMessageRequest AND there is no log upload ongoing.
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

/// Response body for the LogStatusNotification response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LogStatusNotificationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl LogStatusNotificationResponse {
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

    // Tests for LogStatusNotificationRequest

    #[test]
    fn test_log_status_notification_request_new() {
        let status = UploadLogStatusEnumType::Uploaded;
        let request = LogStatusNotificationRequest::new(status.clone());

        assert_eq!(request.status, status);
        assert_eq!(request.request_id, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_log_status_notification_request_with_request_id() {
        let status = UploadLogStatusEnumType::UploadFailure;
        let request = LogStatusNotificationRequest::new(status.clone())
            .with_request_id(123);

        assert_eq!(request.status, status);
        assert_eq!(request.request_id, Some(123));
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_log_status_notification_request_with_custom_data() {
        let status = UploadLogStatusEnumType::Idle;
        let custom_data = create_test_custom_data();
        let request = LogStatusNotificationRequest::new(status.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.status, status);
        assert_eq!(request.request_id, None);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_log_status_notification_request_setters() {
        let status1 = UploadLogStatusEnumType::Uploaded;
        let status2 = UploadLogStatusEnumType::Uploading;
        let custom_data = create_test_custom_data();

        let mut request = LogStatusNotificationRequest::new(status1);
        request.set_status(status2.clone());
        request.set_request_id(Some(456));
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.status, status2);
        assert_eq!(request.request_id, Some(456));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_log_status_notification_request_getters() {
        let status = UploadLogStatusEnumType::NotSupportedOperation;
        let custom_data = create_test_custom_data();
        let request = LogStatusNotificationRequest::new(status.clone())
            .with_request_id(789)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_status(), &status);
        assert_eq!(request.get_request_id(), Some(&789));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_log_status_notification_request_serialization() {
        let status = UploadLogStatusEnumType::Uploaded;
        let request = LogStatusNotificationRequest::new(status);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: LogStatusNotificationRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
    }

    #[test]
    fn test_log_status_notification_request_validation() {
        let status = UploadLogStatusEnumType::Uploaded;
        let request = LogStatusNotificationRequest::new(status);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_log_status_notification_request_validation_negative_request_id() {
        let status = UploadLogStatusEnumType::Uploaded;
        let mut request = LogStatusNotificationRequest::new(status);
        request.set_request_id(Some(-1));

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_log_status_notification_request_all_status_types() {
        let statuses = vec![
            UploadLogStatusEnumType::BadMessage,
            UploadLogStatusEnumType::Idle,
            UploadLogStatusEnumType::NotSupportedOperation,
            UploadLogStatusEnumType::PermissionDenied,
            UploadLogStatusEnumType::Uploaded,
            UploadLogStatusEnumType::UploadFailure,
            UploadLogStatusEnumType::Uploading,
            UploadLogStatusEnumType::AcceptedCanceled,
        ];

        for status in statuses {
            let request = LogStatusNotificationRequest::new(status.clone());
            assert_eq!(request.status, status);
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_log_status_notification_request_json_round_trip() {
        let status = UploadLogStatusEnumType::Uploading;
        let custom_data = create_test_custom_data();
        let request = LogStatusNotificationRequest::new(status)
            .with_request_id(999)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: LogStatusNotificationRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
        assert!(parsed.validate().is_ok());
    }

    // Tests for LogStatusNotificationResponse

    #[test]
    fn test_log_status_notification_response_new() {
        let response = LogStatusNotificationResponse::new();

        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_log_status_notification_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = LogStatusNotificationResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_log_status_notification_response_setters() {
        let custom_data = create_test_custom_data();

        let mut response = LogStatusNotificationResponse::new();
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_log_status_notification_response_getters() {
        let custom_data = create_test_custom_data();
        let response = LogStatusNotificationResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_log_status_notification_response_serialization() {
        let response = LogStatusNotificationResponse::new();

        let json = serde_json::to_string(&response).unwrap();
        let parsed: LogStatusNotificationResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
    }

    #[test]
    fn test_log_status_notification_response_validation() {
        let response = LogStatusNotificationResponse::new();

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_log_status_notification_response_json_round_trip() {
        let custom_data = create_test_custom_data();
        let response = LogStatusNotificationResponse::new()
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: LogStatusNotificationResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
        assert!(parsed.validate().is_ok());
    }
}