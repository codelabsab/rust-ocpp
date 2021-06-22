/// Reference key to a component-variable.
/// VariableType is used by: Common:ComponentVariableType , GetVariablesRequest.GetVariableDataType , GetVariablesResponse.GetVariableResultType , NotifyMonitoringReportRequest.MonitoringDataType , NotifyReportRequest.ReportDataType , SetVariableMonitoringRequest.SetMonitoringDataType , SetVariableMonitoringResponse.SetMonitoringResultType , SetVariablesRequest.SetVariableDataType , SetVariablesResponse.SetVariableResultType , NotifyEventRequest.EventDataType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VariableType {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}
