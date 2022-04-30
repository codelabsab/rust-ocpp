#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum MessageStateEnumType {
    Charging,
    Faulted,
    Idle,
    Unavailable,
}
