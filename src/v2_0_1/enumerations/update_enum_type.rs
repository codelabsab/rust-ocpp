#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum UpdateEnumType {
    Differential,
    #[default]
    Full,
}
