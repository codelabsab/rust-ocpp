#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum MonitorEnumType {
    UpperThreshold,
    LowerThreshold,
    DeltaPeriodic,
    PeriodicClockAligned,
}
