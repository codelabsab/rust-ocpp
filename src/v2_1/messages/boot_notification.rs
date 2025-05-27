use crate::v2_1::datatypes::{ChargingStationType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{BootReasonEnumType, RegistrationStatusEnumType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the BootNotification request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    #[validate(nested)]
    pub charging_station: ChargingStationType,

    pub reason: BootReasonEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl BootNotificationRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `charging_station` - The charging_station field
    /// * `reason` - The reason field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(charging_station: ChargingStationType, reason: BootReasonEnumType) -> Self {
        Self {
            charging_station,
            reason,
            custom_data: None,
        }
    }

    /// Sets the charging_station field.
    ///
    /// * `charging_station` - The charging_station field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_station(&mut self, charging_station: ChargingStationType) -> &mut Self {
        self.charging_station = charging_station;
        self
    }

    /// Sets the reason field.
    ///
    /// * `reason` - The reason field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_reason(&mut self, reason: BootReasonEnumType) -> &mut Self {
        self.reason = reason;
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

    /// Gets a reference to the charging_station field.
    ///
    /// # Returns
    ///
    /// The charging_station field
    pub fn get_charging_station(&self) -> &ChargingStationType {
        &self.charging_station
    }

    /// Gets a reference to the reason field.
    ///
    /// # Returns
    ///
    /// The reason field
    pub fn get_reason(&self) -> &BootReasonEnumType {
        &self.reason
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

/// Response body for the BootNotification response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationResponse {
    /// This contains the CSMS’s current time.
    pub current_time: DateTime<Utc>,

    /// When &lt;&lt;cmn_registrationstatusenumtype,Status&gt;&gt; is Accepted, this contains the heartbeat interval in seconds. If the CSMS returns something other than Accepted, the value of the interval field indicates the minimum wait time before sending a next BootNotification request.
    pub interval: i32,

    pub status: RegistrationStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl BootNotificationResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `current_time` - This contains the CSMS’s current time.
    /// * `interval` - When &lt;&lt;cmn_registrationstatusenumtype,Status&gt;&gt; is Accepted, this contains the heartbeat interval in seconds. If the CSMS returns something other than Accepted, the value of the interval field indicates the minimum wait time before sending a next BootNotification request.
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(current_time: DateTime<Utc>, interval: i32, status: RegistrationStatusEnumType) -> Self {
        Self {
            current_time,
            interval,
            status,
            status_info: None,
            custom_data: None,
        }
    }

    /// Sets the current_time field.
    ///
    /// * `current_time` - This contains the CSMS’s current time.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_current_time(&mut self, current_time: DateTime<Utc>) -> &mut Self {
        self.current_time = current_time;
        self
    }

    /// Sets the interval field.
    ///
    /// * `interval` - When &lt;&lt;cmn_registrationstatusenumtype,Status&gt;&gt; is Accepted, this contains the heartbeat interval in seconds. If the CSMS returns something other than Accepted, the value of the interval field indicates the minimum wait time before sending a next BootNotification request.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_interval(&mut self, interval: i32) -> &mut Self {
        self.interval = interval;
        self
    }

    /// Sets the status field.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_status(&mut self, status: RegistrationStatusEnumType) -> &mut Self {
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

    /// Gets a reference to the current_time field.
    ///
    /// # Returns
    ///
    /// This contains the CSMS’s current time.
    pub fn get_current_time(&self) -> &DateTime<Utc> {
        &self.current_time
    }

    /// Gets a reference to the interval field.
    ///
    /// # Returns
    ///
    /// When &lt;&lt;cmn_registrationstatusenumtype,Status&gt;&gt; is Accepted, this contains the heartbeat interval in seconds. If the CSMS returns something other than Accepted, the value of the interval field indicates the minimum wait time before sending a next BootNotification request.
    pub fn get_interval(&self) -> &i32 {
        &self.interval
    }

    /// Gets a reference to the status field.
    ///
    /// # Returns
    ///
    /// The status field
    pub fn get_status(&self) -> &RegistrationStatusEnumType {
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
    use chrono::Utc;
    use serde_json;
    use validator::Validate;

    fn create_test_charging_station() -> ChargingStationType {
        ChargingStationType::new("TestModel".to_string(), "TestVendor".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("200".to_string())
    }

    #[test]
    fn test_boot_notification_request_new() {
        let charging_station = create_test_charging_station();
        let reason = BootReasonEnumType::PowerUp;
        let request = BootNotificationRequest::new(charging_station.clone(), reason.clone());

        assert_eq!(request.get_charging_station(), &charging_station);
        assert_eq!(request.get_reason(), &reason);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_boot_notification_request_validation() {
        let charging_station = create_test_charging_station();
        let reason = BootReasonEnumType::PowerUp;
        let request = BootNotificationRequest::new(charging_station, reason);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_boot_notification_request_serialization() {
        let charging_station = create_test_charging_station();
        let reason = BootReasonEnumType::PowerUp;
        let request = BootNotificationRequest::new(charging_station, reason);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: BootNotificationRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_boot_notification_request_with_custom_data() {
        let charging_station = create_test_charging_station();
        let reason = BootReasonEnumType::PowerUp;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = BootNotificationRequest::new(charging_station, reason)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_boot_notification_request_set_methods() {
        let charging_station = create_test_charging_station();
        let new_charging_station = ChargingStationType::new("NewModel".to_string(), "NewVendor".to_string());
        let reason = BootReasonEnumType::PowerUp;
        let new_reason = BootReasonEnumType::FirmwareUpdate;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = BootNotificationRequest::new(charging_station, reason);

        request
            .set_charging_station(new_charging_station.clone())
            .set_reason(new_reason.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_charging_station(), &new_charging_station);
        assert_eq!(request.get_reason(), &new_reason);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_boot_notification_request_all_boot_reasons() {
        let charging_station = create_test_charging_station();

        let boot_reasons = vec![
            BootReasonEnumType::ApplicationReset,
            BootReasonEnumType::FirmwareUpdate,
            BootReasonEnumType::LocalReset,
            BootReasonEnumType::PowerUp,
            BootReasonEnumType::RemoteReset,
            BootReasonEnumType::ScheduledReset,
            BootReasonEnumType::Triggered,
            BootReasonEnumType::Unknown,
            BootReasonEnumType::Watchdog,
        ];

        for reason in boot_reasons {
            let request = BootNotificationRequest::new(charging_station.clone(), reason.clone());
            assert_eq!(request.get_reason(), &reason);
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_boot_notification_response_new() {
        let current_time = Utc::now();
        let interval = 300;
        let status = RegistrationStatusEnumType::Accepted;
        let response = BootNotificationResponse::new(current_time, interval, status.clone());

        assert_eq!(response.get_current_time(), &current_time);
        assert_eq!(response.get_interval(), &interval);
        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_boot_notification_response_validation() {
        let current_time = Utc::now();
        let interval = 300;
        let status = RegistrationStatusEnumType::Accepted;
        let response = BootNotificationResponse::new(current_time, interval, status);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_boot_notification_response_serialization() {
        let current_time = Utc::now();
        let interval = 300;
        let status = RegistrationStatusEnumType::Accepted;
        let response = BootNotificationResponse::new(current_time, interval, status);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: BootNotificationResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_boot_notification_response_with_status_info() {
        let current_time = Utc::now();
        let interval = 300;
        let status = RegistrationStatusEnumType::Accepted;
        let status_info = create_test_status_info();

        let response = BootNotificationResponse::new(current_time, interval, status)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
    }

    #[test]
    fn test_boot_notification_response_with_custom_data() {
        let current_time = Utc::now();
        let interval = 300;
        let status = RegistrationStatusEnumType::Accepted;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = BootNotificationResponse::new(current_time, interval, status)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_boot_notification_response_set_methods() {
        let current_time = Utc::now();
        let new_time = Utc::now();
        let interval = 300;
        let new_interval = 600;
        let status = RegistrationStatusEnumType::Accepted;
        let new_status = RegistrationStatusEnumType::Pending;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = BootNotificationResponse::new(current_time, interval, status);

        response
            .set_current_time(new_time)
            .set_interval(new_interval)
            .set_status(new_status.clone())
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_current_time(), &new_time);
        assert_eq!(response.get_interval(), &new_interval);
        assert_eq!(response.get_status(), &new_status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_boot_notification_response_all_registration_statuses() {
        let current_time = Utc::now();
        let interval = 300;

        let registration_statuses = vec![
            RegistrationStatusEnumType::Accepted,
            RegistrationStatusEnumType::Pending,
            RegistrationStatusEnumType::Rejected,
        ];

        for status in registration_statuses {
            let response = BootNotificationResponse::new(current_time, interval, status.clone());
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_boot_notification_response_builder_pattern() {
        let current_time = Utc::now();
        let interval = 300;
        let status = RegistrationStatusEnumType::Accepted;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = BootNotificationResponse::new(current_time, interval, status.clone())
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_current_time(), &current_time);
        assert_eq!(response.get_interval(), &interval);
        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_boot_notification_request_json_round_trip() {
        let charging_station = create_test_charging_station();
        let reason = BootReasonEnumType::PowerUp;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = BootNotificationRequest::new(charging_station, reason)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: BootNotificationRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_boot_notification_response_json_round_trip() {
        let current_time = Utc::now();
        let interval = 300;
        let status = RegistrationStatusEnumType::Accepted;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = BootNotificationResponse::new(current_time, interval, status)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: BootNotificationResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_boot_notification_request_charging_station_validation() {
        // Test with charging station that has validation constraints
        let charging_station = ChargingStationType::new("TestModel".to_string(), "TestVendor".to_string())
            .with_serial_number("SN123456".to_string())
            .with_firmware_version("1.0.0".to_string());

        let reason = BootReasonEnumType::PowerUp;
        let request = BootNotificationRequest::new(charging_station, reason);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_boot_notification_response_edge_cases() {
        let current_time = Utc::now();
        let status = RegistrationStatusEnumType::Accepted;

        // Test with zero interval
        let response = BootNotificationResponse::new(current_time, 0, status.clone());
        assert_eq!(response.get_interval(), &0);
        assert!(response.validate().is_ok());

        // Test with negative interval
        let response = BootNotificationResponse::new(current_time, -1, status.clone());
        assert_eq!(response.get_interval(), &-1);
        assert!(response.validate().is_ok());

        // Test with large interval
        let response = BootNotificationResponse::new(current_time, i32::MAX, status);
        assert_eq!(response.get_interval(), &i32::MAX);
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_boot_notification_request_with_complex_charging_station() {
        use crate::v2_1::datatypes::ModemType;

        let modem = ModemType::new("1234567890123456789".to_string(), "123456789012345".to_string());
        let charging_station = ChargingStationType::new("ComplexModel".to_string(), "ComplexVendor".to_string())
            .with_serial_number("COMPLEX_SN_123".to_string())
            .with_firmware_version("2.1.0".to_string())
            .with_modem(modem);

        let reason = BootReasonEnumType::FirmwareUpdate;
        let custom_data = CustomDataType::new("ComplexVendor".to_string());

        let request = BootNotificationRequest::new(charging_station, reason)
            .with_custom_data(custom_data);

        assert!(request.validate().is_ok());

        // Test serialization with complex data
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: BootNotificationRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }
}