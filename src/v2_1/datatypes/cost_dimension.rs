use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{datatypes::CustomDataType, enumerations::CostDimensionEnumType};

/// Volume consumed of cost dimension.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CostDimensionType {
    /// Type of cost dimension: energy, power, time, etc.
    pub r#type: CostDimensionEnumType,

    /// Volume of the dimension consumed, measured according to the dimension type.
    pub volume: f64,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl CostDimensionType {
    /// Creates a new `CostDimensionType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `type` - Type of cost dimension: energy, power, time, etc
    /// * `volume` - Volume of the dimension consumed, measured according to the dimension type
    ///
    /// # Returns
    ///
    /// A new instance of `CostDimensionType` with optional fields set to `None`
    pub fn new(r#type: CostDimensionEnumType, volume: f64) -> Self {
        Self {
            r#type,
            volume,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this cost dimension
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the type of cost dimension.
    ///
    /// # Returns
    ///
    /// The type of cost dimension
    pub fn r#type(&self) -> &CostDimensionEnumType {
        &self.r#type
    }

    /// Sets the type of cost dimension.
    ///
    /// # Arguments
    ///
    /// * `type` - Type of cost dimension: energy, power, time, etc
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_type(&mut self, r#type: CostDimensionEnumType) -> &mut Self {
        self.r#type = r#type;
        self
    }

    /// Gets the volume of the dimension consumed.
    ///
    /// # Returns
    ///
    /// The volume of the dimension consumed
    pub fn volume(&self) -> f64 {
        self.volume
    }

    /// Sets the volume of the dimension consumed.
    ///
    /// # Arguments
    ///
    /// * `volume` - Volume of the dimension consumed, measured according to the dimension type
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_volume(&mut self, volume: f64) -> &mut Self {
        self.volume = volume;
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
    /// * `custom_data` - Custom data for this cost dimension, or None to clear
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
    fn test_new_cost_dimension() {
        let dimension = CostDimensionType::new(CostDimensionEnumType::Energy, 10.5);

        assert_eq!(dimension.r#type(), &CostDimensionEnumType::Energy);
        assert_eq!(dimension.volume(), 10.5);
        assert_eq!(dimension.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let dimension = CostDimensionType::new(CostDimensionEnumType::ChargingTime, 30.0)
            .with_custom_data(custom_data.clone());

        assert_eq!(dimension.r#type(), &CostDimensionEnumType::ChargingTime);
        assert_eq!(dimension.volume(), 30.0);
        assert_eq!(dimension.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let mut dimension = CostDimensionType::new(CostDimensionEnumType::Energy, 10.5);

        dimension
            .set_type(CostDimensionEnumType::MaxPower)
            .set_volume(50.0)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(dimension.r#type(), &CostDimensionEnumType::MaxPower);
        assert_eq!(dimension.volume(), 50.0);
        assert_eq!(dimension.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        dimension.set_custom_data(None);
        assert_eq!(dimension.custom_data(), None);
    }
}
