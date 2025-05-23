use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::custom_data::CustomDataType;
use crate::v2_1::enumerations::unpublish_firmware_status::UnpublishFirmwareStatusEnumType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UnpublishFirmwareRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub checksum: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UnpublishFirmwareResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: UnpublishFirmwareStatusEnumType,
}

impl UnpublishFirmwareRequest {
    pub fn new(checksum: String) -> Self {
        Self {
            custom_data: None,
            checksum,
        }
    }
}

impl UnpublishFirmwareResponse {
    pub fn new(status: UnpublishFirmwareStatusEnumType) -> Self {
        Self {
            custom_data: None,
            status,
        }
    }
}
