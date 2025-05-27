use serde::{Deserialize, Serialize};

/// Reason the message was triggered.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum TriggerReasonEnumType {
    /// Transaction has been authorized.
    #[default]
    #[serde(rename = "Authorized")]
    Authorized,

    /// Cable has been plugged in.
    #[serde(rename = "CablePluggedIn")]
    CablePluggedIn,

    /// Charging rate has changed.
    #[serde(rename = "ChargingRateChanged")]
    ChargingRateChanged,

    /// Charging state has changed.
    #[serde(rename = "ChargingStateChanged")]
    ChargingStateChanged,

    /// Transaction has been deauthorized.
    #[serde(rename = "Deauthorized")]
    Deauthorized,

    /// Energy limit has been reached.
    #[serde(rename = "EnergyLimitReached")]
    EnergyLimitReached,

    /// Communication with the EV has been lost.
    #[serde(rename = "EVCommunicationLost")]
    EVCommunicationLost,

    /// EV connection timeout.
    #[serde(rename = "EVConnectTimeout")]
    EVConnectTimeout,

    /// Clock-aligned meter value.
    #[serde(rename = "MeterValueClock")]
    MeterValueClock,

    /// Periodic meter value.
    #[serde(rename = "MeterValuePeriodic")]
    MeterValuePeriodic,

    /// Time limit has been reached.
    #[serde(rename = "TimeLimitReached")]
    TimeLimitReached,

    /// Triggered by a trigger message.
    #[serde(rename = "Trigger")]
    Trigger,

    /// Triggered by an unlock command.
    #[serde(rename = "UnlockCommand")]
    UnlockCommand,

    /// Stop has been authorized.
    #[serde(rename = "StopAuthorized")]
    StopAuthorized,

    /// EV has departed.
    #[serde(rename = "EVDeparted")]
    EVDeparted,

    /// EV has been detected.
    #[serde(rename = "EVDetected")]
    EVDetected,

    /// Remote stop command received.
    #[serde(rename = "RemoteStop")]
    RemoteStop,

    /// Remote start command received.
    #[serde(rename = "RemoteStart")]
    RemoteStart,

    /// Abnormal condition detected.
    #[serde(rename = "AbnormalCondition")]
    AbnormalCondition,

    /// Signed data has been received.
    #[serde(rename = "SignedDataReceived")]
    SignedDataReceived,

    /// Reset command received.
    #[serde(rename = "ResetCommand")]
    ResetCommand,
}
