use serde::{Deserialize, Serialize};

/// Status of clearing a tariff.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum TariffClearStatusEnumType {
    /// Request has been accepted and the tariff has been cleared.
    #[default]
    #[serde(rename = "Accepted")]
    Accepted,

    /// Request has been rejected.
    #[serde(rename = "Rejected")]
    Rejected,

    /// Tariff ID specified in the request is invalid.
    #[serde(rename = "InvalidId")]
    InvalidId,
}
