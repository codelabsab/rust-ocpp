#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum MessagePriorityEnumType {
    AlwaysFront,
    InFront,
    NormalCycle,
}
