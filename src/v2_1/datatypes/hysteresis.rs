use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Hysteresis parameters for DER control functions.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct HysteresisType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Hysteresis offset.
    pub offset: f64,
}

impl HysteresisType {
    /// Creates a new `HysteresisType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `offset` - Hysteresis offset
    ///
    /// # Returns
    ///
    /// A new instance of `HysteresisType` with optional fields set to `None`
    pub fn new(offset: f64) -> Self {
        Self {
            offset,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these hysteresis parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the hysteresis offset.
    ///
    /// # Returns
    ///
    /// The hysteresis offset
    pub fn offset(&self) -> f64 {
        self.offset
    }

    /// Sets the hysteresis offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - Hysteresis offset
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_offset(&mut self, offset: f64) -> &mut Self {
        self.offset = offset;
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
    /// * `custom_data` - Custom data for these hysteresis parameters, or None to clear
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
    fn test_new_hysteresis() {
        let offset = 0.5;
        let hysteresis = HysteresisType::new(offset);

        assert_eq!(hysteresis.offset(), offset);
        assert_eq!(hysteresis.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let offset = 0.5;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let hysteresis = HysteresisType::new(offset)
            .with_custom_data(custom_data.clone());

        assert_eq!(hysteresis.offset(), offset);
        assert_eq!(hysteresis.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let offset1 = 0.5;
        let offset2 = 1.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut hysteresis = HysteresisType::new(offset1);

        hysteresis
            .set_offset(offset2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(hysteresis.offset(), offset2);
        assert_eq!(hysteresis.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        hysteresis.set_custom_data(None);
        assert_eq!(hysteresis.custom_data(), None);
    }
}
