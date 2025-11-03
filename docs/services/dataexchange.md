# Dataexchange Service



**Resources**: 7

---

## Overview

The dataexchange service provides access to 7 resource types:

- [Revision](#revision) [CRUD]
- [Asset](#asset) [RUD]
- [Received_data_grant](#received_data_grant) [R]
- [Job](#job) [CR]
- [Event_action](#event_action) [CRUD]
- [Data_set](#data_set) [CRUD]
- [Data_grant](#data_grant) [CRD]

---

## Resources


### Revision

Revision resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `comment` | String |  | <p>An optional comment about the revision.</p> |
| `tags` | HashMap<String, String> |  | <p>A revision tag is an optional label that you can assign to a revision when you create
         it. Each tag consists of a key and an optional value, both of which you define. When you
         use tagging, you can also use tag-based access control in IAM policies to control access to
         these data sets and revisions.</p> |
| `data_set_id` | String | ✅ | <p>The unique identifier for a data set.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `updated_at` | String | <p>The date and time that the revision was last updated, in ISO 8601 format.</p> |
| `revoked` | bool | <p>A status indicating that subscribers' access to the revision was revoked.</p> |
| `comment` | String | <p>An optional comment about the revision.</p> |
| `tags` | HashMap<String, String> | <p>The tags for the revision.</p> |
| `id` | String | <p>The unique identifier for the revision.</p> |
| `arn` | String | <p>The ARN for the revision.</p> |
| `source_id` | String | <p>The revision ID of the owned revision corresponding to the entitled revision being
         viewed. This parameter is returned when a revision owner is viewing the entitled copy of
         its owned revision.</p> |
| `revoked_at` | String | <p>The date and time that the revision was revoked, in ISO 8601 format.</p> |
| `revocation_comment` | String | <p>A required comment to inform subscribers of the reason their access to the revision was
         revoked.</p> |
| `created_at` | String | <p>The date and time that the revision was created, in ISO 8601 format.</p> |
| `data_set_id` | String | <p>The unique identifier for the data set associated with the data set revision.</p> |
| `finalized` | bool | <p>To publish a revision to a data set in a product, the revision must first be finalized.
         Finalizing a revision tells AWS Data Exchange that your changes to the assets in the
         revision are complete. After it's in this read-only state, you can publish the revision to
         your products. Finalized revisions can be published through the AWS Data Exchange console
         or the AWS Marketplace Catalog API, using the StartChangeSet AWS Marketplace Catalog API
         action. When using the API, revisions are uniquely identified by their ARN.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create revision
revision = provider.dataexchange.Revision {
    data_set_id = "value"  # <p>The unique identifier for a data set.</p>
}

# Access revision outputs
revision_id = revision.id
revision_updated_at = revision.updated_at
revision_revoked = revision.revoked
revision_comment = revision.comment
revision_tags = revision.tags
revision_id = revision.id
revision_arn = revision.arn
revision_source_id = revision.source_id
revision_revoked_at = revision.revoked_at
revision_revocation_comment = revision.revocation_comment
revision_created_at = revision.created_at
revision_data_set_id = revision.data_set_id
revision_finalized = revision.finalized
```

---


### Asset

Asset resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_set_id` | String | ✅ | <p>The unique identifier for a data set.</p> |
| `revision_id` | String | ✅ | <p>The unique identifier for a revision.</p> |
| `name` | String | ✅ | <p>The name of the asset. When importing from Amazon S3, the Amazon S3 object key is used
         as the asset name. When exporting to Amazon S3, the asset name is used as default target
         Amazon S3 object key. When importing from Amazon API Gateway API, the API name is used as
         the asset name. When importing from Amazon Redshift, the datashare name is used as the
         asset name. When importing from AWS Lake Formation, the static values of "Database(s)
         included in the LF-tag policy" or "Table(s) included in LF-tag policy" are used as the
         name.</p> |
| `asset_id` | String | ✅ | <p>The unique identifier for an asset.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | <p>The ARN for the asset.</p> |
| `name` | String | <p>The name of the asset. When importing from Amazon S3, the Amazon S3 object key is used
         as the asset name. When exporting to Amazon S3, the asset name is used as default target
         Amazon S3 object key. When importing from Amazon API Gateway API, the API name is used as
         the asset name. When importing from Amazon Redshift, the datashare name is used as the
         asset name. When importing from AWS Lake Formation, the static values of "Database(s)
         included in the LF-tag policy" or "Table(s) included in the LF-tag policy" are used as the
         asset name.</p> |
| `source_id` | String | <p>The asset ID of the owned asset corresponding to the entitled asset being viewed. This
         parameter is returned when an asset owner is viewing the entitled copy of its owned
         asset.</p> |
| `updated_at` | String | <p>The date and time that the asset was last updated, in ISO 8601 format.</p> |
| `data_set_id` | String | <p>The unique identifier for the data set associated with this asset.</p> |
| `id` | String | <p>The unique identifier for the asset.</p> |
| `asset_type` | String | <p>The type of asset that is added to a data set.</p> |
| `revision_id` | String | <p>The unique identifier for the revision associated with this asset.</p> |
| `asset_details` | String | <p>Details about the asset.</p> |
| `created_at` | String | <p>The date and time that the asset was created, in ISO 8601 format.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access asset outputs
asset_id = asset.id
asset_arn = asset.arn
asset_name = asset.name
asset_source_id = asset.source_id
asset_updated_at = asset.updated_at
asset_data_set_id = asset.data_set_id
asset_id = asset.id
asset_asset_type = asset.asset_type
asset_revision_id = asset.revision_id
asset_asset_details = asset.asset_details
asset_created_at = asset.created_at
```

---


### Received_data_grant

ReceivedDataGrant resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>The description of the data grant.</p> |
| `updated_at` | String | <p>The timestamp of when the data grant was last updated.</p> |
| `name` | String | <p>The name of the data grant.</p> |
| `grant_distribution_scope` | String | <p>The distribution scope for the data grant.</p> |
| `data_set_id` | String | <p>The ID of the data set associated to the data grant.</p> |
| `ends_at` | String | <p>The timestamp of when access to the associated data set ends.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the data grant.</p> |
| `sender_principal` | String | <p>The Amazon Web Services account ID of the data grant sender.</p> |
| `created_at` | String | <p>The timestamp of when the data grant was created.</p> |
| `acceptance_state` | String | <p>The acceptance state of the data grant.</p> |
| `accepted_at` | String | <p>The timestamp of when the data grant was accepted.</p> |
| `id` | String | <p>The ID of the data grant.</p> |
| `receiver_principal` | String | <p>The Amazon Web Services account ID of the data grant receiver.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access received_data_grant outputs
received_data_grant_id = received_data_grant.id
received_data_grant_description = received_data_grant.description
received_data_grant_updated_at = received_data_grant.updated_at
received_data_grant_name = received_data_grant.name
received_data_grant_grant_distribution_scope = received_data_grant.grant_distribution_scope
received_data_grant_data_set_id = received_data_grant.data_set_id
received_data_grant_ends_at = received_data_grant.ends_at
received_data_grant_arn = received_data_grant.arn
received_data_grant_sender_principal = received_data_grant.sender_principal
received_data_grant_created_at = received_data_grant.created_at
received_data_grant_acceptance_state = received_data_grant.acceptance_state
received_data_grant_accepted_at = received_data_grant.accepted_at
received_data_grant_id = received_data_grant.id
received_data_grant_receiver_principal = received_data_grant.receiver_principal
```

---


### Job

Job resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String | ✅ | <p>The type of job to be created.</p> |
| `details` | String | ✅ | <p>The details for the CreateJob request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | <p>The ARN for the job.</p> |
| `state` | String | <p>The state of the job.</p> |
| `details` | String | <p>Details about the job.</p> |
| `errors` | Vec<String> | <p>The errors associated with jobs.</p> |
| `type` | String | <p>The job type.</p> |
| `id` | String | <p>The unique identifier for the job.</p> |
| `created_at` | String | <p>The date and time that the job was created, in ISO 8601 format.</p> |
| `updated_at` | String | <p>The date and time that the job was last updated, in ISO 8601 format.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create job
job = provider.dataexchange.Job {
    type = "value"  # <p>The type of job to be created.</p>
    details = "value"  # <p>The details for the CreateJob request.</p>
}

# Access job outputs
job_id = job.id
job_arn = job.arn
job_state = job.state
job_details = job.details
job_errors = job.errors
job_type = job.type
job_id = job.id
job_created_at = job.created_at
job_updated_at = job.updated_at
```

---


### Event_action

EventAction resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event` | String | ✅ | <p>What occurs to start an action.</p> |
| `tags` | HashMap<String, String> |  | <p>Key-value pairs that you can associate with the event action.</p> |
| `action` | String | ✅ | <p>What occurs after a certain event.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | HashMap<String, String> | <p>The tags for the event action.</p> |
| `event` | String | <p>What occurs to start an action.</p> |
| `created_at` | String | <p>The date and time that the event action was created, in ISO 8601 format.</p> |
| `action` | String | <p>What occurs after a certain event.</p> |
| `updated_at` | String | <p>The date and time that the event action was last updated, in ISO 8601 format.</p> |
| `arn` | String | <p>The ARN for the event action.</p> |
| `id` | String | <p>The unique identifier for the event action.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_action
event_action = provider.dataexchange.Event_action {
    event = "value"  # <p>What occurs to start an action.</p>
    action = "value"  # <p>What occurs after a certain event.</p>
}

# Access event_action outputs
event_action_id = event_action.id
event_action_tags = event_action.tags
event_action_event = event_action.event
event_action_created_at = event_action.created_at
event_action_action = event_action.action
event_action_updated_at = event_action.updated_at
event_action_arn = event_action.arn
event_action_id = event_action.id
```

---


### Data_set

DataSet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `asset_type` | String | ✅ | <p>The type of asset that is added to a data set.</p> |
| `description` | String | ✅ | <p>A description for the data set. This value can be up to 16,348 characters long.</p> |
| `tags` | HashMap<String, String> |  | <p>A data set tag is an optional label that you can assign to a data set when you create
         it. Each tag consists of a key and an optional value, both of which you define. When you
         use tagging, you can also use tag-based access control in IAM policies to control access to
         these data sets and revisions.</p> |
| `name` | String | ✅ | <p>The name of the data set.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `asset_type` | String | <p>The type of asset that is added to a data set.</p> |
| `name` | String | <p>The name of the data set.</p> |
| `origin` | String | <p>A property that defines the data set as OWNED by the account (for providers) or ENTITLED
         to the account (for subscribers).</p> |
| `tags` | HashMap<String, String> | <p>The tags for the data set.</p> |
| `source_id` | String | <p>The data set ID of the owned data set corresponding to the entitled data set being
         viewed. This parameter is returned when a data set owner is viewing the entitled copy of
         its owned data set.</p> |
| `updated_at` | String | <p>The date and time that the data set was last updated, in ISO 8601 format.</p> |
| `arn` | String | <p>The ARN for the data set.</p> |
| `created_at` | String | <p>The date and time that the data set was created, in ISO 8601 format.</p> |
| `description` | String | <p>The description for the data set.</p> |
| `origin_details` | String | <p>If the origin of this data set is ENTITLED, includes the details for the product on AWS
         Marketplace.</p> |
| `id` | String | <p>The unique identifier for the data set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_set
data_set = provider.dataexchange.Data_set {
    asset_type = "value"  # <p>The type of asset that is added to a data set.</p>
    description = "value"  # <p>A description for the data set. This value can be up to 16,348 characters long.</p>
    name = "value"  # <p>The name of the data set.</p>
}

# Access data_set outputs
data_set_id = data_set.id
data_set_asset_type = data_set.asset_type
data_set_name = data_set.name
data_set_origin = data_set.origin
data_set_tags = data_set.tags
data_set_source_id = data_set.source_id
data_set_updated_at = data_set.updated_at
data_set_arn = data_set.arn
data_set_created_at = data_set.created_at
data_set_description = data_set.description
data_set_origin_details = data_set.origin_details
data_set_id = data_set.id
```

---


### Data_grant

DataGrant resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ends_at` | String |  | <p>The timestamp of when access to the associated data set ends.</p> |
| `grant_distribution_scope` | String | ✅ | <p>The distribution scope of the data grant.</p> |
| `description` | String |  | <p>The description of the data grant.</p> |
| `receiver_principal` | String | ✅ | <p>The Amazon Web Services account ID of the data grant receiver.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to add to the data grant. A tag is a key-value pair.</p> |
| `name` | String | ✅ | <p>The name of the data grant.</p> |
| `source_data_set_id` | String | ✅ | <p>The ID of the data set used to create the data grant.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `acceptance_state` | String | <p>The acceptance state of the data grant.</p> |
| `description` | String | <p>The description of the data grant.</p> |
| `name` | String | <p>The name of the data grant.</p> |
| `grant_distribution_scope` | String | <p>The distribution scope for the data grant.</p> |
| `ends_at` | String | <p>The timestamp of when access to the associated data set ends.</p> |
| `sender_principal` | String | <p>The Amazon Web Services account ID of the data grant sender.</p> |
| `source_data_set_id` | String | <p>The ID of the data set used to create the data grant.</p> |
| `created_at` | String | <p>The timestamp of when the data grant was created.</p> |
| `tags` | HashMap<String, String> | <p>The tags associated to the data grant. A tag is a key-value pair.</p> |
| `receiver_principal` | String | <p>The Amazon Web Services account ID of the data grant receiver.</p> |
| `data_set_id` | String | <p>The ID of the data set associated to the data grant.</p> |
| `accepted_at` | String | <p>The timestamp of when the data grant was accepted.</p> |
| `updated_at` | String | <p>The timestamp of when the data grant was last updated.</p> |
| `id` | String | <p>The ID of the data grant.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the data grant.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_grant
data_grant = provider.dataexchange.Data_grant {
    grant_distribution_scope = "value"  # <p>The distribution scope of the data grant.</p>
    receiver_principal = "value"  # <p>The Amazon Web Services account ID of the data grant receiver.</p>
    name = "value"  # <p>The name of the data grant.</p>
    source_data_set_id = "value"  # <p>The ID of the data set used to create the data grant.</p>
}

# Access data_grant outputs
data_grant_id = data_grant.id
data_grant_acceptance_state = data_grant.acceptance_state
data_grant_description = data_grant.description
data_grant_name = data_grant.name
data_grant_grant_distribution_scope = data_grant.grant_distribution_scope
data_grant_ends_at = data_grant.ends_at
data_grant_sender_principal = data_grant.sender_principal
data_grant_source_data_set_id = data_grant.source_data_set_id
data_grant_created_at = data_grant.created_at
data_grant_tags = data_grant.tags
data_grant_receiver_principal = data_grant.receiver_principal
data_grant_data_set_id = data_grant.data_set_id
data_grant_accepted_at = data_grant.accepted_at
data_grant_updated_at = data_grant.updated_at
data_grant_id = data_grant.id
data_grant_arn = data_grant.arn
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple revision resources
revision_0 = provider.dataexchange.Revision {
    data_set_id = "value-0"
}
revision_1 = provider.dataexchange.Revision {
    data_set_id = "value-1"
}
revision_2 = provider.dataexchange.Revision {
    data_set_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    revision = provider.dataexchange.Revision {
        data_set_id = "production-value"
    }
```

---

## Related Documentation

- [AWS Dataexchange Documentation](https://docs.aws.amazon.com/dataexchange/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
