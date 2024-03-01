#[cfg(feature = "BatchCreateOrDeleteKafkaTag")]
pub mod BatchCreateOrDeleteKafkaTag;
#[cfg(feature = "BatchDeleteGroup")]
pub mod BatchDeleteGroup;
#[cfg(feature = "BatchDeleteInstanceTopic")]
pub mod BatchDeleteInstanceTopic;
#[cfg(feature = "BatchDeleteInstanceUsers")]
pub mod BatchDeleteInstanceUsers;
#[cfg(feature = "BatchRestartOrDeleteInstances")]
pub mod BatchRestartOrDeleteInstances;
#[cfg(feature = "CloseKafkaManager")]
pub mod CloseKafkaManager;
#[cfg(feature = "CreateConnector")]
pub mod CreateConnector;
#[cfg(feature = "CreateConnectorTask")]
pub mod CreateConnectorTask;
#[cfg(feature = "CreateDeleteConnectorOrder")]
pub mod CreateDeleteConnectorOrder;
#[cfg(feature = "CreateInstanceByEngine")]
pub mod CreateInstanceByEngine;
#[cfg(feature = "CreateInstanceTopic")]
pub mod CreateInstanceTopic;
#[cfg(feature = "CreateInstanceUser")]
pub mod CreateInstanceUser;
#[cfg(feature = "CreateKafkaConsumerGroup")]
pub mod CreateKafkaConsumerGroup;
#[cfg(feature = "CreateKafkaUserClientQuotaTask")]
pub mod CreateKafkaUserClientQuotaTask;
#[cfg(feature = "CreateReassignmentTask")]
pub mod CreateReassignmentTask;
#[cfg(feature = "CreateSinkTask")]
pub mod CreateSinkTask;
#[cfg(feature = "DeleteBackgroundTask")]
pub mod DeleteBackgroundTask;
#[cfg(feature = "DeleteConnector")]
pub mod DeleteConnector;
#[cfg(feature = "DeleteConnectorTask")]
pub mod DeleteConnectorTask;
#[cfg(feature = "DeleteInstance")]
pub mod DeleteInstance;
#[cfg(feature = "DeleteKafkaUserClientQuotaTask")]
pub mod DeleteKafkaUserClientQuotaTask;
#[cfg(feature = "DeleteSinkTask")]
pub mod DeleteSinkTask;
#[cfg(feature = "ListAvailableZones")]
pub mod ListAvailableZones;
#[cfg(feature = "ListBackgroundTasks")]
pub mod ListBackgroundTasks;
#[cfg(feature = "ListConnectorTasks")]
pub mod ListConnectorTasks;
#[cfg(feature = "ListEngineProducts")]
pub mod ListEngineProducts;
#[cfg(feature = "ListInstanceConsumerGroups")]
pub mod ListInstanceConsumerGroups;
#[cfg(feature = "ListInstances")]
pub mod ListInstances;
#[cfg(feature = "ListInstanceTopics")]
pub mod ListInstanceTopics;
#[cfg(feature = "ListSinkTasks")]
pub mod ListSinkTasks;
#[cfg(feature = "ListTopicPartitions")]
pub mod ListTopicPartitions;
#[cfg(feature = "ListTopicProducers")]
pub mod ListTopicProducers;
#[cfg(feature = "ModifyInstanceConfigs")]
pub mod ModifyInstanceConfigs;
#[cfg(feature = "PauseConnectorTask")]
pub mod PauseConnectorTask;
#[cfg(feature = "ResetManagerPassword")]
pub mod ResetManagerPassword;
#[cfg(feature = "ResetMessageOffset")]
pub mod ResetMessageOffset;
#[cfg(feature = "ResetMessageOffsetWithEngine")]
pub mod ResetMessageOffsetWithEngine;
#[cfg(feature = "ResetPassword")]
pub mod ResetPassword;
#[cfg(feature = "ResetUserPasswrod")]
pub mod ResetUserPasswrod;
#[cfg(feature = "ResizeEngineInstance")]
pub mod ResizeEngineInstance;
#[cfg(feature = "RestartConnectorTask")]
pub mod RestartConnectorTask;
#[cfg(feature = "RestartManager")]
pub mod RestartManager;
#[cfg(feature = "ResumeConnectorTask")]
pub mod ResumeConnectorTask;
#[cfg(feature = "SendKafkaMessage")]
pub mod SendKafkaMessage;
#[cfg(feature = "ShowBackgroundTask")]
pub mod ShowBackgroundTask;
#[cfg(feature = "ShowCesHierarchy")]
pub mod ShowCesHierarchy;
#[cfg(feature = "ShowCluster")]
pub mod ShowCluster;
#[cfg(feature = "ShowConnectorTask")]
pub mod ShowConnectorTask;
#[cfg(feature = "ShowCoordinators")]
pub mod ShowCoordinators;
#[cfg(feature = "ShowEngineInstanceExtendProductInfo")]
pub mod ShowEngineInstanceExtendProductInfo;
#[cfg(feature = "ShowGroups")]
pub mod ShowGroups;
#[cfg(feature = "ShowInstance")]
pub mod ShowInstance;
#[cfg(feature = "ShowInstanceConfigs")]
pub mod ShowInstanceConfigs;
#[cfg(feature = "ShowInstanceMessages")]
pub mod ShowInstanceMessages;
#[cfg(feature = "ShowInstanceTopicDetail")]
pub mod ShowInstanceTopicDetail;
#[cfg(feature = "ShowInstanceUsers")]
pub mod ShowInstanceUsers;
#[cfg(feature = "ShowKafkaProjectTags")]
pub mod ShowKafkaProjectTags;
#[cfg(feature = "ShowKafkaTags")]
pub mod ShowKafkaTags;
#[cfg(feature = "ShowKafkaTopicPartitionDiskusage")]
pub mod ShowKafkaTopicPartitionDiskusage;
#[cfg(feature = "ShowKafkaUserClientQuota")]
pub mod ShowKafkaUserClientQuota;
#[cfg(feature = "ShowMaintainWindows")]
pub mod ShowMaintainWindows;
#[cfg(feature = "ShowMessages")]
pub mod ShowMessages;
#[cfg(feature = "ShowPartitionBeginningMessage")]
pub mod ShowPartitionBeginningMessage;
#[cfg(feature = "ShowPartitionEndMessage")]
pub mod ShowPartitionEndMessage;
#[cfg(feature = "ShowPartitionMessage")]
pub mod ShowPartitionMessage;
#[cfg(feature = "ShowSinkTaskDetail")]
pub mod ShowSinkTaskDetail;
#[cfg(feature = "ShowTopicAccessPolicy")]
pub mod ShowTopicAccessPolicy;
#[cfg(feature = "UpdateInstance")]
pub mod UpdateInstance;
#[cfg(feature = "UpdateInstanceAutoCreateTopic")]
pub mod UpdateInstanceAutoCreateTopic;
#[cfg(feature = "UpdateInstanceConsumerGroup")]
pub mod UpdateInstanceConsumerGroup;
#[cfg(feature = "UpdateInstanceCrossVpcIp")]
pub mod UpdateInstanceCrossVpcIp;
#[cfg(feature = "UpdateInstanceTopic")]
pub mod UpdateInstanceTopic;
#[cfg(feature = "UpdateInstanceUser")]
pub mod UpdateInstanceUser;
#[cfg(feature = "UpdateKafkaUserClientQuotaTask")]
pub mod UpdateKafkaUserClientQuotaTask;
#[cfg(feature = "UpdateSinkTaskQuota")]
pub mod UpdateSinkTaskQuota;
#[cfg(feature = "UpdateTopicAccessPolicy")]
pub mod UpdateTopicAccessPolicy;
#[cfg(feature = "UpdateTopicReplica")]
pub mod UpdateTopicReplica;
