use crate::v2_1::datatypes::{BatteryDataType, CustomDataType, IdTokenType};
use crate::v2_1::enumerations::BatterySwapEventEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the BatterySwap request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BatterySwapRequest {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub battery_data: Vec<BatteryDataType>,

    pub event_type: BatterySwapEventEnumType,

    #[validate(nested)]
    pub id_token: IdTokenType,

    /// RequestId to correlate BatteryIn/Out events and optional RequestBatterySwapRequest.
    #[validate(range(min = 0))]
    pub request_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl BatterySwapRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `battery_data` - The battery_data field
    /// * `event_type` - The event_type field
    /// * `id_token` - The id_token field
    /// * `request_id` - RequestId to correlate BatteryIn/Out events and optional RequestBatterySwapRequest.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(battery_data: Vec<BatteryDataType>, event_type: BatterySwapEventEnumType, id_token: IdTokenType, request_id: i32) -> Self {
        Self {
            battery_data,
            event_type,
            id_token,
            request_id,
            custom_data: None,
        }
    }

    /// Sets the battery_data field.
    ///
    /// * `battery_data` - The battery_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_battery_data(&mut self, battery_data: Vec<BatteryDataType>) -> &mut Self {
        self.battery_data = battery_data;
        self
    }

    /// Sets the event_type field.
    ///
    /// * `event_type` - The event_type field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_event_type(&mut self, event_type: BatterySwapEventEnumType) -> &mut Self {
        self.event_type = event_type;
        self
    }

    /// Sets the id_token field.
    ///
    /// * `id_token` - The id_token field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_id_token(&mut self, id_token: IdTokenType) -> &mut Self {
        self.id_token = id_token;
        self
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - RequestId to correlate BatteryIn/Out events and optional RequestBatterySwapRequest.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
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

    /// Gets a reference to the battery_data field.
    ///
    /// # Returns
    ///
    /// The battery_data field
    pub fn get_battery_data(&self) -> &Vec<BatteryDataType> {
        &self.battery_data
    }

    /// Gets a reference to the event_type field.
    ///
    /// # Returns
    ///
    /// The event_type field
    pub fn get_event_type(&self) -> &BatterySwapEventEnumType {
        &self.event_type
    }

    /// Gets a reference to the id_token field.
    ///
    /// # Returns
    ///
    /// The id_token field
    pub fn get_id_token(&self) -> &IdTokenType {
        &self.id_token
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// RequestId to correlate BatteryIn/Out events and optional RequestBatterySwapRequest.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
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

/// Response body for the BatterySwap response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BatterySwapResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl BatterySwapResponse {
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
