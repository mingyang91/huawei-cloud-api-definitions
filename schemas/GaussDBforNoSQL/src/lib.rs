#[cfg(feature = "ApplyConfiguration")]
pub mod ApplyConfiguration;
#[cfg(feature = "BatchTagAction")]
pub mod BatchTagAction;
#[cfg(feature = "CheckDisasterRecoveryOperation")]
pub mod CheckDisasterRecoveryOperation;
#[cfg(feature = "CheckWeekPassword")]
pub mod CheckWeekPassword;
#[cfg(feature = "CompareConfiguration")]
pub mod CompareConfiguration;
#[cfg(feature = "CopyConfiguration")]
pub mod CopyConfiguration;
#[cfg(feature = "CreateBack")]
pub mod CreateBack;
#[cfg(feature = "CreateColdVolume")]
pub mod CreateColdVolume;
#[cfg(feature = "CreateConfiguration")]
pub mod CreateConfiguration;
#[cfg(feature = "CreateDbUser")]
pub mod CreateDbUser;
#[cfg(feature = "CreateDisasterRecovery")]
pub mod CreateDisasterRecovery;
#[cfg(feature = "CreateInstance")]
pub mod CreateInstance;
#[cfg(feature = "DeleteBackup")]
pub mod DeleteBackup;
#[cfg(feature = "DeleteConfiguration")]
pub mod DeleteConfiguration;
#[cfg(feature = "DeleteDbUser")]
pub mod DeleteDbUser;
#[cfg(feature = "DeleteDisasterRecovery")]
pub mod DeleteDisasterRecovery;
#[cfg(feature = "DeleteEnlargeFailNode")]
pub mod DeleteEnlargeFailNode;
#[cfg(feature = "DeleteInstance")]
pub mod DeleteInstance;
#[cfg(feature = "DeleteInstancesSession")]
pub mod DeleteInstancesSession;
#[cfg(feature = "DeleteLtsConfigs")]
pub mod DeleteLtsConfigs;
#[cfg(feature = "ExpandInstanceNode")]
pub mod ExpandInstanceNode;
#[cfg(feature = "ListApiVersion")]
pub mod ListApiVersion;
#[cfg(feature = "ListAvailableFlavorInfos")]
pub mod ListAvailableFlavorInfos;
#[cfg(feature = "ListCassandraSlowLogs")]
pub mod ListCassandraSlowLogs;
#[cfg(feature = "ListConfigurationDatastores")]
pub mod ListConfigurationDatastores;
#[cfg(feature = "ListConfigurations")]
pub mod ListConfigurations;
#[cfg(feature = "ListConfigurationTemplates")]
pub mod ListConfigurationTemplates;
#[cfg(feature = "ListDatastores")]
pub mod ListDatastores;
#[cfg(feature = "ListDbUsers")]
pub mod ListDbUsers;
#[cfg(feature = "ListDedicatedResources")]
pub mod ListDedicatedResources;
#[cfg(feature = "ListEpsQuotas")]
pub mod ListEpsQuotas;
#[cfg(feature = "ListFlavorInfos")]
pub mod ListFlavorInfos;
#[cfg(feature = "ListFlavors")]
pub mod ListFlavors;
#[cfg(feature = "ListInfluxdbSlowLogs")]
pub mod ListInfluxdbSlowLogs;
#[cfg(feature = "ListInstanceDatabases")]
pub mod ListInstanceDatabases;
#[cfg(feature = "ListInstances")]
pub mod ListInstances;
#[cfg(feature = "ListInstancesByResourceTags")]
pub mod ListInstancesByResourceTags;
#[cfg(feature = "ListInstancesByTags")]
pub mod ListInstancesByTags;
#[cfg(feature = "ListInstancesSession")]
pub mod ListInstancesSession;
#[cfg(feature = "ListInstancesSessionStatistics")]
pub mod ListInstancesSessionStatistics;
#[cfg(feature = "ListInstanceTags")]
pub mod ListInstanceTags;
#[cfg(feature = "ListJobs")]
pub mod ListJobs;
#[cfg(feature = "ListLtsConfigs")]
pub mod ListLtsConfigs;
#[cfg(feature = "ListMongodbErrorLogs")]
pub mod ListMongodbErrorLogs;
#[cfg(feature = "ListMongodbSlowLogs")]
pub mod ListMongodbSlowLogs;
#[cfg(feature = "ListProjectTags")]
pub mod ListProjectTags;
#[cfg(feature = "ListRecycleInstances")]
pub mod ListRecycleInstances;
#[cfg(feature = "ListRedisSlowLogs")]
pub mod ListRedisSlowLogs;
#[cfg(feature = "ListRestoreDatabases")]
pub mod ListRestoreDatabases;
#[cfg(feature = "ListRestoreTables")]
pub mod ListRestoreTables;
#[cfg(feature = "ListRestoreTime")]
pub mod ListRestoreTime;
#[cfg(feature = "ListSlowLogs")]
pub mod ListSlowLogs;
#[cfg(feature = "ModifyDbUserPrivilege")]
pub mod ModifyDbUserPrivilege;
#[cfg(feature = "ModifyEpsQuotas")]
pub mod ModifyEpsQuotas;
#[cfg(feature = "ModifyPort")]
pub mod ModifyPort;
#[cfg(feature = "ModifyPublicIp")]
pub mod ModifyPublicIp;
#[cfg(feature = "ModifyVolume")]
pub mod ModifyVolume;
#[cfg(feature = "OfflineNodes")]
pub mod OfflineNodes;
#[cfg(feature = "PauseResumeDataSynchronization")]
pub mod PauseResumeDataSynchronization;
#[cfg(feature = "ResetDbUserPassword")]
pub mod ResetDbUserPassword;
#[cfg(feature = "ResetParamGroupTemplate")]
pub mod ResetParamGroupTemplate;
#[cfg(feature = "ResetPassword")]
pub mod ResetPassword;
#[cfg(feature = "ResizeColdVolume")]
pub mod ResizeColdVolume;
#[cfg(feature = "ResizeInstance")]
pub mod ResizeInstance;
#[cfg(feature = "ResizeInstanceVolume")]
pub mod ResizeInstanceVolume;
#[cfg(feature = "RestartInstance")]
pub mod RestartInstance;
#[cfg(feature = "RestoreExistingInstance")]
pub mod RestoreExistingInstance;
#[cfg(feature = "SaveLtsConfigs")]
pub mod SaveLtsConfigs;
#[cfg(feature = "SetAutoEnlargePolicy")]
pub mod SetAutoEnlargePolicy;
#[cfg(feature = "SetBackupPolicy")]
pub mod SetBackupPolicy;
#[cfg(feature = "SetRecyclePolicy")]
pub mod SetRecyclePolicy;
#[cfg(feature = "ShowAllInstancesBackups")]
pub mod ShowAllInstancesBackups;
#[cfg(feature = "ShowAllInstancesBackupsNew")]
pub mod ShowAllInstancesBackupsNew;
#[cfg(feature = "ShowApiVersion")]
pub mod ShowApiVersion;
#[cfg(feature = "ShowApplicableInstances")]
pub mod ShowApplicableInstances;
#[cfg(feature = "ShowApplyHistory")]
pub mod ShowApplyHistory;
#[cfg(feature = "ShowAutoEnlargePolicy")]
pub mod ShowAutoEnlargePolicy;
#[cfg(feature = "ShowBackupPolicy")]
pub mod ShowBackupPolicy;
#[cfg(feature = "ShowConfigurationDetail")]
pub mod ShowConfigurationDetail;
#[cfg(feature = "ShowElbIpGroup")]
pub mod ShowElbIpGroup;
#[cfg(feature = "ShowErrorLog")]
pub mod ShowErrorLog;
#[cfg(feature = "ShowHighRiskCommands")]
pub mod ShowHighRiskCommands;
#[cfg(feature = "ShowInstanceBiactiveRegions")]
pub mod ShowInstanceBiactiveRegions;
#[cfg(feature = "ShowInstanceConfiguration")]
pub mod ShowInstanceConfiguration;
#[cfg(feature = "ShowInstanceRole")]
pub mod ShowInstanceRole;
#[cfg(feature = "ShowIpNumRequirement")]
pub mod ShowIpNumRequirement;
#[cfg(feature = "ShowModifyHistory")]
pub mod ShowModifyHistory;
#[cfg(feature = "ShowPasswordlessConfig")]
pub mod ShowPasswordlessConfig;
#[cfg(feature = "ShowPauseResumeStutus")]
pub mod ShowPauseResumeStutus;
#[cfg(feature = "ShowQuotas")]
pub mod ShowQuotas;
#[cfg(feature = "ShowRecyclePolicy")]
pub mod ShowRecyclePolicy;
#[cfg(feature = "ShowRedisBigKeys")]
pub mod ShowRedisBigKeys;
#[cfg(feature = "ShowRestorableList")]
pub mod ShowRestorableList;
#[cfg(feature = "ShowSlowLogDesensitization")]
pub mod ShowSlowLogDesensitization;
#[cfg(feature = "ShrinkInstanceNode")]
pub mod ShrinkInstanceNode;
#[cfg(feature = "SwitchIpGroup")]
pub mod SwitchIpGroup;
#[cfg(feature = "SwitchSlowlogDesensitization")]
pub mod SwitchSlowlogDesensitization;
#[cfg(feature = "SwitchSsl")]
pub mod SwitchSsl;
#[cfg(feature = "SwitchToMaster")]
pub mod SwitchToMaster;
#[cfg(feature = "SwitchToSlave")]
pub mod SwitchToSlave;
