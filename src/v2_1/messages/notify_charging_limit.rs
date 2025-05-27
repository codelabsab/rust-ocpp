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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::datatypes::ChargingSchedulePeriodType;
    use crate::v2_1::enumerations::{ChargingLimitSourceEnumType, ChargingRateUnitEnumType};
    use crate::v2_1::enumerations::charging_limit_source::StandardChargingLimitSourceEnumType;
    use serde_json;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_charging_limit() -> ChargingLimitType {
        ChargingLimitType::new(ChargingLimitSourceEnumType::Standard(StandardChargingLimitSourceEnumType::EMS))
    }

    fn create_test_charging_schedule_period() -> ChargingSchedulePeriodType {
        ChargingSchedulePeriodType::new_from_f64(0, 16.0)
    }

    fn create_test_charging_schedule() -> ChargingScheduleType {
        let period = create_test_charging_schedule_period();
        ChargingScheduleType::new(1, ChargingRateUnitEnumType::A, vec![period])
    }

    // Tests for NotifyChargingLimitRequest

    #[test]
    fn test_notify_charging_limit_request_new() {
        let charging_limit = create_test_charging_limit();
        let request = NotifyChargingLimitRequest::new(charging_limit.clone());

        assert_eq!(request.charging_schedule, None);
        assert_eq!(request.evse_id, None);
        assert_eq!(request.charging_limit, charging_limit);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_notify_charging_limit_request_with_charging_schedule() {
        let charging_limit = create_test_charging_limit();
        let charging_schedule = vec![create_test_charging_schedule()];
        let request = NotifyChargingLimitRequest::new(charging_limit.clone())
            .with_charging_schedule(charging_schedule.clone());

        assert_eq!(request.charging_schedule, Some(charging_schedule));
        assert_eq!(request.evse_id, None);
        assert_eq!(request.charging_limit, charging_limit);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_notify_charging_limit_request_with_evse_id() {
        let charging_limit = create_test_charging_limit();
        let request = NotifyChargingLimitRequest::new(charging_limit.clone())
            .with_evse_id(1);

        assert_eq!(request.charging_schedule, None);
        assert_eq!(request.evse_id, Some(1));
        assert_eq!(request.charging_limit, charging_limit);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_notify_charging_limit_request_with_evse_id_zero() {
        let charging_limit = create_test_charging_limit();
        let request = NotifyChargingLimitRequest::new(charging_limit.clone())
            .with_evse_id(0);

        assert_eq!(request.charging_schedule, None);
        assert_eq!(request.evse_id, Some(0));
        assert_eq!(request.charging_limit, charging_limit);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_notify_charging_limit_request_with_custom_data() {
        let charging_limit = create_test_charging_limit();
        let custom_data = create_test_custom_data();
        let request = NotifyChargingLimitRequest::new(charging_limit.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.charging_schedule, None);
        assert_eq!(request.evse_id, None);
        assert_eq!(request.charging_limit, charging_limit);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_notify_charging_limit_request_setters() {
        let charging_limit1 = create_test_charging_limit();
        let charging_limit2 = ChargingLimitType::new(ChargingLimitSourceEnumType::Standard(StandardChargingLimitSourceEnumType::SO));
        let charging_schedule = vec![create_test_charging_schedule()];
        let custom_data = create_test_custom_data();

        let mut request = NotifyChargingLimitRequest::new(charging_limit1);
        request.set_charging_schedule(Some(charging_schedule.clone()));
        request.set_evse_id(Some(2));
        request.set_charging_limit(charging_limit2.clone());
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.charging_schedule, Some(charging_schedule));
        assert_eq!(request.evse_id, Some(2));
        assert_eq!(request.charging_limit, charging_limit2);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_notify_charging_limit_request_getters() {
        let charging_limit = create_test_charging_limit();
        let charging_schedule = vec![create_test_charging_schedule()];
        let custom_data = create_test_custom_data();
        let request = NotifyChargingLimitRequest::new(charging_limit.clone())
            .with_charging_schedule(charging_schedule.clone())
            .with_evse_id(3)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_charging_schedule(), Some(&charging_schedule));
        assert_eq!(request.get_evse_id(), Some(&3));
        assert_eq!(request.get_charging_limit(), &charging_limit);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_charging_limit_request_serialization() {
        let charging_limit = create_test_charging_limit();
        let request = NotifyChargingLimitRequest::new(charging_limit);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: NotifyChargingLimitRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
    }

    #[test]
    fn test_notify_charging_limit_request_validation() {
        let charging_limit = create_test_charging_limit();
        let request = NotifyChargingLimitRequest::new(charging_limit);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_charging_limit_request_validation_negative_evse_id() {
        let charging_limit = create_test_charging_limit();
        let mut request = NotifyChargingLimitRequest::new(charging_limit);
        request.set_evse_id(Some(-1));

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_charging_limit_request_validation_empty_charging_schedule() {
        let charging_limit = create_test_charging_limit();
        let mut request = NotifyChargingLimitRequest::new(charging_limit);
        request.set_charging_schedule(Some(vec![])); // Empty list should fail validation

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_charging_limit_request_multiple_charging_schedules() {
        let charging_limit = create_test_charging_limit();
        let charging_schedules = vec![
            create_test_charging_schedule(),
            create_test_charging_schedule(),
        ];
        let request = NotifyChargingLimitRequest::new(charging_limit)
            .with_charging_schedule(charging_schedules.clone());

        assert_eq!(request.charging_schedule.as_ref().unwrap().len(), 2);
        assert_eq!(request.charging_schedule, Some(charging_schedules));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_charging_limit_request_all_charging_limit_sources() {
        let sources = vec![
            ChargingLimitSourceEnumType::Standard(StandardChargingLimitSourceEnumType::EMS),
            ChargingLimitSourceEnumType::Standard(StandardChargingLimitSourceEnumType::Other),
            ChargingLimitSourceEnumType::Standard(StandardChargingLimitSourceEnumType::SO),
            ChargingLimitSourceEnumType::Standard(StandardChargingLimitSourceEnumType::CSO),
        ];

        for source in sources {
            let charging_limit = ChargingLimitType::new(source.clone());
            let request = NotifyChargingLimitRequest::new(charging_limit.clone());
            assert_eq!(request.charging_limit, charging_limit);
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_notify_charging_limit_request_json_round_trip() {
        let charging_limit = create_test_charging_limit();
        let charging_schedule = vec![create_test_charging_schedule()];
        let custom_data = create_test_custom_data();
        let request = NotifyChargingLimitRequest::new(charging_limit)
            .with_charging_schedule(charging_schedule)
            .with_evse_id(1)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: NotifyChargingLimitRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
        assert!(parsed.validate().is_ok());
    }

    // Tests for NotifyChargingLimitResponse

    #[test]
    fn test_notify_charging_limit_response_new() {
        let response = NotifyChargingLimitResponse::new();

        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_notify_charging_limit_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = NotifyChargingLimitResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_notify_charging_limit_response_setters() {
        let custom_data = create_test_custom_data();

        let mut response = NotifyChargingLimitResponse::new();
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_notify_charging_limit_response_getters() {
        let custom_data = create_test_custom_data();
        let response = NotifyChargingLimitResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_charging_limit_response_serialization() {
        let response = NotifyChargingLimitResponse::new();

        let json = serde_json::to_string(&response).unwrap();
        let parsed: NotifyChargingLimitResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
    }

    #[test]
    fn test_notify_charging_limit_response_validation() {
        let response = NotifyChargingLimitResponse::new();

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_notify_charging_limit_response_json_round_trip() {
        let custom_data = create_test_custom_data();
        let response = NotifyChargingLimitResponse::new()
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: NotifyChargingLimitResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
        assert!(parsed.validate().is_ok());
    }

    #[test]
    fn test_notify_charging_limit_response_clear_custom_data() {
        let custom_data = create_test_custom_data();
        let mut response = NotifyChargingLimitResponse::new()
            .with_custom_data(custom_data);

        assert!(response.custom_data.is_some());

        response.set_custom_data(None);
        assert_eq!(response.custom_data, None);
    }
}