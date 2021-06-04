#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum ReadingContextEnumType {
    InterruptionBegin,
    InterruptionEnd,
    Other,
    SampleClock,
    SamplePeriodic,
    TransactionBegin,
    TransactionEnd,
    Trigger,
}
