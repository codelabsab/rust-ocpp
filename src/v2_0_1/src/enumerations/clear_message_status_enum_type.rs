#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum ClearMessageStatusEnumType {
    #[default]
    Accepted,
    Unknown,
}
