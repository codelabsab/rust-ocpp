/// Status of a firmware download as reported in FirmwareStatusNotificationRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum FirmwareStatus {
    /// New firmware has been downloaded by Charge Point.
    Downloaded,
    /// Charge point failed to download firmware.
    DownloadFailed,
    /// Firmware is being downloaded.
    Downloading,
    /// Charge Point is not performing firmware update related tasks. Status Idle SHALL only be used as in a FirmwareStatusNotificationRequest that was triggered by a TriggerMessageRequest
    Idle,
    /// Installation of new firmware has failed.
    InstallationFailed,
    /// Firmware is being installed.
    Installing,
    /// New firmware has successfully been installed in charge point.
    Installed,
}
