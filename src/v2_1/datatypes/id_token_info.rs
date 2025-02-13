use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, status_info::StatusInfoType};
use crate::v2_1::enumerations::AuthorizationStatusEnumType;

/// Contains status information about an identifier.
/// It is advised to not stop charging if the status is Accepted or ConcurrentTx.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenInfoType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. This contains whether the identifier is allowed for charging.
    pub status: AuthorizationStatusEnumType,

    /// Optional. Only filled in when the status is ConcurrentTx.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_expiry_date_time: Option<DateTime<Utc>>,

    /// Optional. Priority from a business point of view.
    /// Default priority is 0, The range is from -9 to 9.
    /// Higher values indicate a higher priority.
    /// The chargingPriority in a ChargingProfile SHALL overrule this priority range.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = -9, max = 9))]
    pub charging_priority: Option<i8>,

    /// Optional. Contains information about authorization status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. A case insensitive identifier to use for the authorization and the load profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub group_id_token: Option<String>,

    /// Optional. Contains the date at which idToken should be removed from the Authorization Cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<DateTime<Utc>>,

    /// Optional. This contains the identifier to use for charging.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub parent_id_token: Option<String>,

    /// Optional. Contains a case insensitive identifier to use for the user profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub personal_message: Option<String>,
}
