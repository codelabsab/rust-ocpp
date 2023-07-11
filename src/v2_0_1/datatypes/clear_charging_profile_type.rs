use crate::v2_0_1::enumerations::charging_profile_purpose_enum_type::ChargingProfilePurposeEnumType;

/// A ChargingProfile consists of a ChargingSchedule, describing the amount of power or current that can be delivered per time interval.
/// ClearChargingProfileType is used by: ClearChargingProfileRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileType {
    /// Optional. Specifies the id of the EVSE for which to clear charging profiles. An evseId of zero (0) specifies the charging profile for the overall Charging Station. Absence of this parameter means the clearing applies to all charging profiles that match the other criteria in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i64>,
    /// Optional. Specifies to purpose of the charging profiles that will be cleared, if they meet the other criteria in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,
    /// Optional. Specifies the stackLevel for which charging profiles will be cleared, if they meet the other criteria in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i64>,
}
