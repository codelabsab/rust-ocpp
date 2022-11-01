#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum TriggerMessageStatusEnumType {
    #[default]
    Accepted,
    Rejected,
    NotImplemented,
}
