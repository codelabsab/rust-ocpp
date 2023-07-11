#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ReportBaseEnumType {
    ConfigurationInventory,
    #[default]
    FullInventory,
    SummaryInventory,
}
