use crate::v2_1::datatypes::{CustomDataType, MeterValueType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the MeterValues request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesRequest {
    /// This contains a number (&gt;0) designating an EVSE of the Charging Station. ‘0’ (zero) is used to designate the main power meter.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    #[validate(length(min = 1))]
    #[validate(nested)]
    pub meter_value: Vec<MeterValueType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl MeterValuesRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `evse_id` - This contains a number (&gt;0) designating an EVSE of the Charging Station. ‘0’ (zero) is used to designate the main power meter.
    /// * `meter_value` - The meter_value field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(evse_id: i32, meter_value: Vec<MeterValueType>) -> Self {
        Self {
            evse_id,
            meter_value,
            custom_data: None,
        }
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - This contains a number (&gt;0) designating an EVSE of the Charging Station. ‘0’ (zero) is used to designate the main power meter.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: i32) -> &mut Self {
        self.evse_id = evse_id;
        self
    }

    /// Sets the meter_value field.
    ///
    /// * `meter_value` - The meter_value field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_meter_value(&mut self, meter_value: Vec<MeterValueType>) -> &mut Self {
        self.meter_value = meter_value;
        self
    }

    /// Sets the custom_data field.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// This contains a number (&gt;0) designating an EVSE of the Charging Station. ‘0’ (zero) is used to designate the main power meter.
    pub fn get_evse_id(&self) -> &i32 {
        &self.evse_id
    }

    /// Gets a reference to the meter_value field.
    ///
    /// # Returns
    ///
    /// The meter_value field
    pub fn get_meter_value(&self) -> &Vec<MeterValueType> {
        &self.meter_value
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom_data field and returns self for builder pattern.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

}

/// Response body for the MeterValues response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl MeterValuesResponse {
    /// Creates a new instance of the struct.
    ///
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new() -> Self {
        Self {
            custom_data: None,
        }
    }

    /// Sets the custom_data field.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom_data field and returns self for builder pattern.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

}
