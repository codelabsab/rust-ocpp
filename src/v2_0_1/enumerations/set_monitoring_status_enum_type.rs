#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum SetMonitoringStatusEnumType {
    #[default]
    Accepted,
    UnknownComponent,
    UnknownVariable,
    UnsupportedMonitorType,
    Rejected,
    Duplicate,
}
