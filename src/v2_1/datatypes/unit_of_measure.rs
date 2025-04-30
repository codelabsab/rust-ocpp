use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Represents a UnitOfMeasure with a multiplier.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UnitOfMeasureType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Unit of the value. Default = "Wh" if the (default) measurand is an "Energy" type.
    /// This field SHALL use a value from the list Standardized Units of Measurements in Part 2 Appendices.
    /// If an applicable unit is available in that list, otherwise a "custom" unit might be used.
    #[validate(length(max = 20))]
    pub unit: String,

    /// Optional. Multiplier, this value represents the exponent to base 10. I.e. multiplier 3 means 10 raised to the third power.
    /// Default is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = -9, max = 9))]
    pub multiplier: Option<i8>,
}

impl UnitOfMeasureType {
    /// Creates a new `UnitOfMeasureType` with the required fields.
    ///
    /// # Arguments
    ///
    /// * `unit` - Unit of the value
    ///
    /// # Returns
    ///
    /// A new `UnitOfMeasureType` instance with optional fields set to `None`
    pub fn new(unit: String) -> Self {
        Self {
            custom_data: None,
            unit,
            multiplier: None,
        }
    }

    /// Sets the custom data field.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data from the Charging Station
    ///
    /// # Returns
    ///
    /// The modified `UnitOfMeasureType` instance
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the multiplier field.
    ///
    /// # Arguments
    ///
    /// * `multiplier` - Multiplier, this value represents the exponent to base 10
    ///
    /// # Returns
    ///
    /// The modified `UnitOfMeasureType` instance
    pub fn with_multiplier(mut self, multiplier: i8) -> Self {
        self.multiplier = Some(multiplier);
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
    /// * `custom_data` - Custom data from the Charging Station, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `UnitOfMeasureType` instance
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the unit.
    ///
    /// # Returns
    ///
    /// The unit of the value
    pub fn unit(&self) -> &str {
        &self.unit
    }

    /// Sets the unit.
    ///
    /// # Arguments
    ///
    /// * `unit` - Unit of the value
    ///
    /// # Returns
    ///
    /// The modified `UnitOfMeasureType` instance
    pub fn set_unit(&mut self, unit: String) -> &mut Self {
        self.unit = unit;
        self
    }

    /// Gets the multiplier.
    ///
    /// # Returns
    ///
    /// An optional multiplier value
    pub fn multiplier(&self) -> Option<i8> {
        self.multiplier
    }

    /// Sets the multiplier.
    ///
    /// # Arguments
    ///
    /// * `multiplier` - Multiplier, this value represents the exponent to base 10, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `UnitOfMeasureType` instance
    pub fn set_multiplier(&mut self, multiplier: Option<i8>) -> &mut Self {
        self.multiplier = multiplier;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_of_measure_new() {
        let unit = "kWh".to_string();
        let unit_of_measure = UnitOfMeasureType::new(unit.clone());

        assert_eq!(unit_of_measure.unit(), unit);
        assert_eq!(unit_of_measure.multiplier(), None);
        assert_eq!(unit_of_measure.custom_data(), None);
    }

    #[test]
    fn test_unit_of_measure_with_methods() {
        let unit = "kWh".to_string();
        let multiplier = 3;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let unit_of_measure = UnitOfMeasureType::new(unit.clone())
            .with_multiplier(multiplier)
            .with_custom_data(custom_data.clone());

        assert_eq!(unit_of_measure.unit(), unit);
        assert_eq!(unit_of_measure.multiplier(), Some(multiplier));
        assert_eq!(unit_of_measure.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_unit_of_measure_setters() {
        let unit1 = "kWh".to_string();
        let unit2 = "A".to_string();
        let multiplier = 3;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut unit_of_measure = UnitOfMeasureType::new(unit1.clone());

        unit_of_measure
            .set_unit(unit2.clone())
            .set_multiplier(Some(multiplier))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(unit_of_measure.unit(), unit2);
        assert_eq!(unit_of_measure.multiplier(), Some(multiplier));
        assert_eq!(unit_of_measure.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        unit_of_measure
            .set_multiplier(None)
            .set_custom_data(None);

        assert_eq!(unit_of_measure.multiplier(), None);
        assert_eq!(unit_of_measure.custom_data(), None);
    }
}
