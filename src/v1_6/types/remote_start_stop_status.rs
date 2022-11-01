/// The result of a RemoteStartTransaction.req or RemoteStopTransaction.req request.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum RemoteStartStopStatus {
    /// Command will be executed.
    #[default]
    Accepted,
    /// Command will not be executed.
    Rejected,
}
