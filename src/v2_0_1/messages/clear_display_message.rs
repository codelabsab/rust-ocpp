//! ClearDisplayMessage
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::clear_message_status_enum_type::ClearMessageStatusEnumType;

/// ClearDisplayMessageRequest, sent by the CSMS to the Charging Station.
///
/// The CSMS asks the Charging Station to clear a display message that has been
/// configured in the Charging Station to be cleared/removed.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearDisplayMessageRequest {
    /// Id of the message that SHALL be removed from the Charging Station
    pub id: i32,
}

/// ClearDisplayMessageResponse, sent by the Charging Station to the CSMS in a response to a [`ClearDisplayMessageRequest`].
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearDisplayMessageResponse {
    /// Returns whether the Charging Station has been able to remove the message.
    pub status: ClearMessageStatusEnumType,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
