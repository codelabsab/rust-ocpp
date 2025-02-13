use crate::v2_1::{
    datatypes::custom_data::CustomDataType, enumerations::ChargingLimitSourceEnumType,
};
use serde::{Deserialize, Serialize};

/// Represents a charging limit for a charging session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingLimitType {
    /// Represents the source of the charging limit.
    pub charging_limit_source: ChargingLimitSourceEnumType,

    /// True when the reported limit concerns local generation that is providing extra capacity, instead of a limitation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_local_generation: Option<bool>,

    /// Indicates whether the charging limit is critical for the grid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_grid_critical: Option<bool>,

    /// Custom data specific to this charging limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
