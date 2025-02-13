use serde::{Deserialize, Serialize};

/// Represents the source of the charging limit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChargingLimitSourceEnumType {
    /// Energy Management System
    #[serde(rename = "EMS")]
    EMS,
    /// Other source
    #[serde(rename = "Other")]
    Other,
    /// System Operator
    #[serde(rename = "SO")]
    SO,
    /// Charging Station Operator
    #[serde(rename = "CSO")]
    CSO,
}
