# Codeguru_security Service



**Resources**: 5

---

## Overview

The codeguru_security service provides access to 5 resource types:

- [Upload_url](#upload_url) [C]
- [Account_configuration](#account_configuration) [RU]
- [Scan](#scan) [CR]
- [Findings](#findings) [R]
- [Metrics_summary](#metrics_summary) [R]

---

## Resources


### Upload_url

UploadUrl resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scan_name` | String | ✅ | <p>The name of the scan that will use the uploaded resource. CodeGuru Security uses the unique scan name to track revisions across multiple scans of the same resource. Use this <code>scanName</code> when you call <code>CreateScan</code> on the code resource you upload to this URL.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create upload_url
upload_url = provider.codeguru_security.Upload_url {
    scan_name = "value"  # <p>The name of the scan that will use the uploaded resource. CodeGuru Security uses the unique scan name to track revisions across multiple scans of the same resource. Use this <code>scanName</code> when you call <code>CreateScan</code> on the code resource you upload to this URL.</p>
}

```

---


### Account_configuration

AccountConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encryption_config` | String | ✅ | <p>The customer-managed KMS key ARN you want to use for encryption. If not specified, CodeGuru Security will use an AWS-managed key for encryption. If you previously specified a customer-managed KMS key and want CodeGuru Security to use an AWS-managed key for encryption instead, pass nothing.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `encryption_config` | String | <p>An <code>EncryptionConfig</code> object that contains the KMS key ARN that is used for encryption. By default, CodeGuru Security uses an AWS-managed key for encryption. To specify your own key, call <code>UpdateAccountConfiguration</code>. If you do not specify a customer-managed key, returns empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_configuration outputs
account_configuration_id = account_configuration.id
account_configuration_encryption_config = account_configuration.encryption_config
```

---


### Scan

Scan resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_id` | String | ✅ | <p>The identifier for the resource object to be scanned.</p> |
| `analysis_type` | String |  | <p>The type of analysis you want CodeGuru Security to perform in the scan, either <code>Security</code> or <code>All</code>. The <code>Security</code> type only generates findings related to security. The <code>All</code> type generates both security findings and quality findings. Defaults to <code>Security</code> type if missing.</p> |
| `client_token` | String |  | <p>The idempotency token for the request. Amazon CodeGuru Security uses this value to prevent the accidental creation of duplicate scans if there are failures and retries.</p> |
| `tags` | HashMap<String, String> |  | <p>An array of key-value pairs used to tag a scan. A tag is a custom attribute label with two parts:</p> <ul> <li> <p>A tag key. For example, <code>CostCenter</code>, <code>Environment</code>, or <code>Secret</code>. Tag keys are case sensitive.</p> </li> <li> <p>An optional tag value field. For example, <code>111122223333</code>, <code>Production</code>, or a team name. Omitting the tag value is the same as using an empty string. Tag values are case sensitive.</p> </li> </ul> |
| `scan_name` | String | ✅ | <p>The unique name that CodeGuru Security uses to track revisions across multiple scans of the same resource. Only allowed for a <code>STANDARD</code> scan type. </p> |
| `scan_type` | String |  | <p>The type of scan, either <code>Standard</code> or <code>Express</code>. Defaults to <code>Standard</code> type if missing.</p> <p> <code>Express</code> scans run on limited resources and use a limited set of detectors to analyze your code in near-real time. <code>Standard</code> scans have standard resource limits and use the full set of detectors to analyze your code.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `updated_at` | String | <p>The time when the scan was last updated. Only available for <code>STANDARD</code> scan types.</p> |
| `scan_name_arn` | String | <p>The ARN for the scan name.</p> |
| `error_message` | String | <p>Details about the error that causes a scan to fail to be retrieved.</p> |
| `number_of_revisions` | i64 | <p>The number of times a scan has been re-run on a revised resource.</p> |
| `scan_name` | String | <p>The name of the scan.</p> |
| `analysis_type` | String | <p>The type of analysis CodeGuru Security performed in the scan, either <code>Security</code> or <code>All</code>. The <code>Security</code> type only generates findings related to security. The <code>All</code> type generates both security findings and quality findings.</p> |
| `scan_state` | String | <p>The current state of the scan. Returns either <code>InProgress</code>, <code>Successful</code>, or <code>Failed</code>.</p> |
| `run_id` | String | <p>UUID that identifies the individual scan run.</p> |
| `created_at` | String | <p>The time the scan was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create scan
scan = provider.codeguru_security.Scan {
    resource_id = "value"  # <p>The identifier for the resource object to be scanned.</p>
    scan_name = "value"  # <p>The unique name that CodeGuru Security uses to track revisions across multiple scans of the same resource. Only allowed for a <code>STANDARD</code> scan type. </p>
}

# Access scan outputs
scan_id = scan.id
scan_updated_at = scan.updated_at
scan_scan_name_arn = scan.scan_name_arn
scan_error_message = scan.error_message
scan_number_of_revisions = scan.number_of_revisions
scan_scan_name = scan.scan_name
scan_analysis_type = scan.analysis_type
scan_scan_state = scan.scan_state
scan_run_id = scan.run_id
scan_created_at = scan.created_at
```

---


### Findings

Findings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A pagination token. You can use this in future calls to <code>GetFindings</code> to continue listing results after the current page. </p> |
| `findings` | Vec<String> | <p>A list of findings generated by the specified scan.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access findings outputs
findings_id = findings.id
findings_next_token = findings.next_token
findings_findings = findings.findings
```

---


### Metrics_summary

MetricsSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metrics_summary` | String | <p>The summary metrics from the specified date.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access metrics_summary outputs
metrics_summary_id = metrics_summary.id
metrics_summary_metrics_summary = metrics_summary.metrics_summary
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple upload_url resources
upload_url_0 = provider.codeguru_security.Upload_url {
    scan_name = "value-0"
}
upload_url_1 = provider.codeguru_security.Upload_url {
    scan_name = "value-1"
}
upload_url_2 = provider.codeguru_security.Upload_url {
    scan_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    upload_url = provider.codeguru_security.Upload_url {
        scan_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Codeguru_security Documentation](https://docs.aws.amazon.com/codeguru_security/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
