/// Type of update for a SendLocalListRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum UpdateType {
    /// Indicates that the current Local Authorization List must be updated with the values in this message.
    Differential,
    /// Indicates that the current Local Authorization List must be replaced by the values in this message.
    #[default]
    Full,
}
