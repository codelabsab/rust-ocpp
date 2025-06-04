use serde::{Deserialize, Serialize};

/// Status of log file upload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum UploadLogStatusEnumType {
    /// Request contains a bad or incomplete message.
    #[default]
    #[serde(rename = "BadMessage")]
    BadMessage,

    /// Charging Station is idle and can process an upload.
    #[serde(rename = "Idle")]
    Idle,

    /// Charging Station does not support this operation.
    #[serde(rename = "NotSupportedOperation")]
    NotSupportedOperation,

    /// Charging Station has denied permission for this operation.
    #[serde(rename = "PermissionDenied")]
    PermissionDenied,

    /// Log file has been uploaded successfully.
    #[serde(rename = "Uploaded")]
    Uploaded,

    /// Upload of log file failed.
    #[serde(rename = "UploadFailure")]
    UploadFailure,

    /// Log file is being uploaded.
    #[serde(rename = "Uploading")]
    Uploading,

    /// Upload was accepted but was canceled before completion.
    #[serde(rename = "AcceptedCanceled")]
    AcceptedCanceled,
}
