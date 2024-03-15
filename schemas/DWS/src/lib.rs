#[cfg(feature = "AddQueueUserList")]
pub mod AddQueueUserList;
#[cfg(feature = "AddSnapshotCrossRegionPolicy")]
pub mod AddSnapshotCrossRegionPolicy;
#[cfg(feature = "AddWorkloadPlanStage")]
pub mod AddWorkloadPlanStage;
#[cfg(feature = "AddWorkloadQueue")]
pub mod AddWorkloadQueue;
#[cfg(feature = "AssociateEip")]
pub mod AssociateEip;
#[cfg(feature = "AssociateElb")]
pub mod AssociateElb;
#[cfg(feature = "BatchCreateClusterCn")]
pub mod BatchCreateClusterCn;
#[cfg(feature = "BatchCreateResourceTag")]
pub mod BatchCreateResourceTag;
#[cfg(feature = "BatchDeleteClusterCn")]
pub mod BatchDeleteClusterCn;
#[cfg(feature = "BatchDeleteResourceTag")]
pub mod BatchDeleteResourceTag;
#[cfg(feature = "CancelReadonlyCluster")]
pub mod CancelReadonlyCluster;
#[cfg(feature = "CheckCluster")]
pub mod CheckCluster;
#[cfg(feature = "CheckDisasterName")]
pub mod CheckDisasterName;
#[cfg(feature = "CheckTableRestore")]
pub mod CheckTableRestore;
#[cfg(feature = "ConvertToLogicalCluster")]
pub mod ConvertToLogicalCluster;
#[cfg(feature = "CopySnapshot")]
pub mod CopySnapshot;
#[cfg(feature = "CreateAlarmSub")]
pub mod CreateAlarmSub;
#[cfg(feature = "CreateCluster")]
pub mod CreateCluster;
#[cfg(feature = "CreateClusterDns")]
pub mod CreateClusterDns;
#[cfg(feature = "CreateClusterV2")]
pub mod CreateClusterV2;
#[cfg(feature = "CreateClusterWorkload")]
pub mod CreateClusterWorkload;
#[cfg(feature = "CreateDataSource")]
pub mod CreateDataSource;
#[cfg(feature = "CreateDisasterRecovery")]
pub mod CreateDisasterRecovery;
#[cfg(feature = "CreateEventSub")]
pub mod CreateEventSub;
#[cfg(feature = "CreateLogicalCluster")]
pub mod CreateLogicalCluster;
#[cfg(feature = "CreateSnapshot")]
pub mod CreateSnapshot;
#[cfg(feature = "CreateSnapshotPolicy")]
pub mod CreateSnapshotPolicy;
#[cfg(feature = "CreateWorkloadPlan")]
pub mod CreateWorkloadPlan;
#[cfg(feature = "DeleteAlarmSub")]
pub mod DeleteAlarmSub;
#[cfg(feature = "DeleteCluster")]
pub mod DeleteCluster;
#[cfg(feature = "DeleteClusterDns")]
pub mod DeleteClusterDns;
#[cfg(feature = "DeleteClusterNodes")]
pub mod DeleteClusterNodes;
#[cfg(feature = "DeleteDataSource")]
pub mod DeleteDataSource;
#[cfg(feature = "DeleteDisasterRecovery")]
pub mod DeleteDisasterRecovery;
#[cfg(feature = "DeleteEventSub")]
pub mod DeleteEventSub;
#[cfg(feature = "DeleteLogicalCluster")]
pub mod DeleteLogicalCluster;
#[cfg(feature = "DeleteQueueUserList")]
pub mod DeleteQueueUserList;
#[cfg(feature = "DeleteSnapshot")]
pub mod DeleteSnapshot;
#[cfg(feature = "DeleteSnapshotCrossRegionPolicy")]
pub mod DeleteSnapshotCrossRegionPolicy;
#[cfg(feature = "DeleteSnapshotPolicy")]
pub mod DeleteSnapshotPolicy;
#[cfg(feature = "DeleteWorkloadPlan")]
pub mod DeleteWorkloadPlan;
#[cfg(feature = "DeleteWorkloadPlanStage")]
pub mod DeleteWorkloadPlanStage;
#[cfg(feature = "DeleteWorkloadQueue")]
pub mod DeleteWorkloadQueue;
#[cfg(feature = "DisableLtsLogs")]
pub mod DisableLtsLogs;
#[cfg(feature = "DisassociateEip")]
pub mod DisassociateEip;
#[cfg(feature = "DisassociateElb")]
pub mod DisassociateElb;
#[cfg(feature = "EnableLogicalCluster")]
pub mod EnableLogicalCluster;
#[cfg(feature = "EnableLtsLogs")]
pub mod EnableLtsLogs;
#[cfg(feature = "ExecuteClusterUpgradeAction")]
pub mod ExecuteClusterUpgradeAction;
#[cfg(feature = "ExecuteDatabaseOmUserAction")]
pub mod ExecuteDatabaseOmUserAction;
#[cfg(feature = "ExecuteRedistributionCluster")]
pub mod ExecuteRedistributionCluster;
#[cfg(feature = "ExpandInstanceStorage")]
pub mod ExpandInstanceStorage;
#[cfg(feature = "ListAlarmConfigs")]
pub mod ListAlarmConfigs;
#[cfg(feature = "ListAlarmDetail")]
pub mod ListAlarmDetail;
#[cfg(feature = "ListAlarmStatistic")]
pub mod ListAlarmStatistic;
#[cfg(feature = "ListAlarmSubs")]
pub mod ListAlarmSubs;
#[cfg(feature = "ListAuditLog")]
pub mod ListAuditLog;
#[cfg(feature = "ListAvailabilityZones")]
pub mod ListAvailabilityZones;
#[cfg(feature = "ListAvailableDisasterClusters")]
pub mod ListAvailableDisasterClusters;
#[cfg(feature = "ListClusterCn")]
pub mod ListClusterCn;
#[cfg(feature = "ListClusterConfigurations")]
pub mod ListClusterConfigurations;
#[cfg(feature = "ListClusterConfigurationsParameter")]
pub mod ListClusterConfigurationsParameter;
#[cfg(feature = "ListClusterDetails")]
pub mod ListClusterDetails;
#[cfg(feature = "ListClusterNodes")]
pub mod ListClusterNodes;
#[cfg(feature = "ListClusters")]
pub mod ListClusters;
#[cfg(feature = "ListClusterScaleInNumbers")]
pub mod ListClusterScaleInNumbers;
#[cfg(feature = "ListClusterSnapshots")]
pub mod ListClusterSnapshots;
#[cfg(feature = "ListClusterTags")]
pub mod ListClusterTags;
#[cfg(feature = "ListClusterWorkload")]
pub mod ListClusterWorkload;
#[cfg(feature = "ListConfigurationsAuditRecords")]
pub mod ListConfigurationsAuditRecords;
#[cfg(feature = "ListDatabaseUsers")]
pub mod ListDatabaseUsers;
#[cfg(feature = "ListDataSource")]
pub mod ListDataSource;
#[cfg(feature = "ListDisasterRecover")]
pub mod ListDisasterRecover;
#[cfg(feature = "ListDssPools")]
pub mod ListDssPools;
#[cfg(feature = "ListElbs")]
pub mod ListElbs;
#[cfg(feature = "ListEvents")]
pub mod ListEvents;
#[cfg(feature = "ListEventSpecs")]
pub mod ListEventSpecs;
#[cfg(feature = "ListEventSubs")]
pub mod ListEventSubs;
#[cfg(feature = "ListHostDisk")]
pub mod ListHostDisk;
#[cfg(feature = "ListHostNet")]
pub mod ListHostNet;
#[cfg(feature = "ListHostOverview")]
pub mod ListHostOverview;
#[cfg(feature = "ListJobDetails")]
pub mod ListJobDetails;
#[cfg(feature = "ListLogicalClusterRings")]
pub mod ListLogicalClusterRings;
#[cfg(feature = "ListLogicalClusters")]
pub mod ListLogicalClusters;
#[cfg(feature = "ListLogicalClusterTasks")]
pub mod ListLogicalClusterTasks;
#[cfg(feature = "ListLogicalClusterVolumes")]
pub mod ListLogicalClusterVolumes;
#[cfg(feature = "ListLtsLogs")]
pub mod ListLtsLogs;
#[cfg(feature = "ListMetrics")]
pub mod ListMetrics;
#[cfg(feature = "ListMetricsData")]
pub mod ListMetricsData;
#[cfg(feature = "ListMonitorIndicatorData")]
pub mod ListMonitorIndicatorData;
#[cfg(feature = "ListMonitorIndicators")]
pub mod ListMonitorIndicators;
#[cfg(feature = "ListNodeTypes")]
pub mod ListNodeTypes;
#[cfg(feature = "ListPlanExecLogs")]
pub mod ListPlanExecLogs;
#[cfg(feature = "ListQueries")]
pub mod ListQueries;
#[cfg(feature = "ListQuotas")]
pub mod ListQuotas;
#[cfg(feature = "ListSchemas")]
pub mod ListSchemas;
#[cfg(feature = "ListSnapshotCrossRegion")]
pub mod ListSnapshotCrossRegion;
#[cfg(feature = "ListSnapshotCrossRegionPolicy")]
pub mod ListSnapshotCrossRegionPolicy;
#[cfg(feature = "ListSnapshotDetails")]
pub mod ListSnapshotDetails;
#[cfg(feature = "ListSnapshotPolicy")]
pub mod ListSnapshotPolicy;
