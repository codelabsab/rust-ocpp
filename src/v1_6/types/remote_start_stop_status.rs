/// The result of a RemoteStartTransaction.req or RemoteStopTransaction.req request.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum RemoteStartStopStatus {
    /// Command will be executed.
    Accepted,
    /// Command will not be executed.
    Rejected,
}
