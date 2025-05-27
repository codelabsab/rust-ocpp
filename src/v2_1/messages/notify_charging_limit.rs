use crate::v2_1::datatypes::{ChargingLimitType, ChargingScheduleType, CustomDataType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the NotifyChargingLimit request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyChargingLimitRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub charging_schedule: Option<Vec<ChargingScheduleType>>,

    /// The EVSE to which the charging limit is set. If absent or when zero, it applies to the entire Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    #[validate(nested)]
    pub charging_limit: ChargingLimitType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyChargingLimitRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `charging_limit` - The charging_limit field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(charging_limit: ChargingLimitType) -> Self {
        Self {
            charging_schedule: None,
            evse_id: None,
            charging_limit,
            custom_data: None,
        }
    }

    /// Sets the charging_schedule field.
    ///
    /// * `charging_schedule` - The charging_schedule field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_schedule(&mut self, charging_schedule: Option<Vec<ChargingScheduleType>>) -> &mut Self {
        self.charging_schedule = charging_schedule;
        self
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - The EVSE to which the charging limit is set. If absent or when zero, it applies to the entire Charging Station.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: Option<i32>) -> &mut Self {
        self.evse_id = evse_id;
        self
    }

    /// Sets the charging_limit field.
    ///
    /// * `charging_limit` - The charging_limit field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_limit(&mut self, charging_limit: ChargingLimitType) -> &mut Self {
        self.charging_limit = charging_limit;
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

    /// Gets a reference to the charging_schedule field.
    ///
    /// # Returns
    ///
    /// The charging_schedule field
    pub fn get_charging_schedule(&self) -> Option<&Vec<ChargingScheduleType>> {
        self.charging_schedule.as_ref()
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// The EVSE to which the charging limit is set. If absent or when zero, it applies to the entire Charging Station.
    pub fn get_evse_id(&self) -> Option<&i32> {
        self.evse_id.as_ref()
    }

    /// Gets a reference to the charging_limit field.
    ///
    /// # Returns
    ///
    /// The charging_limit field
    pub fn get_charging_limit(&self) -> &ChargingLimitType {
        &self.charging_limit
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the charging_schedule field and returns self for builder pattern.
    ///
    /// * `charging_schedule` - The charging_schedule field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_charging_schedule(mut self, charging_schedule: Vec<ChargingScheduleType>) -> Self {
        self.charging_schedule = Some(charging_schedule);
        self
    }

    /// Sets the evse_id field and returns self for builder pattern.
    ///
    /// * `evse_id` - The EVSE to which the charging limit is set. If absent or when zero, it applies to the entire Charging Station.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_evse_id(mut self, evse_id: i32) -> Self {
        self.evse_id = Some(evse_id);
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

/// Response body for the NotifyChargingLimit response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyChargingLimitResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyChargingLimitResponse {
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
