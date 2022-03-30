#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ReportBaseEnumType {
    ConfigurationInventory,
    FullInventory,
    SummaryInventory,
}
