#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum ResetEnumType {
    Immediate,
    OnIdle,
}
