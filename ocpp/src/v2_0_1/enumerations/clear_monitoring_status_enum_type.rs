#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum ClearMonitoringStatusEnumType {
    Accepted,
    Rejected,
    NotFound,
}
