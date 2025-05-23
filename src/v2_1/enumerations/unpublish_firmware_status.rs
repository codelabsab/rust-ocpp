use serde::{Deserialize, Serialize};

/// Status of unpublishing a firmware.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum UnpublishFirmwareStatusEnumType {
    /// Firmware download is ongoing.
    #[default]
    #[serde(rename = "DownloadOngoing")]
    DownloadOngoing,

    /// No firmware with the given ID was found.
    #[serde(rename = "NoFirmware")]
    NoFirmware,

    /// Firmware was successfully unpublished.
    #[serde(rename = "Unpublished")]
    Unpublished,
}
