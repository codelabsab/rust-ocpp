use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Points defining a DER curve.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DERCurvePointsType {
    /// The data value of the X-axis (independent) variable, depending on the curve type.
    pub x: f64,

    /// The data value of the Y-axis (dependent) variable, depending on the DERUnitEnumType of the curve.
    /// If y is power factor, then a positive value means DER is absorbing reactive power (under-excited),
    /// a negative value when DER is injecting reactive power (over-excited).
    pub y: f64,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl DERCurvePointsType {
    /// Creates a new `DERCurvePointsType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `x` - The data value of the X-axis (independent) variable
    /// * `y` - The data value of the Y-axis (dependent) variable
    ///
    /// # Returns
    ///
    /// A new instance of `DERCurvePointsType` with optional fields set to `None`
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this curve point
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the X-axis value.
    ///
    /// # Returns
    ///
    /// The X-axis value
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Sets the X-axis value.
    ///
    /// # Arguments
    ///
    /// * `x` - The data value of the X-axis (independent) variable
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_x(&mut self, x: f64) -> &mut Self {
        self.x = x;
        self
    }

    /// Gets the Y-axis value.
    ///
    /// # Returns
    ///
    /// The Y-axis value
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Sets the Y-axis value.
    ///
    /// # Arguments
    ///
    /// * `y` - The data value of the Y-axis (dependent) variable
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_y(&mut self, y: f64) -> &mut Self {
        self.y = y;
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
    /// * `custom_data` - Custom data for this curve point, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}

impl Default for DERCurvePointsType {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            custom_data: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_der_curve_points() {
        let point = DERCurvePointsType::new(10.5, 20.3);

        assert_eq!(point.x(), 10.5);
        assert_eq!(point.y(), 20.3);
        assert_eq!(point.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let point = DERCurvePointsType::new(10.5, 20.3).with_custom_data(custom_data.clone());

        assert_eq!(point.x(), 10.5);
        assert_eq!(point.y(), 20.3);
        assert_eq!(point.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let mut point = DERCurvePointsType::new(10.5, 20.3);

        point
            .set_x(15.7)
            .set_y(25.9)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(point.x(), 15.7);
        assert_eq!(point.y(), 25.9);
        assert_eq!(point.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        point.set_custom_data(None);
        assert_eq!(point.custom_data(), None);
    }

    #[test]
    fn test_default() {
        let point = DERCurvePointsType::default();

        assert_eq!(point.x(), 0.0);
        assert_eq!(point.y(), 0.0);
        assert_eq!(point.custom_data(), None);
    }
}
