/// Result of ResetRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ResetRequestStatus {
    /// Command will be executed.
    Hard,
    /// Command will not be executed.
    #[default]
    Soft,
}

/// Result of ResetResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ResetResponseStatus {
    /// Command will be executed.
    #[default]
    Accepted,
    /// Command will not be executed.
    Rejected,
}
