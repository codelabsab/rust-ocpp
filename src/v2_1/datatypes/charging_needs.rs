use crate::v2_1::{
    datatypes::{
        ac_charging_parameters::ACChargingParametersType, custom_data::CustomDataType,
        dc_charging_parameters::DCChargingParametersType,
    },
    enumerations::EnergyTransferModeEnumType,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the charging needs of an EV.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingNeedsType {
    /// Mode of energy transfer requested by the EV.
    pub requested_energy_transfer: EnergyTransferModeEnumType,

    /// Estimated departure time of the EV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_time: Option<DateTime<Utc>>,

    /// EV AC charging parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac_charging_parameters: Option<ACChargingParametersType>,

    /// EV DC charging parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_charging_parameters: Option<DCChargingParametersType>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl ChargingNeedsType {
    /// Creates a new `ChargingNeedsType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `requested_energy_transfer` - Mode of energy transfer requested by the EV
    ///
    /// # Returns
    ///
    /// A new instance of `ChargingNeedsType` with optional fields set to `None`
    pub fn new(requested_energy_transfer: EnergyTransferModeEnumType) -> Self {
        Self {
            requested_energy_transfer,
            departure_time: None,
            ac_charging_parameters: None,
            dc_charging_parameters: None,
            custom_data: None,
        }
    }

    /// Sets the departure time.
    ///
    /// # Arguments
    ///
    /// * `departure_time` - Estimated departure time of the EV
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_departure_time(mut self, departure_time: DateTime<Utc>) -> Self {
        self.departure_time = Some(departure_time);
        self
    }

    /// Sets the AC charging parameters.
    ///
    /// # Arguments
    ///
    /// * `ac_charging_parameters` - EV AC charging parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_ac_charging_parameters(
        mut self,
        ac_charging_parameters: ACChargingParametersType,
    ) -> Self {
        self.ac_charging_parameters = Some(ac_charging_parameters);
        self
    }

    /// Sets the DC charging parameters.
    ///
    /// # Arguments
    ///
    /// * `dc_charging_parameters` - EV DC charging parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_dc_charging_parameters(
        mut self,
        dc_charging_parameters: DCChargingParametersType,
    ) -> Self {
        self.dc_charging_parameters = Some(dc_charging_parameters);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this charging needs
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the requested energy transfer mode.
    ///
    /// # Returns
    ///
    /// The mode of energy transfer requested by the EV
    pub fn requested_energy_transfer(&self) -> &EnergyTransferModeEnumType {
        &self.requested_energy_transfer
    }

    /// Sets the requested energy transfer mode.
    ///
    /// # Arguments
    ///
    /// * `requested_energy_transfer` - Mode of energy transfer requested by the EV
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_requested_energy_transfer(
        &mut self,
        requested_energy_transfer: EnergyTransferModeEnumType,
    ) -> &mut Self {
        self.requested_energy_transfer = requested_energy_transfer;
        self
    }

    /// Gets the departure time.
    ///
    /// # Returns
    ///
    /// An optional reference to the estimated departure time of the EV
    pub fn departure_time(&self) -> Option<&DateTime<Utc>> {
        self.departure_time.as_ref()
    }

    /// Sets the departure time.
    ///
    /// # Arguments
    ///
    /// * `departure_time` - Estimated departure time of the EV, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_departure_time(&mut self, departure_time: Option<DateTime<Utc>>) -> &mut Self {
        self.departure_time = departure_time;
        self
    }

    /// Gets the AC charging parameters.
    ///
    /// # Returns
    ///
    /// An optional reference to the EV AC charging parameters
    pub fn ac_charging_parameters(&self) -> Option<&ACChargingParametersType> {
        self.ac_charging_parameters.as_ref()
    }

    /// Sets the AC charging parameters.
    ///
    /// # Arguments
    ///
    /// * `ac_charging_parameters` - EV AC charging parameters, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ac_charging_parameters(
        &mut self,
        ac_charging_parameters: Option<ACChargingParametersType>,
    ) -> &mut Self {
        self.ac_charging_parameters = ac_charging_parameters;
        self
    }

    /// Gets the DC charging parameters.
    ///
    /// # Returns
    ///
    /// An optional reference to the EV DC charging parameters
    pub fn dc_charging_parameters(&self) -> Option<&DCChargingParametersType> {
        self.dc_charging_parameters.as_ref()
    }

    /// Sets the DC charging parameters.
    ///
    /// # Arguments
    ///
    /// * `dc_charging_parameters` - EV DC charging parameters, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_dc_charging_parameters(
        &mut self,
        dc_charging_parameters: Option<DCChargingParametersType>,
    ) -> &mut Self {
        self.dc_charging_parameters = dc_charging_parameters;
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
    /// * `custom_data` - Custom data for this charging needs, or None to clear
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
    use chrono::TimeZone;

    #[test]
    fn test_new_charging_needs() {
        let needs = ChargingNeedsType::new(EnergyTransferModeEnumType::DC);

        assert_eq!(
            needs.requested_energy_transfer(),
            &EnergyTransferModeEnumType::DC
        );
        assert_eq!(needs.departure_time(), None);
        assert_eq!(needs.ac_charging_parameters(), None);
        assert_eq!(needs.dc_charging_parameters(), None);
        assert_eq!(needs.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let departure_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let ac_params = ACChargingParametersType::new_from_f64(10000.0, 10.0, 32.0, 400.0);
        let dc_params = DCChargingParametersType::new(400.0, 100.0);

        let needs = ChargingNeedsType::new(EnergyTransferModeEnumType::ACThreePhase)
            .with_departure_time(departure_time)
            .with_ac_charging_parameters(ac_params.clone())
            .with_dc_charging_parameters(dc_params.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(
            needs.requested_energy_transfer(),
            &EnergyTransferModeEnumType::ACThreePhase
        );
        assert_eq!(needs.departure_time(), Some(&departure_time));
        assert_eq!(needs.ac_charging_parameters(), Some(&ac_params));
        assert_eq!(needs.dc_charging_parameters(), Some(&dc_params));
        assert_eq!(needs.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let departure_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let ac_params = ACChargingParametersType::new_from_f64(10000.0, 10.0, 32.0, 400.0);
        let dc_params = DCChargingParametersType::new(400.0, 100.0);

        let mut needs = ChargingNeedsType::new(EnergyTransferModeEnumType::ACSinglePhase);

        needs
            .set_requested_energy_transfer(EnergyTransferModeEnumType::DCBPT)
            .set_departure_time(Some(departure_time))
            .set_ac_charging_parameters(Some(ac_params.clone()))
            .set_dc_charging_parameters(Some(dc_params.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(
            needs.requested_energy_transfer(),
            &EnergyTransferModeEnumType::DCBPT
        );
        assert_eq!(needs.departure_time(), Some(&departure_time));
        assert_eq!(needs.ac_charging_parameters(), Some(&ac_params));
        assert_eq!(needs.dc_charging_parameters(), Some(&dc_params));
        assert_eq!(needs.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        needs
            .set_departure_time(None)
            .set_ac_charging_parameters(None)
            .set_dc_charging_parameters(None)
            .set_custom_data(None);

        assert_eq!(needs.departure_time(), None);
        assert_eq!(needs.ac_charging_parameters(), None);
        assert_eq!(needs.dc_charging_parameters(), None);
        assert_eq!(needs.custom_data(), None);
    }
}
