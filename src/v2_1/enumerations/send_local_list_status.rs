use serde::{Deserialize, Serialize};

/// Status indicating whether the Charging Station has successfully received and applied
/// the update of the Local Authorization List.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SendLocalListStatusEnumType {
    /// Local authorization list has been accepted and applied.
    #[default]
    #[serde(rename = "Accepted")]
    Accepted,

    /// Failed to apply the local authorization list.
    #[serde(rename = "Failed")]
    Failed,

    /// Version number in the request does not match the version number in the Charging Station.
    #[serde(rename = "VersionMismatch")]
    VersionMismatch,
}
