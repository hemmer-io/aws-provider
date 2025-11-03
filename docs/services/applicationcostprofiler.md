# Applicationcostprofiler Service



**Resources**: 1

---

## Overview

The applicationcostprofiler service provides access to 1 resource type:

- [Report_definition](#report_definition) [CRUD]

---

## Resources


### Report_definition

ReportDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `report_id` | String | ✅ | <p>Required. ID of the report. You can choose any valid string matching the pattern for the
      ID.</p> |
| `report_description` | String | ✅ | <p>Required. Description of the report.</p> |
| `report_frequency` | String | ✅ | <p>Required. The cadence to generate the report.</p> |
| `format` | String | ✅ | <p>Required. The format to use for the generated report.</p> |
| `destination_s3_location` | String | ✅ | <p>Required. Amazon Simple Storage Service (Amazon S3) location where Application Cost Profiler uploads the
      report.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_at` | String | <p>Timestamp (milliseconds) when this report definition was created.</p> |
| `report_frequency` | String | <p>Cadence used to generate the report.</p> |
| `last_updated` | String | <p>Timestamp (milliseconds) when this report definition was last updated.</p> |
| `report_id` | String | <p>ID of the report retrieved.</p> |
| `format` | String | <p>Format of the generated report.</p> |
| `destination_s3_location` | String | <p>Amazon Simple Storage Service (Amazon S3) location where the report is uploaded.</p> |
| `report_description` | String | <p>Description of the report.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create report_definition
report_definition = provider.applicationcostprofiler.Report_definition {
    report_id = "value"  # <p>Required. ID of the report. You can choose any valid string matching the pattern for the
      ID.</p>
    report_description = "value"  # <p>Required. Description of the report.</p>
    report_frequency = "value"  # <p>Required. The cadence to generate the report.</p>
    format = "value"  # <p>Required. The format to use for the generated report.</p>
    destination_s3_location = "value"  # <p>Required. Amazon Simple Storage Service (Amazon S3) location where Application Cost Profiler uploads the
      report.</p>
}

# Access report_definition outputs
report_definition_id = report_definition.id
report_definition_created_at = report_definition.created_at
report_definition_report_frequency = report_definition.report_frequency
report_definition_last_updated = report_definition.last_updated
report_definition_report_id = report_definition.report_id
report_definition_format = report_definition.format
report_definition_destination_s3_location = report_definition.destination_s3_location
report_definition_report_description = report_definition.report_description
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
report_definition_0 = provider.applicationcostprofiler.Report_definition {
    report_id = "value-0"
    report_description = "value-0"
    report_frequency = "value-0"
    format = "value-0"
    destination_s3_location = "value-0"
}
report_definition_1 = provider.applicationcostprofiler.Report_definition {
    report_id = "value-1"
    report_description = "value-1"
    report_frequency = "value-1"
    format = "value-1"
    destination_s3_location = "value-1"
}
report_definition_2 = provider.applicationcostprofiler.Report_definition {
    report_id = "value-2"
    report_description = "value-2"
    report_frequency = "value-2"
    format = "value-2"
    destination_s3_location = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    report_definition = provider.applicationcostprofiler.Report_definition {
        report_id = "production-value"
        report_description = "production-value"
        report_frequency = "production-value"
        format = "production-value"
        destination_s3_location = "production-value"
    }
```

---

## Related Documentation

- [AWS Applicationcostprofiler Documentation](https://docs.aws.amazon.com/applicationcostprofiler/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
