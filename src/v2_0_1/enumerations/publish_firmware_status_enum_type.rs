#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum PublishFirmwareStatusEnumType {
    #[default]
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
