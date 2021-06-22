#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum UpdateEnumType {
    Differential,
    Full,
}
