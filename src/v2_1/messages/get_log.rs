use crate::v2_1::datatypes::{CustomDataType, LogParametersType, StatusInfoType};
use crate::v2_1::enumerations::{LogEnumType, LogStatusEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetLog request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetLogRequest {
    #[validate(nested)]
    pub log: LogParametersType,

    pub log_type: LogEnumType,

    /// The Id of this request
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// This specifies how many times the Charging Station must retry to upload the log before giving up. If this field is not present, it is left to Charging Station to decide how many times it wants to retry. If the value is 0, it means: no retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub retries: Option<i32>,

    /// The interval in seconds after which a retry may be attempted. If this field is not present, it is left to Charging Station to decide how long to wait between attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetLogRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `log` - The log field
    /// * `log_type` - The log_type field
    /// * `request_id` - The Id of this request
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(log: LogParametersType, log_type: LogEnumType, request_id: i32) -> Self {
        Self {
            log,
            log_type,
            request_id,
            retries: None,
            retry_interval: None,
            custom_data: None,
        }
    }

    /// Sets the log field.
    ///
    /// * `log` - The log field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_log(&mut self, log: LogParametersType) -> &mut Self {
        self.log = log;
        self
    }

    /// Sets the log_type field.
    ///
    /// * `log_type` - The log_type field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_log_type(&mut self, log_type: LogEnumType) -> &mut Self {
        self.log_type = log_type;
        self
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - The Id of this request
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Sets the retries field.
    ///
    /// * `retries` - This specifies how many times the Charging Station must retry to upload the log before giving up. If this field is not present, it is left to Charging Station to decide how many times it wants to retry. If the value is 0, it means: no retries.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_retries(&mut self, retries: Option<i32>) -> &mut Self {
        self.retries = retries;
        self
    }

    /// Sets the retry_interval field.
    ///
    /// * `retry_interval` - The interval in seconds after which a retry may be attempted. If this field is not present, it is left to Charging Station to decide how long to wait between attempts.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_retry_interval(&mut self, retry_interval: Option<i32>) -> &mut Self {
        self.retry_interval = retry_interval;
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

    /// Gets a reference to the log field.
    ///
    /// # Returns
    ///
    /// The log field
    pub fn get_log(&self) -> &LogParametersType {
        &self.log
    }

    /// Gets a reference to the log_type field.
    ///
    /// # Returns
    ///
    /// The log_type field
    pub fn get_log_type(&self) -> &LogEnumType {
        &self.log_type
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// The Id of this request
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the retries field.
    ///
    /// # Returns
    ///
    /// This specifies how many times the Charging Station must retry to upload the log before giving up. If this field is not present, it is left to Charging Station to decide how many times it wants to retry. If the value is 0, it means: no retries.
    pub fn get_retries(&self) -> Option<&i32> {
        self.retries.as_ref()
    }

    /// Gets a reference to the retry_interval field.
    ///
    /// # Returns
    ///
    /// The interval in seconds after which a retry may be attempted. If this field is not present, it is left to Charging Station to decide how long to wait between attempts.
    pub fn get_retry_interval(&self) -> Option<&i32> {
        self.retry_interval.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the retries field and returns self for builder pattern.
    ///
    /// * `retries` - This specifies how many times the Charging Station must retry to upload the log before giving up. If this field is not present, it is left to Charging Station to decide how many times it wants to retry. If the value is 0, it means: no retries.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_retries(mut self, retries: i32) -> Self {
        self.retries = Some(retries);
        self
    }

    /// Sets the retry_interval field and returns self for builder pattern.
    ///
    /// * `retry_interval` - The interval in seconds after which a retry may be attempted. If this field is not present, it is left to Charging Station to decide how long to wait between attempts.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_retry_interval(mut self, retry_interval: i32) -> Self {
        self.retry_interval = Some(retry_interval);
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

/// Response body for the GetLog response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetLogResponse {
    pub status: LogStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// This contains the name of the log file that will be uploaded. This field is not present when no logging information is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub filename: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetLogResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: LogStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
            filename: None,
            custom_data: None,
        }
    }

    /// Sets the status field.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_status(&mut self, status: LogStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Sets the status_info field.
    ///
    /// * `status_info` - The status_info field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_status_info(&mut self, status_info: Option<StatusInfoType>) -> &mut Self {
        self.status_info = status_info;
        self
    }

    /// Sets the filename field.
    ///
    /// * `filename` - This contains the name of the log file that will be uploaded. This field is not present when no logging information is available.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_filename(&mut self, filename: Option<String>) -> &mut Self {
        self.filename = filename;
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

    /// Gets a reference to the status field.
    ///
    /// # Returns
    ///
    /// The status field
    pub fn get_status(&self) -> &LogStatusEnumType {
        &self.status
    }

    /// Gets a reference to the status_info field.
    ///
    /// # Returns
    ///
    /// The status_info field
    pub fn get_status_info(&self) -> Option<&StatusInfoType> {
        self.status_info.as_ref()
    }

    /// Gets a reference to the filename field.
    ///
    /// # Returns
    ///
    /// This contains the name of the log file that will be uploaded. This field is not present when no logging information is available.
    pub fn get_filename(&self) -> Option<&String> {
        self.filename.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the status_info field and returns self for builder pattern.
    ///
    /// * `status_info` - The status_info field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_status_info(mut self, status_info: StatusInfoType) -> Self {
        self.status_info = Some(status_info);
        self
    }

    /// Sets the filename field and returns self for builder pattern.
    ///
    /// * `filename` - This contains the name of the log file that will be uploaded. This field is not present when no logging information is available.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_filename(mut self, filename: String) -> Self {
        self.filename = Some(filename);
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
