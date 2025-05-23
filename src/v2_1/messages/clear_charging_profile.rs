use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, StatusInfoType, ClearChargingProfileType},
    enumerations::ClearChargingProfileStatusEnumType,
};

/// Request to clear charging profiles from a charging station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileRequest {
    /// Optional. The Id of the charging profile to clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_id: Option<i32>,

    /// Optional. Charging profile criteria to use for clearing profiles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_criteria: Option<ClearChargingProfileType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a ClearChargingProfileRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileResponse {
    /// Required. Indicates if the Charging Station was able to execute the request.
    pub status: ClearChargingProfileStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
