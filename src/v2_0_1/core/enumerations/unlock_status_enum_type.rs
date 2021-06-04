#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum UnlockStatusEnumType {
    Unlocked,
    UnlockFailed,
    OngoingAuthorizedTransaction,
    UnknownConnector,
}
