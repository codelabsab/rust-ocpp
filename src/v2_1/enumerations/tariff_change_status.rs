use serde::{Deserialize, Serialize};

/// Status of changing a tariff.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum TariffChangeStatusEnumType {
    /// Request has been accepted and the tariff has been changed.
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
