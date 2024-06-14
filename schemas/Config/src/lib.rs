#[cfg(feature = "BatchCreateRemediationExceptions")]
pub mod BatchCreateRemediationExceptions;
#[cfg(feature = "BatchDeleteRemediationExceptions")]
pub mod BatchDeleteRemediationExceptions;
#[cfg(feature = "CollectAllResourcesSummary")]
pub mod CollectAllResourcesSummary;
#[cfg(feature = "CollectConformancePackComplianceSummary")]
pub mod CollectConformancePackComplianceSummary;
#[cfg(feature = "CollectTrackedResourcesSummary")]
pub mod CollectTrackedResourcesSummary;
#[cfg(feature = "CountAllResources")]
pub mod CountAllResources;
#[cfg(feature = "CountResourcesByTag")]
pub mod CountResourcesByTag;
#[cfg(feature = "CountTrackedResources")]
pub mod CountTrackedResources;
#[cfg(feature = "CreateAggregationAuthorization")]
pub mod CreateAggregationAuthorization;
#[cfg(feature = "CreateConfigurationAggregator")]
pub mod CreateConfigurationAggregator;
#[cfg(feature = "CreateConformancePack")]
pub mod CreateConformancePack;
#[cfg(feature = "CreateOrganizationConformancePack")]
pub mod CreateOrganizationConformancePack;
#[cfg(feature = "CreateOrganizationPolicyAssignment")]
pub mod CreateOrganizationPolicyAssignment;
#[cfg(feature = "CreateOrUpdateRemediationConfiguration")]
pub mod CreateOrUpdateRemediationConfiguration;
#[cfg(feature = "CreatePolicyAssignments")]
pub mod CreatePolicyAssignments;
#[cfg(feature = "CreateStoredQuery")]
pub mod CreateStoredQuery;
#[cfg(feature = "CreateTrackerConfig")]
pub mod CreateTrackerConfig;
#[cfg(feature = "DeleteAggregationAuthorization")]
pub mod DeleteAggregationAuthorization;
#[cfg(feature = "DeleteConfigurationAggregator")]
pub mod DeleteConfigurationAggregator;
#[cfg(feature = "DeleteConformancePack")]
pub mod DeleteConformancePack;
#[cfg(feature = "DeleteOrganizationConformancePack")]
pub mod DeleteOrganizationConformancePack;
#[cfg(feature = "DeleteOrganizationPolicyAssignment")]
pub mod DeleteOrganizationPolicyAssignment;
#[cfg(feature = "DeletePendingAggregationRequest")]
pub mod DeletePendingAggregationRequest;
#[cfg(feature = "DeletePolicyAssignment")]
pub mod DeletePolicyAssignment;
#[cfg(feature = "DeleteRemediationConfiguration")]
pub mod DeleteRemediationConfiguration;
#[cfg(feature = "DeleteStoredQuery")]
pub mod DeleteStoredQuery;
#[cfg(feature = "DeleteTrackerConfig")]
pub mod DeleteTrackerConfig;
#[cfg(feature = "DisablePolicyAssignment")]
pub mod DisablePolicyAssignment;
#[cfg(feature = "EnablePolicyAssignment")]
pub mod EnablePolicyAssignment;
#[cfg(feature = "ListAggregateComplianceByPolicyAssignment")]
pub mod ListAggregateComplianceByPolicyAssignment;
#[cfg(feature = "ListAggregateDiscoveredResources")]
pub mod ListAggregateDiscoveredResources;
#[cfg(feature = "ListAggregationAuthorizations")]
pub mod ListAggregationAuthorizations;
#[cfg(feature = "ListAllResources")]
pub mod ListAllResources;
#[cfg(feature = "ListAllTags")]
pub mod ListAllTags;
#[cfg(feature = "ListBuiltInConformancePackTemplates")]
pub mod ListBuiltInConformancePackTemplates;
#[cfg(feature = "ListBuiltInPolicyDefinitions")]
pub mod ListBuiltInPolicyDefinitions;
#[cfg(feature = "ListConfigurationAggregators")]
pub mod ListConfigurationAggregators;
#[cfg(feature = "ListConformancePackComplianceByPackId")]
pub mod ListConformancePackComplianceByPackId;
#[cfg(feature = "ListConformancePackComplianceDetailsByPackId")]
pub mod ListConformancePackComplianceDetailsByPackId;
#[cfg(feature = "ListConformancePackComplianceScores")]
pub mod ListConformancePackComplianceScores;
#[cfg(feature = "ListConformancePacks")]
pub mod ListConformancePacks;
#[cfg(feature = "ListOrganizationConformancePacks")]
pub mod ListOrganizationConformancePacks;
#[cfg(feature = "ListOrganizationConformancePackStatuses")]
pub mod ListOrganizationConformancePackStatuses;
#[cfg(feature = "ListOrganizationPolicyAssignments")]
pub mod ListOrganizationPolicyAssignments;
#[cfg(feature = "ListPendingAggregationRequests")]
pub mod ListPendingAggregationRequests;
#[cfg(feature = "ListPolicyAssignments")]
pub mod ListPolicyAssignments;
#[cfg(feature = "ListPolicyStatesByAssignmentId")]
pub mod ListPolicyStatesByAssignmentId;
#[cfg(feature = "ListPolicyStatesByDomainId")]
pub mod ListPolicyStatesByDomainId;
#[cfg(feature = "ListPolicyStatesByResourceId")]
pub mod ListPolicyStatesByResourceId;
#[cfg(feature = "ListProviders")]
pub mod ListProviders;
#[cfg(feature = "ListRegions")]
pub mod ListRegions;
#[cfg(feature = "ListRemediationExceptions")]
pub mod ListRemediationExceptions;
#[cfg(feature = "ListRemediationExecutionStatuses")]
pub mod ListRemediationExecutionStatuses;
#[cfg(feature = "ListResources")]
pub mod ListResources;
#[cfg(feature = "ListResourcesByTag")]
pub mod ListResourcesByTag;
#[cfg(feature = "ListSchemas")]
pub mod ListSchemas;
#[cfg(feature = "ListStoredQueries")]
pub mod ListStoredQueries;
#[cfg(feature = "ListTagsForResource")]
pub mod ListTagsForResource;
#[cfg(feature = "ListTagsForResourceType")]
pub mod ListTagsForResourceType;
#[cfg(feature = "ListTrackedResources")]
pub mod ListTrackedResources;
#[cfg(feature = "ListTrackedResourceTags")]
pub mod ListTrackedResourceTags;
#[cfg(feature = "RunAggregateResourceQuery")]
pub mod RunAggregateResourceQuery;
#[cfg(feature = "RunEvaluationByPolicyAssignmentId")]
pub mod RunEvaluationByPolicyAssignmentId;
#[cfg(feature = "RunQuery")]
pub mod RunQuery;
#[cfg(feature = "RunRemediationExecution")]
pub mod RunRemediationExecution;
#[cfg(feature = "ShowAggregateComplianceDetailsByPolicyAssignment")]
pub mod ShowAggregateComplianceDetailsByPolicyAssignment;
#[cfg(feature = "ShowAggregateDiscoveredResourceCounts")]
pub mod ShowAggregateDiscoveredResourceCounts;
#[cfg(feature = "ShowAggregatePolicyAssignmentDetail")]
pub mod ShowAggregatePolicyAssignmentDetail;
#[cfg(feature = "ShowAggregatePolicyStateComplianceSummary")]
pub mod ShowAggregatePolicyStateComplianceSummary;
#[cfg(feature = "ShowAggregateResourceConfig")]
pub mod ShowAggregateResourceConfig;
#[cfg(feature = "ShowBuiltInConformancePackTemplate")]
pub mod ShowBuiltInConformancePackTemplate;
#[cfg(feature = "ShowBuiltInPolicyDefinition")]
pub mod ShowBuiltInPolicyDefinition;
#[cfg(feature = "ShowConfigurationAggregator")]
pub mod ShowConfigurationAggregator;
#[cfg(feature = "ShowConfigurationAggregatorSourcesStatus")]
pub mod ShowConfigurationAggregatorSourcesStatus;
#[cfg(feature = "ShowConformancePack")]
pub mod ShowConformancePack;
#[cfg(feature = "ShowEvaluationStateByAssignmentId")]
pub mod ShowEvaluationStateByAssignmentId;
#[cfg(feature = "ShowOrganizationConformancePack")]
pub mod ShowOrganizationConformancePack;
#[cfg(feature = "ShowOrganizationConformancePackDetailedStatuses")]
pub mod ShowOrganizationConformancePackDetailedStatuses;
#[cfg(feature = "ShowOrganizationPolicyAssignment")]
pub mod ShowOrganizationPolicyAssignment;
#[cfg(feature = "ShowOrganizationPolicyAssignmentDetailedStatus")]
pub mod ShowOrganizationPolicyAssignmentDetailedStatus;
#[cfg(feature = "ShowOrganizationPolicyAssignmentStatuses")]
pub mod ShowOrganizationPolicyAssignmentStatuses;
#[cfg(feature = "ShowPolicyAssignment")]
pub mod ShowPolicyAssignment;
#[cfg(feature = "ShowRemediationConfiguration")]
pub mod ShowRemediationConfiguration;
#[cfg(feature = "ShowResourceById")]
pub mod ShowResourceById;
#[cfg(feature = "ShowResourceDetail")]
pub mod ShowResourceDetail;
#[cfg(feature = "ShowResourceHistory")]
pub mod ShowResourceHistory;
#[cfg(feature = "ShowResourceRelations")]
pub mod ShowResourceRelations;
#[cfg(feature = "ShowResourceRelationsDetail")]
pub mod ShowResourceRelationsDetail;
#[cfg(feature = "ShowStoredQuery")]
pub mod ShowStoredQuery;
#[cfg(feature = "ShowTrackedResourceDetail")]
pub mod ShowTrackedResourceDetail;
#[cfg(feature = "ShowTrackerConfig")]
pub mod ShowTrackerConfig;
#[cfg(feature = "TagResource")]
pub mod TagResource;
#[cfg(feature = "UnTagResource")]
pub mod UnTagResource;
#[cfg(feature = "UpdateConfigurationAggregator")]
pub mod UpdateConfigurationAggregator;
#[cfg(feature = "UpdateConformancePack")]
pub mod UpdateConformancePack;
#[cfg(feature = "UpdateOrganizationConformancePack")]
pub mod UpdateOrganizationConformancePack;
#[cfg(feature = "UpdateOrganizationPolicyAssignment")]
pub mod UpdateOrganizationPolicyAssignment;
#[cfg(feature = "UpdatePolicyAssignment")]
pub mod UpdatePolicyAssignment;
#[cfg(feature = "UpdatePolicyState")]
pub mod UpdatePolicyState;
#[cfg(feature = "UpdateStoredQuery")]
pub mod UpdateStoredQuery;
