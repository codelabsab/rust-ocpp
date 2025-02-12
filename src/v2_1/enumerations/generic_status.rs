use serde::{Deserialize, Serialize};

/// Status of operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GenericStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
}
