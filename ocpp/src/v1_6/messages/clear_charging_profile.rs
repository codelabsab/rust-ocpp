use crate::v1_6::types::{ChargingProfilePurposeType, ClearChargingProfileStatus};
use validator::Validate;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connector_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charging_profile_purpose: Option<ChargingProfilePurposeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack_level: Option<i64>,
}

#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
pub struct ClearChargingProfileResponse {
    /// This contains the field definition of the ClearChargingProfile.conf PDU sent by the Charge Point to the Central
    /// System in response to a ClearChargingProfile.req PDU. See also Clear Charging Profile
    status: ClearChargingProfileStatus,
}
