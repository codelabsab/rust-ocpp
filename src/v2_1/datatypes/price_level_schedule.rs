use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, price_level_schedule_entry::PriceLevelScheduleEntryType};

/// Price level schedule structure defines a list of time periods during which a specific price level applies.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PriceLevelScheduleType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Point in time at which the schedule becomes active.
    pub time_anchor: DateTime<Utc>,

    /// Required. List of price level schedule entries.
    #[validate(length(min = 1, max = 1024))]
    pub price_level_schedule_entries: Vec<PriceLevelScheduleEntryType>,
}
