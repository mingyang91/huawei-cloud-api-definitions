#[cfg(feature = "AddInstanceTags")]
pub mod AddInstanceTags;
#[cfg(feature = "AllowDbPrivileges")]
pub mod AllowDbPrivileges;
#[cfg(feature = "AllowDbRolePrivileges")]
pub mod AllowDbRolePrivileges;
#[cfg(feature = "AttachEip")]
pub mod AttachEip;
#[cfg(feature = "BatchShowUpgradeCandidateVersions")]
pub mod BatchShowUpgradeCandidateVersions;
#[cfg(feature = "CancelScheduleTask")]
pub mod CancelScheduleTask;
#[cfg(feature = "ConfirmRestoredData")]
pub mod ConfirmRestoredData;
#[cfg(feature = "CopyConfiguration")]
pub mod CopyConfiguration;
#[cfg(feature = "CreateConfigurationTemplate")]
pub mod CreateConfigurationTemplate;
#[cfg(feature = "CreateCrossCloudConstructDisaster")]
pub mod CreateCrossCloudConstructDisaster;
#[cfg(feature = "CreateDatabase")]
pub mod CreateDatabase;
#[cfg(feature = "CreateDatabaseInstance")]
pub mod CreateDatabaseInstance;
#[cfg(feature = "CreateDatabaseSchemas")]
pub mod CreateDatabaseSchemas;
#[cfg(feature = "CreateDbInstance")]
pub mod CreateDbInstance;
#[cfg(feature = "CreateDbRole")]
pub mod CreateDbRole;
#[cfg(feature = "CreateDbUser")]
pub mod CreateDbUser;
#[cfg(feature = "CreateGaussDbInstance")]
pub mod CreateGaussDbInstance;
#[cfg(feature = "CreateInstance")]
pub mod CreateInstance;
#[cfg(feature = "CreateLimitTask")]
pub mod CreateLimitTask;
#[cfg(feature = "CreateManualBackup")]
pub mod CreateManualBackup;
#[cfg(feature = "CreateRestoreInstance")]
pub mod CreateRestoreInstance;
#[cfg(feature = "CreateScheduleTask")]
pub mod CreateScheduleTask;
#[cfg(feature = "CreateSlowLogDownload")]
pub mod CreateSlowLogDownload;
#[cfg(feature = "DeleteConfiguration")]
pub mod DeleteConfiguration;
#[cfg(feature = "DeleteDatabase")]
pub mod DeleteDatabase;
#[cfg(feature = "DeleteDatabaseSchema")]
pub mod DeleteDatabaseSchema;
#[cfg(feature = "DeleteInstance")]
pub mod DeleteInstance;
#[cfg(feature = "DeleteInstanceTag")]
pub mod DeleteInstanceTag;
#[cfg(feature = "DeleteJob")]
pub mod DeleteJob;
#[cfg(feature = "DeleteLimitTask")]
pub mod DeleteLimitTask;
#[cfg(feature = "DeleteManualBackup")]
pub mod DeleteManualBackup;
#[cfg(feature = "DeleteScheduleTask")]
pub mod DeleteScheduleTask;
#[cfg(feature = "DownloadBackup")]
pub mod DownloadBackup;
#[cfg(feature = "ExecuteCrossCloudDisasterDataCacheEnd")]
pub mod ExecuteCrossCloudDisasterDataCacheEnd;
#[cfg(feature = "ExecuteCrossCloudDisasterDataCacheStart")]
pub mod ExecuteCrossCloudDisasterDataCacheStart;
#[cfg(feature = "ExecuteCrossCloudDisasterEndSimulation")]
pub mod ExecuteCrossCloudDisasterEndSimulation;
#[cfg(feature = "ExecuteCrossCloudDisasterRecoveryFailover")]
pub mod ExecuteCrossCloudDisasterRecoveryFailover;
#[cfg(feature = "ExecuteCrossCloudDisasterRestore")]
pub mod ExecuteCrossCloudDisasterRestore;
#[cfg(feature = "ExecuteCrossCloudDisasterStartSimulation")]
pub mod ExecuteCrossCloudDisasterStartSimulation;
#[cfg(feature = "ExecuteCrossCloudDisasterSwitchover")]
pub mod ExecuteCrossCloudDisasterSwitchover;
#[cfg(feature = "ExecuteCrossCloudReleaseDisaster")]
pub mod ExecuteCrossCloudReleaseDisaster;
#[cfg(feature = "InstallKernelPlugin")]
pub mod InstallKernelPlugin;
#[cfg(feature = "ListApplicableInstances")]
pub mod ListApplicableInstances;
#[cfg(feature = "ListAppliedHistories")]
pub mod ListAppliedHistories;
#[cfg(feature = "ListAvailableFlavors")]
pub mod ListAvailableFlavors;
#[cfg(feature = "ListBackups")]
pub mod ListBackups;
#[cfg(feature = "ListBackupsDetails")]
pub mod ListBackupsDetails;
#[cfg(feature = "ListBindedEips")]
pub mod ListBindedEips;
#[cfg(feature = "ListCnInfosBeforeReduce")]
pub mod ListCnInfosBeforeReduce;
#[cfg(feature = "ListComponentInfos")]
pub mod ListComponentInfos;
#[cfg(feature = "ListConfigurations")]
pub mod ListConfigurations;
#[cfg(feature = "ListConfigurationsDiff")]
pub mod ListConfigurationsDiff;
#[cfg(feature = "ListDatabaseInstances")]
pub mod ListDatabaseInstances;
#[cfg(feature = "ListDatabaseRoles")]
pub mod ListDatabaseRoles;
#[cfg(feature = "ListDatabases")]
pub mod ListDatabases;
#[cfg(feature = "ListDatabaseSchemas")]
pub mod ListDatabaseSchemas;
#[cfg(feature = "ListDatastores")]
pub mod ListDatastores;
#[cfg(feature = "ListDatastoresDetails")]
pub mod ListDatastoresDetails;
#[cfg(feature = "ListDbBackups")]
pub mod ListDbBackups;
#[cfg(feature = "ListDbFlavors")]
pub mod ListDbFlavors;
#[cfg(feature = "ListDbUsers")]
pub mod ListDbUsers;
#[cfg(feature = "ListDisasterRecoveryRecord")]
pub mod ListDisasterRecoveryRecord;
#[cfg(feature = "ListEpsQuotas")]
pub mod ListEpsQuotas;
#[cfg(feature = "ListFeatures")]
pub mod ListFeatures;
#[cfg(feature = "ListFlavors")]
pub mod ListFlavors;
#[cfg(feature = "ListFlavorsDetails")]
pub mod ListFlavorsDetails;
#[cfg(feature = "ListGaussDbDatastores")]
pub mod ListGaussDbDatastores;
#[cfg(feature = "ListHistoryOperations")]
pub mod ListHistoryOperations;
#[cfg(feature = "ListInstanceDetails")]
pub mod ListInstanceDetails;
#[cfg(feature = "ListInstanceEngineDetail")]
pub mod ListInstanceEngineDetail;
#[cfg(feature = "ListInstanceErrorLogs")]
pub mod ListInstanceErrorLogs;
#[cfg(feature = "ListInstances")]
pub mod ListInstances;
#[cfg(feature = "ListInstancesDetails")]
pub mod ListInstancesDetails;
#[cfg(feature = "ListInstanceTags")]
pub mod ListInstanceTags;
#[cfg(feature = "ListKernelPlugins")]
pub mod ListKernelPlugins;
#[cfg(feature = "ListLimitTask")]
pub mod ListLimitTask;
#[cfg(feature = "ListNodeLimitSqlModel")]
pub mod ListNodeLimitSqlModel;
#[cfg(feature = "ListParameterGroupTemplates")]
pub mod ListParameterGroupTemplates;
#[cfg(feature = "ListParamGroupTemplates")]
pub mod ListParamGroupTemplates;
#[cfg(feature = "ListPluginExtensions")]
pub mod ListPluginExtensions;
#[cfg(feature = "ListPredefinedTags")]
pub mod ListPredefinedTags;
#[cfg(feature = "ListProjectTags")]
pub mod ListProjectTags;
#[cfg(feature = "ListRecycleInstances")]
pub mod ListRecycleInstances;
#[cfg(feature = "ListRecycleInstancesDetails")]
pub mod ListRecycleInstancesDetails;
#[cfg(feature = "ListRestorableInstances")]
pub mod ListRestorableInstances;
#[cfg(feature = "ListRestorableInstancesDetails")]
pub mod ListRestorableInstancesDetails;
#[cfg(feature = "ListRestoreTimes")]
pub mod ListRestoreTimes;
#[cfg(feature = "ListScheduleTask")]
pub mod ListScheduleTask;
#[cfg(feature = "ListStorageTypes")]
pub mod ListStorageTypes;
#[cfg(feature = "ListSupportKernelPlugins")]
pub mod ListSupportKernelPlugins;
#[cfg(feature = "ListTasks")]
pub mod ListTasks;
#[cfg(feature = "ModifyEpsQuota")]
pub mod ModifyEpsQuota;
#[cfg(feature = "ResetConfiguration")]
pub mod ResetConfiguration;
#[cfg(feature = "ResetDrConfig")]
pub mod ResetDrConfig;
#[cfg(feature = "ResetPwd")]
pub mod ResetPwd;
#[cfg(feature = "ResizeInstanceFlavor")]
pub mod ResizeInstanceFlavor;
#[cfg(feature = "RestartInstance")]
pub mod RestartInstance;
#[cfg(feature = "RestoreInstance")]
pub mod RestoreInstance;
#[cfg(feature = "ResumePluginExtensions")]
pub mod ResumePluginExtensions;
#[cfg(feature = "RunInstanceAction")]
pub mod RunInstanceAction;
