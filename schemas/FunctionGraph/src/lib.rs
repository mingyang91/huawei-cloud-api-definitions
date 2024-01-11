#[cfg(feature = "AsyncInvokeFunction")]
pub mod AsyncInvokeFunction;
#[cfg(feature = "BatchDeleteFunctionTriggers")]
pub mod BatchDeleteFunctionTriggers;
#[cfg(feature = "BatchDeleteWorkflows")]
pub mod BatchDeleteWorkflows;
#[cfg(feature = "CancelAsyncInvocation")]
pub mod CancelAsyncInvocation;
#[cfg(feature = "CreateCallbackWorkflow")]
pub mod CreateCallbackWorkflow;
#[cfg(feature = "CreateDependencyVersion")]
pub mod CreateDependencyVersion;
#[cfg(feature = "CreateEvent")]
pub mod CreateEvent;
#[cfg(feature = "CreateFunction")]
pub mod CreateFunction;
#[cfg(feature = "CreateFunctionApp")]
pub mod CreateFunctionApp;
#[cfg(feature = "CreateFunctionTrigger")]
pub mod CreateFunctionTrigger;
#[cfg(feature = "CreateFunctionVersion")]
pub mod CreateFunctionVersion;
#[cfg(feature = "CreateTags")]
pub mod CreateTags;
#[cfg(feature = "CreateVersionAlias")]
pub mod CreateVersionAlias;
#[cfg(feature = "CreateVpcEndpoint")]
pub mod CreateVpcEndpoint;
#[cfg(feature = "CreateWorkflow")]
pub mod CreateWorkflow;
#[cfg(feature = "DeleteDependencyVersion")]
pub mod DeleteDependencyVersion;
#[cfg(feature = "DeleteEvent")]
pub mod DeleteEvent;
#[cfg(feature = "DeleteFunction")]
pub mod DeleteFunction;
#[cfg(feature = "DeleteFunctionApp")]
pub mod DeleteFunctionApp;
#[cfg(feature = "DeleteFunctionAsyncInvokeConfig")]
pub mod DeleteFunctionAsyncInvokeConfig;
#[cfg(feature = "DeleteFunctionTrigger")]
pub mod DeleteFunctionTrigger;
#[cfg(feature = "DeleteTags")]
pub mod DeleteTags;
#[cfg(feature = "DeleteVersionAlias")]
pub mod DeleteVersionAlias;
#[cfg(feature = "DeleteVpcEndpoint")]
pub mod DeleteVpcEndpoint;
#[cfg(feature = "EnableAsyncStatusLog")]
pub mod EnableAsyncStatusLog;
#[cfg(feature = "EnableLtsLogs")]
pub mod EnableLtsLogs;
#[cfg(feature = "ExportFunction")]
pub mod ExportFunction;
#[cfg(feature = "ImportFunction")]
pub mod ImportFunction;
#[cfg(feature = "InvokeFunction")]
pub mod InvokeFunction;
#[cfg(feature = "ListActiveAsyncInvocations")]
pub mod ListActiveAsyncInvocations;
#[cfg(feature = "ListAppTemplates")]
pub mod ListAppTemplates;
#[cfg(feature = "ListAsyncInvocations")]
pub mod ListAsyncInvocations;
#[cfg(feature = "ListBridgeFunctions")]
pub mod ListBridgeFunctions;
#[cfg(feature = "ListBridgeVersions")]
pub mod ListBridgeVersions;
#[cfg(feature = "ListDependencies")]
pub mod ListDependencies;
#[cfg(feature = "ListDependencyVersion")]
pub mod ListDependencyVersion;
#[cfg(feature = "ListEvents")]
pub mod ListEvents;
#[cfg(feature = "ListFunctionApplications")]
pub mod ListFunctionApplications;
#[cfg(feature = "ListFunctionAsMetric")]
pub mod ListFunctionAsMetric;
#[cfg(feature = "ListFunctionAsyncInvokeConfig")]
pub mod ListFunctionAsyncInvokeConfig;
#[cfg(feature = "ListFunctionReservedInstances")]
pub mod ListFunctionReservedInstances;
#[cfg(feature = "ListFunctions")]
pub mod ListFunctions;
#[cfg(feature = "ListFunctionStatistics")]
pub mod ListFunctionStatistics;
#[cfg(feature = "ListFunctionTags")]
pub mod ListFunctionTags;
#[cfg(feature = "ListFunctionTemplate")]
pub mod ListFunctionTemplate;
#[cfg(feature = "ListFunctionTriggers")]
pub mod ListFunctionTriggers;
#[cfg(feature = "ListFunctionVersions")]
pub mod ListFunctionVersions;
#[cfg(feature = "ListQuotas")]
pub mod ListQuotas;
#[cfg(feature = "ListReservedInstanceConfigs")]
pub mod ListReservedInstanceConfigs;
#[cfg(feature = "ListStatistics")]
pub mod ListStatistics;
#[cfg(feature = "ListVersionAliases")]
pub mod ListVersionAliases;
#[cfg(feature = "ListWorkflow")]
pub mod ListWorkflow;
#[cfg(feature = "ListWorkflowExecutions")]
pub mod ListWorkflowExecutions;
#[cfg(feature = "RetryWorkFlow")]
pub mod RetryWorkFlow;
#[cfg(feature = "ShowAppTemplate")]
pub mod ShowAppTemplate;
#[cfg(feature = "ShowDependencyVersion")]
pub mod ShowDependencyVersion;
#[cfg(feature = "ShowEvent")]
pub mod ShowEvent;
#[cfg(feature = "ShowFuncReservedInstanceMetrics")]
pub mod ShowFuncReservedInstanceMetrics;
#[cfg(feature = "ShowFuncSnapshotState")]
pub mod ShowFuncSnapshotState;
#[cfg(feature = "ShowFunctionApp")]
pub mod ShowFunctionApp;
#[cfg(feature = "ShowFunctionAsyncInvokeConfig")]
pub mod ShowFunctionAsyncInvokeConfig;
#[cfg(feature = "ShowFunctionCode")]
pub mod ShowFunctionCode;
#[cfg(feature = "ShowFunctionConfig")]
pub mod ShowFunctionConfig;
#[cfg(feature = "ShowFunctionMetrics")]
pub mod ShowFunctionMetrics;
#[cfg(feature = "ShowFunctionTemplate")]
pub mod ShowFunctionTemplate;
#[cfg(feature = "ShowFunctionTrigger")]
pub mod ShowFunctionTrigger;
#[cfg(feature = "ShowLtsLogDetails")]
pub mod ShowLtsLogDetails;
#[cfg(feature = "ShowProjectAsyncStatusLogInfo")]
pub mod ShowProjectAsyncStatusLogInfo;
#[cfg(feature = "ShowProjectTagsList")]
pub mod ShowProjectTagsList;
#[cfg(feature = "ShowResInstanceInfo")]
pub mod ShowResInstanceInfo;
#[cfg(feature = "ShowTenantMetric")]
pub mod ShowTenantMetric;
#[cfg(feature = "ShowTracing")]
pub mod ShowTracing;
#[cfg(feature = "ShowVersionAlias")]
pub mod ShowVersionAlias;
#[cfg(feature = "ShowWorkFlow")]
pub mod ShowWorkFlow;
#[cfg(feature = "ShowWorkflowExecution")]
pub mod ShowWorkflowExecution;
#[cfg(feature = "ShowWorkflowExecutionForPage")]
pub mod ShowWorkflowExecutionForPage;
#[cfg(feature = "ShowWorkFlowMetric")]
pub mod ShowWorkFlowMetric;
#[cfg(feature = "StartSyncWorkflowExecution")]
pub mod StartSyncWorkflowExecution;
#[cfg(feature = "StartWorkflowExecution")]
pub mod StartWorkflowExecution;
#[cfg(feature = "StopWorkFlow")]
pub mod StopWorkFlow;
#[cfg(feature = "UpdateEvent")]
pub mod UpdateEvent;
#[cfg(feature = "UpdateFuncSnapshot")]
pub mod UpdateFuncSnapshot;
#[cfg(feature = "UpdateFunctionAsyncInvokeConfig")]
pub mod UpdateFunctionAsyncInvokeConfig;
#[cfg(feature = "UpdateFunctionCode")]
pub mod UpdateFunctionCode;
#[cfg(feature = "UpdateFunctionCollectState")]
pub mod UpdateFunctionCollectState;
#[cfg(feature = "UpdateFunctionConfig")]
pub mod UpdateFunctionConfig;
#[cfg(feature = "UpdateFunctionMaxInstanceConfig")]
pub mod UpdateFunctionMaxInstanceConfig;
#[cfg(feature = "UpdateFunctionReservedInstancesCount")]
pub mod UpdateFunctionReservedInstancesCount;
#[cfg(feature = "UpdateTracing")]
pub mod UpdateTracing;
#[cfg(feature = "UpdateTrigger")]
pub mod UpdateTrigger;
#[cfg(feature = "UpdateVersionAlias")]
pub mod UpdateVersionAlias;
#[cfg(feature = "UpdateWorkFlow")]
pub mod UpdateWorkFlow;
