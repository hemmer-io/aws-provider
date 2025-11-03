# Cost_and_usage_report_service Service



**Resources**: 2

---

## Overview

The cost_and_usage_report_service service provides access to 2 resource types:

- [Report_definition](#report_definition) [CD]
- [Report_definitions](#report_definitions) [R]

---

## Resources


### Report_definition

ReportDefinition resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `report_definition` | String | ✅ | <p>Represents the output of the PutReportDefinition operation. The content consists of the detailed 
      metadata and data file information. </p> |
| `tags` | Vec<String> |  | <p>The tags to be assigned to the report definition resource.</p> |



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


### Report_definitions

ReportDefinitions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String |  |
| `report_definitions` | Vec<String> | <p>An Amazon Web Services Cost and Usage Report list owned by the account.</p> |


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
report_definitions_next_token = report_definitions.next_token
report_definitions_report_definitions = report_definitions.report_definitions
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple report_definition resources
report_definition_0 = provider.cost_and_usage_report_service.Report_definition {
    report_definition = "value-0"
}
report_definition_1 = provider.cost_and_usage_report_service.Report_definition {
    report_definition = "value-1"
}
report_definition_2 = provider.cost_and_usage_report_service.Report_definition {
    report_definition = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    report_definition = provider.cost_and_usage_report_service.Report_definition {
        report_definition = "production-value"
    }
```

---

## Related Documentation

- [AWS Cost_and_usage_report_service Documentation](https://docs.aws.amazon.com/cost_and_usage_report_service/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
