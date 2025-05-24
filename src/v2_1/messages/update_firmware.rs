use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{
    custom_data::CustomDataType, firmware::FirmwareType, status_info::StatusInfoType,
};
use crate::v2_1::enumerations::update_firmware_status::UpdateFirmwareStatusEnumType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
    pub request_id: i32,
    pub firmware: FirmwareType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: UpdateFirmwareStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl UpdateFirmwareRequest {
    pub fn new(request_id: i32, firmware: FirmwareType) -> Self {
        Self {
            custom_data: None,
            retries: None,
            retry_interval: None,
            request_id,
            firmware,
        }
    }
}

impl UpdateFirmwareResponse {
    pub fn new(status: UpdateFirmwareStatusEnumType) -> Self {
        Self {
            custom_data: None,
            status,
            status_info: None,
        }
    }
}
