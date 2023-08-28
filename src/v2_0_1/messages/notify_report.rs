use chrono::DateTime;
use chrono::Utc;

use crate::v2_0_1::datatypes::report_data_type::ReportDataType;
use crate::Vec;

/// This contains the field definition of the NotifyReportRequest PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyReportRequest<'a, const N_REPORT_DATA: usize, const N_VARIABLE_ATTRIBUTES: usize> {
    pub request_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
    pub seq_no: i64,
    pub generated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none",borrow)]
    pub report_data: Option<Vec<ReportDataType<'a, N_VARIABLE_ATTRIBUTES>, N_REPORT_DATA>>,
}

/// Response to a NotifyReportRequest message. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyReportResponse {}
