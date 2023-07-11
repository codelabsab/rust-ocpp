#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum MessagePriorityEnumType {
    AlwaysFront,
    InFront,
    #[default]
    NormalCycle,
}
