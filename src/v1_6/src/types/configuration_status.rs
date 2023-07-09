#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ConfigurationStatus {
    /// # From OCPP Specification
    /// Configuration key is supported and setting has been changed.
    #[default]
    Accepted,

    /// # From OCPP Specification
    /// Configuration key is supported, but setting could not be changed.
    Rejected,

    /// # From OCPP Specification
    /// Configuration key is supported and setting has been changed, but change will be available after reboot (Charge Point will not reboot itself)
    RebootRequired,

    /// # From OCPP Specification
    /// Configuration key is not supported.
    NotSupported,
}
