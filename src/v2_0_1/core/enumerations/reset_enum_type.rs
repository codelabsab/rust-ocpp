#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ResetEnumType {
    Immediate,
    OnIdle,
}
