# Sagemaker_a2i_runtime Service



**Resources**: 1

---

## Overview

The sagemaker_a2i_runtime service provides access to 1 resource type:

- [Human_loop](#human_loop) [RD]

---

## Resources


### Human_loop

HumanLoop resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `human_loop_name` | String | <p>The name of the human loop. The name must be lowercase, unique within the Region in your
      account, and can have up to 63 characters. Valid characters: a-z, 0-9, and - (hyphen).</p> |
| `failure_code` | String | <p>A failure code that identifies the type of failure.</p>
         <p>Possible values: <code>ValidationError</code>, <code>Expired</code>,
        <code>InternalError</code>
         </p> |
| `human_loop_output` | String | <p>An object that contains information about the output of the human loop.</p> |
| `failure_reason` | String | <p>The reason why a human loop failed. The failure reason is returned when the status of the
      human loop is <code>Failed</code>.</p> |
| `creation_time` | String | <p>The creation time when Amazon Augmented AI created the human loop.</p> |
| `human_loop_status` | String | <p>The status of the human loop. </p> |
| `flow_definition_arn` | String | <p>The Amazon Resource Name (ARN) of the flow definition.</p> |
| `human_loop_arn` | String | <p>The Amazon Resource Name (ARN) of the human loop.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access human_loop outputs
human_loop_id = human_loop.id
human_loop_human_loop_name = human_loop.human_loop_name
human_loop_failure_code = human_loop.failure_code
human_loop_human_loop_output = human_loop.human_loop_output
human_loop_failure_reason = human_loop.failure_reason
human_loop_creation_time = human_loop.creation_time
human_loop_human_loop_status = human_loop.human_loop_status
human_loop_flow_definition_arn = human_loop.flow_definition_arn
human_loop_human_loop_arn = human_loop.human_loop_arn
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple human_loop resources
human_loop_0 = provider.sagemaker_a2i_runtime.Human_loop {
}
human_loop_1 = provider.sagemaker_a2i_runtime.Human_loop {
}
human_loop_2 = provider.sagemaker_a2i_runtime.Human_loop {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    human_loop = provider.sagemaker_a2i_runtime.Human_loop {
    }
```

---

## Related Documentation

- [AWS Sagemaker_a2i_runtime Documentation](https://docs.aws.amazon.com/sagemaker_a2i_runtime/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
