use serde::{Deserialize, Serialize};

/// Status of firmware update.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum UpdateFirmwareStatusEnumType {
    /// Request has been accepted and will be processed.
    #[default]
    #[serde(rename = "Accepted")]
    Accepted,

    /// Request has been rejected.
    #[serde(rename = "Rejected")]
    Rejected,

    /// Request has been accepted but was canceled.
    #[serde(rename = "AcceptedCanceled")]
    AcceptedCanceled,

    /// Certificate is invalid.
    #[serde(rename = "InvalidCertificate")]
    InvalidCertificate,

    /// Certificate has been revoked.
    #[serde(rename = "RevokedCertificate")]
    RevokedCertificate,
}
