# Auto_scaling_plans Service



**Resources**: 4

---

## Overview

The auto_scaling_plans service provides access to 4 resource types:

- [Scaling_plans](#scaling_plans) [R]
- [Scaling_plan_resources](#scaling_plan_resources) [R]
- [Scaling_plan_resource_forecast_data](#scaling_plan_resource_forecast_data) [R]
- [Scaling_plan](#scaling_plan) [CUD]

---

## Resources


### Scaling_plans

ScalingPlans resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scaling_plans` | Vec<String> | <p>Information about the scaling plans.</p> |
| `next_token` | String | <p>The token required to get the next set of results. This value is <code>null</code> if
         there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access scaling_plans outputs
scaling_plans_id = scaling_plans.id
scaling_plans_scaling_plans = scaling_plans.scaling_plans
scaling_plans_next_token = scaling_plans.next_token
```

---


### Scaling_plan_resources

ScalingPlanResources resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token required to get the next set of results. This value is <code>null</code> if
         there are no more results to return.</p> |
| `scaling_plan_resources` | Vec<String> | <p>Information about the scalable resources.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access scaling_plan_resources outputs
scaling_plan_resources_id = scaling_plan_resources.id
scaling_plan_resources_next_token = scaling_plan_resources.next_token
scaling_plan_resources_scaling_plan_resources = scaling_plan_resources.scaling_plan_resources
```

---


### Scaling_plan_resource_forecast_data

ScalingPlanResourceForecastData resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `datapoints` | Vec<String> | <p>The data points to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access scaling_plan_resource_forecast_data outputs
scaling_plan_resource_forecast_data_id = scaling_plan_resource_forecast_data.id
scaling_plan_resource_forecast_data_datapoints = scaling_plan_resource_forecast_data.datapoints
```

---


### Scaling_plan

ScalingPlan resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scaling_plan_name` | String | ✅ | <p>The name of the scaling plan. Names cannot contain vertical bars, colons, or forward
         slashes.</p> |
| `application_source` | String | ✅ | <p>A CloudFormation stack or set of tags. You can create one scaling plan per application
         source.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/plans/APIReference/API_ApplicationSource.html">ApplicationSource</a> in the <i>AWS Auto Scaling API Reference</i>.</p> |
| `scaling_instructions` | Vec<String> | ✅ | <p>The scaling instructions.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/plans/APIReference/API_ScalingInstruction.html">ScalingInstruction</a> in the <i>AWS Auto Scaling API Reference</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create scaling_plan
scaling_plan = provider.auto_scaling_plans.Scaling_plan {
    scaling_plan_name = "value"  # <p>The name of the scaling plan. Names cannot contain vertical bars, colons, or forward
         slashes.</p>
    application_source = "value"  # <p>A CloudFormation stack or set of tags. You can create one scaling plan per application
         source.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/plans/APIReference/API_ApplicationSource.html">ApplicationSource</a> in the <i>AWS Auto Scaling API Reference</i>.</p>
    scaling_instructions = "value"  # <p>The scaling instructions.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/plans/APIReference/API_ScalingInstruction.html">ScalingInstruction</a> in the <i>AWS Auto Scaling API Reference</i>.</p>
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

# Create multiple scaling_plans resources
scaling_plans_0 = provider.auto_scaling_plans.Scaling_plans {
}
scaling_plans_1 = provider.auto_scaling_plans.Scaling_plans {
}
scaling_plans_2 = provider.auto_scaling_plans.Scaling_plans {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    scaling_plans = provider.auto_scaling_plans.Scaling_plans {
    }
```

---

## Related Documentation

- [AWS Auto_scaling_plans Documentation](https://docs.aws.amazon.com/auto_scaling_plans/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
