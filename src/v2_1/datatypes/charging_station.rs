use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use super::modem::ModemType;

/// The physical system where an Electrical Vehicle (EV) can be charged.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingStationType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Vendor-specific device identifier.
    #[validate(length(max = 25))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,

    /// Required. Defines the model of the device.
    #[validate(length(max = 20))]
    pub model: String,

    /// Required. Identifies the vendor (not necessarily in a unique manner).
    #[validate(length(max = 50))]
    pub vendor_name: String,

    /// This contains the firmware version of the Charging Station.
    #[validate(length(max = 50))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,

    /// Defines parameters required for initiating and maintaining wireless communication with other devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modem: Option<ModemType>,
}

impl ChargingStationType {
    /// Creates a new `ChargingStationType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `model` - Defines the model of the device
    /// * `vendor_name` - Identifies the vendor
    ///
    /// # Returns
    ///
    /// A new instance of `ChargingStationType` with optional fields set to `None`
    pub fn new(model: String, vendor_name: String) -> Self {
        Self {
            model,
            vendor_name,
            custom_data: None,
            serial_number: None,
            firmware_version: None,
            modem: None,
        }
    }

    /// Sets the serial number.
    ///
    /// # Arguments
    ///
    /// * `serial_number` - Vendor-specific device identifier
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_serial_number(mut self, serial_number: String) -> Self {
        self.serial_number = Some(serial_number);
        self
    }

    /// Sets the firmware version.
    ///
    /// # Arguments
    ///
    /// * `firmware_version` - Firmware version of the Charging Station
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_firmware_version(mut self, firmware_version: String) -> Self {
        self.firmware_version = Some(firmware_version);
        self
    }

    /// Sets the modem.
    ///
    /// # Arguments
    ///
    /// * `modem` - Parameters for wireless communication
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_modem(mut self, modem: ModemType) -> Self {
        self.modem = Some(modem);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this charging station
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the model.
    ///
    /// # Returns
    ///
    /// A reference to the model of the device
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Sets the model.
    ///
    /// # Arguments
    ///
    /// * `model` - Defines the model of the device
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_model(&mut self, model: String) -> &mut Self {
        self.model = model;
        self
    }

    /// Gets the vendor name.
    ///
    /// # Returns
    ///
    /// A reference to the vendor name
    pub fn vendor_name(&self) -> &str {
        &self.vendor_name
    }

    /// Sets the vendor name.
    ///
    /// # Arguments
    ///
    /// * `vendor_name` - Identifies the vendor
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_vendor_name(&mut self, vendor_name: String) -> &mut Self {
        self.vendor_name = vendor_name;
        self
    }

    /// Gets the serial number.
    ///
    /// # Returns
    ///
    /// An optional reference to the serial number
    pub fn serial_number(&self) -> Option<&str> {
        self.serial_number.as_deref()
    }

    /// Sets the serial number.
    ///
    /// # Arguments
    ///
    /// * `serial_number` - Vendor-specific device identifier, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_serial_number(&mut self, serial_number: Option<String>) -> &mut Self {
        self.serial_number = serial_number;
        self
    }

    /// Gets the firmware version.
    ///
    /// # Returns
    ///
    /// An optional reference to the firmware version
    pub fn firmware_version(&self) -> Option<&str> {
        self.firmware_version.as_deref()
    }

    /// Sets the firmware version.
    ///
    /// # Arguments
    ///
    /// * `firmware_version` - Firmware version of the Charging Station, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_firmware_version(&mut self, firmware_version: Option<String>) -> &mut Self {
        self.firmware_version = firmware_version;
        self
    }

    /// Gets the modem.
    ///
    /// # Returns
    ///
    /// An optional reference to the modem
    pub fn modem(&self) -> Option<&ModemType> {
        self.modem.as_ref()
    }

    /// Sets the modem.
    ///
    /// # Arguments
    ///
    /// * `modem` - Parameters for wireless communication, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_modem(&mut self, modem: Option<ModemType>) -> &mut Self {
        self.modem = modem;
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
    /// * `custom_data` - Custom data for this charging station, or None to clear
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
    fn test_new_charging_station() {
        let station = ChargingStationType::new("Model X".to_string(), "Vendor Y".to_string());

        assert_eq!(station.model(), "Model X");
        assert_eq!(station.vendor_name(), "Vendor Y");
        assert_eq!(station.serial_number(), None);
        assert_eq!(station.firmware_version(), None);
        assert_eq!(station.modem(), None);
        assert_eq!(station.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let modem = ModemType {
            iccid: "12345678901234567890".to_string(),
            imsi: "123456789012345".to_string(),
            custom_data: None,
        };

        let station = ChargingStationType::new("Model X".to_string(), "Vendor Y".to_string())
            .with_serial_number("SN12345".to_string())
            .with_firmware_version("1.0.0".to_string())
            .with_modem(modem.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(station.model(), "Model X");
        assert_eq!(station.vendor_name(), "Vendor Y");
        assert_eq!(station.serial_number(), Some("SN12345"));
        assert_eq!(station.firmware_version(), Some("1.0.0"));
        assert_eq!(station.modem(), Some(&modem));
        assert_eq!(station.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let modem = ModemType {
            iccid: "12345678901234567890".to_string(),
            imsi: "123456789012345".to_string(),
            custom_data: None,
        };

        let mut station = ChargingStationType::new("Model X".to_string(), "Vendor Y".to_string());

        station
            .set_model("Model Z".to_string())
            .set_vendor_name("Vendor Z".to_string())
            .set_serial_number(Some("SN67890".to_string()))
            .set_firmware_version(Some("2.0.0".to_string()))
            .set_modem(Some(modem.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(station.model(), "Model Z");
        assert_eq!(station.vendor_name(), "Vendor Z");
        assert_eq!(station.serial_number(), Some("SN67890"));
        assert_eq!(station.firmware_version(), Some("2.0.0"));
        assert_eq!(station.modem(), Some(&modem));
        assert_eq!(station.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        station
            .set_serial_number(None)
            .set_firmware_version(None)
            .set_modem(None)
            .set_custom_data(None);

        assert_eq!(station.serial_number(), None);
        assert_eq!(station.firmware_version(), None);
        assert_eq!(station.modem(), None);
        assert_eq!(station.custom_data(), None);
    }
}
