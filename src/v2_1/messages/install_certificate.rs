use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{InstallCertificateStatusEnumType, InstallCertificateUseEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the InstallCertificate request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificateRequest {
    pub certificate_type: InstallCertificateUseEnumType,

    /// A PEM encoded X.509 certificate.
    #[validate(length(max = 10000))]
    pub certificate: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl InstallCertificateRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `certificate_type` - The certificate_type field
    /// * `certificate` - A PEM encoded X.509 certificate.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(certificate_type: InstallCertificateUseEnumType, certificate: String) -> Self {
        Self {
            certificate_type,
            certificate,
            custom_data: None,
        }
    }

    /// Sets the certificate_type field.
    ///
    /// * `certificate_type` - The certificate_type field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_certificate_type(&mut self, certificate_type: InstallCertificateUseEnumType) -> &mut Self {
        self.certificate_type = certificate_type;
        self
    }

    /// Sets the certificate field.
    ///
    /// * `certificate` - A PEM encoded X.509 certificate.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_certificate(&mut self, certificate: String) -> &mut Self {
        self.certificate = certificate;
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

    /// Gets a reference to the certificate_type field.
    ///
    /// # Returns
    ///
    /// The certificate_type field
    pub fn get_certificate_type(&self) -> &InstallCertificateUseEnumType {
        &self.certificate_type
    }

    /// Gets a reference to the certificate field.
    ///
    /// # Returns
    ///
    /// A PEM encoded X.509 certificate.
    pub fn get_certificate(&self) -> &String {
        &self.certificate
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

/// Response body for the InstallCertificate response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificateResponse {
    pub status: InstallCertificateStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl InstallCertificateResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: InstallCertificateStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: InstallCertificateStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &InstallCertificateStatusEnumType {
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

    // Tests for InstallCertificateRequest

    #[test]
    fn test_install_certificate_request_new() {
        let certificate_type = InstallCertificateUseEnumType::CSMSRootCertificate;
        let certificate = "test_certificate_data".to_string();
        let request = InstallCertificateRequest::new(certificate_type.clone(), certificate.clone());

        assert_eq!(request.certificate_type, certificate_type);
        assert_eq!(request.certificate, certificate);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_install_certificate_request_with_custom_data() {
        let certificate_type = InstallCertificateUseEnumType::V2GRootCertificate;
        let certificate = "another_certificate_data".to_string();
        let custom_data = create_test_custom_data();
        let request = InstallCertificateRequest::new(certificate_type.clone(), certificate.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.certificate_type, certificate_type);
        assert_eq!(request.certificate, certificate);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_install_certificate_request_setters() {
        let certificate_type1 = InstallCertificateUseEnumType::CSMSRootCertificate;
        let certificate_type2 = InstallCertificateUseEnumType::MORootCertificate;
        let certificate1 = "cert1".to_string();
        let certificate2 = "cert2".to_string();
        let custom_data = create_test_custom_data();

        let mut request = InstallCertificateRequest::new(certificate_type1, certificate1);
        request.set_certificate_type(certificate_type2.clone());
        request.set_certificate(certificate2.clone());
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.certificate_type, certificate_type2);
        assert_eq!(request.certificate, certificate2);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_install_certificate_request_getters() {
        let certificate_type = InstallCertificateUseEnumType::ManufacturerRootCertificate;
        let certificate = "test_cert_data".to_string();
        let custom_data = create_test_custom_data();
        let request = InstallCertificateRequest::new(certificate_type.clone(), certificate.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_certificate_type(), &certificate_type);
        assert_eq!(request.get_certificate(), &certificate);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_install_certificate_request_serialization() {
        let certificate_type = InstallCertificateUseEnumType::CSMSRootCertificate;
        let certificate = "test_certificate".to_string();
        let request = InstallCertificateRequest::new(certificate_type, certificate);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: InstallCertificateRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
    }

    #[test]
    fn test_install_certificate_request_validation() {
        let certificate_type = InstallCertificateUseEnumType::CSMSRootCertificate;
        let certificate = "valid_certificate".to_string();
        let request = InstallCertificateRequest::new(certificate_type, certificate);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_install_certificate_request_validation_long_certificate() {
        let certificate_type = InstallCertificateUseEnumType::CSMSRootCertificate;
        let long_certificate = "a".repeat(10001); // Exceeds max length of 10000
        let mut request = InstallCertificateRequest::new(certificate_type, "valid".to_string());
        request.set_certificate(long_certificate);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_install_certificate_request_validation_max_certificate() {
        let certificate_type = InstallCertificateUseEnumType::CSMSRootCertificate;
        let max_certificate = "a".repeat(10000); // Exactly at max length
        let request = InstallCertificateRequest::new(certificate_type, max_certificate);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_install_certificate_request_all_certificate_types() {
        let certificate_types = vec![
            InstallCertificateUseEnumType::V2GRootCertificate,
            InstallCertificateUseEnumType::MORootCertificate,
            InstallCertificateUseEnumType::CSMSRootCertificate,
            InstallCertificateUseEnumType::ManufacturerRootCertificate,
            InstallCertificateUseEnumType::OEMRootCertificate,
        ];

        for cert_type in certificate_types {
            let request = InstallCertificateRequest::new(cert_type.clone(), "test_cert".to_string());
            assert_eq!(request.certificate_type, cert_type);
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_install_certificate_request_json_round_trip() {
        let certificate_type = InstallCertificateUseEnumType::V2GRootCertificate;
        let certificate = "test_certificate_data".to_string();
        let custom_data = create_test_custom_data();
        let request = InstallCertificateRequest::new(certificate_type, certificate)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: InstallCertificateRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
        assert!(parsed.validate().is_ok());
    }

    // Tests for InstallCertificateResponse

    #[test]
    fn test_install_certificate_response_new() {
        let response = InstallCertificateResponse::new(InstallCertificateStatusEnumType::Accepted);

        assert_eq!(response.status, InstallCertificateStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_install_certificate_response_with_status_info() {
        let status_info = create_test_status_info();
        let response = InstallCertificateResponse::new(InstallCertificateStatusEnumType::Rejected)
            .with_status_info(status_info.clone());

        assert_eq!(response.status, InstallCertificateStatusEnumType::Rejected);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_install_certificate_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = InstallCertificateResponse::new(InstallCertificateStatusEnumType::Failed)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, InstallCertificateStatusEnumType::Failed);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_install_certificate_response_setters() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let mut response = InstallCertificateResponse::new(InstallCertificateStatusEnumType::Accepted);
        response.set_status(InstallCertificateStatusEnumType::Failed);
        response.set_status_info(Some(status_info.clone()));
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, InstallCertificateStatusEnumType::Failed);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_install_certificate_response_getters() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();
        let response = InstallCertificateResponse::new(InstallCertificateStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &InstallCertificateStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_install_certificate_response_serialization() {
        let response = InstallCertificateResponse::new(InstallCertificateStatusEnumType::Accepted);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: InstallCertificateResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
    }

    #[test]
    fn test_install_certificate_response_validation() {
        let response = InstallCertificateResponse::new(InstallCertificateStatusEnumType::Accepted);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_install_certificate_response_all_status_types() {
        let statuses = vec![
            InstallCertificateStatusEnumType::Accepted,
            InstallCertificateStatusEnumType::Rejected,
            InstallCertificateStatusEnumType::Failed,
        ];

        for status in statuses {
            let response = InstallCertificateResponse::new(status.clone());
            assert_eq!(response.status, status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_install_certificate_response_json_round_trip() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();
        let response = InstallCertificateResponse::new(InstallCertificateStatusEnumType::Failed)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: InstallCertificateResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
        assert!(parsed.validate().is_ok());
    }
}