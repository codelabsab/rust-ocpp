/// Type of update for a SendLocalListRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum UpdateStatus {
    /// Local Authorization List successfully updated.
    Accepted,
    /// Failed to update the Local Authorization List.
    Failed,
    /// Update of Local Authorization List is not supported by Charge Point.
    NotSupported,
    /// Version number in the request for a differential update is less or equal then version number of current list.
    VersionMismatch,
}
