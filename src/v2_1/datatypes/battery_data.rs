use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Contains EV battery parameters.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BatteryDataType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Battery level at the start of charging as percentage of the battery capacity.
    #[validate(range(min = 0.0, max = 100.0))]
    pub charging_start_soc: f64,

    /// Required. Battery level at the end of charging as percentage of the battery capacity.
    #[validate(range(min = 0.0, max = 100.0))]
    pub charging_end_soc: f64,

    /// Required. Battery capacity in kWh.
    pub battery_capacity: f64,

    /// Required. Battery energy capacity that can be recharged, in kWh.
    pub rechargeable_energy_capacity: f64,
}
