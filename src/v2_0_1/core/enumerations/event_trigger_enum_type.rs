#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum EventTriggerEnumType {
    Alerting,
    Delta,
    Periodic,
}
