use crate::v2_1::datatypes::{CompositeScheduleType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{ChargingRateUnitEnumType, GenericStatusEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetCompositeSchedule request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleRequest {
    /// Length of the requested schedule in seconds.
    pub duration: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_rate_unit: Option<ChargingRateUnitEnumType>,

    /// The ID of the EVSE for which the schedule is requested. When evseid=0, the Charging Station will calculate the expected consumption for the grid connection.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetCompositeScheduleRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `duration` - Length of the requested schedule in seconds.
    /// * `evse_id` - The ID of the EVSE for which the schedule is requested. When evseid=0, the Charging Station will calculate the expected consumption for the grid connection.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(duration: i32, evse_id: i32) -> Self {
        Self {
            duration,
            charging_rate_unit: None,
            evse_id,
            custom_data: None,
        }
    }

    /// Sets the duration field.
    ///
    /// * `duration` - Length of the requested schedule in seconds.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_duration(&mut self, duration: i32) -> &mut Self {
        self.duration = duration;
        self
    }

    /// Sets the charging_rate_unit field.
    ///
    /// * `charging_rate_unit` - The charging_rate_unit field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_rate_unit(&mut self, charging_rate_unit: Option<ChargingRateUnitEnumType>) -> &mut Self {
        self.charging_rate_unit = charging_rate_unit;
        self
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - The ID of the EVSE for which the schedule is requested. When evseid=0, the Charging Station will calculate the expected consumption for the grid connection.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: i32) -> &mut Self {
        self.evse_id = evse_id;
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

    /// Gets a reference to the duration field.
    ///
    /// # Returns
    ///
    /// Length of the requested schedule in seconds.
    pub fn get_duration(&self) -> &i32 {
        &self.duration
    }

    /// Gets a reference to the charging_rate_unit field.
    ///
    /// # Returns
    ///
    /// The charging_rate_unit field
    pub fn get_charging_rate_unit(&self) -> Option<&ChargingRateUnitEnumType> {
        self.charging_rate_unit.as_ref()
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// The ID of the EVSE for which the schedule is requested. When evseid=0, the Charging Station will calculate the expected consumption for the grid connection.
    pub fn get_evse_id(&self) -> &i32 {
        &self.evse_id
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the charging_rate_unit field and returns self for builder pattern.
    ///
    /// * `charging_rate_unit` - The charging_rate_unit field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_charging_rate_unit(mut self, charging_rate_unit: ChargingRateUnitEnumType) -> Self {
        self.charging_rate_unit = Some(charging_rate_unit);
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

/// Response body for the GetCompositeSchedule response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleResponse {
    pub status: GenericStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub schedule: Option<CompositeScheduleType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetCompositeScheduleResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: GenericStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
            schedule: None,
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
    pub fn set_status(&mut self, status: GenericStatusEnumType) -> &mut Self {
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

    /// Sets the schedule field.
    ///
    /// * `schedule` - The schedule field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_schedule(&mut self, schedule: Option<CompositeScheduleType>) -> &mut Self {
        self.schedule = schedule;
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
    pub fn get_status(&self) -> &GenericStatusEnumType {
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

    /// Gets a reference to the schedule field.
    ///
    /// # Returns
    ///
    /// The schedule field
    pub fn get_schedule(&self) -> Option<&CompositeScheduleType> {
        self.schedule.as_ref()
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

    /// Sets the schedule field and returns self for builder pattern.
    ///
    /// * `schedule` - The schedule field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_schedule(mut self, schedule: CompositeScheduleType) -> Self {
        self.schedule = Some(schedule);
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
