/*
    3.13.1. Charging profile purposes
    A charging profile consists of a charging schedule, which is basically a list of time intervals with their maximum
    charge power or current, and some values to specify the time period and recurrence of the schedule.
    There are three different types of charging profiles, depending on their purpose:
    • ChargePointMaxProfile
    In load balancing scenarios, the Charge Point has one or more local charging profiles that limit the power or
    current to be shared by all connectors of the Charge Point. The Central System SHALL configure such a profile
    with ChargingProfilePurpose set to “ChargePointMaxProfile”. ChargePointMaxProfile can only be set at Charge
    Point ConnectorId 0.
    • TxDefaultProfile
    Default schedules for new transactions MAY be used to impose charging policies. An example could be a policy
    that prevents charging during the day. For schedules of this purpose, ChargingProfilePurpose SHALL be set to
    TxDefaultProfile.
    If TxDefaultProfile is set to ConnectorId 0, the TxDefaultProfile is applicable to all Connectors.
    If ConnectorId is set >0, it only applies to that specific connector.
    In the event a TxDefaultProfile for connector 0 is installed, and the Central System sends a new profile with ConnectorId
    >0, the TxDefaultProfile SHALL be replaced only for that specific connector.
    • TxProfile
    If a transaction-specific profile with purpose TxProfile is present, it SHALL overrule the default charging profile
    with purpose TxDefaultProfile for the duration of the current transaction only. After the transaction is stopped,
    the profile SHOULD be deleted. If there is no transaction active on the connector specified in a charging profile
    of type TxProfile, then the Charge Point SHALL discard it and return an error status in SetChargingProfile.conf.
    20The final schedule constraints that apply to a transaction are determined by merging the profiles with purposes
    ChargePointMaxProfile with the profile TxProfile or the TxDefaultProfile
*/
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ChargingProfilePurposeType {
    /// Configuration for the maximum power or current available for an entire Charge Point.
    ChargePointMaxProfile,
    /// Default profile *that can be configured in the Charge Point. When a new transaction is started, this profile SHALL be used, unless it was a transaction that was started by a RemoteStartTransaction.req with a ChargeProfile that is accepted by the Charge Point.
    TxDefaultProfile,
    /// Profile with constraints to be imposed by the Charge Point on the current transaction, or on a new transaction when this is started via a RemoteStartTransaction.req with a ChargeProfile. A profile with this purpose SHALL cease to be valid when the transaction terminates.
    TxProfile,
}

// TODO: Not sure this is how to set Default for Enum. Got from https://users.rust-lang.org/t/derive-default-for-enum-non-only-struct/44046/7
impl Default for ChargingProfilePurposeType {
    fn default() -> Self {
        ChargingProfilePurposeType::TxDefaultProfile
    }
}
