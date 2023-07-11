/// Allowable values of the optional "location" field of a value element in SampledValue.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum Location {
    /// Measurement inside body of Charge Point (e.g. Temperature)
    Body,
    ///Measurement taken from cable between EV and Charge Point
    Cable,
    ///Measurement taken by EV
    #[serde(rename = "EV")]
    Ev,
    ///Measurement at network (“grid”) inlet connection
    Inlet,
    ///Measurement at a Connector. Default value
    #[default]
    Outlet,
}
