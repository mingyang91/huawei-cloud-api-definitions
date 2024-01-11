#[cfg(feature = "AddApplication")]
pub mod AddApplication;
#[cfg(feature = "AddCertificate")]
pub mod AddCertificate;
#[cfg(feature = "AddDevice")]
pub mod AddDevice;
#[cfg(feature = "AddDeviceGroup")]
pub mod AddDeviceGroup;
#[cfg(feature = "AddQueue")]
pub mod AddQueue;
#[cfg(feature = "AddTunnel")]
pub mod AddTunnel;
#[cfg(feature = "BatchShowQueue")]
pub mod BatchShowQueue;
#[cfg(feature = "BroadcastMessage")]
pub mod BroadcastMessage;
#[cfg(feature = "ChangeRuleStatus")]
pub mod ChangeRuleStatus;
#[cfg(feature = "CheckCertificate")]
pub mod CheckCertificate;
#[cfg(feature = "CloseDeviceTunnel")]
pub mod CloseDeviceTunnel;
#[cfg(feature = "CreateAccessCode")]
pub mod CreateAccessCode;
#[cfg(feature = "CreateAsyncCommand")]
pub mod CreateAsyncCommand;
#[cfg(feature = "CreateBatchTask")]
pub mod CreateBatchTask;
#[cfg(feature = "CreateCommand")]
pub mod CreateCommand;
#[cfg(feature = "CreateMessage")]
pub mod CreateMessage;
#[cfg(feature = "CreateOrDeleteDeviceInGroup")]
pub mod CreateOrDeleteDeviceInGroup;
#[cfg(feature = "CreateOtaPackage")]
pub mod CreateOtaPackage;
#[cfg(feature = "CreateProduct")]
pub mod CreateProduct;
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
#[cfg(feature = "DeleteCertificate")]
pub mod DeleteCertificate;
#[cfg(feature = "DeleteDevice")]
pub mod DeleteDevice;
#[cfg(feature = "DeleteDeviceGroup")]
pub mod DeleteDeviceGroup;
#[cfg(feature = "DeleteDeviceTunnel")]
pub mod DeleteDeviceTunnel;
#[cfg(feature = "DeleteOtaPackage")]
pub mod DeleteOtaPackage;
#[cfg(feature = "DeleteProduct")]
pub mod DeleteProduct;
#[cfg(feature = "DeleteQueue")]
pub mod DeleteQueue;
#[cfg(feature = "DeleteRoutingRule")]
pub mod DeleteRoutingRule;
#[cfg(feature = "DeleteRule")]
pub mod DeleteRule;
#[cfg(feature = "DeleteRuleAction")]
pub mod DeleteRuleAction;
#[cfg(feature = "FreezeDevice")]
pub mod FreezeDevice;
#[cfg(feature = "ListBatchTaskFiles")]
pub mod ListBatchTaskFiles;
#[cfg(feature = "ListBatchTasks")]
pub mod ListBatchTasks;
#[cfg(feature = "ListCertificates")]
pub mod ListCertificates;
#[cfg(feature = "ListDeviceGroups")]
pub mod ListDeviceGroups;
#[cfg(feature = "ListDeviceMessages")]
pub mod ListDeviceMessages;
#[cfg(feature = "ListDevices")]
pub mod ListDevices;
#[cfg(feature = "ListDeviceTunnels")]
pub mod ListDeviceTunnels;
#[cfg(feature = "ListOtaPackageInfo")]
pub mod ListOtaPackageInfo;
#[cfg(feature = "ListProducts")]
pub mod ListProducts;
#[cfg(feature = "ListProperties")]
pub mod ListProperties;
#[cfg(feature = "ListResourcesByTags")]
pub mod ListResourcesByTags;
#[cfg(feature = "ListRoutingRules")]
pub mod ListRoutingRules;
#[cfg(feature = "ListRuleActions")]
pub mod ListRuleActions;
#[cfg(feature = "ListRules")]
pub mod ListRules;
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
#[cfg(feature = "ShowDeviceGroup")]
pub mod ShowDeviceGroup;
#[cfg(feature = "ShowDeviceMessage")]
pub mod ShowDeviceMessage;
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
#[cfg(feature = "ShowQueue")]
pub mod ShowQueue;
#[cfg(feature = "ShowRoutingRule")]
pub mod ShowRoutingRule;
#[cfg(feature = "ShowRule")]
pub mod ShowRule;
#[cfg(feature = "ShowRuleAction")]
pub mod ShowRuleAction;
#[cfg(feature = "StopBatchTask")]
pub mod StopBatchTask;
#[cfg(feature = "TagDevice")]
pub mod TagDevice;
#[cfg(feature = "UnfreezeDevice")]
pub mod UnfreezeDevice;
#[cfg(feature = "UntagDevice")]
pub mod UntagDevice;
#[cfg(feature = "UpdateDevice")]
pub mod UpdateDevice;
#[cfg(feature = "UpdateDeviceGroup")]
pub mod UpdateDeviceGroup;
#[cfg(feature = "UpdateDeviceShadowDesiredData")]
pub mod UpdateDeviceShadowDesiredData;
#[cfg(feature = "UpdateProduct")]
pub mod UpdateProduct;
#[cfg(feature = "UpdateProperties")]
pub mod UpdateProperties;
#[cfg(feature = "UpdateRoutingRule")]
pub mod UpdateRoutingRule;
#[cfg(feature = "UpdateRule")]
pub mod UpdateRule;
#[cfg(feature = "UpdateRuleAction")]
pub mod UpdateRuleAction;
#[cfg(feature = "UploadBatchTaskFile")]
pub mod UploadBatchTaskFile;