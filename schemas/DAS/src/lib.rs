#[cfg(feature = "CancelShareConnections")]
pub mod CancelShareConnections;
#[cfg(feature = "ChangeSqlLimitSwitchStatus")]
pub mod ChangeSqlLimitSwitchStatus;
#[cfg(feature = "ChangeSqlSwitch")]
pub mod ChangeSqlSwitch;
#[cfg(feature = "ChangeTransactionSwitchStatus")]
pub mod ChangeTransactionSwitchStatus;
#[cfg(feature = "CreateHealthReportTask")]
pub mod CreateHealthReportTask;
#[cfg(feature = "CreateShareConnections")]
pub mod CreateShareConnections;
#[cfg(feature = "CreateSpaceAnalysisTask")]
pub mod CreateSpaceAnalysisTask;
#[cfg(feature = "CreateSqlLimitRules")]
pub mod CreateSqlLimitRules;
#[cfg(feature = "CreateTuning")]
pub mod CreateTuning;
#[cfg(feature = "DeleteDbUser")]
pub mod DeleteDbUser;
#[cfg(feature = "DeleteProcess")]
pub mod DeleteProcess;
#[cfg(feature = "DeleteSqlLimitRules")]
pub mod DeleteSqlLimitRules;
#[cfg(feature = "ExportSlowQueryLogs")]
pub mod ExportSlowQueryLogs;
#[cfg(feature = "ExportSlowSqlStatistics")]
pub mod ExportSlowSqlStatistics;
#[cfg(feature = "ExportSlowSqlTemplatesDetails")]
pub mod ExportSlowSqlTemplatesDetails;
#[cfg(feature = "ExportSlowSqlTrendDetails")]
pub mod ExportSlowSqlTrendDetails;
#[cfg(feature = "ExportSqlStatements")]
pub mod ExportSqlStatements;
#[cfg(feature = "ExportTopRiskInstances")]
pub mod ExportTopRiskInstances;
#[cfg(feature = "ExportTopSqlTemplatesDetails")]
pub mod ExportTopSqlTemplatesDetails;
#[cfg(feature = "ExportTopSqlTrendDetails")]
pub mod ExportTopSqlTrendDetails;
#[cfg(feature = "ListApiVersions")]
pub mod ListApiVersions;
#[cfg(feature = "ListCloudDbaInstances")]
pub mod ListCloudDbaInstances;
#[cfg(feature = "ListDbUsers")]
pub mod ListDbUsers;
#[cfg(feature = "ListHealthReportTask")]
pub mod ListHealthReportTask;
#[cfg(feature = "ListInnodbLocks")]
pub mod ListInnodbLocks;
#[cfg(feature = "ListMetadataLocks")]
pub mod ListMetadataLocks;
#[cfg(feature = "ListProcesses")]
pub mod ListProcesses;
#[cfg(feature = "ListSpaceAnalysis")]
pub mod ListSpaceAnalysis;
#[cfg(feature = "ListSqlLimitRules")]
pub mod ListSqlLimitRules;
#[cfg(feature = "ListTransactions")]
pub mod ListTransactions;
#[cfg(feature = "ParseSqlLimitRules")]
pub mod ParseSqlLimitRules;
#[cfg(feature = "RegisterDbUser")]
pub mod RegisterDbUser;
#[cfg(feature = "ShowApiVersion")]
pub mod ShowApiVersion;
#[cfg(feature = "ShowDbUser")]
pub mod ShowDbUser;
#[cfg(feature = "ShowInstanceHealthReport")]
pub mod ShowInstanceHealthReport;
#[cfg(feature = "ShowQuotas")]
pub mod ShowQuotas;
#[cfg(feature = "ShowSqlExecutionPlan")]
pub mod ShowSqlExecutionPlan;
#[cfg(feature = "ShowSqlExplain")]
pub mod ShowSqlExplain;
#[cfg(feature = "ShowSqlLimitJobInfo")]
pub mod ShowSqlLimitJobInfo;
#[cfg(feature = "ShowSqlLimitSwitchStatus")]
pub mod ShowSqlLimitSwitchStatus;
#[cfg(feature = "ShowSqlSwitchStatus")]
pub mod ShowSqlSwitchStatus;
#[cfg(feature = "ShowTransactionSwitchStatus")]
pub mod ShowTransactionSwitchStatus;
#[cfg(feature = "ShowTuning")]
pub mod ShowTuning;
#[cfg(feature = "SynchronizeInstances")]
pub mod SynchronizeInstances;
#[cfg(feature = "UpdateDbUser")]
pub mod UpdateDbUser;
#[cfg(feature = "UpdateSqlLimitRules")]
pub mod UpdateSqlLimitRules;
