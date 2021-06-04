#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum ClearMonitoringStatusEnumType {
    Accepted,
    Rejected,
    NotFound,
}
