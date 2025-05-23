use serde::{Deserialize, Serialize};

/// The type of reset that the Charging Station or EVSE should perform.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ResetEnumType {
    /// Charging Station shall immediately reset.
    #[default]
    #[serde(rename = "Immediate")]
    Immediate,

    /// Charging Station shall reset when no transaction is ongoing.
    #[serde(rename = "OnIdle")]
    OnIdle,

    /// Charging Station shall immediately reset and resume operations after reset.
    #[serde(rename = "ImmediateAndResume")]
    ImmediateAndResume,
}
