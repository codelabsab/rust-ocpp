use crate::v2_0_1::datatypes::charging_profile_type::ChargingProfileType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::charging_profile_status_enum_type::ChargingProfileStatusEnumType;

/// This contains the field definition of the SetChargingProfileRequest PDU sent by the CSMS to the Charging Station. The CSMS uses this message to send charging profiles to a Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileRequest<'a, const N_SALES_TARIFF_ENTRIES: usize, const N_TARIFF_CONSUMPTION_COSTS: usize, const N_COSTS_PER_TARIFF_CONS_COST: usize, const N_CHARGING_SCHEDULE_PERIODS: usize, const N_CHARGING_SCHEDULES: usize> {
    pub evse_id: i64,
    #[serde(borrow)]
    pub charging_profile: ChargingProfileType<'a,
        N_SALES_TARIFF_ENTRIES, N_TARIFF_CONSUMPTION_COSTS, N_COSTS_PER_TARIFF_CONS_COST,
        N_CHARGING_SCHEDULE_PERIODS, N_CHARGING_SCHEDULES
    >,
}

/// This contains the field definition of the SetChargingProfileResponse PDU sent by the Charging Station to the CSMS in response to SetChargingProfileRequest PDU.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileResponse<'a> {
    pub status: ChargingProfileStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none",borrow)]
    pub status_info: Option<StatusInfoType<'a>>,
}
