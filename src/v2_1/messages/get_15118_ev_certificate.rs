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
