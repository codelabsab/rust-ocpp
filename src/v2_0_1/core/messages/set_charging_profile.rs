use crate::v2_0_1::core::{
    datatypes::{charging_profile_type::ChargingProfileType, status_info_type::StatusInfoType},
    enumerations::charging_profile_status_enum_type::ChargingProfileStatusEnumType,
};

/// This contains the field definition of the SetChargingProfileRequest PDU sent by the CSMS to the Charging Station. The CSMS uses this message to send charging profiles to a Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileRequest {
    pub evse_id: i64,
    pub charging_profile: ChargingProfileType,
}

/// This contains the field definition of the SetChargingProfileResponse PDU sent by the Charging Station to the CSMS in response to SetChargingProfileRequest PDU.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileResponse {
    pub status: ChargingProfileStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
