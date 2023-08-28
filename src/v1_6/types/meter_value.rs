use chrono::DateTime;
use chrono::Utc;

use super::SampledValue;
use crate::Vec;

/// Collection of one or more sampled values in MeterValues.req and StopTransaction.req. All sampled values in a MeterValue are sampled at the same point in time.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeterValue<'a, const N_SAMPLED_VALUES: usize = { crate::N_SAMPLED_VALUES }> {
    /// Required. Timestamp for measured value(s).
    pub timestamp: DateTime<Utc>,
    /// Required. One or more measured values
    #[serde(borrow)]
    pub sampled_value: Vec<SampledValue<'a>, N_SAMPLED_VALUES>,
}
