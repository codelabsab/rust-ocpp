#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum TransactionEventEnumType {
    Ended,
    Started,
    Updated,
}
