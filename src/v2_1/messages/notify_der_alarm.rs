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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json;
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    #[test]
    fn test_notify_der_alarm_request_new() {
        let control_type = DERControlEnumType::FreqDroop;
        let timestamp = Utc::now();

        let request = NotifyDERAlarmRequest::new(control_type.clone(), timestamp);

        assert_eq!(request.get_control_type(), &control_type);
        assert_eq!(request.get_timestamp(), &timestamp);
        assert_eq!(request.get_grid_event_fault(), None);
        assert_eq!(request.get_alarm_ended(), None);
        assert_eq!(request.get_extra_info(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_notify_der_alarm_request_serialization() {
        let control_type = DERControlEnumType::FreqDroop;
        let timestamp = Utc::now();

        let request = NotifyDERAlarmRequest::new(control_type, timestamp);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: NotifyDERAlarmRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_notify_der_alarm_request_validation() {
        let control_type = DERControlEnumType::FreqDroop;
        let timestamp = Utc::now();

        let request = NotifyDERAlarmRequest::new(control_type, timestamp);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_der_alarm_request_validation_extra_info_too_long() {
        let control_type = DERControlEnumType::FreqDroop;
        let timestamp = Utc::now();
        let extra_info = "x".repeat(201); // Exceeds max length of 200

        let request = NotifyDERAlarmRequest::new(control_type, timestamp)
            .with_extra_info(extra_info);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_der_alarm_request_with_all_optional_fields() {
        let control_type = DERControlEnumType::FreqDroop;
        let timestamp = Utc::now();
        let grid_event_fault = GridEventFaultEnumType::OverVoltage;
        let alarm_ended = true;
        let extra_info = "Test alarm information".to_string();
        let custom_data = create_test_custom_data();

        let request = NotifyDERAlarmRequest::new(control_type.clone(), timestamp)
            .with_grid_event_fault(grid_event_fault.clone())
            .with_alarm_ended(alarm_ended)
            .with_extra_info(extra_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_control_type(), &control_type);
        assert_eq!(request.get_timestamp(), &timestamp);
        assert_eq!(request.get_grid_event_fault(), Some(&grid_event_fault));
        assert_eq!(request.get_alarm_ended(), Some(&alarm_ended));
        assert_eq!(request.get_extra_info(), Some(&extra_info));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_der_alarm_request_set_methods() {
        let control_type = DERControlEnumType::FreqDroop;
        let timestamp = Utc::now();

        let mut request = NotifyDERAlarmRequest::new(control_type, timestamp);

        let new_control_type = DERControlEnumType::VoltVar;
        let new_timestamp = Utc::now();
        let grid_event_fault = GridEventFaultEnumType::UnderVoltage;
        let alarm_ended = false;
        let extra_info = "Updated alarm info".to_string();
        let custom_data = create_test_custom_data();

        request
            .set_control_type(new_control_type.clone())
            .set_timestamp(new_timestamp)
            .set_grid_event_fault(Some(grid_event_fault.clone()))
            .set_alarm_ended(Some(alarm_ended))
            .set_extra_info(Some(extra_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_control_type(), &new_control_type);
        assert_eq!(request.get_timestamp(), &new_timestamp);
        assert_eq!(request.get_grid_event_fault(), Some(&grid_event_fault));
        assert_eq!(request.get_alarm_ended(), Some(&alarm_ended));
        assert_eq!(request.get_extra_info(), Some(&extra_info));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_der_alarm_request_builder_pattern() {
        let control_type = DERControlEnumType::HVMustTrip;
        let timestamp = Utc::now();
        let grid_event_fault = GridEventFaultEnumType::OverFrequency;
        let custom_data = create_test_custom_data();

        let request = NotifyDERAlarmRequest::new(control_type.clone(), timestamp)
            .with_grid_event_fault(grid_event_fault.clone())
            .with_alarm_ended(true)
            .with_extra_info("Builder pattern test".to_string())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_control_type(), &control_type);
        assert_eq!(request.get_grid_event_fault(), Some(&grid_event_fault));
        assert_eq!(request.get_alarm_ended(), Some(&true));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_der_alarm_request_json_round_trip() {
        let control_type = DERControlEnumType::LVMayTrip;
        let timestamp = Utc::now();
        let grid_event_fault = GridEventFaultEnumType::CurrentImbalance;
        let custom_data = create_test_custom_data();

        let request = NotifyDERAlarmRequest::new(control_type, timestamp)
            .with_grid_event_fault(grid_event_fault)
            .with_alarm_ended(false)
            .with_extra_info("JSON round trip test".to_string())
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: NotifyDERAlarmRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_notify_der_alarm_request_all_control_types() {
        let timestamp = Utc::now();
        let control_types = vec![
            DERControlEnumType::EnterService,
            DERControlEnumType::FreqDroop,
            DERControlEnumType::FreqWatt,
            DERControlEnumType::FixedPFAbsorb,
            DERControlEnumType::FixedPFInject,
            DERControlEnumType::FixedVar,
            DERControlEnumType::Gradients,
            DERControlEnumType::HFMustTrip,
            DERControlEnumType::HFMayTrip,
            DERControlEnumType::HVMustTrip,
            DERControlEnumType::HVMomCess,
            DERControlEnumType::HVMayTrip,
            DERControlEnumType::LimitMaxDischarge,
            DERControlEnumType::LFMustTrip,
            DERControlEnumType::LVMustTrip,
            DERControlEnumType::LVMomCess,
            DERControlEnumType::LVMayTrip,
            DERControlEnumType::PowerMonitoringMustTrip,
            DERControlEnumType::VoltVar,
            DERControlEnumType::VoltWatt,
            DERControlEnumType::WattPF,
            DERControlEnumType::WattVar,
        ];

        for control_type in control_types {
            let request = NotifyDERAlarmRequest::new(control_type.clone(), timestamp);
            assert_eq!(request.get_control_type(), &control_type);
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_notify_der_alarm_request_all_grid_event_faults() {
        let control_type = DERControlEnumType::FreqDroop;
        let timestamp = Utc::now();
        let grid_event_faults = vec![
            GridEventFaultEnumType::CurrentImbalance,
            GridEventFaultEnumType::LocalEmergency,
            GridEventFaultEnumType::LowInputPower,
            GridEventFaultEnumType::OverCurrent,
            GridEventFaultEnumType::OverFrequency,
            GridEventFaultEnumType::OverVoltage,
            GridEventFaultEnumType::PhaseRotation,
            GridEventFaultEnumType::RemoteEmergency,
            GridEventFaultEnumType::UnderFrequency,
            GridEventFaultEnumType::UnderVoltage,
            GridEventFaultEnumType::VoltageImbalance,
        ];

        for grid_event_fault in grid_event_faults {
            let request = NotifyDERAlarmRequest::new(control_type.clone(), timestamp)
                .with_grid_event_fault(grid_event_fault.clone());
            assert_eq!(request.get_grid_event_fault(), Some(&grid_event_fault));
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_notify_der_alarm_request_boundary_values() {
        let control_type = DERControlEnumType::FreqDroop;
        let timestamp = Utc::now();

        // Test with maximum valid extra_info length
        let extra_info = "x".repeat(200); // Maximum allowed length
        let request = NotifyDERAlarmRequest::new(control_type, timestamp)
            .with_extra_info(extra_info.clone());

        assert_eq!(request.get_extra_info(), Some(&extra_info));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_der_alarm_response_new() {
        let response = NotifyDERAlarmResponse::new();

        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_notify_der_alarm_response_serialization() {
        let response = NotifyDERAlarmResponse::new();

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: NotifyDERAlarmResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_notify_der_alarm_response_validation() {
        let response = NotifyDERAlarmResponse::new();

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_notify_der_alarm_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = NotifyDERAlarmResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_der_alarm_response_set_custom_data() {
        let mut response = NotifyDERAlarmResponse::new();
        let custom_data = create_test_custom_data();

        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_der_alarm_response_builder_pattern() {
        let custom_data = create_test_custom_data();

        let response = NotifyDERAlarmResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_der_alarm_response_json_round_trip() {
        let custom_data = create_test_custom_data();
        let response = NotifyDERAlarmResponse::new()
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: NotifyDERAlarmResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_notify_der_alarm_response_empty_json() {
        let json = "{}";
        let response: NotifyDERAlarmResponse =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(response.get_custom_data(), None);
        assert!(response.validate().is_ok());
    }
}