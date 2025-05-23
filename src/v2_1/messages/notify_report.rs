use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{custom_data::CustomDataType, report_data::ReportDataType};

/// Request to notify the CSMS about a report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyReportRequest {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The id of the GetReportRequest that requested this report.
    pub request_id: i32,

    /// Optional. "to be continued" indicator. Indicates whether another part of the report follows in an upcoming notifyReportRequest message. Default value when omitted is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    /// Required. Sequence number of this message. First message starts at 0.
    #[validate(range(min = 0))]
    pub seq_no: i32,

    /// Required. Timestamp of when this message was generated at the Charging Station.
    pub generated_at: DateTime<Utc>,

    /// Optional. List of report data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub report_data: Option<Vec<ReportDataType>>,
}

/// Response to a NotifyReportRequest. This message has no fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyReportResponse {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyReportRequest {
    /// Creates a new `NotifyReportRequest` with required fields.
    ///
    /// # Arguments
    ///
    /// * `request_id` - The id of the GetReportRequest that requested this report
    /// * `seq_no` - Sequence number of this message
    /// * `generated_at` - Timestamp of when this message was generated
    ///
    /// # Returns
    ///
    /// A new instance of `NotifyReportRequest` with optional fields set to `None`
    pub fn new(request_id: i32, seq_no: i32, generated_at: DateTime<Utc>) -> Self {
        Self {
            custom_data: None,
            request_id,
            tbc: None,
            seq_no,
            generated_at,
            report_data: None,
        }
    }
}

impl NotifyReportResponse {
    /// Creates a new `NotifyReportResponse`.
    ///
    /// # Returns
    ///
    /// A new instance of `NotifyReportResponse` with optional fields set to `None`
    pub fn new() -> Self {
        Self {
            custom_data: None,
        }
    }
}