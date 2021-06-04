#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum ReportBaseEnumType {
    ConfigurationInventory,
    FullInventory,
    SummaryInventory,
}
