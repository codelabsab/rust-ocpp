use crate::v2_1::datatypes::{CustomDataType, ReportDataType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the NotifyReport request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyReportRequest {
    /// The id of the GetReportRequest  or GetBaseReportRequest that requested this report
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// Timestamp of the moment this message was generated at the Charging Station.
    pub generated_at: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub report_data: Option<Vec<ReportDataType>>,

    /// “to be continued” indicator. Indicates whether another part of the report follows in an upcoming notifyReportRequest message. Default value when omitted is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    /// Sequence number of this message. First message starts at 0.
    #[validate(range(min = 0))]
    pub seq_no: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyReportRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `request_id` - The id of the GetReportRequest  or GetBaseReportRequest that requested this report
    /// * `generated_at` - Timestamp of the moment this message was generated at the Charging Station.
    /// * `seq_no` - Sequence number of this message. First message starts at 0.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(request_id: i32, generated_at: DateTime<Utc>, seq_no: i32) -> Self {
        Self {
            request_id,
            generated_at,
            report_data: None,
            tbc: None,
            seq_no,
            custom_data: None,
        }
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - The id of the GetReportRequest  or GetBaseReportRequest that requested this report
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Sets the generated_at field.
    ///
    /// * `generated_at` - Timestamp of the moment this message was generated at the Charging Station.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_generated_at(&mut self, generated_at: DateTime<Utc>) -> &mut Self {
        self.generated_at = generated_at;
        self
    }

    /// Sets the report_data field.
    ///
    /// * `report_data` - The report_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_report_data(&mut self, report_data: Option<Vec<ReportDataType>>) -> &mut Self {
        self.report_data = report_data;
        self
    }

    /// Sets the tbc field.
    ///
    /// * `tbc` - “to be continued” indicator. Indicates whether another part of the report follows in an upcoming notifyReportRequest message. Default value when omitted is false.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_tbc(&mut self, tbc: Option<bool>) -> &mut Self {
        self.tbc = tbc;
        self
    }

    /// Sets the seq_no field.
    ///
    /// * `seq_no` - Sequence number of this message. First message starts at 0.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_seq_no(&mut self, seq_no: i32) -> &mut Self {
        self.seq_no = seq_no;
        self
    }

    /// Sets the custom_data field.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// The id of the GetReportRequest  or GetBaseReportRequest that requested this report
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the generated_at field.
    ///
    /// # Returns
    ///
    /// Timestamp of the moment this message was generated at the Charging Station.
    pub fn get_generated_at(&self) -> &DateTime<Utc> {
        &self.generated_at
    }

    /// Gets a reference to the report_data field.
    ///
    /// # Returns
    ///
    /// The report_data field
    pub fn get_report_data(&self) -> Option<&Vec<ReportDataType>> {
        self.report_data.as_ref()
    }

    /// Gets a reference to the tbc field.
    ///
    /// # Returns
    ///
    /// “to be continued” indicator. Indicates whether another part of the report follows in an upcoming notifyReportRequest message. Default value when omitted is false.
    pub fn get_tbc(&self) -> Option<&bool> {
        self.tbc.as_ref()
    }

    /// Gets a reference to the seq_no field.
    ///
    /// # Returns
    ///
    /// Sequence number of this message. First message starts at 0.
    pub fn get_seq_no(&self) -> &i32 {
        &self.seq_no
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the report_data field and returns self for builder pattern.
    ///
    /// * `report_data` - The report_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_report_data(mut self, report_data: Vec<ReportDataType>) -> Self {
        self.report_data = Some(report_data);
        self
    }

    /// Sets the tbc field and returns self for builder pattern.
    ///
    /// * `tbc` - “to be continued” indicator. Indicates whether another part of the report follows in an upcoming notifyReportRequest message. Default value when omitted is false.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_tbc(mut self, tbc: bool) -> Self {
        self.tbc = Some(tbc);
        self
    }

    /// Sets the custom_data field and returns self for builder pattern.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

}

/// Response body for the NotifyReport response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyReportResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyReportResponse {
    /// Creates a new instance of the struct.
    ///
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new() -> Self {
        Self {
            custom_data: None,
        }
    }

    /// Sets the custom_data field.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom_data field and returns self for builder pattern.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

}
