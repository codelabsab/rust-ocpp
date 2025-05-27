use crate::v2_1::datatypes::{ChargingNeedsType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::NotifyEVChargingNeedsStatusEnumType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the NotifyEVChargingNeeds request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingNeedsRequest {
    /// Defines the EVSE and connector to which the EV is connected. EvseId may not be 0.
    #[validate(range(min = 1))]
    pub evse_id: i32,

    /// Contains the maximum elements the EV supports for: + - ISO 15118-2: schedule tuples in SASchedule (both Pmax and Tariff). + - ISO 15118-20: PowerScheduleEntry, PriceRule and PriceLevelScheduleEntries.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub max_schedule_tuples: Option<i32>,

    #[validate(nested)]
    pub charging_needs: ChargingNeedsType,

    /// *(2.1)* Time when EV charging needs were received. + Field can be added when charging station was offline when charging needs were received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyEVChargingNeedsRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `evse_id` - Defines the EVSE and connector to which the EV is connected. EvseId may not be 0.
    /// * `charging_needs` - The charging_needs field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(evse_id: i32, charging_needs: ChargingNeedsType) -> Self {
        Self {
            evse_id,
            max_schedule_tuples: None,
            charging_needs,
            timestamp: None,
            custom_data: None,
        }
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - Defines the EVSE and connector to which the EV is connected. EvseId may not be 0.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: i32) -> &mut Self {
        self.evse_id = evse_id;
        self
    }

    /// Sets the max_schedule_tuples field.
    ///
    /// * `max_schedule_tuples` - Contains the maximum elements the EV supports for: + - ISO 15118-2: schedule tuples in SASchedule (both Pmax and Tariff). + - ISO 15118-20: PowerScheduleEntry, PriceRule and PriceLevelScheduleEntries.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_max_schedule_tuples(&mut self, max_schedule_tuples: Option<i32>) -> &mut Self {
        self.max_schedule_tuples = max_schedule_tuples;
        self
    }

    /// Sets the charging_needs field.
    ///
    /// * `charging_needs` - The charging_needs field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_needs(&mut self, charging_needs: ChargingNeedsType) -> &mut Self {
        self.charging_needs = charging_needs;
        self
    }

    /// Sets the timestamp field.
    ///
    /// * `timestamp` - *(2.1)* Time when EV charging needs were received. + Field can be added when charging station was offline when charging needs were received.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_timestamp(&mut self, timestamp: Option<DateTime<Utc>>) -> &mut Self {
        self.timestamp = timestamp;
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

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// Defines the EVSE and connector to which the EV is connected. EvseId may not be 0.
    pub fn get_evse_id(&self) -> &i32 {
        &self.evse_id
    }

    /// Gets a reference to the max_schedule_tuples field.
    ///
    /// # Returns
    ///
    /// Contains the maximum elements the EV supports for: + - ISO 15118-2: schedule tuples in SASchedule (both Pmax and Tariff). + - ISO 15118-20: PowerScheduleEntry, PriceRule and PriceLevelScheduleEntries.
    pub fn get_max_schedule_tuples(&self) -> Option<&i32> {
        self.max_schedule_tuples.as_ref()
    }

    /// Gets a reference to the charging_needs field.
    ///
    /// # Returns
    ///
    /// The charging_needs field
    pub fn get_charging_needs(&self) -> &ChargingNeedsType {
        &self.charging_needs
    }

    /// Gets a reference to the timestamp field.
    ///
    /// # Returns
    ///
    /// *(2.1)* Time when EV charging needs were received. + Field can be added when charging station was offline when charging needs were received.
    pub fn get_timestamp(&self) -> Option<&DateTime<Utc>> {
        self.timestamp.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the max_schedule_tuples field and returns self for builder pattern.
    ///
    /// * `max_schedule_tuples` - Contains the maximum elements the EV supports for: + - ISO 15118-2: schedule tuples in SASchedule (both Pmax and Tariff). + - ISO 15118-20: PowerScheduleEntry, PriceRule and PriceLevelScheduleEntries.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_max_schedule_tuples(mut self, max_schedule_tuples: i32) -> Self {
        self.max_schedule_tuples = Some(max_schedule_tuples);
        self
    }

    /// Sets the timestamp field and returns self for builder pattern.
    ///
    /// * `timestamp` - *(2.1)* Time when EV charging needs were received. + Field can be added when charging station was offline when charging needs were received.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = Some(timestamp);
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

/// Response body for the NotifyEVChargingNeeds response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingNeedsResponse {
    pub status: NotifyEVChargingNeedsStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyEVChargingNeedsResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: NotifyEVChargingNeedsStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
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
    pub fn set_status(&mut self, status: NotifyEVChargingNeedsStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &NotifyEVChargingNeedsStatusEnumType {
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
