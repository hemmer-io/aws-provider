# Network_firewall Service



**Resources**: 20

---

## Overview

The network_firewall service provides access to 20 resource types:

- [Rule_group](#rule_group) [CRUD]
- [Firewall_policy_change_protection](#firewall_policy_change_protection) [U]
- [Analysis_report_results](#analysis_report_results) [R]
- [Subnet_change_protection](#subnet_change_protection) [U]
- [Resource_policy](#resource_policy) [CRD]
- [Vpc_endpoint_association](#vpc_endpoint_association) [CRD]
- [Firewall](#firewall) [CRD]
- [Tls_inspection_configuration](#tls_inspection_configuration) [CRUD]
- [Firewall_encryption_configuration](#firewall_encryption_configuration) [U]
- [Rule_group_summary](#rule_group_summary) [R]
- [Network_firewall_transit_gateway_attachment](#network_firewall_transit_gateway_attachment) [D]
- [Availability_zone_change_protection](#availability_zone_change_protection) [U]
- [Firewall_policy](#firewall_policy) [CRUD]
- [Firewall_analysis_settings](#firewall_analysis_settings) [U]
- [Firewall_metadata](#firewall_metadata) [R]
- [Firewall_description](#firewall_description) [U]
- [Rule_group_metadata](#rule_group_metadata) [R]
- [Flow_operation](#flow_operation) [R]
- [Firewall_delete_protection](#firewall_delete_protection) [U]
- [Logging_configuration](#logging_configuration) [RU]

---

## Resources


### Rule_group

RuleGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rule_group` | String |  | <p>An object that defines the rule group rules. </p>
         <note>
            <p>You must provide either this rule group setting or a <code>Rules</code> setting, but not both. </p>
         </note> |
| `capacity` | i64 | ✅ | <p>The maximum operating resources that this rule group can use. Rule group capacity is fixed at creation.
      When you update a rule group, you are limited to this capacity. When you reference a rule group
      from a firewall policy, Network Firewall reserves this capacity for the rule group. </p>
         <p>You can retrieve the capacity that would be required for a rule group before you create the rule group by calling
      <a>CreateRuleGroup</a> with <code>DryRun</code> set to <code>TRUE</code>. </p>
         <note>
            <p>You can't change or exceed this capacity when you update the rule group, so leave
            room for your rule group to grow. </p>
         </note>
         <p>
            <b>Capacity for a stateless rule group</b>
         </p>
         <p>For a stateless rule group, the capacity required is the sum of the capacity
         requirements of the individual rules that you expect to have in the rule group. </p>
         <p>To calculate the capacity requirement of a single rule, multiply the capacity
         requirement values of each of the rule's match settings:</p>
         <ul>
            <li>
               <p>A match setting with no criteria specified has a value of 1. </p>
            </li>
            <li>
               <p>A match setting with <code>Any</code> specified has a value of 1. </p>
            </li>
            <li>
               <p>All other match settings have a value equal to the number of elements provided in
               the setting. For example, a protocol setting ["UDP"] and a source setting
               ["10.0.0.0/24"] each have a value of 1. A protocol setting ["UDP","TCP"] has a value
               of 2. A source setting ["10.0.0.0/24","10.0.0.1/24","10.0.0.2/24"] has a value of 3.
            </p>
            </li>
         </ul>
         <p>A rule with no criteria specified in any of its match settings has a capacity
         requirement of 1. A rule with protocol setting ["UDP","TCP"], source setting
         ["10.0.0.0/24","10.0.0.1/24","10.0.0.2/24"], and a single specification or no specification
         for each of the other match settings has a capacity requirement of 6. </p>
         <p>
            <b>Capacity for a stateful rule group</b>
         </p>
         <p>For a stateful rule group, the minimum capacity required is the number of individual rules that
         you expect to have in the rule group. </p> |
| `dry_run` | bool |  | <p>Indicates whether you want Network Firewall to just check the validity of the request, rather than run the request. </p>
         <p>If set to <code>TRUE</code>, Network Firewall checks whether the request can run successfully,
         but doesn't actually make the requested changes. The call returns the value that the request would return if you ran it with
         dry run set to <code>FALSE</code>, but doesn't make additions or changes to your resources. This option allows you to make sure that you have
         the required permissions to run the request and that your request parameters are valid. </p>
         <p>If set to <code>FALSE</code>, Network Firewall makes the requested changes to your resources. </p> |
| `tags` | Vec<String> |  | <p>The key:value pairs to associate with the resource.</p> |
| `encryption_configuration` | String |  | <p>A complex type that contains settings for encryption of your rule group resources.</p> |
| `rule_group_name` | String | ✅ | <p>The descriptive name of the rule group. You can't change the name of a rule group after you create it.</p> |
| `source_metadata` | String |  | <p>A complex type that contains metadata about the rule group that your own rule group is copied from. You can use the metadata to keep track of updates made to the originating rule group.</p> |
| `rules` | String |  | <p>A string containing stateful rule group rules specifications in Suricata flat format, with one rule
per line. Use this to import your existing Suricata compatible rule groups. </p>
         <note>
            <p>You must provide either this rules setting or a populated <code>RuleGroup</code> setting, but not both. </p>
         </note>
         <p>You can provide your rule group specification in Suricata flat format through this setting when you create or update your rule group. The call
response returns a <a>RuleGroup</a> object that Network Firewall has populated from your string. </p> |
| `description` | String |  | <p>A description of the rule group. </p> |
| `summary_configuration` | String |  | <p>An object that contains a <code>RuleOptions</code> array of strings. 
         You use <code>RuleOptions</code> to determine which of the following <a>RuleSummary</a> values are returned in response to <code>DescribeRuleGroupSummary</code>.</p>
         <ul>
            <li>
               <p>
                  <code>Metadata</code> - returns</p>
            </li>
            <li>
               <p>
                  <code>Msg</code>
               </p>
            </li>
            <li>
               <p>
                  <code>SID</code>
               </p>
            </li>
         </ul> |
| `analyze_rule_group` | bool |  | <p>Indicates whether you want Network Firewall to analyze the stateless rules in the rule group for rule behavior such as asymmetric routing. If set to <code>TRUE</code>, Network Firewall runs the analysis and then creates the rule group for you. To run the stateless rule group analyzer without creating the rule group, set <code>DryRun</code> to <code>TRUE</code>.</p> |
| `type` | String | ✅ | <p>Indicates whether the rule group is stateless or stateful. If the rule group is stateless, it contains
stateless rules. If it is stateful, it contains stateful rules. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rule_group_response` | String | <p>The high-level properties of a rule group. This, along with the <a>RuleGroup</a>, define the rule group. You can retrieve all objects for a rule group by calling <a>DescribeRuleGroup</a>. </p> |
| `update_token` | String | <p>A token used for optimistic locking. Network Firewall returns a token to your requests that access the rule group. The token marks the state of the rule group resource at the time of the request. </p>
         <p>To make changes to the rule group, you provide the token in your request. Network Firewall uses the token to ensure that the rule group hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the rule group again to get a current copy of it with a current token. Reapply your changes as needed, then try the operation again using the new token. </p> |
| `rule_group` | String | <p>The object that defines the rules in a rule group. This, along with <a>RuleGroupResponse</a>, define the rule group. You can retrieve all objects for a rule group by calling <a>DescribeRuleGroup</a>. </p>
         <p>Network Firewall uses a rule group to inspect and control network traffic.
    You define stateless rule groups to inspect individual packets and you define stateful rule groups to inspect packets in the context of their
    traffic flow. </p>
         <p>To use a rule group, you include it by reference in an Network Firewall firewall policy, then you use the policy in a firewall. You can reference a rule group from
    more than one firewall policy, and you can use a firewall policy in more than one firewall. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create rule_group
rule_group = provider.network_firewall.Rule_group {
    capacity = "value"  # <p>The maximum operating resources that this rule group can use. Rule group capacity is fixed at creation.
      When you update a rule group, you are limited to this capacity. When you reference a rule group
      from a firewall policy, Network Firewall reserves this capacity for the rule group. </p>
         <p>You can retrieve the capacity that would be required for a rule group before you create the rule group by calling
      <a>CreateRuleGroup</a> with <code>DryRun</code> set to <code>TRUE</code>. </p>
         <note>
            <p>You can't change or exceed this capacity when you update the rule group, so leave
            room for your rule group to grow. </p>
         </note>
         <p>
            <b>Capacity for a stateless rule group</b>
         </p>
         <p>For a stateless rule group, the capacity required is the sum of the capacity
         requirements of the individual rules that you expect to have in the rule group. </p>
         <p>To calculate the capacity requirement of a single rule, multiply the capacity
         requirement values of each of the rule's match settings:</p>
         <ul>
            <li>
               <p>A match setting with no criteria specified has a value of 1. </p>
            </li>
            <li>
               <p>A match setting with <code>Any</code> specified has a value of 1. </p>
            </li>
            <li>
               <p>All other match settings have a value equal to the number of elements provided in
               the setting. For example, a protocol setting ["UDP"] and a source setting
               ["10.0.0.0/24"] each have a value of 1. A protocol setting ["UDP","TCP"] has a value
               of 2. A source setting ["10.0.0.0/24","10.0.0.1/24","10.0.0.2/24"] has a value of 3.
            </p>
            </li>
         </ul>
         <p>A rule with no criteria specified in any of its match settings has a capacity
         requirement of 1. A rule with protocol setting ["UDP","TCP"], source setting
         ["10.0.0.0/24","10.0.0.1/24","10.0.0.2/24"], and a single specification or no specification
         for each of the other match settings has a capacity requirement of 6. </p>
         <p>
            <b>Capacity for a stateful rule group</b>
         </p>
         <p>For a stateful rule group, the minimum capacity required is the number of individual rules that
         you expect to have in the rule group. </p>
    rule_group_name = "value"  # <p>The descriptive name of the rule group. You can't change the name of a rule group after you create it.</p>
    type = "value"  # <p>Indicates whether the rule group is stateless or stateful. If the rule group is stateless, it contains
stateless rules. If it is stateful, it contains stateful rules. </p>
}

# Access rule_group outputs
rule_group_id = rule_group.id
rule_group_rule_group_response = rule_group.rule_group_response
rule_group_update_token = rule_group.update_token
rule_group_rule_group = rule_group.rule_group
```

---


### Firewall_policy_change_protection

FirewallPolicyChangeProtection resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_token` | String |  | <p>An optional token that you can use for optimistic locking. Network Firewall returns a token to your requests that access the firewall. The token marks the state of the firewall resource at the time of the request. </p>
         <p>To make an unconditional change to the firewall, omit the token in your update request. Without the token, Network Firewall performs your updates regardless of whether the firewall has changed since you last retrieved it.</p>
         <p>To make a conditional change to the firewall, provide the token in your update request. Network Firewall uses the token to ensure that the firewall hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the firewall again to get a current copy of it with a new token. Reapply your changes as needed, then try the operation again using the new token. </p> |
| `firewall_arn` | String |  | <p>The Amazon Resource Name (ARN) of the firewall.</p>
         <p>You must specify the ARN or the name, and you can specify both. </p> |
| `firewall_name` | String |  | <p>The descriptive name of the firewall. You can't change the name of a firewall after you create it.</p>
         <p>You must specify the ARN or the name, and you can specify both. </p> |
| `firewall_policy_change_protection` | bool | ✅ | <p>A setting indicating whether the firewall is protected against a change to the firewall policy association.
         Use this setting to protect against
         accidentally modifying the firewall policy for a firewall that is in use. When you create a firewall, the operation initializes this setting to <code>TRUE</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Analysis_report_results

AnalysisReportResults resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `analysis_type` | String | <p>The type of traffic that will be used to generate a report. </p> |
| `end_time` | String | <p>The date and time, up to the current date, from which to stop retrieving analysis data, 
   in UTC format (for example, <code>YYYY-MM-DDTHH:MM:SSZ</code>). </p> |
| `next_token` | String | <p>When you request a list of objects with a <code>MaxResults</code> setting, if the number of objects that are still available
         for retrieval exceeds the maximum you requested, Network Firewall returns a <code>NextToken</code>
         value in the response. To retrieve the next batch of objects, use the token returned from the prior request in your next request.</p> |
| `status` | String | <p>The status of the analysis report you specify. Statuses include <code>RUNNING</code>, <code>COMPLETED</code>, or <code>FAILED</code>.</p> |
| `report_time` | String | <p>The date and time the analysis report was ran. </p> |
| `analysis_report_results` | Vec<String> | <p>Retrieves the results of a traffic analysis report.</p> |
| `start_time` | String | <p> The date and time within the last 30 days from which to start retrieving analysis data, 
   in UTC format (for example, <code>YYYY-MM-DDTHH:MM:SSZ</code>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access analysis_report_results outputs
analysis_report_results_id = analysis_report_results.id
analysis_report_results_analysis_type = analysis_report_results.analysis_type
analysis_report_results_end_time = analysis_report_results.end_time
analysis_report_results_next_token = analysis_report_results.next_token
analysis_report_results_status = analysis_report_results.status
analysis_report_results_report_time = analysis_report_results.report_time
analysis_report_results_analysis_report_results = analysis_report_results.analysis_report_results
analysis_report_results_start_time = analysis_report_results.start_time
```

---


### Subnet_change_protection

SubnetChangeProtection resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `firewall_name` | String |  | <p>The descriptive name of the firewall. You can't change the name of a firewall after you create it.</p>
         <p>You must specify the ARN or the name, and you can specify both. </p> |
| `subnet_change_protection` | bool | ✅ | <p>A setting indicating whether the firewall is protected against changes to the subnet associations.
         Use this setting to protect against
         accidentally modifying the subnet associations for a firewall that is in use. When you create a firewall, the operation initializes this setting to <code>TRUE</code>.</p> |
| `update_token` | String |  | <p>An optional token that you can use for optimistic locking. Network Firewall returns a token to your requests that access the firewall. The token marks the state of the firewall resource at the time of the request. </p>
         <p>To make an unconditional change to the firewall, omit the token in your update request. Without the token, Network Firewall performs your updates regardless of whether the firewall has changed since you last retrieved it.</p>
         <p>To make a conditional change to the firewall, provide the token in your update request. Network Firewall uses the token to ensure that the firewall hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the firewall again to get a current copy of it with a new token. Reapply your changes as needed, then try the operation again using the new token. </p> |
| `firewall_arn` | String |  | <p>The Amazon Resource Name (ARN) of the firewall.</p>
         <p>You must specify the ARN or the name, and you can specify both. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String | ✅ | <p>The IAM policy statement that lists the accounts that you want to share your Network Firewall resources with
           and the operations that you want the accounts to be able to perform. </p>
         <p>For a rule group resource, you can specify the following operations in the Actions section of the statement:</p>
         <ul>
            <li>
               <p>network-firewall:CreateFirewallPolicy</p>
            </li>
            <li>
               <p>network-firewall:UpdateFirewallPolicy</p>
            </li>
            <li>
               <p>network-firewall:ListRuleGroups</p>
            </li>
         </ul>
         <p>For a firewall policy resource, you can specify the following operations in the Actions section of the statement:</p>
         <ul>
            <li>
               <p>network-firewall:AssociateFirewallPolicy</p>
            </li>
            <li>
               <p>network-firewall:ListFirewallPolicies</p>
            </li>
         </ul>
         <p>For a firewall resource, you can specify the following operations in the Actions section of the statement:</p>
         <ul>
            <li>
               <p>network-firewall:CreateVpcEndpointAssociation</p>
            </li>
            <li>
               <p>network-firewall:DescribeFirewallMetadata</p>
            </li>
            <li>
               <p>network-firewall:ListFirewalls</p>
            </li>
         </ul>
         <p>In the Resource section of the statement, you specify the ARNs for the Network Firewall resources that you want to share with the account that you specified in <code>Arn</code>.</p> |
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the account that you want to share your Network Firewall resources with.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The IAM policy for the resource. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.network_firewall.Resource_policy {
    policy = "value"  # <p>The IAM policy statement that lists the accounts that you want to share your Network Firewall resources with
           and the operations that you want the accounts to be able to perform. </p>
         <p>For a rule group resource, you can specify the following operations in the Actions section of the statement:</p>
         <ul>
            <li>
               <p>network-firewall:CreateFirewallPolicy</p>
            </li>
            <li>
               <p>network-firewall:UpdateFirewallPolicy</p>
            </li>
            <li>
               <p>network-firewall:ListRuleGroups</p>
            </li>
         </ul>
         <p>For a firewall policy resource, you can specify the following operations in the Actions section of the statement:</p>
         <ul>
            <li>
               <p>network-firewall:AssociateFirewallPolicy</p>
            </li>
            <li>
               <p>network-firewall:ListFirewallPolicies</p>
            </li>
         </ul>
         <p>For a firewall resource, you can specify the following operations in the Actions section of the statement:</p>
         <ul>
            <li>
               <p>network-firewall:CreateVpcEndpointAssociation</p>
            </li>
            <li>
               <p>network-firewall:DescribeFirewallMetadata</p>
            </li>
            <li>
               <p>network-firewall:ListFirewalls</p>
            </li>
         </ul>
         <p>In the Resource section of the statement, you specify the ARNs for the Network Firewall resources that you want to share with the account that you specified in <code>Arn</code>.</p>
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the account that you want to share your Network Firewall resources with.</p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_policy = resource_policy.policy
```

---


### Vpc_endpoint_association

VpcEndpointAssociation resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description of the VPC endpoint association. </p> |
| `firewall_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the firewall.</p> |
| `tags` | Vec<String> |  | <p>The key:value pairs to associate with the resource.</p> |
| `vpc_id` | String | ✅ | <p>The unique identifier of the VPC where you want to create a firewall endpoint. </p> |
| `subnet_mapping` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpc_endpoint_association` | String | <p>The configuration settings for the VPC endpoint association. These settings include the firewall and the VPC and subnet to use for the firewall endpoint. </p> |
| `vpc_endpoint_association_status` | String | <p>Detailed information about the current status of a <a>VpcEndpointAssociation</a>. You can retrieve this 
by calling <a>DescribeVpcEndpointAssociation</a> and providing the VPC endpoint association ARN.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_endpoint_association
vpc_endpoint_association = provider.network_firewall.Vpc_endpoint_association {
    firewall_arn = "value"  # <p>The Amazon Resource Name (ARN) of the firewall.</p>
    vpc_id = "value"  # <p>The unique identifier of the VPC where you want to create a firewall endpoint. </p>
    subnet_mapping = "value"  # Required field
}

# Access vpc_endpoint_association outputs
vpc_endpoint_association_id = vpc_endpoint_association.id
vpc_endpoint_association_vpc_endpoint_association = vpc_endpoint_association.vpc_endpoint_association
vpc_endpoint_association_vpc_endpoint_association_status = vpc_endpoint_association.vpc_endpoint_association_status
```

---


### Firewall

Firewall resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `firewall_name` | String | ✅ | <p>The descriptive name of the firewall. You can't change the name of a firewall after you create it.</p> |
| `subnet_mappings` | Vec<String> |  | <p>The public subnets to use for your Network Firewall firewalls. Each subnet must belong to a
         different Availability Zone in the VPC. Network Firewall creates a firewall endpoint in each
         subnet. </p> |
| `vpc_id` | String |  | <p>The unique identifier of the VPC where Network Firewall should create the firewall. </p>
         <p>You can't change this setting after you create the firewall. </p> |
| `enabled_analysis_types` | Vec<String> |  | <p>An optional setting indicating the specific traffic analysis types to enable on the firewall. </p> |
| `delete_protection` | bool |  | <p>A flag indicating whether it is possible to delete the firewall. A setting of <code>TRUE</code> indicates
         that the firewall is protected against deletion. Use this setting to protect against
         accidentally deleting a firewall that is in use. When you create a firewall, the operation initializes this flag to <code>TRUE</code>.</p> |
| `firewall_policy_change_protection` | bool |  | <p>A setting indicating whether the firewall is protected against a change to the firewall policy association.
         Use this setting to protect against
         accidentally modifying the firewall policy for a firewall that is in use. When you create a firewall, the operation initializes this setting to <code>TRUE</code>.</p> |
| `firewall_policy_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the <a>FirewallPolicy</a> that you want to
         use for the firewall.</p> |
| `subnet_change_protection` | bool |  | <p>A setting indicating whether the firewall is protected against changes to the subnet associations.
         Use this setting to protect against
         accidentally modifying the subnet associations for a firewall that is in use. When you create a firewall, the operation initializes this setting to <code>TRUE</code>.</p> |
| `encryption_configuration` | String |  | <p>A complex type that contains settings for encryption of your firewall resources.</p> |
| `availability_zone_mappings` | Vec<String> |  | <p>Required. The Availability Zones where you want to create firewall endpoints for a transit gateway-attached firewall. You must specify at least one Availability Zone. Consider enabling the firewall in every Availability Zone where you have workloads to maintain Availability Zone isolation.</p>
         <p>You can modify Availability Zones later using <a>AssociateAvailabilityZones</a> or <a>DisassociateAvailabilityZones</a>, but this may briefly disrupt traffic. The <code>AvailabilityZoneChangeProtection</code> setting controls whether you can make these modifications.</p> |
| `tags` | Vec<String> |  | <p>The key:value pairs to associate with the resource.</p> |
| `transit_gateway_id` | String |  | <p>Required when creating a transit gateway-attached firewall. The unique identifier of the transit gateway to attach to this firewall. You can provide either a transit gateway from your account or one that has been shared with you through Resource Access Manager.</p>
         <important>
            <p>After creating the firewall, you cannot change the transit gateway association. To use a different transit gateway, you must create a new firewall.</p>
         </important>
         <p>For information about creating firewalls, see <a>CreateFirewall</a>. For specific guidance about transit gateway-attached firewalls, see <a href="https://docs.aws.amazon.com/network-firewall/latest/developerguide/tgw-firewall-considerations.html">Considerations for transit gateway-attached firewalls</a> in the <i>Network Firewall Developer Guide</i>.</p> |
| `availability_zone_change_protection` | bool |  | <p>Optional. A setting indicating whether the firewall is protected against changes to its Availability Zone configuration. When set to <code>TRUE</code>, you cannot add or remove Availability Zones without first disabling this protection using <a>UpdateAvailabilityZoneChangeProtection</a>.</p>
         <p>Default value: <code>FALSE</code>
         </p> |
| `description` | String |  | <p>A description of the firewall.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_token` | String | <p>An optional token that you can use for optimistic locking. Network Firewall returns a token to your requests that access the firewall. The token marks the state of the firewall resource at the time of the request. </p>
         <p>To make an unconditional change to the firewall, omit the token in your update request. Without the token, Network Firewall performs your updates regardless of whether the firewall has changed since you last retrieved it.</p>
         <p>To make a conditional change to the firewall, provide the token in your update request. Network Firewall uses the token to ensure that the firewall hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the firewall again to get a current copy of it with a new token. Reapply your changes as needed, then try the operation again using the new token. </p> |
| `firewall` | String | <p>The configuration settings for the firewall. These settings include the firewall policy and the subnets in your VPC to use for the firewall endpoints. </p> |
| `firewall_status` | String | <p>Detailed information about the current status of a <a>Firewall</a>. You can retrieve this for a firewall by calling <a>DescribeFirewall</a> and providing the firewall name and ARN.</p>
         <p>The firewall status indicates a combined status. It indicates whether all subnets are up-to-date with the latest firewall configurations, which is based on the sync states config values, and also whether all subnets have their endpoints fully enabled, based on their sync states attachment values. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create firewall
firewall = provider.network_firewall.Firewall {
    firewall_name = "value"  # <p>The descriptive name of the firewall. You can't change the name of a firewall after you create it.</p>
    firewall_policy_arn = "value"  # <p>The Amazon Resource Name (ARN) of the <a>FirewallPolicy</a> that you want to
         use for the firewall.</p>
}

# Access firewall outputs
firewall_id = firewall.id
firewall_update_token = firewall.update_token
firewall_firewall = firewall.firewall
firewall_firewall_status = firewall.firewall_status
```

---


### Tls_inspection_configuration

TLSInspectionConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description of the TLS inspection configuration. </p> |
| `encryption_configuration` | String |  |  |
| `tls_inspection_configuration` | String | ✅ | <p>The object that defines a TLS inspection configuration. This, along with <a>TLSInspectionConfigurationResponse</a>, define the TLS inspection configuration. You can retrieve all objects for a TLS inspection configuration by calling <a>DescribeTLSInspectionConfiguration</a>. </p>
         <p>Network Firewall uses a TLS inspection configuration to decrypt traffic. Network Firewall re-encrypts the traffic before sending it to its destination.</p>
         <p>To use a TLS inspection configuration, you add it to a new Network Firewall firewall policy, then you apply the firewall policy to a firewall. Network Firewall acts as a proxy service to decrypt and inspect the traffic traveling through your firewalls. You can reference a TLS inspection configuration from more than one firewall policy, and you can use a firewall policy in more than one firewall. For more information about using TLS inspection configurations, see 
    <a href="https://docs.aws.amazon.com/network-firewall/latest/developerguide/tls-inspection.html">Inspecting SSL/TLS traffic with TLS
inspection configurations</a> in the <i>Network Firewall Developer Guide</i>.</p> |
| `tls_inspection_configuration_name` | String | ✅ | <p>The descriptive name of the TLS inspection configuration. You can't change the name of a TLS inspection configuration after you create it.</p> |
| `tags` | Vec<String> |  | <p>The key:value pairs to associate with the resource.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tls_inspection_configuration` | String | <p>The object that defines a TLS inspection configuration. This, along with <a>TLSInspectionConfigurationResponse</a>, define the TLS inspection configuration. You can retrieve all objects for a TLS inspection configuration by calling <a>DescribeTLSInspectionConfiguration</a>. </p>
         <p>Network Firewall uses a TLS inspection configuration to decrypt traffic. Network Firewall re-encrypts the traffic before sending it to its destination.</p>
         <p>To use a TLS inspection configuration, you add it to a new Network Firewall firewall policy, then you apply the firewall policy to a firewall. Network Firewall acts as a proxy service to decrypt and inspect the traffic traveling through your firewalls. You can reference a TLS inspection configuration from more than one firewall policy, and you can use a firewall policy in more than one firewall. For more information about using TLS inspection configurations, see 
    <a href="https://docs.aws.amazon.com/network-firewall/latest/developerguide/tls-inspection.html">Inspecting SSL/TLS traffic with TLS
inspection configurations</a> in the <i>Network Firewall Developer Guide</i>.</p> |
| `tls_inspection_configuration_response` | String | <p>The high-level properties of a TLS inspection configuration. This, along with the <a>TLSInspectionConfiguration</a>, define the TLS inspection configuration. You can retrieve all objects for a TLS inspection configuration by calling <a>DescribeTLSInspectionConfiguration</a>. </p> |
| `update_token` | String | <p>A token used for optimistic locking. Network Firewall returns a token to your requests that access the TLS inspection configuration. The token marks the state of the TLS inspection configuration resource at the time of the request. </p>
         <p>To make changes to the TLS inspection configuration, you provide the token in your request. Network Firewall uses the token to ensure that the TLS inspection configuration hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the TLS inspection configuration again to get a current copy of it with a current token. Reapply your changes as needed, then try the operation again using the new token. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create tls_inspection_configuration
tls_inspection_configuration = provider.network_firewall.Tls_inspection_configuration {
    tls_inspection_configuration = "value"  # <p>The object that defines a TLS inspection configuration. This, along with <a>TLSInspectionConfigurationResponse</a>, define the TLS inspection configuration. You can retrieve all objects for a TLS inspection configuration by calling <a>DescribeTLSInspectionConfiguration</a>. </p>
         <p>Network Firewall uses a TLS inspection configuration to decrypt traffic. Network Firewall re-encrypts the traffic before sending it to its destination.</p>
         <p>To use a TLS inspection configuration, you add it to a new Network Firewall firewall policy, then you apply the firewall policy to a firewall. Network Firewall acts as a proxy service to decrypt and inspect the traffic traveling through your firewalls. You can reference a TLS inspection configuration from more than one firewall policy, and you can use a firewall policy in more than one firewall. For more information about using TLS inspection configurations, see 
    <a href="https://docs.aws.amazon.com/network-firewall/latest/developerguide/tls-inspection.html">Inspecting SSL/TLS traffic with TLS
inspection configurations</a> in the <i>Network Firewall Developer Guide</i>.</p>
    tls_inspection_configuration_name = "value"  # <p>The descriptive name of the TLS inspection configuration. You can't change the name of a TLS inspection configuration after you create it.</p>
}

# Access tls_inspection_configuration outputs
tls_inspection_configuration_id = tls_inspection_configuration.id
tls_inspection_configuration_tls_inspection_configuration = tls_inspection_configuration.tls_inspection_configuration
tls_inspection_configuration_tls_inspection_configuration_response = tls_inspection_configuration.tls_inspection_configuration_response
tls_inspection_configuration_update_token = tls_inspection_configuration.update_token
```

---


### Firewall_encryption_configuration

FirewallEncryptionConfiguration resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_token` | String |  | <p>An optional token that you can use for optimistic locking. Network Firewall returns a token to your requests that access the firewall. The token marks the state of the firewall resource at the time of the request. </p>
         <p>To make an unconditional change to the firewall, omit the token in your update request. Without the token, Network Firewall performs your updates regardless of whether the firewall has changed since you last retrieved it.</p>
         <p>To make a conditional change to the firewall, provide the token in your update request. Network Firewall uses the token to ensure that the firewall hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the firewall again to get a current copy of it with a new token. Reapply your changes as needed, then try the operation again using the new token. </p> |
| `firewall_name` | String |  | <p>The descriptive name of the firewall. You can't change the name of a firewall after you create it.</p> |
| `encryption_configuration` | String |  |  |
| `firewall_arn` | String |  | <p>The Amazon Resource Name (ARN) of the firewall.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Rule_group_summary

RuleGroupSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `summary` | String | <p>A complex type that contains rule information based on the rule group's configured summary settings. The content varies depending on the fields that you specified to extract in your SummaryConfiguration. When you haven't configured any summary settings, this returns an empty array. The response might include:</p>
         <ul>
            <li>
               <p>Rule identifiers</p>
            </li>
            <li>
               <p>Rule descriptions</p>
            </li>
            <li>
               <p>Any metadata fields that you specified in your SummaryConfiguration</p>
            </li>
         </ul> |
| `rule_group_name` | String | <p>The descriptive name of the rule group. You can't change the name of a rule group after you create it.</p> |
| `description` | String | <p>A description of the rule group. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access rule_group_summary outputs
rule_group_summary_id = rule_group_summary.id
rule_group_summary_summary = rule_group_summary.summary
rule_group_summary_rule_group_name = rule_group_summary.rule_group_name
rule_group_summary_description = rule_group_summary.description
```

---


### Network_firewall_transit_gateway_attachment

NetworkFirewallTransitGatewayAttachment resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Availability_zone_change_protection

AvailabilityZoneChangeProtection resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `availability_zone_change_protection` | bool | ✅ | <p>A setting indicating whether the firewall is protected against changes to the subnet associations.
         Use this setting to protect against
         accidentally modifying the subnet associations for a firewall that is in use. When you create a firewall, the operation initializes this setting to <code>TRUE</code>.</p> |
| `update_token` | String |  | <p>An optional token that you can use for optimistic locking. Network Firewall returns a token to your requests that access the firewall. The token marks the state of the firewall resource at the time of the request. </p>
         <p>To make an unconditional change to the firewall, omit the token in your update request. Without the token, Network Firewall performs your updates regardless of whether the firewall has changed since you last retrieved it.</p>
         <p>To make a conditional change to the firewall, provide the token in your update request. Network Firewall uses the token to ensure that the firewall hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the firewall again to get a current copy of it with a new token. Reapply your changes as needed, then try the operation again using the new token. </p> |
| `firewall_arn` | String |  | <p>The Amazon Resource Name (ARN) of the firewall.</p>
         <p>You must specify the ARN or the name, and you can specify both. </p> |
| `firewall_name` | String |  | <p>The descriptive name of the firewall. You can't change the name of a firewall after you create it.</p>
         <p>You must specify the ARN or the name, and you can specify both. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Firewall_policy

FirewallPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description of the firewall policy.</p> |
| `firewall_policy` | String | ✅ | <p>The rule groups and policy actions to use in the firewall policy.</p> |
| `firewall_policy_name` | String | ✅ | <p>The descriptive name of the firewall policy. You can't change the name of a firewall policy after you create it.</p> |
| `encryption_configuration` | String |  | <p>A complex type that contains settings for encryption of your firewall policy resources.</p> |
| `tags` | Vec<String> |  | <p>The key:value pairs to associate with the resource.</p> |
| `dry_run` | bool |  | <p>Indicates whether you want Network Firewall to just check the validity of the request, rather than run the request. </p>
         <p>If set to <code>TRUE</code>, Network Firewall checks whether the request can run successfully,
         but doesn't actually make the requested changes. The call returns the value that the request would return if you ran it with
         dry run set to <code>FALSE</code>, but doesn't make additions or changes to your resources. This option allows you to make sure that you have
         the required permissions to run the request and that your request parameters are valid. </p>
         <p>If set to <code>FALSE</code>, Network Firewall makes the requested changes to your resources. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `firewall_policy_response` | String | <p>The high-level properties of a firewall policy. This, along with the <a>FirewallPolicy</a>, define the policy. You can retrieve all objects for a firewall policy by calling <a>DescribeFirewallPolicy</a>. </p> |
| `update_token` | String | <p>A token used for optimistic locking. Network Firewall returns a token to your requests that access the firewall policy. The token marks the state of the policy resource at the time of the request. </p>
         <p>To make changes to the policy, you provide the token in your request. Network Firewall uses the token to ensure that the policy hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the firewall policy again to get a current copy of it with current token. Reapply your changes as needed, then try the operation again using the new token. </p> |
| `firewall_policy` | String | <p>The policy for the specified firewall policy. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create firewall_policy
firewall_policy = provider.network_firewall.Firewall_policy {
    firewall_policy = "value"  # <p>The rule groups and policy actions to use in the firewall policy.</p>
    firewall_policy_name = "value"  # <p>The descriptive name of the firewall policy. You can't change the name of a firewall policy after you create it.</p>
}

# Access firewall_policy outputs
firewall_policy_id = firewall_policy.id
firewall_policy_firewall_policy_response = firewall_policy.firewall_policy_response
firewall_policy_update_token = firewall_policy.update_token
firewall_policy_firewall_policy = firewall_policy.firewall_policy
```

---


### Firewall_analysis_settings

FirewallAnalysisSettings resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `firewall_arn` | String |  | <p>The Amazon Resource Name (ARN) of the firewall.</p>
         <p>You must specify the ARN or the name, and you can specify both. </p> |
| `enabled_analysis_types` | Vec<String> |  | <p>An optional setting indicating the specific traffic analysis types to enable on the firewall. </p> |
| `update_token` | String |  | <p>An optional token that you can use for optimistic locking. Network Firewall returns a token to your requests that access the firewall. The token marks the state of the firewall resource at the time of the request. </p>
         <p>To make an unconditional change to the firewall, omit the token in your update request. Without the token, Network Firewall performs your updates regardless of whether the firewall has changed since you last retrieved it.</p>
         <p>To make a conditional change to the firewall, provide the token in your update request. Network Firewall uses the token to ensure that the firewall hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the firewall again to get a current copy of it with a new token. Reapply your changes as needed, then try the operation again using the new token. </p> |
| `firewall_name` | String |  | <p>The descriptive name of the firewall. You can't change the name of a firewall after you create it.</p>
         <p>You must specify the ARN or the name, and you can specify both. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Firewall_metadata

FirewallMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `firewall_policy_arn` | String | <p>The Amazon Resource Name (ARN) of the firewall policy.</p> |
| `transit_gateway_attachment_id` | String | <p>The unique identifier of the transit gateway attachment associated with this firewall. This field is only present for transit gateway-attached firewalls.</p> |
| `supported_availability_zones` | HashMap<String, String> | <p>The Availability Zones that the firewall currently supports. This includes all Availability Zones for which 
       the firewall has a subnet defined. </p> |
| `description` | String | <p>A description of the firewall.</p> |
| `firewall_arn` | String | <p>The Amazon Resource Name (ARN) of the firewall.</p> |
| `status` | String | <p>The readiness of the configured firewall to handle network traffic across all of the
         Availability Zones where you have it configured. This setting is <code>READY</code> only when
         the <code>ConfigurationSyncStateSummary</code> value is <code>IN_SYNC</code> and the
            <code>Attachment</code>
            <code>Status</code> values for all of the configured subnets are <code>READY</code>.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access firewall_metadata outputs
firewall_metadata_id = firewall_metadata.id
firewall_metadata_firewall_policy_arn = firewall_metadata.firewall_policy_arn
firewall_metadata_transit_gateway_attachment_id = firewall_metadata.transit_gateway_attachment_id
firewall_metadata_supported_availability_zones = firewall_metadata.supported_availability_zones
firewall_metadata_description = firewall_metadata.description
firewall_metadata_firewall_arn = firewall_metadata.firewall_arn
firewall_metadata_status = firewall_metadata.status
```

---


### Firewall_description

FirewallDescription resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_token` | String |  | <p>An optional token that you can use for optimistic locking. Network Firewall returns a token to your requests that access the firewall. The token marks the state of the firewall resource at the time of the request. </p>
         <p>To make an unconditional change to the firewall, omit the token in your update request. Without the token, Network Firewall performs your updates regardless of whether the firewall has changed since you last retrieved it.</p>
         <p>To make a conditional change to the firewall, provide the token in your update request. Network Firewall uses the token to ensure that the firewall hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the firewall again to get a current copy of it with a new token. Reapply your changes as needed, then try the operation again using the new token. </p> |
| `firewall_name` | String |  | <p>The descriptive name of the firewall. You can't change the name of a firewall after you create it.</p>
         <p>You must specify the ARN or the name, and you can specify both. </p> |
| `firewall_arn` | String |  | <p>The Amazon Resource Name (ARN) of the firewall.</p>
         <p>You must specify the ARN or the name, and you can specify both. </p> |
| `description` | String |  | <p>The new description for the firewall. If you omit this setting, Network Firewall removes
         the description for the firewall.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Rule_group_metadata

RuleGroupMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>Returns the metadata objects for the specified rule group.
      </p> |
| `rule_group_arn` | String | <p>The descriptive name of the rule group. You can't change the name of a rule group after you create it.</p>
         <p>You must specify the ARN or the name, and you can specify both. </p> |
| `capacity` | i64 | <p>The maximum operating resources that this rule group can use. Rule group capacity is fixed at creation.
      When you update a rule group, you are limited to this capacity. When you reference a rule group
      from a firewall policy, Network Firewall reserves this capacity for the rule group. </p>
         <p>You can retrieve the capacity that would be required for a rule group before you create the rule group by calling
      <a>CreateRuleGroup</a> with <code>DryRun</code> set to <code>TRUE</code>. </p> |
| `stateful_rule_options` | String |  |
| `rule_group_name` | String | <p>The descriptive name of the rule group. You can't change the name of a rule group after you create it.</p>
         <p>You must specify the ARN or the name, and you can specify both. </p> |
| `last_modified_time` | String | <p>A timestamp indicating when the rule group was last modified.</p> |
| `type` | String | <p>Indicates whether the rule group is stateless or stateful. If the rule group is stateless, it contains
stateless rules. If it is stateful, it contains stateful rules. </p>
         <note>
            <p>This setting is required for requests that do not include the <code>RuleGroupARN</code>.</p>
         </note> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access rule_group_metadata outputs
rule_group_metadata_id = rule_group_metadata.id
rule_group_metadata_description = rule_group_metadata.description
rule_group_metadata_rule_group_arn = rule_group_metadata.rule_group_arn
rule_group_metadata_capacity = rule_group_metadata.capacity
rule_group_metadata_stateful_rule_options = rule_group_metadata.stateful_rule_options
rule_group_metadata_rule_group_name = rule_group_metadata.rule_group_name
rule_group_metadata_last_modified_time = rule_group_metadata.last_modified_time
rule_group_metadata_type = rule_group_metadata.type
```

---


### Flow_operation

FlowOperation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `flow_request_timestamp` | String | <p>A timestamp indicating when the Suricata engine identified flows impacted by an operation. </p> |
| `flow_operation` | String | <p>Returns key information about a flow operation, such as related statuses, unique identifiers, and all filters defined in the operation.</p> |
| `availability_zone` | String | <p>The ID of the Availability Zone where the firewall is located. For example, <code>us-east-2a</code>.</p>
         <p>Defines the scope a flow operation. You can use up to 20 filters to configure a single flow operation.</p> |
| `flow_operation_id` | String | <p>A unique identifier for the flow operation. This ID is returned in the responses to start and list commands. You provide to describe commands.</p> |
| `flow_operation_status` | String | <p>Returns the status of the flow operation. This string is returned in the responses to start, list, and describe commands.</p>
         <p>If the status is <code>COMPLETED_WITH_ERRORS</code>, results may be returned with any number of <code>Flows</code> missing from the response. 
If the status is <code>FAILED</code>, <code>Flows</code> returned will be empty.</p> |
| `status_message` | String | <p>If the asynchronous operation fails, Network Firewall populates this with the reason for the error or failure. Options include <code>Flow operation error</code> and <code>Flow timeout</code>.</p> |
| `flow_operation_type` | String | <p>Defines the type of <code>FlowOperation</code>.</p> |
| `firewall_arn` | String | <p>The Amazon Resource Name (ARN) of the firewall.</p> |
| `vpc_endpoint_association_arn` | String | <p>The Amazon Resource Name (ARN) of a VPC endpoint association.</p> |
| `vpc_endpoint_id` | String | <p>A unique identifier for the primary endpoint associated with a firewall.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access flow_operation outputs
flow_operation_id = flow_operation.id
flow_operation_flow_request_timestamp = flow_operation.flow_request_timestamp
flow_operation_flow_operation = flow_operation.flow_operation
flow_operation_availability_zone = flow_operation.availability_zone
flow_operation_flow_operation_id = flow_operation.flow_operation_id
flow_operation_flow_operation_status = flow_operation.flow_operation_status
flow_operation_status_message = flow_operation.status_message
flow_operation_flow_operation_type = flow_operation.flow_operation_type
flow_operation_firewall_arn = flow_operation.firewall_arn
flow_operation_vpc_endpoint_association_arn = flow_operation.vpc_endpoint_association_arn
flow_operation_vpc_endpoint_id = flow_operation.vpc_endpoint_id
```

---


### Firewall_delete_protection

FirewallDeleteProtection resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `firewall_name` | String |  | <p>The descriptive name of the firewall. You can't change the name of a firewall after you create it.</p>
         <p>You must specify the ARN or the name, and you can specify both. </p> |
| `delete_protection` | bool | ✅ | <p>A flag indicating whether it is possible to delete the firewall. A setting of <code>TRUE</code> indicates
         that the firewall is protected against deletion. Use this setting to protect against
         accidentally deleting a firewall that is in use. When you create a firewall, the operation initializes this flag to <code>TRUE</code>.</p> |
| `update_token` | String |  | <p>An optional token that you can use for optimistic locking. Network Firewall returns a token to your requests that access the firewall. The token marks the state of the firewall resource at the time of the request. </p>
         <p>To make an unconditional change to the firewall, omit the token in your update request. Without the token, Network Firewall performs your updates regardless of whether the firewall has changed since you last retrieved it.</p>
         <p>To make a conditional change to the firewall, provide the token in your update request. Network Firewall uses the token to ensure that the firewall hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the firewall again to get a current copy of it with a new token. Reapply your changes as needed, then try the operation again using the new token. </p> |
| `firewall_arn` | String |  | <p>The Amazon Resource Name (ARN) of the firewall.</p>
         <p>You must specify the ARN or the name, and you can specify both. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Logging_configuration

LoggingConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `firewall_arn` | String |  | <p>The Amazon Resource Name (ARN) of the firewall.</p>
         <p>You must specify the ARN or the name, and you can specify both. </p> |
| `firewall_name` | String |  | <p>The descriptive name of the firewall. You can't change the name of a firewall after you create it.</p>
         <p>You must specify the ARN or the name, and you can specify both. </p> |
| `enable_monitoring_dashboard` | bool |  | <p>A boolean that lets you enable or disable the detailed firewall monitoring dashboard on the firewall. </p>
         <p>The monitoring dashboard provides comprehensive visibility into your firewall's flow logs and alert logs. 
         After you enable detailed monitoring, you can access these dashboards directly from the <b>Monitoring</b> page of the Network Firewall console.</p>
         <p>
         Specify <code>TRUE</code> to enable the the detailed monitoring dashboard on the firewall. 
         Specify <code>FALSE</code> to disable the the detailed monitoring dashboard on the firewall. 
      </p> |
| `logging_configuration` | String |  | <p>Defines how Network Firewall performs logging for a firewall. If you omit this setting,
         Network Firewall disables logging for the firewall.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `logging_configuration` | String |  |
| `firewall_arn` | String | <p>The Amazon Resource Name (ARN) of the firewall.</p> |
| `enable_monitoring_dashboard` | bool | <p>A boolean that reflects whether or not the firewall monitoring dashboard is enabled on a firewall.</p>
         <p>
         Returns <code>TRUE</code> when the firewall monitoring dashboard is enabled on the firewall. 
         Returns <code>FALSE</code> when the firewall monitoring dashboard is not enabled on the firewall.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access logging_configuration outputs
logging_configuration_id = logging_configuration.id
logging_configuration_logging_configuration = logging_configuration.logging_configuration
logging_configuration_firewall_arn = logging_configuration.firewall_arn
logging_configuration_enable_monitoring_dashboard = logging_configuration.enable_monitoring_dashboard
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple rule_group resources
rule_group_0 = provider.network_firewall.Rule_group {
    capacity = "value-0"
    rule_group_name = "value-0"
    type = "value-0"
}
rule_group_1 = provider.network_firewall.Rule_group {
    capacity = "value-1"
    rule_group_name = "value-1"
    type = "value-1"
}
rule_group_2 = provider.network_firewall.Rule_group {
    capacity = "value-2"
    rule_group_name = "value-2"
    type = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    rule_group = provider.network_firewall.Rule_group {
        capacity = "production-value"
        rule_group_name = "production-value"
        type = "production-value"
    }
```

---

## Related Documentation

- [AWS Network_firewall Documentation](https://docs.aws.amazon.com/network_firewall/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
