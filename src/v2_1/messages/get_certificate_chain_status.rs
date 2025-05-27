use crate::v2_1::datatypes::{CertificateStatusRequestInfoType, CertificateStatusType, CustomDataType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetCertificateChainStatus request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateChainStatusRequest {
    #[validate(length(min = 1, max = 4))]
    #[validate(nested)]
    pub certificate_status_requests: Vec<CertificateStatusRequestInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetCertificateChainStatusRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `certificate_status_requests` - The certificate_status_requests field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(certificate_status_requests: Vec<CertificateStatusRequestInfoType>) -> Self {
        Self {
            certificate_status_requests,
            custom_data: None,
        }
    }

    /// Sets the certificate_status_requests field.
    ///
    /// * `certificate_status_requests` - The certificate_status_requests field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_certificate_status_requests(&mut self, certificate_status_requests: Vec<CertificateStatusRequestInfoType>) -> &mut Self {
        self.certificate_status_requests = certificate_status_requests;
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

    /// Gets a reference to the certificate_status_requests field.
    ///
    /// # Returns
    ///
    /// The certificate_status_requests field
    pub fn get_certificate_status_requests(&self) -> &Vec<CertificateStatusRequestInfoType> {
        &self.certificate_status_requests
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

/// Response body for the GetCertificateChainStatus response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateChainStatusResponse {
    #[validate(length(min = 1, max = 4))]
    #[validate(nested)]
    pub certificate_status: Vec<CertificateStatusType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetCertificateChainStatusResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `certificate_status` - The certificate_status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(certificate_status: Vec<CertificateStatusType>) -> Self {
        Self {
            certificate_status,
            custom_data: None,
        }
    }

    /// Sets the certificate_status field.
    ///
    /// * `certificate_status` - The certificate_status field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_certificate_status(&mut self, certificate_status: Vec<CertificateStatusType>) -> &mut Self {
        self.certificate_status = certificate_status;
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

    /// Gets a reference to the certificate_status field.
    ///
    /// # Returns
    ///
    /// The certificate_status field
    pub fn get_certificate_status(&self) -> &Vec<CertificateStatusType> {
        &self.certificate_status
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::enumerations::{CertificateStatusEnumType, CertificateStatusSourceEnumType, HashAlgorithmEnumType};
    use crate::v2_1::datatypes::CertificateHashDataType;
    use chrono::{TimeZone, Utc};
    use serde_json;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_certificate_hash_data() -> CertificateHashDataType {
        CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "issuer_name_hash".to_string(),
            "issuer_key_hash".to_string(),
            "serial_number".to_string(),
        )
    }

    fn create_test_certificate_status_request_info() -> CertificateStatusRequestInfoType {
        CertificateStatusRequestInfoType::new(
            create_test_certificate_hash_data(),
            CertificateStatusSourceEnumType::OCSP,
            vec!["https://ocsp.example.com".to_string()],
        )
    }

    fn create_test_certificate_status() -> CertificateStatusType {
        let next_update = Utc.with_ymd_and_hms(2023, 12, 31, 23, 59, 59).unwrap();
        CertificateStatusType::new(
            create_test_certificate_hash_data(),
            CertificateStatusSourceEnumType::OCSP,
            CertificateStatusEnumType::Good,
            next_update,
        )
    }

    // Tests for GetCertificateChainStatusRequest

    #[test]
    fn test_get_certificate_chain_status_request_new() {
        let cert_status_requests = vec![create_test_certificate_status_request_info()];
        let request = GetCertificateChainStatusRequest::new(cert_status_requests.clone());

        assert_eq!(request.certificate_status_requests, cert_status_requests);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_certificate_chain_status_request_with_custom_data() {
        let cert_status_requests = vec![create_test_certificate_status_request_info()];
        let custom_data = create_test_custom_data();
        let request = GetCertificateChainStatusRequest::new(cert_status_requests.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.certificate_status_requests, cert_status_requests);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_certificate_chain_status_request_setters() {
        let cert_status_requests1 = vec![create_test_certificate_status_request_info()];
        let cert_status_requests2 = vec![
            create_test_certificate_status_request_info(),
            CertificateStatusRequestInfoType::new(
                create_test_certificate_hash_data(),
                CertificateStatusSourceEnumType::CRL,
                vec!["https://crl.example.com".to_string()],
            ),
        ];
        let custom_data = create_test_custom_data();

        let mut request = GetCertificateChainStatusRequest::new(cert_status_requests1);
        request.set_certificate_status_requests(cert_status_requests2.clone());
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.certificate_status_requests, cert_status_requests2);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_certificate_chain_status_request_getters() {
        let cert_status_requests = vec![create_test_certificate_status_request_info()];
        let custom_data = create_test_custom_data();
        let request = GetCertificateChainStatusRequest::new(cert_status_requests.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_certificate_status_requests(), &cert_status_requests);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_certificate_chain_status_request_serialization() {
        let cert_status_requests = vec![create_test_certificate_status_request_info()];
        let request = GetCertificateChainStatusRequest::new(cert_status_requests);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetCertificateChainStatusRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
    }

    #[test]
    fn test_get_certificate_chain_status_request_validation() {
        let cert_status_requests = vec![create_test_certificate_status_request_info()];
        let request = GetCertificateChainStatusRequest::new(cert_status_requests);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_get_certificate_chain_status_request_validation_empty_requests() {
        let request = GetCertificateChainStatusRequest::new(vec![]);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_certificate_chain_status_request_validation_too_many_requests() {
        let cert_status_requests = vec![
            create_test_certificate_status_request_info(),
            create_test_certificate_status_request_info(),
            create_test_certificate_status_request_info(),
            create_test_certificate_status_request_info(),
            create_test_certificate_status_request_info(), // 5 requests, max is 4
        ];
        let request = GetCertificateChainStatusRequest::new(cert_status_requests);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_certificate_chain_status_request_validation_max_requests() {
        let cert_status_requests = vec![
            create_test_certificate_status_request_info(),
            create_test_certificate_status_request_info(),
            create_test_certificate_status_request_info(),
            create_test_certificate_status_request_info(), // 4 requests, exactly at max
        ];
        let request = GetCertificateChainStatusRequest::new(cert_status_requests);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_get_certificate_chain_status_request_json_round_trip() {
        let cert_status_requests = vec![
            create_test_certificate_status_request_info(),
            CertificateStatusRequestInfoType::new(
                create_test_certificate_hash_data(),
                CertificateStatusSourceEnumType::CRL,
                vec!["https://crl.example.com".to_string()],
            ),
        ];
        let custom_data = create_test_custom_data();
        let request = GetCertificateChainStatusRequest::new(cert_status_requests)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetCertificateChainStatusRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
        assert!(parsed.validate().is_ok());
    }

    // Tests for GetCertificateChainStatusResponse

    #[test]
    fn test_get_certificate_chain_status_response_new() {
        let cert_status = vec![create_test_certificate_status()];
        let response = GetCertificateChainStatusResponse::new(cert_status.clone());

        assert_eq!(response.certificate_status, cert_status);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_certificate_chain_status_response_with_custom_data() {
        let cert_status = vec![create_test_certificate_status()];
        let custom_data = create_test_custom_data();
        let response = GetCertificateChainStatusResponse::new(cert_status.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.certificate_status, cert_status);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_certificate_chain_status_response_setters() {
        let cert_status1 = vec![create_test_certificate_status()];
        let cert_status2 = vec![
            create_test_certificate_status(),
            {
                let next_update = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
                CertificateStatusType::new(
                    create_test_certificate_hash_data(),
                    CertificateStatusSourceEnumType::CRL,
                    CertificateStatusEnumType::Revoked,
                    next_update,
                )
            },
        ];
        let custom_data = create_test_custom_data();

        let mut response = GetCertificateChainStatusResponse::new(cert_status1);
        response.set_certificate_status(cert_status2.clone());
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.certificate_status, cert_status2);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_certificate_chain_status_response_getters() {
        let cert_status = vec![create_test_certificate_status()];
        let custom_data = create_test_custom_data();
        let response = GetCertificateChainStatusResponse::new(cert_status.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_certificate_status(), &cert_status);
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_certificate_chain_status_response_serialization() {
        let cert_status = vec![create_test_certificate_status()];
        let response = GetCertificateChainStatusResponse::new(cert_status);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetCertificateChainStatusResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
    }

    #[test]
    fn test_get_certificate_chain_status_response_validation() {
        let cert_status = vec![create_test_certificate_status()];
        let response = GetCertificateChainStatusResponse::new(cert_status);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_get_certificate_chain_status_response_validation_empty_status() {
        let response = GetCertificateChainStatusResponse::new(vec![]);

        assert!(response.validate().is_err());
    }

    #[test]
    fn test_get_certificate_chain_status_response_validation_too_many_status() {
        let cert_status = vec![
            create_test_certificate_status(),
            create_test_certificate_status(),
            create_test_certificate_status(),
            create_test_certificate_status(),
            create_test_certificate_status(), // 5 statuses, max is 4
        ];
        let response = GetCertificateChainStatusResponse::new(cert_status);

        assert!(response.validate().is_err());
    }

    #[test]
    fn test_get_certificate_chain_status_response_validation_max_status() {
        let cert_status = vec![
            create_test_certificate_status(),
            create_test_certificate_status(),
            create_test_certificate_status(),
            create_test_certificate_status(), // 4 statuses, exactly at max
        ];
        let response = GetCertificateChainStatusResponse::new(cert_status);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_get_certificate_chain_status_response_json_round_trip() {
        let cert_status = vec![
            create_test_certificate_status(),
            {
                let next_update = Utc.with_ymd_and_hms(2024, 6, 15, 12, 0, 0).unwrap();
                CertificateStatusType::new(
                    create_test_certificate_hash_data(),
                    CertificateStatusSourceEnumType::CRL,
                    CertificateStatusEnumType::Unknown,
                    next_update,
                )
            },
        ];
        let custom_data = create_test_custom_data();
        let response = GetCertificateChainStatusResponse::new(cert_status)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetCertificateChainStatusResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
        assert!(parsed.validate().is_ok());
    }

    #[test]
    fn test_get_certificate_chain_status_all_certificate_status_types() {
        let statuses = vec![
            CertificateStatusEnumType::Good,
            CertificateStatusEnumType::Revoked,
            CertificateStatusEnumType::Unknown,
            CertificateStatusEnumType::Failed,
        ];

        for status in statuses {
            let next_update = Utc.with_ymd_and_hms(2023, 12, 31, 23, 59, 59).unwrap();
            let cert_status = CertificateStatusType::new(
                create_test_certificate_hash_data(),
                CertificateStatusSourceEnumType::OCSP,
                status.clone(),
                next_update,
            );
            let response = GetCertificateChainStatusResponse::new(vec![cert_status]);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_get_certificate_chain_status_all_source_types() {
        let sources = vec![
            CertificateStatusSourceEnumType::OCSP,
            CertificateStatusSourceEnumType::CRL,
        ];

        for source in sources {
            let next_update = Utc.with_ymd_and_hms(2023, 12, 31, 23, 59, 59).unwrap();
            let cert_status = CertificateStatusType::new(
                create_test_certificate_hash_data(),
                source.clone(),
                CertificateStatusEnumType::Good,
                next_update,
            );
            let response = GetCertificateChainStatusResponse::new(vec![cert_status]);
            assert!(response.validate().is_ok());
        }
    }
}