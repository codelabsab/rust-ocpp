use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{
    custom_data::CustomDataType, message_content::MessageContentType, status_info::StatusInfoType,
};
use crate::v2_1::enumerations::tariff_change_status::TariffChangeStatusEnumType;

/// Request to change the tariff for an ongoing transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTransactionTariffRequest {
    /// Required. Transaction Id for which the tariff needs to be changed.
    pub transaction_id: String,

    /// Required. The new tariff that should be applied.
    pub tariff_id: String,

    /// Optional. Message content to be displayed to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_content: Option<MessageContentType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a ChangeTransactionTariffRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTransactionTariffResponse {
    /// Required. Status indicating whether the Charging Station accepts the request.
    pub status: TariffChangeStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
