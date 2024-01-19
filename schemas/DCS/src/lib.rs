#[cfg(feature = "BatchCreateOrDeleteTags")]
pub mod BatchCreateOrDeleteTags;
#[cfg(feature = "BatchDeleteInstances")]
pub mod BatchDeleteInstances;
#[cfg(feature = "BatchShowNodesInformation")]
pub mod BatchShowNodesInformation;
#[cfg(feature = "BatchStopMigrationTasks")]
pub mod BatchStopMigrationTasks;
#[cfg(feature = "ChangeMasterStandby")]
pub mod ChangeMasterStandby;
#[cfg(feature = "ChangeMasterStandbyAsync")]
pub mod ChangeMasterStandbyAsync;
#[cfg(feature = "CopyInstance")]
pub mod CopyInstance;
#[cfg(feature = "CreateAclAccount")]
pub mod CreateAclAccount;
#[cfg(feature = "CreateAutoExpireScanTask")]
pub mod CreateAutoExpireScanTask;
#[cfg(feature = "CreateBigkeyScanTask")]
pub mod CreateBigkeyScanTask;
#[cfg(feature = "CreateCustomTemplate")]
pub mod CreateCustomTemplate;
#[cfg(feature = "CreateDiagnosisTask")]
pub mod CreateDiagnosisTask;
#[cfg(feature = "CreateHotkeyScanTask")]
pub mod CreateHotkeyScanTask;
#[cfg(feature = "CreateInstance")]
pub mod CreateInstance;
#[cfg(feature = "CreateMigrationTask")]
pub mod CreateMigrationTask;
#[cfg(feature = "CreateOnlineMigrationTask")]
pub mod CreateOnlineMigrationTask;
#[cfg(feature = "CreateRedislog")]
pub mod CreateRedislog;
#[cfg(feature = "CreateRedislogDownloadLink")]
pub mod CreateRedislogDownloadLink;
#[cfg(feature = "CreateResizeOrder")]
pub mod CreateResizeOrder;
#[cfg(feature = "DeleteAclAccount")]
pub mod DeleteAclAccount;
#[cfg(feature = "DeleteBackgroundTask")]
pub mod DeleteBackgroundTask;
#[cfg(feature = "DeleteBackupFile")]
pub mod DeleteBackupFile;
#[cfg(feature = "DeleteBigkeyScanTask")]
pub mod DeleteBigkeyScanTask;
#[cfg(feature = "DeleteCenterTask")]
pub mod DeleteCenterTask;
#[cfg(feature = "DeleteConfigTemplate")]
pub mod DeleteConfigTemplate;
#[cfg(feature = "DeleteDiagnosisTask")]
pub mod DeleteDiagnosisTask;
#[cfg(feature = "DeleteHotkeyScanTask")]
pub mod DeleteHotkeyScanTask;
#[cfg(feature = "DeleteIpFromDomainName")]
pub mod DeleteIpFromDomainName;
#[cfg(feature = "DeleteMigrationTask")]
pub mod DeleteMigrationTask;
#[cfg(feature = "DeleteSingleInstance")]
pub mod DeleteSingleInstance;
#[cfg(feature = "DownloadSslCert")]
pub mod DownloadSslCert;
#[cfg(feature = "ExchangeInstanceIp")]
pub mod ExchangeInstanceIp;
#[cfg(feature = "ExecuteClusterSwitchover")]
pub mod ExecuteClusterSwitchover;
#[cfg(feature = "ExecuteCommandMobilization")]
pub mod ExecuteCommandMobilization;
#[cfg(feature = "ExportExcelJob")]
pub mod ExportExcelJob;
#[cfg(feature = "ExportInstancesTask")]
pub mod ExportInstancesTask;
#[cfg(feature = "ListAclAccounts")]
pub mod ListAclAccounts;
#[cfg(feature = "ListAvailableZones")]
pub mod ListAvailableZones;
#[cfg(feature = "ListBackgroundTask")]
pub mod ListBackgroundTask;
#[cfg(feature = "ListBackupFileLinks")]
pub mod ListBackupFileLinks;
#[cfg(feature = "ListBackupRecords")]
pub mod ListBackupRecords;
#[cfg(feature = "ListBigkeyScanTasks")]
pub mod ListBigkeyScanTasks;
#[cfg(feature = "ListCenterTask")]
pub mod ListCenterTask;
#[cfg(feature = "ListConfigHistories")]
pub mod ListConfigHistories;
#[cfg(feature = "ListConfigTemplates")]
pub mod ListConfigTemplates;
#[cfg(feature = "ListConfigurations")]
pub mod ListConfigurations;
#[cfg(feature = "ListDiagnosisTasks")]
pub mod ListDiagnosisTasks;
#[cfg(feature = "ListFlavors")]
pub mod ListFlavors;
#[cfg(feature = "ListGroupReplicationInfo")]
pub mod ListGroupReplicationInfo;
#[cfg(feature = "ListHotKeyScanTasks")]
pub mod ListHotKeyScanTasks;
#[cfg(feature = "ListInstanceOperations")]
pub mod ListInstanceOperations;
#[cfg(feature = "ListInstances")]
pub mod ListInstances;
#[cfg(feature = "ListMaintenanceWindows")]
pub mod ListMaintenanceWindows;
#[cfg(feature = "ListMigrationTask")]
pub mod ListMigrationTask;
#[cfg(feature = "ListMigrationTaskLogs")]
pub mod ListMigrationTaskLogs;
#[cfg(feature = "ListMonitoredObjects")]
pub mod ListMonitoredObjects;
#[cfg(feature = "ListMonitoredObjectsOfInstance")]
pub mod ListMonitoredObjectsOfInstance;
#[cfg(feature = "ListNumberOfInstancesInDifferentStatus")]
pub mod ListNumberOfInstancesInDifferentStatus;
#[cfg(feature = "ListRedislog")]
pub mod ListRedislog;
#[cfg(feature = "ListRestoreRecords")]
pub mod ListRestoreRecords;
#[cfg(feature = "ListSlowlog")]
pub mod ListSlowlog;
#[cfg(feature = "ListStatisticsOfRunningInstances")]
pub mod ListStatisticsOfRunningInstances;
#[cfg(feature = "ListTagsOfTenant")]
pub mod ListTagsOfTenant;
#[cfg(feature = "LoginWebCli")]
pub mod LoginWebCli;
#[cfg(feature = "LogoffWebCli")]
pub mod LogoffWebCli;
#[cfg(feature = "ResetAclAccountPassWord")]
pub mod ResetAclAccountPassWord;
#[cfg(feature = "ResetPassword")]
pub mod ResetPassword;
#[cfg(feature = "ResizeInstance")]
pub mod ResizeInstance;
#[cfg(feature = "RestartOrFlushInstances")]
pub mod RestartOrFlushInstances;
#[cfg(feature = "RestoreInstance")]
pub mod RestoreInstance;
#[cfg(feature = "ScanExpireKey")]
pub mod ScanExpireKey;
#[cfg(feature = "SetOnlineMigrationTask")]
pub mod SetOnlineMigrationTask;
#[cfg(feature = "ShowBackgroundTaskProgress")]
pub mod ShowBackgroundTaskProgress;
#[cfg(feature = "ShowBigkeyAutoscanConfig")]
pub mod ShowBigkeyAutoscanConfig;
#[cfg(feature = "ShowBigkeyScanTaskDetails")]
pub mod ShowBigkeyScanTaskDetails;
#[cfg(feature = "ShowConfigHistoryDetail")]
pub mod ShowConfigHistoryDetail;
#[cfg(feature = "ShowConfigTemplate")]
pub mod ShowConfigTemplate;
#[cfg(feature = "ShowDiagnosisTaskDetails")]
pub mod ShowDiagnosisTaskDetails;
#[cfg(feature = "ShowExpireAutoScanConfig")]
pub mod ShowExpireAutoScanConfig;
#[cfg(feature = "ShowExpireKeyScanInfo")]
pub mod ShowExpireKeyScanInfo;
#[cfg(feature = "ShowHotkeyAutoscanConfig")]
pub mod ShowHotkeyAutoscanConfig;
#[cfg(feature = "ShowHotkeyTaskDetails")]
pub mod ShowHotkeyTaskDetails;
#[cfg(feature = "ShowInstance")]
pub mod ShowInstance;
#[cfg(feature = "ShowInstanceSslDetail")]
pub mod ShowInstanceSslDetail;
#[cfg(feature = "ShowIpWhitelist")]
pub mod ShowIpWhitelist;
#[cfg(feature = "ShowJobInfo")]
pub mod ShowJobInfo;
#[cfg(feature = "ShowMigrationTask")]
pub mod ShowMigrationTask;
#[cfg(feature = "ShowMigrationTaskStats")]
pub mod ShowMigrationTaskStats;
#[cfg(feature = "ShowNodesInformation")]
pub mod ShowNodesInformation;
#[cfg(feature = "ShowQuotaOfTenant")]
pub mod ShowQuotaOfTenant;
#[cfg(feature = "ShowReplicationStates")]
pub mod ShowReplicationStates;
#[cfg(feature = "ShowTags")]
pub mod ShowTags;
#[cfg(feature = "StartInstanceResizeCheckJob")]
pub mod StartInstanceResizeCheckJob;
#[cfg(feature = "StopMigrationTask")]
pub mod StopMigrationTask;
#[cfg(feature = "StopMigrationTaskSync")]
pub mod StopMigrationTaskSync;
#[cfg(feature = "UpdateAclAccount")]
pub mod UpdateAclAccount;
#[cfg(feature = "UpdateAclAccountPassWord")]
pub mod UpdateAclAccountPassWord;
#[cfg(feature = "UpdateAclAccountRemark")]
pub mod UpdateAclAccountRemark;
#[cfg(feature = "UpdateBigkeyAutoscanConfig")]
pub mod UpdateBigkeyAutoscanConfig;
#[cfg(feature = "UpdateClientIpTransparentTransmission")]
pub mod UpdateClientIpTransparentTransmission;
