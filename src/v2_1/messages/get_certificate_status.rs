use crate::v2_1::datatypes::{CustomDataType, OCSPRequestDataType, StatusInfoType};
use crate::v2_1::enumerations::GetCertificateStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetCertificateStatus request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateStatusRequest {
    #[validate(nested)]
    pub ocsp_request_data: OCSPRequestDataType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetCertificateStatusRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `ocsp_request_data` - The ocsp_request_data field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(ocsp_request_data: OCSPRequestDataType) -> Self {
        Self {
            ocsp_request_data,
            custom_data: None,
        }
    }

    /// Sets the ocsp_request_data field.
    ///
    /// * `ocsp_request_data` - The ocsp_request_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_ocsp_request_data(&mut self, ocsp_request_data: OCSPRequestDataType) -> &mut Self {
        self.ocsp_request_data = ocsp_request_data;
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

    /// Gets a reference to the ocsp_request_data field.
    ///
    /// # Returns
    ///
    /// The ocsp_request_data field
    pub fn get_ocsp_request_data(&self) -> &OCSPRequestDataType {
        &self.ocsp_request_data
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

/// Response body for the GetCertificateStatus response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateStatusResponse {
    pub status: GetCertificateStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// *(2.1)* OCSPResponse class as defined in &lt;&lt;ref-ocpp_security_24, IETF RFC 6960&gt;&gt;. DER encoded (as defined in &lt;&lt;ref-ocpp_security_24, IETF RFC 6960&gt;&gt;), and then base64 encoded. MAY only be omitted when status is not Accepted. + The minimum supported length is 18000. If a longer _ocspResult_ is supported, then the supported length must be communicated in variable OCPPCommCtrlr.FieldLength[ "GetCertificateStatusResponse.ocspResult" ].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 18000))]
    pub ocsp_result: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetCertificateStatusResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: GetCertificateStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
            ocsp_result: None,
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
    pub fn set_status(&mut self, status: GetCertificateStatusEnumType) -> &mut Self {
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

    /// Sets the ocsp_result field.
    ///
    /// * `ocsp_result` - *(2.1)* OCSPResponse class as defined in &lt;&lt;ref-ocpp_security_24, IETF RFC 6960&gt;&gt;. DER encoded (as defined in &lt;&lt;ref-ocpp_security_24, IETF RFC 6960&gt;&gt;), and then base64 encoded. MAY only be omitted when status is not Accepted. + The minimum supported length is 18000. If a longer _ocspResult_ is supported, then the supported length must be communicated in variable OCPPCommCtrlr.FieldLength[ "GetCertificateStatusResponse.ocspResult" ].
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_ocsp_result(&mut self, ocsp_result: Option<String>) -> &mut Self {
        self.ocsp_result = ocsp_result;
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
    pub fn get_status(&self) -> &GetCertificateStatusEnumType {
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

    /// Gets a reference to the ocsp_result field.
    ///
    /// # Returns
    ///
    /// *(2.1)* OCSPResponse class as defined in &lt;&lt;ref-ocpp_security_24, IETF RFC 6960&gt;&gt;. DER encoded (as defined in &lt;&lt;ref-ocpp_security_24, IETF RFC 6960&gt;&gt;), and then base64 encoded. MAY only be omitted when status is not Accepted. + The minimum supported length is 18000. If a longer _ocspResult_ is supported, then the supported length must be communicated in variable OCPPCommCtrlr.FieldLength[ "GetCertificateStatusResponse.ocspResult" ].
    pub fn get_ocsp_result(&self) -> Option<&String> {
        self.ocsp_result.as_ref()
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

    /// Sets the ocsp_result field and returns self for builder pattern.
    ///
    /// * `ocsp_result` - *(2.1)* OCSPResponse class as defined in &lt;&lt;ref-ocpp_security_24, IETF RFC 6960&gt;&gt;. DER encoded (as defined in &lt;&lt;ref-ocpp_security_24, IETF RFC 6960&gt;&gt;), and then base64 encoded. MAY only be omitted when status is not Accepted. + The minimum supported length is 18000. If a longer _ocspResult_ is supported, then the supported length must be communicated in variable OCPPCommCtrlr.FieldLength[ "GetCertificateStatusResponse.ocspResult" ].
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_ocsp_result(mut self, ocsp_result: String) -> Self {
        self.ocsp_result = Some(ocsp_result);
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

    fn create_test_ocsp_request_data() -> OCSPRequestDataType {
        OCSPRequestDataType::new(
            HashAlgorithmEnumType::SHA256,
            "issuer_name_hash".to_string(),
            "issuer_key_hash".to_string(),
            "serial_number".to_string(),
            "https://ocsp.example.com".to_string(),
        )
    }

    // Tests for GetCertificateStatusRequest

    #[test]
    fn test_get_certificate_status_request_new() {
        let ocsp_request_data = create_test_ocsp_request_data();
        let request = GetCertificateStatusRequest::new(ocsp_request_data.clone());

        assert_eq!(request.ocsp_request_data, ocsp_request_data);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_certificate_status_request_with_custom_data() {
        let ocsp_request_data = create_test_ocsp_request_data();
        let custom_data = create_test_custom_data();
        let request = GetCertificateStatusRequest::new(ocsp_request_data.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.ocsp_request_data, ocsp_request_data);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_certificate_status_request_setters() {
        let ocsp_request_data1 = create_test_ocsp_request_data();
        let ocsp_request_data2 = OCSPRequestDataType::new(
            HashAlgorithmEnumType::SHA512,
            "new_issuer_name_hash".to_string(),
            "new_issuer_key_hash".to_string(),
            "new_serial_number".to_string(),
            "https://ocsp.example.org".to_string(),
        );
        let custom_data = create_test_custom_data();

        let mut request = GetCertificateStatusRequest::new(ocsp_request_data1);
        request.set_ocsp_request_data(ocsp_request_data2.clone());
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.ocsp_request_data, ocsp_request_data2);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_certificate_status_request_getters() {
        let ocsp_request_data = create_test_ocsp_request_data();
        let custom_data = create_test_custom_data();
        let request = GetCertificateStatusRequest::new(ocsp_request_data.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_ocsp_request_data(), &ocsp_request_data);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_certificate_status_request_serialization() {
        let ocsp_request_data = create_test_ocsp_request_data();
        let request = GetCertificateStatusRequest::new(ocsp_request_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetCertificateStatusRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
    }

    #[test]
    fn test_get_certificate_status_request_validation() {
        let ocsp_request_data = create_test_ocsp_request_data();
        let request = GetCertificateStatusRequest::new(ocsp_request_data);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_get_certificate_status_request_json_round_trip() {
        let ocsp_request_data = create_test_ocsp_request_data();
        let custom_data = create_test_custom_data();
        let request = GetCertificateStatusRequest::new(ocsp_request_data)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetCertificateStatusRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
        assert!(parsed.validate().is_ok());
    }

    // Tests for GetCertificateStatusResponse

    #[test]
    fn test_get_certificate_status_response_new() {
        let response = GetCertificateStatusResponse::new(GetCertificateStatusEnumType::Accepted);

        assert_eq!(response.status, GetCertificateStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.ocsp_result, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_certificate_status_response_with_status_info() {
        let status_info = create_test_status_info();
        let response = GetCertificateStatusResponse::new(GetCertificateStatusEnumType::Failed)
            .with_status_info(status_info.clone());

        assert_eq!(response.status, GetCertificateStatusEnumType::Failed);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.ocsp_result, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_certificate_status_response_with_ocsp_result() {
        let ocsp_result = "base64_encoded_ocsp_response".to_string();
        let response = GetCertificateStatusResponse::new(GetCertificateStatusEnumType::Accepted)
            .with_ocsp_result(ocsp_result.clone());

        assert_eq!(response.status, GetCertificateStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.ocsp_result, Some(ocsp_result));
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_certificate_status_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = GetCertificateStatusResponse::new(GetCertificateStatusEnumType::Accepted)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, GetCertificateStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.ocsp_result, None);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_certificate_status_response_setters() {
        let status_info = create_test_status_info();
        let ocsp_result = "base64_encoded_ocsp_response".to_string();
        let custom_data = create_test_custom_data();

        let mut response = GetCertificateStatusResponse::new(GetCertificateStatusEnumType::Accepted);
        response.set_status(GetCertificateStatusEnumType::Failed);
        response.set_status_info(Some(status_info.clone()));
        response.set_ocsp_result(Some(ocsp_result.clone()));
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, GetCertificateStatusEnumType::Failed);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.ocsp_result, Some(ocsp_result));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_certificate_status_response_getters() {
        let status_info = create_test_status_info();
        let ocsp_result = "base64_encoded_ocsp_response".to_string();
        let custom_data = create_test_custom_data();
        let response = GetCertificateStatusResponse::new(GetCertificateStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_ocsp_result(ocsp_result.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &GetCertificateStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_ocsp_result(), Some(&ocsp_result));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_certificate_status_response_serialization() {
        let response = GetCertificateStatusResponse::new(GetCertificateStatusEnumType::Accepted);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetCertificateStatusResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
    }

    #[test]
    fn test_get_certificate_status_response_validation() {
        let response = GetCertificateStatusResponse::new(GetCertificateStatusEnumType::Accepted);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_get_certificate_status_response_validation_long_ocsp_result() {
        let long_ocsp_result = "a".repeat(18001); // Exceeds max length of 18000
        let mut response = GetCertificateStatusResponse::new(GetCertificateStatusEnumType::Accepted);
        response.set_ocsp_result(Some(long_ocsp_result));

        assert!(response.validate().is_err());
    }

    #[test]
    fn test_get_certificate_status_response_validation_max_ocsp_result() {
        let max_ocsp_result = "a".repeat(18000); // Exactly at max length
        let mut response = GetCertificateStatusResponse::new(GetCertificateStatusEnumType::Accepted);
        response.set_ocsp_result(Some(max_ocsp_result));

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_get_certificate_status_response_all_status_types() {
        let statuses = vec![
            GetCertificateStatusEnumType::Accepted,
            GetCertificateStatusEnumType::Failed,
        ];

        for status in statuses {
            let response = GetCertificateStatusResponse::new(status.clone());
            assert_eq!(response.status, status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_get_certificate_status_response_json_round_trip() {
        let status_info = create_test_status_info();
        let ocsp_result = "base64_encoded_ocsp_response".to_string();
        let custom_data = create_test_custom_data();
        let response = GetCertificateStatusResponse::new(GetCertificateStatusEnumType::Accepted)
            .with_status_info(status_info)
            .with_ocsp_result(ocsp_result)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetCertificateStatusResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
        assert!(parsed.validate().is_ok());
    }
}