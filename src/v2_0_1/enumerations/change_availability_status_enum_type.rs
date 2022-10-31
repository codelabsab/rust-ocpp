#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum ChangeAvailabilityStatusEnumType {
    #[default]
    Accepted,
    Rejected,
    Scheduled,
}
