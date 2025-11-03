# Data_pipeline Service



**Resources**: 4

---

## Overview

The data_pipeline service provides access to 4 resource types:

- [Objects](#objects) [R]
- [Pipeline](#pipeline) [CD]
- [Pipelines](#pipelines) [R]
- [Pipeline_definition](#pipeline_definition) [CR]

---

## Resources


### Objects

Objects resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `has_more_results` | bool | <p>Indicates whether there are more results to return.</p> |
| `pipeline_objects` | Vec<String> | <p>An array of object definitions.</p> |
| `marker` | String | <p>The starting point for the next page of results. To view the next page of results, call <code>DescribeObjects</code> 
           again with this marker value. If the value is null, there are no more results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access objects outputs
objects_id = objects.id
objects_has_more_results = objects.has_more_results
objects_pipeline_objects = objects.pipeline_objects
objects_marker = objects.marker
```

---


### Pipeline

Pipeline resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `unique_id` | String | ✅ | <p>A unique identifier. This identifier is not the same as the pipeline identifier assigned by AWS Data Pipeline. 
            You are responsible for defining the format and ensuring the uniqueness of this identifier. You use this 
            parameter to ensure idempotency during repeated calls to <code>CreatePipeline</code>. For example, if the 
            first call to <code>CreatePipeline</code> does not succeed, you can pass in the same unique identifier and 
            pipeline name combination on a subsequent call to <code>CreatePipeline</code>. <code>CreatePipeline</code> 
            ensures that if a pipeline already exists with the same name and unique identifier, a new pipeline is not 
            created. Instead, you'll receive the pipeline identifier from the previous attempt. The uniqueness of the 
            name and unique identifier combination is scoped to the AWS account or IAM user credentials.</p> |
| `description` | String |  | <p>The description for the pipeline.</p> |
| `name` | String | ✅ | <p>The name for the pipeline. You can use the same name for multiple pipelines associated with your AWS account, 
            because AWS Data Pipeline assigns each pipeline a unique pipeline identifier.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to associate with the pipeline at creation. Tags let you control access to pipelines. 
            For more information, see <a href="http://docs.aws.amazon.com/datapipeline/latest/DeveloperGuide/dp-control-access.html">Controlling User Access to Pipelines</a> 
            in the <i>AWS Data Pipeline Developer Guide</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create pipeline
pipeline = provider.data_pipeline.Pipeline {
    unique_id = "value"  # <p>A unique identifier. This identifier is not the same as the pipeline identifier assigned by AWS Data Pipeline. 
            You are responsible for defining the format and ensuring the uniqueness of this identifier. You use this 
            parameter to ensure idempotency during repeated calls to <code>CreatePipeline</code>. For example, if the 
            first call to <code>CreatePipeline</code> does not succeed, you can pass in the same unique identifier and 
            pipeline name combination on a subsequent call to <code>CreatePipeline</code>. <code>CreatePipeline</code> 
            ensures that if a pipeline already exists with the same name and unique identifier, a new pipeline is not 
            created. Instead, you'll receive the pipeline identifier from the previous attempt. The uniqueness of the 
            name and unique identifier combination is scoped to the AWS account or IAM user credentials.</p>
    name = "value"  # <p>The name for the pipeline. You can use the same name for multiple pipelines associated with your AWS account, 
            because AWS Data Pipeline assigns each pipeline a unique pipeline identifier.</p>
}

```

---


### Pipelines

Pipelines resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pipeline_description_list` | Vec<String> | <p>An array of descriptions for the specified pipelines.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access pipelines outputs
pipelines_id = pipelines.id
pipelines_pipeline_description_list = pipelines.pipeline_description_list
```

---


### Pipeline_definition

PipelineDefinition resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameter_objects` | Vec<String> |  | <p>The parameter objects used with the pipeline.</p> |
| `pipeline_id` | String | ✅ | <p>The ID of the pipeline.</p> |
| `pipeline_objects` | Vec<String> | ✅ | <p>The objects that define the pipeline. These objects overwrite the existing pipeline definition.</p> |
| `parameter_values` | Vec<String> |  | <p>The parameter values used with the pipeline.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pipeline_objects` | Vec<String> | <p>The objects defined in the pipeline.</p> |
| `parameter_values` | Vec<String> | <p>The parameter values used in the pipeline definition.</p> |
| `parameter_objects` | Vec<String> | <p>The parameter objects used in the pipeline definition.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create pipeline_definition
pipeline_definition = provider.data_pipeline.Pipeline_definition {
    pipeline_id = "value"  # <p>The ID of the pipeline.</p>
    pipeline_objects = "value"  # <p>The objects that define the pipeline. These objects overwrite the existing pipeline definition.</p>
}

# Access pipeline_definition outputs
pipeline_definition_id = pipeline_definition.id
pipeline_definition_pipeline_objects = pipeline_definition.pipeline_objects
pipeline_definition_parameter_values = pipeline_definition.parameter_values
pipeline_definition_parameter_objects = pipeline_definition.parameter_objects
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple objects resources
objects_0 = provider.data_pipeline.Objects {
}
objects_1 = provider.data_pipeline.Objects {
}
objects_2 = provider.data_pipeline.Objects {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    objects = provider.data_pipeline.Objects {
    }
```

---

## Related Documentation

- [AWS Data_pipeline Documentation](https://docs.aws.amazon.com/data_pipeline/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
