use crate::v2_0_1::core::datatypes::meter_value_type::MeterValueType;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesRequest {
    pub evse_id: i64,
    pub meter_value: MeterValueType,
}

/// This contains the field definition of the MeterValuesResponse PDU sent by the CSMS to the Charging Station in response to a MeterValuesRequest PDU. This message is deprecated. This message might be removed in a future version of OCPP. It will be replaced by Device Management Monitoring events.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesResponse {}
