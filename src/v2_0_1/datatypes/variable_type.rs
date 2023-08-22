use validator::Validate;
/// Reference key to a component-variable.
/// VariableType is used by: Common:ComponentVariableType , GetVariablesRequest.GetVariableDataType , GetVariablesResponse.GetVariableResultType , NotifyMonitoringReportRequest.MonitoringDataType , NotifyReportRequest.ReportDataType , SetVariableMonitoringRequest.SetMonitoringDataType , SetVariableMonitoringResponse.SetMonitoringResultType , SetVariablesRequest.SetVariableDataType , SetVariablesResponse.SetVariableResultType , NotifyEventRequest.EventDataType
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VariableType<'a> {
    #[validate(length(min = 0, max = 50))]
    pub name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 0, max = 50))]
    pub instance: Option<&'a str>,
}
