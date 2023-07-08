#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum GenericStatusEnumType {
    #[default]
    Accepted,
    Rejected,
}
