use crate::v2_1::datatypes::CustomDataType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the NotifyCustomerInformation request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyCustomerInformationRequest {
    /// (Part of) the requested data. No format specified in which the data is returned. Should be human readable.
    #[validate(length(max = 512))]
    pub data: String,

    /// “to be continued” indicator. Indicates whether another part of the monitoringData follows in an upcoming notifyMonitoringReportRequest message. Default value when omitted is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    /// Sequence number of this message. First message starts at 0.
    #[validate(range(min = 0))]
    pub seq_no: i32,

    /// Timestamp of the moment this message was generated at the Charging Station.
    pub generated_at: DateTime<Utc>,

    /// The Id of the request.
    #[validate(range(min = 0))]
    pub request_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyCustomerInformationRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `data` - (Part of) the requested data. No format specified in which the data is returned. Should be human readable.
    /// * `seq_no` - Sequence number of this message. First message starts at 0.
    /// * `generated_at` - Timestamp of the moment this message was generated at the Charging Station.
    /// * `request_id` - The Id of the request.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(data: String, seq_no: i32, generated_at: DateTime<Utc>, request_id: i32) -> Self {
        Self {
            data,
            tbc: None,
            seq_no,
            generated_at,
            request_id,
            custom_data: None,
        }
    }

    /// Sets the data field.
    ///
    /// * `data` - (Part of) the requested data. No format specified in which the data is returned. Should be human readable.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_data(&mut self, data: String) -> &mut Self {
        self.data = data;
        self
    }

    /// Sets the tbc field.
    ///
    /// * `tbc` - “to be continued” indicator. Indicates whether another part of the monitoringData follows in an upcoming notifyMonitoringReportRequest message. Default value when omitted is false.
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

    /// Sets the request_id field.
    ///
    /// * `request_id` - The Id of the request.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
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

    /// Gets a reference to the data field.
    ///
    /// # Returns
    ///
    /// (Part of) the requested data. No format specified in which the data is returned. Should be human readable.
    pub fn get_data(&self) -> &String {
        &self.data
    }

    /// Gets a reference to the tbc field.
    ///
    /// # Returns
    ///
    /// “to be continued” indicator. Indicates whether another part of the monitoringData follows in an upcoming notifyMonitoringReportRequest message. Default value when omitted is false.
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

    /// Gets a reference to the generated_at field.
    ///
    /// # Returns
    ///
    /// Timestamp of the moment this message was generated at the Charging Station.
    pub fn get_generated_at(&self) -> &DateTime<Utc> {
        &self.generated_at
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// The Id of the request.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the tbc field and returns self for builder pattern.
    ///
    /// * `tbc` - “to be continued” indicator. Indicates whether another part of the monitoringData follows in an upcoming notifyMonitoringReportRequest message. Default value when omitted is false.
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

/// Response body for the NotifyCustomerInformation response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyCustomerInformationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyCustomerInformationResponse {
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
