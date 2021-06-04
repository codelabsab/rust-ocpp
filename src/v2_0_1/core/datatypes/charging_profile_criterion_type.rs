use crate::v2_0_1::core::enumerations::{
    charging_limit_source_enum_type::ChargingLimitSourceEnumType,
    charging_profile_purpose_enum_type::ChargingProfilePurposeEnumType,
};

/// A ChargingProfile consists of ChargingSchedule, describing the amount of power or current that can be delivered per time interval.
/// ChargingProfileCriterionType is used by: GetChargingProfilesRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfileCriterionType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_limit_source: Option<ChargingLimitSourceEnumType>,
}
