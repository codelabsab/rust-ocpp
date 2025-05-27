use crate::v2_1::datatypes::{ChargingScheduleType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::GenericStatusEnumType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the NotifyEVChargingSchedule request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingScheduleRequest {
    /// Periods contained in the charging profile are relative to this point in time.
    pub time_base: DateTime<Utc>,

    #[validate(nested)]
    pub charging_schedule: ChargingScheduleType,

    /// The charging schedule contained in this notification applies to an EVSE. EvseId must be &gt; 0.
    #[validate(range(min = 1))]
    pub evse_id: i32,

    /// *(2.1)* Id  of the _chargingSchedule_ that EV selected from the provided ChargingProfile.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub selected_charging_schedule_id: Option<i32>,

    /// *(2.1)* True when power tolerance is accepted by EV. This value is taken from EVPowerProfile.PowerToleranceAcceptance in the ISO 15118-20 PowerDeliverReq message..
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_tolerance_acceptance: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyEVChargingScheduleRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `time_base` - Periods contained in the charging profile are relative to this point in time.
    /// * `charging_schedule` - The charging_schedule field
    /// * `evse_id` - The charging schedule contained in this notification applies to an EVSE. EvseId must be &gt; 0.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(time_base: DateTime<Utc>, charging_schedule: ChargingScheduleType, evse_id: i32) -> Self {
        Self {
            time_base,
            charging_schedule,
            evse_id,
            selected_charging_schedule_id: None,
            power_tolerance_acceptance: None,
            custom_data: None,
        }
    }

    /// Sets the time_base field.
    ///
    /// * `time_base` - Periods contained in the charging profile are relative to this point in time.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_time_base(&mut self, time_base: DateTime<Utc>) -> &mut Self {
        self.time_base = time_base;
        self
    }

    /// Sets the charging_schedule field.
    ///
    /// * `charging_schedule` - The charging_schedule field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_schedule(&mut self, charging_schedule: ChargingScheduleType) -> &mut Self {
        self.charging_schedule = charging_schedule;
        self
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - The charging schedule contained in this notification applies to an EVSE. EvseId must be &gt; 0.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: i32) -> &mut Self {
        self.evse_id = evse_id;
        self
    }

    /// Sets the selected_charging_schedule_id field.
    ///
    /// * `selected_charging_schedule_id` - *(2.1)* Id  of the _chargingSchedule_ that EV selected from the provided ChargingProfile.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_selected_charging_schedule_id(&mut self, selected_charging_schedule_id: Option<i32>) -> &mut Self {
        self.selected_charging_schedule_id = selected_charging_schedule_id;
        self
    }

    /// Sets the power_tolerance_acceptance field.
    ///
    /// * `power_tolerance_acceptance` - *(2.1)* True when power tolerance is accepted by EV. This value is taken from EVPowerProfile.PowerToleranceAcceptance in the ISO 15118-20 PowerDeliverReq message..
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_power_tolerance_acceptance(&mut self, power_tolerance_acceptance: Option<bool>) -> &mut Self {
        self.power_tolerance_acceptance = power_tolerance_acceptance;
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

    /// Gets a reference to the time_base field.
    ///
    /// # Returns
    ///
    /// Periods contained in the charging profile are relative to this point in time.
    pub fn get_time_base(&self) -> &DateTime<Utc> {
        &self.time_base
    }

    /// Gets a reference to the charging_schedule field.
    ///
    /// # Returns
    ///
    /// The charging_schedule field
    pub fn get_charging_schedule(&self) -> &ChargingScheduleType {
        &self.charging_schedule
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// The charging schedule contained in this notification applies to an EVSE. EvseId must be &gt; 0.
    pub fn get_evse_id(&self) -> &i32 {
        &self.evse_id
    }

    /// Gets a reference to the selected_charging_schedule_id field.
    ///
    /// # Returns
    ///
    /// *(2.1)* Id  of the _chargingSchedule_ that EV selected from the provided ChargingProfile.
    pub fn get_selected_charging_schedule_id(&self) -> Option<&i32> {
        self.selected_charging_schedule_id.as_ref()
    }

    /// Gets a reference to the power_tolerance_acceptance field.
    ///
    /// # Returns
    ///
    /// *(2.1)* True when power tolerance is accepted by EV. This value is taken from EVPowerProfile.PowerToleranceAcceptance in the ISO 15118-20 PowerDeliverReq message..
    pub fn get_power_tolerance_acceptance(&self) -> Option<&bool> {
        self.power_tolerance_acceptance.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the selected_charging_schedule_id field and returns self for builder pattern.
    ///
    /// * `selected_charging_schedule_id` - *(2.1)* Id  of the _chargingSchedule_ that EV selected from the provided ChargingProfile.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_selected_charging_schedule_id(mut self, selected_charging_schedule_id: i32) -> Self {
        self.selected_charging_schedule_id = Some(selected_charging_schedule_id);
        self
    }

    /// Sets the power_tolerance_acceptance field and returns self for builder pattern.
    ///
    /// * `power_tolerance_acceptance` - *(2.1)* True when power tolerance is accepted by EV. This value is taken from EVPowerProfile.PowerToleranceAcceptance in the ISO 15118-20 PowerDeliverReq message..
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_power_tolerance_acceptance(mut self, power_tolerance_acceptance: bool) -> Self {
        self.power_tolerance_acceptance = Some(power_tolerance_acceptance);
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

/// Response body for the NotifyEVChargingSchedule response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingScheduleResponse {
    pub status: GenericStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyEVChargingScheduleResponse {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::datatypes::ChargingSchedulePeriodType;
    use crate::v2_1::enumerations::ChargingRateUnitEnumType;
    use chrono::Utc;
    use serde_json;
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_charging_schedule() -> ChargingScheduleType {
        let period = ChargingSchedulePeriodType::new_from_f64(0, 16.0);
        ChargingScheduleType::new(1, ChargingRateUnitEnumType::W, vec![period])
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("Test status".to_string())
    }

    #[test]
    fn test_notify_ev_charging_schedule_request_new() {
        let time_base = Utc::now();
        let charging_schedule = create_test_charging_schedule();
        let evse_id = 1;

        let request = NotifyEVChargingScheduleRequest::new(
            time_base,
            charging_schedule.clone(),
            evse_id,
        );

        assert_eq!(request.get_time_base(), &time_base);
        assert_eq!(request.get_charging_schedule(), &charging_schedule);
        assert_eq!(request.get_evse_id(), &evse_id);
        assert_eq!(request.get_selected_charging_schedule_id(), None);
        assert_eq!(request.get_power_tolerance_acceptance(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_notify_ev_charging_schedule_request_validation() {
        let time_base = Utc::now();
        let charging_schedule = create_test_charging_schedule();
        let evse_id = 1;

        let request = NotifyEVChargingScheduleRequest::new(
            time_base,
            charging_schedule,
            evse_id,
        );

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_ev_charging_schedule_request_validation_invalid_evse_id() {
        let time_base = Utc::now();
        let charging_schedule = create_test_charging_schedule();
        let evse_id = 0; // Invalid - must be >= 1

        let request = NotifyEVChargingScheduleRequest::new(
            time_base,
            charging_schedule,
            evse_id,
        );

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_ev_charging_schedule_request_validation_negative_selected_schedule_id() {
        let time_base = Utc::now();
        let charging_schedule = create_test_charging_schedule();
        let evse_id = 1;

        let request = NotifyEVChargingScheduleRequest::new(
            time_base,
            charging_schedule,
            evse_id,
        ).with_selected_charging_schedule_id(-1); // Invalid negative value

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_ev_charging_schedule_request_with_all_optional_fields() {
        let time_base = Utc::now();
        let charging_schedule = create_test_charging_schedule();
        let evse_id = 1;
        let selected_charging_schedule_id = 5;
        let power_tolerance_acceptance = true;
        let custom_data = create_test_custom_data();

        let request = NotifyEVChargingScheduleRequest::new(
            time_base,
            charging_schedule.clone(),
            evse_id,
        )
        .with_selected_charging_schedule_id(selected_charging_schedule_id)
        .with_power_tolerance_acceptance(power_tolerance_acceptance)
        .with_custom_data(custom_data.clone());

        assert_eq!(request.get_time_base(), &time_base);
        assert_eq!(request.get_charging_schedule(), &charging_schedule);
        assert_eq!(request.get_evse_id(), &evse_id);
        assert_eq!(request.get_selected_charging_schedule_id(), Some(&selected_charging_schedule_id));
        assert_eq!(request.get_power_tolerance_acceptance(), Some(&power_tolerance_acceptance));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_ev_charging_schedule_request_set_methods() {
        let time_base = Utc::now();
        let charging_schedule = create_test_charging_schedule();
        let evse_id = 1;

        let mut request = NotifyEVChargingScheduleRequest::new(
            time_base,
            charging_schedule,
            evse_id,
        );

        let new_time_base = Utc::now();
        let new_charging_schedule = create_test_charging_schedule();
        let new_evse_id = 2;
        let selected_charging_schedule_id = 10;
        let power_tolerance_acceptance = false;
        let custom_data = create_test_custom_data();

        request
            .set_time_base(new_time_base)
            .set_charging_schedule(new_charging_schedule.clone())
            .set_evse_id(new_evse_id)
            .set_selected_charging_schedule_id(Some(selected_charging_schedule_id))
            .set_power_tolerance_acceptance(Some(power_tolerance_acceptance))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_time_base(), &new_time_base);
        assert_eq!(request.get_charging_schedule(), &new_charging_schedule);
        assert_eq!(request.get_evse_id(), &new_evse_id);
        assert_eq!(request.get_selected_charging_schedule_id(), Some(&selected_charging_schedule_id));
        assert_eq!(request.get_power_tolerance_acceptance(), Some(&power_tolerance_acceptance));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_ev_charging_schedule_request_json_round_trip() {
        let time_base = Utc::now();
        let charging_schedule = create_test_charging_schedule();
        let evse_id = 1;
        let custom_data = create_test_custom_data();

        let request = NotifyEVChargingScheduleRequest::new(
            time_base,
            charging_schedule,
            evse_id,
        )
        .with_selected_charging_schedule_id(5)
        .with_power_tolerance_acceptance(true)
        .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: NotifyEVChargingScheduleRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_notify_ev_charging_schedule_request_boundary_values() {
        let time_base = Utc::now();
        let charging_schedule = create_test_charging_schedule();

        // Test with minimum valid evse_id
        let evse_id = 1; // Minimum valid value
        let request = NotifyEVChargingScheduleRequest::new(
            time_base,
            charging_schedule.clone(),
            evse_id,
        );

        assert_eq!(request.get_evse_id(), &evse_id);
        assert!(request.validate().is_ok());

        // Test with minimum valid selected_charging_schedule_id
        let request = NotifyEVChargingScheduleRequest::new(
            time_base,
            charging_schedule,
            evse_id,
        ).with_selected_charging_schedule_id(0); // Minimum valid value

        assert_eq!(request.get_selected_charging_schedule_id(), Some(&0));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_ev_charging_schedule_response_new() {
        let status = GenericStatusEnumType::Accepted;

        let response = NotifyEVChargingScheduleResponse::new(status.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_notify_ev_charging_schedule_response_validation() {
        let status = GenericStatusEnumType::Accepted;

        let response = NotifyEVChargingScheduleResponse::new(status);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_notify_ev_charging_schedule_response_with_optional_fields() {
        let status = GenericStatusEnumType::Rejected;
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let response = NotifyEVChargingScheduleResponse::new(status.clone())
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_ev_charging_schedule_response_all_status_types() {
        let statuses = vec![
            GenericStatusEnumType::Accepted,
            GenericStatusEnumType::Rejected,
        ];

        for status in statuses {
            let response = NotifyEVChargingScheduleResponse::new(status.clone());
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_notify_ev_charging_schedule_response_json_round_trip() {
        let status = GenericStatusEnumType::Rejected;
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let response = NotifyEVChargingScheduleResponse::new(status)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: NotifyEVChargingScheduleResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }
}