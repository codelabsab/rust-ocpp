use crate::v2_1::datatypes::{ConstantStreamDataType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::GenericStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the OpenPeriodicEventStream request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OpenPeriodicEventStreamRequest {
    #[validate(nested)]
    pub constant_stream_data: ConstantStreamDataType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl OpenPeriodicEventStreamRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `constant_stream_data` - The constant_stream_data field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(constant_stream_data: ConstantStreamDataType) -> Self {
        Self {
            constant_stream_data,
            custom_data: None,
        }
    }

    /// Sets the constant_stream_data field.
    ///
    /// * `constant_stream_data` - The constant_stream_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_constant_stream_data(&mut self, constant_stream_data: ConstantStreamDataType) -> &mut Self {
        self.constant_stream_data = constant_stream_data;
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

    /// Gets a reference to the constant_stream_data field.
    ///
    /// # Returns
    ///
    /// The constant_stream_data field
    pub fn get_constant_stream_data(&self) -> &ConstantStreamDataType {
        &self.constant_stream_data
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

/// Response body for the OpenPeriodicEventStream response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OpenPeriodicEventStreamResponse {
    pub status: GenericStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl OpenPeriodicEventStreamResponse {
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
    use crate::v2_1::datatypes::PeriodicEventStreamParamsType;
    use serde_json;
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("Test status info".to_string())
    }

    fn create_test_constant_stream_data() -> ConstantStreamDataType {
        let params = PeriodicEventStreamParamsType::new(300, 10);
        ConstantStreamDataType::new(1, params, 100)
    }

    fn create_test_open_periodic_event_stream_request() -> OpenPeriodicEventStreamRequest {
        let constant_stream_data = create_test_constant_stream_data();
        OpenPeriodicEventStreamRequest::new(constant_stream_data)
    }

    fn create_test_open_periodic_event_stream_response() -> OpenPeriodicEventStreamResponse {
        OpenPeriodicEventStreamResponse::new(GenericStatusEnumType::Accepted)
    }

    #[test]
    fn test_open_periodic_event_stream_request_new() {
        let constant_stream_data = create_test_constant_stream_data();
        let request = OpenPeriodicEventStreamRequest::new(constant_stream_data.clone());

        assert_eq!(request.constant_stream_data, constant_stream_data);
        assert!(request.custom_data.is_none());
    }

    #[test]
    fn test_open_periodic_event_stream_request_serialization() {
        let request = create_test_open_periodic_event_stream_request();

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: OpenPeriodicEventStreamRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_open_periodic_event_stream_request_validation_valid() {
        let request = create_test_open_periodic_event_stream_request();
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_open_periodic_event_stream_request_set_methods() {
        let mut request = create_test_open_periodic_event_stream_request();
        let new_constant_stream_data = create_test_constant_stream_data();

        request.set_constant_stream_data(new_constant_stream_data.clone());
        assert_eq!(request.constant_stream_data, new_constant_stream_data);
    }

    #[test]
    fn test_open_periodic_event_stream_request_get_methods() {
        let request = create_test_open_periodic_event_stream_request();

        assert_eq!(request.get_constant_stream_data(), &request.constant_stream_data);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_open_periodic_event_stream_request_with_custom_data() {
        let custom_data = create_test_custom_data();
        let request = create_test_open_periodic_event_stream_request()
            .with_custom_data(custom_data.clone());

        assert_eq!(request.custom_data, Some(custom_data));
        assert_eq!(request.get_custom_data(), request.custom_data.as_ref());
    }

    #[test]
    fn test_open_periodic_event_stream_request_set_custom_data() {
        let mut request = create_test_open_periodic_event_stream_request();
        let custom_data = create_test_custom_data();

        request.set_custom_data(Some(custom_data.clone()));
        assert_eq!(request.custom_data, Some(custom_data));

        request.set_custom_data(None);
        assert!(request.custom_data.is_none());
    }

    #[test]
    fn test_open_periodic_event_stream_request_json_format() {
        let request = create_test_open_periodic_event_stream_request();
        let json = serde_json::to_value(&request).expect("Failed to serialize to JSON");

        assert!(json.get("constantStreamData").is_some());

        // Custom data should not be present if None
        if request.custom_data.is_none() {
            assert!(json.get("customData").is_none());
        }
    }

    #[test]
    fn test_open_periodic_event_stream_response_new() {
        let status = GenericStatusEnumType::Rejected;
        let response = OpenPeriodicEventStreamResponse::new(status.clone());

        assert_eq!(response.status, status);
        assert!(response.status_info.is_none());
        assert!(response.custom_data.is_none());
    }

    #[test]
    fn test_open_periodic_event_stream_response_serialization() {
        let response = create_test_open_periodic_event_stream_response();

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: OpenPeriodicEventStreamResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_open_periodic_event_stream_response_validation_valid() {
        let response = create_test_open_periodic_event_stream_response();
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_open_periodic_event_stream_response_set_methods() {
        let mut response = create_test_open_periodic_event_stream_response();
        let new_status = GenericStatusEnumType::Rejected;
        let status_info = create_test_status_info();

        response.set_status(new_status.clone())
                .set_status_info(Some(status_info.clone()));

        assert_eq!(response.status, new_status);
        assert_eq!(response.status_info, Some(status_info));
    }

    #[test]
    fn test_open_periodic_event_stream_response_get_methods() {
        let response = create_test_open_periodic_event_stream_response();

        assert_eq!(response.get_status(), &response.status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_open_periodic_event_stream_response_with_methods() {
        let custom_data = create_test_custom_data();
        let status_info = create_test_status_info();

        let response = create_test_open_periodic_event_stream_response()
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_open_periodic_event_stream_response_status_variants() {
        // Test all GenericStatusEnumType variants
        let statuses = vec![
            GenericStatusEnumType::Accepted,
            GenericStatusEnumType::Rejected,
        ];

        for status in statuses {
            let response = OpenPeriodicEventStreamResponse::new(status.clone());
            assert_eq!(response.status, status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_open_periodic_event_stream_response_json_format() {
        let response = create_test_open_periodic_event_stream_response();
        let json = serde_json::to_value(&response).expect("Failed to serialize to JSON");

        assert!(json.get("status").is_some());

        // Optional fields should not be present if None
        if response.status_info.is_none() {
            assert!(json.get("statusInfo").is_none());
        }
        if response.custom_data.is_none() {
            assert!(json.get("customData").is_none());
        }
    }

    #[test]
    fn test_open_periodic_event_stream_round_trip_with_all_fields() {
        let custom_data = create_test_custom_data();
        let status_info = create_test_status_info();

        let request = create_test_open_periodic_event_stream_request()
            .with_custom_data(custom_data.clone());

        let response = create_test_open_periodic_event_stream_response()
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        // Test request round trip
        let request_json = serde_json::to_string(&request).expect("Failed to serialize request");
        let request_deserialized: OpenPeriodicEventStreamRequest = serde_json::from_str(&request_json).expect("Failed to deserialize request");
        assert_eq!(request, request_deserialized);

        // Test response round trip
        let response_json = serde_json::to_string(&response).expect("Failed to serialize response");
        let response_deserialized: OpenPeriodicEventStreamResponse = serde_json::from_str(&response_json).expect("Failed to deserialize response");
        assert_eq!(response, response_deserialized);
    }

    #[test]
    fn test_open_periodic_event_stream_constant_stream_data_scenarios() {
        // Test different constant stream data configurations
        let params1 = PeriodicEventStreamParamsType::new(60, 5);
        let stream_data1 = ConstantStreamDataType::new(1, params1, 50);
        let request1 = OpenPeriodicEventStreamRequest::new(stream_data1);
        assert!(request1.validate().is_ok());

        let params2 = PeriodicEventStreamParamsType::new(300, 15);
        let stream_data2 = ConstantStreamDataType::new(2, params2, 200);
        let request2 = OpenPeriodicEventStreamRequest::new(stream_data2);
        assert!(request2.validate().is_ok());

        // Test serialization preserves data
        let json1 = serde_json::to_string(&request1).expect("Failed to serialize");
        let deserialized1: OpenPeriodicEventStreamRequest = serde_json::from_str(&json1).expect("Failed to deserialize");
        assert_eq!(request1, deserialized1);
    }

    #[test]
    fn test_open_periodic_event_stream_response_with_status_info() {
        let status_info = create_test_status_info();
        let response = create_test_open_periodic_event_stream_response()
            .with_status_info(status_info.clone());

        assert_eq!(response.status_info, Some(status_info));
        assert!(response.validate().is_ok());

        // Test that status_info appears in JSON when present
        let json = serde_json::to_value(&response).expect("Failed to serialize to JSON");
        assert!(json.get("statusInfo").is_some());
    }
}