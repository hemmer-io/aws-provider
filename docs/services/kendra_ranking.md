# Kendra_ranking Service



**Resources**: 1

---

## Overview

The kendra_ranking service provides access to 1 resource type:

- [Rescore_execution_plan](#rescore_execution_plan) [CRUD]

---

## Resources


### Rescore_execution_plan

RescoreExecutionPlan resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description for the rescore execution plan.</p> |
| `name` | String | ✅ | <p>A name for the rescore execution plan.</p> |
| `tags` | Vec<String> |  | <p>A list of key-value pairs that identify or categorize your 
            rescore execution plan. You can also use tags to help control 
            access to the rescore execution plan. Tag keys and values can 
            consist of Unicode letters, digits, white space, and any of 
            the following symbols: _ . : / = + - @.</p> |
| `client_token` | String |  | <p>A token that you provide to identify the request to create 
            a rescore execution plan. Multiple calls to the 
            <code>CreateRescoreExecutionPlanRequest</code> API with the 
            same client token will create only one rescore execution plan.</p> |
| `capacity_units` | String |  | <p>You can set additional capacity units to meet the 
            needs of your rescore execution plan. You are given a single 
            capacity unit by default. If you want to use the default 
            capacity, you don't set additional capacity units. For more 
            information on the default capacity and additional capacity 
            units, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/adjusting-capacity.html">Adjusting 
                capacity</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | <p>The identifier of the rescore execution plan.</p> |
| `updated_at` | String | <p>The Unix timestamp of when the rescore execution plan was 
            last updated.</p> |
| `status` | String | <p>The current status of the rescore execution plan. When the 
            value is <code>ACTIVE</code>, the rescore execution plan is 
            ready for use. If the <code>Status</code> field value is 
            <code>FAILED</code>, the <code>ErrorMessage</code> field 
            contains a message that explains why.</p> |
| `capacity_units` | String | <p>The capacity units set for the rescore execution plan. 
            A capacity of zero indicates that the rescore execution 
            plan is using the default capacity. For more information on the 
            default capacity and additional capacity units, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/adjusting-capacity.html">Adjusting 
                capacity</a>.</p> |
| `error_message` | String | <p>When the <code>Status</code> field value is 
            <code>FAILED</code>, the <code>ErrorMessage</code> field 
            contains a message that explains why.</p> |
| `name` | String | <p>The name for the rescore execution plan.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the rescore execution 
            plan.</p> |
| `description` | String | <p>The description for the rescore execution plan.</p> |
| `created_at` | String | <p>The Unix timestamp of when the rescore execution plan was 
            created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create rescore_execution_plan
rescore_execution_plan = provider.kendra_ranking.Rescore_execution_plan {
    name = "value"  # <p>A name for the rescore execution plan.</p>
}

# Access rescore_execution_plan outputs
rescore_execution_plan_id = rescore_execution_plan.id
rescore_execution_plan_id = rescore_execution_plan.id
rescore_execution_plan_updated_at = rescore_execution_plan.updated_at
rescore_execution_plan_status = rescore_execution_plan.status
rescore_execution_plan_capacity_units = rescore_execution_plan.capacity_units
rescore_execution_plan_error_message = rescore_execution_plan.error_message
rescore_execution_plan_name = rescore_execution_plan.name
rescore_execution_plan_arn = rescore_execution_plan.arn
rescore_execution_plan_description = rescore_execution_plan.description
rescore_execution_plan_created_at = rescore_execution_plan.created_at
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple rescore_execution_plan resources
rescore_execution_plan_0 = provider.kendra_ranking.Rescore_execution_plan {
    name = "value-0"
}
rescore_execution_plan_1 = provider.kendra_ranking.Rescore_execution_plan {
    name = "value-1"
}
rescore_execution_plan_2 = provider.kendra_ranking.Rescore_execution_plan {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    rescore_execution_plan = provider.kendra_ranking.Rescore_execution_plan {
        name = "production-value"
    }
```

---

## Related Documentation

- [AWS Kendra_ranking Documentation](https://docs.aws.amazon.com/kendra_ranking/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
