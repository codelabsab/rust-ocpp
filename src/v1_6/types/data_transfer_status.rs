/// Status in DataTransferRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum DataTransferStatus {
    /// Message has been accepted and the contained request is accepted.
    #[default]
    Accepted,
    /// Message has been accepted but the contained request is rejected.
    Rejected,
    /// Message could not be interpreted due to unknown messageId string.
    UnknownMessageId,
    /// Message could not be interpreted due to unknown vendorId string.
    UnknownVendorId,
}
