#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum MonitoringCriterionEnumType {
    ThresholdMonitoring,
    DeltaMonitoring,
    PeriodicMonitoring,
}
