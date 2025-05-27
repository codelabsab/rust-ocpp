use crate::v2_1::datatypes::{ChargingProfileCriterionType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::GetChargingProfileStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetChargingProfiles request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetChargingProfilesRequest {
    /// Reference identification that is to be used by the Charging Station in the &lt;&lt;reportchargingprofilesrequest, ReportChargingProfilesRequest&gt;&gt; when provided.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// For which EVSE installed charging profiles SHALL be reported. If 0, only charging profiles installed on the Charging Station itself (the grid connection) SHALL be reported. If omitted, all installed charging profiles SHALL be reported. + Reported charging profiles SHALL match the criteria in field _chargingProfile_.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    #[validate(nested)]
    pub charging_profile: ChargingProfileCriterionType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetChargingProfilesRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `request_id` - Reference identification that is to be used by the Charging Station in the &lt;&lt;reportchargingprofilesrequest, ReportChargingProfilesRequest&gt;&gt; when provided.
    /// * `charging_profile` - The charging_profile field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(request_id: i32, charging_profile: ChargingProfileCriterionType) -> Self {
        Self {
            request_id,
            evse_id: None,
            charging_profile,
            custom_data: None,
        }
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - Reference identification that is to be used by the Charging Station in the &lt;&lt;reportchargingprofilesrequest, ReportChargingProfilesRequest&gt;&gt; when provided.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - For which EVSE installed charging profiles SHALL be reported. If 0, only charging profiles installed on the Charging Station itself (the grid connection) SHALL be reported. If omitted, all installed charging profiles SHALL be reported. + Reported charging profiles SHALL match the criteria in field _chargingProfile_.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: Option<i32>) -> &mut Self {
        self.evse_id = evse_id;
        self
    }

    /// Sets the charging_profile field.
    ///
    /// * `charging_profile` - The charging_profile field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_profile(&mut self, charging_profile: ChargingProfileCriterionType) -> &mut Self {
        self.charging_profile = charging_profile;
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

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// Reference identification that is to be used by the Charging Station in the &lt;&lt;reportchargingprofilesrequest, ReportChargingProfilesRequest&gt;&gt; when provided.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// For which EVSE installed charging profiles SHALL be reported. If 0, only charging profiles installed on the Charging Station itself (the grid connection) SHALL be reported. If omitted, all installed charging profiles SHALL be reported. + Reported charging profiles SHALL match the criteria in field _chargingProfile_.
    pub fn get_evse_id(&self) -> Option<&i32> {
        self.evse_id.as_ref()
    }

    /// Gets a reference to the charging_profile field.
    ///
    /// # Returns
    ///
    /// The charging_profile field
    pub fn get_charging_profile(&self) -> &ChargingProfileCriterionType {
        &self.charging_profile
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the evse_id field and returns self for builder pattern.
    ///
    /// * `evse_id` - For which EVSE installed charging profiles SHALL be reported. If 0, only charging profiles installed on the Charging Station itself (the grid connection) SHALL be reported. If omitted, all installed charging profiles SHALL be reported. + Reported charging profiles SHALL match the criteria in field _chargingProfile_.
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

/// Response body for the GetChargingProfiles response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetChargingProfilesResponse {
    pub status: GetChargingProfileStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetChargingProfilesResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: GetChargingProfileStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: GetChargingProfileStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &GetChargingProfileStatusEnumType {
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
