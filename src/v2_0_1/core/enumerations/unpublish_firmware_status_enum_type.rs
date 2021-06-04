#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum UnpublishFirmwareStatusEnumType {
    DownloadOngoing,
    NoFirmware,
    Unpublished,
}
