#[cfg(feature = "CreateDatabase")]
pub mod CreateDatabase;
#[cfg(feature = "CreateGroup")]
pub mod CreateGroup;
#[cfg(feature = "CreateInstance")]
pub mod CreateInstance;
#[cfg(feature = "CreateUsers")]
pub mod CreateUsers;
#[cfg(feature = "DeleteDatabase")]
pub mod DeleteDatabase;
#[cfg(feature = "DeleteInstance")]
pub mod DeleteInstance;
#[cfg(feature = "DeleteUser")]
pub mod DeleteUser;
#[cfg(feature = "ExecuteKillLogicalProcesses")]
pub mod ExecuteKillLogicalProcesses;
#[cfg(feature = "ExecuteKillPhysicalProcesses")]
pub mod ExecuteKillPhysicalProcesses;
#[cfg(feature = "ExpandInstanceNodes")]
pub mod ExpandInstanceNodes;
#[cfg(feature = "ListAvailableRdsList")]
pub mod ListAvailableRdsList;
#[cfg(feature = "ListDatabases")]
pub mod ListDatabases;
#[cfg(feature = "ListEngines")]
pub mod ListEngines;
#[cfg(feature = "ListFlavors")]
pub mod ListFlavors;
#[cfg(feature = "ListGroup")]
pub mod ListGroup;
#[cfg(feature = "ListInstances")]
pub mod ListInstances;
#[cfg(feature = "ListNodes")]
pub mod ListNodes;
#[cfg(feature = "ListReadWriteRatio")]
pub mod ListReadWriteRatio;
#[cfg(feature = "ListSlowLog")]
pub mod ListSlowLog;
#[cfg(feature = "ListUsers")]
pub mod ListUsers;
#[cfg(feature = "RebuildConfig")]
pub mod RebuildConfig;
#[cfg(feature = "ResetAdministrator")]
pub mod ResetAdministrator;
#[cfg(feature = "ResetUserPassword")]
pub mod ResetUserPassword;
#[cfg(feature = "ResizeFlavor")]
pub mod ResizeFlavor;
#[cfg(feature = "RestartInstance")]
pub mod RestartInstance;
#[cfg(feature = "ShowDatabase")]
pub mod ShowDatabase;
#[cfg(feature = "ShowInstance")]
pub mod ShowInstance;
#[cfg(feature = "ShowInstanceParam")]
pub mod ShowInstanceParam;
#[cfg(feature = "ShowLogicalProcesses")]
pub mod ShowLogicalProcesses;
#[cfg(feature = "ShowNode")]
pub mod ShowNode;
#[cfg(feature = "ShowPhysicalProcesses")]
pub mod ShowPhysicalProcesses;
#[cfg(feature = "ShowProcessesAuditLog")]
pub mod ShowProcessesAuditLog;
#[cfg(feature = "ShrinkInstanceNodes")]
pub mod ShrinkInstanceNodes;
#[cfg(feature = "UpdateDatabaseInfo")]
pub mod UpdateDatabaseInfo;
#[cfg(feature = "UpdateInstanceName")]
pub mod UpdateInstanceName;
#[cfg(feature = "UpdateInstanceParam")]
pub mod UpdateInstanceParam;
#[cfg(feature = "UpdateInstanceSecurityGroup")]
pub mod UpdateInstanceSecurityGroup;
#[cfg(feature = "UpdateReadAndWriteStrategy")]
pub mod UpdateReadAndWriteStrategy;
#[cfg(feature = "UpdateUser")]
pub mod UpdateUser;
#[cfg(feature = "ValidateWeakPassword")]
pub mod ValidateWeakPassword;
