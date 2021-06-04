#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum TriggerMessageStatusEnumType {
    Accepted,
    Rejected,
    NotImplemented,
}
