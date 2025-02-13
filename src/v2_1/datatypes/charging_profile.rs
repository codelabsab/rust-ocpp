use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{charging_schedule::ChargingScheduleType, CustomDataType},
    enumerations::{
        ChargingProfileKindEnumType, ChargingProfilePurposeEnumType, RecurrencyKindEnumType,
    },
};

/// A ChargingProfile consists of 1 to 3 ChargingSchedules with a list of ChargingSchedulePeriods,
/// describing the amount of power or current that can be delivered per time interval.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfileType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Id of ChargingProfile. Unique within charging station. Id can have a negative value.
    /// This is useful to distinguish charging profiles from an external actor (external constraints)
    /// from charging profiles received from CSMS.
    pub id: i32,

    /// Value determining level in hierarchy stack of profiles. Higher values have precedence over lower values.
    /// Lowest level is 0.
    pub stack_level: i32,

    /// Defines the purpose of the schedule transferred by this profile
    pub charging_profile_purpose: ChargingProfilePurposeEnumType,

    /// Indicates the kind of schedule.
    pub charging_profile_kind: ChargingProfileKindEnumType,

    /// Indicates the start point of a recurrence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrency_kind: Option<RecurrencyKindEnumType>,

    /// Point in time at which the profile starts to be valid.
    /// If absent, the profile is valid as soon as it is received by the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<DateTime<Utc>>,

    /// Point in time at which the profile stops to be valid.
    /// If absent, the profile is valid until it is replaced by another profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<DateTime<Utc>>,

    /// Contains limits for the available power or current over time.
    pub charging_schedule: Vec<ChargingScheduleType>,

    /// SHALL only be included if ChargingProfilePurpose is set to TxProfile.
    /// The transactionId is used to match the profile to a specific transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}
