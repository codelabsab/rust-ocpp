#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum MonitorEnumType {
    UpperThreshold,
    LowerThreshold,
    DeltaPeriodic,
    PeriodicClockAligned,
}
