#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum MonitoringBaseEnumType {
    All,
    FactoryDefault,
    HardWiredOnly,
}
