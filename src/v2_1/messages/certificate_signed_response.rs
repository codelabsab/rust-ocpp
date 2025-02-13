use super::super::datatypes::{CustomDataType, StatusInfoType};
use serde::{Deserialize, Serialize};

/// Enum indicating whether certificate signing has been accepted or rejected.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CertificateSignedStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
}

/// Response to a CertificateSignedRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateSignedResponse {
    /// Returns whether certificate signing has been accepted, otherwise rejected.
    pub status: CertificateSignedStatusEnumType,

    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
