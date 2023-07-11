#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum OperationalStatusEnumType {
    Inoperative,
    #[default]
    Operative,
}
