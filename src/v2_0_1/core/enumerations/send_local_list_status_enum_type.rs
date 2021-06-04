#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum SendLocalListStatusEnumType {
    Accepted,
    Failed,
    VersionMismatch,
}
