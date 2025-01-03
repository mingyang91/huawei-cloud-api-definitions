#[cfg(feature = "AddDesktopPoolVolumes")]
pub mod AddDesktopPoolVolumes;
#[cfg(feature = "AddMetricNotifyRule")]
pub mod AddMetricNotifyRule;
#[cfg(feature = "AddVolumes")]
pub mod AddVolumes;
#[cfg(feature = "ApplyDesktopsInternet")]
pub mod ApplyDesktopsInternet;
#[cfg(feature = "ApplyWorkspace")]
pub mod ApplyWorkspace;
#[cfg(feature = "AssociateDesktopsEip")]
pub mod AssociateDesktopsEip;
#[cfg(feature = "AttachInstances")]
pub mod AttachInstances;
#[cfg(feature = "BatchAddDesktopsTags")]
pub mod BatchAddDesktopsTags;
#[cfg(feature = "BatchChangeTags")]
pub mod BatchChangeTags;
#[cfg(feature = "BatchCreateUsers")]
pub mod BatchCreateUsers;
#[cfg(feature = "BatchDeleteAccessPolicies")]
pub mod BatchDeleteAccessPolicies;
#[cfg(feature = "BatchDeleteDesktopNamePolicy")]
pub mod BatchDeleteDesktopNamePolicy;
#[cfg(feature = "BatchDeleteDesktops")]
pub mod BatchDeleteDesktops;
#[cfg(feature = "BatchDeleteDesktopsTags")]
pub mod BatchDeleteDesktopsTags;
#[cfg(feature = "BatchDeleteOtpDevices")]
pub mod BatchDeleteOtpDevices;
#[cfg(feature = "BatchDeleteScheduledTasks")]
pub mod BatchDeleteScheduledTasks;
#[cfg(feature = "BatchDeleteUser")]
pub mod BatchDeleteUser;
#[cfg(feature = "BatchDeleteUserGroups")]
pub mod BatchDeleteUserGroups;
#[cfg(feature = "BatchDisassociateDesktopsEip")]
pub mod BatchDisassociateDesktopsEip;
#[cfg(feature = "BatchLogoffDesktops")]
pub mod BatchLogoffDesktops;
#[cfg(feature = "BatchRebuildDesktopsSystemDisk")]
pub mod BatchRebuildDesktopsSystemDisk;
#[cfg(feature = "BatchRunDesktops")]
pub mod BatchRunDesktops;
#[cfg(feature = "CancelRemoteAssistance")]
pub mod CancelRemoteAssistance;
#[cfg(feature = "CancelWorkspace")]
pub mod CancelWorkspace;
#[cfg(feature = "ChangeDesktopNetwork")]
pub mod ChangeDesktopNetwork;
#[cfg(feature = "ChangeUserPrivilegeGroup")]
pub mod ChangeUserPrivilegeGroup;
#[cfg(feature = "ChangeUserStatus")]
pub mod ChangeUserStatus;
#[cfg(feature = "CreateAccessPolicy")]
pub mod CreateAccessPolicy;
#[cfg(feature = "CreateAgencies")]
pub mod CreateAgencies;
#[cfg(feature = "CreateDesktop")]
pub mod CreateDesktop;
#[cfg(feature = "CreateDesktopNamePolicy")]
pub mod CreateDesktopNamePolicy;
#[cfg(feature = "CreateDesktopPool")]
pub mod CreateDesktopPool;
#[cfg(feature = "CreateDesktopPoolAuthorizedObjects")]
pub mod CreateDesktopPoolAuthorizedObjects;
#[cfg(feature = "CreateDesktopUser")]
pub mod CreateDesktopUser;
#[cfg(feature = "CreateRemoteAssistance")]
pub mod CreateRemoteAssistance;
#[cfg(feature = "CreateScheduledTasks")]
pub mod CreateScheduledTasks;
#[cfg(feature = "CreateTag")]
pub mod CreateTag;
#[cfg(feature = "CreateTerminalsBindingDesktops")]
pub mod CreateTerminalsBindingDesktops;
#[cfg(feature = "CreateUserGroup")]
pub mod CreateUserGroup;
#[cfg(feature = "DeleteDesktop")]
pub mod DeleteDesktop;
#[cfg(feature = "DeleteDesktopPool")]
pub mod DeleteDesktopPool;
#[cfg(feature = "DeleteDesktopPoolVolumes")]
pub mod DeleteDesktopPoolVolumes;
#[cfg(feature = "DeleteDesktopVolumes")]
pub mod DeleteDesktopVolumes;
#[cfg(feature = "DeleteMetricNotifyRule")]
pub mod DeleteMetricNotifyRule;
#[cfg(feature = "DeleteScheduledTasks")]
pub mod DeleteScheduledTasks;
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
#[cfg(feature = "ExecuteDesktopPoolAction")]
pub mod ExecuteDesktopPoolAction;
#[cfg(feature = "ExpandDesktopPool")]
pub mod ExpandDesktopPool;
#[cfg(feature = "ExpandDesktopPoolVolumes")]
pub mod ExpandDesktopPoolVolumes;
#[cfg(feature = "ExpandVolumes")]
pub mod ExpandVolumes;
#[cfg(feature = "ExportUserLoginInfoNew")]
pub mod ExportUserLoginInfoNew;
#[cfg(feature = "ListAccessPolicies")]
pub mod ListAccessPolicies;
#[cfg(feature = "ListAccessPolicyObjects")]
pub mod ListAccessPolicyObjects;
#[cfg(feature = "ListAgencies")]
pub mod ListAgencies;
#[cfg(feature = "ListAvailabilityZones")]
pub mod ListAvailabilityZones;
#[cfg(feature = "ListDesktopByTags")]
pub mod ListDesktopByTags;
#[cfg(feature = "ListDesktopNamePolicy")]
pub mod ListDesktopNamePolicy;
#[cfg(feature = "ListDesktopPoolAuthorizedObjects")]
pub mod ListDesktopPoolAuthorizedObjects;
#[cfg(feature = "ListDesktopPools")]
pub mod ListDesktopPools;
#[cfg(feature = "ListDesktops")]
pub mod ListDesktops;
#[cfg(feature = "ListDesktopsConnectStatus")]
pub mod ListDesktopsConnectStatus;
#[cfg(feature = "ListDesktopsDetail")]
pub mod ListDesktopsDetail;
#[cfg(feature = "ListDesktopsEips")]
pub mod ListDesktopsEips;
#[cfg(feature = "ListDesktopUsageMetric")]
pub mod ListDesktopUsageMetric;
#[cfg(feature = "ListFutureExecutions")]
pub mod ListFutureExecutions;
#[cfg(feature = "ListHistoryOnlineInfoNew")]
pub mod ListHistoryOnlineInfoNew;
#[cfg(feature = "ListImages")]
pub mod ListImages;
#[cfg(feature = "ListItaSubJobs")]
pub mod ListItaSubJobs;
#[cfg(feature = "ListLoginRecordsNew")]
pub mod ListLoginRecordsNew;
#[cfg(feature = "ListMetricNotifyRecord")]
pub mod ListMetricNotifyRecord;
#[cfg(feature = "ListMetricNotifyRule")]
pub mod ListMetricNotifyRule;
#[cfg(feature = "ListMetrics")]
pub mod ListMetrics;
#[cfg(feature = "ListMetricsTrend")]
pub mod ListMetricsTrend;
#[cfg(feature = "ListOtpDevicesByUserId")]
pub mod ListOtpDevicesByUserId;
#[cfg(feature = "ListPoolDesktopsDetail")]
pub mod ListPoolDesktopsDetail;
#[cfg(feature = "ListProducts")]
pub mod ListProducts;
#[cfg(feature = "ListProjectTags")]
pub mod ListProjectTags;
#[cfg(feature = "ListScheduledTasks")]
pub mod ListScheduledTasks;
#[cfg(feature = "ListScheduledTasksRecords")]
pub mod ListScheduledTasksRecords;
#[cfg(feature = "ListScheduledTasksRecordsDetails")]
pub mod ListScheduledTasksRecordsDetails;
#[cfg(feature = "ListTerminalsBindingDesktops")]
pub mod ListTerminalsBindingDesktops;
#[cfg(feature = "ListTerminalsBindingDesktopsConfig")]
pub mod ListTerminalsBindingDesktopsConfig;
#[cfg(feature = "ListUnusedDesktops")]
pub mod ListUnusedDesktops;
#[cfg(feature = "ListUsedDesktopInfo")]
pub mod ListUsedDesktopInfo;
#[cfg(feature = "ListUserDetail")]
pub mod ListUserDetail;
#[cfg(feature = "ListUserGroups")]
pub mod ListUserGroups;
#[cfg(feature = "ListUsers")]
pub mod ListUsers;
#[cfg(feature = "ListUsersOfGroup")]
pub mod ListUsersOfGroup;
#[cfg(feature = "ListUserUsageMetric")]
pub mod ListUserUsageMetric;
#[cfg(feature = "ListWorkspaces")]
pub mod ListWorkspaces;
#[cfg(feature = "RebuildDesktopPool")]
pub mod RebuildDesktopPool;
#[cfg(feature = "ResetRandomPassword")]
pub mod ResetRandomPassword;
#[cfg(feature = "ResizeDesktop")]
pub mod ResizeDesktop;
#[cfg(feature = "ResizeDesktopPool")]
pub mod ResizeDesktopPool;
#[cfg(feature = "RunActionsOnGroup")]
pub mod RunActionsOnGroup;
#[cfg(feature = "ShowAssistAuthConfig")]
pub mod ShowAssistAuthConfig;
