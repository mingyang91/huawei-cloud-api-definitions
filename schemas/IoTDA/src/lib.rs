#[cfg(feature = "AddApplication")]
pub mod AddApplication;
#[cfg(feature = "AddBridge")]
pub mod AddBridge;
#[cfg(feature = "AddCertificate")]
pub mod AddCertificate;
#[cfg(feature = "AddDevice")]
pub mod AddDevice;
#[cfg(feature = "AddDeviceGroup")]
pub mod AddDeviceGroup;
#[cfg(feature = "AddFunctions")]
pub mod AddFunctions;
#[cfg(feature = "AddQueue")]
pub mod AddQueue;
#[cfg(feature = "AddTunnel")]
pub mod AddTunnel;
#[cfg(feature = "BatchShowQueue")]
pub mod BatchShowQueue;
#[cfg(feature = "BindDevicePolicy")]
pub mod BindDevicePolicy;
#[cfg(feature = "BroadcastMessage")]
pub mod BroadcastMessage;
#[cfg(feature = "ChangeRuleStatus")]
pub mod ChangeRuleStatus;
#[cfg(feature = "CheckCertificate")]
pub mod CheckCertificate;
#[cfg(feature = "CloseDeviceTunnel")]
pub mod CloseDeviceTunnel;
#[cfg(feature = "CountAsyncHistoryCommands")]
pub mod CountAsyncHistoryCommands;
#[cfg(feature = "CreateAccessCode")]
pub mod CreateAccessCode;
#[cfg(feature = "CreateAsyncCommand")]
pub mod CreateAsyncCommand;
#[cfg(feature = "CreateBatchTask")]
pub mod CreateBatchTask;
#[cfg(feature = "CreateCommand")]
pub mod CreateCommand;
#[cfg(feature = "CreateDeviceAuthorizer")]
pub mod CreateDeviceAuthorizer;
#[cfg(feature = "CreateDevicePolicy")]
pub mod CreateDevicePolicy;
#[cfg(feature = "CreateDeviceProxy")]
pub mod CreateDeviceProxy;
#[cfg(feature = "CreateMessage")]
pub mod CreateMessage;
#[cfg(feature = "CreateOrDeleteDeviceInGroup")]
pub mod CreateOrDeleteDeviceInGroup;
#[cfg(feature = "CreateOtaPackage")]
pub mod CreateOtaPackage;
#[cfg(feature = "CreateProduct")]
pub mod CreateProduct;
#[cfg(feature = "CreateProvisioningTemplate")]
pub mod CreateProvisioningTemplate;
#[cfg(feature = "CreateRoutingBacklogPolicy")]
pub mod CreateRoutingBacklogPolicy;
#[cfg(feature = "CreateRoutingFlowControlPolicy")]
pub mod CreateRoutingFlowControlPolicy;
#[cfg(feature = "CreateRoutingRule")]
pub mod CreateRoutingRule;
#[cfg(feature = "CreateRule")]
pub mod CreateRule;
#[cfg(feature = "CreateRuleAction")]
pub mod CreateRuleAction;
#[cfg(feature = "DeleteApplication")]
pub mod DeleteApplication;
#[cfg(feature = "DeleteBatchTask")]
pub mod DeleteBatchTask;
#[cfg(feature = "DeleteBatchTaskFile")]
pub mod DeleteBatchTaskFile;
#[cfg(feature = "DeleteBridge")]
pub mod DeleteBridge;
#[cfg(feature = "DeleteCertificate")]
pub mod DeleteCertificate;
#[cfg(feature = "DeleteDevice")]
pub mod DeleteDevice;
#[cfg(feature = "DeleteDeviceAuthorizer")]
pub mod DeleteDeviceAuthorizer;
#[cfg(feature = "DeleteDeviceGroup")]
pub mod DeleteDeviceGroup;
#[cfg(feature = "DeleteDevicePolicy")]
pub mod DeleteDevicePolicy;
#[cfg(feature = "DeleteDeviceProxy")]
pub mod DeleteDeviceProxy;
#[cfg(feature = "DeleteDeviceTunnel")]
pub mod DeleteDeviceTunnel;
#[cfg(feature = "DeleteFunctions")]
pub mod DeleteFunctions;
#[cfg(feature = "DeleteOtaPackage")]
pub mod DeleteOtaPackage;
#[cfg(feature = "DeleteProduct")]
pub mod DeleteProduct;
#[cfg(feature = "DeleteProvisioningTemplate")]
pub mod DeleteProvisioningTemplate;
#[cfg(feature = "DeleteQueue")]
pub mod DeleteQueue;
#[cfg(feature = "DeleteRoutingBacklogPolicy")]
pub mod DeleteRoutingBacklogPolicy;
#[cfg(feature = "DeleteRoutingFlowControlPolicy")]
pub mod DeleteRoutingFlowControlPolicy;
#[cfg(feature = "DeleteRoutingRule")]
pub mod DeleteRoutingRule;
#[cfg(feature = "DeleteRule")]
pub mod DeleteRule;
#[cfg(feature = "DeleteRuleAction")]
pub mod DeleteRuleAction;
#[cfg(feature = "FreezeDevice")]
pub mod FreezeDevice;
#[cfg(feature = "ListAsyncCommands")]
pub mod ListAsyncCommands;
#[cfg(feature = "ListAsyncHistoryCommands")]
pub mod ListAsyncHistoryCommands;
#[cfg(feature = "ListBatchTaskFiles")]
pub mod ListBatchTaskFiles;
#[cfg(feature = "ListBatchTasks")]
pub mod ListBatchTasks;
#[cfg(feature = "ListBridges")]
pub mod ListBridges;
#[cfg(feature = "ListCertificates")]
pub mod ListCertificates;
#[cfg(feature = "ListDeviceAuthorizers")]
pub mod ListDeviceAuthorizers;
#[cfg(feature = "ListDeviceGroups")]
pub mod ListDeviceGroups;
#[cfg(feature = "ListDeviceGroupsByDevice")]
pub mod ListDeviceGroupsByDevice;
#[cfg(feature = "ListDeviceMessages")]
pub mod ListDeviceMessages;
#[cfg(feature = "ListDevicePolicies")]
pub mod ListDevicePolicies;
#[cfg(feature = "ListDeviceProxies")]
pub mod ListDeviceProxies;
#[cfg(feature = "ListDevices")]
pub mod ListDevices;
#[cfg(feature = "ListDeviceTunnels")]
pub mod ListDeviceTunnels;
#[cfg(feature = "ListFunctions")]
pub mod ListFunctions;
#[cfg(feature = "ListOtaPackageInfo")]
pub mod ListOtaPackageInfo;
#[cfg(feature = "ListProducts")]
pub mod ListProducts;
#[cfg(feature = "ListProperties")]
pub mod ListProperties;
#[cfg(feature = "ListProvisioningTemplates")]
pub mod ListProvisioningTemplates;
#[cfg(feature = "ListResourcesByTags")]
pub mod ListResourcesByTags;
#[cfg(feature = "ListRoutingBacklogPolicy")]
pub mod ListRoutingBacklogPolicy;
#[cfg(feature = "ListRoutingFlowControlPolicy")]
pub mod ListRoutingFlowControlPolicy;
#[cfg(feature = "ListRoutingRules")]
pub mod ListRoutingRules;
#[cfg(feature = "ListRuleActions")]
pub mod ListRuleActions;
#[cfg(feature = "ListRules")]
pub mod ListRules;
#[cfg(feature = "ResetBridgeSecret")]
pub mod ResetBridgeSecret;
#[cfg(feature = "ResetDeviceSecret")]
pub mod ResetDeviceSecret;
#[cfg(feature = "ResetFingerprint")]
pub mod ResetFingerprint;
#[cfg(feature = "RetryBatchTask")]
pub mod RetryBatchTask;
#[cfg(feature = "SearchDevices")]
pub mod SearchDevices;
#[cfg(feature = "ShowApplication")]
pub mod ShowApplication;
#[cfg(feature = "ShowApplications")]
pub mod ShowApplications;
#[cfg(feature = "ShowAsyncDeviceCommand")]
pub mod ShowAsyncDeviceCommand;
#[cfg(feature = "ShowBatchTask")]
pub mod ShowBatchTask;
#[cfg(feature = "ShowDevice")]
pub mod ShowDevice;
#[cfg(feature = "ShowDeviceAuthorizer")]
pub mod ShowDeviceAuthorizer;
#[cfg(feature = "ShowDeviceGroup")]
pub mod ShowDeviceGroup;
#[cfg(feature = "ShowDeviceMessage")]
pub mod ShowDeviceMessage;
#[cfg(feature = "ShowDevicePolicy")]
pub mod ShowDevicePolicy;
#[cfg(feature = "ShowDeviceProxy")]
pub mod ShowDeviceProxy;
#[cfg(feature = "ShowDeviceShadow")]
pub mod ShowDeviceShadow;
#[cfg(feature = "ShowDevicesInGroup")]
pub mod ShowDevicesInGroup;
#[cfg(feature = "ShowDeviceTunnel")]
pub mod ShowDeviceTunnel;
#[cfg(feature = "ShowOtaPackage")]
pub mod ShowOtaPackage;
#[cfg(feature = "ShowProduct")]
pub mod ShowProduct;
#[cfg(feature = "ShowProvisioningTemplate")]
pub mod ShowProvisioningTemplate;
