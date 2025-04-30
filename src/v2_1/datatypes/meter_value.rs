use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, sampled_value::SampledValueType};

/// Collection of one or more sampled values in MeterValuesRequest and StopTransactionRequest.
/// All sampled values in a MeterValue are sampled at the same point in time.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeterValueType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Timestamp for measured value(s).
    pub timestamp: DateTime<Utc>,

    /// Required. One or more measured values.
    #[validate(length(min = 1))]
    pub sampled_value: Vec<SampledValueType>,
}

impl MeterValueType {
    /// Creates a new `MeterValueType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Timestamp for measured value(s)
    /// * `sampled_value` - One or more measured values
    ///
    /// # Returns
    ///
    /// A new instance of `MeterValueType` with optional fields set to `None`
    pub fn new(timestamp: DateTime<Utc>, sampled_value: Vec<SampledValueType>) -> Self {
        Self {
            custom_data: None,
            timestamp,
            sampled_value,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this MeterValue
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
    /// * `custom_data` - Custom data for this MeterValue, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the timestamp.
    ///
    /// # Returns
    ///
    /// The timestamp for measured value(s)
    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    /// Sets the timestamp.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Timestamp for measured value(s)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_timestamp(&mut self, timestamp: DateTime<Utc>) -> &mut Self {
        self.timestamp = timestamp;
        self
    }

    /// Gets the sampled values.
    ///
    /// # Returns
    ///
    /// Reference to the vector of sampled values
    pub fn sampled_value(&self) -> &[SampledValueType] {
        &self.sampled_value
    }

    /// Sets the sampled values.
    ///
    /// # Arguments
    ///
    /// * `sampled_value` - Vector of sampled values
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_sampled_value(&mut self, sampled_value: Vec<SampledValueType>) -> &mut Self {
        self.sampled_value = sampled_value;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_meter_value() {
        let timestamp = Utc::now();
        let sampled_value = vec![SampledValueType {
            value: 42.0,
            context: None,
            measurand: None,
            phase: None,
            location: None,
            signed_meter_value: None,
            custom_data: None,
        }];

        let meter_value = MeterValueType::new(timestamp, sampled_value.clone());

        assert_eq!(meter_value.timestamp(), timestamp);
        assert_eq!(meter_value.sampled_value(), &sampled_value);
        assert_eq!(meter_value.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let timestamp = Utc::now();
        let sampled_value = vec![SampledValueType {
            value: 42.0,
            context: None,
            measurand: None,
            phase: None,
            location: None,
            signed_meter_value: None,
            custom_data: None,
        }];
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let meter_value = MeterValueType::new(timestamp, sampled_value.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(meter_value.timestamp(), timestamp);
        assert_eq!(meter_value.sampled_value(), &sampled_value);
        assert_eq!(meter_value.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let timestamp1 = Utc::now();
        let timestamp2 = timestamp1 + chrono::Duration::seconds(60);

        let sampled_value1 = vec![SampledValueType {
            value: 42.0,
            context: None,
            measurand: None,
            phase: None,
            location: None,
            signed_meter_value: None,
            custom_data: None,
        }];

        let sampled_value2 = vec![SampledValueType {
            value: 50.0,
            context: None,
            measurand: None,
            phase: None,
            location: None,
            signed_meter_value: None,
            custom_data: None,
        }];

        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut meter_value = MeterValueType::new(timestamp1, sampled_value1.clone());

        meter_value
            .set_timestamp(timestamp2)
            .set_sampled_value(sampled_value2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(meter_value.timestamp(), timestamp2);
        assert_eq!(meter_value.sampled_value(), &sampled_value2);
        assert_eq!(meter_value.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        meter_value.set_custom_data(None);
        assert_eq!(meter_value.custom_data(), None);
    }
}
