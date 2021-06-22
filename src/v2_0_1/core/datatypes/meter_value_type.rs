use chrono::{DateTime, Utc};

use super::sampled_value_type::SampledValueType;

/// Collection of one or more sampled values in MeterValuesRequest and TransactionEvent. All sampled values in a MeterValue are sampled at the same point in time.
/// MeterValueType is used by: MeterValuesRequest , TransactionEventRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MeterValueType {
    pub timestamp: DateTime<Utc>,
    pub sampled_value: SampledValueType,
}
