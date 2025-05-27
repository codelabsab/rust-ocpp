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
