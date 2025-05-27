use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{GenericDeviceModelStatusEnumType, MonitoringBaseEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the SetMonitoringBase request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringBaseRequest {
    pub monitoring_base: MonitoringBaseEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetMonitoringBaseRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `monitoring_base` - The monitoring_base field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(monitoring_base: MonitoringBaseEnumType) -> Self {
        Self {
            monitoring_base,
            custom_data: None,
        }
    }

    /// Sets the monitoring_base field.
    ///
    /// * `monitoring_base` - The monitoring_base field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_monitoring_base(&mut self, monitoring_base: MonitoringBaseEnumType) -> &mut Self {
        self.monitoring_base = monitoring_base;
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

    /// Gets a reference to the monitoring_base field.
    ///
    /// # Returns
    ///
    /// The monitoring_base field
    pub fn get_monitoring_base(&self) -> &MonitoringBaseEnumType {
        &self.monitoring_base
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

/// Response body for the SetMonitoringBase response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringBaseResponse {
    pub status: GenericDeviceModelStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetMonitoringBaseResponse {
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
    use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
    use crate::v2_1::enumerations::{GenericDeviceModelStatusEnumType, MonitoringBaseEnumType};

    #[test]
    fn test_set_monitoring_base_request_new() {
        let request = SetMonitoringBaseRequest::new(MonitoringBaseEnumType::All);
        assert_eq!(request.monitoring_base, MonitoringBaseEnumType::All);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_set_monitoring_base_request_serialization() {
        let request = SetMonitoringBaseRequest::new(MonitoringBaseEnumType::FactoryDefault);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: SetMonitoringBaseRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(json.contains("\"monitoringBase\":\"FactoryDefault\""));
    }

    #[test]
    fn test_set_monitoring_base_request_validation() {
        let request = SetMonitoringBaseRequest::new(MonitoringBaseEnumType::HardWiredOnly);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_set_monitoring_base_request_builder_pattern() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = SetMonitoringBaseRequest::new(MonitoringBaseEnumType::All)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.monitoring_base, MonitoringBaseEnumType::All);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_monitoring_base_request_setters() {
        let mut request = SetMonitoringBaseRequest::new(MonitoringBaseEnumType::All);
        let custom_data = CustomDataType::new("TestVendor".to_string());

        request.set_monitoring_base(MonitoringBaseEnumType::FactoryDefault)
               .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.monitoring_base, MonitoringBaseEnumType::FactoryDefault);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_monitoring_base_request_getters() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = SetMonitoringBaseRequest::new(MonitoringBaseEnumType::HardWiredOnly)
            .with_custom_data(custom_data.clone());

        assert_eq!(*request.get_monitoring_base(), MonitoringBaseEnumType::HardWiredOnly);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_monitoring_base_response_new() {
        let response = SetMonitoringBaseResponse::new(GenericDeviceModelStatusEnumType::Accepted);
        assert_eq!(response.status, GenericDeviceModelStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_set_monitoring_base_response_serialization() {
        let response = SetMonitoringBaseResponse::new(GenericDeviceModelStatusEnumType::Rejected);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: SetMonitoringBaseResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(json.contains("\"status\":\"Rejected\""));
    }

    #[test]
    fn test_set_monitoring_base_response_builder_pattern() {
        let status_info = StatusInfoType::new("Monitoring base conflict".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = SetMonitoringBaseResponse::new(GenericDeviceModelStatusEnumType::NotSupported)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, GenericDeviceModelStatusEnumType::NotSupported);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_monitoring_base_response_setters() {
        let mut response = SetMonitoringBaseResponse::new(GenericDeviceModelStatusEnumType::Accepted);
        let status_info = StatusInfoType::new("Updated status".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        response.set_status(GenericDeviceModelStatusEnumType::Rejected)
                .set_status_info(Some(status_info.clone()))
                .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, GenericDeviceModelStatusEnumType::Rejected);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_monitoring_base_response_getters() {
        let status_info = StatusInfoType::new("Test status".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = SetMonitoringBaseResponse::new(GenericDeviceModelStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(*response.get_status(), GenericDeviceModelStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_monitoring_base_edge_cases() {
        // Test all monitoring base enum variants
        let variants = vec![
            MonitoringBaseEnumType::All,
            MonitoringBaseEnumType::FactoryDefault,
            MonitoringBaseEnumType::HardWiredOnly,
        ];

        for variant in variants {
            let request = SetMonitoringBaseRequest::new(variant.clone());
            assert!(request.validate().is_ok());
            assert_eq!(request.monitoring_base, variant);
        }
    }

    #[test]
    fn test_set_monitoring_base_response_validation() {
        let response = SetMonitoringBaseResponse::new(GenericDeviceModelStatusEnumType::Accepted);
        assert!(response.validate().is_ok());
    }
}