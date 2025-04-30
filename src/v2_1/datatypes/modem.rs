use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Defines parameters required for initiating and maintaining wireless communication with other devices.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ModemType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. This contains the ICCID of the modem's SIM card.
    #[validate(length(max = 20))]
    pub iccid: String,

    /// Required. This contains the IMSI of the modem's SIM card.
    #[validate(length(max = 20))]
    pub imsi: String,
}

impl ModemType {
    /// Creates a new `ModemType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `iccid` - ICCID of the modem's SIM card
    /// * `imsi` - IMSI of the modem's SIM card
    ///
    /// # Returns
    ///
    /// A new instance of `ModemType` with optional fields set to `None`
    pub fn new(iccid: String, imsi: String) -> Self {
        Self {
            iccid,
            imsi,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this modem
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the ICCID.
    ///
    /// # Returns
    ///
    /// A reference to the ICCID of the modem's SIM card
    pub fn iccid(&self) -> &str {
        &self.iccid
    }

    /// Sets the ICCID.
    ///
    /// # Arguments
    ///
    /// * `iccid` - ICCID of the modem's SIM card
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_iccid(&mut self, iccid: String) -> &mut Self {
        self.iccid = iccid;
        self
    }

    /// Gets the IMSI.
    ///
    /// # Returns
    ///
    /// A reference to the IMSI of the modem's SIM card
    pub fn imsi(&self) -> &str {
        &self.imsi
    }

    /// Sets the IMSI.
    ///
    /// # Arguments
    ///
    /// * `imsi` - IMSI of the modem's SIM card
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_imsi(&mut self, imsi: String) -> &mut Self {
        self.imsi = imsi;
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
    /// * `custom_data` - Custom data for this modem, or None to clear
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
    fn test_new_modem() {
        let modem = ModemType::new(
            "12345678901234567890".to_string(),
            "123456789012345".to_string(),
        );

        assert_eq!(modem.iccid(), "12345678901234567890");
        assert_eq!(modem.imsi(), "123456789012345");
        assert_eq!(modem.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let modem = ModemType::new(
            "12345678901234567890".to_string(),
            "123456789012345".to_string(),
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(modem.iccid(), "12345678901234567890");
        assert_eq!(modem.imsi(), "123456789012345");
        assert_eq!(modem.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut modem = ModemType::new(
            "12345678901234567890".to_string(),
            "123456789012345".to_string(),
        );

        modem
            .set_iccid("09876543210987654321".to_string())
            .set_imsi("543210987654321".to_string())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(modem.iccid(), "09876543210987654321");
        assert_eq!(modem.imsi(), "543210987654321");
        assert_eq!(modem.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        modem.set_custom_data(None);
        assert_eq!(modem.custom_data(), None);
    }
}
