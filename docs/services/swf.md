# Swf Service



**Resources**: 5

---

## Overview

The swf service provides access to 5 resource types:

- [Activity_type](#activity_type) [RD]
- [Workflow_execution_history](#workflow_execution_history) [R]
- [Workflow_execution](#workflow_execution) [R]
- [Workflow_type](#workflow_type) [RD]
- [Domain](#domain) [R]

---

## Resources


### Activity_type

ActivityType resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration` | String | <p>The configuration settings registered with the activity type.</p> |
| `type_info` | String | <p>General information about the activity type.</p>
         <p>The status of activity type (returned in the ActivityTypeInfo structure) can be one of the following.</p>
         <ul>
            <li>
               <p>
                  <code>REGISTERED</code> – The type is registered and available. Workers supporting this
        type should be running.
      </p>
            </li>
            <li>
               <p>
                  <code>DEPRECATED</code> – The type was deprecated using <a>DeprecateActivityType</a>, but is
        still in use. You should keep workers supporting this type running.
        You cannot create new tasks of this type.
      </p>
            </li>
         </ul> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access activity_type outputs
activity_type_id = activity_type.id
activity_type_configuration = activity_type.configuration
activity_type_type_info = activity_type.type_info
```

---


### Workflow_execution_history

WorkflowExecutionHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | <p>If a <code>NextPageToken</code> was returned by a previous call, there are more
  results available. To retrieve the next page of results, make the call again using the returned token in
  <code>nextPageToken</code>. Keep all other arguments unchanged.</p>
         <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p> |
| `events` | Vec<String> | <p>The list of history events.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workflow_execution_history outputs
workflow_execution_history_id = workflow_execution_history.id
workflow_execution_history_next_page_token = workflow_execution_history.next_page_token
workflow_execution_history_events = workflow_execution_history.events
```

---


### Workflow_execution

WorkflowExecution resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `execution_configuration` | String | <p>The configuration settings for this workflow execution including timeout values, tasklist etc.</p> |
| `latest_activity_task_timestamp` | String | <p>The time when the last activity task was scheduled for this workflow execution. You can use this information to determine if the workflow has not made progress for an unusually long period of time and might require a corrective action.</p> |
| `execution_info` | String | <p>Information about the workflow execution.</p> |
| `open_counts` | String | <p>The number of tasks for this workflow execution. This includes open and closed tasks of all types.</p> |
| `latest_execution_context` | String | <p>The latest executionContext provided by the decider for this workflow execution. A decider can provide an
      executionContext (a free-form string) when closing a decision task using <a>RespondDecisionTaskCompleted</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workflow_execution outputs
workflow_execution_id = workflow_execution.id
workflow_execution_execution_configuration = workflow_execution.execution_configuration
workflow_execution_latest_activity_task_timestamp = workflow_execution.latest_activity_task_timestamp
workflow_execution_execution_info = workflow_execution.execution_info
workflow_execution_open_counts = workflow_execution.open_counts
workflow_execution_latest_execution_context = workflow_execution.latest_execution_context
```

---


### Workflow_type

WorkflowType resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration` | String | <p>Configuration settings of the workflow type registered through <a>RegisterWorkflowType</a>
         </p> |
| `type_info` | String | <p>General information about the workflow type.</p>
         <p>The status of the workflow type (returned in the WorkflowTypeInfo structure) can be one of the following.</p>
         <ul>
            <li>
               <p>
                  <code>REGISTERED</code> – The type is registered and available. Workers supporting this type should be running.</p>
            </li>
            <li>
               <p>
                  <code>DEPRECATED</code> – The type was deprecated using <a>DeprecateWorkflowType</a>, but is still in use. You should
        keep workers supporting this type running. You cannot create new workflow executions of this type.</p>
            </li>
         </ul> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workflow_type outputs
workflow_type_id = workflow_type.id
workflow_type_configuration = workflow_type.configuration
workflow_type_type_info = workflow_type.type_info
```

---


### Domain

Domain resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_info` | String | <p>The basic information about a domain, such as its name, status, and
      description.</p> |
| `configuration` | String | <p>The domain configuration. Currently, this includes only the domain's retention
      period.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access domain outputs
domain_id = domain.id
domain_domain_info = domain.domain_info
domain_configuration = domain.configuration
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple activity_type resources
activity_type_0 = provider.swf.Activity_type {
}
activity_type_1 = provider.swf.Activity_type {
}
activity_type_2 = provider.swf.Activity_type {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    activity_type = provider.swf.Activity_type {
    }
```

---

## Related Documentation

- [AWS Swf Documentation](https://docs.aws.amazon.com/swf/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
