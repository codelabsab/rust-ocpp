use crate::v2_1::datatypes::CustomDataType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the NotifyCustomerInformation request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyCustomerInformationRequest {
    /// (Part of) the requested data. No format specified in which the data is returned. Should be human readable.
    #[validate(length(max = 512))]
    pub data: String,

    /// “to be continued” indicator. Indicates whether another part of the monitoringData follows in an upcoming notifyMonitoringReportRequest message. Default value when omitted is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    /// Sequence number of this message. First message starts at 0.
    #[validate(range(min = 0))]
    pub seq_no: i32,

    /// Timestamp of the moment this message was generated at the Charging Station.
    pub generated_at: DateTime<Utc>,

    /// The Id of the request.
    #[validate(range(min = 0))]
    pub request_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyCustomerInformationRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `data` - (Part of) the requested data. No format specified in which the data is returned. Should be human readable.
    /// * `seq_no` - Sequence number of this message. First message starts at 0.
    /// * `generated_at` - Timestamp of the moment this message was generated at the Charging Station.
    /// * `request_id` - The Id of the request.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(data: String, seq_no: i32, generated_at: DateTime<Utc>, request_id: i32) -> Self {
        Self {
            data,
            tbc: None,
            seq_no,
            generated_at,
            request_id,
            custom_data: None,
        }
    }

    /// Sets the data field.
    ///
    /// * `data` - (Part of) the requested data. No format specified in which the data is returned. Should be human readable.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_data(&mut self, data: String) -> &mut Self {
        self.data = data;
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

    /// Sets the request_id field.
    ///
    /// * `request_id` - The Id of the request.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
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

    /// Gets a reference to the data field.
    ///
    /// # Returns
    ///
    /// (Part of) the requested data. No format specified in which the data is returned. Should be human readable.
    pub fn get_data(&self) -> &String {
        &self.data
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

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// The Id of the request.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
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

/// Response body for the NotifyCustomerInformation response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyCustomerInformationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyCustomerInformationResponse {
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
    fn test_notify_customer_information_request_new() {
        let data = "Test customer data".to_string();
        let seq_no = 1;
        let generated_at = Utc::now();
        let request_id = 123;

        let request = NotifyCustomerInformationRequest::new(
            data.clone(),
            seq_no,
            generated_at,
            request_id,
        );

        assert_eq!(request.get_data(), &data);
        assert_eq!(request.get_seq_no(), &seq_no);
        assert_eq!(request.get_generated_at(), &generated_at);
        assert_eq!(request.get_request_id(), &request_id);
        assert_eq!(request.get_tbc(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_notify_customer_information_request_serialization() {
        let data = "Test customer data".to_string();
        let seq_no = 1;
        let generated_at = Utc::now();
        let request_id = 123;

        let request = NotifyCustomerInformationRequest::new(
            data,
            seq_no,
            generated_at,
            request_id,
        );

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: NotifyCustomerInformationRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_notify_customer_information_request_validation() {
        let data = "Test customer data".to_string();
        let seq_no = 1;
        let generated_at = Utc::now();
        let request_id = 123;

        let request = NotifyCustomerInformationRequest::new(
            data,
            seq_no,
            generated_at,
            request_id,
        );

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notify_customer_information_request_validation_data_too_long() {
        let data = "x".repeat(513); // Exceeds max length of 512
        let seq_no = 1;
        let generated_at = Utc::now();
        let request_id = 123;

        let request = NotifyCustomerInformationRequest::new(
            data,
            seq_no,
            generated_at,
            request_id,
        );

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_customer_information_request_validation_negative_seq_no() {
        let data = "Test customer data".to_string();
        let seq_no = -1; // Invalid negative value
        let generated_at = Utc::now();
        let request_id = 123;

        let request = NotifyCustomerInformationRequest::new(
            data,
            seq_no,
            generated_at,
            request_id,
        );

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_customer_information_request_validation_negative_request_id() {
        let data = "Test customer data".to_string();
        let seq_no = 1;
        let generated_at = Utc::now();
        let request_id = -1; // Invalid negative value

        let request = NotifyCustomerInformationRequest::new(
            data,
            seq_no,
            generated_at,
            request_id,
        );

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_notify_customer_information_request_with_tbc() {
        let data = "Test customer data".to_string();
        let seq_no = 1;
        let generated_at = Utc::now();
        let request_id = 123;

        let request = NotifyCustomerInformationRequest::new(
            data,
            seq_no,
            generated_at,
            request_id,
        ).with_tbc(true);

        assert_eq!(request.get_tbc(), Some(&true));
    }

    #[test]
    fn test_notify_customer_information_request_with_custom_data() {
        let data = "Test customer data".to_string();
        let seq_no = 1;
        let generated_at = Utc::now();
        let request_id = 123;
        let custom_data = create_test_custom_data();

        let request = NotifyCustomerInformationRequest::new(
            data,
            seq_no,
            generated_at,
            request_id,
        ).with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_customer_information_request_set_methods() {
        let data = "Test customer data".to_string();
        let seq_no = 1;
        let generated_at = Utc::now();
        let request_id = 123;

        let mut request = NotifyCustomerInformationRequest::new(
            data,
            seq_no,
            generated_at,
            request_id,
        );

        let new_data = "Updated customer data".to_string();
        let new_seq_no = 2;
        let new_generated_at = Utc::now();
        let new_request_id = 456;
        let custom_data = create_test_custom_data();

        request
            .set_data(new_data.clone())
            .set_seq_no(new_seq_no)
            .set_generated_at(new_generated_at)
            .set_request_id(new_request_id)
            .set_tbc(Some(true))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_data(), &new_data);
        assert_eq!(request.get_seq_no(), &new_seq_no);
        assert_eq!(request.get_generated_at(), &new_generated_at);
        assert_eq!(request.get_request_id(), &new_request_id);
        assert_eq!(request.get_tbc(), Some(&true));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_customer_information_request_builder_pattern() {
        let data = "Test customer data".to_string();
        let seq_no = 1;
        let generated_at = Utc::now();
        let request_id = 123;
        let custom_data = create_test_custom_data();

        let request = NotifyCustomerInformationRequest::new(
            data.clone(),
            seq_no,
            generated_at,
            request_id,
        )
        .with_tbc(true)
        .with_custom_data(custom_data.clone());

        assert_eq!(request.get_data(), &data);
        assert_eq!(request.get_tbc(), Some(&true));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_customer_information_request_json_round_trip() {
        let data = "Test customer data".to_string();
        let seq_no = 1;
        let generated_at = Utc::now();
        let request_id = 123;
        let custom_data = create_test_custom_data();

        let request = NotifyCustomerInformationRequest::new(
            data,
            seq_no,
            generated_at,
            request_id,
        )
        .with_tbc(true)
        .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: NotifyCustomerInformationRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_notify_customer_information_response_new() {
        let response = NotifyCustomerInformationResponse::new();

        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_notify_customer_information_response_serialization() {
        let response = NotifyCustomerInformationResponse::new();

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: NotifyCustomerInformationResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_notify_customer_information_response_validation() {
        let response = NotifyCustomerInformationResponse::new();

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_notify_customer_information_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = NotifyCustomerInformationResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_customer_information_response_set_custom_data() {
        let mut response = NotifyCustomerInformationResponse::new();
        let custom_data = create_test_custom_data();

        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_customer_information_response_builder_pattern() {
        let custom_data = create_test_custom_data();

        let response = NotifyCustomerInformationResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_notify_customer_information_response_json_round_trip() {
        let custom_data = create_test_custom_data();
        let response = NotifyCustomerInformationResponse::new()
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: NotifyCustomerInformationResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_notify_customer_information_response_empty_json() {
        let json = "{}";
        let response: NotifyCustomerInformationResponse =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(response.get_custom_data(), None);
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_notify_customer_information_request_boundary_values() {
        // Test with minimum valid values
        let data = "x".to_string(); // Minimum length
        let seq_no = 0; // Minimum valid value
        let generated_at = Utc::now();
        let request_id = 0; // Minimum valid value

        let request = NotifyCustomerInformationRequest::new(
            data,
            seq_no,
            generated_at,
            request_id,
        );

        assert!(request.validate().is_ok());

        // Test with maximum valid data length
        let data = "x".repeat(512); // Maximum allowed length
        let request = NotifyCustomerInformationRequest::new(
            data,
            seq_no,
            generated_at,
            request_id,
        );

        assert!(request.validate().is_ok());
    }
}