use crate::v2_0_1::core::{
    datatypes::{
        clear_charging_profile_type::ClearChargingProfileType, status_info_type::StatusInfoType,
    },
    enumerations::clear_charging_profile_status_enum_type::ClearChargingProfileStatusEnumType,
};

/// This contains the field definition of the ClearChargingProfileRequest PDU sent by the CSMS to the Charging Station. The CSMS can use this message to clear (remove) either a specific charging profile (denoted by id) or a selection of charging profiles that match with the values of the optional evse, stackLevel and ChargingProfilePurpose fields.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_criteria: Option<ClearChargingProfileType>,
}

/// This contains the field definition of the ClearChargingProfileResponse PDU sent by the Charging Station to the CSMS in response to a ClearChargingProfileRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileResponse {
    pub status: ClearChargingProfileStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
