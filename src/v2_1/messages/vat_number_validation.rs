use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{
    address::AddressType, custom_data::CustomDataType, status_info::StatusInfoType,
};
use crate::v2_1::enumerations::generic_status::GenericStatusEnumType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VatNumberValidationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub vat_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VatNumberValidationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<AddressType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
    pub vat_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
    pub status: GenericStatusEnumType,
}

impl VatNumberValidationRequest {
    pub fn new(vat_number: String) -> Self {
        Self {
            custom_data: None,
            vat_number,
            evse_id: None,
        }
    }
}

impl VatNumberValidationResponse {
    pub fn new(vat_number: String, status: GenericStatusEnumType) -> Self {
        Self {
            custom_data: None,
            company: None,
            status_info: None,
            vat_number,
            evse_id: None,
            status,
        }
    }
}
