use serde::{Deserialize, Serialize};

/// Status of setting a network profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SetNetworkProfileStatusEnumType {
    /// Request has been accepted and the network profile has been set.
    #[default]
    #[serde(rename = "Accepted")]
    Accepted,

    /// Request has been rejected.
    #[serde(rename = "Rejected")]
    Rejected,

    /// Request has been accepted but failed to be applied.
    #[serde(rename = "Failed")]
    Failed,
}
