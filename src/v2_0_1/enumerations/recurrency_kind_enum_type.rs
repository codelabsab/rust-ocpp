#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum RecurrencyKindEnumType {
    #[default]
    Daily,
    Weekly,
}
