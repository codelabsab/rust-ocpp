use crate::v2_1::datatypes::CustomDataType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the NotifyDERStartStop request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDERStartStopRequest {
    /// Id of the started or stopped DER control. Corresponds to the _controlId_ of the SetDERControlRequest.
    #[validate(length(max = 36))]
    pub control_id: String,

    /// True if DER control has started. False if it has ended.
    pub started: bool,

    /// Time of start or end of event.
    pub timestamp: DateTime<Utc>,

    /// List of controlIds that are superseded as a result of this control starting.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    pub superseded_ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyDERStartStopRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `control_id` - Id of the started or stopped DER control. Corresponds to the _controlId_ of the SetDERControlRequest.
    /// * `started` - True if DER control has started. False if it has ended.
    /// * `timestamp` - Time of start or end of event.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(control_id: String, started: bool, timestamp: DateTime<Utc>) -> Self {
        Self {
            control_id,
            started,
            timestamp,
            superseded_ids: None,
            custom_data: None,
        }
    }

    /// Sets the control_id field.
    ///
    /// * `control_id` - Id of the started or stopped DER control. Corresponds to the _controlId_ of the SetDERControlRequest.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_control_id(&mut self, control_id: String) -> &mut Self {
        self.control_id = control_id;
        self
    }

    /// Sets the started field.
    ///
    /// * `started` - True if DER control has started. False if it has ended.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_started(&mut self, started: bool) -> &mut Self {
        self.started = started;
        self
    }

    /// Sets the timestamp field.
    ///
    /// * `timestamp` - Time of start or end of event.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_timestamp(&mut self, timestamp: DateTime<Utc>) -> &mut Self {
        self.timestamp = timestamp;
        self
    }

    /// Sets the superseded_ids field.
    ///
    /// * `superseded_ids` - List of controlIds that are superseded as a result of this control starting.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_superseded_ids(&mut self, superseded_ids: Option<Vec<String>>) -> &mut Self {
        self.superseded_ids = superseded_ids;
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

    /// Gets a reference to the control_id field.
    ///
    /// # Returns
    ///
    /// Id of the started or stopped DER control. Corresponds to the _controlId_ of the SetDERControlRequest.
    pub fn get_control_id(&self) -> &String {
        &self.control_id
    }

    /// Gets a reference to the started field.
    ///
    /// # Returns
    ///
    /// True if DER control has started. False if it has ended.
    pub fn get_started(&self) -> &bool {
        &self.started
    }

    /// Gets a reference to the timestamp field.
    ///
    /// # Returns
    ///
    /// Time of start or end of event.
    pub fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    /// Gets a reference to the superseded_ids field.
    ///
    /// # Returns
    ///
    /// List of controlIds that are superseded as a result of this control starting.
    pub fn get_superseded_ids(&self) -> Option<&Vec<String>> {
        self.superseded_ids.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the superseded_ids field and returns self for builder pattern.
    ///
    /// * `superseded_ids` - List of controlIds that are superseded as a result of this control starting.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_superseded_ids(mut self, superseded_ids: Vec<String>) -> Self {
        self.superseded_ids = Some(superseded_ids);
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

/// Response body for the NotifyDERStartStop response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDERStartStopResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyDERStartStopResponse {
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
