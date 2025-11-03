# Cost_and_usage_report_service Service



**Resources**: 2

---

## Overview

The cost_and_usage_report_service service provides access to 2 resource types:

- [Report_definitions](#report_definitions) [R]
- [Report_definition](#report_definition) [CD]

---

## Resources


### Report_definitions

ReportDefinitions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `report_definitions` | Vec<String> | <p>An Amazon Web Services Cost and Usage Report list owned by the account.</p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access report_definitions outputs
report_definitions_id = report_definitions.id
report_definitions_report_definitions = report_definitions.report_definitions
report_definitions_next_token = report_definitions.next_token
```

---


### Report_definition

ReportDefinition resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The tags to be assigned to the report definition resource.</p> |
| `report_definition` | String | ✅ | <p>Represents the output of the PutReportDefinition operation. The content consists of the detailed 
      metadata and data file information. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create report_definition
report_definition = provider.cost_and_usage_report_service.Report_definition {
    report_definition = "value"  # <p>Represents the output of the PutReportDefinition operation. The content consists of the detailed 
      metadata and data file information. </p>
}

```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple report_definitions resources
report_definitions_0 = provider.cost_and_usage_report_service.Report_definitions {
}
report_definitions_1 = provider.cost_and_usage_report_service.Report_definitions {
}
report_definitions_2 = provider.cost_and_usage_report_service.Report_definitions {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    report_definitions = provider.cost_and_usage_report_service.Report_definitions {
    }
```

---

## Related Documentation

- [AWS Cost_and_usage_report_service Documentation](https://docs.aws.amazon.com/cost_and_usage_report_service/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
