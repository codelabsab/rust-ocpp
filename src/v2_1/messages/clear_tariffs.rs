use crate::v2_1::datatypes::{ClearTariffsResultType, CustomDataType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the ClearTariffs request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearTariffsRequest {
    /// List of tariff Ids to clear. When absent clears all tariffs at _evseId_.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub tariff_ids: Option<Vec<String>>,

    /// When present only clear tariffs matching _tariffIds_ at EVSE _evseId_.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClearTariffsRequest {
    /// Creates a new instance of the struct.
    ///
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new() -> Self {
        Self {
            tariff_ids: None,
            evse_id: None,
            custom_data: None,
        }
    }

    /// Sets the tariff_ids field.
    ///
    /// * `tariff_ids` - List of tariff Ids to clear. When absent clears all tariffs at _evseId_.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_tariff_ids(&mut self, tariff_ids: Option<Vec<String>>) -> &mut Self {
        self.tariff_ids = tariff_ids;
        self
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - When present only clear tariffs matching _tariffIds_ at EVSE _evseId_.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: Option<i32>) -> &mut Self {
        self.evse_id = evse_id;
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

    /// Gets a reference to the tariff_ids field.
    ///
    /// # Returns
    ///
    /// List of tariff Ids to clear. When absent clears all tariffs at _evseId_.
    pub fn get_tariff_ids(&self) -> Option<&Vec<String>> {
        self.tariff_ids.as_ref()
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// When present only clear tariffs matching _tariffIds_ at EVSE _evseId_.
    pub fn get_evse_id(&self) -> Option<&i32> {
        self.evse_id.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the tariff_ids field and returns self for builder pattern.
    ///
    /// * `tariff_ids` - List of tariff Ids to clear. When absent clears all tariffs at _evseId_.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_tariff_ids(mut self, tariff_ids: Vec<String>) -> Self {
        self.tariff_ids = Some(tariff_ids);
        self
    }

    /// Sets the evse_id field and returns self for builder pattern.
    ///
    /// * `evse_id` - When present only clear tariffs matching _tariffIds_ at EVSE _evseId_.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_evse_id(mut self, evse_id: i32) -> Self {
        self.evse_id = Some(evse_id);
        self
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

/// Response body for the ClearTariffs response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearTariffsResponse {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub clear_tariffs_result: Vec<ClearTariffsResultType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClearTariffsResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `clear_tariffs_result` - The clear_tariffs_result field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(clear_tariffs_result: Vec<ClearTariffsResultType>) -> Self {
        Self {
            clear_tariffs_result,
            custom_data: None,
        }
    }

    /// Sets the clear_tariffs_result field.
    ///
    /// * `clear_tariffs_result` - The clear_tariffs_result field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_clear_tariffs_result(&mut self, clear_tariffs_result: Vec<ClearTariffsResultType>) -> &mut Self {
        self.clear_tariffs_result = clear_tariffs_result;
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

    /// Gets a reference to the clear_tariffs_result field.
    ///
    /// # Returns
    ///
    /// The clear_tariffs_result field
    pub fn get_clear_tariffs_result(&self) -> &Vec<ClearTariffsResultType> {
        &self.clear_tariffs_result
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
