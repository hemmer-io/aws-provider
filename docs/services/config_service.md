# Config_service Service



**Resources**: 60

---

## Overview

The config_service service provides access to 60 resource types:

- [Conformance_pack_compliance_details](#conformance_pack_compliance_details) [R]
- [Organization_conformance_pack_detailed_status](#organization_conformance_pack_detailed_status) [R]
- [Pending_aggregation_request](#pending_aggregation_request) [D]
- [Configuration_aggregator_sources_status](#configuration_aggregator_sources_status) [R]
- [Organization_config_rule](#organization_config_rule) [CD]
- [Aggregate_compliance_by_config_rules](#aggregate_compliance_by_config_rules) [R]
- [Compliance_by_resource](#compliance_by_resource) [R]
- [Discovered_resource_counts](#discovered_resource_counts) [R]
- [Custom_rule_policy](#custom_rule_policy) [R]
- [Compliance_summary_by_config_rule](#compliance_summary_by_config_rule) [R]
- [Stored_query](#stored_query) [CRD]
- [Delivery_channel_status](#delivery_channel_status) [R]
- [Conformance_pack_status](#conformance_pack_status) [R]
- [Conformance_pack_compliance](#conformance_pack_compliance) [R]
- [Evaluation_results](#evaluation_results) [D]
- [Compliance_by_config_rule](#compliance_by_config_rule) [R]
- [Organization_config_rule_detailed_status](#organization_config_rule_detailed_status) [R]
- [Configuration_recorder](#configuration_recorder) [CD]
- [Remediation_execution_status](#remediation_execution_status) [R]
- [Aggregate_conformance_pack_compliance_summary](#aggregate_conformance_pack_compliance_summary) [R]
- [Config_rule](#config_rule) [CD]
- [Organization_custom_rule_policy](#organization_custom_rule_policy) [R]
- [Conformance_pack_compliance_summary](#conformance_pack_compliance_summary) [R]
- [Aggregation_authorization](#aggregation_authorization) [CD]
- [Remediation_configurations](#remediation_configurations) [CR]
- [Remediation_configuration](#remediation_configuration) [D]
- [Aggregate_discovered_resource_counts](#aggregate_discovered_resource_counts) [R]
- [Remediation_exceptions](#remediation_exceptions) [CRD]
- [Organization_conformance_pack_statuses](#organization_conformance_pack_statuses) [R]
- [External_evaluation](#external_evaluation) [C]
- [Compliance_details_by_config_rule](#compliance_details_by_config_rule) [R]
- [Aggregate_compliance_by_conformance_packs](#aggregate_compliance_by_conformance_packs) [R]
- [Evaluations](#evaluations) [C]
- [Conformance_packs](#conformance_packs) [R]
- [Aggregate_compliance_details_by_config_rule](#aggregate_compliance_details_by_config_rule) [R]
- [Config_rule_evaluation_status](#config_rule_evaluation_status) [R]
- [Retention_configurations](#retention_configurations) [R]
- [Configuration_recorders](#configuration_recorders) [R]
- [Organization_config_rules](#organization_config_rules) [R]
- [Retention_configuration](#retention_configuration) [CD]
- [Configuration_aggregator](#configuration_aggregator) [CD]
- [Aggregation_authorizations](#aggregation_authorizations) [R]
- [Compliance_summary_by_resource_type](#compliance_summary_by_resource_type) [R]
- [Resource_config](#resource_config) [CD]
- [Aggregate_resource_config](#aggregate_resource_config) [R]
- [Service_linked_configuration_recorder](#service_linked_configuration_recorder) [CD]
- [Resource_config_history](#resource_config_history) [R]
- [Delivery_channels](#delivery_channels) [R]
- [Configuration_aggregators](#configuration_aggregators) [R]
- [Config_rules](#config_rules) [R]
- [Conformance_pack](#conformance_pack) [CD]
- [Organization_config_rule_statuses](#organization_config_rule_statuses) [R]
- [Organization_conformance_packs](#organization_conformance_packs) [R]
- [Resource_evaluation_summary](#resource_evaluation_summary) [R]
- [Configuration_recorder_status](#configuration_recorder_status) [R]
- [Aggregate_config_rule_compliance_summary](#aggregate_config_rule_compliance_summary) [R]
- [Pending_aggregation_requests](#pending_aggregation_requests) [R]
- [Compliance_details_by_resource](#compliance_details_by_resource) [R]
- [Organization_conformance_pack](#organization_conformance_pack) [CD]
- [Delivery_channel](#delivery_channel) [CD]

---

## Resources


### Conformance_pack_compliance_details

ConformancePackComplianceDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `conformance_pack_rule_evaluation_results` | Vec<String> | <p>Returns a list of <code>ConformancePackEvaluationResult</code> objects.</p> |
| `next_token` | String | <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p> |
| `conformance_pack_name` | String | <p>Name of the conformance pack.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access conformance_pack_compliance_details outputs
conformance_pack_compliance_details_id = conformance_pack_compliance_details.id
conformance_pack_compliance_details_conformance_pack_rule_evaluation_results = conformance_pack_compliance_details.conformance_pack_rule_evaluation_results
conformance_pack_compliance_details_next_token = conformance_pack_compliance_details.next_token
conformance_pack_compliance_details_conformance_pack_name = conformance_pack_compliance_details.conformance_pack_name
```

---


### Organization_conformance_pack_detailed_status

OrganizationConformancePackDetailedStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response. </p> |
| `organization_conformance_pack_detailed_statuses` | Vec<String> | <p>A list of <code>OrganizationConformancePackDetailedStatus</code> objects. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organization_conformance_pack_detailed_status outputs
organization_conformance_pack_detailed_status_id = organization_conformance_pack_detailed_status.id
organization_conformance_pack_detailed_status_next_token = organization_conformance_pack_detailed_status.next_token
organization_conformance_pack_detailed_status_organization_conformance_pack_detailed_statuses = organization_conformance_pack_detailed_status.organization_conformance_pack_detailed_statuses
```

---


### Pending_aggregation_request

PendingAggregationRequest resource

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


### Configuration_aggregator_sources_status

ConfigurationAggregatorSourcesStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `aggregated_source_status_list` | Vec<String> | <p>Returns an AggregatedSourceStatus object.
			</p> |
| `next_token` | String | <p>The <code>nextToken</code> string returned on a previous page that you use
			to get the next page of results in a paginated response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access configuration_aggregator_sources_status outputs
configuration_aggregator_sources_status_id = configuration_aggregator_sources_status.id
configuration_aggregator_sources_status_aggregated_source_status_list = configuration_aggregator_sources_status.aggregated_source_status_list
configuration_aggregator_sources_status_next_token = configuration_aggregator_sources_status.next_token
```

---


### Organization_config_rule

OrganizationConfigRule resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `organization_config_rule_name` | String | ✅ | <p>The name that you assign to an organization Config rule.</p> |
| `organization_managed_rule_metadata` | String |  | <p>An <code>OrganizationManagedRuleMetadata</code> object. This object specifies organization
			managed rule metadata such as resource type and ID of Amazon Web Services resource along with the rule identifier.
			It also provides the frequency with which you want Config to run evaluations for the rule if the trigger type is periodic.</p> |
| `organization_custom_policy_rule_metadata` | String |  | <p>An <code>OrganizationCustomPolicyRuleMetadata</code> object. This object specifies metadata for your organization's Config Custom Policy rule. The metadata includes the runtime system in use, which accounts have debug
			logging enabled, and other custom rule metadata, such as resource type, resource ID of
			Amazon Web Services resource, and organization trigger types that initiate Config to evaluate Amazon Web Services resources against a rule.</p> |
| `excluded_accounts` | Vec<String> |  | <p>A comma-separated list of accounts that you want to exclude from an organization Config rule.</p> |
| `organization_custom_rule_metadata` | String |  | <p>An <code>OrganizationCustomRuleMetadata</code> object. This object specifies organization custom rule metadata such as resource type,
			resource ID of Amazon Web Services resource, Lambda function ARN, and organization trigger types that trigger Config to evaluate your Amazon Web Services resources against a rule.
			It also provides the frequency with which you want Config to run evaluations for the rule if the trigger type is periodic.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create organization_config_rule
organization_config_rule = provider.config_service.Organization_config_rule {
    organization_config_rule_name = "value"  # <p>The name that you assign to an organization Config rule.</p>
}

```

---


### Aggregate_compliance_by_config_rules

AggregateComplianceByConfigRules resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The <code>nextToken</code> string returned on a previous page that you use
			to get the next page of results in a paginated response.</p> |
| `aggregate_compliance_by_config_rules` | Vec<String> | <p>Returns a list of AggregateComplianceByConfigRule
			object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access aggregate_compliance_by_config_rules outputs
aggregate_compliance_by_config_rules_id = aggregate_compliance_by_config_rules.id
aggregate_compliance_by_config_rules_next_token = aggregate_compliance_by_config_rules.next_token
aggregate_compliance_by_config_rules_aggregate_compliance_by_config_rules = aggregate_compliance_by_config_rules.aggregate_compliance_by_config_rules
```

---


### Compliance_by_resource

ComplianceByResource resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `compliance_by_resources` | Vec<String> | <p>Indicates whether the specified Amazon Web Services resource complies with all
			of the Config rules that evaluate it.</p> |
| `next_token` | String | <p>The string that you use in a subsequent request to get the next
			page of results in a paginated response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access compliance_by_resource outputs
compliance_by_resource_id = compliance_by_resource.id
compliance_by_resource_compliance_by_resources = compliance_by_resource.compliance_by_resources
compliance_by_resource_next_token = compliance_by_resource.next_token
```

---


### Discovered_resource_counts

DiscoveredResourceCounts resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_discovered_resources` | i64 | <p>The total number of resources that Config is recording in
			the region for your account. If you specify resource types in the
			request, Config returns only the total number of resources for
			those resource types.</p>
         <p class="title">
            <b>Example</b>
         </p>
         <ol>
            <li>
               <p>Config is recording three resource types in the US
					East (Ohio) Region for your account: 25 EC2 instances, 20
					IAM users, and 15 S3 buckets, for a total of 60
					resources.</p>
            </li>
            <li>
               <p>You make a call to the
						<code>GetDiscoveredResourceCounts</code> action and
					specify the resource type,
						<code>"AWS::EC2::Instances"</code>, in the
					request.</p>
            </li>
            <li>
               <p>Config returns 25 for
						<code>totalDiscoveredResources</code>.</p>
            </li>
         </ol> |
| `resource_counts` | Vec<String> | <p>The list of <code>ResourceCount</code> objects. Each object is
			listed in descending order by the number of resources.</p> |
| `next_token` | String | <p>The string that you use in a subsequent request to get the next
			page of results in a paginated response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access discovered_resource_counts outputs
discovered_resource_counts_id = discovered_resource_counts.id
discovered_resource_counts_total_discovered_resources = discovered_resource_counts.total_discovered_resources
discovered_resource_counts_resource_counts = discovered_resource_counts.resource_counts
discovered_resource_counts_next_token = discovered_resource_counts.next_token
```

---


### Custom_rule_policy

CustomRulePolicy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_text` | String | <p>The policy definition containing the logic for your Config Custom Policy rule.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access custom_rule_policy outputs
custom_rule_policy_id = custom_rule_policy.id
custom_rule_policy_policy_text = custom_rule_policy.policy_text
```

---


### Compliance_summary_by_config_rule

ComplianceSummaryByConfigRule resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `compliance_summary` | String | <p>The number of Config rules that are compliant and the
			number that are noncompliant, up to a maximum of 25 for
			each.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access compliance_summary_by_config_rule outputs
compliance_summary_by_config_rule_id = compliance_summary_by_config_rule.id
compliance_summary_by_config_rule_compliance_summary = compliance_summary_by_config_rule.compliance_summary
```

---


### Stored_query

StoredQuery resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of <code>Tags</code> object.</p> |
| `stored_query` | String | ✅ | <p>A list of <code>StoredQuery</code> objects. 
			The mandatory fields are <code>QueryName</code> and <code>Expression</code>.</p>
         <note>
            <p>When you are creating a query, you must provide a query name and an expression. 
			When you are updating a query, you must provide a query name but updating the description is optional.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stored_query` | String | <p>Returns a <code>StoredQuery</code> object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create stored_query
stored_query = provider.config_service.Stored_query {
    stored_query = "value"  # <p>A list of <code>StoredQuery</code> objects. 
			The mandatory fields are <code>QueryName</code> and <code>Expression</code>.</p>
         <note>
            <p>When you are creating a query, you must provide a query name and an expression. 
			When you are updating a query, you must provide a query name but updating the description is optional.</p>
         </note>
}

# Access stored_query outputs
stored_query_id = stored_query.id
stored_query_stored_query = stored_query.stored_query
```

---


### Delivery_channel_status

DeliveryChannelStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delivery_channels_status` | Vec<String> | <p>A list that contains the status of a specified delivery
			channel.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access delivery_channel_status outputs
delivery_channel_status_id = delivery_channel_status.id
delivery_channel_status_delivery_channels_status = delivery_channel_status.delivery_channels_status
```

---


### Conformance_pack_status

ConformancePackStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `conformance_pack_status_details` | Vec<String> | <p>A list of <code>ConformancePackStatusDetail</code> objects.</p> |
| `next_token` | String | <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access conformance_pack_status outputs
conformance_pack_status_id = conformance_pack_status.id
conformance_pack_status_conformance_pack_status_details = conformance_pack_status.conformance_pack_status_details
conformance_pack_status_next_token = conformance_pack_status.next_token
```

---


### Conformance_pack_compliance

ConformancePackCompliance resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `conformance_pack_rule_compliance_list` | Vec<String> | <p>Returns a list of <code>ConformancePackRuleCompliance</code> objects.</p> |
| `next_token` | String | <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p> |
| `conformance_pack_name` | String | <p>Name of the conformance pack.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access conformance_pack_compliance outputs
conformance_pack_compliance_id = conformance_pack_compliance.id
conformance_pack_compliance_conformance_pack_rule_compliance_list = conformance_pack_compliance.conformance_pack_rule_compliance_list
conformance_pack_compliance_next_token = conformance_pack_compliance.next_token
conformance_pack_compliance_conformance_pack_name = conformance_pack_compliance.conformance_pack_name
```

---


### Evaluation_results

EvaluationResults resource

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


### Compliance_by_config_rule

ComplianceByConfigRule resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `compliance_by_config_rules` | Vec<String> | <p>Indicates whether each of the specified Config rules is
			compliant.</p> |
| `next_token` | String | <p>The string that you use in a subsequent request to get the next
			page of results in a paginated response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access compliance_by_config_rule outputs
compliance_by_config_rule_id = compliance_by_config_rule.id
compliance_by_config_rule_compliance_by_config_rules = compliance_by_config_rule.compliance_by_config_rules
compliance_by_config_rule_next_token = compliance_by_config_rule.next_token
```

---


### Organization_config_rule_detailed_status

OrganizationConfigRuleDetailedStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `organization_config_rule_detailed_status` | Vec<String> | <p>A list of <code>MemberAccountStatus</code> objects.</p> |
| `next_token` | String | <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organization_config_rule_detailed_status outputs
organization_config_rule_detailed_status_id = organization_config_rule_detailed_status.id
organization_config_rule_detailed_status_organization_config_rule_detailed_status = organization_config_rule_detailed_status.organization_config_rule_detailed_status
organization_config_rule_detailed_status_next_token = organization_config_rule_detailed_status.next_token
```

---


### Configuration_recorder

ConfigurationRecorder resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The tags for the customer managed configuration recorder. Each tag consists of a key and an optional value, both of which you define.</p> |
| `configuration_recorder` | String | ✅ | <p>An object for the configuration recorder. A configuration recorder records configuration changes for the resource types in scope.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_recorder
configuration_recorder = provider.config_service.Configuration_recorder {
    configuration_recorder = "value"  # <p>An object for the configuration recorder. A configuration recorder records configuration changes for the resource types in scope.</p>
}

```

---


### Remediation_execution_status

RemediationExecutionStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p> |
| `remediation_execution_statuses` | Vec<String> | <p>Returns a list of remediation execution statuses objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access remediation_execution_status outputs
remediation_execution_status_id = remediation_execution_status.id
remediation_execution_status_next_token = remediation_execution_status.next_token
remediation_execution_status_remediation_execution_statuses = remediation_execution_status.remediation_execution_statuses
```

---


### Aggregate_conformance_pack_compliance_summary

AggregateConformancePackComplianceSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p> |
| `aggregate_conformance_pack_compliance_summaries` | Vec<String> | <p>Returns a list of <code>AggregateConformancePackComplianceSummary</code> object.</p> |
| `group_by_key` | String | <p>Groups the result based on Amazon Web Services account ID or Amazon Web Services Region.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access aggregate_conformance_pack_compliance_summary outputs
aggregate_conformance_pack_compliance_summary_id = aggregate_conformance_pack_compliance_summary.id
aggregate_conformance_pack_compliance_summary_next_token = aggregate_conformance_pack_compliance_summary.next_token
aggregate_conformance_pack_compliance_summary_aggregate_conformance_pack_compliance_summaries = aggregate_conformance_pack_compliance_summary.aggregate_conformance_pack_compliance_summaries
aggregate_conformance_pack_compliance_summary_group_by_key = aggregate_conformance_pack_compliance_summary.group_by_key
```

---


### Config_rule

ConfigRule resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>An array of tag object.</p> |
| `config_rule` | String | ✅ | <p>The rule that you want to add to your account.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create config_rule
config_rule = provider.config_service.Config_rule {
    config_rule = "value"  # <p>The rule that you want to add to your account.</p>
}

```

---


### Organization_custom_rule_policy

OrganizationCustomRulePolicy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_text` | String | <p>The policy definition containing the logic for your organization Config Custom Policy rule.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organization_custom_rule_policy outputs
organization_custom_rule_policy_id = organization_custom_rule_policy.id
organization_custom_rule_policy_policy_text = organization_custom_rule_policy.policy_text
```

---


### Conformance_pack_compliance_summary

ConformancePackComplianceSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p> |
| `conformance_pack_compliance_summary_list` | Vec<String> | <p>A list of <code>ConformancePackComplianceSummary</code> objects. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access conformance_pack_compliance_summary outputs
conformance_pack_compliance_summary_id = conformance_pack_compliance_summary.id
conformance_pack_compliance_summary_next_token = conformance_pack_compliance_summary.next_token
conformance_pack_compliance_summary_conformance_pack_compliance_summary_list = conformance_pack_compliance_summary.conformance_pack_compliance_summary_list
```

---


### Aggregation_authorization

AggregationAuthorization resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `authorized_account_id` | String | ✅ | <p>The 12-digit account ID of the account authorized to aggregate data.</p> |
| `tags` | Vec<String> |  | <p>An array of tag object.</p> |
| `authorized_aws_region` | String | ✅ | <p>The region authorized to collect aggregated data.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create aggregation_authorization
aggregation_authorization = provider.config_service.Aggregation_authorization {
    authorized_account_id = "value"  # <p>The 12-digit account ID of the account authorized to aggregate data.</p>
    authorized_aws_region = "value"  # <p>The region authorized to collect aggregated data.</p>
}

```

---


### Remediation_configurations

RemediationConfigurations resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `remediation_configurations` | Vec<String> | ✅ | <p>A list of remediation configuration objects.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `remediation_configurations` | Vec<String> | <p>Returns a remediation configuration object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create remediation_configurations
remediation_configurations = provider.config_service.Remediation_configurations {
    remediation_configurations = "value"  # <p>A list of remediation configuration objects.</p>
}

# Access remediation_configurations outputs
remediation_configurations_id = remediation_configurations.id
remediation_configurations_remediation_configurations = remediation_configurations.remediation_configurations
```

---


### Remediation_configuration

RemediationConfiguration resource

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


### Aggregate_discovered_resource_counts

AggregateDiscoveredResourceCounts resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_discovered_resources` | i64 | <p>The total number of resources that are present in an aggregator with the filters that you provide.</p> |
| `group_by_key` | String | <p>The key passed into the request object. If <code>GroupByKey</code> is not provided, the result will be empty.</p> |
| `grouped_resource_counts` | Vec<String> | <p>Returns a list of GroupedResourceCount objects.</p> |
| `next_token` | String | <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access aggregate_discovered_resource_counts outputs
aggregate_discovered_resource_counts_id = aggregate_discovered_resource_counts.id
aggregate_discovered_resource_counts_total_discovered_resources = aggregate_discovered_resource_counts.total_discovered_resources
aggregate_discovered_resource_counts_group_by_key = aggregate_discovered_resource_counts.group_by_key
aggregate_discovered_resource_counts_grouped_resource_counts = aggregate_discovered_resource_counts.grouped_resource_counts
aggregate_discovered_resource_counts_next_token = aggregate_discovered_resource_counts.next_token
```

---


### Remediation_exceptions

RemediationExceptions resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_keys` | Vec<String> | ✅ | <p>An exception list of resource exception keys to be processed with the current request. Config adds exception for each resource key. For example, Config adds 3 exceptions for 3 resource keys. </p> |
| `message` | String |  | <p>The message contains an explanation of the exception.</p> |
| `config_rule_name` | String | ✅ | <p>The name of the Config rule for which you want to create remediation exception.</p> |
| `expiration_time` | String |  | <p>The exception is automatically deleted after the expiration date.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p> |
| `remediation_exceptions` | Vec<String> | <p>Returns a list of remediation exception objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create remediation_exceptions
remediation_exceptions = provider.config_service.Remediation_exceptions {
    resource_keys = "value"  # <p>An exception list of resource exception keys to be processed with the current request. Config adds exception for each resource key. For example, Config adds 3 exceptions for 3 resource keys. </p>
    config_rule_name = "value"  # <p>The name of the Config rule for which you want to create remediation exception.</p>
}

# Access remediation_exceptions outputs
remediation_exceptions_id = remediation_exceptions.id
remediation_exceptions_next_token = remediation_exceptions.next_token
remediation_exceptions_remediation_exceptions = remediation_exceptions.remediation_exceptions
```

---


### Organization_conformance_pack_statuses

OrganizationConformancePackStatuses resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response. </p> |
| `organization_conformance_pack_statuses` | Vec<String> | <p>A list of <code>OrganizationConformancePackStatus</code> objects. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organization_conformance_pack_statuses outputs
organization_conformance_pack_statuses_id = organization_conformance_pack_statuses.id
organization_conformance_pack_statuses_next_token = organization_conformance_pack_statuses.next_token
organization_conformance_pack_statuses_organization_conformance_pack_statuses = organization_conformance_pack_statuses.organization_conformance_pack_statuses
```

---


### External_evaluation

ExternalEvaluation resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `config_rule_name` | String | ✅ | <p>The name of the Config rule.</p> |
| `external_evaluation` | String | ✅ | <p>An <code>ExternalEvaluation</code> object that provides details about compliance.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create external_evaluation
external_evaluation = provider.config_service.External_evaluation {
    config_rule_name = "value"  # <p>The name of the Config rule.</p>
    external_evaluation = "value"  # <p>An <code>ExternalEvaluation</code> object that provides details about compliance.</p>
}

```

---


### Compliance_details_by_config_rule

ComplianceDetailsByConfigRule resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The string that you use in a subsequent request to get the next
			page of results in a paginated response.</p> |
| `evaluation_results` | Vec<String> | <p>Indicates whether the Amazon Web Services resource complies with the specified
			Config rule.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access compliance_details_by_config_rule outputs
compliance_details_by_config_rule_id = compliance_details_by_config_rule.id
compliance_details_by_config_rule_next_token = compliance_details_by_config_rule.next_token
compliance_details_by_config_rule_evaluation_results = compliance_details_by_config_rule.evaluation_results
```

---


### Aggregate_compliance_by_conformance_packs

AggregateComplianceByConformancePacks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p> |
| `aggregate_compliance_by_conformance_packs` | Vec<String> | <p>Returns the <code>AggregateComplianceByConformancePack</code> object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access aggregate_compliance_by_conformance_packs outputs
aggregate_compliance_by_conformance_packs_id = aggregate_compliance_by_conformance_packs.id
aggregate_compliance_by_conformance_packs_next_token = aggregate_compliance_by_conformance_packs.next_token
aggregate_compliance_by_conformance_packs_aggregate_compliance_by_conformance_packs = aggregate_compliance_by_conformance_packs.aggregate_compliance_by_conformance_packs
```

---


### Evaluations

Evaluations resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `evaluations` | Vec<String> |  | <p>The assessments that the Lambda function performs. Each
			evaluation identifies an Amazon Web Services resource and indicates whether it
			complies with the Config rule that invokes the Lambda
			function.</p> |
| `test_mode` | bool |  | <p>Use this parameter to specify a test run for
			<code>PutEvaluations</code>. You can verify whether your Lambda function will deliver evaluation results to Config. No
			updates occur to your existing evaluations, and evaluation results
			are not sent to Config.</p>
         <note>
            <p>When <code>TestMode</code> is <code>true</code>,
					<code>PutEvaluations</code> doesn't require a valid value
				for the <code>ResultToken</code> parameter, but the value cannot
				be null.</p>
         </note> |
| `result_token` | String | ✅ | <p>An encrypted token that associates an evaluation with an Config rule. Identifies the rule and the event that triggered the
			evaluation.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create evaluations
evaluations = provider.config_service.Evaluations {
    result_token = "value"  # <p>An encrypted token that associates an evaluation with an Config rule. Identifies the rule and the event that triggered the
			evaluation.</p>
}

```

---


### Conformance_packs

ConformancePacks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p> |
| `conformance_pack_details` | Vec<String> | <p>Returns a list of <code>ConformancePackDetail</code> objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access conformance_packs outputs
conformance_packs_id = conformance_packs.id
conformance_packs_next_token = conformance_packs.next_token
conformance_packs_conformance_pack_details = conformance_packs.conformance_pack_details
```

---


### Aggregate_compliance_details_by_config_rule

AggregateComplianceDetailsByConfigRule resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `aggregate_evaluation_results` | Vec<String> | <p>Returns an AggregateEvaluationResults object.</p> |
| `next_token` | String | <p>The <code>nextToken</code> string returned on a previous page that you use
			to get the next page of results in a paginated response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access aggregate_compliance_details_by_config_rule outputs
aggregate_compliance_details_by_config_rule_id = aggregate_compliance_details_by_config_rule.id
aggregate_compliance_details_by_config_rule_aggregate_evaluation_results = aggregate_compliance_details_by_config_rule.aggregate_evaluation_results
aggregate_compliance_details_by_config_rule_next_token = aggregate_compliance_details_by_config_rule.next_token
```

---


### Config_rule_evaluation_status

ConfigRuleEvaluationStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The string that you use in a subsequent request to get the next
			page of results in a paginated response.</p> |
| `config_rules_evaluation_status` | Vec<String> | <p>Status information about your Config managed rules.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access config_rule_evaluation_status outputs
config_rule_evaluation_status_id = config_rule_evaluation_status.id
config_rule_evaluation_status_next_token = config_rule_evaluation_status.next_token
config_rule_evaluation_status_config_rules_evaluation_status = config_rule_evaluation_status.config_rules_evaluation_status
```

---


### Retention_configurations

RetentionConfigurations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `retention_configurations` | Vec<String> | <p>Returns a retention configuration object.</p> |
| `next_token` | String | <p>The <code>nextToken</code> string returned on a previous page
			that you use to get the next page of results in a paginated
			response. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access retention_configurations outputs
retention_configurations_id = retention_configurations.id
retention_configurations_retention_configurations = retention_configurations.retention_configurations
retention_configurations_next_token = retention_configurations.next_token
```

---


### Configuration_recorders

ConfigurationRecorders resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration_recorders` | Vec<String> | <p>A list that contains the descriptions of the specified
			configuration recorders.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access configuration_recorders outputs
configuration_recorders_id = configuration_recorders.id
configuration_recorders_configuration_recorders = configuration_recorders.configuration_recorders
```

---


### Organization_config_rules

OrganizationConfigRules resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `organization_config_rules` | Vec<String> | <p>Returns a list of <code>OrganizationConfigRule</code> objects.</p> |
| `next_token` | String | <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organization_config_rules outputs
organization_config_rules_id = organization_config_rules.id
organization_config_rules_organization_config_rules = organization_config_rules.organization_config_rules
organization_config_rules_next_token = organization_config_rules.next_token
```

---


### Retention_configuration

RetentionConfiguration resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `retention_period_in_days` | i64 | ✅ | <p>Number of days Config stores your historical
			information.</p>
         <note>
            <p>Currently, only applicable to the configuration item
				history.</p>
         </note> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create retention_configuration
retention_configuration = provider.config_service.Retention_configuration {
    retention_period_in_days = "value"  # <p>Number of days Config stores your historical
			information.</p>
         <note>
            <p>Currently, only applicable to the configuration item
				history.</p>
         </note>
}

```

---


### Configuration_aggregator

ConfigurationAggregator resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_aggregation_sources` | Vec<String> |  | <p>A list of AccountAggregationSource object.
			
		</p> |
| `organization_aggregation_source` | String |  | <p>An OrganizationAggregationSource object.</p> |
| `tags` | Vec<String> |  | <p>An array of tag object.</p> |
| `aggregator_filters` | String |  | <p>An object to filter configuration recorders in an aggregator. Either <code>ResourceType</code> or <code>ServicePrincipal</code> is required.</p> |
| `configuration_aggregator_name` | String | ✅ | <p>The name of the configuration aggregator.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_aggregator
configuration_aggregator = provider.config_service.Configuration_aggregator {
    configuration_aggregator_name = "value"  # <p>The name of the configuration aggregator.</p>
}

```

---


### Aggregation_authorizations

AggregationAuthorizations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `aggregation_authorizations` | Vec<String> | <p>Returns a list of authorizations granted to various aggregator
			accounts and regions.</p> |
| `next_token` | String | <p>The <code>nextToken</code> string returned on a previous page that you use
			to get the next page of results in a paginated response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access aggregation_authorizations outputs
aggregation_authorizations_id = aggregation_authorizations.id
aggregation_authorizations_aggregation_authorizations = aggregation_authorizations.aggregation_authorizations
aggregation_authorizations_next_token = aggregation_authorizations.next_token
```

---


### Compliance_summary_by_resource_type

ComplianceSummaryByResourceType resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `compliance_summaries_by_resource_type` | Vec<String> | <p>The number of resources that are compliant and the number that
			are noncompliant. If one or more resource types were provided with
			the request, the numbers are returned for each resource type. The
			maximum number returned is 100.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access compliance_summary_by_resource_type outputs
compliance_summary_by_resource_type_id = compliance_summary_by_resource_type.id
compliance_summary_by_resource_type_compliance_summaries_by_resource_type = compliance_summary_by_resource_type.compliance_summaries_by_resource_type
```

---


### Resource_config

ResourceConfig resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>Tags associated with the resource.</p>
         <note>
            <p>This field is not to be confused with the Amazon Web Services-wide tag feature for Amazon Web Services resources.
			Tags for <code>PutResourceConfig</code> are tags that you supply for the configuration items of your custom resources.</p>
         </note> |
| `resource_name` | String |  | <p>Name of the resource.</p> |
| `configuration` | String | ✅ | <p>The configuration object of the resource in valid JSON format. It must match the schema registered with CloudFormation.</p>
         <note>
            <p>The configuration JSON must not exceed 64 KB.</p>
         </note> |
| `resource_type` | String | ✅ | <p>The type of the resource. The custom resource type must be registered with CloudFormation. </p>
         <note>
            <p>You cannot use the organization names “amzn”, “amazon”, “alexa”, “custom” with custom resource types. It is the first part of the ResourceType up to the first ::.</p>
         </note> |
| `schema_version_id` | String | ✅ | <p>Version of the schema registered for the ResourceType in CloudFormation.</p> |
| `resource_id` | String | ✅ | <p>Unique identifier of the resource.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_config
resource_config = provider.config_service.Resource_config {
    configuration = "value"  # <p>The configuration object of the resource in valid JSON format. It must match the schema registered with CloudFormation.</p>
         <note>
            <p>The configuration JSON must not exceed 64 KB.</p>
         </note>
    resource_type = "value"  # <p>The type of the resource. The custom resource type must be registered with CloudFormation. </p>
         <note>
            <p>You cannot use the organization names “amzn”, “amazon”, “alexa”, “custom” with custom resource types. It is the first part of the ResourceType up to the first ::.</p>
         </note>
    schema_version_id = "value"  # <p>Version of the schema registered for the ResourceType in CloudFormation.</p>
    resource_id = "value"  # <p>Unique identifier of the resource.</p>
}

```

---


### Aggregate_resource_config

AggregateResourceConfig resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration_item` | String | <p>Returns a <code>ConfigurationItem</code> object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access aggregate_resource_config outputs
aggregate_resource_config_id = aggregate_resource_config.id
aggregate_resource_config_configuration_item = aggregate_resource_config.configuration_item
```

---


### Service_linked_configuration_recorder

ServiceLinkedConfigurationRecorder resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The tags for a service-linked configuration recorder. Each tag consists of a key and an optional value, both of which you define.</p> |
| `service_principal` | String | ✅ | <p>The service principal of the Amazon Web Services service for the service-linked configuration recorder that you want to create.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create service_linked_configuration_recorder
service_linked_configuration_recorder = provider.config_service.Service_linked_configuration_recorder {
    service_principal = "value"  # <p>The service principal of the Amazon Web Services service for the service-linked configuration recorder that you want to create.</p>
}

```

---


### Resource_config_history

ResourceConfigHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration_items` | Vec<String> | <p>An array of <code>ConfigurationItems</code> Objects. Contatins the configuration history for one or more
			resources.</p> |
| `next_token` | String | <p>The string that you use in a subsequent request to get the next
			page of results in a paginated response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_config_history outputs
resource_config_history_id = resource_config_history.id
resource_config_history_configuration_items = resource_config_history.configuration_items
resource_config_history_next_token = resource_config_history.next_token
```

---


### Delivery_channels

DeliveryChannels resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delivery_channels` | Vec<String> | <p>A list that contains the descriptions of the specified delivery
			channel.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access delivery_channels outputs
delivery_channels_id = delivery_channels.id
delivery_channels_delivery_channels = delivery_channels.delivery_channels
```

---


### Configuration_aggregators

ConfigurationAggregators resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration_aggregators` | Vec<String> | <p>Returns a ConfigurationAggregators object.</p> |
| `next_token` | String | <p>The <code>nextToken</code> string returned on a previous page that you use
			to get the next page of results in a paginated response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access configuration_aggregators outputs
configuration_aggregators_id = configuration_aggregators.id
configuration_aggregators_configuration_aggregators = configuration_aggregators.configuration_aggregators
configuration_aggregators_next_token = configuration_aggregators.next_token
```

---


### Config_rules

ConfigRules resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `config_rules` | Vec<String> | <p>The details about your Config rules.</p> |
| `next_token` | String | <p>The string that you use in a subsequent request to get the next
			page of results in a paginated response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access config_rules outputs
config_rules_id = config_rules.id
config_rules_config_rules = config_rules.config_rules
config_rules_next_token = config_rules.next_token
```

---


### Conformance_pack

ConformancePack resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `conformance_pack_input_parameters` | Vec<String> |  | <p>A list of <code>ConformancePackInputParameter</code> objects.</p> |
| `template_ssm_document_details` | String |  | <p>An object of type <code>TemplateSSMDocumentDetails</code>, which contains the name or the Amazon Resource Name (ARN) of the Amazon Web Services Systems Manager document (SSM document) and the version of the SSM document that is used to create a conformance pack.</p> |
| `template_s3_uri` | String |  | <p>The location of the file containing the template body (<code>s3://bucketname/prefix</code>). The uri must point to a conformance pack template (max size: 300 KB) that is located in an Amazon S3 bucket in the same Region as the conformance pack. </p>
         <note>
            <p>You must have access to read Amazon S3 bucket.
			In addition, in order to ensure a successful deployment, the template object must not be in an <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/storage-class-intro.html">archived storage class</a> if this parameter is passed.</p>
         </note> |
| `template_body` | String |  | <p>A string that contains the full conformance pack template body. The structure containing the template body has a minimum length of 1 byte and a maximum length of 51,200 bytes.</p>
         <note>
            <p>You can use a YAML template with two resource types: Config rule (<code>AWS::Config::ConfigRule</code>) and remediation action (<code>AWS::Config::RemediationConfiguration</code>).</p>
         </note> |
| `delivery_s3_bucket` | String |  | <p>The name of the Amazon S3 bucket where Config stores conformance pack templates.</p>
         <note>
            <p>This field is optional.</p>
         </note> |
| `conformance_pack_name` | String | ✅ | <p>The unique name of the conformance pack you want to deploy.</p> |
| `delivery_s3_key_prefix` | String |  | <p>The prefix for the Amazon S3 bucket. </p>
         <note>
            <p>This field is optional.</p>
         </note> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create conformance_pack
conformance_pack = provider.config_service.Conformance_pack {
    conformance_pack_name = "value"  # <p>The unique name of the conformance pack you want to deploy.</p>
}

```

---


### Organization_config_rule_statuses

OrganizationConfigRuleStatuses resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `organization_config_rule_statuses` | Vec<String> | <p>A list of <code>OrganizationConfigRuleStatus</code> objects.</p> |
| `next_token` | String | <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organization_config_rule_statuses outputs
organization_config_rule_statuses_id = organization_config_rule_statuses.id
organization_config_rule_statuses_organization_config_rule_statuses = organization_config_rule_statuses.organization_config_rule_statuses
organization_config_rule_statuses_next_token = organization_config_rule_statuses.next_token
```

---


### Organization_conformance_packs

OrganizationConformancePacks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `organization_conformance_packs` | Vec<String> | <p>Returns a list of OrganizationConformancePacks objects.</p> |
| `next_token` | String | <p>The nextToken string returned on a previous page that you use to get the next page of results in a
			paginated response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organization_conformance_packs outputs
organization_conformance_packs_id = organization_conformance_packs.id
organization_conformance_packs_organization_conformance_packs = organization_conformance_packs.organization_conformance_packs
organization_conformance_packs_next_token = organization_conformance_packs.next_token
```

---


### Resource_evaluation_summary

ResourceEvaluationSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `evaluation_context` | String | <p>Returns an <code>EvaluationContext</code> object.</p> |
| `evaluation_mode` | String | <p>Lists results of the mode that you requested to retrieve the resource evaluation summary. The valid values are Detective or Proactive.</p> |
| `resource_evaluation_id` | String | <p>The unique <code>ResourceEvaluationId</code> of Amazon Web Services resource execution for which you want to retrieve the evaluation summary.</p> |
| `evaluation_status` | String | <p>Returns an <code>EvaluationStatus</code> object.</p> |
| `compliance` | String | <p>The compliance status of the resource evaluation summary.</p> |
| `resource_details` | String | <p>Returns a <code>ResourceDetails</code> object.</p> |
| `evaluation_start_timestamp` | String | <p>The start timestamp when Config rule starts evaluating compliance for the provided resource details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_evaluation_summary outputs
resource_evaluation_summary_id = resource_evaluation_summary.id
resource_evaluation_summary_evaluation_context = resource_evaluation_summary.evaluation_context
resource_evaluation_summary_evaluation_mode = resource_evaluation_summary.evaluation_mode
resource_evaluation_summary_resource_evaluation_id = resource_evaluation_summary.resource_evaluation_id
resource_evaluation_summary_evaluation_status = resource_evaluation_summary.evaluation_status
resource_evaluation_summary_compliance = resource_evaluation_summary.compliance
resource_evaluation_summary_resource_details = resource_evaluation_summary.resource_details
resource_evaluation_summary_evaluation_start_timestamp = resource_evaluation_summary.evaluation_start_timestamp
```

---


### Configuration_recorder_status

ConfigurationRecorderStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration_recorders_status` | Vec<String> | <p>A list that contains status of the specified
			recorders.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access configuration_recorder_status outputs
configuration_recorder_status_id = configuration_recorder_status.id
configuration_recorder_status_configuration_recorders_status = configuration_recorder_status.configuration_recorders_status
```

---


### Aggregate_config_rule_compliance_summary

AggregateConfigRuleComplianceSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The <code>nextToken</code> string returned on a previous page that you use
			to get the next page of results in a paginated response.</p> |
| `aggregate_compliance_counts` | Vec<String> | <p>Returns a list of AggregateComplianceCounts object.</p> |
| `group_by_key` | String | <p>Groups the result based on ACCOUNT_ID or AWS_REGION.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access aggregate_config_rule_compliance_summary outputs
aggregate_config_rule_compliance_summary_id = aggregate_config_rule_compliance_summary.id
aggregate_config_rule_compliance_summary_next_token = aggregate_config_rule_compliance_summary.next_token
aggregate_config_rule_compliance_summary_aggregate_compliance_counts = aggregate_config_rule_compliance_summary.aggregate_compliance_counts
aggregate_config_rule_compliance_summary_group_by_key = aggregate_config_rule_compliance_summary.group_by_key
```

---


### Pending_aggregation_requests

PendingAggregationRequests resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The <code>nextToken</code> string returned on a previous page that you use
			to get the next page of results in a paginated response.</p> |
| `pending_aggregation_requests` | Vec<String> | <p>Returns a PendingAggregationRequests object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access pending_aggregation_requests outputs
pending_aggregation_requests_id = pending_aggregation_requests.id
pending_aggregation_requests_next_token = pending_aggregation_requests.next_token
pending_aggregation_requests_pending_aggregation_requests = pending_aggregation_requests.pending_aggregation_requests
```

---


### Compliance_details_by_resource

ComplianceDetailsByResource resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The string that you use in a subsequent request to get the next
			page of results in a paginated response.</p> |
| `evaluation_results` | Vec<String> | <p>Indicates whether the specified Amazon Web Services resource complies each Config rule.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access compliance_details_by_resource outputs
compliance_details_by_resource_id = compliance_details_by_resource.id
compliance_details_by_resource_next_token = compliance_details_by_resource.next_token
compliance_details_by_resource_evaluation_results = compliance_details_by_resource.evaluation_results
```

---


### Organization_conformance_pack

OrganizationConformancePack resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `conformance_pack_input_parameters` | Vec<String> |  | <p>A list of <code>ConformancePackInputParameter</code> objects.</p> |
| `excluded_accounts` | Vec<String> |  | <p>A list of Amazon Web Services accounts to be excluded from an organization conformance pack while deploying a conformance pack.</p> |
| `organization_conformance_pack_name` | String | ✅ | <p>Name of the organization conformance pack you want to create.</p> |
| `template_s3_uri` | String |  | <p>Location of file containing the template body. The uri must point to the conformance pack template
			(max size: 300 KB).</p>
         <note>
            <p>You must have access to read Amazon S3 bucket.
			In addition, in order to ensure a successful deployment, the template object must not be in an <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/storage-class-intro.html">archived storage class</a> if this parameter is passed.</p>
         </note> |
| `template_body` | String |  | <p>A string that contains the full conformance pack template body. Structure containing the template body
			with a minimum length of 1 byte and a maximum length of 51,200 bytes.</p> |
| `delivery_s3_bucket` | String |  | <p>The name of the Amazon S3 bucket where Config stores conformance pack templates.</p>
         <note>
            <p>This field is optional. If used, it must be prefixed with <code>awsconfigconforms</code>.</p>
         </note> |
| `delivery_s3_key_prefix` | String |  | <p>The prefix for the Amazon S3 bucket.</p>
         <note>
            <p>This field is optional.</p>
         </note> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create organization_conformance_pack
organization_conformance_pack = provider.config_service.Organization_conformance_pack {
    organization_conformance_pack_name = "value"  # <p>Name of the organization conformance pack you want to create.</p>
}

```

---


### Delivery_channel

DeliveryChannel resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delivery_channel` | String | ✅ | <p>An object for the delivery channel. A delivery channel sends notifications and updated configuration states.
		</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create delivery_channel
delivery_channel = provider.config_service.Delivery_channel {
    delivery_channel = "value"  # <p>An object for the delivery channel. A delivery channel sends notifications and updated configuration states.
		</p>
}

```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple conformance_pack_compliance_details resources
conformance_pack_compliance_details_0 = provider.config_service.Conformance_pack_compliance_details {
}
conformance_pack_compliance_details_1 = provider.config_service.Conformance_pack_compliance_details {
}
conformance_pack_compliance_details_2 = provider.config_service.Conformance_pack_compliance_details {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    conformance_pack_compliance_details = provider.config_service.Conformance_pack_compliance_details {
    }
```

---

## Related Documentation

- [AWS Config_service Documentation](https://docs.aws.amazon.com/config_service/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
