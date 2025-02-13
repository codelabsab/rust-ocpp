use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// V2X charging parameters for an ISO 15118-20 session.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct V2XChargingParametersType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Maximum discharge power in W (DC) or VA (AC).
    pub ev_max_discharge_power: f64,

    /// Required. Maximum discharge current in A.
    pub ev_max_discharge_current: f64,

    /// Required. Maximum voltage at which the EV can discharge.
    pub ev_max_voltage: f64,

    /// Optional. Minimum voltage at which the EV can discharge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_min_voltage: Option<f64>,

    /// Optional. Minimum discharge current that the EV needs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_min_discharge_current: Option<f64>,

    /// Optional. Minimum discharge power that the EV needs, in W (DC) or VA (AC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_min_discharge_power: Option<f64>,
}
