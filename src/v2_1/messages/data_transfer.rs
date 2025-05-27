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
