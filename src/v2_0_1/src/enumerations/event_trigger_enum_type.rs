#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum EventTriggerEnumType {
    #[default]
    Alerting,
    Delta,
    Periodic,
}
