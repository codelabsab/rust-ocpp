/// Allowable values of the optional "measurand" field of a Value element, as used in MeterValuesRequest and StopTransaction.req messages. Default value of "measurand" is always "Energy.Active.Import.Register"
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Measurand {
    ///Instantaneous current flow from EV
    #[serde(rename = "Current.Export")]
    CurrentExport,
    /// Instantaneous current flow to EV
    #[serde(rename = "Current.Import")]
    CurrentImport,
    /// Maximum current offered to EV
    #[serde(rename = "Current.Offered")]
    CurrentOffered,
    /// Numerical value read from the "active electrical energy" (Wh or kWh) register of the (most authoritative) electrical meter measuring energy exported (to the grid).
    #[serde(rename = "Energy.Active.Export.Register")]
    EnergyActiveExportRegister,
    /// Numerical value read from the "active electrical energy" (Wh or kWh) register of the (most authoritative) electrical meter measuring energy imported (from the grid supply).
    #[serde(rename = "Energy.Active.Import.Register")]
    EnergyActiveImportRegister,
    ///  Numerical value read from the "reactive electrical energy" (VARh or kVARh) register of the (most authoritative) electrical meter measuring energy exported (to the grid).
    #[serde(rename = "Energy.Reactive.Export.Register")]
    EnergyReactiveExportRegister,
    /// Numerical value read from the "reactive electrical energy" (VARh or kVARh) register of the (most authoritative) electrical meter measuring energy imported (from the grid supply).
    #[serde(rename = "Energy.Reactive.Import.Register")]
    EnergyReactiveImportRegister,
    /// Absolute amount of "active electrical energy" (Wh or kWh) exported (to the grid) during an associated time "interval", specified by a Metervalues ReadingContext, and applicable interval duration configuration values (in seconds) for "ClockAlignedDataInterval" and "MeterValueSampleInterval".
    #[serde(rename = "Energy.Active.Export.Interval")]
    EnergyActiveExportInterval,
    /// Absolute amount of "active electrical energy" (Wh or kWh) imported (from the grid supply) during an associated time "interval", specified by a Metervalues ReadingContext, and applicable interval duration configuration values (in seconds) for "ClockAlignedDataInterval" and "MeterValueSampleInterval".
    #[serde(rename = "Energy.Active.Import.Interval")]
    EnergyActiveImportInterval,
    /// Absolute amount of "reactive electrical energy" (VARh or kVARh) exported (to the grid) during an associated time "interval", specified by a Metervalues ReadingContext, and applicable interval duration configuration values (in seconds) for "ClockAlignedDataInterval" and "MeterValueSampleInterval".
    #[serde(rename = "Energy.Reactive.Export.Interval")]
    EnergyReactiveExportInterval,
    ///  Absolute amount of "reactive electrical energy" (VARh or kVARh) imported (from the grid supply) during an associated time "interval", specified by a Metervalues ReadingContext, and applicable interval duration configuration values (in seconds) for "ClockAlignedDataInterval" and "MeterValueSampleInterval".
    #[serde(rename = "Energy.Reactive.Import.Interval")]
    EnergyReactiveImportInterval,
    /// Instantaneous reading of powerline frequency. NOTE: OCPP 1.6 does not have a UnitOfMeasure for frequency, the UnitOfMeasure for any SampledValue with measurand: Frequency is Hertz.
    Frequency,
    /// Instantaneous active power exported by EV. (W or kW)
    #[serde(rename = "Power.Active.Export")]
    PowerActiveExport,
    /// Instantaneous active power imported by EV. (W or kW)
    #[serde(rename = "Power.Active.Import")]
    PowerActiveImport,
    /// Instantaneous power factor of total energy flow
    #[serde(rename = "Power.Factor")]
    PowerFactor,
    /// Maximum power offered to EV
    #[serde(rename = "Power.Offered")]
    PowerOffered,
    /// Instantaneous reactive power exported by EV. (var or kvar)
    #[serde(rename = "Power.Reactive.Export")]
    PowerReactiveExport,
    /// Instantaneous reactive power imported by EV. (var or kvar)
    #[serde(rename = "Power.Reactive.Import")]
    PowerReactiveImport,
    /// Fan speed in RPM
    #[serde(rename = "RPM")]
    Rpm,
    /// State of charge of charging vehicle in percentage
    SoC,
    /// Temperature reading inside Charge Point.
    Temperature,
    /// Instantaneous AC RMS supply voltage
    Voltage,
}
