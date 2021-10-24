/// Status in TriggerMessageResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum TriggerMessageStatus {
    /// Requested notification will be sent.
    Accepted,
    /// Requested notification will not be sent.
    Rejected,
    /// Requested notification cannot be sent because it is either not implemented or unknown.
    NotImplemented,
}
