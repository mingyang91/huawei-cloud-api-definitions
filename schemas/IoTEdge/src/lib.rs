#[cfg(feature = "AddAppConfigsTemplates")]
pub mod AddAppConfigsTemplates;
#[cfg(feature = "AddDevice")]
pub mod AddDevice;
#[cfg(feature = "AddGeneralAppConfigsTemplate")]
pub mod AddGeneralAppConfigsTemplate;
#[cfg(feature = "AddGeneralOtTemplate")]
pub mod AddGeneralOtTemplate;
#[cfg(feature = "AddOtTemplates")]
pub mod AddOtTemplates;
#[cfg(feature = "BatchAssociateNaToNodes")]
pub mod BatchAssociateNaToNodes;
#[cfg(feature = "BatchConfirmConfigsNew")]
pub mod BatchConfirmConfigsNew;
#[cfg(feature = "BatchImportConfigs")]
pub mod BatchImportConfigs;
#[cfg(feature = "BatchListAppConfigsTemplates")]
pub mod BatchListAppConfigsTemplates;
#[cfg(feature = "BatchListDcDevices")]
pub mod BatchListDcDevices;
#[cfg(feature = "BatchListDcDs")]
pub mod BatchListDcDs;
#[cfg(feature = "BatchListDcPoints")]
pub mod BatchListDcPoints;
#[cfg(feature = "BatchListEdgeApps")]
pub mod BatchListEdgeApps;
#[cfg(feature = "BatchListEdgeAppVersions")]
pub mod BatchListEdgeAppVersions;
#[cfg(feature = "BatchListModules")]
pub mod BatchListModules;
#[cfg(feature = "BatchListOtTemplates")]
pub mod BatchListOtTemplates;
#[cfg(feature = "CreateApp")]
pub mod CreateApp;
#[cfg(feature = "CreateAppInstance")]
pub mod CreateAppInstance;
#[cfg(feature = "CreateAppVersion")]
pub mod CreateAppVersion;
#[cfg(feature = "CreateCluster")]
pub mod CreateCluster;
#[cfg(feature = "CreateClusterInstallCmd")]
pub mod CreateClusterInstallCmd;
#[cfg(feature = "CreateDcPoint")]
pub mod CreateDcPoint;
#[cfg(feature = "CreateDs")]
pub mod CreateDs;
#[cfg(feature = "CreateEdgeApp")]
pub mod CreateEdgeApp;
#[cfg(feature = "CreateEdgeApplicationVersion")]
pub mod CreateEdgeApplicationVersion;
#[cfg(feature = "CreateEdgeNode")]
pub mod CreateEdgeNode;
#[cfg(feature = "CreateExternalEntity")]
pub mod CreateExternalEntity;
#[cfg(feature = "CreateInstallCmd")]
pub mod CreateInstallCmd;
#[cfg(feature = "CreateModule")]
pub mod CreateModule;
#[cfg(feature = "CreateSchedule")]
pub mod CreateSchedule;
#[cfg(feature = "DeleteApp")]
pub mod DeleteApp;
#[cfg(feature = "DeleteAppConfigsTemplate")]
pub mod DeleteAppConfigsTemplate;
#[cfg(feature = "DeleteAppInstance")]
pub mod DeleteAppInstance;
#[cfg(feature = "DeleteAppVersion")]
pub mod DeleteAppVersion;
#[cfg(feature = "DeleteCluster")]
pub mod DeleteCluster;
#[cfg(feature = "DeleteDcDs")]
pub mod DeleteDcDs;
#[cfg(feature = "DeleteDcPoint")]
pub mod DeleteDcPoint;
#[cfg(feature = "DeleteDevice")]
pub mod DeleteDevice;
#[cfg(feature = "DeleteEdgeApp")]
pub mod DeleteEdgeApp;
#[cfg(feature = "DeleteEdgeApplicationVersion")]
pub mod DeleteEdgeApplicationVersion;
#[cfg(feature = "DeleteEdgeNode")]
pub mod DeleteEdgeNode;
#[cfg(feature = "DeleteExternalEntity")]
pub mod DeleteExternalEntity;
#[cfg(feature = "DeleteIaConfig")]
pub mod DeleteIaConfig;
#[cfg(feature = "DeleteModule")]
pub mod DeleteModule;
#[cfg(feature = "DeleteNa")]
pub mod DeleteNa;
#[cfg(feature = "DeleteOtTemplate")]
pub mod DeleteOtTemplate;
#[cfg(feature = "DeleteSchedule")]
pub mod DeleteSchedule;
#[cfg(feature = "DownloadAppVersion")]
pub mod DownloadAppVersion;
#[cfg(feature = "ExecuteDeviceControlsRelease")]
pub mod ExecuteDeviceControlsRelease;
#[cfg(feature = "ExecuteDeviceControlsSet")]
pub mod ExecuteDeviceControlsSet;
#[cfg(feature = "ImportPoints")]
pub mod ImportPoints;
#[cfg(feature = "InvokeDeleteProxy")]
pub mod InvokeDeleteProxy;
#[cfg(feature = "InvokeGetProxy")]
pub mod InvokeGetProxy;
#[cfg(feature = "InvokePatchProxy")]
pub mod InvokePatchProxy;
#[cfg(feature = "InvokePostProxy")]
pub mod InvokePostProxy;
#[cfg(feature = "InvokePutProxy")]
pub mod InvokePutProxy;
#[cfg(feature = "ListAppImage")]
pub mod ListAppImage;
#[cfg(feature = "ListAppInstanceHistory")]
pub mod ListAppInstanceHistory;
#[cfg(feature = "ListAppInstances")]
pub mod ListAppInstances;
#[cfg(feature = "ListApps")]
pub mod ListApps;
#[cfg(feature = "ListAppVersions")]
pub mod ListAppVersions;
#[cfg(feature = "ListClusters")]
pub mod ListClusters;
#[cfg(feature = "ListDevices")]
pub mod ListDevices;
#[cfg(feature = "ListEdgeNodes")]
pub mod ListEdgeNodes;
#[cfg(feature = "ListExternalEntity")]
pub mod ListExternalEntity;
#[cfg(feature = "ListIaConfigs")]
pub mod ListIaConfigs;
#[cfg(feature = "ListNaAuthorizedNodes")]
pub mod ListNaAuthorizedNodes;
#[cfg(feature = "ListNas")]
pub mod ListNas;
#[cfg(feature = "ListRoutes")]
pub mod ListRoutes;
#[cfg(feature = "ShowApp")]
pub mod ShowApp;
#[cfg(feature = "ShowAppConfigsTemplate")]
pub mod ShowAppConfigsTemplate;
#[cfg(feature = "ShowAppVersion")]
pub mod ShowAppVersion;
#[cfg(feature = "ShowCluster")]
pub mod ShowCluster;
#[cfg(feature = "ShowDcDs")]
pub mod ShowDcDs;
#[cfg(feature = "ShowDcPoint")]
pub mod ShowDcPoint;
#[cfg(feature = "ShowEdgeApp")]
pub mod ShowEdgeApp;
#[cfg(feature = "ShowEdgeApplicationVersion")]
pub mod ShowEdgeApplicationVersion;
#[cfg(feature = "ShowEdgeNode")]
pub mod ShowEdgeNode;
#[cfg(feature = "ShowIaConfig")]
pub mod ShowIaConfig;
#[cfg(feature = "ShowModule")]
pub mod ShowModule;
#[cfg(feature = "ShowModuleShadow")]
pub mod ShowModuleShadow;
#[cfg(feature = "ShowNa")]
pub mod ShowNa;
#[cfg(feature = "ShowOtTemplate")]
pub mod ShowOtTemplate;
#[cfg(feature = "ShowPoints")]
pub mod ShowPoints;
#[cfg(feature = "ShowPointTemplate")]
pub mod ShowPointTemplate;
#[cfg(feature = "ShowProductConfig")]
pub mod ShowProductConfig;
#[cfg(feature = "SynchronizeDcConfigs")]
pub mod SynchronizeDcConfigs;
#[cfg(feature = "UpdateAppInstance")]
pub mod UpdateAppInstance;
#[cfg(feature = "UpdateDcDs")]
pub mod UpdateDcDs;
#[cfg(feature = "UpdateDcPoint")]
pub mod UpdateDcPoint;
#[cfg(feature = "UpdateDevice")]
pub mod UpdateDevice;
#[cfg(feature = "UpdateEdgeApplicationVersion")]
pub mod UpdateEdgeApplicationVersion;
#[cfg(feature = "UpdateEdgeApplicationVersionState")]
pub mod UpdateEdgeApplicationVersionState;
#[cfg(feature = "UpdateExternalEntity")]
pub mod UpdateExternalEntity;
#[cfg(feature = "UpdateIaConfig")]
pub mod UpdateIaConfig;
#[cfg(feature = "UpdateModule")]
pub mod UpdateModule;
#[cfg(feature = "UpdateModuleShadow")]
pub mod UpdateModuleShadow;
#[cfg(feature = "UpdateModuleState")]
pub mod UpdateModuleState;
#[cfg(feature = "UpdateNa")]
pub mod UpdateNa;
#[cfg(feature = "UpdateRoutes")]
pub mod UpdateRoutes;