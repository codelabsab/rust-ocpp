/// Status in DiagnosticsStatusNotificationRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum DiagnosticsStatus {
    /// Charge Point is not performing diagnostics related tasks. Status Idle SHALL only be used as in a DiagnosticsStatusNotification.req that was triggered by a TriggerMessage.req
    Idle,
    /// Diagnostics information has been uploaded.
    Uploaded,
    /// Uploading of diagnostics failed.
    UploadFailed,
    /// File is being uploaded.
    Uploading,
}
