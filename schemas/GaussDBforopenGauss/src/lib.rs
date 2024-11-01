#[cfg(feature = "AddInstanceTags")]
pub mod AddInstanceTags;
#[cfg(feature = "AllowDbPrivileges")]
pub mod AllowDbPrivileges;
#[cfg(feature = "AttachEip")]
pub mod AttachEip;
#[cfg(feature = "ConfirmRestoredData")]
pub mod ConfirmRestoredData;
#[cfg(feature = "CopyConfiguration")]
pub mod CopyConfiguration;
#[cfg(feature = "CreateConfigurationTemplate")]
pub mod CreateConfigurationTemplate;
#[cfg(feature = "CreateDatabase")]
pub mod CreateDatabase;
#[cfg(feature = "CreateDatabaseSchemas")]
pub mod CreateDatabaseSchemas;
#[cfg(feature = "CreateDbInstance")]
pub mod CreateDbInstance;
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
#[cfg(feature = "CreateSlowLogDownload")]
pub mod CreateSlowLogDownload;
#[cfg(feature = "DeleteConfiguration")]
pub mod DeleteConfiguration;
#[cfg(feature = "DeleteDatabase")]
pub mod DeleteDatabase;
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
#[cfg(feature = "DownloadBackup")]
pub mod DownloadBackup;
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
#[cfg(feature = "ListDatabases")]
pub mod ListDatabases;
#[cfg(feature = "ListDatabaseSchemas")]
pub mod ListDatabaseSchemas;
#[cfg(feature = "ListDatastores")]
pub mod ListDatastores;
#[cfg(feature = "ListDbBackups")]
pub mod ListDbBackups;
#[cfg(feature = "ListDbFlavors")]
pub mod ListDbFlavors;
#[cfg(feature = "ListDbUsers")]
pub mod ListDbUsers;
#[cfg(feature = "ListEpsQuotas")]
pub mod ListEpsQuotas;
#[cfg(feature = "ListFlavors")]
pub mod ListFlavors;
#[cfg(feature = "ListGaussDbDatastores")]
pub mod ListGaussDbDatastores;
#[cfg(feature = "ListHistoryOperations")]
pub mod ListHistoryOperations;
#[cfg(feature = "ListInstanceDetails")]
pub mod ListInstanceDetails;
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
#[cfg(feature = "ListRestorableInstances")]
pub mod ListRestorableInstances;
#[cfg(feature = "ListRestoreTimes")]
pub mod ListRestoreTimes;
#[cfg(feature = "ListStorageTypes")]
pub mod ListStorageTypes;
#[cfg(feature = "ListSupportKernelPlugins")]
pub mod ListSupportKernelPlugins;
#[cfg(feature = "ListTasks")]
pub mod ListTasks;
#[cfg(feature = "ListTopIoTraffics")]
pub mod ListTopIoTraffics;
#[cfg(feature = "ModifyEpsQuota")]
pub mod ModifyEpsQuota;
#[cfg(feature = "ResetConfiguration")]
pub mod ResetConfiguration;
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
#[cfg(feature = "SearchAutoEnlargePolicy")]
pub mod SearchAutoEnlargePolicy;
#[cfg(feature = "SetBackupPolicy")]
pub mod SetBackupPolicy;
#[cfg(feature = "SetDbUserPwd")]
pub mod SetDbUserPwd;
#[cfg(feature = "SetKernelPluginLicense")]
pub mod SetKernelPluginLicense;
#[cfg(feature = "SetNewBackupPolicy")]
pub mod SetNewBackupPolicy;
#[cfg(feature = "SetRecyclePolicy")]
pub mod SetRecyclePolicy;
#[cfg(feature = "ShowBackupPolicy")]
pub mod ShowBackupPolicy;
#[cfg(feature = "ShowBalanceStatus")]
pub mod ShowBalanceStatus;
#[cfg(feature = "ShowBatchUpgradeCandidateVersions")]
pub mod ShowBatchUpgradeCandidateVersions;
#[cfg(feature = "ShowConfigurationDetail")]
pub mod ShowConfigurationDetail;
#[cfg(feature = "ShowDeploymentForm")]
pub mod ShowDeploymentForm;
#[cfg(feature = "ShowErrorLogSwitchStatus")]
pub mod ShowErrorLogSwitchStatus;
#[cfg(feature = "ShowInstanceConfiguration")]
pub mod ShowInstanceConfiguration;
#[cfg(feature = "ShowInstanceDisk")]
pub mod ShowInstanceDisk;
#[cfg(feature = "ShowInstanceParamGroup")]
pub mod ShowInstanceParamGroup;
#[cfg(feature = "ShowInstanceSnapshot")]
pub mod ShowInstanceSnapshot;
#[cfg(feature = "ShowJobDetail")]
pub mod ShowJobDetail;
#[cfg(feature = "ShowLimitTask")]
pub mod ShowLimitTask;
#[cfg(feature = "ShowProjectQuotas")]
pub mod ShowProjectQuotas;
#[cfg(feature = "ShowRecyclePolicy")]
pub mod ShowRecyclePolicy;
#[cfg(feature = "ShowSlowLogDownload")]
pub mod ShowSlowLogDownload;
#[cfg(feature = "ShowSslCertDownloadLink")]
pub mod ShowSslCertDownloadLink;
#[cfg(feature = "ShowUpgradeCandidateVersions")]
pub mod ShowUpgradeCandidateVersions;
#[cfg(feature = "StartInstance")]
pub mod StartInstance;
#[cfg(feature = "StartMysqlCompatibility")]
pub mod StartMysqlCompatibility;
#[cfg(feature = "StopBackup")]
pub mod StopBackup;
#[cfg(feature = "StopInstance")]
pub mod StopInstance;
#[cfg(feature = "SwitchConfiguration")]
pub mod SwitchConfiguration;
#[cfg(feature = "SwitchShard")]
pub mod SwitchShard;
