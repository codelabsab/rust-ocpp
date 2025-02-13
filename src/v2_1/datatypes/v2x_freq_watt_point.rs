use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Point in a frequency-watt curve for V2X.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct V2XFreqWattPointType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Frequency in Hz.
    pub freq: f64,

    /// Required. Power in percent of maximum power. Negative values indicate power being discharged from the EV.
    pub power: f64,
}
