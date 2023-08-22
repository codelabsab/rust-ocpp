use crate::v2_0_1::datatypes::message_info_type::MessageInfoType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::display_message_status_enum_type::DisplayMessageStatusEnumType;

/// This contains the field definition of the SetDisplayMessageRequest PDU sent by the CSMS to the Charging Station. The CSMS asks the Charging Station to configure a new display message that the Charging Station will display (in the future). See also O01 - Set Display Message, O02 - Set Display Message for Transaction and O06 - Replace Display Message
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDisplayMessageRequest<'a> {
    #[serde(borrow)]
    pub message: MessageInfoType<'a>,
}

/// This contains the field definition of the SetDisplayMessageResponse PDU sent by the Charging Station to the CSMS in a response to a SetDisplayMessageRequest. See also O01 - Set Display Message, O02 - Set Display Message for Transaction and O06 - Replace Display Message
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDisplayMessageResponse<'a> {
    pub status: DisplayMessageStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none",borrow)]
    pub status_info: Option<StatusInfoType<'a>>,
}
