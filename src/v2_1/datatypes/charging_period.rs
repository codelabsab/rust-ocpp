use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{cost_dimension::CostDimensionType, custom_data::CustomDataType};

/// A ChargingPeriodType consists of a start time, and a list of possible values that influence this period,
/// for example: amount of energy charged this period, maximum current during this period etc.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingPeriodType {
    /// Start timestamp of charging period. A period ends when the next period starts.
    /// The last period ends when the session ends.
    pub start_period: DateTime<Utc>,

    /// List of dimensions that influence this period.
    pub dimensions: Vec<CostDimensionType>,

    /// Unique identifier of the Tariff that was used to calculate cost.
    /// If not provided, then cost was calculated by some other means.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 60))]
    pub tariff_id: Option<String>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
