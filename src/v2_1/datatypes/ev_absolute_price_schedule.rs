use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    custom_data::CustomDataType, ev_absolute_price_schedule_entry::EVAbsolutePriceScheduleEntryType,
};

/// Price schedule of EV energy offer.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EVAbsolutePriceScheduleType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Starting point in time of the EVEnergyOffer.
    pub time_anchor: DateTime<Utc>,

    /// Currency code according to ISO 4217.
    #[validate(length(max = 3))]
    pub currency: String,

    /// ISO 15118-20 URN of price algorithm: Power, PeakPower, StackedEnergy.
    #[validate(length(max = 2000))]
    pub price_algorithm: String,

    /// List of price schedule entries.
    #[validate(length(min = 1, max = 1024))]
    pub ev_absolute_price_schedule_entries: Vec<EVAbsolutePriceScheduleEntryType>,
}
