#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum TransactionEventEnumType {
    Ended,
    Started,
    Updated,
}
