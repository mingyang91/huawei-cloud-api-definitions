#[cfg(feature = "AssociateQueueToElasticResourcePool")]
pub mod AssociateQueueToElasticResourcePool;
#[cfg(feature = "AssociateQueueToEnhancedConnection")]
pub mod AssociateQueueToEnhancedConnection;
#[cfg(feature = "BatchCreateResourceTags")]
pub mod BatchCreateResourceTags;
#[cfg(feature = "BatchDeleteFlinkJobs")]
pub mod BatchDeleteFlinkJobs;
#[cfg(feature = "BatchDeleteQueuePlans")]
pub mod BatchDeleteQueuePlans;
#[cfg(feature = "BatchDeleteResourceTags")]
pub mod BatchDeleteResourceTags;
#[cfg(feature = "BatchDeleteSqlJobTemplates")]
pub mod BatchDeleteSqlJobTemplates;
#[cfg(feature = "BatchRunFlinkJobs")]
pub mod BatchRunFlinkJobs;
#[cfg(feature = "CancelSparkJob")]
pub mod CancelSparkJob;
#[cfg(feature = "CancelSqlJob")]
pub mod CancelSqlJob;
#[cfg(feature = "ChangeAuthorization")]
pub mod ChangeAuthorization;
#[cfg(feature = "ChangeFlinkJobStatusReport")]
pub mod ChangeFlinkJobStatusReport;
#[cfg(feature = "ChangeQueuePlan")]
pub mod ChangeQueuePlan;
#[cfg(feature = "CheckSql")]
pub mod CheckSql;
#[cfg(feature = "CountResourcesByTags")]
pub mod CountResourcesByTags;
#[cfg(feature = "CreateAuthInfo")]
pub mod CreateAuthInfo;
#[cfg(feature = "CreateConnectivityTask")]
pub mod CreateConnectivityTask;
#[cfg(feature = "CreateDatabase")]
pub mod CreateDatabase;
#[cfg(feature = "CreateDatasourceConnection")]
pub mod CreateDatasourceConnection;
#[cfg(feature = "CreateDliAgency")]
pub mod CreateDliAgency;
#[cfg(feature = "CreateElasticResourcePool")]
pub mod CreateElasticResourcePool;
#[cfg(feature = "CreateEnhancedConnection")]
pub mod CreateEnhancedConnection;
#[cfg(feature = "CreateEnhancedConnectionRoutes")]
pub mod CreateEnhancedConnectionRoutes;
#[cfg(feature = "CreateFlinkJarJob")]
pub mod CreateFlinkJarJob;
#[cfg(feature = "CreateFlinkSqlJob")]
pub mod CreateFlinkSqlJob;
#[cfg(feature = "CreateFlinkSqlJobGraph")]
pub mod CreateFlinkSqlJobGraph;
#[cfg(feature = "CreateFlinkSqlJobTemplate")]
pub mod CreateFlinkSqlJobTemplate;
#[cfg(feature = "CreateGlobalVariable")]
pub mod CreateGlobalVariable;
#[cfg(feature = "CreateIefMessageChannel")]
pub mod CreateIefMessageChannel;
#[cfg(feature = "CreateIefSystemEvents")]
pub mod CreateIefSystemEvents;
#[cfg(feature = "CreateJobAuthInfo")]
pub mod CreateJobAuthInfo;
#[cfg(feature = "CreatePeriodElasticResourcePoolOrder")]
pub mod CreatePeriodElasticResourcePoolOrder;
#[cfg(feature = "CreatePeriodElasticResourcePoolSpecChangeOrder")]
pub mod CreatePeriodElasticResourcePoolSpecChangeOrder;
#[cfg(feature = "CreateQueue")]
pub mod CreateQueue;
#[cfg(feature = "CreateQueuePlan")]
pub mod CreateQueuePlan;
#[cfg(feature = "CreateQueueProperty")]
pub mod CreateQueueProperty;
#[cfg(feature = "CreateRouteToEnhancedConnection")]
pub mod CreateRouteToEnhancedConnection;
#[cfg(feature = "CreateSparkJob")]
pub mod CreateSparkJob;
#[cfg(feature = "CreateSparkJobTemplate")]
pub mod CreateSparkJobTemplate;
#[cfg(feature = "CreateSqlJob")]
pub mod CreateSqlJob;
#[cfg(feature = "CreateSqlJobTemplate")]
pub mod CreateSqlJobTemplate;
#[cfg(feature = "CreateTable")]
pub mod CreateTable;
#[cfg(feature = "DeleteAuthInfo")]
pub mod DeleteAuthInfo;
#[cfg(feature = "DeleteDatabase")]
pub mod DeleteDatabase;
#[cfg(feature = "DeleteDatasourceConnection")]
pub mod DeleteDatasourceConnection;
#[cfg(feature = "DeleteElasticResourcePool")]
pub mod DeleteElasticResourcePool;
#[cfg(feature = "DeleteEnhancedConnection")]
pub mod DeleteEnhancedConnection;
#[cfg(feature = "DeleteEnhancedConnectionRoutes")]
pub mod DeleteEnhancedConnectionRoutes;
#[cfg(feature = "DeleteFlinkJob")]
pub mod DeleteFlinkJob;
#[cfg(feature = "DeleteFlinkSqlJobTemplate")]
pub mod DeleteFlinkSqlJobTemplate;
#[cfg(feature = "DeleteGlobalVariable")]
pub mod DeleteGlobalVariable;
#[cfg(feature = "DeleteJobAuthInfo")]
pub mod DeleteJobAuthInfo;
#[cfg(feature = "DeleteQueue")]
pub mod DeleteQueue;
#[cfg(feature = "DeleteQueuePlan")]
pub mod DeleteQueuePlan;
#[cfg(feature = "DeleteQueueProperty")]
pub mod DeleteQueueProperty;
#[cfg(feature = "DeleteResource")]
pub mod DeleteResource;
#[cfg(feature = "DeleteRouteFromEnhancedConnection")]
pub mod DeleteRouteFromEnhancedConnection;
#[cfg(feature = "DeleteTable")]
pub mod DeleteTable;
#[cfg(feature = "DisassociateQueueFromEnhancedConnection")]
pub mod DisassociateQueueFromEnhancedConnection;
#[cfg(feature = "ExecuteFlinkJobSavepoint")]
pub mod ExecuteFlinkJobSavepoint;
#[cfg(feature = "ExportFlinkJobs")]
pub mod ExportFlinkJobs;
#[cfg(feature = "ExportSqlJobResult")]
pub mod ExportSqlJobResult;
#[cfg(feature = "ExportTable")]
pub mod ExportTable;
#[cfg(feature = "ImportFlinkJobs")]
pub mod ImportFlinkJobs;
#[cfg(feature = "ImportFlinkJobSavepoint")]
pub mod ImportFlinkJobSavepoint;
#[cfg(feature = "ImportTable")]
pub mod ImportTable;
#[cfg(feature = "ListAllTables")]
pub mod ListAllTables;
#[cfg(feature = "ListAuthInfo")]
pub mod ListAuthInfo;
#[cfg(feature = "ListAuthorizationPrivileges")]
pub mod ListAuthorizationPrivileges;
#[cfg(feature = "ListCatalogs")]
pub mod ListCatalogs;
#[cfg(feature = "ListDatabases")]
pub mod ListDatabases;
#[cfg(feature = "ListDatabaseUsers")]
pub mod ListDatabaseUsers;
#[cfg(feature = "ListDatasourceConnections")]
pub mod ListDatasourceConnections;
#[cfg(feature = "ListElasticResourcePoolQueues")]
pub mod ListElasticResourcePoolQueues;
#[cfg(feature = "ListElasticResourcePools")]
pub mod ListElasticResourcePools;
#[cfg(feature = "ListElasticResourcePoolScaleRecords")]
pub mod ListElasticResourcePoolScaleRecords;
#[cfg(feature = "ListEnhancedConnections")]
pub mod ListEnhancedConnections;
#[cfg(feature = "ListFlinkJobs")]
pub mod ListFlinkJobs;
#[cfg(feature = "ListFlinkSqlJobTemplates")]
pub mod ListFlinkSqlJobTemplates;
#[cfg(feature = "ListGlobalVariables")]
pub mod ListGlobalVariables;
#[cfg(feature = "ListJobAuthInfos")]
pub mod ListJobAuthInfos;
#[cfg(feature = "ListQueuePlans")]
pub mod ListQueuePlans;
#[cfg(feature = "ListQueueProperties")]
pub mod ListQueueProperties;
#[cfg(feature = "ListQueues")]
pub mod ListQueues;
#[cfg(feature = "ListQueueUsers")]
pub mod ListQueueUsers;
#[cfg(feature = "ListResources")]
pub mod ListResources;
#[cfg(feature = "ListResourcesByTags")]
pub mod ListResourcesByTags;
#[cfg(feature = "ListResourcesTags")]
pub mod ListResourcesTags;
#[cfg(feature = "ListSparkJobs")]
pub mod ListSparkJobs;
#[cfg(feature = "ListSparkJobTemplates")]
pub mod ListSparkJobTemplates;
#[cfg(feature = "ListSqlJobs")]
pub mod ListSqlJobs;
#[cfg(feature = "ListSqlJobTemplates")]
pub mod ListSqlJobTemplates;
#[cfg(feature = "ListTablePrivileges")]
pub mod ListTablePrivileges;
#[cfg(feature = "ListTableUsers")]
pub mod ListTableUsers;
#[cfg(feature = "PreviewSqlJobResult")]
pub mod PreviewSqlJobResult;
#[cfg(feature = "RegisterAuthorizedQueue")]
pub mod RegisterAuthorizedQueue;
#[cfg(feature = "RegisterBucket")]
pub mod RegisterBucket;
#[cfg(feature = "RunAuthorizationAction")]
pub mod RunAuthorizationAction;
#[cfg(feature = "RunCatalogAction")]
pub mod RunCatalogAction;
#[cfg(feature = "RunIefJobActionCallBack")]
pub mod RunIefJobActionCallBack;
