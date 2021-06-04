#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum MonitoringCriterionEnumType {
    ThresholdMonitoring,
    DeltaMonitoring,
    PeriodicMonitoring,
}
