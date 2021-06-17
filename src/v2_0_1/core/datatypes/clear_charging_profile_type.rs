use crate::v2_0_1::core::enumerations::charging_profile_purpose_enum_type::ChargingProfilePurposeEnumType;

/// A ChargingProfile consists of a ChargingSchedule, describing the amount of power or current that can be delivered per time interval.
/// ClearChargingProfileType is used by: ClearChargingProfileRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i64>,
}
