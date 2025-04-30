use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, signed_meter_value::SignedMeterValueType};
use crate::v2_1::enumerations::{
    LocationEnumType, MeasurandEnumType, PhaseEnumType, ReadingContextEnumType,
};

/// Single sampled value in MeterValues.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SampledValueType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Value as a floating-point number.
    pub value: f64,

    /// Optional. Type of detail value: start, end or sample.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ReadingContextEnumType>,

    /// Optional. Type of measurement value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurand: Option<MeasurandEnumType>,

    /// Optional. Phase as measured or assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<PhaseEnumType>,

    /// Optional. Location of measurement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationEnumType>,

    /// Optional. Contains the signed version of the meter value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_meter_value: Option<SignedMeterValueType>,
}

impl SampledValueType {
    /// Creates a new `SampledValueType` with the required value.
    ///
    /// # Arguments
    ///
    /// * `value` - Value as a floating-point number
    ///
    /// # Returns
    ///
    /// A new instance of `SampledValueType` with optional fields set to `None`
    pub fn new(value: f64) -> Self {
        Self {
            custom_data: None,
            value,
            context: None,
            measurand: None,
            phase: None,
            location: None,
            signed_meter_value: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this sampled value
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the context.
    ///
    /// # Arguments
    ///
    /// * `context` - Type of detail value: start, end or sample
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_context(mut self, context: ReadingContextEnumType) -> Self {
        self.context = Some(context);
        self
    }

    /// Sets the measurand.
    ///
    /// # Arguments
    ///
    /// * `measurand` - Type of measurement value
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_measurand(mut self, measurand: MeasurandEnumType) -> Self {
        self.measurand = Some(measurand);
        self
    }

    /// Sets the phase.
    ///
    /// # Arguments
    ///
    /// * `phase` - Phase as measured or assumed
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_phase(mut self, phase: PhaseEnumType) -> Self {
        self.phase = Some(phase);
        self
    }

    /// Sets the location.
    ///
    /// # Arguments
    ///
    /// * `location` - Location of measurement
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_location(mut self, location: LocationEnumType) -> Self {
        self.location = Some(location);
        self
    }

    /// Sets the signed meter value.
    ///
    /// # Arguments
    ///
    /// * `signed_meter_value` - Contains the signed version of the meter value
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_signed_meter_value(mut self, signed_meter_value: SignedMeterValueType) -> Self {
        self.signed_meter_value = Some(signed_meter_value);
        self
    }

    /// Gets the value.
    ///
    /// # Returns
    ///
    /// The value as a floating-point number
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Sets the value.
    ///
    /// # Arguments
    ///
    /// * `value` - Value as a floating-point number
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_value(&mut self, value: f64) -> &mut Self {
        self.value = value;
        self
    }

    /// Gets the context.
    ///
    /// # Returns
    ///
    /// An optional type of detail value
    pub fn context(&self) -> Option<&ReadingContextEnumType> {
        self.context.as_ref()
    }

    /// Sets the context.
    ///
    /// # Arguments
    ///
    /// * `context` - Type of detail value, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_context(&mut self, context: Option<ReadingContextEnumType>) -> &mut Self {
        self.context = context;
        self
    }

    /// Gets the measurand.
    ///
    /// # Returns
    ///
    /// An optional type of measurement value
    pub fn measurand(&self) -> Option<&MeasurandEnumType> {
        self.measurand.as_ref()
    }

    /// Sets the measurand.
    ///
    /// # Arguments
    ///
    /// * `measurand` - Type of measurement value, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_measurand(&mut self, measurand: Option<MeasurandEnumType>) -> &mut Self {
        self.measurand = measurand;
        self
    }

    /// Gets the phase.
    ///
    /// # Returns
    ///
    /// An optional phase as measured or assumed
    pub fn phase(&self) -> Option<&PhaseEnumType> {
        self.phase.as_ref()
    }

    /// Sets the phase.
    ///
    /// # Arguments
    ///
    /// * `phase` - Phase as measured or assumed, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_phase(&mut self, phase: Option<PhaseEnumType>) -> &mut Self {
        self.phase = phase;
        self
    }

    /// Gets the location.
    ///
    /// # Returns
    ///
    /// An optional location of measurement
    pub fn location(&self) -> Option<&LocationEnumType> {
        self.location.as_ref()
    }

    /// Sets the location.
    ///
    /// # Arguments
    ///
    /// * `location` - Location of measurement, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_location(&mut self, location: Option<LocationEnumType>) -> &mut Self {
        self.location = location;
        self
    }

    /// Gets the signed meter value.
    ///
    /// # Returns
    ///
    /// An optional signed version of the meter value
    pub fn signed_meter_value(&self) -> Option<&SignedMeterValueType> {
        self.signed_meter_value.as_ref()
    }

    /// Sets the signed meter value.
    ///
    /// # Arguments
    ///
    /// * `signed_meter_value` - Signed version of the meter value, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_signed_meter_value(&mut self, signed_meter_value: Option<SignedMeterValueType>) -> &mut Self {
        self.signed_meter_value = signed_meter_value;
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
    /// * `custom_data` - Custom data for this sampled value, or None to clear
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
    use crate::v2_1::enumerations::signing_method::SigningMethodEnumType;

    #[test]
    fn test_new_sampled_value() {
        let value = 42.5;
        let sampled_value = SampledValueType::new(value);

        assert_eq!(sampled_value.value(), value);
        assert_eq!(sampled_value.context(), None);
        assert_eq!(sampled_value.measurand(), None);
        assert_eq!(sampled_value.phase(), None);
        assert_eq!(sampled_value.location(), None);
        assert_eq!(sampled_value.signed_meter_value(), None);
        assert_eq!(sampled_value.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let value = 42.5;
        let context = ReadingContextEnumType::SamplePeriodic;
        let measurand = MeasurandEnumType::CurrentImport;
        let phase = PhaseEnumType::L1;
        let location = LocationEnumType::Outlet;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let signed_meter_value = SignedMeterValueType {
            signed_meter_data: "signed_data".to_string(),
            signing_method: SigningMethodEnumType::Custom("signing_method".to_string()),
            encoding_method: "encoding_method".to_string(),
            public_key: "public_key".to_string(),
            custom_data: None,
        };

        let sampled_value = SampledValueType::new(value)
            .with_context(context.clone())
            .with_measurand(measurand.clone())
            .with_phase(phase.clone())
            .with_location(location.clone())
            .with_signed_meter_value(signed_meter_value.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(sampled_value.value(), value);
        assert_eq!(sampled_value.context(), Some(&context));
        assert_eq!(sampled_value.measurand(), Some(&measurand));
        assert_eq!(sampled_value.phase(), Some(&phase));
        assert_eq!(sampled_value.location(), Some(&location));
        assert_eq!(sampled_value.signed_meter_value(), Some(&signed_meter_value));
        assert_eq!(sampled_value.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let value1 = 42.5;
        let mut sampled_value = SampledValueType::new(value1);

        let value2 = 84.0;
        let context = ReadingContextEnumType::SamplePeriodic;
        let measurand = MeasurandEnumType::CurrentImport;
        let phase = PhaseEnumType::L1;
        let location = LocationEnumType::Outlet;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let signed_meter_value = SignedMeterValueType {
            signed_meter_data: "signed_data".to_string(),
            signing_method: SigningMethodEnumType::Custom("signing_method".to_string()),
            encoding_method: "encoding_method".to_string(),
            public_key: "public_key".to_string(),
            custom_data: None,
        };

        sampled_value
            .set_value(value2)
            .set_context(Some(context.clone()))
            .set_measurand(Some(measurand.clone()))
            .set_phase(Some(phase.clone()))
            .set_location(Some(location.clone()))
            .set_signed_meter_value(Some(signed_meter_value.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(sampled_value.value(), value2);
        assert_eq!(sampled_value.context(), Some(&context));
        assert_eq!(sampled_value.measurand(), Some(&measurand));
        assert_eq!(sampled_value.phase(), Some(&phase));
        assert_eq!(sampled_value.location(), Some(&location));
        assert_eq!(sampled_value.signed_meter_value(), Some(&signed_meter_value));
        assert_eq!(sampled_value.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        sampled_value
            .set_context(None)
            .set_measurand(None)
            .set_phase(None)
            .set_location(None)
            .set_signed_meter_value(None)
            .set_custom_data(None);

        assert_eq!(sampled_value.context(), None);
        assert_eq!(sampled_value.measurand(), None);
        assert_eq!(sampled_value.phase(), None);
        assert_eq!(sampled_value.location(), None);
        assert_eq!(sampled_value.signed_meter_value(), None);
        assert_eq!(sampled_value.custom_data(), None);
    }
}
