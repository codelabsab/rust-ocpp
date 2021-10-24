/// Status returned in response to ClearCacheRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ClearCacheStatus {
    /// Command has been executed.
    Accepted,
    /// Command has not been executed.
    Rejected,
}
