# Arc_region_switch Service



**Resources**: 4

---

## Overview

The arc_region_switch service provides access to 4 resource types:

- [Plan_execution_step](#plan_execution_step) [U]
- [Plan_execution](#plan_execution) [RU]
- [Plan_in_region](#plan_in_region) [R]
- [Plan_evaluation_status](#plan_evaluation_status) [R]

---

## Resources


### Plan_execution_step

PlanExecutionStep resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action_to_take` | String | ✅ | <p>The updated action to take for the step. This can be used to skip or retry a step.</p> |
| `step_name` | String | ✅ | <p>The name of the execution step to update.</p> |
| `execution_id` | String | ✅ | <p>The unique identifier of the plan execution containing the step to update.</p> |
| `comment` | String | ✅ | <p>An optional comment about the plan execution.</p> |
| `plan_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the plan containing the execution step to update.</p> |



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


### Plan_execution

PlanExecution resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `plan_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the plan with the execution to update.</p> |
| `execution_id` | String | ✅ | <p>The execution identifier of a plan execution.</p> |
| `action` | String | ✅ | <p>The action specified for a plan execution, for example, Switch to Graceful or Pause.</p> |
| `comment` | String |  | <p>An optional comment about the plan execution.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `execution_region` | String | <p>The Amazon Web Services Region for a plan execution.</p> |
| `start_time` | String | <p>The time (UTC) when the plan execution started.</p> |
| `plan` | String | <p>The details of the Region switch plan.</p> |
| `updated_at` | String | <p>The timestamp when the plan execution was last updated.</p> |
| `next_token` | String | <p>Specifies that you want to receive the next page of results. Valid only if you received a <code>nextToken</code> response in the previous request. If you did, it indicates that more output is available. Set this parameter to the value provided by the previous call's <code>nextToken</code> response to request the next page of results.</p> |
| `execution_id` | String | <p>The execution identifier of a plan execution.</p> |
| `mode` | String | <p>The plan execution mode. Valid values are <code>Practice</code>, for testing without making actual changes, or <code>Recovery</code>, for actual traffic shifting and application recovery.</p> |
| `actual_recovery_time` | String | <p>The actual recovery time that Region switch calculates for a plan execution. Actual recovery time includes the time for the plan to run added to the time elapsed until the application health alarms that you've specified are healthy again.</p> |
| `end_time` | String | <p>The time (UTC) when the plan execution ended.</p> |
| `version` | String | <p>The version for the plan.</p> |
| `execution_state` | String | <p>The plan execution state. Provides the state of a plan execution, for example, In Progress or Paused by Operator.</p> |
| `plan_arn` | String | <p>The Amazon Resource Name (ARN) of the plan.</p> |
| `comment` | String | <p>A comment included on the plan execution.</p> |
| `execution_action` | String | <p>The plan execution action. Valid values are <code>Activate</code>, to activate an Amazon Web Services Region, or <code>Deactivate</code>, to deactivate a Region.</p> |
| `step_states` | Vec<String> | <p>The states of the steps in the plan execution.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access plan_execution outputs
plan_execution_id = plan_execution.id
plan_execution_execution_region = plan_execution.execution_region
plan_execution_start_time = plan_execution.start_time
plan_execution_plan = plan_execution.plan
plan_execution_updated_at = plan_execution.updated_at
plan_execution_next_token = plan_execution.next_token
plan_execution_execution_id = plan_execution.execution_id
plan_execution_mode = plan_execution.mode
plan_execution_actual_recovery_time = plan_execution.actual_recovery_time
plan_execution_end_time = plan_execution.end_time
plan_execution_version = plan_execution.version
plan_execution_execution_state = plan_execution.execution_state
plan_execution_plan_arn = plan_execution.plan_arn
plan_execution_comment = plan_execution.comment
plan_execution_execution_action = plan_execution.execution_action
plan_execution_step_states = plan_execution.step_states
```

---


### Plan_in_region

PlanInRegion resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `plan` | String | <p>The details of the Region switch plan.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access plan_in_region outputs
plan_in_region_id = plan_in_region.id
plan_in_region_plan = plan_in_region.plan
```

---


### Plan_evaluation_status

PlanEvaluationStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>Specifies that you want to receive the next page of results. Valid only if you received a <code>nextToken</code> response in the previous request. If you did, it indicates that more output is available. Set this parameter to the value provided by the previous call's <code>nextToken</code> response to request the next page of results.</p> |
| `warnings` | Vec<String> | <p>The current evaluation warnings for the plan. </p> |
| `last_evaluated_version` | String | <p>The version of the last evaluation of the plan.</p> |
| `region` | String | <p>The Amazon Web Services Region for the plan.</p> |
| `last_evaluation_time` | String | <p>The time of the last time that Region switch ran an evaluation of the plan.</p> |
| `plan_arn` | String | <p>The Amazon Resource Name (ARN) of the plan.</p> |
| `evaluation_state` | String | <p>The evaluation state for the plan.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access plan_evaluation_status outputs
plan_evaluation_status_id = plan_evaluation_status.id
plan_evaluation_status_next_token = plan_evaluation_status.next_token
plan_evaluation_status_warnings = plan_evaluation_status.warnings
plan_evaluation_status_last_evaluated_version = plan_evaluation_status.last_evaluated_version
plan_evaluation_status_region = plan_evaluation_status.region
plan_evaluation_status_last_evaluation_time = plan_evaluation_status.last_evaluation_time
plan_evaluation_status_plan_arn = plan_evaluation_status.plan_arn
plan_evaluation_status_evaluation_state = plan_evaluation_status.evaluation_state
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple plan_execution_step resources
plan_execution_step_0 = provider.arc_region_switch.Plan_execution_step {
    action_to_take = "value-0"
    step_name = "value-0"
    execution_id = "value-0"
    comment = "value-0"
    plan_arn = "value-0"
}
plan_execution_step_1 = provider.arc_region_switch.Plan_execution_step {
    action_to_take = "value-1"
    step_name = "value-1"
    execution_id = "value-1"
    comment = "value-1"
    plan_arn = "value-1"
}
plan_execution_step_2 = provider.arc_region_switch.Plan_execution_step {
    action_to_take = "value-2"
    step_name = "value-2"
    execution_id = "value-2"
    comment = "value-2"
    plan_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    plan_execution_step = provider.arc_region_switch.Plan_execution_step {
        action_to_take = "production-value"
        step_name = "production-value"
        execution_id = "production-value"
        comment = "production-value"
        plan_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Arc_region_switch Documentation](https://docs.aws.amazon.com/arc_region_switch/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
