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
