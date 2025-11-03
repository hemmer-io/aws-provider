# Osis Service



**Resources**: 5

---

## Overview

The osis service provides access to 5 resource types:

- [Pipeline](#pipeline) [CRUD]
- [Pipeline_change_progress](#pipeline_change_progress) [R]
- [Pipeline_blueprint](#pipeline_blueprint) [R]
- [Pipeline_endpoint](#pipeline_endpoint) [CD]
- [Resource_policy](#resource_policy) [CRD]

---

## Resources


### Pipeline

Pipeline resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vpc_options` | String |  | <p>Container for the values required to configure VPC access for the pipeline. If you don't
   specify these values, OpenSearch Ingestion creates the pipeline with a public endpoint.</p> |
| `max_units` | i64 | ✅ | <p>The maximum pipeline capacity, in Ingestion Compute Units (ICUs).</p> |
| `pipeline_configuration_body` | String | ✅ | <p>The pipeline configuration in YAML format. The command accepts the pipeline configuration as
   a string or within a .yaml file. If you provide the configuration as a string, each new line must
   be escaped with <code>\n</code>.</p> |
| `min_units` | i64 | ✅ | <p>The minimum pipeline capacity, in Ingestion Compute Units (ICUs).</p> |
| `encryption_at_rest_options` | String |  | <p>Key-value pairs to configure encryption for data that is written to a persistent
   buffer.</p> |
| `tags` | Vec<String> |  | <p>List of tags to add to the pipeline upon creation.</p> |
| `buffer_options` | String |  | <p>Key-value pairs to configure persistent buffering for the pipeline.</p> |
| `pipeline_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM role that grants the pipeline permission to access
    Amazon Web Services resources.</p> |
| `pipeline_name` | String | ✅ | <p>The name of the OpenSearch Ingestion pipeline to create. Pipeline names are unique across the
   pipelines owned by an account within an Amazon Web Services Region.</p> |
| `log_publishing_options` | String |  | <p>Key-value pairs to configure log publishing.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pipeline` | String | <p>Detailed information about the requested pipeline.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create pipeline
pipeline = provider.osis.Pipeline {
    max_units = "value"  # <p>The maximum pipeline capacity, in Ingestion Compute Units (ICUs).</p>
    pipeline_configuration_body = "value"  # <p>The pipeline configuration in YAML format. The command accepts the pipeline configuration as
   a string or within a .yaml file. If you provide the configuration as a string, each new line must
   be escaped with <code>\n</code>.</p>
    min_units = "value"  # <p>The minimum pipeline capacity, in Ingestion Compute Units (ICUs).</p>
    pipeline_name = "value"  # <p>The name of the OpenSearch Ingestion pipeline to create. Pipeline names are unique across the
   pipelines owned by an account within an Amazon Web Services Region.</p>
}

# Access pipeline outputs
pipeline_id = pipeline.id
pipeline_pipeline = pipeline.pipeline
```

---


### Pipeline_change_progress

PipelineChangeProgress resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `change_progress_statuses` | Vec<String> | <p>The current status of the change happening on the pipeline.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access pipeline_change_progress outputs
pipeline_change_progress_id = pipeline_change_progress.id
pipeline_change_progress_change_progress_statuses = pipeline_change_progress.change_progress_statuses
```

---


### Pipeline_blueprint

PipelineBlueprint resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `format` | String | <p>The format of the blueprint.</p> |
| `blueprint` | String | <p>The requested blueprint in YAML format.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access pipeline_blueprint outputs
pipeline_blueprint_id = pipeline_blueprint.id
pipeline_blueprint_format = pipeline_blueprint.format
pipeline_blueprint_blueprint = pipeline_blueprint.blueprint
```

---


### Pipeline_endpoint

PipelineEndpoint resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pipeline_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the pipeline to create the endpoint for.</p> |
| `vpc_options` | String | ✅ | <p>Container for the VPC configuration for the pipeline endpoint, including subnet IDs and
   security group IDs.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create pipeline_endpoint
pipeline_endpoint = provider.osis.Pipeline_endpoint {
    pipeline_arn = "value"  # <p>The Amazon Resource Name (ARN) of the pipeline to create the endpoint for.</p>
    vpc_options = "value"  # <p>Container for the VPC configuration for the pipeline endpoint, including subnet IDs and
   security group IDs.</p>
}

```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the resource to attach the policy to.</p> |
| `policy` | String | ✅ | <p>The resource-based policy document in JSON format.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_arn` | String | <p>The Amazon Resource Name (ARN) of the resource.</p> |
| `policy` | String | <p>The resource-based policy document in JSON format.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.osis.Resource_policy {
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the resource to attach the policy to.</p>
    policy = "value"  # <p>The resource-based policy document in JSON format.</p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_resource_arn = resource_policy.resource_arn
resource_policy_policy = resource_policy.policy
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple pipeline resources
pipeline_0 = provider.osis.Pipeline {
    max_units = "value-0"
    pipeline_configuration_body = "value-0"
    min_units = "value-0"
    pipeline_name = "value-0"
}
pipeline_1 = provider.osis.Pipeline {
    max_units = "value-1"
    pipeline_configuration_body = "value-1"
    min_units = "value-1"
    pipeline_name = "value-1"
}
pipeline_2 = provider.osis.Pipeline {
    max_units = "value-2"
    pipeline_configuration_body = "value-2"
    min_units = "value-2"
    pipeline_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    pipeline = provider.osis.Pipeline {
        max_units = "production-value"
        pipeline_configuration_body = "production-value"
        min_units = "production-value"
        pipeline_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Osis Documentation](https://docs.aws.amazon.com/osis/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
