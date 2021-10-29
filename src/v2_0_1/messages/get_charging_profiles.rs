//! The message GetChargingProfilesRequest can be used by the CSMS to request installed
//! charging profiles from the Charging Station. The charging profiles will then be
//! reported by the Charging Station via ReportChargingProfilesRequest messages.
use crate::v2_0_1::datatypes::charging_profile_criterion_type::ChargingProfileCriterionType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::get_charging_profile_status_enum_type::GetChargingProfileStatusEnumType;

/// The message GetChargingProfilesRequest can be used by the CSMS to request installed
/// charging profiles from the Charging Station. The charging profiles will then be
/// reported by the Charging Station via ReportChargingProfilesRequest messages.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetChargingProfilesRequest {
    pub request_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i64>,
    pub charging_profile: ChargingProfileCriterionType,
}

/// This contains the field definition of the GetChargingProfilesResponse PDU sent by the Charging Station to the CSMS in response to a GetChargingProfilesRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetChargingProfilesResponse {
    pub status: GetChargingProfileStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
