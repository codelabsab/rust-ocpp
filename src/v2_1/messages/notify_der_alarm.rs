use crate::v2_1::datatypes::CustomDataType;
use crate::v2_1::enumerations::{DERControlEnumType, GridEventFaultEnumType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the NotifyDERAlarm request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDERAlarmRequest {
    pub control_type: DERControlEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_event_fault: Option<GridEventFaultEnumType>,

    /// True when error condition has ended. Absent or false when alarm has started.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_ended: Option<bool>,

    /// Time of start or end of alarm.
    pub timestamp: DateTime<Utc>,

    /// Optional info provided by EV.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 200))]
    pub extra_info: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyDERAlarmRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `control_type` - The control_type field
    /// * `timestamp` - Time of start or end of alarm.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(control_type: DERControlEnumType, timestamp: DateTime<Utc>) -> Self {
        Self {
            control_type,
            grid_event_fault: None,
            alarm_ended: None,
            timestamp,
            extra_info: None,
            custom_data: None,
        }
    }

    /// Sets the control_type field.
    ///
    /// * `control_type` - The control_type field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_control_type(&mut self, control_type: DERControlEnumType) -> &mut Self {
        self.control_type = control_type;
        self
    }

    /// Sets the grid_event_fault field.
    ///
    /// * `grid_event_fault` - The grid_event_fault field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_grid_event_fault(&mut self, grid_event_fault: Option<GridEventFaultEnumType>) -> &mut Self {
        self.grid_event_fault = grid_event_fault;
        self
    }

    /// Sets the alarm_ended field.
    ///
    /// * `alarm_ended` - True when error condition has ended. Absent or false when alarm has started.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_alarm_ended(&mut self, alarm_ended: Option<bool>) -> &mut Self {
        self.alarm_ended = alarm_ended;
        self
    }

    /// Sets the timestamp field.
    ///
    /// * `timestamp` - Time of start or end of alarm.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_timestamp(&mut self, timestamp: DateTime<Utc>) -> &mut Self {
        self.timestamp = timestamp;
        self
    }

    /// Sets the extra_info field.
    ///
    /// * `extra_info` - Optional info provided by EV.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_extra_info(&mut self, extra_info: Option<String>) -> &mut Self {
        self.extra_info = extra_info;
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

    /// Gets a reference to the control_type field.
    ///
    /// # Returns
    ///
    /// The control_type field
    pub fn get_control_type(&self) -> &DERControlEnumType {
        &self.control_type
    }

    /// Gets a reference to the grid_event_fault field.
    ///
    /// # Returns
    ///
    /// The grid_event_fault field
    pub fn get_grid_event_fault(&self) -> Option<&GridEventFaultEnumType> {
        self.grid_event_fault.as_ref()
    }

    /// Gets a reference to the alarm_ended field.
    ///
    /// # Returns
    ///
    /// True when error condition has ended. Absent or false when alarm has started.
    pub fn get_alarm_ended(&self) -> Option<&bool> {
        self.alarm_ended.as_ref()
    }

    /// Gets a reference to the timestamp field.
    ///
    /// # Returns
    ///
    /// Time of start or end of alarm.
    pub fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    /// Gets a reference to the extra_info field.
    ///
    /// # Returns
    ///
    /// Optional info provided by EV.
    pub fn get_extra_info(&self) -> Option<&String> {
        self.extra_info.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the grid_event_fault field and returns self for builder pattern.
    ///
    /// * `grid_event_fault` - The grid_event_fault field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_grid_event_fault(mut self, grid_event_fault: GridEventFaultEnumType) -> Self {
        self.grid_event_fault = Some(grid_event_fault);
        self
    }

    /// Sets the alarm_ended field and returns self for builder pattern.
    ///
    /// * `alarm_ended` - True when error condition has ended. Absent or false when alarm has started.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_alarm_ended(mut self, alarm_ended: bool) -> Self {
        self.alarm_ended = Some(alarm_ended);
        self
    }

    /// Sets the extra_info field and returns self for builder pattern.
    ///
    /// * `extra_info` - Optional info provided by EV.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_extra_info(mut self, extra_info: String) -> Self {
        self.extra_info = Some(extra_info);
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

/// Response body for the NotifyDERAlarm response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDERAlarmResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyDERAlarmResponse {
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
