# Application_insights Service



**Resources**: 9

---

## Overview

The application_insights service provides access to 9 resource types:

- [Component_configuration_recommendation](#component_configuration_recommendation) [R]
- [Component](#component) [CRUD]
- [Problem](#problem) [RU]
- [Component_configuration](#component_configuration) [RU]
- [Log_pattern](#log_pattern) [CRUD]
- [Observation](#observation) [R]
- [Problem_observations](#problem_observations) [R]
- [Workload](#workload) [RU]
- [Application](#application) [CRUD]

---

## Resources


### Component_configuration_recommendation

ComponentConfigurationRecommendation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `component_configuration` | String | <p>The recommended configuration settings of the component. The value is the escaped JSON
         of the configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access component_configuration_recommendation outputs
component_configuration_recommendation_id = component_configuration_recommendation.id
component_configuration_recommendation_component_configuration = component_configuration_recommendation.component_configuration
```

---


### Component

Component resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `component_name` | String | ✅ | <p>The name of the component.</p> |
| `resource_group_name` | String | ✅ | <p>The name of the resource group.</p> |
| `resource_list` | Vec<String> | ✅ | <p>The list of resource ARNs that belong to the component.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `application_component` | String |  |
| `resource_list` | Vec<String> | <p>The list of resource ARNs that belong to the component.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create component
component = provider.application_insights.Component {
    component_name = "value"  # <p>The name of the component.</p>
    resource_group_name = "value"  # <p>The name of the resource group.</p>
    resource_list = "value"  # <p>The list of resource ARNs that belong to the component.</p>
}

# Access component outputs
component_id = component.id
component_application_component = component.application_component
component_resource_list = component.resource_list
```

---


### Problem

Problem resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_status` | String |  | <p>The status of the problem. Arguments can be passed for only problems that show a status
         of <code>RECOVERING</code>.</p> |
| `problem_id` | String | ✅ | <p>The ID of the problem.</p> |
| `visibility` | String |  | <p>The visibility of a problem. When you pass a value of <code>IGNORED</code>, the problem
         is removed from the default view, and all notifications for the problem are suspended. When
         <code>VISIBLE</code> is passed, the <code>IGNORED</code> action is reversed.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `problem` | String | <p>Information about the problem. </p> |
| `sns_notification_arn` | String | <p>
         The SNS notification topic ARN of the problem.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access problem outputs
problem_id = problem.id
problem_problem = problem.problem
problem_sns_notification_arn = problem.sns_notification_arn
```

---


### Component_configuration

ComponentConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auto_config_enabled` | bool |  | <p> Automatically configures the component by applying the recommended configurations.
      </p> |
| `resource_group_name` | String | ✅ | <p>The name of the resource group.</p> |
| `monitor` | bool |  | <p>Indicates whether the application component is monitored.</p> |
| `component_name` | String | ✅ | <p>The name of the component.</p> |
| `tier` | String |  | <p>The tier of the application component.</p> |
| `component_configuration` | String |  | <p>The configuration settings of the component. The value is the escaped JSON of the
         configuration. For more information about the JSON format, see <a href="https://docs.aws.amazon.com/sdk-for-javascript/v2/developer-guide/working-with-json.html">Working with JSON</a>. You can send a request to
            <code>DescribeComponentConfigurationRecommendation</code> to see the recommended
         configuration for a component. For the complete format of the component configuration file,
         see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/component-config.html">Component Configuration</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `component_configuration` | String | <p>The configuration settings of the component. The value is the escaped JSON of the
         configuration.</p> |
| `monitor` | bool | <p>Indicates whether the application component is monitored.</p> |
| `tier` | String | <p>The tier of the application component. Supported tiers include
         <code>DOT_NET_CORE</code>, <code>DOT_NET_WORKER</code>, <code>DOT_NET_WEB</code>,
            <code>SQL_SERVER</code>, and <code>DEFAULT</code>
         </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access component_configuration outputs
component_configuration_id = component_configuration.id
component_configuration_component_configuration = component_configuration.component_configuration
component_configuration_monitor = component_configuration.monitor
component_configuration_tier = component_configuration.tier
```

---


### Log_pattern

LogPattern resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pattern_set_name` | String | ✅ | <p>The name of the log pattern set.</p> |
| `rank` | i64 | ✅ | <p>Rank of the log pattern. Must be a value between <code>1</code> and
            <code>1,000,000</code>. The patterns are sorted by rank, so we recommend that you set
         your highest priority patterns with the lowest rank. A pattern of rank <code>1</code> will
         be the first to get matched to a log line. A pattern of rank <code>1,000,000</code> will be
         last to get matched. When you configure custom log patterns from the console, a
            <code>Low</code> severity pattern translates to a <code>750,000</code> rank. A
            <code>Medium</code> severity pattern translates to a <code>500,000</code> rank. And a
            <code>High</code> severity pattern translates to a <code>250,000</code> rank. Rank
         values less than <code>1</code> or greater than <code>1,000,000</code> are reserved for
         Amazon Web Services provided patterns. </p> |
| `pattern` | String | ✅ | <p>The log pattern. The pattern must be DFA compatible. Patterns that utilize forward
         lookahead or backreference constructions are not supported.</p> |
| `resource_group_name` | String | ✅ | <p>The name of the resource group.</p> |
| `pattern_name` | String | ✅ | <p>The name of the log pattern.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `log_pattern` | String | <p>The successfully created log pattern.</p> |
| `account_id` | String | <p>The Amazon Web Services account ID for the resource group owner.</p> |
| `resource_group_name` | String | <p>The name of the resource group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create log_pattern
log_pattern = provider.application_insights.Log_pattern {
    pattern_set_name = "value"  # <p>The name of the log pattern set.</p>
    rank = "value"  # <p>Rank of the log pattern. Must be a value between <code>1</code> and
            <code>1,000,000</code>. The patterns are sorted by rank, so we recommend that you set
         your highest priority patterns with the lowest rank. A pattern of rank <code>1</code> will
         be the first to get matched to a log line. A pattern of rank <code>1,000,000</code> will be
         last to get matched. When you configure custom log patterns from the console, a
            <code>Low</code> severity pattern translates to a <code>750,000</code> rank. A
            <code>Medium</code> severity pattern translates to a <code>500,000</code> rank. And a
            <code>High</code> severity pattern translates to a <code>250,000</code> rank. Rank
         values less than <code>1</code> or greater than <code>1,000,000</code> are reserved for
         Amazon Web Services provided patterns. </p>
    pattern = "value"  # <p>The log pattern. The pattern must be DFA compatible. Patterns that utilize forward
         lookahead or backreference constructions are not supported.</p>
    resource_group_name = "value"  # <p>The name of the resource group.</p>
    pattern_name = "value"  # <p>The name of the log pattern.</p>
}

# Access log_pattern outputs
log_pattern_id = log_pattern.id
log_pattern_log_pattern = log_pattern.log_pattern
log_pattern_account_id = log_pattern.account_id
log_pattern_resource_group_name = log_pattern.resource_group_name
```

---


### Observation

Observation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `observation` | String | <p>Information about the observation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access observation outputs
observation_id = observation.id
observation_observation = observation.observation
```

---


### Problem_observations

ProblemObservations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `related_observations` | String | <p>Observations related to the problem.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access problem_observations outputs
problem_observations_id = problem_observations.id
problem_observations_related_observations = problem_observations.related_observations
```

---


### Workload

Workload resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `workload_id` | String |  | <p>The ID of the workload.</p> |
| `workload_configuration` | String | ✅ | <p>The configuration settings of the workload. The value is the escaped JSON of the configuration.</p> |
| `resource_group_name` | String | ✅ | <p>The name of the resource group.</p> |
| `component_name` | String | ✅ | <p> The name of the component. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workload_remarks` | String | <p>If logging is supported for the resource type, shows whether the component has configured logs to be monitored.</p> |
| `workload_configuration` | String | <p>The configuration settings of the workload. The value is the escaped JSON of the configuration.</p> |
| `workload_id` | String | <p>The ID of the workload.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workload outputs
workload_id = workload.id
workload_workload_remarks = workload.workload_remarks
workload_workload_configuration = workload.workload_configuration
workload_workload_id = workload.workload_id
```

---


### Application

Application resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sns_notification_arn` | String |  | <p>
         The SNS notification topic ARN.
      </p> |
| `auto_config_enabled` | bool |  | <p> Indicates whether Application Insights automatically configures unmonitored resources
         in the resource group. </p> |
| `cwe_monitor_enabled` | bool |  | <p> Indicates whether Application Insights can listen to CloudWatch events for the
         application resources, such as <code>instance terminated</code>, <code>failed
            deployment</code>, and others. </p> |
| `attach_missing_permission` | bool |  | <p>If set to true, the managed policies for SSM and CW will be attached to the instance roles if they are missing.</p> |
| `tags` | Vec<String> |  | <p>List of tags to add to the application. tag key (<code>Key</code>) and an associated tag
         value (<code>Value</code>). The maximum length of a tag key is 128 characters. The maximum
         length of a tag value is 256 characters.</p> |
| `ops_item_sns_topic_arn` | String |  | <p> The SNS topic provided to Application Insights that is associated to the created
         opsItem. Allows you to receive notifications for updates to the opsItem. </p> |
| `auto_create` | bool |  | <p> Configures all of the resources in the resource group by applying the recommended
         configurations. </p> |
| `resource_group_name` | String |  | <p>The name of the resource group.</p> |
| `grouping_type` | String |  | <p>Application Insights can create applications based on a resource group or on an account.
         To create an account-based application using all of the resources in the account, set this
         parameter to <code>ACCOUNT_BASED</code>. </p> |
| `ops_center_enabled` | bool |  | <p> When set to <code>true</code>, creates opsItems for any problems detected on an
         application. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `application_info` | String | <p>Information about the application.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application
application = provider.application_insights.Application {
}

# Access application outputs
application_id = application.id
application_application_info = application.application_info
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple component_configuration_recommendation resources
component_configuration_recommendation_0 = provider.application_insights.Component_configuration_recommendation {
}
component_configuration_recommendation_1 = provider.application_insights.Component_configuration_recommendation {
}
component_configuration_recommendation_2 = provider.application_insights.Component_configuration_recommendation {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    component_configuration_recommendation = provider.application_insights.Component_configuration_recommendation {
    }
```

---

## Related Documentation

- [AWS Application_insights Documentation](https://docs.aws.amazon.com/application_insights/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
