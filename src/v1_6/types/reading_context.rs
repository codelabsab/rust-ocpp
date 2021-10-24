/// Values of the context field of a value in SampledValue.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ReadingContext {
    /// Value taken at start of interruption.
    #[serde(rename = "Interruption.Begin")]
    InterruptionBegin,
    /// Value taken when resuming after interruption.
    #[serde(rename = "Interruption.End")]
    InterruptionEnd,
    /// Value for any other situations.
    Other,
    /// Value taken at clock aligned interval.
    #[serde(rename = "Sample.Clock")]
    SampleClock,
    /// Value taken as periodic sample relative to start time of transaction.
    #[serde(rename = "Sample.Periodic")]
    SamplePeriodic,
    /// Value taken at start of transaction.
    #[serde(rename = "Transaction.Begin")]
    TransactionBegin,
    /// Value taken at end of transaction.
    #[serde(rename = "Transaction.End")]
    TransactionEnd,
    /// Value taken in response to a TriggerMessage.req
    Trigger,
}
