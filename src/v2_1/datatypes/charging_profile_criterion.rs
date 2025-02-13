use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{datatypes::CustomDataType, enumerations::ChargingProfilePurposeEnumType};

/// A ChargingProfileCriterionType is a filter for charging profiles to be selected by a GetChargingProfilesRequest.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfileCriterionType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Defines the purpose of the schedule transferred by this profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,

    /// Value determining level in hierarchy stack of profiles. Higher values have precedence over lower values. Lowest level is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i32>,

    /// List of all the chargingProfileIds requested. Any ChargingProfile that matches one of these profiles will be reported.
    /// If omitted, the Charging Station SHALL not filter on chargingProfileId.
    /// This field SHALL NOT contain more ids than set in ChargingProfileEntries.maxLimit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_id: Option<Vec<i32>>,
}
