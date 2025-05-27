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
