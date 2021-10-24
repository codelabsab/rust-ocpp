use crate::v2_0_1::core::datatypes::composite_schedule_type::CompositeScheduleType;
use crate::v2_0_1::core::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::core::enumerations::charging_rate_unit_enum_type::ChargingRateUnitEnumType;
use crate::v2_0_1::core::enumerations::generic_status_enum_type::GenericStatusEnumType;

/// This contains the field definition of the GetCompositeScheduleRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleRequest {
    pub duration: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_rate_unit: Option<ChargingRateUnitEnumType>,
    pub evse_id: i64,
}

/// This contains the field definition of the GetCompositeScheduleResponse PDU sent by the Charging Station to the CSMS in response to a GetCompositeScheduleRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleResponse {
    pub status: GenericStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<CompositeScheduleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
