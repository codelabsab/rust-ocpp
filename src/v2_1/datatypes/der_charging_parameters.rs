use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::der_control::DERControlEnumType;

/// DERChargingParametersType is used in ChargingNeedsType during an ISO 15118-20 session for AC_BPT_DER
/// to report the inverter settings related to DER control that were agreed between EVSE and EV.
///
/// Fields starting with "ev" contain values from the EV.
/// Other fields contain a value that is supported by both EV and EVSE.
///
/// DERChargingParametersType type is only relevant in case of an ISO 15118-20 AC_BPT_DER/AC_DER charging session.
///
/// NOTE: All these fields have values greater or equal to zero (i.e. are non-negative)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DERChargingParametersType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// DER control functions supported by EV.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType:DERControlFunctions (bitmap)
    pub ev_supported_der_control: Vec<DERControlEnumType>,
}

impl DERChargingParametersType {
    /// Creates a new `DERChargingParametersType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `ev_supported_der_control` - DER control functions supported by EV
    ///
    /// # Returns
    ///
    /// A new instance of `DERChargingParametersType` with optional fields set to `None`
    pub fn new(ev_supported_der_control: Vec<DERControlEnumType>) -> Self {
        Self {
            ev_supported_der_control,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the DER control functions supported by EV.
    ///
    /// # Returns
    ///
    /// A reference to the vector of DER control functions
    pub fn ev_supported_der_control(&self) -> &Vec<DERControlEnumType> {
        &self.ev_supported_der_control
    }

    /// Sets the DER control functions supported by EV.
    ///
    /// # Arguments
    ///
    /// * `ev_supported_der_control` - DER control functions supported by EV
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_supported_der_control(&mut self, ev_supported_der_control: Vec<DERControlEnumType>) -> &mut Self {
        self.ev_supported_der_control = ev_supported_der_control;
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
    /// * `custom_data` - Custom data for these parameters, or None to clear
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
    fn test_new_der_charging_parameters() {
        let der_controls = vec![DERControlEnumType::FreqDroop, DERControlEnumType::PowerFactor];
        let params = DERChargingParametersType::new(der_controls.clone());

        assert_eq!(params.ev_supported_der_control(), &der_controls);
        assert_eq!(params.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let der_controls = vec![DERControlEnumType::FreqDroop];
        let custom_data = CustomDataType::new("VendorX".to_string());

        let params = DERChargingParametersType::new(der_controls.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(params.ev_supported_der_control(), &der_controls);
        assert_eq!(params.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let der_controls1 = vec![DERControlEnumType::FreqDroop];
        let der_controls2 = vec![DERControlEnumType::PowerFactor, DERControlEnumType::FixedVar];
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut params = DERChargingParametersType::new(der_controls1.clone());

        params
            .set_ev_supported_der_control(der_controls2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(params.ev_supported_der_control(), &der_controls2);
        assert_eq!(params.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        params.set_custom_data(None);
        assert_eq!(params.custom_data(), None);
    }
}
