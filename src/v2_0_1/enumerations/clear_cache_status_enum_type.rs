#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum ClearCacheStatusEnumType {
    #[default]
    Accepted,
    Rejected,
}
