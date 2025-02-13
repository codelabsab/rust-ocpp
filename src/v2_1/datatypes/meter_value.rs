use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, sampled_value::SampledValueType};

/// Collection of one or more sampled values in MeterValuesRequest and StopTransactionRequest.
/// All sampled values in a MeterValue are sampled at the same point in time.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeterValueType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Timestamp for measured value(s).
    pub timestamp: DateTime<Utc>,

    /// Required. One or more measured values.
    #[validate(length(min = 1))]
    pub sampled_value: Vec<SampledValueType>,
}
