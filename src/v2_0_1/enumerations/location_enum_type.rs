#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum LocationEnumType {
    Body,
    Cable,
    EV,
    Inlet,
    Outlet,
}
