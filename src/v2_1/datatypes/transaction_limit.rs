use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Transaction limits for a charging session.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLimitType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Optional. Maximum amount of energy in kWh that may be delivered to an EV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_limit: Option<f64>,

    /// Optional. Maximum duration in seconds that a transaction may last.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_limit: Option<i32>,
}
