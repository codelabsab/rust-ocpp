#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum TransactionEventEnumType {
    Ended,
    #[default]
    Started,
    Updated,
}
