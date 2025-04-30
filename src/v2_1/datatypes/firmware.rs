use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Contains information about a specific firmware version.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// URL from which the firmware can be downloaded.
    #[validate(length(max = 512))]
    pub location: String,

    /// Date and time at which the firmware shall be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_date: Option<String>,

    /// Date and time at which the firmware shall be installed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_date: Option<String>,

    /// Firmware version.
    #[validate(length(max = 50))]
    pub signature: String,

    /// MD5 checksum over the entire firmware file as a hexadecimal string of length 32.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(equal = 32))]
    pub signing_certificate: Option<String>,
}

impl FirmwareType {
    /// Creates a new `FirmwareType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `location` - URL from which the firmware can be downloaded
    /// * `signature` - Firmware version
    ///
    /// # Returns
    ///
    /// A new instance of `FirmwareType` with optional fields set to `None`
    pub fn new(location: String, signature: String) -> Self {
        Self {
            location,
            signature,
            retrieve_date: None,
            install_date: None,
            signing_certificate: None,
            custom_data: None,
        }
    }

    /// Sets the retrieve date.
    ///
    /// # Arguments
    ///
    /// * `retrieve_date` - Date and time at which the firmware shall be retrieved
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_retrieve_date(mut self, retrieve_date: String) -> Self {
        self.retrieve_date = Some(retrieve_date);
        self
    }

    /// Sets the install date.
    ///
    /// # Arguments
    ///
    /// * `install_date` - Date and time at which the firmware shall be installed
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_install_date(mut self, install_date: String) -> Self {
        self.install_date = Some(install_date);
        self
    }

    /// Sets the signing certificate.
    ///
    /// # Arguments
    ///
    /// * `signing_certificate` - MD5 checksum over the entire firmware file
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_signing_certificate(mut self, signing_certificate: String) -> Self {
        self.signing_certificate = Some(signing_certificate);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this firmware
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the location.
    ///
    /// # Returns
    ///
    /// The URL from which the firmware can be downloaded
    pub fn location(&self) -> &str {
        &self.location
    }

    /// Sets the location.
    ///
    /// # Arguments
    ///
    /// * `location` - URL from which the firmware can be downloaded
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_location(&mut self, location: String) -> &mut Self {
        self.location = location;
        self
    }

    /// Gets the retrieve date.
    ///
    /// # Returns
    ///
    /// An optional reference to the date and time at which the firmware shall be retrieved
    pub fn retrieve_date(&self) -> Option<&str> {
        self.retrieve_date.as_deref()
    }

    /// Sets the retrieve date.
    ///
    /// # Arguments
    ///
    /// * `retrieve_date` - Date and time at which the firmware shall be retrieved, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_retrieve_date(&mut self, retrieve_date: Option<String>) -> &mut Self {
        self.retrieve_date = retrieve_date;
        self
    }

    /// Gets the install date.
    ///
    /// # Returns
    ///
    /// An optional reference to the date and time at which the firmware shall be installed
    pub fn install_date(&self) -> Option<&str> {
        self.install_date.as_deref()
    }

    /// Sets the install date.
    ///
    /// # Arguments
    ///
    /// * `install_date` - Date and time at which the firmware shall be installed, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_install_date(&mut self, install_date: Option<String>) -> &mut Self {
        self.install_date = install_date;
        self
    }

    /// Gets the signature.
    ///
    /// # Returns
    ///
    /// The firmware version
    pub fn signature(&self) -> &str {
        &self.signature
    }

    /// Sets the signature.
    ///
    /// # Arguments
    ///
    /// * `signature` - Firmware version
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_signature(&mut self, signature: String) -> &mut Self {
        self.signature = signature;
        self
    }

    /// Gets the signing certificate.
    ///
    /// # Returns
    ///
    /// An optional reference to the MD5 checksum over the entire firmware file
    pub fn signing_certificate(&self) -> Option<&str> {
        self.signing_certificate.as_deref()
    }

    /// Sets the signing certificate.
    ///
    /// # Arguments
    ///
    /// * `signing_certificate` - MD5 checksum over the entire firmware file, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_signing_certificate(&mut self, signing_certificate: Option<String>) -> &mut Self {
        self.signing_certificate = signing_certificate;
        self
    }

    /// Gets the custom data.
    ///
    /// # Returns
    ///
    /// An optional reference to the custom data
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this firmware, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_firmware() {
        let location = "https://example.com/firmware/v1.2.3".to_string();
        let signature = "1.2.3".to_string();

        let firmware = FirmwareType::new(location.clone(), signature.clone());

        assert_eq!(firmware.location(), location);
        assert_eq!(firmware.signature(), signature);
        assert_eq!(firmware.retrieve_date(), None);
        assert_eq!(firmware.install_date(), None);
        assert_eq!(firmware.signing_certificate(), None);
        assert_eq!(firmware.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let location = "https://example.com/firmware/v1.2.3".to_string();
        let signature = "1.2.3".to_string();
        let retrieve_date = "2023-01-01T12:00:00Z".to_string();
        let install_date = "2023-01-02T12:00:00Z".to_string();
        let signing_certificate = "0123456789abcdef0123456789abcdef".to_string();
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let firmware = FirmwareType::new(location.clone(), signature.clone())
            .with_retrieve_date(retrieve_date.clone())
            .with_install_date(install_date.clone())
            .with_signing_certificate(signing_certificate.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(firmware.location(), location);
        assert_eq!(firmware.signature(), signature);
        assert_eq!(firmware.retrieve_date(), Some(retrieve_date.as_str()));
        assert_eq!(firmware.install_date(), Some(install_date.as_str()));
        assert_eq!(firmware.signing_certificate(), Some(signing_certificate.as_str()));
        assert_eq!(firmware.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let location1 = "https://example.com/firmware/v1.2.3".to_string();
        let signature1 = "1.2.3".to_string();
        let location2 = "https://example.com/firmware/v1.2.4".to_string();
        let signature2 = "1.2.4".to_string();
        let retrieve_date = "2023-01-01T12:00:00Z".to_string();
        let install_date = "2023-01-02T12:00:00Z".to_string();
        let signing_certificate = "0123456789abcdef0123456789abcdef".to_string();
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut firmware = FirmwareType::new(location1.clone(), signature1.clone());

        firmware
            .set_location(location2.clone())
            .set_signature(signature2.clone())
            .set_retrieve_date(Some(retrieve_date.clone()))
            .set_install_date(Some(install_date.clone()))
            .set_signing_certificate(Some(signing_certificate.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(firmware.location(), location2);
        assert_eq!(firmware.signature(), signature2);
        assert_eq!(firmware.retrieve_date(), Some(retrieve_date.as_str()));
        assert_eq!(firmware.install_date(), Some(install_date.as_str()));
        assert_eq!(firmware.signing_certificate(), Some(signing_certificate.as_str()));
        assert_eq!(firmware.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        firmware
            .set_retrieve_date(None)
            .set_install_date(None)
            .set_signing_certificate(None)
            .set_custom_data(None);

        assert_eq!(firmware.retrieve_date(), None);
        assert_eq!(firmware.install_date(), None);
        assert_eq!(firmware.signing_certificate(), None);
        assert_eq!(firmware.custom_data(), None);
    }
}
