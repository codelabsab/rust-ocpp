use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, signed_meter_value::SignedMeterValueType};
use crate::v2_1::enumerations::{
    LocationEnumType, MeasurandEnumType, PhaseEnumType, ReadingContextEnumType,
};

/// Single sampled value in MeterValues.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SampledValueType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Value as a floating-point number.
    pub value: f64,

    /// Optional. Type of detail value: start, end or sample.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ReadingContextEnumType>,

    /// Optional. Type of measurement value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurand: Option<MeasurandEnumType>,

    /// Optional. Phase as measured or assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<PhaseEnumType>,

    /// Optional. Location of measurement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationEnumType>,

    /// Optional. Contains the signed version of the meter value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_meter_value: Option<SignedMeterValueType>,
}
