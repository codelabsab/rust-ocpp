#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum UnpublishFirmwareStatusEnumType {
    DownloadOngoing,
    NoFirmware,
    Unpublished,
}
