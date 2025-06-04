use crate::v2_1::datatypes::{CustomDataType, StatusInfoType, TariffAssignmentType};
use crate::v2_1::enumerations::TariffGetStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetTariffs request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetTariffsRequest {
    /// EVSE id to get tariff from. When _evseId_ = 0, this gets tariffs from all EVSEs.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetTariffsRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `evse_id` - EVSE id to get tariff from. When _evseId_ = 0, this gets tariffs from all EVSEs.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(evse_id: i32) -> Self {
        Self {
            evse_id,
            custom_data: None,
        }
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - EVSE id to get tariff from. When _evseId_ = 0, this gets tariffs from all EVSEs.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: i32) -> &mut Self {
        self.evse_id = evse_id;
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
    /// EVSE id to get tariff from. When _evseId_ = 0, this gets tariffs from all EVSEs.
    pub fn get_evse_id(&self) -> &i32 {
        &self.evse_id
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

/// Response body for the GetTariffs response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetTariffsResponse {
    pub status: TariffGetStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub tariff_assignments: Option<Vec<TariffAssignmentType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetTariffsResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: TariffGetStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
            tariff_assignments: None,
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
    pub fn set_status(&mut self, status: TariffGetStatusEnumType) -> &mut Self {
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

    /// Sets the tariff_assignments field.
    ///
    /// * `tariff_assignments` - The tariff_assignments field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_tariff_assignments(&mut self, tariff_assignments: Option<Vec<TariffAssignmentType>>) -> &mut Self {
        self.tariff_assignments = tariff_assignments;
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
    pub fn get_status(&self) -> &TariffGetStatusEnumType {
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

    /// Gets a reference to the tariff_assignments field.
    ///
    /// # Returns
    ///
    /// The tariff_assignments field
    pub fn get_tariff_assignments(&self) -> Option<&Vec<TariffAssignmentType>> {
        self.tariff_assignments.as_ref()
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

    /// Sets the tariff_assignments field and returns self for builder pattern.
    ///
    /// * `tariff_assignments` - The tariff_assignments field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_tariff_assignments(mut self, tariff_assignments: Vec<TariffAssignmentType>) -> Self {
        self.tariff_assignments = Some(tariff_assignments);
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

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("TestReason".to_string())
    }

    fn create_test_tariff_assignment() -> TariffAssignmentType {
        TariffAssignmentType::new(
            "TestTariff".to_string(),
            Utc::now(),
        )
    }

    // Tests for GetTariffsRequest

    #[test]
    fn test_get_tariffs_request_new() {
        let request = GetTariffsRequest::new(0);

        assert_eq!(request.evse_id, 0);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_tariffs_request_with_custom_data() {
        let custom_data = create_test_custom_data();
        let request = GetTariffsRequest::new(1)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.evse_id, 1);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_tariffs_request_setters() {
        let custom_data = create_test_custom_data();

        let mut request = GetTariffsRequest::new(0);
        request.set_evse_id(2);
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.evse_id, 2);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_tariffs_request_getters() {
        let custom_data = create_test_custom_data();
        let request = GetTariffsRequest::new(3)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_evse_id(), &3);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_tariffs_request_serialization() {
        let request = GetTariffsRequest::new(0);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetTariffsRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
    }

    #[test]
    fn test_get_tariffs_request_validation() {
        let request = GetTariffsRequest::new(1);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_get_tariffs_request_validation_negative_evse_id() {
        let mut request = GetTariffsRequest::new(1);
        request.set_evse_id(-1);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_tariffs_request_validation_zero_evse_id() {
        let request = GetTariffsRequest::new(0);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_get_tariffs_request_json_round_trip() {
        let custom_data = create_test_custom_data();
        let request = GetTariffsRequest::new(5)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetTariffsRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
        assert!(parsed.validate().is_ok());
    }

    // Tests for GetTariffsResponse

    #[test]
    fn test_get_tariffs_response_new() {
        let response = GetTariffsResponse::new(TariffGetStatusEnumType::Accepted);

        assert_eq!(response.status, TariffGetStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.tariff_assignments, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_tariffs_response_with_status_info() {
        let status_info = create_test_status_info();
        let response = GetTariffsResponse::new(TariffGetStatusEnumType::Rejected)
            .with_status_info(status_info.clone());

        assert_eq!(response.status, TariffGetStatusEnumType::Rejected);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.tariff_assignments, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_tariffs_response_with_tariff_assignments() {
        let tariff_assignments = vec![create_test_tariff_assignment()];
        let response = GetTariffsResponse::new(TariffGetStatusEnumType::Accepted)
            .with_tariff_assignments(tariff_assignments.clone());

        assert_eq!(response.status, TariffGetStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.tariff_assignments, Some(tariff_assignments));
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_tariffs_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = GetTariffsResponse::new(TariffGetStatusEnumType::Accepted)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, TariffGetStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.tariff_assignments, None);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_tariffs_response_setters() {
        let status_info = create_test_status_info();
        let tariff_assignments = vec![create_test_tariff_assignment()];
        let custom_data = create_test_custom_data();

        let mut response = GetTariffsResponse::new(TariffGetStatusEnumType::Accepted);
        response.set_status(TariffGetStatusEnumType::InvalidId);
        response.set_status_info(Some(status_info.clone()));
        response.set_tariff_assignments(Some(tariff_assignments.clone()));
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, TariffGetStatusEnumType::InvalidId);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.tariff_assignments, Some(tariff_assignments));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_tariffs_response_getters() {
        let status_info = create_test_status_info();
        let tariff_assignments = vec![create_test_tariff_assignment()];
        let custom_data = create_test_custom_data();
        let response = GetTariffsResponse::new(TariffGetStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_tariff_assignments(tariff_assignments.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &TariffGetStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_tariff_assignments(), Some(&tariff_assignments));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_tariffs_response_serialization() {
        let response = GetTariffsResponse::new(TariffGetStatusEnumType::Accepted);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetTariffsResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
    }

    #[test]
    fn test_get_tariffs_response_validation() {
        let response = GetTariffsResponse::new(TariffGetStatusEnumType::Accepted);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_get_tariffs_response_validation_empty_tariff_assignments() {
        let mut response = GetTariffsResponse::new(TariffGetStatusEnumType::Accepted);
        response.set_tariff_assignments(Some(vec![])); // Empty list should fail validation

        assert!(response.validate().is_err());
    }

    #[test]
    fn test_get_tariffs_response_all_status_types() {
        let statuses = vec![
            TariffGetStatusEnumType::Accepted,
            TariffGetStatusEnumType::Rejected,
            TariffGetStatusEnumType::InvalidId,
        ];

        for status in statuses {
            let response = GetTariffsResponse::new(status.clone());
            assert_eq!(response.status, status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_get_tariffs_response_multiple_tariff_assignments() {
        let tariff_assignment1 = TariffAssignmentType::new("Tariff1".to_string(), Utc::now());
        let tariff_assignment2 = TariffAssignmentType::new("Tariff2".to_string(), Utc::now());
        let tariff_assignments = vec![tariff_assignment1, tariff_assignment2];

        let response = GetTariffsResponse::new(TariffGetStatusEnumType::Accepted)
            .with_tariff_assignments(tariff_assignments.clone());

        assert_eq!(response.tariff_assignments, Some(tariff_assignments));
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_get_tariffs_response_json_round_trip() {
        let status_info = create_test_status_info();
        let tariff_assignments = vec![create_test_tariff_assignment()];
        let custom_data = create_test_custom_data();
        let response = GetTariffsResponse::new(TariffGetStatusEnumType::Accepted)
            .with_status_info(status_info)
            .with_tariff_assignments(tariff_assignments)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetTariffsResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
        assert!(parsed.validate().is_ok());
    }
}