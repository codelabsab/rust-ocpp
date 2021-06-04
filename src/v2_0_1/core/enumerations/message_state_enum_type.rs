#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum MessageStateEnumType {
    Charging,
    Faulted,
    Idle,
    Unavailable,
}
