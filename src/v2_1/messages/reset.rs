use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{custom_data::CustomDataType, status_info::StatusInfoType};
use crate::v2_1::enumerations::{reset::ResetEnumType, reset_status::ResetStatusEnumType};

/// Request body for the Reset request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResetRequest {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. This contains the type of reset that the Charging Station or EVSE should perform.
    #[serde(rename = "type")]
    pub reset_type: ResetEnumType,

    /// Optional. This contains the ID of a specific EVSE that needs to be reset, instead of the entire Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,
}

/// Response body for the Reset response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResetResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. This indicates whether the Charging Station is able to perform the reset.
    pub status: ResetStatusEnumType,

    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl ResetRequest {
    /// Creates a new `ResetRequest` with required fields.
    ///
    /// # Arguments
    ///
    /// * `reset_type` - The type of reset that the Charging Station or EVSE should perform
    ///
    /// # Returns
    ///
    /// A new instance of `ResetRequest` with optional fields set to `None`
    pub fn new(reset_type: ResetEnumType) -> Self {
        Self {
            custom_data: None,
            reset_type,
            evse_id: None,
        }
    }
}

impl ResetResponse {
    /// Creates a new `ResetResponse` with required fields.
    ///
    /// # Arguments
    ///
    /// * `status` - Indicates whether the Charging Station is able to perform the reset
    ///
    /// # Returns
    ///
    /// A new instance of `ResetResponse` with optional fields set to `None`
    pub fn new(status: ResetStatusEnumType) -> Self {
        Self {
            custom_data: None,
            status,
            status_info: None,
        }
    }
}
