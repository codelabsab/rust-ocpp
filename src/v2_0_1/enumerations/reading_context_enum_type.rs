#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
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
