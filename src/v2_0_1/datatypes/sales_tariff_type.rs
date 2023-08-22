use super::sales_tariff_entry_type::SalesTariffEntryType;
#[cfg(feature = "std")]
use validator::Validate;

/// This dataType is based on dataTypes from ISO 15118-2.
/// SalesTariffType is used by: Common:ChargingScheduleType
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffType<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_tariff_description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_e_price_levels: Option<i64>,
    #[cfg_attr(feature="std", validate(length(min = 1, max = 1024)))]
    pub sales_tariff_entry: Vec<SalesTariffEntryType>,
}
