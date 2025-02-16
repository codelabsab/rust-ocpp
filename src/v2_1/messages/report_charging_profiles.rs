use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{ChargingProfileType, CustomDataType};

/// Request body for the ReportChargingProfiles request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReportChargingProfilesRequest {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Source that has installed this charging profile. Values defined in Appendix as ChargingLimitSourceEnumStringType.
    #[validate(length(max = 20))]
    pub charging_limit_source: String,

    /// Required. Array of charging profiles that are installed on the Charging Station.
    #[validate(length(min = 1))]
    pub charging_profile: Vec<ChargingProfileType>,

    /// Required. Id used to match the GetChargingProfilesRequest message with the resulting ReportChargingProfilesRequest messages. When the CSMS provided a requestId in the GetChargingProfilesRequest, this field SHALL contain the same value.
    pub request_id: i32,

    /// Required. The evse to which the charging profile applies. If evseId = 0, the message contains an overall limit for the Charging Station.
    pub evse_id: i32,

    /// Optional. To Be Continued. Default value when omitted: false. false indicates that there are no further messages as part of this report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
}

/// Response body for the ReportChargingProfiles response.
/// This contains no fields as per the OCPP 2.1 specification.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReportChargingProfilesResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
