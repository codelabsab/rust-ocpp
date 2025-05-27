use crate::v2_1::datatypes::{CustomDataType, MonitoringDataType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the NotifyMonitoringReport request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyMonitoringReportRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub monitor: Option<Vec<MonitoringDataType>>,

    /// The id of the GetMonitoringRequest that requested this report.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// “to be continued” indicator. Indicates whether another part of the monitoringData follows in an upcoming notifyMonitoringReportRequest message. Default value when omitted is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    /// Sequence number of this message. First message starts at 0.
    #[validate(range(min = 0))]
    pub seq_no: i32,

    /// Timestamp of the moment this message was generated at the Charging Station.
    pub generated_at: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyMonitoringReportRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `request_id` - The id of the GetMonitoringRequest that requested this report.
    /// * `seq_no` - Sequence number of this message. First message starts at 0.
    /// * `generated_at` - Timestamp of the moment this message was generated at the Charging Station.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(request_id: i32, seq_no: i32, generated_at: DateTime<Utc>) -> Self {
        Self {
            monitor: None,
            request_id,
            tbc: None,
            seq_no,
            generated_at,
            custom_data: None,
        }
    }

    /// Sets the monitor field.
    ///
    /// * `monitor` - The monitor field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_monitor(&mut self, monitor: Option<Vec<MonitoringDataType>>) -> &mut Self {
        self.monitor = monitor;
        self
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - The id of the GetMonitoringRequest that requested this report.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Sets the tbc field.
    ///
    /// * `tbc` - “to be continued” indicator. Indicates whether another part of the monitoringData follows in an upcoming notifyMonitoringReportRequest message. Default value when omitted is false.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_tbc(&mut self, tbc: Option<bool>) -> &mut Self {
        self.tbc = tbc;
        self
    }

    /// Sets the seq_no field.
    ///
    /// * `seq_no` - Sequence number of this message. First message starts at 0.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_seq_no(&mut self, seq_no: i32) -> &mut Self {
        self.seq_no = seq_no;
        self
    }

    /// Sets the generated_at field.
    ///
    /// * `generated_at` - Timestamp of the moment this message was generated at the Charging Station.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_generated_at(&mut self, generated_at: DateTime<Utc>) -> &mut Self {
        self.generated_at = generated_at;
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

    /// Gets a reference to the monitor field.
    ///
    /// # Returns
    ///
    /// The monitor field
    pub fn get_monitor(&self) -> Option<&Vec<MonitoringDataType>> {
        self.monitor.as_ref()
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// The id of the GetMonitoringRequest that requested this report.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the tbc field.
    ///
    /// # Returns
    ///
    /// “to be continued” indicator. Indicates whether another part of the monitoringData follows in an upcoming notifyMonitoringReportRequest message. Default value when omitted is false.
    pub fn get_tbc(&self) -> Option<&bool> {
        self.tbc.as_ref()
    }

    /// Gets a reference to the seq_no field.
    ///
    /// # Returns
    ///
    /// Sequence number of this message. First message starts at 0.
    pub fn get_seq_no(&self) -> &i32 {
        &self.seq_no
    }

    /// Gets a reference to the generated_at field.
    ///
    /// # Returns
    ///
    /// Timestamp of the moment this message was generated at the Charging Station.
    pub fn get_generated_at(&self) -> &DateTime<Utc> {
        &self.generated_at
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the monitor field and returns self for builder pattern.
    ///
    /// * `monitor` - The monitor field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_monitor(mut self, monitor: Vec<MonitoringDataType>) -> Self {
        self.monitor = Some(monitor);
        self
    }

    /// Sets the tbc field and returns self for builder pattern.
    ///
    /// * `tbc` - “to be continued” indicator. Indicates whether another part of the monitoringData follows in an upcoming notifyMonitoringReportRequest message. Default value when omitted is false.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_tbc(mut self, tbc: bool) -> Self {
        self.tbc = Some(tbc);
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

/// Response body for the NotifyMonitoringReport response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyMonitoringReportResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyMonitoringReportResponse {
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
    use crate::v2_1::datatypes::{ComponentType, VariableType, VariableMonitoringType};
    use crate::v2_1::enumerations::{MonitorEnumType, EventNotificationEnumType};
    use chrono::Utc;
    use rust_decimal_macros::dec;
    use serde_json;
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_monitoring_data() -> MonitoringDataType {
        let component = ComponentType::new("Connector".to_string());
        let variable = VariableType::new_with_instance("Temperature".to_string(), "Outlet".to_string());
        let variable_monitoring = vec![VariableMonitoringType::new(
            1,
            false,
            dec!(80.0),
            MonitorEnumType::UpperThreshold,
            0,
            EventNotificationEnumType::CustomMonitor,
        )];

        MonitoringDataType::new(component, variable, variable_monitoring)
    }

    #[test]
    fn test_notify_monitoring_report_request_new() {
        let request_id = 123;
        let seq_no = 0;
        let generated_at = Utc::now();

        let request = NotifyMonitoringReportRequest::new(request_id, seq_no, generated_at);

        assert_eq!(request.get_request_id(), &request_id);
        assert_eq!(request.get_seq_no(), &seq_no);
        assert_eq!(request.get_generated_at(), &generated_at);
        assert_eq!(request.get_monitor(), None);
        assert_eq!(request.get_tbc(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_notify_monitoring_report_request_validation() {
        let request_id = 123;
        let seq_no = 0;
        let generated_at = Utc::now();

        let request = NotifyMonitoringReportRequest::new(request_id, seq_no, generated_at);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_monitoring_report_request_validation_negative_request_id() {
        let request_id = -1; // Invalid negative value
        let seq_no = 0;
        let generated_at = Utc::now();

        let request = NotifyMonitoringReportRequest::new(request_id, seq_no, generated_at);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_monitoring_report_request_validation_negative_seq_no() {
        let request_id = 123;
        let seq_no = -1; // Invalid negative value
        let generated_at = Utc::now();

        let request = NotifyMonitoringReportRequest::new(request_id, seq_no, generated_at);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_monitoring_report_request_validation_empty_monitor() {
        let request_id = 123;
        let seq_no = 0;
        let generated_at = Utc::now();
        let monitor = vec![]; // Empty vector violates min length of 1

        let request = NotifyMonitoringReportRequest::new(request_id, seq_no, generated_at)
            .with_monitor(monitor);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_monitoring_report_request_with_optional_fields() {
        let request_id = 123;
        let seq_no = 1;
        let generated_at = Utc::now();
        let monitor = vec![create_test_monitoring_data()];
        let custom_data = create_test_custom_data();

        let request = NotifyMonitoringReportRequest::new(request_id, seq_no, generated_at)
            .with_monitor(monitor.clone())
            .with_tbc(true)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_request_id(), &request_id);
        assert_eq!(request.get_seq_no(), &seq_no);
        assert_eq!(request.get_generated_at(), &generated_at);
        assert_eq!(request.get_monitor(), Some(&monitor));
        assert_eq!(request.get_tbc(), Some(&true));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_monitoring_report_request_set_methods() {
        let request_id = 123;
        let seq_no = 0;
        let generated_at = Utc::now();

        let mut request = NotifyMonitoringReportRequest::new(request_id, seq_no, generated_at);

        let new_request_id = 456;
        let new_seq_no = 2;
        let new_generated_at = Utc::now();
        let monitor = vec![create_test_monitoring_data()];
        let custom_data = create_test_custom_data();

        request
            .set_request_id(new_request_id)
            .set_seq_no(new_seq_no)
            .set_generated_at(new_generated_at)
            .set_monitor(Some(monitor.clone()))
            .set_tbc(Some(false))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_request_id(), &new_request_id);
        assert_eq!(request.get_seq_no(), &new_seq_no);
        assert_eq!(request.get_generated_at(), &new_generated_at);
        assert_eq!(request.get_monitor(), Some(&monitor));
        assert_eq!(request.get_tbc(), Some(&false));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_monitoring_report_request_json_round_trip() {
        let request_id = 123;
        let seq_no = 1;
        let generated_at = Utc::now();
        let monitor = vec![create_test_monitoring_data()];
        let custom_data = create_test_custom_data();

        let request = NotifyMonitoringReportRequest::new(request_id, seq_no, generated_at)
            .with_monitor(monitor)
            .with_tbc(true)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: NotifyMonitoringReportRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_notify_monitoring_report_request_boundary_values() {
        let generated_at = Utc::now();

        // Test with minimum valid request_id and seq_no
        let request_id = 0; // Minimum valid value
        let seq_no = 0; // Minimum valid value

        let request = NotifyMonitoringReportRequest::new(request_id, seq_no, generated_at);

        assert_eq!(request.get_request_id(), &request_id);
        assert_eq!(request.get_seq_no(), &seq_no);
        assert!(request.validate().is_ok());

        // Test with minimum valid monitor length
        let monitor = vec![create_test_monitoring_data()]; // Minimum length of 1
        let request = NotifyMonitoringReportRequest::new(request_id, seq_no, generated_at)
            .with_monitor(monitor.clone());

        assert_eq!(request.get_monitor(), Some(&monitor));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_monitoring_report_response_new() {
        let response = NotifyMonitoringReportResponse::new();

        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_notify_monitoring_report_response_validation() {
        let response = NotifyMonitoringReportResponse::new();

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_notify_monitoring_report_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = NotifyMonitoringReportResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_monitoring_report_response_set_custom_data() {
        let mut response = NotifyMonitoringReportResponse::new();
        let custom_data = create_test_custom_data();

        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_monitoring_report_response_json_round_trip() {
        let custom_data = create_test_custom_data();
        let response = NotifyMonitoringReportResponse::new()
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: NotifyMonitoringReportResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_notify_monitoring_report_response_empty_json() {
        let json = "{}";
        let response: NotifyMonitoringReportResponse =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(response.get_custom_data(), None);
        assert!(response.validate().is_ok());
    }
}