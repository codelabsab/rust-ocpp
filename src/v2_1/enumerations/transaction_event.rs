use serde::{Deserialize, Serialize};

/// Type of event for a transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum TransactionEventEnumType {
    /// Transaction has ended.
    #[default]
    #[serde(rename = "Ended")]
    Ended,

    /// Transaction has started.
    #[serde(rename = "Started")]
    Started,

    /// Transaction information has been updated.
    #[serde(rename = "Updated")]
    Updated,
}
