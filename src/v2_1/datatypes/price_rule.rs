use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::EnergyTransferModeEnumType;

/// Rule that describes the price of charging.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PriceRuleType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Mode of energy transfer for which this price rule applies.
    pub energy_transfer_mode: EnergyTransferModeEnumType,

    /// Required. Price per energy unit, excluding taxes, in the specified currency.
    pub energy_fee: f64,

    /// Required. Price per time unit, excluding taxes, in the specified currency.
    pub time_fee: f64,

    /// Required. Price for parking time, excluding taxes, in the specified currency.
    /// Only applicable when the EV is not charging.
    pub parking_fee: f64,

    /// Required. Minimum duration in seconds that a charging session SHALL last to benefit from this price rule.
    #[validate(range(min = 0))]
    pub minimum_duration: i32,

    /// Required. Maximum duration in seconds that a charging session can last under this price rule.
    #[validate(range(min = 0))]
    pub maximum_duration: i32,

    /// Required. Maximum power in kW that can be delivered under this price rule.
    pub maximum_power: f64,

    /// Required. Minimum power in kW that must be delivered under this price rule.
    pub minimum_power: f64,
}

impl PriceRuleType {
    /// Creates a new `PriceRuleType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `energy_transfer_mode` - Mode of energy transfer for which this price rule applies
    /// * `energy_fee` - Price per energy unit, excluding taxes
    /// * `time_fee` - Price per time unit, excluding taxes
    /// * `parking_fee` - Price for parking time, excluding taxes
    /// * `minimum_duration` - Minimum duration in seconds that a charging session must last
    /// * `maximum_duration` - Maximum duration in seconds that a charging session can last
    /// * `maximum_power` - Maximum power in kW that can be delivered
    /// * `minimum_power` - Minimum power in kW that must be delivered
    ///
    /// # Returns
    ///
    /// A new instance of `PriceRuleType` with optional fields set to `None`
    pub fn new(
        energy_transfer_mode: EnergyTransferModeEnumType,
        energy_fee: f64,
        time_fee: f64,
        parking_fee: f64,
        minimum_duration: i32,
        maximum_duration: i32,
        maximum_power: f64,
        minimum_power: f64,
    ) -> Self {
        Self {
            custom_data: None,
            energy_transfer_mode,
            energy_fee,
            time_fee,
            parking_fee,
            minimum_duration,
            maximum_duration,
            maximum_power,
            minimum_power,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this price rule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
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
    /// * `custom_data` - Custom data for this price rule, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the energy transfer mode.
    ///
    /// # Returns
    ///
    /// The mode of energy transfer for which this price rule applies
    pub fn energy_transfer_mode(&self) -> EnergyTransferModeEnumType {
        self.energy_transfer_mode.clone()
    }

    /// Sets the energy transfer mode.
    ///
    /// # Arguments
    ///
    /// * `energy_transfer_mode` - Mode of energy transfer for which this price rule applies
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_energy_transfer_mode(
        &mut self,
        energy_transfer_mode: EnergyTransferModeEnumType,
    ) -> &mut Self {
        self.energy_transfer_mode = energy_transfer_mode;
        self
    }

    /// Gets the energy fee.
    ///
    /// # Returns
    ///
    /// The price per energy unit, excluding taxes
    pub fn energy_fee(&self) -> f64 {
        self.energy_fee
    }

    /// Sets the energy fee.
    ///
    /// # Arguments
    ///
    /// * `energy_fee` - Price per energy unit, excluding taxes
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_energy_fee(&mut self, energy_fee: f64) -> &mut Self {
        self.energy_fee = energy_fee;
        self
    }

    /// Gets the time fee.
    ///
    /// # Returns
    ///
    /// The price per time unit, excluding taxes
    pub fn time_fee(&self) -> f64 {
        self.time_fee
    }

    /// Sets the time fee.
    ///
    /// # Arguments
    ///
    /// * `time_fee` - Price per time unit, excluding taxes
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_time_fee(&mut self, time_fee: f64) -> &mut Self {
        self.time_fee = time_fee;
        self
    }

    /// Gets the parking fee.
    ///
    /// # Returns
    ///
    /// The price for parking time, excluding taxes
    pub fn parking_fee(&self) -> f64 {
        self.parking_fee
    }

    /// Sets the parking fee.
    ///
    /// # Arguments
    ///
    /// * `parking_fee` - Price for parking time, excluding taxes
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_parking_fee(&mut self, parking_fee: f64) -> &mut Self {
        self.parking_fee = parking_fee;
        self
    }

    /// Gets the minimum duration.
    ///
    /// # Returns
    ///
    /// The minimum duration in seconds that a charging session must last
    pub fn minimum_duration(&self) -> i32 {
        self.minimum_duration
    }

    /// Sets the minimum duration.
    ///
    /// # Arguments
    ///
    /// * `minimum_duration` - Minimum duration in seconds that a charging session must last
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_minimum_duration(&mut self, minimum_duration: i32) -> &mut Self {
        self.minimum_duration = minimum_duration;
        self
    }

    /// Gets the maximum duration.
    ///
    /// # Returns
    ///
    /// The maximum duration in seconds that a charging session can last
    pub fn maximum_duration(&self) -> i32 {
        self.maximum_duration
    }

    /// Sets the maximum duration.
    ///
    /// # Arguments
    ///
    /// * `maximum_duration` - Maximum duration in seconds that a charging session can last
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_maximum_duration(&mut self, maximum_duration: i32) -> &mut Self {
        self.maximum_duration = maximum_duration;
        self
    }

    /// Gets the maximum power.
    ///
    /// # Returns
    ///
    /// The maximum power in kW that can be delivered
    pub fn maximum_power(&self) -> f64 {
        self.maximum_power
    }

    /// Sets the maximum power.
    ///
    /// # Arguments
    ///
    /// * `maximum_power` - Maximum power in kW that can be delivered
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_maximum_power(&mut self, maximum_power: f64) -> &mut Self {
        self.maximum_power = maximum_power;
        self
    }

    /// Gets the minimum power.
    ///
    /// # Returns
    ///
    /// The minimum power in kW that must be delivered
    pub fn minimum_power(&self) -> f64 {
        self.minimum_power
    }

    /// Sets the minimum power.
    ///
    /// # Arguments
    ///
    /// * `minimum_power` - Minimum power in kW that must be delivered
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_minimum_power(&mut self, minimum_power: f64) -> &mut Self {
        self.minimum_power = minimum_power;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_price_rule() {
        let energy_transfer_mode = EnergyTransferModeEnumType::DC;
        let energy_fee = 0.25;
        let time_fee = 0.10;
        let parking_fee = 0.05;
        let minimum_duration = 300;
        let maximum_duration = 7200;
        let maximum_power = 50.0;
        let minimum_power = 10.0;

        let price_rule = PriceRuleType::new(
            energy_transfer_mode.clone(),
            energy_fee,
            time_fee,
            parking_fee,
            minimum_duration,
            maximum_duration,
            maximum_power,
            minimum_power,
        );

        assert_eq!(price_rule.energy_transfer_mode(), energy_transfer_mode);
        assert_eq!(price_rule.energy_fee(), energy_fee);
        assert_eq!(price_rule.time_fee(), time_fee);
        assert_eq!(price_rule.parking_fee(), parking_fee);
        assert_eq!(price_rule.minimum_duration(), minimum_duration);
        assert_eq!(price_rule.maximum_duration(), maximum_duration);
        assert_eq!(price_rule.maximum_power(), maximum_power);
        assert_eq!(price_rule.minimum_power(), minimum_power);
        assert_eq!(price_rule.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let energy_transfer_mode = EnergyTransferModeEnumType::DC;
        let energy_fee = 0.25;
        let time_fee = 0.10;
        let parking_fee = 0.05;
        let minimum_duration = 300;
        let maximum_duration = 7200;
        let maximum_power = 50.0;
        let minimum_power = 10.0;

        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let price_rule = PriceRuleType::new(
            energy_transfer_mode.clone(),
            energy_fee,
            time_fee,
            parking_fee,
            minimum_duration,
            maximum_duration,
            maximum_power,
            minimum_power,
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(price_rule.energy_transfer_mode(), energy_transfer_mode);
        assert_eq!(price_rule.energy_fee(), energy_fee);
        assert_eq!(price_rule.time_fee(), time_fee);
        assert_eq!(price_rule.parking_fee(), parking_fee);
        assert_eq!(price_rule.minimum_duration(), minimum_duration);
        assert_eq!(price_rule.maximum_duration(), maximum_duration);
        assert_eq!(price_rule.maximum_power(), maximum_power);
        assert_eq!(price_rule.minimum_power(), minimum_power);
        assert_eq!(price_rule.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let energy_transfer_mode1 = EnergyTransferModeEnumType::DC;
        let energy_transfer_mode2 = EnergyTransferModeEnumType::ACSinglePhase;
        let energy_fee1 = 0.25;
        let energy_fee2 = 0.30;
        let time_fee1 = 0.10;
        let time_fee2 = 0.15;
        let parking_fee1 = 0.05;
        let parking_fee2 = 0.08;
        let minimum_duration1 = 300;
        let minimum_duration2 = 600;
        let maximum_duration1 = 7200;
        let maximum_duration2 = 10800;
        let maximum_power1 = 50.0;
        let maximum_power2 = 75.0;
        let minimum_power1 = 10.0;
        let minimum_power2 = 15.0;

        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut price_rule = PriceRuleType::new(
            energy_transfer_mode1,
            energy_fee1,
            time_fee1,
            parking_fee1,
            minimum_duration1,
            maximum_duration1,
            maximum_power1,
            minimum_power1,
        );

        price_rule
            .set_energy_transfer_mode(energy_transfer_mode2.clone())
            .set_energy_fee(energy_fee2)
            .set_time_fee(time_fee2)
            .set_parking_fee(parking_fee2)
            .set_minimum_duration(minimum_duration2)
            .set_maximum_duration(maximum_duration2)
            .set_maximum_power(maximum_power2)
            .set_minimum_power(minimum_power2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(price_rule.energy_transfer_mode(), energy_transfer_mode2);
        assert_eq!(price_rule.energy_fee(), energy_fee2);
        assert_eq!(price_rule.time_fee(), time_fee2);
        assert_eq!(price_rule.parking_fee(), parking_fee2);
        assert_eq!(price_rule.minimum_duration(), minimum_duration2);
        assert_eq!(price_rule.maximum_duration(), maximum_duration2);
        assert_eq!(price_rule.maximum_power(), maximum_power2);
        assert_eq!(price_rule.minimum_power(), minimum_power2);
        assert_eq!(price_rule.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        price_rule.set_custom_data(None);
        assert_eq!(price_rule.custom_data(), None);
    }
}
