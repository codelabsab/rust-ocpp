use super::signed_meter_value_type::SignedMeterValueType;
use super::unit_of_measure_type::UnitOfMeasureType;
use crate::v2_0_1::enumerations::location_enum_type::LocationEnumType;
use crate::v2_0_1::enumerations::measurand_enum_type::MeasurandEnumType;
use crate::v2_0_1::enumerations::phase_enum_type::PhaseEnumType;
use crate::v2_0_1::enumerations::reading_context_enum_type::ReadingContextEnumType;

/// Single sampled value in MeterValues. Each value can be accompanied by optional fields.
/// To save on mobile data usage, default values of all of the optional fields are such that. The value without any additional fields will be interpreted, as a register reading of active import energy in Wh (Watt-hour) units.
/// SampledValueType is used by: Common:MeterValueType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SampledValueType<'a> {
    pub value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ReadingContextEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurand: Option<MeasurandEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<PhaseEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationEnumType>,
    #[serde(skip_serializing_if = "Option::is_none",borrow)]
    pub signed_meter_value: Option<SignedMeterValueType<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<UnitOfMeasureType<'a>>,
}
