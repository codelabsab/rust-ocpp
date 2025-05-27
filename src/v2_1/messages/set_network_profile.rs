use crate::v2_1::datatypes::{CustomDataType, NetworkConnectionProfileType, StatusInfoType};
use crate::v2_1::enumerations::SetNetworkProfileStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the SetNetworkProfile request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetNetworkProfileRequest {
    /// Slot in which the configuration should be stored.
    pub configuration_slot: i32,

    #[validate(nested)]
    pub connection_data: NetworkConnectionProfileType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetNetworkProfileRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `configuration_slot` - Slot in which the configuration should be stored.
    /// * `connection_data` - The connection_data field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(configuration_slot: i32, connection_data: NetworkConnectionProfileType) -> Self {
        Self {
            configuration_slot,
            connection_data,
            custom_data: None,
        }
    }

    /// Sets the configuration_slot field.
    ///
    /// * `configuration_slot` - Slot in which the configuration should be stored.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_configuration_slot(&mut self, configuration_slot: i32) -> &mut Self {
        self.configuration_slot = configuration_slot;
        self
    }

    /// Sets the connection_data field.
    ///
    /// * `connection_data` - The connection_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_connection_data(&mut self, connection_data: NetworkConnectionProfileType) -> &mut Self {
        self.connection_data = connection_data;
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

    /// Gets a reference to the configuration_slot field.
    ///
    /// # Returns
    ///
    /// Slot in which the configuration should be stored.
    pub fn get_configuration_slot(&self) -> &i32 {
        &self.configuration_slot
    }

    /// Gets a reference to the connection_data field.
    ///
    /// # Returns
    ///
    /// The connection_data field
    pub fn get_connection_data(&self) -> &NetworkConnectionProfileType {
        &self.connection_data
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

/// Response body for the SetNetworkProfile response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetNetworkProfileResponse {
    pub status: SetNetworkProfileStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetNetworkProfileResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: SetNetworkProfileStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
            custom_data: None,
        }
    }

    /// Sets the status field.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_status(&mut self, status: SetNetworkProfileStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Sets the status_info field.
    ///
    /// * `status_info` - The status_info field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_status_info(&mut self, status_info: Option<StatusInfoType>) -> &mut Self {
        self.status_info = status_info;
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

    /// Gets a reference to the status field.
    ///
    /// # Returns
    ///
    /// The status field
    pub fn get_status(&self) -> &SetNetworkProfileStatusEnumType {
        &self.status
    }

    /// Gets a reference to the status_info field.
    ///
    /// # Returns
    ///
    /// The status_info field
    pub fn get_status_info(&self) -> Option<&StatusInfoType> {
        self.status_info.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the status_info field and returns self for builder pattern.
    ///
    /// * `status_info` - The status_info field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_status_info(mut self, status_info: StatusInfoType) -> Self {
        self.status_info = Some(status_info);
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
