use crate::v2_1::datatypes::{CustomDataType, StatusInfoType, TariffAssignmentType};
use crate::v2_1::enumerations::TariffGetStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetTariffs request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetTariffsRequest {
    /// EVSE id to get tariff from. When _evseId_ = 0, this gets tariffs from all EVSEs.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetTariffsRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `evse_id` - EVSE id to get tariff from. When _evseId_ = 0, this gets tariffs from all EVSEs.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(evse_id: i32) -> Self {
        Self {
            evse_id,
            custom_data: None,
        }
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - EVSE id to get tariff from. When _evseId_ = 0, this gets tariffs from all EVSEs.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: i32) -> &mut Self {
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

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// EVSE id to get tariff from. When _evseId_ = 0, this gets tariffs from all EVSEs.
    pub fn get_evse_id(&self) -> &i32 {
        &self.evse_id
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

/// Response body for the GetTariffs response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetTariffsResponse {
    pub status: TariffGetStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub tariff_assignments: Option<Vec<TariffAssignmentType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetTariffsResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: TariffGetStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
            tariff_assignments: None,
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
    pub fn set_status(&mut self, status: TariffGetStatusEnumType) -> &mut Self {
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

    /// Sets the tariff_assignments field.
    ///
    /// * `tariff_assignments` - The tariff_assignments field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_tariff_assignments(&mut self, tariff_assignments: Option<Vec<TariffAssignmentType>>) -> &mut Self {
        self.tariff_assignments = tariff_assignments;
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
    pub fn get_status(&self) -> &TariffGetStatusEnumType {
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

    /// Gets a reference to the tariff_assignments field.
    ///
    /// # Returns
    ///
    /// The tariff_assignments field
    pub fn get_tariff_assignments(&self) -> Option<&Vec<TariffAssignmentType>> {
        self.tariff_assignments.as_ref()
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

    /// Sets the tariff_assignments field and returns self for builder pattern.
    ///
    /// * `tariff_assignments` - The tariff_assignments field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_tariff_assignments(mut self, tariff_assignments: Vec<TariffAssignmentType>) -> Self {
        self.tariff_assignments = Some(tariff_assignments);
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
