use serde::{Deserialize, Serialize};

/// This indicates whether the Charging Station is able to perform the reset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResetStatusEnumType {
    /// Reset request has been accepted and will be performed.
    #[serde(rename = "Accepted")]
    Accepted,

    /// Reset request has been rejected.
    #[serde(rename = "Rejected")]
    Rejected,

    /// Reset request has been scheduled for later execution.
    #[serde(rename = "Scheduled")]
    Scheduled,
}
