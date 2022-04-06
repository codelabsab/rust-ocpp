/// Type of reset requested by ResetRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ResetType {
    /// Restart (all) the hardware, the Charge Point is not required to gracefully stop ongoing transaction. If possible the Charge Point sends a StopTransaction.req for previously ongoing transactions after having restarted and having been accepted by the Central System via a BootNotification.conf. This is a last resort solution for a not correctly functioning Charge Point, by sending a "hard" reset, (queued) information might get lost.
    Hard,
    /// Stop ongoing transactions gracefully and sending StopTransaction.req for every ongoing transaction. It should then restart the application software (if possible, otherwise restart the processor/controller).
    Soft,
}
