//! GetCompositeSchedule
use crate::v2_0_1::datatypes::composite_schedule_type::CompositeScheduleType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::charging_rate_unit_enum_type::ChargingRateUnitEnumType;
use crate::v2_0_1::enumerations::generic_status_enum_type::GenericStatusEnumType;

/// GetCompositeScheduleRequest, sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleRequest {
    /// Length of the requested schedule in seconds
    pub duration: i64,
    /// Can be used to force a power or current profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_rate_unit: Option<ChargingRateUnitEnumType>,
    /// The ID of the EVSE for which the schedule isrequested. When evseid=0, the Charging Station willcalculate the expected consumption for the gridconnection.
    pub evse_id: i64,
}

/// GetCompositeScheduleResponse, sent by the Charging Station to the CSMS in response to a GetCompositeScheduleRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleResponse<'a, const N_CHARGING_SCHEDULE_PERIODS: usize> {
    /// The Charging Station will indicate if it was ableto process the request
    pub status: GenericStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// This field contains the calculated compositeschedule. It may only be omitted when this messagecontains status Rejected
    pub schedule: Option<CompositeScheduleType<N_CHARGING_SCHEDULE_PERIODS>>,
    /// Detailed status information
    #[serde(skip_serializing_if = "Option::is_none",borrow)]
    pub status_info: Option<StatusInfoType<'a>>,
}
