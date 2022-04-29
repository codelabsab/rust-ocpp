#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum SendLocalListStatusEnumType {
    Accepted,
    Failed,
    VersionMismatch,
}
