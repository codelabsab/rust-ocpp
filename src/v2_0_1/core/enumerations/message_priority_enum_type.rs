#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum MessagePriorityEnumType {
    AlwaysFront,
    InFront,
    NormalCycle,
}
