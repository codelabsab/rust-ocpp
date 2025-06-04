use crate::v2_1::datatypes::{AddressType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::GenericStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the VatNumberValidation request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VatNumberValidationRequest {
    /// VAT number to check.
    #[validate(length(max = 20))]
    pub vat_number: String,

    /// EVSE id for which check is done
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl VatNumberValidationRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `vat_number` - VAT number to check.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(vat_number: String) -> Self {
        Self {
            vat_number,
            evse_id: None,
            custom_data: None,
        }
    }

    /// Sets the vat_number field.
    ///
    /// * `vat_number` - VAT number to check.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_vat_number(&mut self, vat_number: String) -> &mut Self {
        self.vat_number = vat_number;
        self
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - EVSE id for which check is done
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: Option<i32>) -> &mut Self {
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

    /// Gets a reference to the vat_number field.
    ///
    /// # Returns
    ///
    /// VAT number to check.
    pub fn get_vat_number(&self) -> &String {
        &self.vat_number
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// EVSE id for which check is done
    pub fn get_evse_id(&self) -> Option<&i32> {
        self.evse_id.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the evse_id field and returns self for builder pattern.
    ///
    /// * `evse_id` - EVSE id for which check is done
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_evse_id(mut self, evse_id: i32) -> Self {
        self.evse_id = Some(evse_id);
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

/// Response body for the VatNumberValidation response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VatNumberValidationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub company: Option<AddressType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// VAT number that was requested.
    #[validate(length(max = 20))]
    pub vat_number: String,

    /// EVSE id for which check was requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    pub status: GenericStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl VatNumberValidationResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `vat_number` - VAT number that was requested.
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(vat_number: String, status: GenericStatusEnumType) -> Self {
        Self {
            company: None,
            status_info: None,
            vat_number,
            evse_id: None,
            status,
            custom_data: None,
        }
    }

    /// Sets the company field.
    ///
    /// * `company` - The company field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_company(&mut self, company: Option<AddressType>) -> &mut Self {
        self.company = company;
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

    /// Sets the vat_number field.
    ///
    /// * `vat_number` - VAT number that was requested.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_vat_number(&mut self, vat_number: String) -> &mut Self {
        self.vat_number = vat_number;
        self
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - EVSE id for which check was requested.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: Option<i32>) -> &mut Self {
        self.evse_id = evse_id;
        self
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

    /// Gets a reference to the company field.
    ///
    /// # Returns
    ///
    /// The company field
    pub fn get_company(&self) -> Option<&AddressType> {
        self.company.as_ref()
    }

    /// Gets a reference to the status_info field.
    ///
    /// # Returns
    ///
    /// The status_info field
    pub fn get_status_info(&self) -> Option<&StatusInfoType> {
        self.status_info.as_ref()
    }

    /// Gets a reference to the vat_number field.
    ///
    /// # Returns
    ///
    /// VAT number that was requested.
    pub fn get_vat_number(&self) -> &String {
        &self.vat_number
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// EVSE id for which check was requested.
    pub fn get_evse_id(&self) -> Option<&i32> {
        self.evse_id.as_ref()
    }

    /// Gets a reference to the status field.
    ///
    /// # Returns
    ///
    /// The status field
    pub fn get_status(&self) -> &GenericStatusEnumType {
        &self.status
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the company field and returns self for builder pattern.
    ///
    /// * `company` - The company field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_company(mut self, company: AddressType) -> Self {
        self.company = Some(company);
        self
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

    /// Sets the evse_id field and returns self for builder pattern.
    ///
    /// * `evse_id` - EVSE id for which check was requested.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_evse_id(mut self, evse_id: i32) -> Self {
        self.evse_id = Some(evse_id);
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
    use crate::v2_1::datatypes::{AddressType, CustomDataType, StatusInfoType};
    use crate::v2_1::enumerations::GenericStatusEnumType;
    use serde_json;
    use validator::Validate;

    // Tests for VatNumberValidationRequest

    #[test]
    fn test_vat_number_validation_request_new() {
        let vat_number = "DE123456789".to_string();
        let request = VatNumberValidationRequest::new(vat_number.clone());

        assert_eq!(request.get_vat_number(), &vat_number);
        assert_eq!(request.get_evse_id(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_vat_number_validation_request_serialization() {
        let request = VatNumberValidationRequest::new("FR98765432101".to_string());

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: VatNumberValidationRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_vat_number_validation_request_validation() {
        let request = VatNumberValidationRequest::new("GB123456789".to_string());

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_vat_number_validation_request_with_evse_id() {
        let request = VatNumberValidationRequest::new("IT12345678901".to_string())
            .with_evse_id(5);

        assert_eq!(request.get_evse_id(), Some(&5));
    }

    #[test]
    fn test_vat_number_validation_request_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = VatNumberValidationRequest::new("ES123456789".to_string())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_vat_number_validation_request_set_methods() {
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = VatNumberValidationRequest::new("original".to_string());

        request
            .set_vat_number("updated".to_string())
            .set_evse_id(Some(10))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_vat_number(), "updated");
        assert_eq!(request.get_evse_id(), Some(&10));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_vat_number_validation_request_builder_pattern() {
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = VatNumberValidationRequest::new("NL123456789B01".to_string())
            .with_evse_id(3)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_evse_id(), Some(&3));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_vat_number_validation_request_vat_number_validation() {
        let max_length_vat = "a".repeat(20); // Max allowed length
        let request = VatNumberValidationRequest::new(max_length_vat);

        assert!(request.validate().is_ok());

        let too_long_vat = "a".repeat(21); // Exceeds max length
        let invalid_request = VatNumberValidationRequest::new(too_long_vat);

        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_vat_number_validation_request_evse_id_validation() {
        let mut request = VatNumberValidationRequest::new("VALID123".to_string());
        request.set_evse_id(Some(0)); // Min allowed value

        assert!(request.validate().is_ok());

        request.set_evse_id(Some(-1)); // Below min value

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_vat_number_validation_request_edge_cases() {
        // Empty VAT number (should be valid according to length constraint)
        let request_empty = VatNumberValidationRequest::new("".to_string());
        assert!(request_empty.validate().is_ok());

        // Single character VAT number
        let request_single = VatNumberValidationRequest::new("A".to_string());
        assert!(request_single.validate().is_ok());

        // Large EVSE ID
        let request_large_evse = VatNumberValidationRequest::new("TEST123".to_string())
            .with_evse_id(999999);
        assert!(request_large_evse.validate().is_ok());
    }

    // Tests for VatNumberValidationResponse

    #[test]
    fn test_vat_number_validation_response_new() {
        let vat_number = "DE987654321".to_string();
        let status = GenericStatusEnumType::Accepted;
        let response = VatNumberValidationResponse::new(vat_number.clone(), status.clone());

        assert_eq!(response.get_vat_number(), &vat_number);
        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_company(), None);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_evse_id(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_vat_number_validation_response_serialization() {
        let response = VatNumberValidationResponse::new("FR11223344556".to_string(), GenericStatusEnumType::Rejected);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: VatNumberValidationResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_vat_number_validation_response_validation() {
        let response = VatNumberValidationResponse::new("GB999888777".to_string(), GenericStatusEnumType::Accepted);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_vat_number_validation_response_with_company() {
        let company = AddressType::new("Test Company".to_string(), "123 Test Street".to_string(), "Test City".to_string(), "Test Country".to_string());
        let response = VatNumberValidationResponse::new("IT99887766554".to_string(), GenericStatusEnumType::Accepted)
            .with_company(company.clone());

        assert_eq!(response.get_company(), Some(&company));
    }

    #[test]
    fn test_vat_number_validation_response_with_status_info() {
        let status_info = StatusInfoType::new("Validation successful".to_string());
        let response = VatNumberValidationResponse::new("ES556677889".to_string(), GenericStatusEnumType::Accepted)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
    }

    #[test]
    fn test_vat_number_validation_response_with_evse_id() {
        let response = VatNumberValidationResponse::new("NL001122334B56".to_string(), GenericStatusEnumType::Accepted)
            .with_evse_id(7);

        assert_eq!(response.get_evse_id(), Some(&7));
    }

    #[test]
    fn test_vat_number_validation_response_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = VatNumberValidationResponse::new("AT123456789".to_string(), GenericStatusEnumType::Rejected)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_vat_number_validation_response_set_methods() {
        let company = AddressType::new("Test Company".to_string(), "123 Test Street".to_string(), "Test City".to_string(), "Test Country".to_string());
        let status_info = StatusInfoType::new("Updated info".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = VatNumberValidationResponse::new("ORIGINAL".to_string(), GenericStatusEnumType::Accepted);

        response
            .set_company(Some(company.clone()))
            .set_status_info(Some(status_info.clone()))
            .set_vat_number("UPDATED".to_string())
            .set_evse_id(Some(15))
            .set_status(GenericStatusEnumType::Rejected)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_company(), Some(&company));
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_vat_number(), "UPDATED");
        assert_eq!(response.get_evse_id(), Some(&15));
        assert_eq!(response.get_status(), &GenericStatusEnumType::Rejected);
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_vat_number_validation_response_builder_pattern() {
        let company = AddressType::new("Test Company".to_string(), "123 Test Street".to_string(), "Test City".to_string(), "Test Country".to_string());
        let status_info = StatusInfoType::new("Builder test".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = VatNumberValidationResponse::new("BUILDER123".to_string(), GenericStatusEnumType::Accepted)
            .with_company(company.clone())
            .with_status_info(status_info.clone())
            .with_evse_id(20)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_company(), Some(&company));
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_evse_id(), Some(&20));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_vat_number_validation_response_all_status_types() {
        let status_types = vec![
            GenericStatusEnumType::Accepted,
            GenericStatusEnumType::Rejected,
        ];

        for status in status_types {
            let response = VatNumberValidationResponse::new("TEST123".to_string(), status.clone());
            
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());

            let json = serde_json::to_string(&response).expect("Failed to serialize");
            let deserialized: VatNumberValidationResponse = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(response, deserialized);
        }
    }

    #[test]
    fn test_vat_number_validation_response_vat_number_validation() {
        let max_length_vat = "a".repeat(20); // Max allowed length
        let response = VatNumberValidationResponse::new(max_length_vat, GenericStatusEnumType::Accepted);

        assert!(response.validate().is_ok());

        let too_long_vat = "a".repeat(21); // Exceeds max length
        let invalid_response = VatNumberValidationResponse::new(too_long_vat, GenericStatusEnumType::Accepted);

        assert!(invalid_response.validate().is_err());
    }

    #[test]
    fn test_vat_number_validation_response_evse_id_validation() {
        let mut response = VatNumberValidationResponse::new("VALID123".to_string(), GenericStatusEnumType::Accepted);
        response.set_evse_id(Some(0)); // Min allowed value

        assert!(response.validate().is_ok());

        response.set_evse_id(Some(-1)); // Below min value

        assert!(response.validate().is_err());
    }

    #[test]
    fn test_vat_number_validation_request_json_round_trip() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = VatNumberValidationRequest::new("ROUNDTRIP123".to_string())
            .with_evse_id(25)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: VatNumberValidationRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_vat_number_validation_response_json_round_trip() {
        let company = AddressType::new("Test Company".to_string(), "123 Test Street".to_string(), "Test City".to_string(), "Test Country".to_string());
        let status_info = StatusInfoType::new("Round trip test".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = VatNumberValidationResponse::new("ROUNDTRIP456".to_string(), GenericStatusEnumType::Rejected)
            .with_company(company)
            .with_status_info(status_info)
            .with_evse_id(30)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: VatNumberValidationResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_vat_number_validation_response_with_all_optional_fields() {
        let company = AddressType::new("Test Company".to_string(), "123 Test Street".to_string(), "Test City".to_string(), "Test Country".to_string());
        let status_info = StatusInfoType::new("All fields test".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = VatNumberValidationResponse::new("ALLFIELDS123".to_string(), GenericStatusEnumType::Accepted)
            .with_company(company.clone())
            .with_status_info(status_info.clone())
            .with_evse_id(35)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_vat_number(), "ALLFIELDS123");
        assert_eq!(response.get_status(), &GenericStatusEnumType::Accepted);
        assert_eq!(response.get_company(), Some(&company));
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_evse_id(), Some(&35));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_vat_number_validation_request_empty_vat_number() {
        let request = VatNumberValidationRequest::new("".to_string());

        assert!(request.validate().is_ok()); // Empty string is allowed
    }

    #[test]
    fn test_vat_number_validation_response_empty_vat_number() {
        let response = VatNumberValidationResponse::new("".to_string(), GenericStatusEnumType::Accepted);

        assert!(response.validate().is_ok()); // Empty string is allowed
    }

    #[test]
    fn test_vat_number_validation_request_with_custom_data_validation() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = VatNumberValidationRequest::new("CUSTOMTEST123".to_string())
            .with_custom_data(custom_data);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_vat_number_validation_response_with_nested_validation() {
        let company = AddressType::new("Test Company".to_string(), "123 Test Street".to_string(), "Test City".to_string(), "Test Country".to_string());
        let status_info = StatusInfoType::new("Nested test".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());
        
        let response = VatNumberValidationResponse::new("NESTED123".to_string(), GenericStatusEnumType::Accepted)
            .with_company(company)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        assert!(response.validate().is_ok());
    }
}