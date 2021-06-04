use crate::v2_0_1::core::enumerations::data_enum_type::DataEnumType;

/// Fixed read-only parameters of a variable.
/// VariableCharacteristicsType is used by: NotifyReportRequest.ReportDataType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VariableCharacteristicsType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    pub data_type: DataEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_list: Option<String>,
    pub supports_monitoring: bool,
}
