#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum MonitoringBaseEnumType {
    All,
    FactoryDefault,
    HardWiredOnly,
}
