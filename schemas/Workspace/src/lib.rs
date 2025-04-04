#[cfg(feature = "AddDesktopPoolVolumes")]
pub mod AddDesktopPoolVolumes;
#[cfg(feature = "AddDesktopSubResources")]
pub mod AddDesktopSubResources;
#[cfg(feature = "AddDesktopVolumes")]
pub mod AddDesktopVolumes;
#[cfg(feature = "AddMetricNotifyRule")]
pub mod AddMetricNotifyRule;
#[cfg(feature = "AddOu")]
pub mod AddOu;
#[cfg(feature = "AddRestrictedRule")]
pub mod AddRestrictedRule;
#[cfg(feature = "AddSite")]
pub mod AddSite;
#[cfg(feature = "AddVolumes")]
pub mod AddVolumes;
#[cfg(feature = "ApplyDesktopsInternet")]
pub mod ApplyDesktopsInternet;
#[cfg(feature = "ApplyInternet")]
pub mod ApplyInternet;
#[cfg(feature = "ApplySubnetBandwidth")]
pub mod ApplySubnetBandwidth;
#[cfg(feature = "ApplyWorkspace")]
pub mod ApplyWorkspace;
#[cfg(feature = "AssociateDesktopsEip")]
pub mod AssociateDesktopsEip;
#[cfg(feature = "AttachInstances")]
pub mod AttachInstances;
#[cfg(feature = "BatchAddDesktopsTags")]
pub mod BatchAddDesktopsTags;
#[cfg(feature = "BatchAssociateInstances")]
pub mod BatchAssociateInstances;
#[cfg(feature = "BatchAttachInstances")]
pub mod BatchAttachInstances;
#[cfg(feature = "BatchChangeDesktopNetwork")]
pub mod BatchChangeDesktopNetwork;
#[cfg(feature = "BatchChangeTags")]
pub mod BatchChangeTags;
#[cfg(feature = "BatchCreateDesktopSnapshot")]
pub mod BatchCreateDesktopSnapshot;
#[cfg(feature = "BatchCreateUsers")]
pub mod BatchCreateUsers;
#[cfg(feature = "BatchDeleteAccessPolicies")]
pub mod BatchDeleteAccessPolicies;
#[cfg(feature = "BatchDeleteAppRules")]
pub mod BatchDeleteAppRules;
#[cfg(feature = "BatchDeleteApps")]
pub mod BatchDeleteApps;
#[cfg(feature = "BatchDeleteDesktopNamePolicy")]
pub mod BatchDeleteDesktopNamePolicy;
#[cfg(feature = "BatchDeleteDesktops")]
pub mod BatchDeleteDesktops;
#[cfg(feature = "BatchDeleteDesktopSnapshot")]
pub mod BatchDeleteDesktopSnapshot;
#[cfg(feature = "BatchDeleteDesktopsTags")]
pub mod BatchDeleteDesktopsTags;
#[cfg(feature = "BatchDeleteJobs")]
pub mod BatchDeleteJobs;
#[cfg(feature = "BatchDeleteOtpDevices")]
pub mod BatchDeleteOtpDevices;
#[cfg(feature = "BatchDeleteScheduledTasks")]
pub mod BatchDeleteScheduledTasks;
#[cfg(feature = "BatchDeleteScreenRecords")]
pub mod BatchDeleteScreenRecords;
#[cfg(feature = "BatchDeleteSubJobs")]
pub mod BatchDeleteSubJobs;
#[cfg(feature = "BatchDeleteUser")]
pub mod BatchDeleteUser;
#[cfg(feature = "BatchDeleteUserGroups")]
pub mod BatchDeleteUserGroups;
#[cfg(feature = "BatchDetachInstances")]
pub mod BatchDetachInstances;
#[cfg(feature = "BatchDisableApps")]
pub mod BatchDisableApps;
#[cfg(feature = "BatchDisassociateDesktopsEip")]
pub mod BatchDisassociateDesktopsEip;
#[cfg(feature = "BatchEnableApps")]
pub mod BatchEnableApps;
#[cfg(feature = "BatchInstallAgent")]
pub mod BatchInstallAgent;
#[cfg(feature = "BatchInstallApps")]
pub mod BatchInstallApps;
#[cfg(feature = "BatchLogoffDesktops")]
pub mod BatchLogoffDesktops;
#[cfg(feature = "BatchRebuildDesktopsSystemDisk")]
pub mod BatchRebuildDesktopsSystemDisk;
#[cfg(feature = "BatchRestoreDesktopSnapshot")]
pub mod BatchRestoreDesktopSnapshot;
#[cfg(feature = "BatchRunDesktops")]
pub mod BatchRunDesktops;
#[cfg(feature = "BatchUpdateAppAuthorizations")]
pub mod BatchUpdateAppAuthorizations;
#[cfg(feature = "BatchUpdateTargetOfPolicyGroup")]
pub mod BatchUpdateTargetOfPolicyGroup;
#[cfg(feature = "CancelRemoteAssistance")]
pub mod CancelRemoteAssistance;
#[cfg(feature = "CancelWorkspace")]
pub mod CancelWorkspace;
#[cfg(feature = "ChangeDesktopNetwork")]
pub mod ChangeDesktopNetwork;
#[cfg(feature = "ChangeDesktopToImage")]
pub mod ChangeDesktopToImage;
#[cfg(feature = "ChangeUserPrivilegeGroup")]
pub mod ChangeUserPrivilegeGroup;
#[cfg(feature = "ChangeUserStatus")]
pub mod ChangeUserStatus;
#[cfg(feature = "CreateAccessPolicy")]
pub mod CreateAccessPolicy;
#[cfg(feature = "CreateAgencies")]
pub mod CreateAgencies;
#[cfg(feature = "CreateAndAuthorizeBucket")]
pub mod CreateAndAuthorizeBucket;
#[cfg(feature = "CreateAppRule")]
pub mod CreateAppRule;
#[cfg(feature = "CreateBucketCredential")]
pub mod CreateBucketCredential;
#[cfg(feature = "CreateChangeOrder")]
pub mod CreateChangeOrder;
#[cfg(feature = "CreateDesktop")]
pub mod CreateDesktop;
#[cfg(feature = "CreateDesktopBatchOrder")]
pub mod CreateDesktopBatchOrder;
#[cfg(feature = "CreateDesktopNamePolicy")]
pub mod CreateDesktopNamePolicy;
#[cfg(feature = "CreateDesktopOrder")]
pub mod CreateDesktopOrder;
#[cfg(feature = "CreateDesktopPool")]
pub mod CreateDesktopPool;
#[cfg(feature = "CreateDesktopPoolAuthorizedObjects")]
pub mod CreateDesktopPoolAuthorizedObjects;
#[cfg(feature = "CreateDesktopPoolChangeOrder")]
pub mod CreateDesktopPoolChangeOrder;
#[cfg(feature = "CreateDesktopUser")]
pub mod CreateDesktopUser;
#[cfg(feature = "CreateOrder")]
pub mod CreateOrder;
#[cfg(feature = "CreatePolicyGroup")]
pub mod CreatePolicyGroup;
#[cfg(feature = "CreateRemoteAssistance")]
pub mod CreateRemoteAssistance;
#[cfg(feature = "CreateScheduledTasks")]
pub mod CreateScheduledTasks;
#[cfg(feature = "CreateScript")]
pub mod CreateScript;
#[cfg(feature = "CreateSubnetBandwidthChangeOrder")]
pub mod CreateSubnetBandwidthChangeOrder;
#[cfg(feature = "CreateTag")]
pub mod CreateTag;
#[cfg(feature = "CreateTerminalsBindingDesktops")]
pub mod CreateTerminalsBindingDesktops;
#[cfg(feature = "CreateUserGroup")]
pub mod CreateUserGroup;
#[cfg(feature = "DeleteApp")]
pub mod DeleteApp;
#[cfg(feature = "DeleteAppRule")]
pub mod DeleteAppRule;
#[cfg(feature = "DeleteDesktop")]
pub mod DeleteDesktop;
#[cfg(feature = "DeleteDesktopPool")]
pub mod DeleteDesktopPool;
#[cfg(feature = "DeleteDesktopPoolVolumes")]
pub mod DeleteDesktopPoolVolumes;
#[cfg(feature = "DeleteDesktopSubResources")]
pub mod DeleteDesktopSubResources;
#[cfg(feature = "DeleteDesktopVolumes")]
pub mod DeleteDesktopVolumes;
#[cfg(feature = "DeleteMetricNotifyRule")]
pub mod DeleteMetricNotifyRule;
#[cfg(feature = "DeleteOu")]
pub mod DeleteOu;
#[cfg(feature = "DeletePolicyGroup")]
pub mod DeletePolicyGroup;
#[cfg(feature = "DeleteRestrictedRule")]
pub mod DeleteRestrictedRule;
#[cfg(feature = "DeleteScheduledTasks")]
pub mod DeleteScheduledTasks;
#[cfg(feature = "DeleteScript")]
pub mod DeleteScript;
#[cfg(feature = "DeleteSite")]
pub mod DeleteSite;
#[cfg(feature = "DeleteSubnetBandwidth")]
pub mod DeleteSubnetBandwidth;
#[cfg(feature = "DeleteTag")]
pub mod DeleteTag;
#[cfg(feature = "DeleteTerminalsBindingDesktops")]
pub mod DeleteTerminalsBindingDesktops;
#[cfg(feature = "DeleteUser")]
pub mod DeleteUser;
#[cfg(feature = "DeleteUserGroup")]
pub mod DeleteUserGroup;
#[cfg(feature = "DetachInstances")]
pub mod DetachInstances;
#[cfg(feature = "DisableRuleRestriction")]
pub mod DisableRuleRestriction;
#[cfg(feature = "EnableRuleRestriction")]
pub mod EnableRuleRestriction;
#[cfg(feature = "EstimateAddResources")]
pub mod EstimateAddResources;
#[cfg(feature = "EstimateChangeImages")]
pub mod EstimateChangeImages;
