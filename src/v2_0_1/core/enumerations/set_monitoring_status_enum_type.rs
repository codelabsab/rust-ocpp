#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum SetMonitoringStatusEnumType {
    Accepted,
    UnknownComponent,
    UnknownVariable,
    UnsupportedMonitorType,
    Rejected,
    Duplicate,
}
