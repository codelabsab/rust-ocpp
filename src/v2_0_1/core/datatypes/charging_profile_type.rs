use chrono::{DateTime, Utc};

use crate::v2_0_1::core::enumerations::{
    charging_profile_kind_enum_type::ChargingProfileKindEnumType,
    charging_profile_purpose_enum_type::ChargingProfilePurposeEnumType,
    recurrency_kind_enum_type::RecurrencyKindEnumType,
};

use super::charging_schedule_type::ChargingScheduleType;

/// A ChargingProfile consists of ChargingSchedule, describing the amount of power or current that can be delivered per time interval
/// ChargingProfileType is used by: RequestStartTransactionRequest , SetChargingProfileRequest , ReportChargingProfilesRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfileType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ChargingProfilePurposeEnumType>,
    pub stack_level: i64,
    pub charging_profile_purpose: ChargingProfilePurposeEnumType,
    pub charging_profile_kind: ChargingProfileKindEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrency_kind: Option<RecurrencyKindEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    pub charging_schedule: ChargingScheduleType,
}
