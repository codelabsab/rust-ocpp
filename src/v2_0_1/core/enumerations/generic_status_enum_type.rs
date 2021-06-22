#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum GenericStatusEnumType {
    Accepted,
    Rejected,
}
