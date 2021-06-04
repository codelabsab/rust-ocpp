#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum UpdateEnumType {
    Differential,
    Full,
}
