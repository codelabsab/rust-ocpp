#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum UploadLogStatusEnumType {
    BadMessage,
    #[default]
    Idle,
    NotSupportedOperation,
    PermissionDenied,
    Uploaded,
    UploadFailure,
    Uploading,
    AcceptedCanceled,
}
