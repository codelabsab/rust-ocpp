#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum MonitorEnumType {
    #[default]
    UpperThreshold,
    LowerThreshold,
    Delta,
    Periodic,
    PeriodicClockAligned,
}
