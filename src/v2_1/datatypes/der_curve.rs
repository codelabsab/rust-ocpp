use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    custom_data::CustomDataType, der_curve_points::DERCurvePointsType, hysteresis::HysteresisType,
    reactive_power_params::ReactivePowerParamsType, voltage_params::VoltageParamsType,
};
use crate::v2_1::enumerations::der_unit::DERUnitEnumType;

/// DER curve type for various DER control modes.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DERCurveType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// List of curve points defining this curve.
    #[validate(length(min = 1, max = 10))]
    pub curve_data: Vec<DERCurvePointsType>,

    /// Hysteresis parameters for this curve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hysteresis: Option<HysteresisType>,

    /// Priority of curve (0=highest)
    #[validate(range(min = 0))]
    pub priority: i32,

    /// Reactive power parameters for this curve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactive_power_params: Option<ReactivePowerParamsType>,

    /// Voltage parameters for this curve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage_params: Option<VoltageParamsType>,

    /// Unit of the Y-axis values.
    pub y_unit: DERUnitEnumType,

    /// Open loop response time, the time to ramp up to 90% of the new target in response to the change in voltage, in seconds.
    /// A value of 0 is used to mean no limit. When not present, the device should follow its default behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time: Option<f64>,

    /// Point in time when this curve will become activated. Only absent when default is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,

    /// Duration in seconds that this curve will be active. Only absent when default is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
}

impl DERCurveType {
    /// Creates a new `DERCurveType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `curve_data` - List of curve points defining this curve
    /// * `priority` - Priority of curve (0=highest)
    /// * `y_unit` - Unit of the Y-axis values
    ///
    /// # Returns
    ///
    /// A new instance of `DERCurveType` with optional fields set to `None`
    pub fn new(
        curve_data: Vec<DERCurvePointsType>,
        priority: i32,
        y_unit: DERUnitEnumType,
    ) -> Self {
        Self {
            curve_data,
            priority,
            y_unit,
            custom_data: None,
            hysteresis: None,
            reactive_power_params: None,
            voltage_params: None,
            response_time: None,
            start_time: None,
            duration: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this curve
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the hysteresis parameters.
    ///
    /// # Arguments
    ///
    /// * `hysteresis` - Hysteresis parameters for this curve
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_hysteresis(mut self, hysteresis: HysteresisType) -> Self {
        self.hysteresis = Some(hysteresis);
        self
    }

    /// Sets the reactive power parameters.
    ///
    /// # Arguments
    ///
    /// * `reactive_power_params` - Reactive power parameters for this curve
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_reactive_power_params(
        mut self,
        reactive_power_params: ReactivePowerParamsType,
    ) -> Self {
        self.reactive_power_params = Some(reactive_power_params);
        self
    }

    /// Sets the voltage parameters.
    ///
    /// # Arguments
    ///
    /// * `voltage_params` - Voltage parameters for this curve
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_voltage_params(mut self, voltage_params: VoltageParamsType) -> Self {
        self.voltage_params = Some(voltage_params);
        self
    }

    /// Sets the response time.
    ///
    /// # Arguments
    ///
    /// * `response_time` - Open loop response time in seconds
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_response_time(mut self, response_time: f64) -> Self {
        self.response_time = Some(response_time);
        self
    }

    /// Sets the start time.
    ///
    /// # Arguments
    ///
    /// * `start_time` - Point in time when this curve will become activated
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_start_time(mut self, start_time: DateTime<Utc>) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// Sets the duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration in seconds that this curve will be active
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Gets the curve data.
    ///
    /// # Returns
    ///
    /// A reference to the list of curve points
    pub fn curve_data(&self) -> &Vec<DERCurvePointsType> {
        &self.curve_data
    }

    /// Sets the curve data.
    ///
    /// # Arguments
    ///
    /// * `curve_data` - List of curve points defining this curve
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_curve_data(&mut self, curve_data: Vec<DERCurvePointsType>) -> &mut Self {
        self.curve_data = curve_data;
        self
    }

    /// Gets the priority.
    ///
    /// # Returns
    ///
    /// The priority of the curve
    pub fn priority(&self) -> i32 {
        self.priority
    }

    /// Sets the priority.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of curve (0=highest)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_priority(&mut self, priority: i32) -> &mut Self {
        self.priority = priority;
        self
    }

    /// Gets the Y-axis unit.
    ///
    /// # Returns
    ///
    /// The unit of the Y-axis values
    pub fn y_unit(&self) -> DERUnitEnumType {
        self.y_unit.clone()
    }

    /// Sets the Y-axis unit.
    ///
    /// # Arguments
    ///
    /// * `y_unit` - Unit of the Y-axis values
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_y_unit(&mut self, y_unit: DERUnitEnumType) -> &mut Self {
        self.y_unit = y_unit;
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
    /// * `custom_data` - Custom data for this curve, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the hysteresis parameters.
    ///
    /// # Returns
    ///
    /// An optional reference to the hysteresis parameters
    pub fn hysteresis(&self) -> Option<&HysteresisType> {
        self.hysteresis.as_ref()
    }

    /// Sets the hysteresis parameters.
    ///
    /// # Arguments
    ///
    /// * `hysteresis` - Hysteresis parameters for this curve, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_hysteresis(&mut self, hysteresis: Option<HysteresisType>) -> &mut Self {
        self.hysteresis = hysteresis;
        self
    }

    /// Gets the reactive power parameters.
    ///
    /// # Returns
    ///
    /// An optional reference to the reactive power parameters
    pub fn reactive_power_params(&self) -> Option<&ReactivePowerParamsType> {
        self.reactive_power_params.as_ref()
    }

    /// Sets the reactive power parameters.
    ///
    /// # Arguments
    ///
    /// * `reactive_power_params` - Reactive power parameters for this curve, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_reactive_power_params(
        &mut self,
        reactive_power_params: Option<ReactivePowerParamsType>,
    ) -> &mut Self {
        self.reactive_power_params = reactive_power_params;
        self
    }

    /// Gets the voltage parameters.
    ///
    /// # Returns
    ///
    /// An optional reference to the voltage parameters
    pub fn voltage_params(&self) -> Option<&VoltageParamsType> {
        self.voltage_params.as_ref()
    }

    /// Sets the voltage parameters.
    ///
    /// # Arguments
    ///
    /// * `voltage_params` - Voltage parameters for this curve, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_voltage_params(&mut self, voltage_params: Option<VoltageParamsType>) -> &mut Self {
        self.voltage_params = voltage_params;
        self
    }

    /// Gets the response time.
    ///
    /// # Returns
    ///
    /// An optional reference to the response time
    pub fn response_time(&self) -> Option<f64> {
        self.response_time
    }

    /// Sets the response time.
    ///
    /// # Arguments
    ///
    /// * `response_time` - Open loop response time in seconds, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_response_time(&mut self, response_time: Option<f64>) -> &mut Self {
        self.response_time = response_time;
        self
    }

    /// Gets the start time.
    ///
    /// # Returns
    ///
    /// An optional reference to the start time
    pub fn start_time(&self) -> Option<&DateTime<Utc>> {
        self.start_time.as_ref()
    }

    /// Sets the start time.
    ///
    /// # Arguments
    ///
    /// * `start_time` - Point in time when this curve will become activated, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_start_time(&mut self, start_time: Option<DateTime<Utc>>) -> &mut Self {
        self.start_time = start_time;
        self
    }

    /// Gets the duration.
    ///
    /// # Returns
    ///
    /// An optional reference to the duration
    pub fn duration(&self) -> Option<f64> {
        self.duration
    }

    /// Sets the duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration in seconds that this curve will be active, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_duration(&mut self, duration: Option<f64>) -> &mut Self {
        self.duration = duration;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_new_der_curve() {
        let curve_points = vec![DERCurvePointsType::default()];
        let priority = 1;
        let y_unit = DERUnitEnumType::PctMaxW;

        let curve = DERCurveType::new(curve_points.clone(), priority, y_unit.clone());

        assert_eq!(curve.curve_data(), &curve_points);
        assert_eq!(curve.priority(), priority);
        assert_eq!(curve.y_unit(), y_unit);
        assert_eq!(curve.custom_data(), None);
        assert_eq!(curve.hysteresis(), None);
        assert_eq!(curve.reactive_power_params(), None);
        assert_eq!(curve.voltage_params(), None);
        assert_eq!(curve.response_time(), None);
        assert_eq!(curve.start_time(), None);
        assert_eq!(curve.duration(), None);
    }

    #[test]
    fn test_with_optional_fields() {
        let curve_points = vec![DERCurvePointsType::default()];
        let priority = 1;
        let y_unit = DERUnitEnumType::PctMaxW;
        let custom_data = CustomDataType::new("VendorX".to_string());
        let start_time = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

        let curve = DERCurveType::new(curve_points.clone(), priority, y_unit.clone())
            .with_custom_data(custom_data.clone())
            .with_response_time(10.5)
            .with_start_time(start_time.clone())
            .with_duration(3600.0);

        assert_eq!(curve.curve_data(), &curve_points);
        assert_eq!(curve.priority(), priority);
        assert_eq!(curve.y_unit(), y_unit);
        assert_eq!(curve.custom_data(), Some(&custom_data));
        assert_eq!(curve.response_time(), Some(10.5));
        assert_eq!(curve.start_time(), Some(&start_time));
        assert_eq!(curve.duration(), Some(3600.0));
    }

    #[test]
    fn test_setter_methods() {
        let curve_points1 = vec![DERCurvePointsType::default()];
        let curve_points2 = vec![DERCurvePointsType::default(), DERCurvePointsType::default()];
        let priority1 = 1;
        let priority2 = 2;
        let y_unit1 = DERUnitEnumType::PctMaxW;
        let y_unit2 = DERUnitEnumType::PctMaxVar;
        let custom_data = CustomDataType::new("VendorX".to_string());
        let start_time = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

        let mut curve = DERCurveType::new(curve_points1.clone(), priority1, y_unit1.clone());

        curve
            .set_curve_data(curve_points2.clone())
            .set_priority(priority2)
            .set_y_unit(y_unit2.clone())
            .set_custom_data(Some(custom_data.clone()))
            .set_response_time(Some(10.5))
            .set_start_time(Some(start_time.clone()))
            .set_duration(Some(3600.0));

        assert_eq!(curve.curve_data(), &curve_points2);
        assert_eq!(curve.priority(), priority2);
        assert_eq!(curve.y_unit(), y_unit2);
        assert_eq!(curve.custom_data(), Some(&custom_data));
        assert_eq!(curve.response_time(), Some(10.5));
        assert_eq!(curve.start_time(), Some(&start_time));
        assert_eq!(curve.duration(), Some(3600.0));

        // Test clearing optional fields
        curve
            .set_custom_data(None)
            .set_response_time(None)
            .set_start_time(None)
            .set_duration(None);

        assert_eq!(curve.custom_data(), None);
        assert_eq!(curve.response_time(), None);
        assert_eq!(curve.start_time(), None);
        assert_eq!(curve.duration(), None);
    }
}
