#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum UnpublishFirmwareStatusEnumType {
    #[default]
    DownloadOngoing,
    NoFirmware,
    Unpublished,
}
