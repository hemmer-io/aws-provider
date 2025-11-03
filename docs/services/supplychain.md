# Supplychain Service



**Resources**: 2

---

## Overview

The supplychain service provides access to 2 resource types:

- [Data_integration_flow_execution](#data_integration_flow_execution) [R]
- [Data_integration_event](#data_integration_event) [R]

---

## Resources


### Data_integration_flow_execution

DataIntegrationFlowExecution resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `flow_execution` | String | <p>The flow execution details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_integration_flow_execution outputs
data_integration_flow_execution_id = data_integration_flow_execution.id
data_integration_flow_execution_flow_execution = data_integration_flow_execution.flow_execution
```

---


### Data_integration_event

DataIntegrationEvent resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event` | String | <p>The details of the DataIntegrationEvent returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_integration_event outputs
data_integration_event_id = data_integration_event.id
data_integration_event_event = data_integration_event.event
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple data_integration_flow_execution resources
data_integration_flow_execution_0 = provider.supplychain.Data_integration_flow_execution {
}
data_integration_flow_execution_1 = provider.supplychain.Data_integration_flow_execution {
}
data_integration_flow_execution_2 = provider.supplychain.Data_integration_flow_execution {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    data_integration_flow_execution = provider.supplychain.Data_integration_flow_execution {
    }
```

---

## Related Documentation

- [AWS Supplychain Documentation](https://docs.aws.amazon.com/supplychain/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
