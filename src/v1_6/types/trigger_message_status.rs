/// Status in TriggerMessageResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum TriggerMessageStatus {
    /// Requested notification will be sent.
    #[default]
    Accepted,
    /// Requested notification will not be sent.
    Rejected,
    /// Requested notification cannot be sent because it is either not implemented or unknown.
    NotImplemented,
}
