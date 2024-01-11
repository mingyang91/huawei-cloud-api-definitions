#[cfg(feature = "AcceptVpcPeering")]
pub mod AcceptVpcPeering;
#[cfg(feature = "AddFirewallRules")]
pub mod AddFirewallRules;
#[cfg(feature = "AddSecurityGroups")]
pub mod AddSecurityGroups;
#[cfg(feature = "AddSourcesToTrafficMirrorSession")]
pub mod AddSourcesToTrafficMirrorSession;
#[cfg(feature = "AddVpcExtendCidr")]
pub mod AddVpcExtendCidr;
#[cfg(feature = "AssociateRouteTable")]
pub mod AssociateRouteTable;
#[cfg(feature = "AssociateSubnetFirewall")]
pub mod AssociateSubnetFirewall;
#[cfg(feature = "BatchCreateSecurityGroupRules")]
pub mod BatchCreateSecurityGroupRules;
#[cfg(feature = "BatchCreateSubnetTags")]
pub mod BatchCreateSubnetTags;
#[cfg(feature = "BatchCreateSubNetworkInterface")]
pub mod BatchCreateSubNetworkInterface;
#[cfg(feature = "BatchCreateVpcTags")]
pub mod BatchCreateVpcTags;
#[cfg(feature = "BatchDeleteSubnetTags")]
pub mod BatchDeleteSubnetTags;
#[cfg(feature = "BatchDeleteVpcTags")]
pub mod BatchDeleteVpcTags;
#[cfg(feature = "CreateAddressGroup")]
pub mod CreateAddressGroup;
#[cfg(feature = "CreateFirewall")]
pub mod CreateFirewall;
#[cfg(feature = "CreateFlowLog")]
pub mod CreateFlowLog;
#[cfg(feature = "CreatePort")]
pub mod CreatePort;
#[cfg(feature = "CreatePrivateip")]
pub mod CreatePrivateip;
#[cfg(feature = "CreateRouteTable")]
pub mod CreateRouteTable;
#[cfg(feature = "CreateSecurityGroup")]
pub mod CreateSecurityGroup;
#[cfg(feature = "CreateSecurityGroupRule")]
pub mod CreateSecurityGroupRule;
#[cfg(feature = "CreateSubnet")]
pub mod CreateSubnet;
#[cfg(feature = "CreateSubnetTag")]
pub mod CreateSubnetTag;
#[cfg(feature = "CreateSubNetworkInterface")]
pub mod CreateSubNetworkInterface;
#[cfg(feature = "CreateTrafficMirrorFilter")]
pub mod CreateTrafficMirrorFilter;
#[cfg(feature = "CreateTrafficMirrorFilterRule")]
pub mod CreateTrafficMirrorFilterRule;
#[cfg(feature = "CreateTrafficMirrorSession")]
pub mod CreateTrafficMirrorSession;
#[cfg(feature = "CreateVpc")]
pub mod CreateVpc;
#[cfg(feature = "CreateVpcPeering")]
pub mod CreateVpcPeering;
#[cfg(feature = "CreateVpcResourceTag")]
pub mod CreateVpcResourceTag;
#[cfg(feature = "CreateVpcRoute")]
pub mod CreateVpcRoute;
#[cfg(feature = "DeleteAddressGroup")]
pub mod DeleteAddressGroup;
#[cfg(feature = "DeleteFirewall")]
pub mod DeleteFirewall;
#[cfg(feature = "DeleteFlowLog")]
pub mod DeleteFlowLog;
#[cfg(feature = "DeleteIpAddressGroupForce")]
pub mod DeleteIpAddressGroupForce;
#[cfg(feature = "DeletePort")]
pub mod DeletePort;
#[cfg(feature = "DeletePrivateip")]
pub mod DeletePrivateip;
#[cfg(feature = "DeleteRouteTable")]
pub mod DeleteRouteTable;
#[cfg(feature = "DeleteSecurityGroup")]
pub mod DeleteSecurityGroup;
#[cfg(feature = "DeleteSecurityGroupRule")]
pub mod DeleteSecurityGroupRule;
#[cfg(feature = "DeleteSubnet")]
pub mod DeleteSubnet;
#[cfg(feature = "DeleteSubnetTag")]
pub mod DeleteSubnetTag;
#[cfg(feature = "DeleteSubNetworkInterface")]
pub mod DeleteSubNetworkInterface;
#[cfg(feature = "DeleteTrafficMirrorFilter")]
pub mod DeleteTrafficMirrorFilter;
#[cfg(feature = "DeleteTrafficMirrorFilterRule")]
pub mod DeleteTrafficMirrorFilterRule;
#[cfg(feature = "DeleteTrafficMirrorSession")]
pub mod DeleteTrafficMirrorSession;
#[cfg(feature = "DeleteVpc")]
pub mod DeleteVpc;
#[cfg(feature = "DeleteVpcPeering")]
pub mod DeleteVpcPeering;
#[cfg(feature = "DeleteVpcRoute")]
pub mod DeleteVpcRoute;
#[cfg(feature = "DeleteVpcTag")]
pub mod DeleteVpcTag;
#[cfg(feature = "DisassociateRouteTable")]
pub mod DisassociateRouteTable;
#[cfg(feature = "DisassociateSubnetFirewall")]
pub mod DisassociateSubnetFirewall;
#[cfg(feature = "ListAddressGroup")]
pub mod ListAddressGroup;
#[cfg(feature = "ListApiVersion")]
pub mod ListApiVersion;
#[cfg(feature = "ListFirewall")]
pub mod ListFirewall;
#[cfg(feature = "ListFlowLogs")]
pub mod ListFlowLogs;
#[cfg(feature = "ListPorts")]
pub mod ListPorts;
#[cfg(feature = "ListPrivateips")]
pub mod ListPrivateips;
#[cfg(feature = "ListRouteTables")]
pub mod ListRouteTables;
#[cfg(feature = "ListSecurityGroupRules")]
pub mod ListSecurityGroupRules;
#[cfg(feature = "ListSecurityGroups")]
pub mod ListSecurityGroups;
#[cfg(feature = "ListSubnets")]
pub mod ListSubnets;
#[cfg(feature = "ListSubnetsByTags")]
pub mod ListSubnetsByTags;
#[cfg(feature = "ListSubnetTags")]
pub mod ListSubnetTags;
#[cfg(feature = "ListSubNetworkInterfaces")]
pub mod ListSubNetworkInterfaces;
#[cfg(feature = "ListTrafficMirrorFilterRules")]
pub mod ListTrafficMirrorFilterRules;
#[cfg(feature = "ListTrafficMirrorFilters")]
pub mod ListTrafficMirrorFilters;
#[cfg(feature = "ListTrafficMirrorSessions")]
pub mod ListTrafficMirrorSessions;
#[cfg(feature = "ListVpcPeerings")]
pub mod ListVpcPeerings;
#[cfg(feature = "ListVpcRoutes")]
pub mod ListVpcRoutes;
#[cfg(feature = "ListVpcs")]
pub mod ListVpcs;
#[cfg(feature = "ListVpcsByTags")]
pub mod ListVpcsByTags;
#[cfg(feature = "ListVpcTags")]
pub mod ListVpcTags;
#[cfg(feature = "NeutronAddFirewallRule")]
pub mod NeutronAddFirewallRule;
#[cfg(feature = "NeutronAddRouterInterface")]
pub mod NeutronAddRouterInterface;
#[cfg(feature = "NeutronCreateFirewallGroup")]
pub mod NeutronCreateFirewallGroup;
#[cfg(feature = "NeutronCreateFirewallPolicy")]
pub mod NeutronCreateFirewallPolicy;
#[cfg(feature = "NeutronCreateFirewallRule")]
pub mod NeutronCreateFirewallRule;
#[cfg(feature = "NeutronCreateNetwork")]
pub mod NeutronCreateNetwork;
#[cfg(feature = "NeutronCreatePort")]
pub mod NeutronCreatePort;
#[cfg(feature = "NeutronCreateRouter")]
pub mod NeutronCreateRouter;
#[cfg(feature = "NeutronCreateSecurityGroup")]
pub mod NeutronCreateSecurityGroup;
#[cfg(feature = "NeutronCreateSecurityGroupRule")]
pub mod NeutronCreateSecurityGroupRule;
#[cfg(feature = "NeutronCreateSubnet")]
pub mod NeutronCreateSubnet;
#[cfg(feature = "NeutronDeleteFirewallGroup")]
pub mod NeutronDeleteFirewallGroup;
#[cfg(feature = "NeutronDeleteFirewallPolicy")]
pub mod NeutronDeleteFirewallPolicy;
#[cfg(feature = "NeutronDeleteFirewallRule")]
pub mod NeutronDeleteFirewallRule;
#[cfg(feature = "NeutronDeleteNetwork")]
pub mod NeutronDeleteNetwork;
#[cfg(feature = "NeutronDeletePort")]
pub mod NeutronDeletePort;
#[cfg(feature = "NeutronDeleteRouter")]
pub mod NeutronDeleteRouter;
#[cfg(feature = "NeutronDeleteSecurityGroup")]
pub mod NeutronDeleteSecurityGroup;
#[cfg(feature = "NeutronDeleteSecurityGroupRule")]
pub mod NeutronDeleteSecurityGroupRule;
#[cfg(feature = "NeutronDeleteSubnet")]
pub mod NeutronDeleteSubnet;
#[cfg(feature = "NeutronListFirewallGroups")]
pub mod NeutronListFirewallGroups;
#[cfg(feature = "NeutronListFirewallPolicies")]
pub mod NeutronListFirewallPolicies;
#[cfg(feature = "NeutronListFirewallRules")]
pub mod NeutronListFirewallRules;
#[cfg(feature = "NeutronListNetworks")]
pub mod NeutronListNetworks;
#[cfg(feature = "NeutronListPorts")]
pub mod NeutronListPorts;
#[cfg(feature = "NeutronListRouters")]
pub mod NeutronListRouters;
#[cfg(feature = "NeutronListSecurityGroupRules")]
pub mod NeutronListSecurityGroupRules;