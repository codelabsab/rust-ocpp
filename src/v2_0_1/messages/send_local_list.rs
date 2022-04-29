use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::send_local_list_status_enum_type::SendLocalListStatusEnumType;
use crate::v2_0_1::enumerations::update_enum_type::UpdateEnumType;

/// This contains the field definition of the SendLocalListRequest PDU sent by the CSMS to the Charging Station. If no (empty) localAuthorizationList is given and the updateType is Full, all IdTokens are removed from the list. Requesting a Differential update without or with empty localAuthorizationList will have no effect on the list. All IdTokens in the localAuthorizationList MUST be unique, no duplicate values are allowed.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalListRequest {
    pub version_number: i64,
    pub update_type: UpdateEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_info: Option<String>,
}

/// Sent by the CSMS to the Charging Station to confirm the receipt of a SecurityEventNotificationRequest message. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalListResponse {
    pub status: SendLocalListStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>

}
