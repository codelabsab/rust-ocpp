use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    custom_data::CustomDataType, ev_absolute_price_schedule::EVAbsolutePriceScheduleType,
    ev_power_schedule::EVPowerScheduleType,
};

/// Energy offer from EV to EVSE.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EVEnergyOfferType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Power schedule of EV energy offer.
    pub ev_power_schedule: EVPowerScheduleType,

    /// Price schedule of EV energy offer.
    pub ev_absolute_price_schedule: EVAbsolutePriceScheduleType,
}
