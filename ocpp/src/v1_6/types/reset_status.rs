/// Result of ResetRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ResetStatus {
    /// Command will be executed.
    Accepted,
    /// Command will not be executed.
    Rejected,
}
