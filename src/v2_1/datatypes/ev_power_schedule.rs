use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, ev_power_schedule_entry::EVPowerScheduleEntryType};

/// Power schedule of EV energy offer.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EVPowerScheduleType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Starting point in time of the EVEnergyOffer.
    pub time_anchor: DateTime<Utc>,

    /// List of power schedule entries.
    #[validate(length(min = 1, max = 1024))]
    pub ev_power_schedule_entries: Vec<EVPowerScheduleEntryType>,
}
