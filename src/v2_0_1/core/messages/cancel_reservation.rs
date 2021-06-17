use crate::v2_0_1::core::{
    datatypes::status_info_type::StatusInfoType,
    enumerations::cancel_reservation_status_enum_type::CancelReservationStatusEnumType,
};

/// This contains the field definition of the CancelReservationRequest PDU sent by the CSMS to the Charging Station
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservationRequest {
    pub reservation_id: i64,
}

/// This contains the field definition of the CancelReservationResponse PDU sent by the Charging Station to the CSMS in response to a CancelReservationRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservationResponse {
    pub status: CancelReservationStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
