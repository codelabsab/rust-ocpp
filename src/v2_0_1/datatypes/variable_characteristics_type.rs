use rust_decimal::Decimal;

use crate::v2_0_1::enumerations::data_enum_type::DataEnumType;

/// Fixed read-only parameters of a variable.
/// VariableCharacteristicsType is used by: NotifyReportRequest.ReportDataType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VariableCharacteristicsType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    pub data_type: DataEnumType,
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_limit: Option<Decimal>,
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_limit: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_list: Option<String>,
    pub supports_monitoring: bool,
}
