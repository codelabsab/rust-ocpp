use crate::v2_0_1::core::{
    datatypes::{
        network_connection_profile_type::NetworkConnectionProfileType,
        status_info_type::StatusInfoType,
    },
    enumerations::set_network_profile_status_enum_type::SetNetworkProfileStatusEnumType,
};

/// With this message the CSMS gains the ability to configure the connection data (e.g. CSMS URL, OCPP version, APN, etc) on a Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetNetworkProfileRequest {
    pub configuration_slot: i64,
    pub connection_data: NetworkConnectionProfileType,
}

/// This contains the field definition of the SetNetworkProfileResponse PDU sent by the Charging Station to the CSMS in response to a SetNetworkProfileRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetNetworkProfileResponse {
    pub status: SetNetworkProfileStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
