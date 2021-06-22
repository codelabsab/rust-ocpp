#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum EventTriggerEnumType {
    Alerting,
    Delta,
    Periodic,
}
