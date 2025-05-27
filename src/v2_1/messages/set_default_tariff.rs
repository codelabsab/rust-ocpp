use crate::v2_1::datatypes::{CustomDataType, StatusInfoType, TariffType};
use crate::v2_1::enumerations::TariffSetStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the SetDefaultTariff request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetDefaultTariffRequest {
    /// EVSE that tariff applies to. When _evseId_ = 0, then tarriff applies to all EVSEs.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    #[validate(nested)]
    pub tariff: TariffType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetDefaultTariffRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `evse_id` - EVSE that tariff applies to. When _evseId_ = 0, then tarriff applies to all EVSEs.
    /// * `tariff` - The tariff field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(evse_id: i32, tariff: TariffType) -> Self {
        Self {
            evse_id,
            tariff,
            custom_data: None,
        }
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - EVSE that tariff applies to. When _evseId_ = 0, then tarriff applies to all EVSEs.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: i32) -> &mut Self {
        self.evse_id = evse_id;
        self
    }

    /// Sets the tariff field.
    ///
    /// * `tariff` - The tariff field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_tariff(&mut self, tariff: TariffType) -> &mut Self {
        self.tariff = tariff;
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

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// EVSE that tariff applies to. When _evseId_ = 0, then tarriff applies to all EVSEs.
    pub fn get_evse_id(&self) -> &i32 {
        &self.evse_id
    }

    /// Gets a reference to the tariff field.
    ///
    /// # Returns
    ///
    /// The tariff field
    pub fn get_tariff(&self) -> &TariffType {
        &self.tariff
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

/// Response body for the SetDefaultTariff response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetDefaultTariffResponse {
    pub status: TariffSetStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetDefaultTariffResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: TariffSetStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: TariffSetStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &TariffSetStatusEnumType {
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
    use crate::v2_1::datatypes::{CustomDataType, StatusInfoType, TariffType};
    use crate::v2_1::enumerations::TariffSetStatusEnumType;

    fn create_test_tariff() -> TariffType {
        TariffType::new("test_tariff".to_string(), "USD".to_string())
    }

    #[test]
    fn test_set_default_tariff_request_new() {
        let tariff = create_test_tariff();
        let request = SetDefaultTariffRequest::new(1, tariff.clone());

        assert_eq!(request.evse_id, 1);
        assert_eq!(request.tariff, tariff);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_set_default_tariff_request_serialization() {
        let tariff = create_test_tariff();
        let request = SetDefaultTariffRequest::new(2, tariff);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: SetDefaultTariffRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(json.contains("\"evseId\":2"));
    }

    #[test]
    fn test_set_default_tariff_request_validation() {
        let tariff = create_test_tariff();

        // Test valid request
        let valid_request = SetDefaultTariffRequest::new(0, tariff.clone());
        assert!(valid_request.validate().is_ok());

        // Test invalid request with negative evse_id
        let invalid_request = SetDefaultTariffRequest::new(-1, tariff);
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_set_default_tariff_request_builder_pattern() {
        let tariff = create_test_tariff();
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let request = SetDefaultTariffRequest::new(3, tariff.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.evse_id, 3);
        assert_eq!(request.tariff, tariff);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_default_tariff_request_setters() {
        let tariff1 = create_test_tariff();
        let tariff2 = TariffType::new("updated_tariff".to_string(), "EUR".to_string());
        let mut request = SetDefaultTariffRequest::new(1, tariff1);
        let custom_data = CustomDataType::new("test_vendor".to_string());

        request.set_evse_id(4)
               .set_tariff(tariff2.clone())
               .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.evse_id, 4);
        assert_eq!(request.tariff, tariff2);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_default_tariff_request_getters() {
        let tariff = create_test_tariff();
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let request = SetDefaultTariffRequest::new(5, tariff.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(*request.get_evse_id(), 5);
        assert_eq!(*request.get_tariff(), tariff);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_default_tariff_response_new() {
        let response = SetDefaultTariffResponse::new(TariffSetStatusEnumType::Accepted);
        assert_eq!(response.status, TariffSetStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_set_default_tariff_response_serialization() {
        let response = SetDefaultTariffResponse::new(TariffSetStatusEnumType::Rejected);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: SetDefaultTariffResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(json.contains("\"status\":\"Rejected\""));
    }

    #[test]
    fn test_set_default_tariff_response_builder_pattern() {
        let status_info = StatusInfoType::new("Tariff conflict".to_string());
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let response = SetDefaultTariffResponse::new(TariffSetStatusEnumType::Rejected)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, TariffSetStatusEnumType::Rejected);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_default_tariff_response_setters() {
        let mut response = SetDefaultTariffResponse::new(TariffSetStatusEnumType::Accepted);
        let status_info = StatusInfoType::new("Updated status".to_string());
        let custom_data = CustomDataType::new("test_vendor".to_string());

        response.set_status(TariffSetStatusEnumType::Rejected)
                .set_status_info(Some(status_info.clone()))
                .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, TariffSetStatusEnumType::Rejected);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_default_tariff_response_getters() {
        let status_info = StatusInfoType::new("Test status".to_string());
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let response = SetDefaultTariffResponse::new(TariffSetStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(*response.get_status(), TariffSetStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_default_tariff_edge_cases() {
        let tariff = create_test_tariff();

        // Test with evse_id = 0 (applies to all EVSEs)
        let request = SetDefaultTariffRequest::new(0, tariff);
        assert!(request.validate().is_ok());
        assert_eq!(request.evse_id, 0);
    }

    #[test]
    fn test_set_default_tariff_response_validation() {
        let response = SetDefaultTariffResponse::new(TariffSetStatusEnumType::Accepted);
        assert!(response.validate().is_ok());
    }
}