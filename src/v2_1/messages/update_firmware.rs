use crate::v2_1::datatypes::{CustomDataType, FirmwareType, StatusInfoType};
use crate::v2_1::enumerations::UpdateFirmwareStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the UpdateFirmware request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareRequest {
    /// This specifies how many times Charging Station must retry to download the firmware before giving up. If this field is not present, it is left to Charging Station to decide how many times it wants to retry. If the value is 0, it means: no retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub retries: Option<i32>,

    /// The interval in seconds after which a retry may be attempted. If this field is not present, it is left to Charging Station to decide how long to wait between attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,

    /// The Id of this request
    #[validate(range(min = 0))]
    pub request_id: i32,

    #[validate(nested)]
    pub firmware: FirmwareType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl UpdateFirmwareRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `request_id` - The Id of this request
    /// * `firmware` - The firmware field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(request_id: i32, firmware: FirmwareType) -> Self {
        Self {
            retries: None,
            retry_interval: None,
            request_id,
            firmware,
            custom_data: None,
        }
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

    /// Sets the firmware field.
    ///
    /// * `firmware` - The firmware field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_firmware(&mut self, firmware: FirmwareType) -> &mut Self {
        self.firmware = firmware;
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

    /// Gets a reference to the retries field.
    ///
    /// # Returns
    ///
    /// This specifies how many times Charging Station must retry to download the firmware before giving up. If this field is not present, it is left to Charging Station to decide how many times it wants to retry. If the value is 0, it means: no retries.
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

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// The Id of this request
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the firmware field.
    ///
    /// # Returns
    ///
    /// The firmware field
    pub fn get_firmware(&self) -> &FirmwareType {
        &self.firmware
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

/// Response body for the UpdateFirmware response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareResponse {
    pub status: UpdateFirmwareStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl UpdateFirmwareResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: UpdateFirmwareStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: UpdateFirmwareStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &UpdateFirmwareStatusEnumType {
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
    use crate::v2_1::datatypes::{CustomDataType, FirmwareType, StatusInfoType};
    use crate::v2_1::enumerations::UpdateFirmwareStatusEnumType;
    use serde_json;
    use validator::Validate;

    fn create_test_firmware() -> FirmwareType {
        FirmwareType::new(
            "https://example.com/firmware.bin".to_string(),
            "abcd1234efgh5678".to_string(),
        )
    }

    // Tests for UpdateFirmwareRequest

    #[test]
    fn test_update_firmware_request_new() {
        let request_id = 123;
        let firmware = create_test_firmware();
        let request = UpdateFirmwareRequest::new(request_id, firmware.clone());

        assert_eq!(request.get_request_id(), &request_id);
        assert_eq!(request.get_firmware(), &firmware);
        assert_eq!(request.get_retries(), None);
        assert_eq!(request.get_retry_interval(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_update_firmware_request_serialization() {
        let firmware = create_test_firmware();
        let request = UpdateFirmwareRequest::new(456, firmware);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: UpdateFirmwareRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_update_firmware_request_validation() {
        let firmware = create_test_firmware();
        let request = UpdateFirmwareRequest::new(789, firmware);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_firmware_request_with_retries() {
        let firmware = create_test_firmware();
        let request = UpdateFirmwareRequest::new(101, firmware)
            .with_retries(3);

        assert_eq!(request.get_retries(), Some(&3));
    }

    #[test]
    fn test_update_firmware_request_with_retry_interval() {
        let firmware = create_test_firmware();
        let request = UpdateFirmwareRequest::new(202, firmware)
            .with_retry_interval(60);

        assert_eq!(request.get_retry_interval(), Some(&60));
    }

    #[test]
    fn test_update_firmware_request_with_custom_data() {
        let firmware = create_test_firmware();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = UpdateFirmwareRequest::new(303, firmware)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_update_firmware_request_set_methods() {
        let firmware = create_test_firmware();
        let new_firmware = create_test_firmware();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = UpdateFirmwareRequest::new(100, firmware);

        request
            .set_request_id(200)
            .set_firmware(new_firmware.clone())
            .set_retries(Some(5))
            .set_retry_interval(Some(120))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_request_id(), &200);
        assert_eq!(request.get_firmware(), &new_firmware);
        assert_eq!(request.get_retries(), Some(&5));
        assert_eq!(request.get_retry_interval(), Some(&120));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_update_firmware_request_builder_pattern() {
        let firmware = create_test_firmware();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = UpdateFirmwareRequest::new(404, firmware)
            .with_retries(2)
            .with_retry_interval(30)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_retries(), Some(&2));
        assert_eq!(request.get_retry_interval(), Some(&30));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_update_firmware_request_negative_request_id_validation() {
        let firmware = create_test_firmware();
        let mut request = UpdateFirmwareRequest::new(1, firmware);
        request.set_request_id(-1);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_update_firmware_request_negative_retries_validation() {
        let firmware = create_test_firmware();
        let mut request = UpdateFirmwareRequest::new(1, firmware);
        request.set_retries(Some(-1));

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_update_firmware_request_zero_retries() {
        let firmware = create_test_firmware();
        let request = UpdateFirmwareRequest::new(1, firmware)
            .with_retries(0);

        assert_eq!(request.get_retries(), Some(&0));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_firmware_request_zero_request_id_validation() {
        let firmware = create_test_firmware();
        let request = UpdateFirmwareRequest::new(0, firmware);

        assert!(request.validate().is_ok());
    }

    // Tests for UpdateFirmwareResponse

    #[test]
    fn test_update_firmware_response_new() {
        let status = UpdateFirmwareStatusEnumType::Accepted;
        let response = UpdateFirmwareResponse::new(status.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_update_firmware_response_serialization() {
        let response = UpdateFirmwareResponse::new(UpdateFirmwareStatusEnumType::Rejected);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: UpdateFirmwareResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_update_firmware_response_validation() {
        let response = UpdateFirmwareResponse::new(UpdateFirmwareStatusEnumType::Accepted);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_update_firmware_response_with_status_info() {
        let status_info = StatusInfoType::new("Success".to_string());
        let response = UpdateFirmwareResponse::new(UpdateFirmwareStatusEnumType::Accepted)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
    }

    #[test]
    fn test_update_firmware_response_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = UpdateFirmwareResponse::new(UpdateFirmwareStatusEnumType::InvalidCertificate)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_update_firmware_response_set_methods() {
        let status_info = StatusInfoType::new("Error".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = UpdateFirmwareResponse::new(UpdateFirmwareStatusEnumType::Accepted);

        response
            .set_status(UpdateFirmwareStatusEnumType::RevokedCertificate)
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_status(), &UpdateFirmwareStatusEnumType::RevokedCertificate);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_update_firmware_response_builder_pattern() {
        let status_info = StatusInfoType::new("Info".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = UpdateFirmwareResponse::new(UpdateFirmwareStatusEnumType::AcceptedCanceled)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_update_firmware_response_all_status_types() {
        let status_types = vec![
            UpdateFirmwareStatusEnumType::Accepted,
            UpdateFirmwareStatusEnumType::Rejected,
            UpdateFirmwareStatusEnumType::AcceptedCanceled,
            UpdateFirmwareStatusEnumType::InvalidCertificate,
            UpdateFirmwareStatusEnumType::RevokedCertificate,
        ];

        for status in status_types {
            let response = UpdateFirmwareResponse::new(status.clone());
            
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());

            let json = serde_json::to_string(&response).expect("Failed to serialize");
            let deserialized: UpdateFirmwareResponse = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(response, deserialized);
        }
    }

    #[test]
    fn test_update_firmware_request_json_round_trip() {
        let firmware = create_test_firmware();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = UpdateFirmwareRequest::new(555, firmware)
            .with_retries(3)
            .with_retry_interval(180)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: UpdateFirmwareRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_update_firmware_response_json_round_trip() {
        let status_info = StatusInfoType::new("Details".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = UpdateFirmwareResponse::new(UpdateFirmwareStatusEnumType::InvalidCertificate)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: UpdateFirmwareResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_update_firmware_request_with_large_values() {
        let firmware = create_test_firmware();
        let request = UpdateFirmwareRequest::new(999999, firmware)
            .with_retries(100)
            .with_retry_interval(3600);

        assert_eq!(request.get_request_id(), &999999);
        assert_eq!(request.get_retries(), Some(&100));
        assert_eq!(request.get_retry_interval(), Some(&3600));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_firmware_response_with_all_optional_fields() {
        let status_info = StatusInfoType::new("Complete".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = UpdateFirmwareResponse::new(UpdateFirmwareStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &UpdateFirmwareStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_update_firmware_request_with_custom_data_validation() {
        let firmware = create_test_firmware();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = UpdateFirmwareRequest::new(777, firmware)
            .with_custom_data(custom_data);

        assert!(request.validate().is_ok());
    }
}
