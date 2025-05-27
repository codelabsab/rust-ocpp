use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{DERControlEnumType, DERControlStatusEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetDERControl request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetDERControlRequest {
    /// RequestId to be used in ReportDERControlRequest.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// True: get a default DER control. False: get a scheduled control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_type: Option<DERControlEnumType>,

    /// Id of setting to get. When omitted all settings for _controlType_ are retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub control_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetDERControlRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `request_id` - RequestId to be used in ReportDERControlRequest.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(request_id: i32) -> Self {
        Self {
            request_id,
            is_default: None,
            control_type: None,
            control_id: None,
            custom_data: None,
        }
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - RequestId to be used in ReportDERControlRequest.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Sets the is_default field.
    ///
    /// * `is_default` - True: get a default DER control. False: get a scheduled control.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_is_default(&mut self, is_default: Option<bool>) -> &mut Self {
        self.is_default = is_default;
        self
    }

    /// Sets the control_type field.
    ///
    /// * `control_type` - The control_type field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_control_type(&mut self, control_type: Option<DERControlEnumType>) -> &mut Self {
        self.control_type = control_type;
        self
    }

    /// Sets the control_id field.
    ///
    /// * `control_id` - Id of setting to get. When omitted all settings for _controlType_ are retrieved.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_control_id(&mut self, control_id: Option<String>) -> &mut Self {
        self.control_id = control_id;
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
    /// RequestId to be used in ReportDERControlRequest.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the is_default field.
    ///
    /// # Returns
    ///
    /// True: get a default DER control. False: get a scheduled control.
    pub fn get_is_default(&self) -> Option<&bool> {
        self.is_default.as_ref()
    }

    /// Gets a reference to the control_type field.
    ///
    /// # Returns
    ///
    /// The control_type field
    pub fn get_control_type(&self) -> Option<&DERControlEnumType> {
        self.control_type.as_ref()
    }

    /// Gets a reference to the control_id field.
    ///
    /// # Returns
    ///
    /// Id of setting to get. When omitted all settings for _controlType_ are retrieved.
    pub fn get_control_id(&self) -> Option<&String> {
        self.control_id.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the is_default field and returns self for builder pattern.
    ///
    /// * `is_default` - True: get a default DER control. False: get a scheduled control.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_is_default(mut self, is_default: bool) -> Self {
        self.is_default = Some(is_default);
        self
    }

    /// Sets the control_type field and returns self for builder pattern.
    ///
    /// * `control_type` - The control_type field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_control_type(mut self, control_type: DERControlEnumType) -> Self {
        self.control_type = Some(control_type);
        self
    }

    /// Sets the control_id field and returns self for builder pattern.
    ///
    /// * `control_id` - Id of setting to get. When omitted all settings for _controlType_ are retrieved.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_control_id(mut self, control_id: String) -> Self {
        self.control_id = Some(control_id);
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

/// Response body for the GetDERControl response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetDERControlResponse {
    pub status: DERControlStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetDERControlResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: DERControlStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: DERControlStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &DERControlStatusEnumType {
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
    use serde_json;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("TestReason".to_string())
    }

    // Tests for GetDERControlRequest

    #[test]
    fn test_get_der_control_request_new() {
        let request = GetDERControlRequest::new(123);

        assert_eq!(request.request_id, 123);
        assert_eq!(request.is_default, None);
        assert_eq!(request.control_type, None);
        assert_eq!(request.control_id, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_der_control_request_with_is_default() {
        let request = GetDERControlRequest::new(456)
            .with_is_default(true);

        assert_eq!(request.request_id, 456);
        assert_eq!(request.is_default, Some(true));
        assert_eq!(request.control_type, None);
        assert_eq!(request.control_id, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_der_control_request_with_control_type() {
        let request = GetDERControlRequest::new(789)
            .with_control_type(DERControlEnumType::FreqWatt);

        assert_eq!(request.request_id, 789);
        assert_eq!(request.is_default, None);
        assert_eq!(request.control_type, Some(DERControlEnumType::FreqWatt));
        assert_eq!(request.control_id, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_der_control_request_with_control_id() {
        let request = GetDERControlRequest::new(999)
            .with_control_id("control_123".to_string());

        assert_eq!(request.request_id, 999);
        assert_eq!(request.is_default, None);
        assert_eq!(request.control_type, None);
        assert_eq!(request.control_id, Some("control_123".to_string()));
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_der_control_request_with_custom_data() {
        let custom_data = create_test_custom_data();
        let request = GetDERControlRequest::new(111)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.request_id, 111);
        assert_eq!(request.is_default, None);
        assert_eq!(request.control_type, None);
        assert_eq!(request.control_id, None);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_der_control_request_setters() {
        let custom_data = create_test_custom_data();

        let mut request = GetDERControlRequest::new(100);
        request.set_request_id(200);
        request.set_is_default(Some(false));
        request.set_control_type(Some(DERControlEnumType::VoltVar));
        request.set_control_id(Some("control_456".to_string()));
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.request_id, 200);
        assert_eq!(request.is_default, Some(false));
        assert_eq!(request.control_type, Some(DERControlEnumType::VoltVar));
        assert_eq!(request.control_id, Some("control_456".to_string()));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_der_control_request_getters() {
        let custom_data = create_test_custom_data();
        let request = GetDERControlRequest::new(555)
            .with_is_default(true)
            .with_control_type(DERControlEnumType::PowerLimitation)
            .with_control_id("control_789".to_string())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_request_id(), &555);
        assert_eq!(request.get_is_default(), Some(&true));
        assert_eq!(request.get_control_type(), Some(&DERControlEnumType::PowerLimitation));
        assert_eq!(request.get_control_id(), Some(&"control_789".to_string()));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_der_control_request_serialization() {
        let request = GetDERControlRequest::new(123);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetDERControlRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
    }

    #[test]
    fn test_get_der_control_request_validation() {
        let request = GetDERControlRequest::new(100);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_get_der_control_request_validation_negative_request_id() {
        let mut request = GetDERControlRequest::new(100);
        request.set_request_id(-1);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_der_control_request_validation_long_control_id() {
        let long_control_id = "a".repeat(37); // Exceeds max length of 36
        let mut request = GetDERControlRequest::new(100);
        request.set_control_id(Some(long_control_id));

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_der_control_request_validation_max_control_id() {
        let max_control_id = "a".repeat(36); // Exactly at max length
        let mut request = GetDERControlRequest::new(100);
        request.set_control_id(Some(max_control_id));

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_get_der_control_request_all_control_types() {
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
            DERControlEnumType::PowerLimitation,
            DERControlEnumType::PowerTarget,
            DERControlEnumType::PowerFactor,
            DERControlEnumType::VoltageTarget,
            DERControlEnumType::CurrentTarget,
            DERControlEnumType::LoadPriority,
        ];

        for control_type in control_types {
            let request = GetDERControlRequest::new(123)
                .with_control_type(control_type.clone());
            assert_eq!(request.control_type, Some(control_type));
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_get_der_control_request_json_round_trip() {
        let custom_data = create_test_custom_data();
        let request = GetDERControlRequest::new(777)
            .with_is_default(false)
            .with_control_type(DERControlEnumType::VoltWatt)
            .with_control_id("control_test".to_string())
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetDERControlRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
        assert!(parsed.validate().is_ok());
    }

    // Tests for GetDERControlResponse

    #[test]
    fn test_get_der_control_response_new() {
        let response = GetDERControlResponse::new(DERControlStatusEnumType::Accepted);

        assert_eq!(response.status, DERControlStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_der_control_response_with_status_info() {
        let status_info = create_test_status_info();
        let response = GetDERControlResponse::new(DERControlStatusEnumType::Rejected)
            .with_status_info(status_info.clone());

        assert_eq!(response.status, DERControlStatusEnumType::Rejected);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_der_control_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = GetDERControlResponse::new(DERControlStatusEnumType::NotSupported)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, DERControlStatusEnumType::NotSupported);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_der_control_response_setters() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let mut response = GetDERControlResponse::new(DERControlStatusEnumType::Accepted);
        response.set_status(DERControlStatusEnumType::NotFound);
        response.set_status_info(Some(status_info.clone()));
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, DERControlStatusEnumType::NotFound);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_der_control_response_getters() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();
        let response = GetDERControlResponse::new(DERControlStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &DERControlStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_der_control_response_serialization() {
        let response = GetDERControlResponse::new(DERControlStatusEnumType::Accepted);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetDERControlResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
    }

    #[test]
    fn test_get_der_control_response_validation() {
        let response = GetDERControlResponse::new(DERControlStatusEnumType::Accepted);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_get_der_control_response_all_status_types() {
        let statuses = vec![
            DERControlStatusEnumType::Accepted,
            DERControlStatusEnumType::Rejected,
            DERControlStatusEnumType::NotSupported,
            DERControlStatusEnumType::NotFound,
        ];

        for status in statuses {
            let response = GetDERControlResponse::new(status.clone());
            assert_eq!(response.status, status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_get_der_control_response_json_round_trip() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();
        let response = GetDERControlResponse::new(DERControlStatusEnumType::NotSupported)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetDERControlResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
        assert!(parsed.validate().is_ok());
    }
}