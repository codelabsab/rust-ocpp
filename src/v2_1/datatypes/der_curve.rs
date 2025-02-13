use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    custom_data::CustomDataType, der_curve_points::DERCurvePointsType, hysteresis::HysteresisType,
    reactive_power_params::ReactivePowerParamsType, voltage_params::VoltageParamsType,
};
use crate::v2_1::enumerations::der_unit::DERUnitEnumType;

/// DER curve type for various DER control modes.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DERCurveType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// List of curve points defining this curve.
    #[validate(length(min = 1, max = 10))]
    pub curve_data: Vec<DERCurvePointsType>,

    /// Hysteresis parameters for this curve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hysteresis: Option<HysteresisType>,

    /// Priority of curve (0=highest)
    #[validate(range(min = 0))]
    pub priority: i32,

    /// Reactive power parameters for this curve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactive_power_params: Option<ReactivePowerParamsType>,

    /// Voltage parameters for this curve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage_params: Option<VoltageParamsType>,

    /// Unit of the Y-axis values.
    pub y_unit: DERUnitEnumType,

    /// Open loop response time, the time to ramp up to 90% of the new target in response to the change in voltage, in seconds.
    /// A value of 0 is used to mean no limit. When not present, the device should follow its default behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time: Option<f64>,

    /// Point in time when this curve will become activated. Only absent when default is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,

    /// Duration in seconds that this curve will be active. Only absent when default is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
}
