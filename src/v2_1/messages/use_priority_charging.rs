use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{custom_data::CustomDataType, status_info::StatusInfoType};
use crate::v2_1::enumerations::priority_charging_status::PriorityChargingStatusEnumType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UsePriorityChargingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub transaction_id: String,
    pub activate: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UsePriorityChargingResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: PriorityChargingStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl UsePriorityChargingRequest {
    pub fn new(transaction_id: String, activate: bool) -> Self {
        Self {
            custom_data: None,
            transaction_id,
            activate,
        }
    }
}

impl UsePriorityChargingResponse {
    pub fn new(status: PriorityChargingStatusEnumType) -> Self {
        Self {
            custom_data: None,
            status,
            status_info: None,
        }
    }
}
