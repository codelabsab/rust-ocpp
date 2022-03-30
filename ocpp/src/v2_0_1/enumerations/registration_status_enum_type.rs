#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum RegistrationStatusEnumType {
    Accepted,
    Pending,
    Rejected,
}
