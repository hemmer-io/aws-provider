# Marketplace_catalog Service



**Resources**: 3

---

## Overview

The marketplace_catalog service provides access to 3 resource types:

- [Entity](#entity) [R]
- [Resource_policy](#resource_policy) [CRD]
- [Change_set](#change_set) [R]

---

## Resources


### Entity

Entity resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_modified_date` | String | <p>The last modified date of the entity, in ISO 8601 format
            (2018-02-27T13:45:22Z).</p> |
| `details` | String | <p>This stringified JSON object includes the details of the entity.</p> |
| `details_document` | String | <p>The JSON value of the details specific to the entity.</p>
         <p>To download "DetailsDocument" shapes, see the <a href="https://github.com/awslabs/aws-marketplace-catalog-api-shapes-for-python">Python</a> 
            and <a href="https://github.com/awslabs/aws-marketplace-catalog-api-shapes-for-java/tree/main">Java</a> shapes on GitHub.</p> |
| `entity_arn` | String | <p>The ARN associated to the unique identifier for the entity referenced in this
            request.</p> |
| `entity_identifier` | String | <p>The identifier of the entity, in the format of
            <code>EntityId@RevisionId</code>.</p> |
| `entity_type` | String | <p>The named type of the entity, in the format of <code>EntityType@Version</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access entity outputs
entity_id = entity.id
entity_last_modified_date = entity.last_modified_date
entity_details = entity.details
entity_details_document = entity.details_document
entity_entity_arn = entity.entity_arn
entity_entity_identifier = entity.entity_identifier
entity_entity_type = entity.entity_type
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the entity resource you want to associate with a
            resource policy.</p> |
| `policy` | String | ✅ | <p>The policy document to set; formatted in JSON.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The policy document to set; formatted in JSON.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.marketplace_catalog.Resource_policy {
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the entity resource you want to associate with a
            resource policy.</p>
    policy = "value"  # <p>The policy document to set; formatted in JSON.</p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_policy = resource_policy.policy
```

---


### Change_set

ChangeSet resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `change_set_name` | String | <p>The optional name provided in the <code>StartChangeSet</code> request. If you do not
            provide a name, one is set by default.</p> |
| `intent` | String | <p>The optional intent provided in the <code>StartChangeSet</code> request. If you do not
            provide an intent, <code>APPLY</code> is set by default.</p> |
| `change_set_arn` | String | <p>The ARN associated with the unique identifier for the change set referenced in this
            request.</p> |
| `failure_code` | String | <p>Returned if the change set is in <code>FAILED</code> status. Can be either
                <code>CLIENT_ERROR</code>, which means that there are issues with the request (see
            the <code>ErrorDetailList</code>), or <code>SERVER_FAULT</code>, which means that there
            is a problem in the system, and you should retry your request.</p> |
| `failure_description` | String | <p>Returned if there is a failure on the change set, but that failure is not related to
            any of the changes in the request.</p> |
| `change_set` | Vec<String> | <p>An array of <code>ChangeSummary</code> objects.</p> |
| `change_set_id` | String | <p>Required. The unique identifier for the change set referenced in this request.</p> |
| `end_time` | String | <p>The date and time, in ISO 8601 format (2018-02-27T13:45:22Z), the request transitioned
            to a terminal state. The change cannot transition to a different state. Null if the
            request is not in a terminal state. </p> |
| `status` | String | <p>The status of the change request.</p> |
| `start_time` | String | <p>The date and time, in ISO 8601 format (2018-02-27T13:45:22Z), the request started.
        </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access change_set outputs
change_set_id = change_set.id
change_set_change_set_name = change_set.change_set_name
change_set_intent = change_set.intent
change_set_change_set_arn = change_set.change_set_arn
change_set_failure_code = change_set.failure_code
change_set_failure_description = change_set.failure_description
change_set_change_set = change_set.change_set
change_set_change_set_id = change_set.change_set_id
change_set_end_time = change_set.end_time
change_set_status = change_set.status
change_set_start_time = change_set.start_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple entity resources
entity_0 = provider.marketplace_catalog.Entity {
}
entity_1 = provider.marketplace_catalog.Entity {
}
entity_2 = provider.marketplace_catalog.Entity {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    entity = provider.marketplace_catalog.Entity {
    }
```

---

## Related Documentation

- [AWS Marketplace_catalog Documentation](https://docs.aws.amazon.com/marketplace_catalog/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
