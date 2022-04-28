/// Result of ResetRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ResetRequestStatus {
    /// Command will be executed.
    Hard,
    /// Command will not be executed.
    Soft,
}

/// Result of ResetResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ResetResponseStatus {
    /// Command will be executed.
    Accepted,
    /// Command will not be executed.
    Rejected,
}
