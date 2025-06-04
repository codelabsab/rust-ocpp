use crate::v2_1::datatypes::CustomDataType;
use crate::v2_1::enumerations::UnpublishFirmwareStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the UnpublishFirmware request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UnpublishFirmwareRequest {
    /// The MD5 checksum over the entire firmware file as a hexadecimal string of length 32.
    #[validate(length(max = 32))]
    pub checksum: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl UnpublishFirmwareRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `checksum` - The MD5 checksum over the entire firmware file as a hexadecimal string of length 32.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(checksum: String) -> Self {
        Self {
            checksum,
            custom_data: None,
        }
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

    /// Gets a reference to the checksum field.
    ///
    /// # Returns
    ///
    /// The MD5 checksum over the entire firmware file as a hexadecimal string of length 32.
    pub fn get_checksum(&self) -> &String {
        &self.checksum
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

/// Response body for the UnpublishFirmware response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UnpublishFirmwareResponse {
    pub status: UnpublishFirmwareStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl UnpublishFirmwareResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: UnpublishFirmwareStatusEnumType) -> Self {
        Self {
            status,
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
    pub fn set_status(&mut self, status: UnpublishFirmwareStatusEnumType) -> &mut Self {
        self.status = status;
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
    pub fn get_status(&self) -> &UnpublishFirmwareStatusEnumType {
        &self.status
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
    use crate::v2_1::enumerations::UnpublishFirmwareStatusEnumType;
    use serde_json;
    use validator::Validate;

    // Tests for UnpublishFirmwareRequest

    #[test]
    fn test_unpublish_firmware_request_new() {
        let checksum = "a1b2c3d4e5f67890abcdef1234567890".to_string();
        let request = UnpublishFirmwareRequest::new(checksum.clone());

        assert_eq!(request.get_checksum(), &checksum);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_unpublish_firmware_request_serialization() {
        let checksum = "12345678901234567890123456789012".to_string();
        let request = UnpublishFirmwareRequest::new(checksum);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: UnpublishFirmwareRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_unpublish_firmware_request_validation() {
        let checksum = "abcd1234efgh5678ijkl9012mnop3456".to_string();
        let request = UnpublishFirmwareRequest::new(checksum);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_unpublish_firmware_request_with_custom_data() {
        let checksum = "1a2b3c4d5e6f7890abcdef1234567890".to_string();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = UnpublishFirmwareRequest::new(checksum)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_unpublish_firmware_request_set_methods() {
        let checksum = "original1234567890123456789012".to_string();
        let new_checksum = "updated567890123456789012345678".to_string();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = UnpublishFirmwareRequest::new(checksum);

        request
            .set_checksum(new_checksum.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_checksum(), &new_checksum);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_unpublish_firmware_request_builder_pattern() {
        let checksum = "build1234567890123456789012345".to_string();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = UnpublishFirmwareRequest::new(checksum)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_unpublish_firmware_request_checksum_validation() {
        let valid_checksum = "a".repeat(32); // Max allowed length
        let request = UnpublishFirmwareRequest::new(valid_checksum);

        assert!(request.validate().is_ok());

        let too_long_checksum = "a".repeat(33); // Exceeds max length
        let invalid_request = UnpublishFirmwareRequest::new(too_long_checksum);

        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_unpublish_firmware_request_short_checksum() {
        let short_checksum = "abc123".to_string(); // Valid but short
        let request = UnpublishFirmwareRequest::new(short_checksum.clone());

        assert_eq!(request.get_checksum(), &short_checksum);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_unpublish_firmware_request_empty_checksum() {
        let empty_checksum = "".to_string();
        let request = UnpublishFirmwareRequest::new(empty_checksum);

        assert!(request.validate().is_ok()); // No minimum length constraint
    }

    // Tests for UnpublishFirmwareResponse

    #[test]
    fn test_unpublish_firmware_response_new() {
        let status = UnpublishFirmwareStatusEnumType::Unpublished;
        let response = UnpublishFirmwareResponse::new(status.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_unpublish_firmware_response_serialization() {
        let response = UnpublishFirmwareResponse::new(UnpublishFirmwareStatusEnumType::NoFirmware);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: UnpublishFirmwareResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_unpublish_firmware_response_validation() {
        let response = UnpublishFirmwareResponse::new(UnpublishFirmwareStatusEnumType::DownloadOngoing);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_unpublish_firmware_response_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = UnpublishFirmwareResponse::new(UnpublishFirmwareStatusEnumType::Unpublished)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_unpublish_firmware_response_set_methods() {
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = UnpublishFirmwareResponse::new(UnpublishFirmwareStatusEnumType::Unpublished);

        response
            .set_status(UnpublishFirmwareStatusEnumType::NoFirmware)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_status(), &UnpublishFirmwareStatusEnumType::NoFirmware);
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_unpublish_firmware_response_builder_pattern() {
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = UnpublishFirmwareResponse::new(UnpublishFirmwareStatusEnumType::DownloadOngoing)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_unpublish_firmware_response_all_status_types() {
        let status_types = vec![
            UnpublishFirmwareStatusEnumType::DownloadOngoing,
            UnpublishFirmwareStatusEnumType::NoFirmware,
            UnpublishFirmwareStatusEnumType::Unpublished,
        ];

        for status in status_types {
            let response = UnpublishFirmwareResponse::new(status.clone());
            
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());

            let json = serde_json::to_string(&response).expect("Failed to serialize");
            let deserialized: UnpublishFirmwareResponse = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(response, deserialized);
        }
    }

    #[test]
    fn test_unpublish_firmware_request_json_round_trip() {
        let checksum = "deadbeef12345678abcdef9876543210".to_string();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = UnpublishFirmwareRequest::new(checksum)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: UnpublishFirmwareRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_unpublish_firmware_response_json_round_trip() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = UnpublishFirmwareResponse::new(UnpublishFirmwareStatusEnumType::Unpublished)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: UnpublishFirmwareResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_unpublish_firmware_request_with_hex_checksum() {
        let hex_checksum = "0123456789abcdef0123456789abcdef".to_string();
        let request = UnpublishFirmwareRequest::new(hex_checksum.clone());

        assert_eq!(request.get_checksum(), &hex_checksum);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_unpublish_firmware_request_with_custom_data_validation() {
        let checksum = "testchecksum1234567890123456789".to_string();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = UnpublishFirmwareRequest::new(checksum)
            .with_custom_data(custom_data);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_unpublish_firmware_response_with_all_optional_fields() {
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = UnpublishFirmwareResponse::new(UnpublishFirmwareStatusEnumType::Unpublished)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &UnpublishFirmwareStatusEnumType::Unpublished);
        assert_eq!(response.get_custom_data(), Some(&custom_data));
        assert!(response.validate().is_ok());
    }
}