use super::super::datatypes::{CustomDataType, StatusInfoType};
use super::super::enumerations::CancelReservationStatusEnumType;
use serde::{Deserialize, Serialize};

/// Response to a CancelReservationRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelReservationResponse {
    /// This indicates the success or failure of the canceling of a reservation by CSMS.
    pub status: CancelReservationStatusEnumType,

    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
