//! ClearChargingProfile
use crate::v2_0_1::datatypes::clear_charging_profile_type::ClearChargingProfileType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::clear_charging_profile_status_enum_type::ClearChargingProfileStatusEnumType;

/// `ClearChargingProfileRequest`, sent by the CSMS to the Charging Station.
///
/// The CSMS can use this message to clear (remove) either a specific charging
/// profile (denoted by id) or a selection of charging profiles that match
///  with the values of the optional evse, stackLevel and ChargingProfilePurpose fields.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileRequest {
    /// The Id of the charging profile to clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_id: Option<i32>,
    /// Specifies the charging profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_criteria: Option<ClearChargingProfileType>,
}

/// `ClearChargingProfileResponse`, sent by the Charging Station to the CSMS
///  in response to a [`ClearChargingProfileRequest`].
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileResponse {
    /// Indicates if the Charging Station was able to execute the request.
    pub status: ClearChargingProfileStatusEnumType,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
