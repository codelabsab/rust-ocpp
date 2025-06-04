use crate::v2_1::datatypes::{
    CertificateHashDataType,
    CustomDataType,
    IdTokenType,
    StatusInfoType,
};
use crate::v2_1::enumerations::CustomerInformationStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the CustomerInformation request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInformationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub customer_certificate: Option<CertificateHashDataType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub id_token: Option<IdTokenType>,

    /// The Id of the request.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// Flag indicating whether the Charging Station should return NotifyCustomerInformationRequest messages containing information about the customer referred to.
    pub report: bool,

    /// Flag indicating whether the Charging Station should clear all information about the customer referred to.
    pub clear: bool,

    /// A (e.g. vendor specific) identifier of the customer this request refers to. This field contains a custom identifier other than IdToken and Certificate. One of the possible identifiers (customerIdentifier, customerIdToken or customerCertificate) should be in the request message.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 64))]
    pub customer_identifier: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl CustomerInformationRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `request_id` - The Id of the request.
    /// * `report` - Flag indicating whether the Charging Station should return NotifyCustomerInformationRequest messages containing information about the customer referred to.
    /// * `clear` - Flag indicating whether the Charging Station should clear all information about the customer referred to.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(request_id: i32, report: bool, clear: bool) -> Self {
        Self {
            customer_certificate: None,
            id_token: None,
            request_id,
            report,
            clear,
            customer_identifier: None,
            custom_data: None,
        }
    }

    /// Sets the customer_certificate field.
    ///
    /// * `customer_certificate` - The customer_certificate field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_customer_certificate(&mut self, customer_certificate: Option<CertificateHashDataType>) -> &mut Self {
        self.customer_certificate = customer_certificate;
        self
    }

    /// Sets the id_token field.
    ///
    /// * `id_token` - The id_token field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_id_token(&mut self, id_token: Option<IdTokenType>) -> &mut Self {
        self.id_token = id_token;
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

    /// Sets the report field.
    ///
    /// * `report` - Flag indicating whether the Charging Station should return NotifyCustomerInformationRequest messages containing information about the customer referred to.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_report(&mut self, report: bool) -> &mut Self {
        self.report = report;
        self
    }

    /// Sets the clear field.
    ///
    /// * `clear` - Flag indicating whether the Charging Station should clear all information about the customer referred to.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_clear(&mut self, clear: bool) -> &mut Self {
        self.clear = clear;
        self
    }

    /// Sets the customer_identifier field.
    ///
    /// * `customer_identifier` - A (e.g. vendor specific) identifier of the customer this request refers to. This field contains a custom identifier other than IdToken and Certificate. One of the possible identifiers (customerIdentifier, customerIdToken or customerCertificate) should be in the request message.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_customer_identifier(&mut self, customer_identifier: Option<String>) -> &mut Self {
        self.customer_identifier = customer_identifier;
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

    /// Gets a reference to the customer_certificate field.
    ///
    /// # Returns
    ///
    /// The customer_certificate field
    pub fn get_customer_certificate(&self) -> Option<&CertificateHashDataType> {
        self.customer_certificate.as_ref()
    }

    /// Gets a reference to the id_token field.
    ///
    /// # Returns
    ///
    /// The id_token field
    pub fn get_id_token(&self) -> Option<&IdTokenType> {
        self.id_token.as_ref()
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// The Id of the request.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the report field.
    ///
    /// # Returns
    ///
    /// Flag indicating whether the Charging Station should return NotifyCustomerInformationRequest messages containing information about the customer referred to.
    pub fn get_report(&self) -> &bool {
        &self.report
    }

    /// Gets a reference to the clear field.
    ///
    /// # Returns
    ///
    /// Flag indicating whether the Charging Station should clear all information about the customer referred to.
    pub fn get_clear(&self) -> &bool {
        &self.clear
    }

    /// Gets a reference to the customer_identifier field.
    ///
    /// # Returns
    ///
    /// A (e.g. vendor specific) identifier of the customer this request refers to. This field contains a custom identifier other than IdToken and Certificate. One of the possible identifiers (customerIdentifier, customerIdToken or customerCertificate) should be in the request message.
    pub fn get_customer_identifier(&self) -> Option<&String> {
        self.customer_identifier.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the customer_certificate field and returns self for builder pattern.
    ///
    /// * `customer_certificate` - The customer_certificate field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_customer_certificate(mut self, customer_certificate: CertificateHashDataType) -> Self {
        self.customer_certificate = Some(customer_certificate);
        self
    }

    /// Sets the id_token field and returns self for builder pattern.
    ///
    /// * `id_token` - The id_token field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_id_token(mut self, id_token: IdTokenType) -> Self {
        self.id_token = Some(id_token);
        self
    }

    /// Sets the customer_identifier field and returns self for builder pattern.
    ///
    /// * `customer_identifier` - A (e.g. vendor specific) identifier of the customer this request refers to. This field contains a custom identifier other than IdToken and Certificate. One of the possible identifiers (customerIdentifier, customerIdToken or customerCertificate) should be in the request message.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_customer_identifier(mut self, customer_identifier: String) -> Self {
        self.customer_identifier = Some(customer_identifier);
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

/// Response body for the CustomerInformation response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInformationResponse {
    pub status: CustomerInformationStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl CustomerInformationResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: CustomerInformationStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: CustomerInformationStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &CustomerInformationStatusEnumType {
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
    use crate::v2_1::enumerations::HashAlgorithmEnumType;
    use serde_json;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("TestReason".to_string())
    }

    fn create_test_certificate_hash_data() -> CertificateHashDataType {
        CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "issuer_name_hash".to_string(),
            "issuer_key_hash".to_string(),
            "serial_number".to_string(),
        )
    }

    fn create_test_id_token() -> IdTokenType {
        IdTokenType::new("test_token_123".to_string(), "RFID".to_string())
    }

    #[test]
    fn test_customer_information_request_new() {
        let request_id = 123;
        let report = true;
        let clear = false;

        let request = CustomerInformationRequest::new(request_id, report, clear);

        assert_eq!(request.request_id, request_id);
        assert_eq!(request.report, report);
        assert_eq!(request.clear, clear);
        assert_eq!(request.customer_certificate, None);
        assert_eq!(request.id_token, None);
        assert_eq!(request.customer_identifier, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_customer_information_request_with_customer_certificate() {
        let request_id = 456;
        let report = false;
        let clear = true;
        let certificate = create_test_certificate_hash_data();

        let request = CustomerInformationRequest::new(request_id, report, clear)
            .with_customer_certificate(certificate.clone());

        assert_eq!(request.request_id, request_id);
        assert_eq!(request.report, report);
        assert_eq!(request.clear, clear);
        assert_eq!(request.customer_certificate, Some(certificate));
        assert_eq!(request.id_token, None);
        assert_eq!(request.customer_identifier, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_customer_information_request_with_id_token() {
        let request_id = 789;
        let report = true;
        let clear = true;
        let id_token = create_test_id_token();

        let request = CustomerInformationRequest::new(request_id, report, clear)
            .with_id_token(id_token.clone());

        assert_eq!(request.request_id, request_id);
        assert_eq!(request.report, report);
        assert_eq!(request.clear, clear);
        assert_eq!(request.customer_certificate, None);
        assert_eq!(request.id_token, Some(id_token));
        assert_eq!(request.customer_identifier, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_customer_information_request_with_customer_identifier() {
        let request_id = 101;
        let report = false;
        let clear = false;
        let customer_identifier = "customer_123".to_string();

        let request = CustomerInformationRequest::new(request_id, report, clear)
            .with_customer_identifier(customer_identifier.clone());

        assert_eq!(request.request_id, request_id);
        assert_eq!(request.report, report);
        assert_eq!(request.clear, clear);
        assert_eq!(request.customer_certificate, None);
        assert_eq!(request.id_token, None);
        assert_eq!(request.customer_identifier, Some(customer_identifier));
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_customer_information_request_with_custom_data() {
        let request_id = 202;
        let report = true;
        let clear = false;
        let custom_data = create_test_custom_data();

        let request = CustomerInformationRequest::new(request_id, report, clear)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.request_id, request_id);
        assert_eq!(request.report, report);
        assert_eq!(request.clear, clear);
        assert_eq!(request.customer_certificate, None);
        assert_eq!(request.id_token, None);
        assert_eq!(request.customer_identifier, None);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_customer_information_request_setters() {
        let mut request = CustomerInformationRequest::new(1, true, false);

        let new_request_id = 999;
        let new_report = false;
        let new_clear = true;
        let certificate = create_test_certificate_hash_data();
        let id_token = create_test_id_token();
        let customer_identifier = "new_customer_456".to_string();
        let custom_data = create_test_custom_data();

        request.set_request_id(new_request_id)
               .set_report(new_report)
               .set_clear(new_clear)
               .set_customer_certificate(Some(certificate.clone()))
               .set_id_token(Some(id_token.clone()))
               .set_customer_identifier(Some(customer_identifier.clone()))
               .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.request_id, new_request_id);
        assert_eq!(request.report, new_report);
        assert_eq!(request.clear, new_clear);
        assert_eq!(request.customer_certificate, Some(certificate));
        assert_eq!(request.id_token, Some(id_token));
        assert_eq!(request.customer_identifier, Some(customer_identifier));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_customer_information_request_getters() {
        let request_id = 303;
        let report = true;
        let clear = false;
        let certificate = create_test_certificate_hash_data();
        let id_token = create_test_id_token();
        let customer_identifier = "customer_789".to_string();
        let custom_data = create_test_custom_data();

        let request = CustomerInformationRequest::new(request_id, report, clear)
            .with_customer_certificate(certificate.clone())
            .with_id_token(id_token.clone())
            .with_customer_identifier(customer_identifier.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_request_id(), &request_id);
        assert_eq!(request.get_report(), &report);
        assert_eq!(request.get_clear(), &clear);
        assert_eq!(request.get_customer_certificate(), Some(&certificate));
        assert_eq!(request.get_id_token(), Some(&id_token));
        assert_eq!(request.get_customer_identifier(), Some(&customer_identifier));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_customer_information_request_serialization() {
        let request_id = 404;
        let report = false;
        let clear = true;

        let request = CustomerInformationRequest::new(request_id, report, clear);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: CustomerInformationRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_customer_information_request_validation_negative_request_id() {
        let request = CustomerInformationRequest::new(-1, true, false);

        let validation_result = request.validate();
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_customer_information_request_validation_customer_identifier_too_long() {
        let long_identifier = "a".repeat(65); // Max is 64

        let request = CustomerInformationRequest::new(1, true, false)
            .with_customer_identifier(long_identifier);

        let validation_result = request.validate();
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_customer_information_request_validation_valid() {
        let request_id = 0; // 0 is valid (min = 0)
        let customer_identifier = "a".repeat(64); // Max length

        let request = CustomerInformationRequest::new(request_id, true, false)
            .with_customer_identifier(customer_identifier);

        let validation_result = request.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_customer_information_response_new() {
        let status = CustomerInformationStatusEnumType::Accepted;

        let response = CustomerInformationResponse::new(status.clone());

        assert_eq!(response.status, status);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_customer_information_response_with_status_info() {
        let status = CustomerInformationStatusEnumType::Rejected;
        let status_info = create_test_status_info();

        let response = CustomerInformationResponse::new(status.clone())
            .with_status_info(status_info.clone());

        assert_eq!(response.status, status);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_customer_information_response_with_custom_data() {
        let status = CustomerInformationStatusEnumType::Invalid;
        let custom_data = create_test_custom_data();

        let response = CustomerInformationResponse::new(status.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, status);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_customer_information_response_setters() {
        let mut response = CustomerInformationResponse::new(CustomerInformationStatusEnumType::Accepted);

        let new_status = CustomerInformationStatusEnumType::Rejected;
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        response.set_status(new_status.clone())
                .set_status_info(Some(status_info.clone()))
                .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, new_status);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_customer_information_response_getters() {
        let status = CustomerInformationStatusEnumType::Invalid;
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let response = CustomerInformationResponse::new(status.clone())
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_customer_information_response_serialization() {
        let status = CustomerInformationStatusEnumType::Accepted;

        let response = CustomerInformationResponse::new(status);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: CustomerInformationResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_customer_information_response_all_status_variants() {
        let statuses = vec![
            CustomerInformationStatusEnumType::Accepted,
            CustomerInformationStatusEnumType::Rejected,
            CustomerInformationStatusEnumType::Invalid,
        ];

        for status in statuses {
            let response = CustomerInformationResponse::new(status.clone());
            assert_eq!(response.status, status);

            // Test serialization for each variant
            let json = serde_json::to_string(&response).expect("Failed to serialize");
            let deserialized: CustomerInformationResponse =
                serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(response, deserialized);
        }
    }

    #[test]
    fn test_customer_information_response_validation() {
        let status = CustomerInformationStatusEnumType::Accepted;
        let response = CustomerInformationResponse::new(status);

        let validation_result = response.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_customer_information_request_builder_pattern() {
        let request_id = 505;
        let report = true;
        let clear = false;
        let certificate = create_test_certificate_hash_data();
        let id_token = create_test_id_token();
        let customer_identifier = "customer_builder".to_string();
        let custom_data = create_test_custom_data();

        let request = CustomerInformationRequest::new(request_id, report, clear)
            .with_customer_certificate(certificate.clone())
            .with_id_token(id_token.clone())
            .with_customer_identifier(customer_identifier.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.request_id, request_id);
        assert_eq!(request.report, report);
        assert_eq!(request.clear, clear);
        assert_eq!(request.customer_certificate, Some(certificate));
        assert_eq!(request.id_token, Some(id_token));
        assert_eq!(request.customer_identifier, Some(customer_identifier));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_customer_information_response_builder_pattern() {
        let status = CustomerInformationStatusEnumType::Accepted;
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let response = CustomerInformationResponse::new(status.clone())
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, status);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_customer_information_request_edge_cases() {
        // Test with zero request ID
        let request_zero = CustomerInformationRequest::new(0, true, false);
        assert_eq!(request_zero.request_id, 0);
        assert!(request_zero.validate().is_ok());

        // Test with large request ID
        let request_large = CustomerInformationRequest::new(999999, false, true);
        assert_eq!(request_large.request_id, 999999);
        assert!(request_large.validate().is_ok());

        // Test with empty customer identifier
        let request_empty_id = CustomerInformationRequest::new(1, true, false)
            .with_customer_identifier("".to_string());
        assert_eq!(request_empty_id.customer_identifier, Some("".to_string()));
        assert!(request_empty_id.validate().is_ok());

        // Test with maximum length customer identifier
        let max_identifier = "a".repeat(64);
        let request_max_id = CustomerInformationRequest::new(1, true, false)
            .with_customer_identifier(max_identifier.clone());
        assert_eq!(request_max_id.customer_identifier, Some(max_identifier));
        assert!(request_max_id.validate().is_ok());
    }

    #[test]
    fn test_customer_information_request_boolean_combinations() {
        let test_cases = vec![
            (true, true),
            (true, false),
            (false, true),
            (false, false),
        ];

        for (report, clear) in test_cases {
            let request = CustomerInformationRequest::new(1, report, clear);
            assert_eq!(request.report, report);
            assert_eq!(request.clear, clear);
            assert!(request.validate().is_ok());

            // Test serialization for each combination
            let json = serde_json::to_string(&request).expect("Failed to serialize");
            let deserialized: CustomerInformationRequest =
                serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(request, deserialized);
        }
    }

    #[test]
    fn test_customer_information_request_with_all_identifiers() {
        let request_id = 606;
        let report = true;
        let clear = false;
        let certificate = create_test_certificate_hash_data();
        let id_token = create_test_id_token();
        let customer_identifier = "all_identifiers".to_string();
        let custom_data = create_test_custom_data();

        let request = CustomerInformationRequest::new(request_id, report, clear)
            .with_customer_certificate(certificate.clone())
            .with_id_token(id_token.clone())
            .with_customer_identifier(customer_identifier.clone())
            .with_custom_data(custom_data.clone());

        // Verify all fields are set
        assert_eq!(request.customer_certificate, Some(certificate));
        assert_eq!(request.id_token, Some(id_token));
        assert_eq!(request.customer_identifier, Some(customer_identifier));
        assert_eq!(request.custom_data, Some(custom_data));

        // Test validation and serialization
        assert!(request.validate().is_ok());

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: CustomerInformationRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_customer_information_response_with_all_fields() {
        let status = CustomerInformationStatusEnumType::Rejected;
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let response = CustomerInformationResponse::new(status.clone())
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        // Verify all fields are set
        assert_eq!(response.status, status);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));

        // Test validation and serialization
        assert!(response.validate().is_ok());

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: CustomerInformationResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(response, deserialized);
    }
}