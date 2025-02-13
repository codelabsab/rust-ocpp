use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    component::ComponentType, custom_data::CustomDataType, variable::VariableType,
    variable_monitoring::VariableMonitoringType,
};

/// Class to hold parameters of SetVariableMonitoring request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringDataType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Component for which a variable is monitored.
    pub component: ComponentType,

    /// Required. Variable that is monitored.
    pub variable: VariableType,

    /// Required. The type of this monitor, e.g. a threshold, delta or periodic monitor.
    pub variable_monitoring: Vec<VariableMonitoringType>,
}
