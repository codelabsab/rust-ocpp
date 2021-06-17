#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum OperationalStatusEnumType {
    Inoperative,
    Operative,
}
