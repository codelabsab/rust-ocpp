/// Type of request to be triggered in a TriggerMessageRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum MessageTrigger {
    /// To trigger a BootNotification request
    BootNotification,
    ///To trigger a DiagnosticsStatusNotification request
    DiagnosticsStatusNotification,
    /// To trigger a FirmwareStatusNotification request
    FirmwareStatusNotification,
    /// To trigger a Heartbeat request
    #[default]
    Heartbeat,
    /// To trigger a MeterValues request
    MeterValues,
    ///  To trigger a StatusNotification request
    StatusNotification,
}
