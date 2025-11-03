# Health Service



**Resources**: 12

---

## Overview

The health service provides access to 12 resource types:

- [Event_types](#event_types) [R]
- [Entity_aggregates](#entity_aggregates) [R]
- [Health_service_status_for_organization](#health_service_status_for_organization) [R]
- [Events_for_organization](#events_for_organization) [R]
- [Event_details](#event_details) [R]
- [Entity_aggregates_for_organization](#entity_aggregates_for_organization) [R]
- [Affected_accounts_for_organization](#affected_accounts_for_organization) [R]
- [Affected_entities](#affected_entities) [R]
- [Event_details_for_organization](#event_details_for_organization) [R]
- [Affected_entities_for_organization](#affected_entities_for_organization) [R]
- [Event_aggregates](#event_aggregates) [R]
- [Events](#events) [R]

---

## Resources


### Event_types

EventTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If the results of a search are large, only a portion of the
results are returned, and a <code>nextToken</code> pagination token is returned in the response. To
retrieve the next batch of results, reissue the search request and include the returned token.
When all results have been returned, the response does not contain a pagination token value.</p> |
| `event_types` | Vec<String> | <p>A list of event types that match the filter criteria. Event types have a category
            (<code>issue</code>, <code>accountNotification</code>, or <code>scheduledChange</code>),
         a service (for example, <code>EC2</code>, <code>RDS</code>, <code>DATAPIPELINE</code>,
            <code>BILLING</code>), and a code (in the format
               <code>AWS_<i>SERVICE</i>_<i>DESCRIPTION</i>
            </code>; for
         example, <code>AWS_EC2_SYSTEM_MAINTENANCE_EVENT</code>).</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access event_types outputs
event_types_id = event_types.id
event_types_next_token = event_types.next_token
event_types_event_types = event_types.event_types
```

---


### Entity_aggregates

EntityAggregates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entity_aggregates` | Vec<String> | <p>The number of entities that are affected by each of the specified events.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access entity_aggregates outputs
entity_aggregates_id = entity_aggregates.id
entity_aggregates_entity_aggregates = entity_aggregates.entity_aggregates
```

---


### Health_service_status_for_organization

HealthServiceStatusForOrganization resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `health_service_access_status_for_organization` | String | <p>Information about the status of enabling or disabling the Health organizational
         view feature in your organization.</p>
         <p>Valid values are <code>ENABLED | DISABLED | PENDING</code>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access health_service_status_for_organization outputs
health_service_status_for_organization_id = health_service_status_for_organization.id
health_service_status_for_organization_health_service_access_status_for_organization = health_service_status_for_organization.health_service_access_status_for_organization
```

---


### Events_for_organization

EventsForOrganization resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If the results of a search are large, only a portion of the
results are returned, and a <code>nextToken</code> pagination token is returned in the response. To
retrieve the next batch of results, reissue the search request and include the returned token.
When all results have been returned, the response does not contain a pagination token value.</p> |
| `events` | Vec<String> | <p>The events that match the specified filter criteria.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access events_for_organization outputs
events_for_organization_id = events_for_organization.id
events_for_organization_next_token = events_for_organization.next_token
events_for_organization_events = events_for_organization.events
```

---


### Event_details

EventDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `successful_set` | Vec<String> | <p>Information about the events that could be retrieved.</p> |
| `failed_set` | Vec<String> | <p>Error messages for any events that could not be retrieved.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access event_details outputs
event_details_id = event_details.id
event_details_successful_set = event_details.successful_set
event_details_failed_set = event_details.failed_set
```

---


### Entity_aggregates_for_organization

EntityAggregatesForOrganization resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `organization_entity_aggregates` | Vec<String> | <p>The list of entity aggregates for each of the specified accounts that are affected by each of the specified events.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access entity_aggregates_for_organization outputs
entity_aggregates_for_organization_id = entity_aggregates_for_organization.id
entity_aggregates_for_organization_organization_entity_aggregates = entity_aggregates_for_organization.organization_entity_aggregates
```

---


### Affected_accounts_for_organization

AffectedAccountsForOrganization resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If the results of a search are large, only a portion of the
results are returned, and a <code>nextToken</code> pagination token is returned in the response. To
retrieve the next batch of results, reissue the search request and include the returned token.
When all results have been returned, the response does not contain a pagination token value.</p> |
| `event_scope_code` | String | <p>This parameter specifies if the Health event is a public Amazon Web Services service event or an account-specific event.</p>
         <ul>
            <li>
               <p>If the <code>eventScopeCode</code> value is <code>PUBLIC</code>, then the
                  <code>affectedAccounts</code> value is always empty.</p>
            </li>
            <li>
               <p>If the <code>eventScopeCode</code> value is <code>ACCOUNT_SPECIFIC</code>, then
               the <code>affectedAccounts</code> value lists the affected Amazon Web Services accounts in your
               organization. For example, if an event affects a service such as Amazon Elastic Compute Cloud and you
               have Amazon Web Services accounts that use that service, those account IDs appear in the
               response.</p>
            </li>
            <li>
               <p>If the <code>eventScopeCode</code> value is <code>NONE</code>, then the
                  <code>eventArn</code> that you specified in the request is invalid or doesn't
               exist.</p>
            </li>
         </ul> |
| `affected_accounts` | Vec<String> | <p>A JSON set of elements of the affected accounts.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access affected_accounts_for_organization outputs
affected_accounts_for_organization_id = affected_accounts_for_organization.id
affected_accounts_for_organization_next_token = affected_accounts_for_organization.next_token
affected_accounts_for_organization_event_scope_code = affected_accounts_for_organization.event_scope_code
affected_accounts_for_organization_affected_accounts = affected_accounts_for_organization.affected_accounts
```

---


### Affected_entities

AffectedEntities resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If the results of a search are large, only a portion of the
results are returned, and a <code>nextToken</code> pagination token is returned in the response. To
retrieve the next batch of results, reissue the search request and include the returned token.
When all results have been returned, the response does not contain a pagination token value.</p> |
| `entities` | Vec<String> | <p>The entities that match the filter criteria.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access affected_entities outputs
affected_entities_id = affected_entities.id
affected_entities_next_token = affected_entities.next_token
affected_entities_entities = affected_entities.entities
```

---


### Event_details_for_organization

EventDetailsForOrganization resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `successful_set` | Vec<String> | <p>Information about the events that could be retrieved.</p> |
| `failed_set` | Vec<String> | <p>Error messages for any events that could not be retrieved.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access event_details_for_organization outputs
event_details_for_organization_id = event_details_for_organization.id
event_details_for_organization_successful_set = event_details_for_organization.successful_set
event_details_for_organization_failed_set = event_details_for_organization.failed_set
```

---


### Affected_entities_for_organization

AffectedEntitiesForOrganization resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entities` | Vec<String> | <p>A JSON set of elements including the <code>awsAccountId</code> and its
            <code>entityArn</code>, <code>entityValue</code> and its <code>entityArn</code>,
            <code>lastUpdatedTime</code>, and <code>statusCode</code>.</p> |
| `next_token` | String | <p>If the results of a search are large, only a portion of the
results are returned, and a <code>nextToken</code> pagination token is returned in the response. To
retrieve the next batch of results, reissue the search request and include the returned token.
When all results have been returned, the response does not contain a pagination token value.</p> |
| `failed_set` | Vec<String> | <p>A JSON set of elements of the failed response, including the <code>awsAccountId</code>,
            <code>errorMessage</code>, <code>errorName</code>, and <code>eventArn</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access affected_entities_for_organization outputs
affected_entities_for_organization_id = affected_entities_for_organization.id
affected_entities_for_organization_entities = affected_entities_for_organization.entities
affected_entities_for_organization_next_token = affected_entities_for_organization.next_token
affected_entities_for_organization_failed_set = affected_entities_for_organization.failed_set
```

---


### Event_aggregates

EventAggregates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If the results of a search are large, only a portion of the
results are returned, and a <code>nextToken</code> pagination token is returned in the response. To
retrieve the next batch of results, reissue the search request and include the returned token.
When all results have been returned, the response does not contain a pagination token value.</p> |
| `event_aggregates` | Vec<String> | <p>The number of events in each category that meet the optional filter criteria.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access event_aggregates outputs
event_aggregates_id = event_aggregates.id
event_aggregates_next_token = event_aggregates.next_token
event_aggregates_event_aggregates = event_aggregates.event_aggregates
```

---


### Events

Events resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `events` | Vec<String> | <p>The events that match the specified filter criteria.</p> |
| `next_token` | String | <p>If the results of a search are large, only a portion of the
results are returned, and a <code>nextToken</code> pagination token is returned in the response. To
retrieve the next batch of results, reissue the search request and include the returned token.
When all results have been returned, the response does not contain a pagination token value.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access events outputs
events_id = events.id
events_events = events.events
events_next_token = events.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple event_types resources
event_types_0 = provider.health.Event_types {
}
event_types_1 = provider.health.Event_types {
}
event_types_2 = provider.health.Event_types {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    event_types = provider.health.Event_types {
    }
```

---

## Related Documentation

- [AWS Health Documentation](https://docs.aws.amazon.com/health/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
