/// RelativeTimeIntervalType is used by: Common:SalesTariffEntryType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RelativeTimeIntervalType {
    pub start: i64,
    pub duration: i64,
}
