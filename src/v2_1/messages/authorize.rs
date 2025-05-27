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
