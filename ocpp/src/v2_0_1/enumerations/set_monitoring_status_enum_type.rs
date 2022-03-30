#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum SetMonitoringStatusEnumType {
    Accepted,
    UnknownComponent,
    UnknownVariable,
    UnsupportedMonitorType,
    Rejected,
    Duplicate,
}
