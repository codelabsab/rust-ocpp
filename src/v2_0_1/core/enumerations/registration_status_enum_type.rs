#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum RegistrationStatusEnumType {
    Accepted,
    Pending,
    Rejected,
}
