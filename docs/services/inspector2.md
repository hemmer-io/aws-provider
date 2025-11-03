# Inspector2 Service



**Resources**: 18

---

## Overview

The inspector2 service provides access to 18 resource types:

- [Member](#member) [R]
- [Filter](#filter) [CUD]
- [Findings_report](#findings_report) [C]
- [Cis_scan_report](#cis_scan_report) [R]
- [Delegated_admin_account](#delegated_admin_account) [R]
- [Code_security_scan](#code_security_scan) [R]
- [Findings_report_status](#findings_report_status) [R]
- [Org_ec2_deep_inspection_configuration](#org_ec2_deep_inspection_configuration) [U]
- [Cis_scan_configuration](#cis_scan_configuration) [CUD]
- [Sbom_export](#sbom_export) [CR]
- [Encryption_key](#encryption_key) [RU]
- [Code_security_integration](#code_security_integration) [CRUD]
- [Organization_configuration](#organization_configuration) [RU]
- [Cis_scan_result_details](#cis_scan_result_details) [R]
- [Clusters_for_image](#clusters_for_image) [R]
- [Configuration](#configuration) [RU]
- [Ec2_deep_inspection_configuration](#ec2_deep_inspection_configuration) [RU]
- [Code_security_scan_configuration](#code_security_scan_configuration) [CRUD]

---

## Resources


### Member

Member resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `member` | String | <p>Details of the retrieved member account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access member outputs
member_id = member.id
member_member = member.member
```

---


### Filter

Filter resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action` | String | ✅ | <p>Defines the action that is to be applied to the findings that match the filter.</p> |
| `name` | String | ✅ | <p>The name of the filter. Minimum length of 3. Maximum length of 64. Valid characters
         include alphanumeric characters, dot (.), underscore (_), and dash (-). Spaces are not
         allowed.</p> |
| `description` | String |  | <p>A description of the filter.</p> |
| `reason` | String |  | <p>The reason for creating the filter.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of tags for the filter.</p> |
| `filter_criteria` | String | ✅ | <p>Defines the criteria to be used in the filter for querying findings.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create filter
filter = provider.inspector2.Filter {
    action = "value"  # <p>Defines the action that is to be applied to the findings that match the filter.</p>
    name = "value"  # <p>The name of the filter. Minimum length of 3. Maximum length of 64. Valid characters
         include alphanumeric characters, dot (.), underscore (_), and dash (-). Spaces are not
         allowed.</p>
    filter_criteria = "value"  # <p>Defines the criteria to be used in the filter for querying findings.</p>
}

```

---


### Findings_report

FindingsReport resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `report_format` | String | ✅ | <p>The format to generate the report in.</p> |
| `filter_criteria` | String |  | <p>The filter criteria to apply to the results of the finding report.</p> |
| `s3_destination` | String | ✅ | <p>The Amazon S3 export destination for the report.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create findings_report
findings_report = provider.inspector2.Findings_report {
    report_format = "value"  # <p>The format to generate the report in.</p>
    s3_destination = "value"  # <p>The Amazon S3 export destination for the report.</p>
}

```

---


### Cis_scan_report

CisScanReport resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The status.</p> |
| `url` | String | <p> The URL where a PDF or CSV of the CIS scan report can be downloaded. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cis_scan_report outputs
cis_scan_report_id = cis_scan_report.id
cis_scan_report_status = cis_scan_report.status
cis_scan_report_url = cis_scan_report.url
```

---


### Delegated_admin_account

DelegatedAdminAccount resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delegated_admin` | String | <p>The Amazon Web Services account ID of the Amazon Inspector delegated administrator.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access delegated_admin_account outputs
delegated_admin_account_id = delegated_admin_account.id
delegated_admin_account_delegated_admin = delegated_admin_account.delegated_admin
```

---


### Code_security_scan

CodeSecurityScan resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource` | String | <p>The resource identifier for the code repository that was scanned.</p> |
| `last_commit_id` | String | <p>The identifier of the last commit that was scanned. This is only returned if the scan
         was successful or skipped.</p> |
| `status` | String | <p>The current status of the scan.</p> |
| `scan_id` | String | <p>The unique identifier of the scan.</p> |
| `status_reason` | String | <p>The reason for the current status of the scan.</p> |
| `account_id` | String | <p>The Amazon Web Services account ID associated with the scan.</p> |
| `created_at` | String | <p>The timestamp when the scan was created.</p> |
| `updated_at` | String | <p>The timestamp when the scan was last updated.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access code_security_scan outputs
code_security_scan_id = code_security_scan.id
code_security_scan_resource = code_security_scan.resource
code_security_scan_last_commit_id = code_security_scan.last_commit_id
code_security_scan_status = code_security_scan.status
code_security_scan_scan_id = code_security_scan.scan_id
code_security_scan_status_reason = code_security_scan.status_reason
code_security_scan_account_id = code_security_scan.account_id
code_security_scan_created_at = code_security_scan.created_at
code_security_scan_updated_at = code_security_scan.updated_at
```

---


### Findings_report_status

FindingsReportStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The status of the report.</p> |
| `error_message` | String | <p>The error message of the report.</p> |
| `error_code` | String | <p>The error code of the report.</p> |
| `report_id` | String | <p>The ID of the report.</p> |
| `destination` | String | <p>The destination of the report.</p> |
| `filter_criteria` | String | <p>The filter criteria associated with the report.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access findings_report_status outputs
findings_report_status_id = findings_report_status.id
findings_report_status_status = findings_report_status.status
findings_report_status_error_message = findings_report_status.error_message
findings_report_status_error_code = findings_report_status.error_code
findings_report_status_report_id = findings_report_status.report_id
findings_report_status_destination = findings_report_status.destination
findings_report_status_filter_criteria = findings_report_status.filter_criteria
```

---


### Org_ec2_deep_inspection_configuration

OrgEc2DeepInspectionConfiguration resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `org_package_paths` | Vec<String> | ✅ | <p>The Amazon Inspector deep inspection custom paths you are adding for your organization.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Cis_scan_configuration

CisScanConfiguration resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `security_level` | String | ✅ | <p> The security level for the CIS scan configuration. Security level refers to the
         Benchmark levels that CIS assigns to a profile. </p> |
| `targets` | String | ✅ | <p>The targets for the CIS scan configuration.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags for the CIS scan configuration.</p> |
| `scan_name` | String | ✅ | <p>The scan name for the CIS scan configuration.</p> |
| `schedule` | String | ✅ | <p>The schedule for the CIS scan configuration.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cis_scan_configuration
cis_scan_configuration = provider.inspector2.Cis_scan_configuration {
    security_level = "value"  # <p> The security level for the CIS scan configuration. Security level refers to the
         Benchmark levels that CIS assigns to a profile. </p>
    targets = "value"  # <p>The targets for the CIS scan configuration.</p>
    scan_name = "value"  # <p>The scan name for the CIS scan configuration.</p>
    schedule = "value"  # <p>The schedule for the CIS scan configuration.</p>
}

```

---


### Sbom_export

SbomExport resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `s3_destination` | String | ✅ | <p>Contains details of the Amazon S3 bucket and KMS key used to export findings.</p> |
| `report_format` | String | ✅ | <p>The output format for the software bill of materials (SBOM) report.</p> |
| `resource_filter_criteria` | String |  | <p>The resource filter criteria for the software bill of materials (SBOM) report.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `report_id` | String | <p>The report ID of the software bill of materials (SBOM) report.</p> |
| `error_code` | String | <p>An error code.</p> |
| `s3_destination` | String | <p>Contains details of the Amazon S3 bucket and KMS key used to export findings</p> |
| `format` | String | <p>The format of the software bill of materials (SBOM) report.</p> |
| `error_message` | String | <p>An error message.</p> |
| `status` | String | <p>The status of the software bill of materials (SBOM) report.</p> |
| `filter_criteria` | String | <p>Contains details about the resource filter criteria used for the software bill of
         materials (SBOM) report.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sbom_export
sbom_export = provider.inspector2.Sbom_export {
    s3_destination = "value"  # <p>Contains details of the Amazon S3 bucket and KMS key used to export findings.</p>
    report_format = "value"  # <p>The output format for the software bill of materials (SBOM) report.</p>
}

# Access sbom_export outputs
sbom_export_id = sbom_export.id
sbom_export_report_id = sbom_export.report_id
sbom_export_error_code = sbom_export.error_code
sbom_export_s3_destination = sbom_export.s3_destination
sbom_export_format = sbom_export.format
sbom_export_error_message = sbom_export.error_message
sbom_export_status = sbom_export.status
sbom_export_filter_criteria = sbom_export.filter_criteria
```

---


### Encryption_key

EncryptionKey resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scan_type` | String | ✅ | <p>The scan type for the encryption key.</p> |
| `resource_type` | String | ✅ | <p>The resource type for the encryption key.</p> |
| `kms_key_id` | String | ✅ | <p>A KMS key ID for the encryption key.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kms_key_id` | String | <p>A kms key ID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access encryption_key outputs
encryption_key_id = encryption_key.id
encryption_key_kms_key_id = encryption_key.kms_key_id
```

---


### Code_security_integration

CodeSecurityIntegration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the code security integration.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to apply to the code security integration.</p> |
| `type` | String | ✅ | <p>The type of repository provider for the integration.</p> |
| `details` | String |  | <p>The integration details specific to the repository provider type.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The current status of the code security integration.</p> |
| `created_on` | String | <p>The timestamp when the code security integration was created.</p> |
| `tags` | HashMap<String, String> | <p>The tags associated with the code security integration.</p> |
| `authorization_url` | String | <p>The URL used to authorize the integration with the repository provider. This is only
         returned if reauthorization is required to fix a connection issue. Otherwise, it is
         null.</p> |
| `last_update_on` | String | <p>The timestamp when the code security integration was last updated.</p> |
| `integration_arn` | String | <p>The Amazon Resource Name (ARN) of the code security integration.</p> |
| `name` | String | <p>The name of the code security integration.</p> |
| `status_reason` | String | <p>The reason for the current status of the code security integration.</p> |
| `type` | String | <p>The type of repository provider for the integration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create code_security_integration
code_security_integration = provider.inspector2.Code_security_integration {
    name = "value"  # <p>The name of the code security integration.</p>
    type = "value"  # <p>The type of repository provider for the integration.</p>
}

# Access code_security_integration outputs
code_security_integration_id = code_security_integration.id
code_security_integration_status = code_security_integration.status
code_security_integration_created_on = code_security_integration.created_on
code_security_integration_tags = code_security_integration.tags
code_security_integration_authorization_url = code_security_integration.authorization_url
code_security_integration_last_update_on = code_security_integration.last_update_on
code_security_integration_integration_arn = code_security_integration.integration_arn
code_security_integration_name = code_security_integration.name
code_security_integration_status_reason = code_security_integration.status_reason
code_security_integration_type = code_security_integration.type
```

---


### Organization_configuration

OrganizationConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auto_enable` | String | ✅ | <p>Defines which scan types are enabled automatically for new members of your Amazon Inspector
         organization.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `max_account_limit_reached` | bool | <p>Represents whether your organization has reached the maximum Amazon Web Services account limit for
         Amazon Inspector.</p> |
| `auto_enable` | String | <p>The scan types are automatically enabled for new members of your organization.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organization_configuration outputs
organization_configuration_id = organization_configuration.id
organization_configuration_max_account_limit_reached = organization_configuration.max_account_limit_reached
organization_configuration_auto_enable = organization_configuration.auto_enable
```

---


### Cis_scan_result_details

CisScanResultDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scan_result_details` | Vec<String> | <p>The scan result details.</p> |
| `next_token` | String | <p>The pagination token from a previous request that's used to retrieve the next page of
         results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cis_scan_result_details outputs
cis_scan_result_details_id = cis_scan_result_details.id
cis_scan_result_details_scan_result_details = cis_scan_result_details.scan_result_details
cis_scan_result_details_next_token = cis_scan_result_details.next_token
```

---


### Clusters_for_image

ClustersForImage resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The pagination token from a previous request used to retrieve the next page of
         results.</p> |
| `cluster` | Vec<String> | <p>A unit of work inside of a cluster, which can include metadata about the cluster.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access clusters_for_image outputs
clusters_for_image_id = clusters_for_image.id
clusters_for_image_next_token = clusters_for_image.next_token
clusters_for_image_cluster = clusters_for_image.cluster
```

---


### Configuration

Configuration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ecr_configuration` | String |  | <p>Specifies how the ECR automated re-scan will be updated for your environment.</p> |
| `ec2_configuration` | String |  | <p>Specifies how the Amazon EC2 automated scan will be updated for your environment.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ecr_configuration` | String | <p>Specifies how the ECR automated re-scan duration is currently configured for your
         environment.</p> |
| `ec2_configuration` | String | <p>Specifies how the Amazon EC2 automated scan mode is currently configured for your
         environment.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access configuration outputs
configuration_id = configuration.id
configuration_ecr_configuration = configuration.ecr_configuration
configuration_ec2_configuration = configuration.ec2_configuration
```

---


### Ec2_deep_inspection_configuration

Ec2DeepInspectionConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `activate_deep_inspection` | bool |  | <p>Specify <code>TRUE</code> to activate Amazon Inspector deep inspection in your account, or
            <code>FALSE</code> to deactivate. Member accounts in an organization cannot deactivate
         deep inspection, instead the delegated administrator for the organization can deactivate a
         member account using <a href="https://docs.aws.amazon.com/inspector/v2/APIReference/API_BatchUpdateMemberEc2DeepInspectionStatus.html">BatchUpdateMemberEc2DeepInspectionStatus</a>.</p> |
| `package_paths` | Vec<String> |  | <p>The Amazon Inspector deep inspection custom paths you are adding for your account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `package_paths` | Vec<String> | <p>The Amazon Inspector deep inspection custom paths for your account.</p> |
| `status` | String | <p>The activation status of Amazon Inspector deep inspection in your account.</p> |
| `error_message` | String | <p>An error message explaining why Amazon Inspector deep inspection configurations could not be
         retrieved for your account.</p> |
| `org_package_paths` | Vec<String> | <p>The Amazon Inspector deep inspection custom paths for your organization.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ec2_deep_inspection_configuration outputs
ec2_deep_inspection_configuration_id = ec2_deep_inspection_configuration.id
ec2_deep_inspection_configuration_package_paths = ec2_deep_inspection_configuration.package_paths
ec2_deep_inspection_configuration_status = ec2_deep_inspection_configuration.status
ec2_deep_inspection_configuration_error_message = ec2_deep_inspection_configuration.error_message
ec2_deep_inspection_configuration_org_package_paths = ec2_deep_inspection_configuration.org_package_paths
```

---


### Code_security_scan_configuration

CodeSecurityScanConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `level` | String | ✅ | <p>The security level for the scan configuration.</p> |
| `name` | String | ✅ | <p>The name of the scan configuration.</p> |
| `configuration` | String | ✅ | <p>The configuration settings for the code security scan.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to apply to the scan configuration.</p> |
| `scope_settings` | String |  | <p>The scope settings that define which repositories will be scanned. Include this
         parameter to create a default scan configuration. Otherwise Amazon Inspector creates a general scan
         configuration. </p>
         <p>A default scan configuration automatically applies to all existing and future projects
         imported into Amazon Inspector. Use the <code>BatchAssociateCodeSecurityScanConfiguration</code>
         operation to associate a general scan configuration with projects.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_at` | String | <p>The timestamp when the scan configuration was created.</p> |
| `last_updated_at` | String | <p>The timestamp when the scan configuration was last updated.</p> |
| `configuration` | String | <p>The configuration settings for the code security scan.</p> |
| `level` | String | <p>The security level for the scan configuration.</p> |
| `scope_settings` | String | <p>The scope settings that define which repositories will be scanned. If the
            <code>ScopeSetting</code> parameter is <code>ALL</code> the scan configuration applies
         to all existing and future projects imported into Amazon Inspector.</p> |
| `name` | String | <p>The name of the scan configuration.</p> |
| `scan_configuration_arn` | String | <p>The Amazon Resource Name (ARN) of the scan configuration.</p> |
| `tags` | HashMap<String, String> | <p>The tags associated with the scan configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create code_security_scan_configuration
code_security_scan_configuration = provider.inspector2.Code_security_scan_configuration {
    level = "value"  # <p>The security level for the scan configuration.</p>
    name = "value"  # <p>The name of the scan configuration.</p>
    configuration = "value"  # <p>The configuration settings for the code security scan.</p>
}

# Access code_security_scan_configuration outputs
code_security_scan_configuration_id = code_security_scan_configuration.id
code_security_scan_configuration_created_at = code_security_scan_configuration.created_at
code_security_scan_configuration_last_updated_at = code_security_scan_configuration.last_updated_at
code_security_scan_configuration_configuration = code_security_scan_configuration.configuration
code_security_scan_configuration_level = code_security_scan_configuration.level
code_security_scan_configuration_scope_settings = code_security_scan_configuration.scope_settings
code_security_scan_configuration_name = code_security_scan_configuration.name
code_security_scan_configuration_scan_configuration_arn = code_security_scan_configuration.scan_configuration_arn
code_security_scan_configuration_tags = code_security_scan_configuration.tags
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple member resources
member_0 = provider.inspector2.Member {
}
member_1 = provider.inspector2.Member {
}
member_2 = provider.inspector2.Member {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    member = provider.inspector2.Member {
    }
```

---

## Related Documentation

- [AWS Inspector2 Documentation](https://docs.aws.amazon.com/inspector2/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
