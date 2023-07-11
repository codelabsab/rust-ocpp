#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum MessageStateEnumType {
    Charging,
    Faulted,
    #[default]
    Idle,
    Unavailable,
}
