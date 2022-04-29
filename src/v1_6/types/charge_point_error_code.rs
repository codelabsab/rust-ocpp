/// Charge Point status reported in StatusNotification.req.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ChargePointErrorCode {
    /// Failure to lock or unlock connector.
    ConnectorLockFailure,
    /// Communication failure with the vehicle, might be Mode 3 or other communication protocol problem. This is not a real error in the sense that the Charge Point doesn’t need to go to the faulted state. Instead, it should go to the SuspendedEVSE state.
    EVCommunicationError,
    /// Ground fault circuit interrupter has been activated.
    GroundFailure,
    /// Temperature inside Charge Point is too high.
    HighTemperature,
    /// Error in internal hard- or software component.
    InternalError,
    /// The authorization information received from the Central System is in conflict with the LocalAuthorizationList.
    LocalListConflict,
    /// No error to report.
    NoError,
    /// Other type of error. More information in vendorErrorCode.
    OtherError,
    /// Over current protection device has tripped.
    OverCurrentFailure,
    /// Voltage has risen above an acceptable level.
    OverVoltage,
    /// Failure to read electrical/energy/power meter.
    PowerMeterFailure,
    /// Failure to control power switch.
    PowerSwitchFailure,
    /// Failure with idTag reader.
    ReaderFailure,
    /// Unable to perform a reset.
    ResetFailure,
    /// Voltage has dropped below an acceptable level.
    UnderVoltage,
    /// Wireless communication device reports a weak signal.
    WeakSignal,
}
