use crate::v2_1::datatypes::{ChargingProfileType, CustomDataType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the ReportChargingProfiles request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReportChargingProfilesRequest {
    /// Id used to match the &lt;&lt;getchargingprofilesrequest, GetChargingProfilesRequest&gt;&gt; message with the resulting ReportChargingProfilesRequest messages. When the CSMS provided a requestId in the &lt;&lt;getchargingprofilesrequest, GetChargingProfilesRequest&gt;&gt;, this field SHALL contain the same value.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// Source that has installed this charging profile. Values defined in Appendix as ChargingLimitSourceEnumStringType.
    #[validate(length(max = 20))]
    pub charging_limit_source: String,

    #[validate(length(min = 1))]
    #[validate(nested)]
    pub charging_profile: Vec<ChargingProfileType>,

    /// To Be Continued. Default value when omitted: false. false indicates that there are no further messages as part of this report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    /// The evse to which the charging profile applies. If evseId = 0, the message contains an overall limit for the Charging Station.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ReportChargingProfilesRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `request_id` - Id used to match the &lt;&lt;getchargingprofilesrequest, GetChargingProfilesRequest&gt;&gt; message with the resulting ReportChargingProfilesRequest messages. When the CSMS provided a requestId in the &lt;&lt;getchargingprofilesrequest, GetChargingProfilesRequest&gt;&gt;, this field SHALL contain the same value.
    /// * `charging_limit_source` - Source that has installed this charging profile. Values defined in Appendix as ChargingLimitSourceEnumStringType.
    /// * `charging_profile` - The charging_profile field
    /// * `evse_id` - The evse to which the charging profile applies. If evseId = 0, the message contains an overall limit for the Charging Station.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(request_id: i32, charging_limit_source: String, charging_profile: Vec<ChargingProfileType>, evse_id: i32) -> Self {
        Self {
            request_id,
            charging_limit_source,
            charging_profile,
            tbc: None,
            evse_id,
            custom_data: None,
        }
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - Id used to match the &lt;&lt;getchargingprofilesrequest, GetChargingProfilesRequest&gt;&gt; message with the resulting ReportChargingProfilesRequest messages. When the CSMS provided a requestId in the &lt;&lt;getchargingprofilesrequest, GetChargingProfilesRequest&gt;&gt;, this field SHALL contain the same value.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Sets the charging_limit_source field.
    ///
    /// * `charging_limit_source` - Source that has installed this charging profile. Values defined in Appendix as ChargingLimitSourceEnumStringType.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_limit_source(&mut self, charging_limit_source: String) -> &mut Self {
        self.charging_limit_source = charging_limit_source;
        self
    }

    /// Sets the charging_profile field.
    ///
    /// * `charging_profile` - The charging_profile field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_profile(&mut self, charging_profile: Vec<ChargingProfileType>) -> &mut Self {
        self.charging_profile = charging_profile;
        self
    }

    /// Sets the tbc field.
    ///
    /// * `tbc` - To Be Continued. Default value when omitted: false. false indicates that there are no further messages as part of this report.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_tbc(&mut self, tbc: Option<bool>) -> &mut Self {
        self.tbc = tbc;
        self
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - The evse to which the charging profile applies. If evseId = 0, the message contains an overall limit for the Charging Station.
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

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// Id used to match the &lt;&lt;getchargingprofilesrequest, GetChargingProfilesRequest&gt;&gt; message with the resulting ReportChargingProfilesRequest messages. When the CSMS provided a requestId in the &lt;&lt;getchargingprofilesrequest, GetChargingProfilesRequest&gt;&gt;, this field SHALL contain the same value.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the charging_limit_source field.
    ///
    /// # Returns
    ///
    /// Source that has installed this charging profile. Values defined in Appendix as ChargingLimitSourceEnumStringType.
    pub fn get_charging_limit_source(&self) -> &String {
        &self.charging_limit_source
    }

    /// Gets a reference to the charging_profile field.
    ///
    /// # Returns
    ///
    /// The charging_profile field
    pub fn get_charging_profile(&self) -> &Vec<ChargingProfileType> {
        &self.charging_profile
    }

    /// Gets a reference to the tbc field.
    ///
    /// # Returns
    ///
    /// To Be Continued. Default value when omitted: false. false indicates that there are no further messages as part of this report.
    pub fn get_tbc(&self) -> Option<&bool> {
        self.tbc.as_ref()
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// The evse to which the charging profile applies. If evseId = 0, the message contains an overall limit for the Charging Station.
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

    /// Sets the tbc field and returns self for builder pattern.
    ///
    /// * `tbc` - To Be Continued. Default value when omitted: false. false indicates that there are no further messages as part of this report.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_tbc(mut self, tbc: bool) -> Self {
        self.tbc = Some(tbc);
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

/// Response body for the ReportChargingProfiles response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReportChargingProfilesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ReportChargingProfilesResponse {
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
