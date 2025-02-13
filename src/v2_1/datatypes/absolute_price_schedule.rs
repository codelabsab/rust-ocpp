use crate::v2_1::datatypes::custom_data::CustomDataType;
use serde::{Deserialize, Serialize};

/// The AbsolutePriceScheduleType is modeled after the same type that is defined in ISO 15118-20,
/// such that if it is supplied by an EMSP as a signed EXI message, the conversion from EXI to JSON
/// (in OCPP) and back to EXI (for ISO 15118-20) does not change the digest and therefore does not
/// invalidate the signature.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbsolutePriceScheduleType {
    /// Starting point of price schedule.
    pub time_anchor: String, // date-time format

    /// Unique ID of price schedule
    #[serde(rename = "priceScheduleID")]
    pub price_schedule_id: i32,

    /// Description of the price schedule.
    #[serde(rename = "priceScheduleDescription")]
    pub price_schedule_description: Option<String>,

    /// Currency according to ISO 4217.
    pub currency: Option<String>,

    /// String that indicates what language is used for the human readable strings in the price schedule.
    /// Based on ISO 639.
    pub language: Option<String>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
