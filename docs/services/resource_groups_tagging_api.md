# Resource_groups_tagging_api Service



**Resources**: 5

---

## Overview

The resource_groups_tagging_api service provides access to 5 resource types:

- [Tag_keys](#tag_keys) [R]
- [Compliance_summary](#compliance_summary) [R]
- [Resources](#resources) [R]
- [Tag_values](#tag_values) [R]
- [Report_creation](#report_creation) [R]

---

## Resources


### Tag_keys

TagKeys resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pagination_token` | String | <p>A string that indicates that there is more data available than this
    response contains. To receive the next part of the response, specify this response value 
    as the <code>PaginationToken</code> value in the request for the next page.</p> |
| `tag_keys` | Vec<String> | <p>A list of all tag keys in the Amazon Web Services account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access tag_keys outputs
tag_keys_id = tag_keys.id
tag_keys_pagination_token = tag_keys.pagination_token
tag_keys_tag_keys = tag_keys.tag_keys
```

---


### Compliance_summary

ComplianceSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pagination_token` | String | <p>A string that indicates that there is more data available than this
    response contains. To receive the next part of the response, specify this response value 
    as the <code>PaginationToken</code> value in the request for the next page.</p> |
| `summary_list` | Vec<String> | <p>A table that shows counts of noncompliant resources.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access compliance_summary outputs
compliance_summary_id = compliance_summary.id
compliance_summary_pagination_token = compliance_summary.pagination_token
compliance_summary_summary_list = compliance_summary.summary_list
```

---


### Resources

Resources resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_tag_mapping_list` | Vec<String> | <p>A list of resource ARNs and the tags (keys and values) associated with 
    each.</p> |
| `pagination_token` | String | <p>A string that indicates that there is more data available than this
    response contains. To receive the next part of the response, specify this response value 
    as the <code>PaginationToken</code> value in the request for the next page.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resources outputs
resources_id = resources.id
resources_resource_tag_mapping_list = resources.resource_tag_mapping_list
resources_pagination_token = resources.pagination_token
```

---


### Tag_values

TagValues resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pagination_token` | String | <p>A string that indicates that there is more data available than this
    response contains. To receive the next part of the response, specify this response value 
    as the <code>PaginationToken</code> value in the request for the next page.</p> |
| `tag_values` | Vec<String> | <p>A list of all tag values for the specified key currently used in the specified Amazon Web Services
            Region for the calling account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access tag_values outputs
tag_values_id = tag_values.id
tag_values_pagination_token = tag_values.pagination_token
tag_values_tag_values = tag_values.tag_values
```

---


### Report_creation

ReportCreation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>Reports the status of the operation.</p>
         <p>The operation status can be one of the following:</p>
         <ul>
            <li>
               <p>
                  <code>RUNNING</code> - Report creation is in progress.</p>
            </li>
            <li>
               <p>
                  <code>SUCCEEDED</code> - Report creation is complete. You can open the report
                    from the Amazon S3 bucket that you specified when you ran
                        <code>StartReportCreation</code>.</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code> - Report creation timed out or the Amazon S3 bucket is not
                    accessible. </p>
            </li>
            <li>
               <p>
                  <code>NO REPORT</code> - No report was generated in the last 90 days.</p>
            </li>
         </ul> |
| `error_message` | String | <p>Details of the common errors that all operations return.</p> |
| `s3_location` | String | <p>The path to the Amazon S3 bucket where the report was stored on creation.</p> |
| `start_date` | String | <p>The date and time that the report was started. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access report_creation outputs
report_creation_id = report_creation.id
report_creation_status = report_creation.status
report_creation_error_message = report_creation.error_message
report_creation_s3_location = report_creation.s3_location
report_creation_start_date = report_creation.start_date
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple tag_keys resources
tag_keys_0 = provider.resource_groups_tagging_api.Tag_keys {
}
tag_keys_1 = provider.resource_groups_tagging_api.Tag_keys {
}
tag_keys_2 = provider.resource_groups_tagging_api.Tag_keys {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    tag_keys = provider.resource_groups_tagging_api.Tag_keys {
    }
```

---

## Related Documentation

- [AWS Resource_groups_tagging_api Documentation](https://docs.aws.amazon.com/resource_groups_tagging_api/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
