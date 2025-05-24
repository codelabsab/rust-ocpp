use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{custom_data::CustomDataType, status_info::StatusInfoType};
use crate::v2_1::enumerations::unlock_status::UnlockStatusEnumType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnectorRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub evse_id: i32,
    pub connector_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnectorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: UnlockStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl UnlockConnectorRequest {
    pub fn new(evse_id: i32, connector_id: i32) -> Self {
        Self {
            custom_data: None,
            evse_id,
            connector_id,
        }
    }
}

impl UnlockConnectorResponse {
    pub fn new(status: UnlockStatusEnumType) -> Self {
        Self {
            custom_data: None,
            status,
            status_info: None,
        }
    }
}
