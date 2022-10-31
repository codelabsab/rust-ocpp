#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum ClearMonitoringStatusEnumType {
    #[default]
    Accepted,
    Rejected,
    NotFound,
}
