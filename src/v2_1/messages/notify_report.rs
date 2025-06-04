use crate::v2_1::datatypes::{CustomDataType, ReportDataType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the NotifyReport request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyReportRequest {
    /// The id of the GetReportRequest  or GetBaseReportRequest that requested this report
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// Timestamp of the moment this message was generated at the Charging Station.
    pub generated_at: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub report_data: Option<Vec<ReportDataType>>,

    /// “to be continued” indicator. Indicates whether another part of the report follows in an upcoming notifyReportRequest message. Default value when omitted is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    /// Sequence number of this message. First message starts at 0.
    #[validate(range(min = 0))]
    pub seq_no: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyReportRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `request_id` - The id of the GetReportRequest  or GetBaseReportRequest that requested this report
    /// * `generated_at` - Timestamp of the moment this message was generated at the Charging Station.
    /// * `seq_no` - Sequence number of this message. First message starts at 0.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(request_id: i32, generated_at: DateTime<Utc>, seq_no: i32) -> Self {
        Self {
            request_id,
            generated_at,
            report_data: None,
            tbc: None,
            seq_no,
            custom_data: None,
        }
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - The id of the GetReportRequest  or GetBaseReportRequest that requested this report
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
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

    /// Sets the report_data field.
    ///
    /// * `report_data` - The report_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_report_data(&mut self, report_data: Option<Vec<ReportDataType>>) -> &mut Self {
        self.report_data = report_data;
        self
    }

    /// Sets the tbc field.
    ///
    /// * `tbc` - “to be continued” indicator. Indicates whether another part of the report follows in an upcoming notifyReportRequest message. Default value when omitted is false.
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

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// The id of the GetReportRequest  or GetBaseReportRequest that requested this report
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the generated_at field.
    ///
    /// # Returns
    ///
    /// Timestamp of the moment this message was generated at the Charging Station.
    pub fn get_generated_at(&self) -> &DateTime<Utc> {
        &self.generated_at
    }

    /// Gets a reference to the report_data field.
    ///
    /// # Returns
    ///
    /// The report_data field
    pub fn get_report_data(&self) -> Option<&Vec<ReportDataType>> {
        self.report_data.as_ref()
    }

    /// Gets a reference to the tbc field.
    ///
    /// # Returns
    ///
    /// “to be continued” indicator. Indicates whether another part of the report follows in an upcoming notifyReportRequest message. Default value when omitted is false.
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

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the report_data field and returns self for builder pattern.
    ///
    /// * `report_data` - The report_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_report_data(mut self, report_data: Vec<ReportDataType>) -> Self {
        self.report_data = Some(report_data);
        self
    }

    /// Sets the tbc field and returns self for builder pattern.
    ///
    /// * `tbc` - “to be continued” indicator. Indicates whether another part of the report follows in an upcoming notifyReportRequest message. Default value when omitted is false.
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

/// Response body for the NotifyReport response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyReportResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyReportResponse {
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
    use crate::v2_1::datatypes::{ComponentType, VariableType, VariableAttributeType};
    use crate::v2_1::enumerations::{AttributeEnumType, MutabilityEnumType};
    use chrono::Utc;
    use serde_json;
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_report_data() -> ReportDataType {
        // Create a minimal ReportDataType for testing
        let component = ComponentType::new("test_component".to_string());
        let variable = VariableType::new("test_variable".to_string());
        let attribute = VariableAttributeType::new_with_value(
            AttributeEnumType::Actual,
            "test_value".to_string(),
            MutabilityEnumType::ReadOnly,
        );
        ReportDataType::new(component, variable, vec![attribute])
    }

    fn create_test_notify_report_request() -> NotifyReportRequest {
        let generated_at = Utc::now();
        NotifyReportRequest::new(123, generated_at, 0)
    }

    fn create_test_notify_report_response() -> NotifyReportResponse {
        NotifyReportResponse::new()
    }

    #[test]
    fn test_notify_report_request_new() {
        let request_id = 456;
        let generated_at = Utc::now();
        let seq_no = 5;

        let request = NotifyReportRequest::new(request_id, generated_at, seq_no);

        assert_eq!(request.request_id, request_id);
        assert_eq!(request.generated_at, generated_at);
        assert_eq!(request.seq_no, seq_no);
        assert!(request.report_data.is_none());
        assert!(request.tbc.is_none());
        assert!(request.custom_data.is_none());
    }

    #[test]
    fn test_notify_report_request_serialization() {
        let request = create_test_notify_report_request();

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: NotifyReportRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_notify_report_request_validation_valid() {
        let request = create_test_notify_report_request();
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_report_request_validation_negative_request_id() {
        let generated_at = Utc::now();
        let request = NotifyReportRequest::new(-1, generated_at, 0);

        let validation_result = request.validate();
        assert!(validation_result.is_err());
        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("request_id"));
    }

    #[test]
    fn test_notify_report_request_validation_negative_seq_no() {
        let generated_at = Utc::now();
        let request = NotifyReportRequest::new(123, generated_at, -1);

        let validation_result = request.validate();
        assert!(validation_result.is_err());
        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("seq_no"));
    }

    #[test]
    fn test_notify_report_request_validation_empty_report_data() {
        let mut request = create_test_notify_report_request();
        request.report_data = Some(vec![]); // Empty vector should fail validation

        let validation_result = request.validate();
        assert!(validation_result.is_err());
        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("report_data"));
    }

    #[test]
    fn test_notify_report_request_set_methods() {
        let mut request = create_test_notify_report_request();
        let new_request_id = 999;
        let new_generated_at = Utc::now();
        let new_seq_no = 10;
        let report_data = vec![create_test_report_data()];

        request.set_request_id(new_request_id)
               .set_generated_at(new_generated_at)
               .set_seq_no(new_seq_no)
               .set_report_data(Some(report_data.clone()))
               .set_tbc(Some(true));

        assert_eq!(request.request_id, new_request_id);
        assert_eq!(request.generated_at, new_generated_at);
        assert_eq!(request.seq_no, new_seq_no);
        assert_eq!(request.report_data, Some(report_data));
        assert_eq!(request.tbc, Some(true));
    }

    #[test]
    fn test_notify_report_request_get_methods() {
        let request = create_test_notify_report_request();

        assert_eq!(request.get_request_id(), &request.request_id);
        assert_eq!(request.get_generated_at(), &request.generated_at);
        assert_eq!(request.get_seq_no(), &request.seq_no);
        assert_eq!(request.get_report_data(), None);
        assert_eq!(request.get_tbc(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_notify_report_request_with_methods() {
        let custom_data = create_test_custom_data();
        let report_data = vec![create_test_report_data()];

        let request = create_test_notify_report_request()
            .with_report_data(report_data.clone())
            .with_tbc(true)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.report_data, Some(report_data));
        assert_eq!(request.tbc, Some(true));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_notify_report_request_boundary_values() {
        let generated_at = Utc::now();

        // Test minimum valid values
        let request_min = NotifyReportRequest::new(0, generated_at, 0);
        assert!(request_min.validate().is_ok());

        // Test large valid values
        let request_max = NotifyReportRequest::new(i32::MAX, generated_at, i32::MAX);
        assert!(request_max.validate().is_ok());
    }

    #[test]
    fn test_notify_report_request_json_format() {
        let request = create_test_notify_report_request();
        let json = serde_json::to_value(&request).expect("Failed to serialize to JSON");

        assert!(json.get("requestId").is_some());
        assert!(json.get("generatedAt").is_some());
        assert!(json.get("seqNo").is_some());

        // Optional fields should not be present if None
        if request.report_data.is_none() {
            assert!(json.get("reportData").is_none());
        }
        if request.tbc.is_none() {
            assert!(json.get("tbc").is_none());
        }
        if request.custom_data.is_none() {
            assert!(json.get("customData").is_none());
        }
    }

    #[test]
    fn test_notify_report_response_new() {
        let response = NotifyReportResponse::new();
        assert!(response.custom_data.is_none());
    }

    #[test]
    fn test_notify_report_response_serialization() {
        let response = create_test_notify_report_response();

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: NotifyReportResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_notify_report_response_validation_valid() {
        let response = create_test_notify_report_response();
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_notify_report_response_set_methods() {
        let mut response = create_test_notify_report_response();
        let custom_data = create_test_custom_data();

        response.set_custom_data(Some(custom_data.clone()));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_notify_report_response_get_methods() {
        let response = create_test_notify_report_response();
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_notify_report_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = create_test_notify_report_response()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.custom_data, Some(custom_data));
        assert_eq!(response.get_custom_data(), response.custom_data.as_ref());
    }

    #[test]
    fn test_notify_report_tbc_scenarios() {
        let mut request = create_test_notify_report_request();

        // Test tbc = true (more data to follow)
        request.set_tbc(Some(true));
        assert_eq!(request.tbc, Some(true));
        assert!(request.validate().is_ok());

        // Test tbc = false (last message)
        request.set_tbc(Some(false));
        assert_eq!(request.tbc, Some(false));
        assert!(request.validate().is_ok());

        // Test tbc = None (default false)
        request.set_tbc(None);
        assert!(request.tbc.is_none());
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_report_with_multiple_report_data() {
        let report_data = vec![
            create_test_report_data(),
            {
                let component = ComponentType::new("component2".to_string());
                let variable = VariableType::new("variable2".to_string());
                let attribute = VariableAttributeType::new_with_value(
                    AttributeEnumType::Actual,
                    "value2".to_string(),
                    MutabilityEnumType::ReadOnly,
                );
                ReportDataType::new(component, variable, vec![attribute])
            },
            {
                let component = ComponentType::new("component3".to_string());
                let variable = VariableType::new("variable3".to_string());
                let attribute = VariableAttributeType::new_with_value(
                    AttributeEnumType::Actual,
                    "value3".to_string(),
                    MutabilityEnumType::ReadOnly,
                );
                ReportDataType::new(component, variable, vec![attribute])
            },
        ];

        let request = create_test_notify_report_request()
            .with_report_data(report_data.clone());

        assert_eq!(request.report_data, Some(report_data));
        assert!(request.validate().is_ok());

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: NotifyReportRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_notify_report_sequence_numbers() {
        let generated_at = Utc::now();

        // Test sequence starting from 0
        let request_0 = NotifyReportRequest::new(123, generated_at, 0);
        assert_eq!(request_0.seq_no, 0);
        assert!(request_0.validate().is_ok());

        // Test subsequent sequence numbers
        let request_1 = NotifyReportRequest::new(123, generated_at, 1);
        assert_eq!(request_1.seq_no, 1);
        assert!(request_1.validate().is_ok());

        let request_100 = NotifyReportRequest::new(123, generated_at, 100);
        assert_eq!(request_100.seq_no, 100);
        assert!(request_100.validate().is_ok());
    }
}