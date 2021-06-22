#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum UnlockStatusEnumType {
    Unlocked,
    UnlockFailed,
    OngoingAuthorizedTransaction,
    UnknownConnector,
}
