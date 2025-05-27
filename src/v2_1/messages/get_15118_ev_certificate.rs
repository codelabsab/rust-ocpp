use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{CertificateActionEnumType, Iso15118EVCertificateStatusEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the Get15118EVCertificate request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Get15118EVCertificateRequest {
    /// Schema version currently used for the 15118 session between EV and Charging Station. Needed for parsing of the EXI stream by the CSMS.
    #[validate(length(max = 50))]
    pub iso_15118_schema_version: String,

    pub action: CertificateActionEnumType,

    /// *(2.1)* Raw CertificateInstallationReq request from EV, Base64 encoded. + Extended to support ISO 15118-20 certificates. The minimum supported length is 11000. If a longer _exiRequest_ is supported, then the supported length must be communicated in variable OCPPCommCtrlr.FieldLength[ "Get15118EVCertificateRequest.exiRequest" ].
    #[validate(length(max = 11000))]
    pub exi_request: String,

    /// *(2.1)* Absent during ISO 15118-2 session. Required during ISO 15118-20 session. + Maximum number of contracts that EV wants to install.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub maximum_contract_certificate_chains: Option<i32>,

    /// *(2.1)*  Absent during ISO 15118-2 session. Optional during ISO 15118-20 session. List of EMAIDs for which contract certificates must be requested first, in case there are more certificates than allowed by _maximumContractCertificateChains_.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 8))]
    pub prioritized_emai_ds: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl Get15118EVCertificateRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `iso_15118_schema_version` - Schema version currently used for the 15118 session between EV and Charging Station. Needed for parsing of the EXI stream by the CSMS.
    /// * `action` - The action field
    /// * `exi_request` - *(2.1)* Raw CertificateInstallationReq request from EV, Base64 encoded. + Extended to support ISO 15118-20 certificates. The minimum supported length is 11000. If a longer _exiRequest_ is supported, then the supported length must be communicated in variable OCPPCommCtrlr.FieldLength[ "Get15118EVCertificateRequest.exiRequest" ].
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(iso_15118_schema_version: String, action: CertificateActionEnumType, exi_request: String) -> Self {
        Self {
            iso_15118_schema_version,
            action,
            exi_request,
            maximum_contract_certificate_chains: None,
            prioritized_emai_ds: None,
            custom_data: None,
        }
    }

    /// Sets the iso_15118_schema_version field.
    ///
    /// * `iso_15118_schema_version` - Schema version currently used for the 15118 session between EV and Charging Station. Needed for parsing of the EXI stream by the CSMS.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_iso_15118_schema_version(&mut self, iso_15118_schema_version: String) -> &mut Self {
        self.iso_15118_schema_version = iso_15118_schema_version;
        self
    }

    /// Sets the action field.
    ///
    /// * `action` - The action field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_action(&mut self, action: CertificateActionEnumType) -> &mut Self {
        self.action = action;
        self
    }

    /// Sets the exi_request field.
    ///
    /// * `exi_request` - *(2.1)* Raw CertificateInstallationReq request from EV, Base64 encoded. + Extended to support ISO 15118-20 certificates. The minimum supported length is 11000. If a longer _exiRequest_ is supported, then the supported length must be communicated in variable OCPPCommCtrlr.FieldLength[ "Get15118EVCertificateRequest.exiRequest" ].
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_exi_request(&mut self, exi_request: String) -> &mut Self {
        self.exi_request = exi_request;
        self
    }

    /// Sets the maximum_contract_certificate_chains field.
    ///
    /// * `maximum_contract_certificate_chains` - *(2.1)* Absent during ISO 15118-2 session. Required during ISO 15118-20 session. + Maximum number of contracts that EV wants to install.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_maximum_contract_certificate_chains(&mut self, maximum_contract_certificate_chains: Option<i32>) -> &mut Self {
        self.maximum_contract_certificate_chains = maximum_contract_certificate_chains;
        self
    }

    /// Sets the prioritized_emai_ds field.
    ///
    /// * `prioritized_emai_ds` - *(2.1)*  Absent during ISO 15118-2 session. Optional during ISO 15118-20 session. List of EMAIDs for which contract certificates must be requested first, in case there are more certificates than allowed by _maximumContractCertificateChains_.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_prioritized_emai_ds(&mut self, prioritized_emai_ds: Option<Vec<String>>) -> &mut Self {
        self.prioritized_emai_ds = prioritized_emai_ds;
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

    /// Gets a reference to the iso_15118_schema_version field.
    ///
    /// # Returns
    ///
    /// Schema version currently used for the 15118 session between EV and Charging Station. Needed for parsing of the EXI stream by the CSMS.
    pub fn get_iso_15118_schema_version(&self) -> &String {
        &self.iso_15118_schema_version
    }

    /// Gets a reference to the action field.
    ///
    /// # Returns
    ///
    /// The action field
    pub fn get_action(&self) -> &CertificateActionEnumType {
        &self.action
    }

    /// Gets a reference to the exi_request field.
    ///
    /// # Returns
    ///
    /// *(2.1)* Raw CertificateInstallationReq request from EV, Base64 encoded. + Extended to support ISO 15118-20 certificates. The minimum supported length is 11000. If a longer _exiRequest_ is supported, then the supported length must be communicated in variable OCPPCommCtrlr.FieldLength[ "Get15118EVCertificateRequest.exiRequest" ].
    pub fn get_exi_request(&self) -> &String {
        &self.exi_request
    }

    /// Gets a reference to the maximum_contract_certificate_chains field.
    ///
    /// # Returns
    ///
    /// *(2.1)* Absent during ISO 15118-2 session. Required during ISO 15118-20 session. + Maximum number of contracts that EV wants to install.
    pub fn get_maximum_contract_certificate_chains(&self) -> Option<&i32> {
        self.maximum_contract_certificate_chains.as_ref()
    }

    /// Gets a reference to the prioritized_emai_ds field.
    ///
    /// # Returns
    ///
    /// *(2.1)*  Absent during ISO 15118-2 session. Optional during ISO 15118-20 session. List of EMAIDs for which contract certificates must be requested first, in case there are more certificates than allowed by _maximumContractCertificateChains_.
    pub fn get_prioritized_emai_ds(&self) -> Option<&Vec<String>> {
        self.prioritized_emai_ds.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the maximum_contract_certificate_chains field and returns self for builder pattern.
    ///
    /// * `maximum_contract_certificate_chains` - *(2.1)* Absent during ISO 15118-2 session. Required during ISO 15118-20 session. + Maximum number of contracts that EV wants to install.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_maximum_contract_certificate_chains(mut self, maximum_contract_certificate_chains: i32) -> Self {
        self.maximum_contract_certificate_chains = Some(maximum_contract_certificate_chains);
        self
    }

    /// Sets the prioritized_emai_ds field and returns self for builder pattern.
    ///
    /// * `prioritized_emai_ds` - *(2.1)*  Absent during ISO 15118-2 session. Optional during ISO 15118-20 session. List of EMAIDs for which contract certificates must be requested first, in case there are more certificates than allowed by _maximumContractCertificateChains_.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_prioritized_emai_ds(mut self, prioritized_emai_ds: Vec<String>) -> Self {
        self.prioritized_emai_ds = Some(prioritized_emai_ds);
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

/// Response body for the Get15118EVCertificate response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Get15118EVCertificateResponse {
    pub status: Iso15118EVCertificateStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// *(2/1)* Raw CertificateInstallationRes response for the EV, Base64 encoded. + Extended to support ISO 15118-20 certificates. The minimum supported length is 17000. If a longer _exiResponse_ is supported, then the supported length must be communicated in variable OCPPCommCtrlr.FieldLength[ "Get15118EVCertificateResponse.exiResponse" ].
    #[validate(length(max = 17000))]
    pub exi_response: String,

    /// *(2.1)* Number of contracts that can be retrieved with additional requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub remaining_contracts: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl Get15118EVCertificateResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    /// * `exi_response` - *(2/1)* Raw CertificateInstallationRes response for the EV, Base64 encoded. + Extended to support ISO 15118-20 certificates. The minimum supported length is 17000. If a longer _exiResponse_ is supported, then the supported length must be communicated in variable OCPPCommCtrlr.FieldLength[ "Get15118EVCertificateResponse.exiResponse" ].
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: Iso15118EVCertificateStatusEnumType, exi_response: String) -> Self {
        Self {
            status,
            status_info: None,
            exi_response,
            remaining_contracts: None,
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
    pub fn set_status(&mut self, status: Iso15118EVCertificateStatusEnumType) -> &mut Self {
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

    /// Sets the exi_response field.
    ///
    /// * `exi_response` - *(2/1)* Raw CertificateInstallationRes response for the EV, Base64 encoded. + Extended to support ISO 15118-20 certificates. The minimum supported length is 17000. If a longer _exiResponse_ is supported, then the supported length must be communicated in variable OCPPCommCtrlr.FieldLength[ "Get15118EVCertificateResponse.exiResponse" ].
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_exi_response(&mut self, exi_response: String) -> &mut Self {
        self.exi_response = exi_response;
        self
    }

    /// Sets the remaining_contracts field.
    ///
    /// * `remaining_contracts` - *(2.1)* Number of contracts that can be retrieved with additional requests.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_remaining_contracts(&mut self, remaining_contracts: Option<i32>) -> &mut Self {
        self.remaining_contracts = remaining_contracts;
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
    pub fn get_status(&self) -> &Iso15118EVCertificateStatusEnumType {
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

    /// Gets a reference to the exi_response field.
    ///
    /// # Returns
    ///
    /// *(2/1)* Raw CertificateInstallationRes response for the EV, Base64 encoded. + Extended to support ISO 15118-20 certificates. The minimum supported length is 17000. If a longer _exiResponse_ is supported, then the supported length must be communicated in variable OCPPCommCtrlr.FieldLength[ "Get15118EVCertificateResponse.exiResponse" ].
    pub fn get_exi_response(&self) -> &String {
        &self.exi_response
    }

    /// Gets a reference to the remaining_contracts field.
    ///
    /// # Returns
    ///
    /// *(2.1)* Number of contracts that can be retrieved with additional requests.
    pub fn get_remaining_contracts(&self) -> Option<&i32> {
        self.remaining_contracts.as_ref()
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

    /// Sets the remaining_contracts field and returns self for builder pattern.
    ///
    /// * `remaining_contracts` - *(2.1)* Number of contracts that can be retrieved with additional requests.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_remaining_contracts(mut self, remaining_contracts: i32) -> Self {
        self.remaining_contracts = Some(remaining_contracts);
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

    // Tests for Get15118EVCertificateRequest
    
    #[test]
    fn test_get_15118_ev_certificate_request_new() {
        let request = Get15118EVCertificateRequest::new(
            "urn:iso:15118:2:2013:MsgDef".to_string(),
            CertificateActionEnumType::Install,
            "YmFzZTY0X2VuY29kZWRfZXhhbXBsZQ==".to_string()
        );
        
        assert_eq!(request.iso_15118_schema_version, "urn:iso:15118:2:2013:MsgDef");
        assert_eq!(request.action, CertificateActionEnumType::Install);
        assert_eq!(request.exi_request, "YmFzZTY0X2VuY29kZWRfZXhhbXBsZQ==");
        assert_eq!(request.maximum_contract_certificate_chains, None);
        assert_eq!(request.prioritized_emai_ds, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_15118_ev_certificate_request_with_optional_fields() {
        let request = Get15118EVCertificateRequest::new(
            "urn:iso:15118:2:2013:MsgDef".to_string(),
            CertificateActionEnumType::Update,
            "YmFzZTY0X2VuY29kZWRfZXhhbXBsZQ==".to_string()
        )
        .with_maximum_contract_certificate_chains(5)
        .with_prioritized_emai_ds(vec!["DE-ABC-123456".to_string(), "DE-XYZ-789012".to_string()])
        .with_custom_data(CustomDataType::new("Vendor".to_string()));
        
        assert_eq!(request.maximum_contract_certificate_chains, Some(5));
        assert_eq!(request.prioritized_emai_ds, Some(vec!["DE-ABC-123456".to_string(), "DE-XYZ-789012".to_string()]));
        assert!(request.custom_data.is_some());
    }

    #[test]
    fn test_get_15118_ev_certificate_request_setters() {
        let mut request = Get15118EVCertificateRequest::new(
            "urn:iso:15118:2:2013:MsgDef".to_string(),
            CertificateActionEnumType::Install,
            "YmFzZTY0X2VuY29kZWRfZXhhbXBsZQ==".to_string()
        );
        
        request.set_iso_15118_schema_version("urn:iso:15118:20:2022:MsgDef".to_string());
        request.set_action(CertificateActionEnumType::Update);
        request.set_exi_request("bmV3X2V4aV9yZXF1ZXN0".to_string());
        request.set_maximum_contract_certificate_chains(Some(10));
        request.set_prioritized_emai_ds(Some(vec!["EMAID1".to_string()]));
        request.set_custom_data(Some(CustomDataType::new("TestVendor".to_string())));
        
        assert_eq!(request.iso_15118_schema_version, "urn:iso:15118:20:2022:MsgDef");
        assert_eq!(request.action, CertificateActionEnumType::Update);
        assert_eq!(request.exi_request, "bmV3X2V4aV9yZXF1ZXN0");
        assert_eq!(request.maximum_contract_certificate_chains, Some(10));
        assert_eq!(request.prioritized_emai_ds, Some(vec!["EMAID1".to_string()]));
        assert!(request.custom_data.is_some());
    }

    #[test]
    fn test_get_15118_ev_certificate_request_getters() {
        let request = Get15118EVCertificateRequest::new(
            "urn:iso:15118:2:2013:MsgDef".to_string(),
            CertificateActionEnumType::Install,
            "YmFzZTY0X2VuY29kZWRfZXhhbXBsZQ==".to_string()
        )
        .with_maximum_contract_certificate_chains(3)
        .with_prioritized_emai_ds(vec!["EMAID1".to_string()]);
        
        assert_eq!(request.get_iso_15118_schema_version(), "urn:iso:15118:2:2013:MsgDef");
        assert_eq!(*request.get_action(), CertificateActionEnumType::Install);
        assert_eq!(request.get_exi_request(), "YmFzZTY0X2VuY29kZWRfZXhhbXBsZQ==");
        assert_eq!(request.get_maximum_contract_certificate_chains(), Some(&3));
        assert_eq!(request.get_prioritized_emai_ds(), Some(&vec!["EMAID1".to_string()]));
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_get_15118_ev_certificate_request_serialization() {
        let request = Get15118EVCertificateRequest::new(
            "urn:iso:15118:2:2013:MsgDef".to_string(),
            CertificateActionEnumType::Install,
            "YmFzZTY0X2VuY29kZWRfZXhhbXBsZQ==".to_string()
        );
        
        let json = serde_json::to_string(&request).unwrap();
        let parsed: Get15118EVCertificateRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(request, parsed);
    }

    #[test]
    fn test_get_15118_ev_certificate_request_deserialization() {
        let json = r#"{
            "iso15118SchemaVersion": "urn:iso:15118:2:2013:MsgDef",
            "action": "Install",
            "exiRequest": "YmFzZTY0X2VuY29kZWRfZXhhbXBsZQ==",
            "maximumContractCertificateChains": 5,
            "prioritizedEmaiDs": ["DE-ABC-123456", "DE-XYZ-789012"]
        }"#;
        
        let request: Get15118EVCertificateRequest = serde_json::from_str(json).unwrap();
        
        assert_eq!(request.iso_15118_schema_version, "urn:iso:15118:2:2013:MsgDef");
        assert_eq!(request.action, CertificateActionEnumType::Install);
        assert_eq!(request.exi_request, "YmFzZTY0X2VuY29kZWRfZXhhbXBsZQ==");
        assert_eq!(request.maximum_contract_certificate_chains, Some(5));
        assert_eq!(request.prioritized_emai_ds, Some(vec!["DE-ABC-123456".to_string(), "DE-XYZ-789012".to_string()]));
    }

    #[test]
    fn test_get_15118_ev_certificate_request_validation_schema_version_too_long() {
        let request = Get15118EVCertificateRequest::new(
            "a".repeat(51), // 51 characters, exceeds max length of 50
            CertificateActionEnumType::Install,
            "YmFzZTY0X2VuY29kZWRfZXhhbXBsZQ==".to_string()
        );
        
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_15118_ev_certificate_request_validation_exi_request_too_long() {
        let request = Get15118EVCertificateRequest::new(
            "urn:iso:15118:2:2013:MsgDef".to_string(),
            CertificateActionEnumType::Install,
            "a".repeat(11001) // 11001 characters, exceeds max length of 11000
        );
        
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_15118_ev_certificate_request_validation_negative_contract_chains() {
        let mut request = Get15118EVCertificateRequest::new(
            "urn:iso:15118:2:2013:MsgDef".to_string(),
            CertificateActionEnumType::Install,
            "YmFzZTY0X2VuY29kZWRfZXhhbXBsZQ==".to_string()
        );
        request.set_maximum_contract_certificate_chains(Some(-1));
        
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_15118_ev_certificate_request_validation_empty_prioritized_emai_ds() {
        let mut request = Get15118EVCertificateRequest::new(
            "urn:iso:15118:2:2013:MsgDef".to_string(),
            CertificateActionEnumType::Install,
            "YmFzZTY0X2VuY29kZWRfZXhhbXBsZQ==".to_string()
        );
        request.set_prioritized_emai_ds(Some(vec![])); // Empty vector, min length is 1
        
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_15118_ev_certificate_request_validation_too_many_prioritized_emai_ds() {
        let mut request = Get15118EVCertificateRequest::new(
            "urn:iso:15118:2:2013:MsgDef".to_string(),
            CertificateActionEnumType::Install,
            "YmFzZTY0X2VuY29kZWRfZXhhbXBsZQ==".to_string()
        );
        let emai_ds: Vec<String> = (0..9).map(|i| format!("EMAID{}", i)).collect(); // 9 items, max is 8
        request.set_prioritized_emai_ds(Some(emai_ds));
        
        assert!(request.validate().is_err());
    }

    // Tests for Get15118EVCertificateResponse
    
    #[test]
    fn test_get_15118_ev_certificate_response_new() {
        let response = Get15118EVCertificateResponse::new(
            Iso15118EVCertificateStatusEnumType::Accepted,
            "YmFzZTY0X2VuY29kZWRfcmVzcG9uc2U=".to_string()
        );
        
        assert_eq!(response.status, Iso15118EVCertificateStatusEnumType::Accepted);
        assert_eq!(response.exi_response, "YmFzZTY0X2VuY29kZWRfcmVzcG9uc2U=");
        assert_eq!(response.status_info, None);
        assert_eq!(response.remaining_contracts, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_15118_ev_certificate_response_with_optional_fields() {
        let response = Get15118EVCertificateResponse::new(
            Iso15118EVCertificateStatusEnumType::Accepted,
            "YmFzZTY0X2VuY29kZWRfcmVzcG9uc2U=".to_string()
        )
        .with_status_info(StatusInfoType::new("Success".to_string())
            .with_additional_info("Certificate installed successfully".to_string()))
        .with_remaining_contracts(3)
        .with_custom_data(CustomDataType::new("Vendor".to_string()));
        
        assert!(response.status_info.is_some());
        assert_eq!(response.remaining_contracts, Some(3));
        assert!(response.custom_data.is_some());
    }

    #[test]
    fn test_get_15118_ev_certificate_response_setters() {
        let mut response = Get15118EVCertificateResponse::new(
            Iso15118EVCertificateStatusEnumType::Accepted,
            "YmFzZTY0X2VuY29kZWRfcmVzcG9uc2U=".to_string()
        );
        
        response.set_status(Iso15118EVCertificateStatusEnumType::Failed);
        response.set_exi_response("bmV3X2V4aV9yZXNwb25zZQ==".to_string());
        response.set_status_info(Some(StatusInfoType::new("Error".to_string())));
        response.set_remaining_contracts(Some(5));
        response.set_custom_data(Some(CustomDataType::new("TestVendor".to_string())));
        
        assert_eq!(response.status, Iso15118EVCertificateStatusEnumType::Failed);
        assert_eq!(response.exi_response, "bmV3X2V4aV9yZXNwb25zZQ==");
        assert!(response.status_info.is_some());
        assert_eq!(response.remaining_contracts, Some(5));
        assert!(response.custom_data.is_some());
    }

    #[test]
    fn test_get_15118_ev_certificate_response_getters() {
        let response = Get15118EVCertificateResponse::new(
            Iso15118EVCertificateStatusEnumType::Accepted,
            "YmFzZTY0X2VuY29kZWRfcmVzcG9uc2U=".to_string()
        )
        .with_remaining_contracts(2);
        
        assert_eq!(*response.get_status(), Iso15118EVCertificateStatusEnumType::Accepted);
        assert_eq!(response.get_exi_response(), "YmFzZTY0X2VuY29kZWRfcmVzcG9uc2U=");
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_remaining_contracts(), Some(&2));
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_get_15118_ev_certificate_response_serialization() {
        let response = Get15118EVCertificateResponse::new(
            Iso15118EVCertificateStatusEnumType::Accepted,
            "YmFzZTY0X2VuY29kZWRfcmVzcG9uc2U=".to_string()
        );
        
        let json = serde_json::to_string(&response).unwrap();
        let parsed: Get15118EVCertificateResponse = serde_json::from_str(&json).unwrap();
        
        assert_eq!(response, parsed);
    }

    #[test]
    fn test_get_15118_ev_certificate_response_deserialization() {
        let json = r#"{
            "status": "Accepted",
            "exiResponse": "YmFzZTY0X2VuY29kZWRfcmVzcG9uc2U=",
            "remainingContracts": 2
        }"#;
        
        let response: Get15118EVCertificateResponse = serde_json::from_str(json).unwrap();
        
        assert_eq!(response.status, Iso15118EVCertificateStatusEnumType::Accepted);
        assert_eq!(response.exi_response, "YmFzZTY0X2VuY29kZWRfcmVzcG9uc2U=");
        assert_eq!(response.remaining_contracts, Some(2));
    }

    #[test]
    fn test_get_15118_ev_certificate_response_validation_exi_response_too_long() {
        let response = Get15118EVCertificateResponse::new(
            Iso15118EVCertificateStatusEnumType::Accepted,
            "a".repeat(17001) // 17001 characters, exceeds max length of 17000
        );
        
        assert!(response.validate().is_err());
    }

    #[test]
    fn test_get_15118_ev_certificate_response_validation_negative_remaining_contracts() {
        let mut response = Get15118EVCertificateResponse::new(
            Iso15118EVCertificateStatusEnumType::Accepted,
            "YmFzZTY0X2VuY29kZWRfcmVzcG9uc2U=".to_string()
        );
        response.set_remaining_contracts(Some(-1));
        
        assert!(response.validate().is_err());
    }

    #[test]
    fn test_get_15118_ev_certificate_response_all_status_types() {
        // Test with different status types
        let statuses = vec![
            Iso15118EVCertificateStatusEnumType::Accepted,
            Iso15118EVCertificateStatusEnumType::Failed,
        ];
        
        for status in statuses {
            let response = Get15118EVCertificateResponse::new(
                status.clone(),
                "YmFzZTY0X2VuY29kZWRfcmVzcG9uc2U=".to_string()
            );
            assert_eq!(response.status, status);
        }
    }

    #[test]
    fn test_get_15118_ev_certificate_request_all_action_types() {
        // Test with different action types
        let actions = vec![
            CertificateActionEnumType::Install,
            CertificateActionEnumType::Update,
        ];
        
        for action in actions {
            let request = Get15118EVCertificateRequest::new(
                "urn:iso:15118:2:2013:MsgDef".to_string(),
                action.clone(),
                "YmFzZTY0X2VuY29kZWRfZXhhbXBsZQ==".to_string()
            );
            assert_eq!(request.action, action);
        }
    }

    #[test]
    fn test_get_15118_ev_certificate_request_json_round_trip_with_all_fields() {
        let request = Get15118EVCertificateRequest::new(
            "urn:iso:15118:2:2013:MsgDef".to_string(),
            CertificateActionEnumType::Install,
            "YmFzZTY0X2VuY29kZWRfZXhhbXBsZQ==".to_string()
        )
        .with_maximum_contract_certificate_chains(5)
        .with_prioritized_emai_ds(vec!["EMAID1".to_string(), "EMAID2".to_string()])
        .with_custom_data(CustomDataType::new("TestVendor".to_string()));
        
        let json = serde_json::to_string(&request).unwrap();
        let parsed: Get15118EVCertificateRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(request, parsed);
        assert_eq!(parsed.maximum_contract_certificate_chains, Some(5));
        assert_eq!(parsed.prioritized_emai_ds, Some(vec!["EMAID1".to_string(), "EMAID2".to_string()]));
        assert_eq!(parsed.custom_data.as_ref().unwrap().vendor_id, "TestVendor");
    }

    #[test]
    fn test_get_15118_ev_certificate_response_json_round_trip_with_all_fields() {
        let response = Get15118EVCertificateResponse::new(
            Iso15118EVCertificateStatusEnumType::Accepted,
            "YmFzZTY0X2VuY29kZWRfcmVzcG9uc2U=".to_string()
        )
        .with_status_info(StatusInfoType::new("Success".to_string())
            .with_additional_info("All good".to_string()))
        .with_remaining_contracts(3)
        .with_custom_data(CustomDataType::new("TestVendor".to_string()));
        
        let json = serde_json::to_string(&response).unwrap();
        let parsed: Get15118EVCertificateResponse = serde_json::from_str(&json).unwrap();
        
        assert_eq!(response, parsed);
        assert_eq!(parsed.remaining_contracts, Some(3));
        assert_eq!(parsed.status_info.as_ref().unwrap().reason_code, "Success");
        assert_eq!(parsed.custom_data.as_ref().unwrap().vendor_id, "TestVendor");
    }
}
