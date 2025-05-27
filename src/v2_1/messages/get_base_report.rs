use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{GenericDeviceModelStatusEnumType, ReportBaseEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetBaseReport request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetBaseReportRequest {
    /// The Id of the request.
    #[validate(range(min = 0))]
    pub request_id: i32,

    pub report_base: ReportBaseEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetBaseReportRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `request_id` - The Id of the request.
    /// * `report_base` - The report_base field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(request_id: i32, report_base: ReportBaseEnumType) -> Self {
        Self {
            request_id,
            report_base,
            custom_data: None,
        }
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

    /// Sets the report_base field.
    ///
    /// * `report_base` - The report_base field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_report_base(&mut self, report_base: ReportBaseEnumType) -> &mut Self {
        self.report_base = report_base;
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
    /// The Id of the request.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the report_base field.
    ///
    /// # Returns
    ///
    /// The report_base field
    pub fn get_report_base(&self) -> &ReportBaseEnumType {
        &self.report_base
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

/// Response body for the GetBaseReport response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetBaseReportResponse {
    pub status: GenericDeviceModelStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetBaseReportResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: GenericDeviceModelStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: GenericDeviceModelStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &GenericDeviceModelStatusEnumType {
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

    // Tests for GetBaseReportRequest

    #[test]
    fn test_get_base_report_request_new() {
        let request = GetBaseReportRequest::new(123, ReportBaseEnumType::FullInventory);

        assert_eq!(request.request_id, 123);
        assert_eq!(request.report_base, ReportBaseEnumType::FullInventory);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_base_report_request_with_custom_data() {
        let custom_data = create_test_custom_data();
        let request = GetBaseReportRequest::new(456, ReportBaseEnumType::ConfigurationInventory)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.request_id, 456);
        assert_eq!(request.report_base, ReportBaseEnumType::ConfigurationInventory);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_base_report_request_setters() {
        let custom_data = create_test_custom_data();

        let mut request = GetBaseReportRequest::new(100, ReportBaseEnumType::FullInventory);
        request.set_request_id(200);
        request.set_report_base(ReportBaseEnumType::SummaryInventory);
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.request_id, 200);
        assert_eq!(request.report_base, ReportBaseEnumType::SummaryInventory);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_base_report_request_getters() {
        let custom_data = create_test_custom_data();
        let request = GetBaseReportRequest::new(789, ReportBaseEnumType::ConfigurationInventory)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_request_id(), &789);
        assert_eq!(request.get_report_base(), &ReportBaseEnumType::ConfigurationInventory);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_base_report_request_serialization() {
        let request = GetBaseReportRequest::new(999, ReportBaseEnumType::FullInventory);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetBaseReportRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
    }

    #[test]
    fn test_get_base_report_request_deserialization() {
        let json = r#"{
            "requestId": 555,
            "reportBase": "SummaryInventory"
        }"#;

        let request: GetBaseReportRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.request_id, 555);
        assert_eq!(request.report_base, ReportBaseEnumType::SummaryInventory);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_base_report_request_validation() {
        let request = GetBaseReportRequest::new(100, ReportBaseEnumType::FullInventory);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_get_base_report_request_validation_negative_request_id() {
        let mut request = GetBaseReportRequest::new(100, ReportBaseEnumType::FullInventory);
        request.set_request_id(-1);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_base_report_request_all_report_base_types() {
        let report_bases = vec![
            ReportBaseEnumType::ConfigurationInventory,
            ReportBaseEnumType::FullInventory,
            ReportBaseEnumType::SummaryInventory,
        ];

        for report_base in report_bases {
            let request = GetBaseReportRequest::new(123, report_base.clone());
            assert_eq!(request.report_base, report_base);
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_get_base_report_request_json_round_trip() {
        let custom_data = create_test_custom_data();
        let request = GetBaseReportRequest::new(777, ReportBaseEnumType::ConfigurationInventory)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetBaseReportRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
        assert!(parsed.validate().is_ok());
    }

    // Tests for GetBaseReportResponse

    #[test]
    fn test_get_base_report_response_new() {
        let response = GetBaseReportResponse::new(GenericDeviceModelStatusEnumType::Accepted);

        assert_eq!(response.status, GenericDeviceModelStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_base_report_response_with_status_info() {
        let status_info = create_test_status_info();
        let response = GetBaseReportResponse::new(GenericDeviceModelStatusEnumType::Rejected)
            .with_status_info(status_info.clone());

        assert_eq!(response.status, GenericDeviceModelStatusEnumType::Rejected);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_base_report_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = GetBaseReportResponse::new(GenericDeviceModelStatusEnumType::NotSupported)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, GenericDeviceModelStatusEnumType::NotSupported);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_base_report_response_setters() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let mut response = GetBaseReportResponse::new(GenericDeviceModelStatusEnumType::Accepted);
        response.set_status(GenericDeviceModelStatusEnumType::EmptyResultSet);
        response.set_status_info(Some(status_info.clone()));
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, GenericDeviceModelStatusEnumType::EmptyResultSet);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_base_report_response_getters() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();
        let response = GetBaseReportResponse::new(GenericDeviceModelStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &GenericDeviceModelStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_base_report_response_serialization() {
        let response = GetBaseReportResponse::new(GenericDeviceModelStatusEnumType::Accepted);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetBaseReportResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
    }

    #[test]
    fn test_get_base_report_response_deserialization() {
        let json = r#"{
            "status": "Rejected",
            "statusInfo": {
                "reasonCode": "InvalidRequest"
            }
        }"#;

        let response: GetBaseReportResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, GenericDeviceModelStatusEnumType::Rejected);
        assert!(response.status_info.is_some());
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_base_report_response_validation() {
        let response = GetBaseReportResponse::new(GenericDeviceModelStatusEnumType::Accepted);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_get_base_report_response_all_status_types() {
        let statuses = vec![
            GenericDeviceModelStatusEnumType::Accepted,
            GenericDeviceModelStatusEnumType::Rejected,
            GenericDeviceModelStatusEnumType::NotSupported,
            GenericDeviceModelStatusEnumType::EmptyResultSet,
        ];

        for status in statuses {
            let response = GetBaseReportResponse::new(status.clone());
            assert_eq!(response.status, status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_get_base_report_response_json_round_trip() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();
        let response = GetBaseReportResponse::new(GenericDeviceModelStatusEnumType::NotSupported)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetBaseReportResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
        assert!(parsed.validate().is_ok());
    }
}