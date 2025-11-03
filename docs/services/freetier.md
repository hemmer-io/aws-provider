# Freetier Service



**Resources**: 3

---

## Overview

The freetier service provides access to 3 resource types:

- [Free_tier_usage](#free_tier_usage) [R]
- [Account_activity](#account_activity) [R]
- [Account_plan_state](#account_plan_state) [R]

---

## Resources


### Free_tier_usage

FreeTierUsage resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `free_tier_usages` | Vec<String> | <p>The list of Free Tier usage objects that meet your filter expression.</p> |
| `next_token` | String | <p>The pagination token that indicates the next set of results to retrieve.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access free_tier_usage outputs
free_tier_usage_id = free_tier_usage.id
free_tier_usage_free_tier_usages = free_tier_usage.free_tier_usages
free_tier_usage_next_token = free_tier_usage.next_token
```

---


### Account_activity

AccountActivity resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `activity_id` | String | <p> A unique identifier that identifies the activity. </p> |
| `title` | String | <p> A short activity title. </p> |
| `expires_at` | String | <p> The time by which the activity must be completed to receive a reward. </p> |
| `instructions_url` | String | <p> The URL resource that provides guidance on activity requirements and completion. </p> |
| `completed_at` | String | <p> The timestamp when the activity is completed. This field appears only for activities in the <code>COMPLETED</code> state. </p> |
| `description` | String | <p> Provides detailed information about the activity and its expected outcomes. </p> |
| `status` | String | <p> The current activity status. </p> |
| `reward` | String | <p> A reward granted upon activity completion. </p> |
| `started_at` | String | <p> The timestamp when the activity started. This field appears only for activities in the <code>IN_PROGRESS</code> or <code>COMPLETED</code> states. </p> |
| `estimated_time_to_complete_in_minutes` | i64 | <p> The estimated time to complete the activity. This is the duration in minutes. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_activity outputs
account_activity_id = account_activity.id
account_activity_activity_id = account_activity.activity_id
account_activity_title = account_activity.title
account_activity_expires_at = account_activity.expires_at
account_activity_instructions_url = account_activity.instructions_url
account_activity_completed_at = account_activity.completed_at
account_activity_description = account_activity.description
account_activity_status = account_activity.status
account_activity_reward = account_activity.reward
account_activity_started_at = account_activity.started_at
account_activity_estimated_time_to_complete_in_minutes = account_activity.estimated_time_to_complete_in_minutes
```

---


### Account_plan_state

AccountPlanState resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_id` | String | <p> A unique identifier that identifies the account. </p> |
| `account_plan_remaining_credits` | String | <p> The amount of credits remaining for the account. </p> |
| `account_plan_status` | String | <p> The current status for the account plan. </p> |
| `account_plan_expiration_date` | String | <p> The timestamp for when the current account plan expires. </p> |
| `account_plan_type` | String | <p> The plan type for the account. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_plan_state outputs
account_plan_state_id = account_plan_state.id
account_plan_state_account_id = account_plan_state.account_id
account_plan_state_account_plan_remaining_credits = account_plan_state.account_plan_remaining_credits
account_plan_state_account_plan_status = account_plan_state.account_plan_status
account_plan_state_account_plan_expiration_date = account_plan_state.account_plan_expiration_date
account_plan_state_account_plan_type = account_plan_state.account_plan_type
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple free_tier_usage resources
free_tier_usage_0 = provider.freetier.Free_tier_usage {
}
free_tier_usage_1 = provider.freetier.Free_tier_usage {
}
free_tier_usage_2 = provider.freetier.Free_tier_usage {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    free_tier_usage = provider.freetier.Free_tier_usage {
    }
```

---

## Related Documentation

- [AWS Freetier Documentation](https://docs.aws.amazon.com/freetier/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
