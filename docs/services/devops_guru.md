# Devops_guru Service



**Resources**: 13

---

## Overview

The devops_guru service provides access to 13 resource types:

- [Insight](#insight) [RD]
- [Organization_resource_collection_health](#organization_resource_collection_health) [R]
- [Feedback](#feedback) [CR]
- [Cost_estimation](#cost_estimation) [R]
- [Resource_collection_health](#resource_collection_health) [R]
- [Account_overview](#account_overview) [R]
- [Event_sources_config](#event_sources_config) [RU]
- [Organization_overview](#organization_overview) [R]
- [Anomaly](#anomaly) [R]
- [Service_integration](#service_integration) [RU]
- [Account_health](#account_health) [R]
- [Resource_collection](#resource_collection) [RU]
- [Organization_health](#organization_health) [R]

---

## Resources


### Insight

Insight resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `proactive_insight` | String | <p> A <code>ProactiveInsight</code> object that represents the requested insight. </p> |
| `reactive_insight` | String | <p> A <code>ReactiveInsight</code> object that represents the requested insight. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access insight outputs
insight_id = insight.id
insight_proactive_insight = insight.proactive_insight
insight_reactive_insight = insight.reactive_insight
```

---


### Organization_resource_collection_health

OrganizationResourceCollectionHealth resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cloud_formation` | Vec<String> | <p>The returned <code>CloudFormationHealthOverview</code> object that contains an
				<code>InsightHealthOverview</code> object with the requested system health
			information.</p> |
| `service` | Vec<String> | <p>An array of <code>ServiceHealth</code> objects that describes the health of the Amazon Web Services
			services associated with the resources in the collection.</p> |
| `next_token` | String | <p>The pagination token to use to retrieve 
   the next page of results for this operation. If there are no more pages, this value is null.</p> |
| `tags` | Vec<String> | <p>Tags help you identify and organize your Amazon Web Services resources. Many Amazon Web Services services support
   		tagging, so you can assign the same tag to resources from different services to indicate
   		that the resources are related. For example, you can assign the same tag to an Amazon DynamoDB
   		table resource that you assign to an Lambda function. For more information about
   		using tags, see the <a href="https://docs.aws.amazon.com/whitepapers/latest/tagging-best-practices/tagging-best-practices.html">Tagging
   			best practices</a> whitepaper. </p>
         <p>Each Amazon Web Services tag has two parts. </p>
         <ul>
            <li>
               <p>A tag <i>key</i> (for example, <code>CostCenter</code>,
   				<code>Environment</code>, <code>Project</code>, or <code>Secret</code>). Tag
   				<i>keys</i> are case-sensitive.</p>
            </li>
            <li>
               <p>An optional field known as a tag <i>value</i> (for example,
   				<code>111122223333</code>, <code>Production</code>, or a team
   				name). Omitting the tag <i>value</i> is the same as using an empty
   				string. Like tag <i>keys</i>, tag <i>values</i> are
   				case-sensitive.</p>
            </li>
         </ul>
         <p>Together these are known as <i>key</i>-<i>value</i> pairs.</p>
         <important>
            <p>The string used for a <i>key</i> in a tag that you use to define your resource coverage must begin with the
			prefix <code>Devops-guru-</code>. The tag <i>key</i> might be
			<code>DevOps-Guru-deployment-application</code> or
			<code>devops-guru-rds-application</code>. When you create a <i>key</i>, the case of characters in the <i>key</i> can be whatever you choose. After you create a <i>key</i>, it is case-sensitive.
			 For example, DevOps Guru works with a
			<i>key</i> named <code>devops-guru-rds</code> and a <i>key</i> named
			<code>DevOps-Guru-RDS</code>, and these act as two different <i>keys</i>. Possible <i>key</i>/<i>value</i> pairs in your
			application might be <code>Devops-Guru-production-application/RDS</code> or
			<code>Devops-Guru-production-application/containers</code>.</p>
         </important> |
| `account` | Vec<String> | <p>The name of the organization's account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organization_resource_collection_health outputs
organization_resource_collection_health_id = organization_resource_collection_health.id
organization_resource_collection_health_cloud_formation = organization_resource_collection_health.cloud_formation
organization_resource_collection_health_service = organization_resource_collection_health.service
organization_resource_collection_health_next_token = organization_resource_collection_health.next_token
organization_resource_collection_health_tags = organization_resource_collection_health.tags
organization_resource_collection_health_account = organization_resource_collection_health.account
```

---


### Feedback

Feedback resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `insight_feedback` | String |  | <p> The feedback from customers is about the recommendations in this insight. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `insight_feedback` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create feedback
feedback = provider.devops_guru.Feedback {
}

# Access feedback outputs
feedback_id = feedback.id
feedback_insight_feedback = feedback.insight_feedback
```

---


### Cost_estimation

CostEstimation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `costs` | Vec<String> | <p>An array of <code>ResourceCost</code> objects that each contains details about the
			monthly cost estimate to analyze one of your Amazon Web Services resources.</p> |
| `time_range` | String | <p>The start and end time of the cost estimation.</p> |
| `next_token` | String | <p>The pagination token to use to retrieve 
   the next page of results for this operation. If there are no more pages, this value is null.</p> |
| `total_cost` | f64 | <p>The estimated monthly cost to analyze the Amazon Web Services resources. This value is the sum of
			the estimated costs to analyze each resource in the <code>Costs</code> object in this
			response.</p> |
| `status` | String | <p>The status of creating this cost estimate. If it's still in progress, the status
				<code>ONGOING</code> is returned. If it is finished, the status
				<code>COMPLETED</code> is returned.</p> |
| `resource_collection` | String | <p>The collection of the Amazon Web Services resources used to create your monthly DevOps Guru cost
			estimate.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cost_estimation outputs
cost_estimation_id = cost_estimation.id
cost_estimation_costs = cost_estimation.costs
cost_estimation_time_range = cost_estimation.time_range
cost_estimation_next_token = cost_estimation.next_token
cost_estimation_total_cost = cost_estimation.total_cost
cost_estimation_status = cost_estimation.status
cost_estimation_resource_collection = cost_estimation.resource_collection
```

---


### Resource_collection_health

ResourceCollectionHealth resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cloud_formation` | Vec<String> | <p> The returned <code>CloudFormationHealthOverview</code> object that contains an
				<code>InsightHealthOverview</code> object with the requested system health
			information. </p> |
| `service` | Vec<String> | <p>An array of <code>ServiceHealth</code> objects that describes the health of the Amazon Web Services
			services associated with the resources in the collection.</p> |
| `next_token` | String | <p>The pagination token to use to retrieve 
   the next page of results for this operation. If there are no more pages, this value is null.</p> |
| `tags` | Vec<String> | <p>The Amazon Web Services tags that are used by resources in the resource collection.</p>
         <p>Tags help you identify and organize your Amazon Web Services resources. Many Amazon Web Services services support
   		tagging, so you can assign the same tag to resources from different services to indicate
   		that the resources are related. For example, you can assign the same tag to an Amazon DynamoDB
   		table resource that you assign to an Lambda function. For more information about
   		using tags, see the <a href="https://docs.aws.amazon.com/whitepapers/latest/tagging-best-practices/tagging-best-practices.html">Tagging
   			best practices</a> whitepaper. </p>
         <p>Each Amazon Web Services tag has two parts. </p>
         <ul>
            <li>
               <p>A tag <i>key</i> (for example, <code>CostCenter</code>,
   				<code>Environment</code>, <code>Project</code>, or <code>Secret</code>). Tag
   				<i>keys</i> are case-sensitive.</p>
            </li>
            <li>
               <p>An optional field known as a tag <i>value</i> (for example,
   				<code>111122223333</code>, <code>Production</code>, or a team
   				name). Omitting the tag <i>value</i> is the same as using an empty
   				string. Like tag <i>keys</i>, tag <i>values</i> are
   				case-sensitive.</p>
            </li>
         </ul>
         <p>Together these are known as <i>key</i>-<i>value</i> pairs.</p>
         <important>
            <p>The string used for a <i>key</i> in a tag that you use to define your resource coverage must begin with the
			prefix <code>Devops-guru-</code>. The tag <i>key</i> might be
			<code>DevOps-Guru-deployment-application</code> or
			<code>devops-guru-rds-application</code>. When you create a <i>key</i>, the case of characters in the <i>key</i> can be whatever you choose. After you create a <i>key</i>, it is case-sensitive.
			 For example, DevOps Guru works with a
			<i>key</i> named <code>devops-guru-rds</code> and a <i>key</i> named
			<code>DevOps-Guru-RDS</code>, and these act as two different <i>keys</i>. Possible <i>key</i>/<i>value</i> pairs in your
			application might be <code>Devops-Guru-production-application/RDS</code> or
			<code>Devops-Guru-production-application/containers</code>.</p>
         </important> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_collection_health outputs
resource_collection_health_id = resource_collection_health.id
resource_collection_health_cloud_formation = resource_collection_health.cloud_formation
resource_collection_health_service = resource_collection_health.service
resource_collection_health_next_token = resource_collection_health.next_token
resource_collection_health_tags = resource_collection_health.tags
```

---


### Account_overview

AccountOverview resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `proactive_insights` | i64 | <p> An integer that specifies the number of open proactive insights in your Amazon Web Services account
			that were created during the time range passed in. </p> |
| `reactive_insights` | i64 | <p> An integer that specifies the number of open reactive insights in your Amazon Web Services account
			that were created during the time range passed in. </p> |
| `mean_time_to_recover_in_milliseconds` | i64 | <p> The Mean Time to Recover (MTTR) for all closed insights that were created during the time range passed in.
		</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_overview outputs
account_overview_id = account_overview.id
account_overview_proactive_insights = account_overview.proactive_insights
account_overview_reactive_insights = account_overview.reactive_insights
account_overview_mean_time_to_recover_in_milliseconds = account_overview.mean_time_to_recover_in_milliseconds
```

---


### Event_sources_config

EventSourcesConfig resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_sources` | String |  | <p>Configuration information about the integration of DevOps Guru as the Consumer via
			EventBridge with another AWS Service.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_sources` | String | <p>Lists the event sources in the configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access event_sources_config outputs
event_sources_config_id = event_sources_config.id
event_sources_config_event_sources = event_sources_config.event_sources
```

---


### Organization_overview

OrganizationOverview resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `proactive_insights` | i64 | <p>An integer that specifies the number of open proactive insights in your Amazon Web Services
			account.</p> |
| `reactive_insights` | i64 | <p>An integer that specifies the number of open reactive insights in your Amazon Web Services
			account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organization_overview outputs
organization_overview_id = organization_overview.id
organization_overview_proactive_insights = organization_overview.proactive_insights
organization_overview_reactive_insights = organization_overview.reactive_insights
```

---


### Anomaly

Anomaly resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `proactive_anomaly` | String | <p> A <code>ProactiveAnomaly</code> object that represents the requested anomaly. </p> |
| `reactive_anomaly` | String | <p> A <code>ReactiveAnomaly</code> object that represents the requested anomaly. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access anomaly outputs
anomaly_id = anomaly.id
anomaly_proactive_anomaly = anomaly.proactive_anomaly
anomaly_reactive_anomaly = anomaly.reactive_anomaly
```

---


### Service_integration

ServiceIntegration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_integration` | String | ✅ | <p> An <code>IntegratedServiceConfig</code> object used to specify the integrated service
			you want to update, and whether you want to update it to enabled or disabled. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_integration` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_integration outputs
service_integration_id = service_integration.id
service_integration_service_integration = service_integration.service_integration
```

---


### Account_health

AccountHealth resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `open_reactive_insights` | i64 | <p> An integer that specifies the number of open reactive insights in your Amazon Web Services account.
		</p> |
| `open_proactive_insights` | i64 | <p> An integer that specifies the number of open proactive insights in your Amazon Web Services
			account. </p> |
| `metrics_analyzed` | i64 | <p> An integer that specifies the number of metrics that have been analyzed in your Amazon Web Services
			account. </p> |
| `analyzed_resource_count` | i64 | <p>
			Number of resources that DevOps Guru is monitoring in your Amazon Web Services account.
		</p> |
| `resource_hours` | i64 | <p>The number of Amazon DevOps Guru resource analysis hours billed to the current Amazon Web Services account in
			the last hour. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_health outputs
account_health_id = account_health.id
account_health_open_reactive_insights = account_health.open_reactive_insights
account_health_open_proactive_insights = account_health.open_proactive_insights
account_health_metrics_analyzed = account_health.metrics_analyzed
account_health_analyzed_resource_count = account_health.analyzed_resource_count
account_health_resource_hours = account_health.resource_hours
```

---


### Resource_collection

ResourceCollection resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action` | String | ✅ | <p> Specifies if the resource collection in the request is added or deleted to the
			resource collection. </p> |
| `resource_collection` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_collection` | String | <p> The requested list of Amazon Web Services resource collections.
			The two types of Amazon Web Services resource collections supported are Amazon Web Services CloudFormation stacks and 
          Amazon Web Services resources that contain the same Amazon Web Services tag. DevOps Guru can be configured to analyze 
      	the Amazon Web Services resources that are defined in the stacks or that are tagged using the same tag <i>key</i>. You can specify up to 500 Amazon Web Services CloudFormation stacks. </p> |
| `next_token` | String | <p>The pagination token to use to retrieve 
   the next page of results for this operation. If there are no more pages, this value is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_collection outputs
resource_collection_id = resource_collection.id
resource_collection_resource_collection = resource_collection.resource_collection
resource_collection_next_token = resource_collection.next_token
```

---


### Organization_health

OrganizationHealth resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `open_reactive_insights` | i64 | <p>An integer that specifies the number of open reactive insights in your Amazon Web Services
			account.</p> |
| `open_proactive_insights` | i64 | <p>An integer that specifies the number of open proactive insights in your Amazon Web Services
			account.</p> |
| `metrics_analyzed` | i64 | <p>An integer that specifies the number of metrics that have been analyzed in your
			organization.</p> |
| `resource_hours` | i64 | <p>The number of Amazon DevOps Guru resource analysis hours billed to the current Amazon Web Services account in
			the last hour. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organization_health outputs
organization_health_id = organization_health.id
organization_health_open_reactive_insights = organization_health.open_reactive_insights
organization_health_open_proactive_insights = organization_health.open_proactive_insights
organization_health_metrics_analyzed = organization_health.metrics_analyzed
organization_health_resource_hours = organization_health.resource_hours
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple insight resources
insight_0 = provider.devops_guru.Insight {
}
insight_1 = provider.devops_guru.Insight {
}
insight_2 = provider.devops_guru.Insight {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    insight = provider.devops_guru.Insight {
    }
```

---

## Related Documentation

- [AWS Devops_guru Documentation](https://docs.aws.amazon.com/devops_guru/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
