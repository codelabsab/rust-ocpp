use crate::v2_1::datatypes::{AuthorizationData, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{SendLocalListStatusEnumType, UpdateEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the SendLocalList request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub local_authorization_list: Option<Vec<AuthorizationData>>,

    /// In case of a full update this is the version number of the full list. In case of a differential update it is the version number of the list after the update has been applied.
    pub version_number: i32,

    pub update_type: UpdateEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SendLocalListRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `version_number` - In case of a full update this is the version number of the full list. In case of a differential update it is the version number of the list after the update has been applied.
    /// * `update_type` - The update_type field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(version_number: i32, update_type: UpdateEnumType) -> Self {
        Self {
            local_authorization_list: None,
            version_number,
            update_type,
            custom_data: None,
        }
    }

    /// Sets the local_authorization_list field.
    ///
    /// * `local_authorization_list` - The local_authorization_list field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_local_authorization_list(&mut self, local_authorization_list: Option<Vec<AuthorizationData>>) -> &mut Self {
        self.local_authorization_list = local_authorization_list;
        self
    }

    /// Sets the version_number field.
    ///
    /// * `version_number` - In case of a full update this is the version number of the full list. In case of a differential update it is the version number of the list after the update has been applied.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_version_number(&mut self, version_number: i32) -> &mut Self {
        self.version_number = version_number;
        self
    }

    /// Sets the update_type field.
    ///
    /// * `update_type` - The update_type field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_update_type(&mut self, update_type: UpdateEnumType) -> &mut Self {
        self.update_type = update_type;
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

    /// Gets a reference to the local_authorization_list field.
    ///
    /// # Returns
    ///
    /// The local_authorization_list field
    pub fn get_local_authorization_list(&self) -> Option<&Vec<AuthorizationData>> {
        self.local_authorization_list.as_ref()
    }

    /// Gets a reference to the version_number field.
    ///
    /// # Returns
    ///
    /// In case of a full update this is the version number of the full list. In case of a differential update it is the version number of the list after the update has been applied.
    pub fn get_version_number(&self) -> &i32 {
        &self.version_number
    }

    /// Gets a reference to the update_type field.
    ///
    /// # Returns
    ///
    /// The update_type field
    pub fn get_update_type(&self) -> &UpdateEnumType {
        &self.update_type
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the local_authorization_list field and returns self for builder pattern.
    ///
    /// * `local_authorization_list` - The local_authorization_list field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_local_authorization_list(mut self, local_authorization_list: Vec<AuthorizationData>) -> Self {
        self.local_authorization_list = Some(local_authorization_list);
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

/// Response body for the SendLocalList response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalListResponse {
    pub status: SendLocalListStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SendLocalListResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: SendLocalListStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: SendLocalListStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &SendLocalListStatusEnumType {
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
