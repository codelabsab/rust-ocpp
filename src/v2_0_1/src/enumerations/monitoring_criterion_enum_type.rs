#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum MonitoringCriterionEnumType {
    #[default]
    ThresholdMonitoring,
    DeltaMonitoring,
    PeriodicMonitoring,
}
