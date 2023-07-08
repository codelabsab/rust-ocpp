//! ChangeAvailability
use crate::datatypes::evse_type::EVSEType;
use crate::datatypes::status_info_type::StatusInfoType;
use crate::enumerations::change_availability_status_enum_type::ChangeAvailabilityStatusEnumType;
use crate::enumerations::operational_status_enum_type::OperationalStatusEnumType;

/// `ChangeAvailabilityRequest`, sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityRequest {
    /// This contains the type of availability change that the Charging Station should perform.
    pub operational_status: OperationalStatusEnumType,
    /// Contains Id’s to designate a specific EVSE/connector by index numbers. When omitted, the message refers to the Charging Station as a whole.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
}

/// `ChangeAvailabilityResponse`, sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityResponse {
    /// This indicates whether the Charging Station is able to perform the availability change.
    pub status: ChangeAvailabilityStatusEnumType,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
