#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum SendLocalListStatusEnumType {
    #[default]
    Accepted,
    Failed,
    VersionMismatch,
}
