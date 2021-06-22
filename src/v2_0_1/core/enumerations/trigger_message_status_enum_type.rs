#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum TriggerMessageStatusEnumType {
    Accepted,
    Rejected,
    NotImplemented,
}
