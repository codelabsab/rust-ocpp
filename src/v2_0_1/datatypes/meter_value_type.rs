use chrono::DateTime;
use chrono::Utc;

use super::sampled_value_type::SampledValueType;
use crate::Vec;

/// Collection of one or more sampled values in MeterValuesRequest and TransactionEvent. All sampled values in a MeterValue are sampled at the same point in time.
/// MeterValueType is used by: MeterValuesRequest , TransactionEventRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeterValueType<'a, const N_SAMPLED_VALUES: usize> {
    pub timestamp: DateTime<Utc>,
    #[serde(borrow)]
    pub sampled_value: Vec<SampledValueType<'a>, N_SAMPLED_VALUES>,
}
