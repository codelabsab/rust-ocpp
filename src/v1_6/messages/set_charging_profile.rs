use crate::v1_6::types::{ChargingProfile, ChargingProfileStatus};

/// This contains the field definition of the SendLocalListRequest PDU sent by the Central System to the Charge Point. If no (empty) local_authorization_list is given and the updateType is Full, all identifications are removed from the list. Requesting a Differential update without (empty) local_authorization_list will have no effect on the list. All idTags in the local_authorization_list MUST be unique, no duplicate values are allowed. See also Send Local List
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileRequest<const CHARGING_SCHEDULE_SIZE: usize> {
    /// Required. The connector to which the charging profile applies. If connectorId = 0, the message contains an overall limit for the Charge Point.
    pub connector_id: i64,
    /// Required. The charging profile to be set at the Charge Point.
    #[serde(rename = "csChargingProfiles")]
    pub cs_charging_profiles: ChargingProfile<CHARGING_SCHEDULE_SIZE>,
}

/// This contains the field definition of the ResetResponse PDU sent by the Charge Point to the Central System inresponse to a ResetRequest PDU. See also Reset
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileResponse {
    // Required. This indicates whether the Charge Point is able to perform the reset.
    pub status: ChargingProfileStatus,
}
