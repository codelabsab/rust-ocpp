/// RelativeTimeIntervalType is used by: Common:SalesTariffEntryType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RelativeTimeIntervalType {
    pub start: i32,
    pub duration: i32,
}
