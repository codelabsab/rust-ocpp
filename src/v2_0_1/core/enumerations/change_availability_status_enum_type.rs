#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum ChangeAvailabilityStatusEnumType {
    Accepted,
    Rejected,
    Scheduled,
}
