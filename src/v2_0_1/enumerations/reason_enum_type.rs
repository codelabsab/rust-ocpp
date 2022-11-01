#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ReasonEnumType {
    DeAuthorized,
    EmergencyStop,
    EnergyLimitReached,
    EVDisconnected,
    GroundFault,
    ImmediateReset,
    Local,
    LocalOutOfCredit,
    MasterPass,
    #[default]
    Other,
    OvercurrentFault,
    PowerLoss,
    PowerQuality,
    Reboot,
    Remote,
    SOCLimitReached,
    StoppedByEV,
    TimeLimitReached,
    Timeout,
}
