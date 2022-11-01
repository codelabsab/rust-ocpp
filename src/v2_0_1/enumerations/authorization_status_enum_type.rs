#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum AuthorizationStatusEnumType {
    #[default]
    Accepted,
    Blocked,
    ConcurrentTx,
    Expired,
    Invalid,
    NoCredit,
    NotAllowedTypeEVSE,
    NotAtThisLocation,
    NotAtThisTime,
    Unknown,
}
