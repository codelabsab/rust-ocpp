/// # From OCPP Specification
/// Result of registration in response to BootNotification.req.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum RegistrationStatus {
    /// # From OCPP Specification
    /// Charge point is accepted by Central System.
    #[default]
    Accepted,
    /// # From OCPP Specification
    /// Central System is not yet ready to accept the Charge Point. Central System may send
    /// messages to retrieve information or prepare the Charge Point.
    Pending,
    /// # From OCPP Specification
    /// Charge point is not accepted by Central System. This may happen when the Charge Point id is
    /// not known by Central System.
    Rejected,
}
