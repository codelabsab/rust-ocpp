use crate::v2_0_1::core::datatypes::evse_type::EVSEType;
use crate::v2_0_1::core::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::core::enumerations::change_availability_status_enum_type::ChangeAvailabilityStatusEnumType;
use crate::v2_0_1::core::enumerations::operational_status_enum_type::OperationalStatusEnumType;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityRequest {
    pub operational_status: OperationalStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityResponse {
    pub status: ChangeAvailabilityStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
