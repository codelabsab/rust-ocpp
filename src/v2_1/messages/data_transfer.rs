use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::DataTransferStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the DataTransfer request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferRequest {
    /// May be used to indicate a specific message or implementation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub message_id: Option<String>,

    /// Data without specified length or format. This needs to be decided by both parties (Open to implementation).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub data: Option<String>,

    /// This identifies the Vendor specific implementation
    #[validate(length(max = 255))]
    pub vendor_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl DataTransferRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `vendor_id` - This identifies the Vendor specific implementation
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(vendor_id: String) -> Self {
        Self {
            message_id: None,
            data: None,
            vendor_id,
            custom_data: None,
        }
    }

    /// Sets the message_id field.
    ///
    /// * `message_id` - May be used to indicate a specific message or implementation.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_message_id(&mut self, message_id: Option<String>) -> &mut Self {
        self.message_id = message_id;
        self
    }

    /// Sets the data field.
    ///
    /// * `data` - Data without specified length or format. This needs to be decided by both parties (Open to implementation).
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_data(&mut self, data: Option<String>) -> &mut Self {
        self.data = data;
        self
    }

    /// Sets the vendor_id field.
    ///
    /// * `vendor_id` - This identifies the Vendor specific implementation
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_vendor_id(&mut self, vendor_id: String) -> &mut Self {
        self.vendor_id = vendor_id;
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

    /// Gets a reference to the message_id field.
    ///
    /// # Returns
    ///
    /// May be used to indicate a specific message or implementation.
    pub fn get_message_id(&self) -> Option<&String> {
        self.message_id.as_ref()
    }

    /// Gets a reference to the data field.
    ///
    /// # Returns
    ///
    /// Data without specified length or format. This needs to be decided by both parties (Open to implementation).
    pub fn get_data(&self) -> Option<&String> {
        self.data.as_ref()
    }

    /// Gets a reference to the vendor_id field.
    ///
    /// # Returns
    ///
    /// This identifies the Vendor specific implementation
    pub fn get_vendor_id(&self) -> &String {
        &self.vendor_id
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the message_id field and returns self for builder pattern.
    ///
    /// * `message_id` - May be used to indicate a specific message or implementation.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_message_id(mut self, message_id: String) -> Self {
        self.message_id = Some(message_id);
        self
    }

    /// Sets the data field and returns self for builder pattern.
    ///
    /// * `data` - Data without specified length or format. This needs to be decided by both parties (Open to implementation).
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_data(mut self, data: String) -> Self {
        self.data = Some(data);
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

/// Response body for the DataTransfer response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferResponse {
    pub status: DataTransferStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// Data without specified length or format, in response to request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl DataTransferResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: DataTransferStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
            data: None,
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
    pub fn set_status(&mut self, status: DataTransferStatusEnumType) -> &mut Self {
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

    /// Sets the data field.
    ///
    /// * `data` - Data without specified length or format, in response to request.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_data(&mut self, data: Option<String>) -> &mut Self {
        self.data = data;
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
    pub fn get_status(&self) -> &DataTransferStatusEnumType {
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

    /// Gets a reference to the data field.
    ///
    /// # Returns
    ///
    /// Data without specified length or format, in response to request.
    pub fn get_data(&self) -> Option<&String> {
        self.data.as_ref()
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

    /// Sets the data field and returns self for builder pattern.
    ///
    /// * `data` - Data without specified length or format, in response to request.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_data(mut self, data: String) -> Self {
        self.data = Some(data);
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

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("TestReason".to_string())
    }

    #[test]
    fn test_data_transfer_request_new() {
        let vendor_id = "TestVendor123".to_string();

        let request = DataTransferRequest::new(vendor_id.clone());

        assert_eq!(request.vendor_id, vendor_id);
        assert_eq!(request.message_id, None);
        assert_eq!(request.data, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_data_transfer_request_with_message_id() {
        let vendor_id = "TestVendor123".to_string();
        let message_id = "msg_001".to_string();

        let request = DataTransferRequest::new(vendor_id.clone())
            .with_message_id(message_id.clone());

        assert_eq!(request.vendor_id, vendor_id);
        assert_eq!(request.message_id, Some(message_id));
        assert_eq!(request.data, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_data_transfer_request_with_data() {
        let vendor_id = "TestVendor123".to_string();
        let data = "test_data_payload".to_string();

        let request = DataTransferRequest::new(vendor_id.clone())
            .with_data(data.clone());

        assert_eq!(request.vendor_id, vendor_id);
        assert_eq!(request.message_id, None);
        assert_eq!(request.data, Some(data));
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_data_transfer_request_with_custom_data() {
        let vendor_id = "TestVendor123".to_string();
        let custom_data = create_test_custom_data();

        let request = DataTransferRequest::new(vendor_id.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.vendor_id, vendor_id);
        assert_eq!(request.message_id, None);
        assert_eq!(request.data, None);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_data_transfer_request_setters() {
        let mut request = DataTransferRequest::new("InitialVendor".to_string());

        let new_vendor_id = "NewVendor456".to_string();
        let message_id = "msg_002".to_string();
        let data = "new_data_payload".to_string();
        let custom_data = create_test_custom_data();

        request.set_vendor_id(new_vendor_id.clone())
               .set_message_id(Some(message_id.clone()))
               .set_data(Some(data.clone()))
               .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.vendor_id, new_vendor_id);
        assert_eq!(request.message_id, Some(message_id));
        assert_eq!(request.data, Some(data));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_data_transfer_request_getters() {
        let vendor_id = "TestVendor789".to_string();
        let message_id = "msg_003".to_string();
        let data = "getter_test_data".to_string();
        let custom_data = create_test_custom_data();

        let request = DataTransferRequest::new(vendor_id.clone())
            .with_message_id(message_id.clone())
            .with_data(data.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_vendor_id(), &vendor_id);
        assert_eq!(request.get_message_id(), Some(&message_id));
        assert_eq!(request.get_data(), Some(&data));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_data_transfer_request_serialization() {
        let vendor_id = "SerializationVendor".to_string();
        let message_id = "msg_serialize".to_string();

        let request = DataTransferRequest::new(vendor_id)
            .with_message_id(message_id);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: DataTransferRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_data_transfer_request_validation_vendor_id_too_long() {
        let long_vendor_id = "a".repeat(256); // Max is 255

        let request = DataTransferRequest::new(long_vendor_id);

        let validation_result = request.validate();
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_data_transfer_request_validation_message_id_too_long() {
        let vendor_id = "ValidVendor".to_string();
        let long_message_id = "a".repeat(51); // Max is 50

        let request = DataTransferRequest::new(vendor_id)
            .with_message_id(long_message_id);

        let validation_result = request.validate();
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_data_transfer_request_validation_data_too_long() {
        let vendor_id = "ValidVendor".to_string();
        let long_data = "a".repeat(256); // Max is 255

        let request = DataTransferRequest::new(vendor_id)
            .with_data(long_data);

        let validation_result = request.validate();
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_data_transfer_request_validation_valid() {
        let vendor_id = "a".repeat(255); // Max length
        let message_id = "a".repeat(50); // Max length
        let data = "a".repeat(255); // Max length

        let request = DataTransferRequest::new(vendor_id)
            .with_message_id(message_id)
            .with_data(data);

        let validation_result = request.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_data_transfer_response_new() {
        let status = DataTransferStatusEnumType::Accepted;

        let response = DataTransferResponse::new(status.clone());

        assert_eq!(response.status, status);
        assert_eq!(response.status_info, None);
        assert_eq!(response.data, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_data_transfer_response_with_status_info() {
        let status = DataTransferStatusEnumType::Rejected;
        let status_info = create_test_status_info();

        let response = DataTransferResponse::new(status.clone())
            .with_status_info(status_info.clone());

        assert_eq!(response.status, status);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.data, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_data_transfer_response_with_data() {
        let status = DataTransferStatusEnumType::Accepted;
        let data = "response_data_payload".to_string();

        let response = DataTransferResponse::new(status.clone())
            .with_data(data.clone());

        assert_eq!(response.status, status);
        assert_eq!(response.status_info, None);
        assert_eq!(response.data, Some(data));
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_data_transfer_response_with_custom_data() {
        let status = DataTransferStatusEnumType::UnknownMessageId;
        let custom_data = create_test_custom_data();

        let response = DataTransferResponse::new(status.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, status);
        assert_eq!(response.status_info, None);
        assert_eq!(response.data, None);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_data_transfer_response_setters() {
        let mut response = DataTransferResponse::new(DataTransferStatusEnumType::Accepted);

        let new_status = DataTransferStatusEnumType::UnknownVendorId;
        let status_info = create_test_status_info();
        let data = "setter_test_data".to_string();
        let custom_data = create_test_custom_data();

        response.set_status(new_status.clone())
                .set_status_info(Some(status_info.clone()))
                .set_data(Some(data.clone()))
                .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, new_status);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.data, Some(data));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_data_transfer_response_getters() {
        let status = DataTransferStatusEnumType::Rejected;
        let status_info = create_test_status_info();
        let data = "getter_response_data".to_string();
        let custom_data = create_test_custom_data();

        let response = DataTransferResponse::new(status.clone())
            .with_status_info(status_info.clone())
            .with_data(data.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_data(), Some(&data));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_data_transfer_response_serialization() {
        let status = DataTransferStatusEnumType::UnknownMessageId;
        let data = "serialization_test_data".to_string();

        let response = DataTransferResponse::new(status)
            .with_data(data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: DataTransferResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_data_transfer_response_validation_data_too_long() {
        let status = DataTransferStatusEnumType::Accepted;
        let long_data = "a".repeat(256); // Max is 255

        let response = DataTransferResponse::new(status)
            .with_data(long_data);

        let validation_result = response.validate();
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_data_transfer_response_validation_valid() {
        let status = DataTransferStatusEnumType::Accepted;
        let data = "a".repeat(255); // Max length

        let response = DataTransferResponse::new(status)
            .with_data(data);

        let validation_result = response.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_data_transfer_response_all_status_variants() {
        let statuses = vec![
            DataTransferStatusEnumType::Accepted,
            DataTransferStatusEnumType::Rejected,
            DataTransferStatusEnumType::UnknownMessageId,
            DataTransferStatusEnumType::UnknownVendorId,
        ];

        for status in statuses {
            let response = DataTransferResponse::new(status.clone());
            assert_eq!(response.status, status);

            // Test serialization for each variant
            let json = serde_json::to_string(&response).expect("Failed to serialize");
            let deserialized: DataTransferResponse =
                serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(response, deserialized);
        }
    }

    #[test]
    fn test_data_transfer_request_builder_pattern() {
        let vendor_id = "BuilderVendor".to_string();
        let message_id = "builder_msg".to_string();
        let data = "builder_data".to_string();
        let custom_data = create_test_custom_data();

        let request = DataTransferRequest::new(vendor_id.clone())
            .with_message_id(message_id.clone())
            .with_data(data.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.vendor_id, vendor_id);
        assert_eq!(request.message_id, Some(message_id));
        assert_eq!(request.data, Some(data));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_data_transfer_response_builder_pattern() {
        let status = DataTransferStatusEnumType::Accepted;
        let status_info = create_test_status_info();
        let data = "builder_response_data".to_string();
        let custom_data = create_test_custom_data();

        let response = DataTransferResponse::new(status.clone())
            .with_status_info(status_info.clone())
            .with_data(data.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, status);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.data, Some(data));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_data_transfer_request_edge_cases() {
        // Test with empty vendor ID
        let empty_vendor = "".to_string();
        let request_empty = DataTransferRequest::new(empty_vendor.clone());
        assert_eq!(request_empty.vendor_id, empty_vendor);
        assert!(request_empty.validate().is_ok());

        // Test with maximum length vendor ID
        let max_vendor = "a".repeat(255);
        let request_max = DataTransferRequest::new(max_vendor.clone());
        assert_eq!(request_max.vendor_id, max_vendor);
        assert!(request_max.validate().is_ok());

        // Test with empty message ID
        let request_empty_msg = DataTransferRequest::new("Vendor".to_string())
            .with_message_id("".to_string());
        assert_eq!(request_empty_msg.message_id, Some("".to_string()));
        assert!(request_empty_msg.validate().is_ok());

        // Test with maximum length message ID
        let max_message_id = "a".repeat(50);
        let request_max_msg = DataTransferRequest::new("Vendor".to_string())
            .with_message_id(max_message_id.clone());
        assert_eq!(request_max_msg.message_id, Some(max_message_id));
        assert!(request_max_msg.validate().is_ok());

        // Test with empty data
        let request_empty_data = DataTransferRequest::new("Vendor".to_string())
            .with_data("".to_string());
        assert_eq!(request_empty_data.data, Some("".to_string()));
        assert!(request_empty_data.validate().is_ok());

        // Test with maximum length data
        let max_data = "a".repeat(255);
        let request_max_data = DataTransferRequest::new("Vendor".to_string())
            .with_data(max_data.clone());
        assert_eq!(request_max_data.data, Some(max_data));
        assert!(request_max_data.validate().is_ok());
    }

    #[test]
    fn test_data_transfer_response_edge_cases() {
        // Test with empty data
        let response_empty_data = DataTransferResponse::new(DataTransferStatusEnumType::Accepted)
            .with_data("".to_string());
        assert_eq!(response_empty_data.data, Some("".to_string()));
        assert!(response_empty_data.validate().is_ok());

        // Test with maximum length data
        let max_data = "a".repeat(255);
        let response_max_data = DataTransferResponse::new(DataTransferStatusEnumType::Accepted)
            .with_data(max_data.clone());
        assert_eq!(response_max_data.data, Some(max_data));
        assert!(response_max_data.validate().is_ok());
    }

    #[test]
    fn test_data_transfer_request_with_all_fields() {
        let vendor_id = "CompleteVendor".to_string();
        let message_id = "complete_msg".to_string();
        let data = "complete_data".to_string();
        let custom_data = create_test_custom_data();

        let request = DataTransferRequest::new(vendor_id.clone())
            .with_message_id(message_id.clone())
            .with_data(data.clone())
            .with_custom_data(custom_data.clone());

        // Verify all fields are set
        assert_eq!(request.vendor_id, vendor_id);
        assert_eq!(request.message_id, Some(message_id));
        assert_eq!(request.data, Some(data));
        assert_eq!(request.custom_data, Some(custom_data));

        // Test validation and serialization
        assert!(request.validate().is_ok());

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: DataTransferRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_data_transfer_response_with_all_fields() {
        let status = DataTransferStatusEnumType::Rejected;
        let status_info = create_test_status_info();
        let data = "complete_response_data".to_string();
        let custom_data = create_test_custom_data();

        let response = DataTransferResponse::new(status.clone())
            .with_status_info(status_info.clone())
            .with_data(data.clone())
            .with_custom_data(custom_data.clone());

        // Verify all fields are set
        assert_eq!(response.status, status);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.data, Some(data));
        assert_eq!(response.custom_data, Some(custom_data));

        // Test validation and serialization
        assert!(response.validate().is_ok());

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: DataTransferResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_data_transfer_request_minimal() {
        // Test with only required field
        let vendor_id = "MinimalVendor".to_string();
        let request = DataTransferRequest::new(vendor_id.clone());

        assert_eq!(request.vendor_id, vendor_id);
        assert_eq!(request.message_id, None);
        assert_eq!(request.data, None);
        assert_eq!(request.custom_data, None);

        assert!(request.validate().is_ok());

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: DataTransferRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_data_transfer_response_minimal() {
        // Test with only required field
        let status = DataTransferStatusEnumType::UnknownVendorId;
        let response = DataTransferResponse::new(status.clone());

        assert_eq!(response.status, status);
        assert_eq!(response.status_info, None);
        assert_eq!(response.data, None);
        assert_eq!(response.custom_data, None);

        assert!(response.validate().is_ok());

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: DataTransferResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(response, deserialized);
    }
}