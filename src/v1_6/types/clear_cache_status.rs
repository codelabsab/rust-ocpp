/// Status returned in response to ClearCacheRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ClearCacheStatus {
    /// Command has been executed.
    #[default]
    Accepted,
    /// Command has not been executed.
    Rejected,
}
