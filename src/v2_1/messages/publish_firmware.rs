use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{custom_data::CustomDataType, status_info::StatusInfoType};
use crate::v2_1::enumerations::generic_status::GenericStatusEnumType;

/// Request to publish firmware to a Local Controller.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareRequest {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. This contains a string containing a URI pointing to a
    /// location from which to retrieve the firmware.
    #[validate(length(min = 1))]
    pub location: String,

    /// Optional. This specifies how many times Charging Station must retry
    /// to download the firmware before giving up. If this field is not
    /// present, it is left to Charging Station to decide how many times it wants to retry.
    /// If the value is 0, it means: no retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub retries: Option<i32>,

    /// Required. The MD5 checksum over the entire firmware file as a hexadecimal string of length 32.
    #[validate(length(min = 32, max = 32))]
    pub checksum: String,

    /// Required. The Id of the request.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// Optional. The interval in seconds
    /// after which a retry may be
    /// attempted. If this field is not
    /// present, it is left to Charging
    /// Station to decide how long to wait
    /// between attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub retry_interval: Option<i32>,
}

/// Response to a PublishFirmwareRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareResponse {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Indicates whether the request was accepted.
    pub status: GenericStatusEnumType,

    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl PublishFirmwareRequest {
    /// Creates a new `PublishFirmwareRequest` with required fields.
    ///
    /// # Arguments
    ///
    /// * `location` - URI pointing to a location from which to retrieve the firmware
    /// * `checksum` - MD5 checksum over the entire firmware file
    /// * `request_id` - The Id of the request
    ///
    /// # Returns
    ///
    /// A new instance of `PublishFirmwareRequest` with optional fields set to `None`
    pub fn new(location: String, checksum: String, request_id: i32) -> Self {
        Self {
            custom_data: None,
            location,
            retries: None,
            checksum,
            request_id,
            retry_interval: None,
        }
    }
}

impl PublishFirmwareResponse {
    /// Creates a new `PublishFirmwareResponse` with required fields.
    ///
    /// # Arguments
    ///
    /// * `status` - Indicates whether the request was accepted
    ///
    /// # Returns
    ///
    /// A new instance of `PublishFirmwareResponse` with optional fields set to `None`
    pub fn new(status: GenericStatusEnumType) -> Self {
        Self {
            custom_data: None,
            status,
            status_info: None,
        }
    }
}
