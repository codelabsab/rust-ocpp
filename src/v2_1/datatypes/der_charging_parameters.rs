use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::der_control::DERControlEnumType;

/// DERChargingParametersType is used in ChargingNeedsType during an ISO 15118-20 session for AC_BPT_DER
/// to report the inverter settings related to DER control that were agreed between EVSE and EV.
///
/// Fields starting with "ev" contain values from the EV.
/// Other fields contain a value that is supported by both EV and EVSE.
///
/// DERChargingParametersType type is only relevant in case of an ISO 15118-20 AC_BPT_DER/AC_DER charging session.
///
/// NOTE: All these fields have values greater or equal to zero (i.e. are non-negative)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DERChargingParametersType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// DER control functions supported by EV.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType:DERControlFunctions (bitmap)
    pub ev_supported_der_control: Vec<DERControlEnumType>,
}
