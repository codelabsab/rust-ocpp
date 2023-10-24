use crate::v2_0_1::enumerations::reservation_update_status_enum_type::ReservationUpdateStatusEnumType;

/// This contains the field definition of the ReservationStatusUpdateRequest PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReservationStatusUpdateRequest {
    pub reservation_id: i32,
    pub reservation_update_status: ReservationUpdateStatusEnumType,
}

/// This contains the field definition of the ReservationStatusUpdateResponse PDU sent by the CSMS to the Charging Station in response to a ReservationStatusUpdateRequest. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReservationStatusUpdateResponse {}
