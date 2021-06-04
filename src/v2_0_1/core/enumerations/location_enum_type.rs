#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum LocationEnumType {
    Body,
    Cable,
    EV,
    Inlet,
    Outlet,
}
