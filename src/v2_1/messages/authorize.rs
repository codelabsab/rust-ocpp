use crate::v2_1::datatypes::{
    CustomDataType,
    IdTokenInfoType,
    IdTokenType,
    OCSPRequestDataType,
    TariffType,
};
use crate::v2_1::enumerations::{AuthorizeCertificateStatusEnumType, EnergyTransferModeEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the Authorize request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
    #[validate(nested)]
    pub id_token: IdTokenType,

    /// *(2.1)* The X.509 certificate chain presented by EV and encoded in PEM format. Order of certificates in chain is from leaf up to (but excluding) root certificate. + Only needed in case of central contract validation when Charging Station cannot validate the contract certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 10000))]
    pub certificate: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 4))]
    #[validate(nested)]
    pub iso_15118_certificate_hash_data: Option<Vec<OCSPRequestDataType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl AuthorizeRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `id_token` - The id_token field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(id_token: IdTokenType) -> Self {
        Self {
            id_token,
            certificate: None,
            iso_15118_certificate_hash_data: None,
            custom_data: None,
        }
    }

    /// Sets the id_token field.
    ///
    /// * `id_token` - The id_token field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_id_token(&mut self, id_token: IdTokenType) -> &mut Self {
        self.id_token = id_token;
        self
    }

    /// Sets the certificate field.
    ///
    /// * `certificate` - *(2.1)* The X.509 certificate chain presented by EV and encoded in PEM format. Order of certificates in chain is from leaf up to (but excluding) root certificate. + Only needed in case of central contract validation when Charging Station cannot validate the contract certificate.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_certificate(&mut self, certificate: Option<String>) -> &mut Self {
        self.certificate = certificate;
        self
    }

    /// Sets the iso_15118_certificate_hash_data field.
    ///
    /// * `iso_15118_certificate_hash_data` - The iso_15118_certificate_hash_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_iso_15118_certificate_hash_data(&mut self, iso_15118_certificate_hash_data: Option<Vec<OCSPRequestDataType>>) -> &mut Self {
        self.iso_15118_certificate_hash_data = iso_15118_certificate_hash_data;
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

    /// Gets a reference to the id_token field.
    ///
    /// # Returns
    ///
    /// The id_token field
    pub fn get_id_token(&self) -> &IdTokenType {
        &self.id_token
    }

    /// Gets a reference to the certificate field.
    ///
    /// # Returns
    ///
    /// *(2.1)* The X.509 certificate chain presented by EV and encoded in PEM format. Order of certificates in chain is from leaf up to (but excluding) root certificate. + Only needed in case of central contract validation when Charging Station cannot validate the contract certificate.
    pub fn get_certificate(&self) -> Option<&String> {
        self.certificate.as_ref()
    }

    /// Gets a reference to the iso_15118_certificate_hash_data field.
    ///
    /// # Returns
    ///
    /// The iso_15118_certificate_hash_data field
    pub fn get_iso_15118_certificate_hash_data(&self) -> Option<&Vec<OCSPRequestDataType>> {
        self.iso_15118_certificate_hash_data.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the certificate field and returns self for builder pattern.
    ///
    /// * `certificate` - *(2.1)* The X.509 certificate chain presented by EV and encoded in PEM format. Order of certificates in chain is from leaf up to (but excluding) root certificate. + Only needed in case of central contract validation when Charging Station cannot validate the contract certificate.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_certificate(mut self, certificate: String) -> Self {
        self.certificate = Some(certificate);
        self
    }

    /// Sets the iso_15118_certificate_hash_data field and returns self for builder pattern.
    ///
    /// * `iso_15118_certificate_hash_data` - The iso_15118_certificate_hash_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_iso_15118_certificate_hash_data(mut self, iso_15118_certificate_hash_data: Vec<OCSPRequestDataType>) -> Self {
        self.iso_15118_certificate_hash_data = Some(iso_15118_certificate_hash_data);
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

/// Response body for the Authorize response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeResponse {
    #[validate(nested)]
    pub id_token_info: IdTokenInfoType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_status: Option<AuthorizeCertificateStatusEnumType>,

    /// *(2.1)* List of allowed energy transfer modes the EV can choose from. If omitted this defaults to charging only.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub allowed_energy_transfer: Option<Vec<EnergyTransferModeEnumType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub tariff: Option<TariffType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl AuthorizeResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `id_token_info` - The id_token_info field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(id_token_info: IdTokenInfoType) -> Self {
        Self {
            id_token_info,
            certificate_status: None,
            allowed_energy_transfer: None,
            tariff: None,
            custom_data: None,
        }
    }

    /// Sets the id_token_info field.
    ///
    /// * `id_token_info` - The id_token_info field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_id_token_info(&mut self, id_token_info: IdTokenInfoType) -> &mut Self {
        self.id_token_info = id_token_info;
        self
    }

    /// Sets the certificate_status field.
    ///
    /// * `certificate_status` - The certificate_status field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_certificate_status(&mut self, certificate_status: Option<AuthorizeCertificateStatusEnumType>) -> &mut Self {
        self.certificate_status = certificate_status;
        self
    }

    /// Sets the allowed_energy_transfer field.
    ///
    /// * `allowed_energy_transfer` - *(2.1)* List of allowed energy transfer modes the EV can choose from. If omitted this defaults to charging only.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_allowed_energy_transfer(&mut self, allowed_energy_transfer: Option<Vec<EnergyTransferModeEnumType>>) -> &mut Self {
        self.allowed_energy_transfer = allowed_energy_transfer;
        self
    }

    /// Sets the tariff field.
    ///
    /// * `tariff` - The tariff field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_tariff(&mut self, tariff: Option<TariffType>) -> &mut Self {
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

    /// Gets a reference to the id_token_info field.
    ///
    /// # Returns
    ///
    /// The id_token_info field
    pub fn get_id_token_info(&self) -> &IdTokenInfoType {
        &self.id_token_info
    }

    /// Gets a reference to the certificate_status field.
    ///
    /// # Returns
    ///
    /// The certificate_status field
    pub fn get_certificate_status(&self) -> Option<&AuthorizeCertificateStatusEnumType> {
        self.certificate_status.as_ref()
    }

    /// Gets a reference to the allowed_energy_transfer field.
    ///
    /// # Returns
    ///
    /// *(2.1)* List of allowed energy transfer modes the EV can choose from. If omitted this defaults to charging only.
    pub fn get_allowed_energy_transfer(&self) -> Option<&Vec<EnergyTransferModeEnumType>> {
        self.allowed_energy_transfer.as_ref()
    }

    /// Gets a reference to the tariff field.
    ///
    /// # Returns
    ///
    /// The tariff field
    pub fn get_tariff(&self) -> Option<&TariffType> {
        self.tariff.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the certificate_status field and returns self for builder pattern.
    ///
    /// * `certificate_status` - The certificate_status field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_certificate_status(mut self, certificate_status: AuthorizeCertificateStatusEnumType) -> Self {
        self.certificate_status = Some(certificate_status);
        self
    }

    /// Sets the allowed_energy_transfer field and returns self for builder pattern.
    ///
    /// * `allowed_energy_transfer` - *(2.1)* List of allowed energy transfer modes the EV can choose from. If omitted this defaults to charging only.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_allowed_energy_transfer(mut self, allowed_energy_transfer: Vec<EnergyTransferModeEnumType>) -> Self {
        self.allowed_energy_transfer = Some(allowed_energy_transfer);
        self
    }

    /// Sets the tariff field and returns self for builder pattern.
    ///
    /// * `tariff` - The tariff field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_tariff(mut self, tariff: TariffType) -> Self {
        self.tariff = Some(tariff);
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
    use crate::v2_1::enumerations::{AuthorizationStatusEnumType, EnergyTransferModeEnumType, HashAlgorithmEnumType};
    use crate::v2_1::datatypes::OCSPRequestDataType;
    use serde_json;
    use validator::Validate;

    fn create_test_id_token() -> IdTokenType {
        IdTokenType::new("4F62C4E0123456789".to_string(), "ISO14443".to_string())
    }

    fn create_test_id_token_info() -> IdTokenInfoType {
        IdTokenInfoType::new(AuthorizationStatusEnumType::Accepted)
    }

    fn create_test_ocsp_request_data() -> OCSPRequestDataType {
        OCSPRequestDataType::new(
            HashAlgorithmEnumType::SHA256,
            "issuer_name_hash".to_string(),
            "issuer_key_hash".to_string(),
            "serial_number".to_string(),
            "http://responder.url".to_string(),
        )
    }

    #[test]
    fn test_authorize_request_new() {
        let id_token = create_test_id_token();
        let request = AuthorizeRequest::new(id_token.clone());

        assert_eq!(request.get_id_token(), &id_token);
        assert_eq!(request.get_certificate(), None);
        assert_eq!(request.get_iso_15118_certificate_hash_data(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_authorize_request_validation() {
        let id_token = create_test_id_token();
        let request = AuthorizeRequest::new(id_token);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_authorize_request_with_certificate() {
        let id_token = create_test_id_token();
        let certificate = "-----BEGIN CERTIFICATE-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...\n-----END CERTIFICATE-----".to_string();

        let request = AuthorizeRequest::new(id_token)
            .with_certificate(certificate.clone());

        assert_eq!(request.get_certificate(), Some(&certificate));
    }

    #[test]
    fn test_authorize_request_certificate_length_validation() {
        let id_token = create_test_id_token();
        let long_certificate = "a".repeat(10001); // Exceeds max length of 10000

        let mut request = AuthorizeRequest::new(id_token);
        request.set_certificate(Some(long_certificate));

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_authorize_request_with_iso_15118_certificate_hash_data() {
        let id_token = create_test_id_token();
        let ocsp_data = vec![create_test_ocsp_request_data()];

        let request = AuthorizeRequest::new(id_token)
            .with_iso_15118_certificate_hash_data(ocsp_data.clone());

        assert_eq!(request.get_iso_15118_certificate_hash_data(), Some(&ocsp_data));
    }

    #[test]
    fn test_authorize_request_iso_15118_certificate_hash_data_validation() {
        let id_token = create_test_id_token();
        let mut request = AuthorizeRequest::new(id_token);

        // Test empty array (should fail validation - min 1)
        request.set_iso_15118_certificate_hash_data(Some(vec![]));
        assert!(request.validate().is_err());

        // Test array with 5 items (should fail validation - max 4)
        let ocsp_data = vec![
            create_test_ocsp_request_data(),
            create_test_ocsp_request_data(),
            create_test_ocsp_request_data(),
            create_test_ocsp_request_data(),
            create_test_ocsp_request_data(),
        ];
        request.set_iso_15118_certificate_hash_data(Some(ocsp_data));
        assert!(request.validate().is_err());

        // Test valid array (1-4 items)
        let valid_ocsp_data = vec![create_test_ocsp_request_data()];
        request.set_iso_15118_certificate_hash_data(Some(valid_ocsp_data));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_authorize_request_serialization() {
        let id_token = create_test_id_token();
        let request = AuthorizeRequest::new(id_token);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: AuthorizeRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_authorize_request_builder_pattern() {
        let id_token = create_test_id_token();
        let certificate = "test_certificate".to_string();
        let ocsp_data = vec![create_test_ocsp_request_data()];
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = AuthorizeRequest::new(id_token.clone())
            .with_certificate(certificate.clone())
            .with_iso_15118_certificate_hash_data(ocsp_data.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_id_token(), &id_token);
        assert_eq!(request.get_certificate(), Some(&certificate));
        assert_eq!(request.get_iso_15118_certificate_hash_data(), Some(&ocsp_data));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_authorize_request_set_methods() {
        let id_token = create_test_id_token();
        let new_id_token = IdTokenType::new("ABCDEF123456".to_string(), "RFID".to_string());
        let certificate = "test_certificate".to_string();
        let ocsp_data = vec![create_test_ocsp_request_data()];
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = AuthorizeRequest::new(id_token);

        request
            .set_id_token(new_id_token.clone())
            .set_certificate(Some(certificate.clone()))
            .set_iso_15118_certificate_hash_data(Some(ocsp_data.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_id_token(), &new_id_token);
        assert_eq!(request.get_certificate(), Some(&certificate));
        assert_eq!(request.get_iso_15118_certificate_hash_data(), Some(&ocsp_data));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_authorize_response_new() {
        let id_token_info = create_test_id_token_info();
        let response = AuthorizeResponse::new(id_token_info.clone());

        assert_eq!(response.get_id_token_info(), &id_token_info);
        assert_eq!(response.get_certificate_status(), None);
        assert_eq!(response.get_allowed_energy_transfer(), None);
        assert_eq!(response.get_tariff(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_authorize_response_validation() {
        let id_token_info = create_test_id_token_info();
        let response = AuthorizeResponse::new(id_token_info);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_authorize_response_with_certificate_status() {
        let id_token_info = create_test_id_token_info();
        let certificate_status = AuthorizeCertificateStatusEnumType::Accepted;

        let response = AuthorizeResponse::new(id_token_info)
            .with_certificate_status(certificate_status.clone());

        assert_eq!(response.get_certificate_status(), Some(&certificate_status));
    }

    #[test]
    fn test_authorize_response_with_allowed_energy_transfer() {
        let id_token_info = create_test_id_token_info();
        let energy_transfer = vec![
            EnergyTransferModeEnumType::ACSinglePhase,
            EnergyTransferModeEnumType::ACThreePhase,
        ];

        let response = AuthorizeResponse::new(id_token_info)
            .with_allowed_energy_transfer(energy_transfer.clone());

        assert_eq!(response.get_allowed_energy_transfer(), Some(&energy_transfer));
    }

    #[test]
    fn test_authorize_response_allowed_energy_transfer_validation() {
        let id_token_info = create_test_id_token_info();
        let mut response = AuthorizeResponse::new(id_token_info);

        // Test empty array (should fail validation - min 1)
        response.set_allowed_energy_transfer(Some(vec![]));
        assert!(response.validate().is_err());

        // Test valid array (min 1)
        let valid_energy_transfer = vec![EnergyTransferModeEnumType::ACSinglePhase];
        response.set_allowed_energy_transfer(Some(valid_energy_transfer));
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_authorize_response_serialization() {
        let id_token_info = create_test_id_token_info();
        let response = AuthorizeResponse::new(id_token_info);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: AuthorizeResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_authorize_response_builder_pattern() {
        let id_token_info = create_test_id_token_info();
        let certificate_status = AuthorizeCertificateStatusEnumType::Accepted;
        let energy_transfer = vec![EnergyTransferModeEnumType::ACSinglePhase];
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = AuthorizeResponse::new(id_token_info.clone())
            .with_certificate_status(certificate_status.clone())
            .with_allowed_energy_transfer(energy_transfer.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_id_token_info(), &id_token_info);
        assert_eq!(response.get_certificate_status(), Some(&certificate_status));
        assert_eq!(response.get_allowed_energy_transfer(), Some(&energy_transfer));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_authorize_response_set_methods() {
        let id_token_info = create_test_id_token_info();
        let new_id_token_info = IdTokenInfoType::new(AuthorizationStatusEnumType::Blocked);
        let certificate_status = AuthorizeCertificateStatusEnumType::CertificateRevoked;
        let energy_transfer = vec![EnergyTransferModeEnumType::DC];
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = AuthorizeResponse::new(id_token_info);

        response
            .set_id_token_info(new_id_token_info.clone())
            .set_certificate_status(Some(certificate_status.clone()))
            .set_allowed_energy_transfer(Some(energy_transfer.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_id_token_info(), &new_id_token_info);
        assert_eq!(response.get_certificate_status(), Some(&certificate_status));
        assert_eq!(response.get_allowed_energy_transfer(), Some(&energy_transfer));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_authorize_request_json_round_trip() {
        let id_token = create_test_id_token();
        let certificate = "test_certificate".to_string();
        let ocsp_data = vec![create_test_ocsp_request_data()];
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = AuthorizeRequest::new(id_token)
            .with_certificate(certificate)
            .with_iso_15118_certificate_hash_data(ocsp_data)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: AuthorizeRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_authorize_response_json_round_trip() {
        let id_token_info = create_test_id_token_info();
        let certificate_status = AuthorizeCertificateStatusEnumType::Accepted;
        let energy_transfer = vec![EnergyTransferModeEnumType::ACSinglePhase];
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = AuthorizeResponse::new(id_token_info)
            .with_certificate_status(certificate_status)
            .with_allowed_energy_transfer(energy_transfer)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: AuthorizeResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_authorize_request_edge_cases() {
        let id_token = create_test_id_token();

        // Test with maximum valid certificate length
        let max_certificate = "a".repeat(10000);
        let mut request = AuthorizeRequest::new(id_token.clone());
        request.set_certificate(Some(max_certificate));
        assert!(request.validate().is_ok());

        // Test with maximum valid iso_15118_certificate_hash_data array
        let max_ocsp_data = vec![
            create_test_ocsp_request_data(),
            create_test_ocsp_request_data(),
            create_test_ocsp_request_data(),
            create_test_ocsp_request_data(),
        ];
        request.set_iso_15118_certificate_hash_data(Some(max_ocsp_data));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_authorize_response_edge_cases() {
        let id_token_info = create_test_id_token_info();
        let mut response = AuthorizeResponse::new(id_token_info);

        // Test with single energy transfer mode (minimum valid)
        let min_energy_transfer = vec![EnergyTransferModeEnumType::ACSinglePhase];
        response.set_allowed_energy_transfer(Some(min_energy_transfer));
        assert!(response.validate().is_ok());
    }
}