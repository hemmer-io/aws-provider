# Deadline Service



**Resources**: 3

---

## Overview

The deadline service provides access to 3 resource types:

- [Queue_fleet_association](#queue_fleet_association) [CRUD]
- [Sessions_statistics_aggregation](#sessions_statistics_aggregation) [R]
- [Queue_limit_association](#queue_limit_association) [CRUD]

---

## Resources


### Queue_fleet_association

QueueFleetAssociation resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `queue_id` | String | ✅ | <p>The queue ID.</p> |
| `fleet_id` | String | ✅ | <p>The fleet ID.</p> |
| `farm_id` | String | ✅ | <p>The ID of the farm that the queue and fleet belong to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `queue_id` | String | <p>The queue ID for the queue-fleet association.</p> |
| `created_at` | String | <p>The date and time the resource was created.</p> |
| `fleet_id` | String | <p>The fleet ID for the queue-fleet association.</p> |
| `updated_at` | String | <p>The date and time the resource was updated.</p> |
| `updated_by` | String | <p>The user or system that updated this resource.</p> |
| `status` | String | <p>The status of the queue-fleet association.</p> |
| `created_by` | String | <p>The user or system that created this resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create queue_fleet_association
queue_fleet_association = provider.deadline.Queue_fleet_association {
    queue_id = "value"  # <p>The queue ID.</p>
    fleet_id = "value"  # <p>The fleet ID.</p>
    farm_id = "value"  # <p>The ID of the farm that the queue and fleet belong to.</p>
}

# Access queue_fleet_association outputs
queue_fleet_association_id = queue_fleet_association.id
queue_fleet_association_queue_id = queue_fleet_association.queue_id
queue_fleet_association_created_at = queue_fleet_association.created_at
queue_fleet_association_fleet_id = queue_fleet_association.fleet_id
queue_fleet_association_updated_at = queue_fleet_association.updated_at
queue_fleet_association_updated_by = queue_fleet_association.updated_by
queue_fleet_association_status = queue_fleet_association.status
queue_fleet_association_created_by = queue_fleet_association.created_by
```

---


### Sessions_statistics_aggregation

SessionsStatisticsAggregation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `statistics` | Vec<String> | <p>The statistics for the specified fleets or queues.</p> |
| `next_token` | String | <p>If Deadline Cloud returns <code>nextToken</code>, then there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. To retrieve the next page, call the operation again using the returned token. Keep all other arguments unchanged. If no results remain, then <code>nextToken</code> is set to <code>null</code>. Each pagination token expires after 24 hours. If you provide a token that isn't valid, then you receive an HTTP 400 <code>ValidationException</code> error.</p> |
| `status` | String | <p>The status of the aggregated results. An aggregation may fail or time out if the results
         are too large. If this happens, you can call the
            <code>StartSessionsStatisticsAggregation</code> operation after you reduce the
         aggregation time frame, reduce the number of queues or fleets in the aggregation, or
         increase the period length.</p>
         <p>If you call the <code>StartSessionsStatisticsAggregation </code> operation when the
         status is <code>IN_PROGRESS</code>, you will receive a
         <code>ThrottlingException</code>.</p> |
| `status_message` | String | <p>A message that describes the status.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sessions_statistics_aggregation outputs
sessions_statistics_aggregation_id = sessions_statistics_aggregation.id
sessions_statistics_aggregation_statistics = sessions_statistics_aggregation.statistics
sessions_statistics_aggregation_next_token = sessions_statistics_aggregation.next_token
sessions_statistics_aggregation_status = sessions_statistics_aggregation.status
sessions_statistics_aggregation_status_message = sessions_statistics_aggregation.status_message
```

---


### Queue_limit_association

QueueLimitAssociation resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `limit_id` | String | ✅ | <p>The unique identifier of the limit to associate with the queue.</p> |
| `farm_id` | String | ✅ | <p>The unique identifier of the farm that contains the queue and limit to associate.</p> |
| `queue_id` | String | ✅ | <p>The unique identifier of the queue to associate with the limit.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `updated_at` | String | <p>The Unix timestamp of the date and time that the association was last updated.</p> |
| `queue_id` | String | <p>The unique identifier of the queue associated with the limit.</p> |
| `created_at` | String | <p>The Unix timestamp of the date and time that the association was created.</p> |
| `limit_id` | String | <p>The unique identifier of the limit associated with the queue.</p> |
| `updated_by` | String | <p>The user identifier of the person that last updated the association.</p> |
| `status` | String | <p>The current status of the limit.</p> |
| `created_by` | String | <p>The user identifier of the person that created the association.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create queue_limit_association
queue_limit_association = provider.deadline.Queue_limit_association {
    limit_id = "value"  # <p>The unique identifier of the limit to associate with the queue.</p>
    farm_id = "value"  # <p>The unique identifier of the farm that contains the queue and limit to associate.</p>
    queue_id = "value"  # <p>The unique identifier of the queue to associate with the limit.</p>
}

# Access queue_limit_association outputs
queue_limit_association_id = queue_limit_association.id
queue_limit_association_updated_at = queue_limit_association.updated_at
queue_limit_association_queue_id = queue_limit_association.queue_id
queue_limit_association_created_at = queue_limit_association.created_at
queue_limit_association_limit_id = queue_limit_association.limit_id
queue_limit_association_updated_by = queue_limit_association.updated_by
queue_limit_association_status = queue_limit_association.status
queue_limit_association_created_by = queue_limit_association.created_by
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple queue_fleet_association resources
queue_fleet_association_0 = provider.deadline.Queue_fleet_association {
    queue_id = "value-0"
    fleet_id = "value-0"
    farm_id = "value-0"
}
queue_fleet_association_1 = provider.deadline.Queue_fleet_association {
    queue_id = "value-1"
    fleet_id = "value-1"
    farm_id = "value-1"
}
queue_fleet_association_2 = provider.deadline.Queue_fleet_association {
    queue_id = "value-2"
    fleet_id = "value-2"
    farm_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    queue_fleet_association = provider.deadline.Queue_fleet_association {
        queue_id = "production-value"
        fleet_id = "production-value"
        farm_id = "production-value"
    }
```

---

## Related Documentation

- [AWS Deadline Documentation](https://docs.aws.amazon.com/deadline/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
