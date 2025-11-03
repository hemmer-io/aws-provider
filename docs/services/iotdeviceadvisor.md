# Iotdeviceadvisor Service



**Resources**: 4

---

## Overview

The iotdeviceadvisor service provides access to 4 resource types:

- [Endpoint](#endpoint) [R]
- [Suite_run](#suite_run) [R]
- [Suite_run_report](#suite_run_report) [R]
- [Suite_definition](#suite_definition) [CRUD]

---

## Resources


### Endpoint

Endpoint resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoint` | String | <p>The response of an Device Advisor endpoint.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access endpoint outputs
endpoint_id = endpoint.id
endpoint_endpoint = endpoint.endpoint
```

---


### Suite_run

SuiteRun resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `suite_definition_id` | String | <p>Suite definition ID for the test suite run.</p> |
| `suite_definition_version` | String | <p>Suite definition version for the test suite run.</p> |
| `test_result` | String | <p>Test results for the test suite run.</p> |
| `end_time` | String | <p>Date (in Unix epoch time) when the test suite run ended.</p> |
| `suite_run_id` | String | <p>Suite run ID for the test suite run.</p> |
| `suite_run_arn` | String | <p>The ARN of the suite run.</p> |
| `error_reason` | String | <p>Error reason for any test suite run failure.</p> |
| `tags` | HashMap<String, String> | <p>The tags attached to the suite run.</p> |
| `start_time` | String | <p>Date (in Unix epoch time) when the test suite run started.</p> |
| `status` | String | <p>Status for the test suite run.</p> |
| `suite_run_configuration` | String | <p>Suite run configuration for the test suite run.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access suite_run outputs
suite_run_id = suite_run.id
suite_run_suite_definition_id = suite_run.suite_definition_id
suite_run_suite_definition_version = suite_run.suite_definition_version
suite_run_test_result = suite_run.test_result
suite_run_end_time = suite_run.end_time
suite_run_suite_run_id = suite_run.suite_run_id
suite_run_suite_run_arn = suite_run.suite_run_arn
suite_run_error_reason = suite_run.error_reason
suite_run_tags = suite_run.tags
suite_run_start_time = suite_run.start_time
suite_run_status = suite_run.status
suite_run_suite_run_configuration = suite_run.suite_run_configuration
```

---


### Suite_run_report

SuiteRunReport resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `qualification_report_download_url` | String | <p>Download URL of the qualification report.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access suite_run_report outputs
suite_run_report_id = suite_run_report.id
suite_run_report_qualification_report_download_url = suite_run_report.qualification_report_download_url
```

---


### Suite_definition

SuiteDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `suite_definition_configuration` | String | ✅ | <p>Creates a Device Advisor test suite with suite definition configuration.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to be attached to the suite definition.</p> |
| `client_token` | String |  | <p>The client token for the test suite definition creation. 
            This token is used for tracking test suite definition creation 
            using retries and obtaining its status. This parameter is optional.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `latest_version` | String | <p>Latest suite definition version of the suite definition.</p> |
| `suite_definition_version` | String | <p>Suite definition version of the suite definition.</p> |
| `tags` | HashMap<String, String> | <p>Tags attached to the suite definition.</p> |
| `suite_definition_id` | String | <p>Suite definition ID of the suite definition.</p> |
| `suite_definition_arn` | String | <p>The ARN of the suite definition.</p> |
| `suite_definition_configuration` | String | <p>Suite configuration of the suite definition.</p> |
| `last_modified_at` | String | <p>Date (in Unix epoch time) when the suite definition was last modified.</p> |
| `created_at` | String | <p>Date (in Unix epoch time) when the suite definition was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create suite_definition
suite_definition = provider.iotdeviceadvisor.Suite_definition {
    suite_definition_configuration = "value"  # <p>Creates a Device Advisor test suite with suite definition configuration.</p>
}

# Access suite_definition outputs
suite_definition_id = suite_definition.id
suite_definition_latest_version = suite_definition.latest_version
suite_definition_suite_definition_version = suite_definition.suite_definition_version
suite_definition_tags = suite_definition.tags
suite_definition_suite_definition_id = suite_definition.suite_definition_id
suite_definition_suite_definition_arn = suite_definition.suite_definition_arn
suite_definition_suite_definition_configuration = suite_definition.suite_definition_configuration
suite_definition_last_modified_at = suite_definition.last_modified_at
suite_definition_created_at = suite_definition.created_at
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple endpoint resources
endpoint_0 = provider.iotdeviceadvisor.Endpoint {
}
endpoint_1 = provider.iotdeviceadvisor.Endpoint {
}
endpoint_2 = provider.iotdeviceadvisor.Endpoint {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    endpoint = provider.iotdeviceadvisor.Endpoint {
    }
```

---

## Related Documentation

- [AWS Iotdeviceadvisor Documentation](https://docs.aws.amazon.com/iotdeviceadvisor/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
