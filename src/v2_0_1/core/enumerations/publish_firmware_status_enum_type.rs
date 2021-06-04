#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum PublishFirmwareStatusEnumType {
    Idle,
    DownloadScheduled,
    Downloading,
    Downloaded,
    Published,
    DownloadFailed,
    DownloadPaused,
    InvalidChecksum,
    ChecksumVerified,
    PublishFailed,
}
