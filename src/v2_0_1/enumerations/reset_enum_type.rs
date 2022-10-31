#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ResetEnumType {
    Immediate,
    #[default]
    OnIdle,
}
