use serde::{Deserialize, Serialize};

/// Status of setting a variable monitoring.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SetMonitoringStatusEnumType {
    /// Request has been accepted and the monitoring has been set.
    #[default]
    #[serde(rename = "Accepted")]
    Accepted,

    /// Component specified in the request is unknown.
    #[serde(rename = "UnknownComponent")]
    UnknownComponent,

    /// Variable specified in the request is unknown.
    #[serde(rename = "UnknownVariable")]
    UnknownVariable,

    /// Monitor type specified in the request is not supported.
    #[serde(rename = "UnsupportedMonitorType")]
    UnsupportedMonitorType,

    /// Request has been rejected.
    #[serde(rename = "Rejected")]
    Rejected,

    /// Value is outside the allowed range.
    #[serde(rename = "OutOfRange")]
    OutOfRange,
}
