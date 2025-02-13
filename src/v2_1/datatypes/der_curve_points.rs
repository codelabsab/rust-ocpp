use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Points defining a DER curve.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DERCurvePointsType {
    /// The data value of the X-axis (independent) variable, depending on the curve type.
    pub x: f64,

    /// The data value of the Y-axis (dependent) variable, depending on the DERUnitEnumType of the curve.
    /// If y is power factor, then a positive value means DER is absorbing reactive power (under-excited),
    /// a negative value when DER is injecting reactive power (over-excited).
    pub y: f64,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
