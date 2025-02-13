use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// EVSE object with properties common to OCPP 2.0.1 and OCPP 2.1.0.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EVSEType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Identified Object. MRID. Numeric ID of the EVSE within the Charging Station.
    pub id: i32,

    /// An id to designate a specific connector (on an EVSE) by connector index number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
}
