use crate::v2_1::datatypes::{custom_data::CustomDataType, rational_number::RationalNumberType};
use serde::{Deserialize, Serialize};

/// Part of ISO 15118-20 price schedule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalSelectedServicesType {
    /// Service fee
    pub service_fee: RationalNumberType,

    /// Human-readable string to identify this service.
    pub service_name: String,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
