# License_manager Service



**Resources**: 13

---

## Overview

The license_manager service provides access to 13 resource types:

- [License_configuration](#license_configuration) [CRUD]
- [License_version](#license_version) [C]
- [Token](#token) [CD]
- [License_conversion_task](#license_conversion_task) [R]
- [License](#license) [CRD]
- [License_conversion_task_for_resource](#license_conversion_task_for_resource) [C]
- [License_manager_report_generator](#license_manager_report_generator) [CRUD]
- [Grant_version](#grant_version) [C]
- [Grant](#grant) [CRD]
- [License_usage](#license_usage) [R]
- [License_specifications_for_resource](#license_specifications_for_resource) [U]
- [Access_token](#access_token) [R]
- [Service_settings](#service_settings) [RU]

---

## Resources


### License_configuration

LicenseConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>Tags to add to the license configuration.</p> |
| `disassociate_when_not_found` | bool |  | <p>When true, disassociates a resource when software is uninstalled.</p> |
| `product_information_list` | Vec<String> |  | <p>Product information.</p> |
| `name` | String | ✅ | <p>Name of the license configuration.</p> |
| `license_rules` | String |  | <p>License rules. The syntax is #name=value (for example, #allowedTenancy=EC2-DedicatedHost). The available rules 
         vary by dimension, as follows.</p>
         <ul>
            <li>
               <p>
                  <code>Cores</code> dimension: <code>allowedTenancy</code> |
               <code>licenseAffinityToHost</code> |
               <code>maximumCores</code> | <code>minimumCores</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Instances</code> dimension: <code>allowedTenancy</code> | 
               <code>maximumVcpus</code> | <code>minimumVcpus</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Sockets</code> dimension: <code>allowedTenancy</code> | 
               <code>licenseAffinityToHost</code> |
               <code>maximumSockets</code> | <code>minimumSockets</code>
               </p>
            </li>
            <li>
               <p>
                  <code>vCPUs</code> dimension: <code>allowedTenancy</code> | 
               <code>honorVcpuOptimization</code> | 
               <code>maximumVcpus</code> | <code>minimumVcpus</code>
               </p>
            </li>
         </ul>
         <p>The unit for <code>licenseAffinityToHost</code> is days and the range is 1 to 180. The possible 
         values for <code>allowedTenancy</code> are <code>EC2-Default</code>, <code>EC2-DedicatedHost</code>, and 
         <code>EC2-DedicatedInstance</code>. The possible values for <code>honorVcpuOptimization</code> are 
         <code>True</code> and <code>False</code>.</p> |
| `description` | String |  | <p>Description of the license configuration.</p> |
| `license_count` | i64 |  | <p>Number of licenses managed by the license configuration.</p> |
| `license_count_hard_limit` | bool |  | <p>Indicates whether hard or soft license enforcement is used. Exceeding a hard limit
         blocks the launch of new instances.</p> |
| `license_counting_type` | String | ✅ | <p>Dimension used to track the license inventory.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `disassociate_when_not_found` | bool | <p>When true, disassociates a resource when software is uninstalled.</p> |
| `license_count_hard_limit` | bool | <p>Sets the number of available licenses as a hard limit.</p> |
| `product_information_list` | Vec<String> | <p>Product information.</p> |
| `status` | String | <p>License configuration status.</p> |
| `managed_resource_summary_list` | Vec<String> | <p>Summaries of the managed resources.</p> |
| `license_rules` | String | <p>License rules.</p> |
| `description` | String | <p>Description of the license configuration.</p> |
| `automated_discovery_information` | String | <p>Automated discovery information.</p> |
| `tags` | Vec<String> | <p>Tags for the license configuration.</p> |
| `license_configuration_id` | String | <p>Unique ID for the license configuration.</p> |
| `license_counting_type` | String | <p>Dimension for which the licenses are counted.</p> |
| `license_count` | i64 | <p>Number of available licenses.</p> |
| `owner_account_id` | String | <p>Account ID of the owner of the license configuration.</p> |
| `consumed_licenses` | i64 | <p>Number of licenses assigned to resources.</p> |
| `consumed_license_summary_list` | Vec<String> | <p>Summaries of the licenses consumed by resources.</p> |
| `license_configuration_arn` | String | <p>Amazon Resource Name (ARN) of the license configuration.</p> |
| `name` | String | <p>Name of the license configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create license_configuration
license_configuration = provider.license_manager.License_configuration {
    name = "value"  # <p>Name of the license configuration.</p>
    license_counting_type = "value"  # <p>Dimension used to track the license inventory.</p>
}

# Access license_configuration outputs
license_configuration_id = license_configuration.id
license_configuration_disassociate_when_not_found = license_configuration.disassociate_when_not_found
license_configuration_license_count_hard_limit = license_configuration.license_count_hard_limit
license_configuration_product_information_list = license_configuration.product_information_list
license_configuration_status = license_configuration.status
license_configuration_managed_resource_summary_list = license_configuration.managed_resource_summary_list
license_configuration_license_rules = license_configuration.license_rules
license_configuration_description = license_configuration.description
license_configuration_automated_discovery_information = license_configuration.automated_discovery_information
license_configuration_tags = license_configuration.tags
license_configuration_license_configuration_id = license_configuration.license_configuration_id
license_configuration_license_counting_type = license_configuration.license_counting_type
license_configuration_license_count = license_configuration.license_count
license_configuration_owner_account_id = license_configuration.owner_account_id
license_configuration_consumed_licenses = license_configuration.consumed_licenses
license_configuration_consumed_license_summary_list = license_configuration.consumed_license_summary_list
license_configuration_license_configuration_arn = license_configuration.license_configuration_arn
license_configuration_name = license_configuration.name
```

---


### License_version

LicenseVersion resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entitlements` | Vec<String> | ✅ | <p>License entitlements.</p> |
| `issuer` | String | ✅ | <p>License issuer.</p> |
| `consumption_configuration` | String | ✅ | <p>Configuration for consumption of the license. Choose a provisional configuration for workloads
         running with continuous connectivity. Choose a borrow configuration for workloads with offline
         usage.</p> |
| `source_version` | String |  | <p>Current version of the license.</p> |
| `license_name` | String | ✅ | <p>License name.</p> |
| `license_arn` | String | ✅ | <p>Amazon Resource Name (ARN) of the license.</p> |
| `status` | String | ✅ | <p>License status.</p> |
| `validity` | String | ✅ | <p>Date and time range during which the license is valid, in ISO8601-UTC format.</p> |
| `product_name` | String | ✅ | <p>Product name.</p> |
| `license_metadata` | Vec<String> |  | <p>Information about the license.</p> |
| `home_region` | String | ✅ | <p>Home Region of the license.</p> |
| `client_token` | String | ✅ | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create license_version
license_version = provider.license_manager.License_version {
    entitlements = "value"  # <p>License entitlements.</p>
    issuer = "value"  # <p>License issuer.</p>
    consumption_configuration = "value"  # <p>Configuration for consumption of the license. Choose a provisional configuration for workloads
         running with continuous connectivity. Choose a borrow configuration for workloads with offline
         usage.</p>
    license_name = "value"  # <p>License name.</p>
    license_arn = "value"  # <p>Amazon Resource Name (ARN) of the license.</p>
    status = "value"  # <p>License status.</p>
    validity = "value"  # <p>Date and time range during which the license is valid, in ISO8601-UTC format.</p>
    product_name = "value"  # <p>Product name.</p>
    home_region = "value"  # <p>Home Region of the license.</p>
    client_token = "value"  # <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
}

```

---


### Token

Token resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expiration_in_days` | i64 |  | <p>Token expiration, in days, counted from token creation. The default is 365 days.</p> |
| `role_arns` | Vec<String> |  | <p>Amazon Resource Name (ARN) of the IAM roles to embed in the token. 
         License Manager does not check whether the roles are in use.</p> |
| `license_arn` | String | ✅ | <p>Amazon Resource Name (ARN) of the license. The ARN is mapped to the aud claim of the
          JWT token.</p> |
| `client_token` | String | ✅ | <p>Idempotency token, valid for 10 minutes.</p> |
| `token_properties` | Vec<String> |  | <p>Data specified by the caller to be included in the JWT token. The data is mapped
          to the amr claim of the JWT token.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create token
token = provider.license_manager.Token {
    license_arn = "value"  # <p>Amazon Resource Name (ARN) of the license. The ARN is mapped to the aud claim of the
          JWT token.</p>
    client_token = "value"  # <p>Idempotency token, valid for 10 minutes.</p>
}

```

---


### License_conversion_task

LicenseConversionTask resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | <p>Time at which the license type conversion task was started .</p> |
| `license_conversion_time` | String | <p>Amount of time to complete the license type conversion.</p> |
| `resource_arn` | String | <p>Amazon Resource Names (ARN) of the resources the license conversion task is associated with.</p> |
| `source_license_context` | String | <p>Information about the license type converted from.</p> |
| `license_conversion_task_id` | String | <p>ID of the license type conversion task.</p> |
| `destination_license_context` | String | <p>Information about the license type converted to.</p> |
| `status` | String | <p>Status of the license type conversion task.</p> |
| `end_time` | String | <p>Time at which the license type conversion task was completed.</p> |
| `status_message` | String | <p>The status message for the conversion task.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access license_conversion_task outputs
license_conversion_task_id = license_conversion_task.id
license_conversion_task_start_time = license_conversion_task.start_time
license_conversion_task_license_conversion_time = license_conversion_task.license_conversion_time
license_conversion_task_resource_arn = license_conversion_task.resource_arn
license_conversion_task_source_license_context = license_conversion_task.source_license_context
license_conversion_task_license_conversion_task_id = license_conversion_task.license_conversion_task_id
license_conversion_task_destination_license_context = license_conversion_task.destination_license_context
license_conversion_task_status = license_conversion_task.status
license_conversion_task_end_time = license_conversion_task.end_time
license_conversion_task_status_message = license_conversion_task.status_message
```

---


### License

License resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `license_name` | String | ✅ | <p>License name.</p> |
| `product_name` | String | ✅ | <p>Product name.</p> |
| `product_sku` | String | ✅ | <p>Product SKU.</p> |
| `license_metadata` | Vec<String> |  | <p>Information about the license.</p> |
| `entitlements` | Vec<String> | ✅ | <p>License entitlements.</p> |
| `validity` | String | ✅ | <p>Date and time range during which the license is valid, in ISO8601-UTC format.</p> |
| `beneficiary` | String | ✅ | <p>License beneficiary.</p> |
| `client_token` | String | ✅ | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p> |
| `home_region` | String | ✅ | <p>Home Region for the license.</p> |
| `tags` | Vec<String> |  | <p>Tags to add to the license. For more information about tagging support in
         License Manager, see the <a href="https://docs.aws.amazon.com/license-manager/latest/APIReference/API_TagResource.html">TagResource</a> operation.</p> |
| `consumption_configuration` | String | ✅ | <p>Configuration for consumption of the license. Choose a provisional configuration for workloads
          running with continuous connectivity. Choose a borrow configuration for workloads with offline
          usage.</p> |
| `issuer` | String | ✅ | <p>License issuer.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `license` | String | <p>License details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create license
license = provider.license_manager.License {
    license_name = "value"  # <p>License name.</p>
    product_name = "value"  # <p>Product name.</p>
    product_sku = "value"  # <p>Product SKU.</p>
    entitlements = "value"  # <p>License entitlements.</p>
    validity = "value"  # <p>Date and time range during which the license is valid, in ISO8601-UTC format.</p>
    beneficiary = "value"  # <p>License beneficiary.</p>
    client_token = "value"  # <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    home_region = "value"  # <p>Home Region for the license.</p>
    consumption_configuration = "value"  # <p>Configuration for consumption of the license. Choose a provisional configuration for workloads
          running with continuous connectivity. Choose a borrow configuration for workloads with offline
          usage.</p>
    issuer = "value"  # <p>License issuer.</p>
}

# Access license outputs
license_id = license.id
license_license = license.license
```

---


### License_conversion_task_for_resource

LicenseConversionTaskForResource resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination_license_context` | String | ✅ | <p>Information that identifies the license type you are converting to. For the structure of the destination license, see <a href="https://docs.aws.amazon.com/license-manager/latest/userguide/conversion-procedures.html#conversion-cli">Convert a license type using the CLI </a> in the <i>License Manager User Guide</i>.</p> |
| `source_license_context` | String | ✅ | <p>Information that identifies the license type you are converting from. 

         For the structure of the source license, see <a href="https://docs.aws.amazon.com/license-manager/latest/userguide/conversion-procedures.html#conversion-cli">Convert a license type using the CLI </a> in the <i>License Manager User Guide</i>.</p> |
| `resource_arn` | String | ✅ | <p>Amazon Resource Name (ARN) of the resource you are converting the license type for.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create license_conversion_task_for_resource
license_conversion_task_for_resource = provider.license_manager.License_conversion_task_for_resource {
    destination_license_context = "value"  # <p>Information that identifies the license type you are converting to. For the structure of the destination license, see <a href="https://docs.aws.amazon.com/license-manager/latest/userguide/conversion-procedures.html#conversion-cli">Convert a license type using the CLI </a> in the <i>License Manager User Guide</i>.</p>
    source_license_context = "value"  # <p>Information that identifies the license type you are converting from. 

         For the structure of the source license, see <a href="https://docs.aws.amazon.com/license-manager/latest/userguide/conversion-procedures.html#conversion-cli">Convert a license type using the CLI </a> in the <i>License Manager User Guide</i>.</p>
    resource_arn = "value"  # <p>Amazon Resource Name (ARN) of the resource you are converting the license type for.</p>
}

```

---


### License_manager_report_generator

LicenseManagerReportGenerator resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `report_context` | String | ✅ | <p>Defines the type of license configuration the report generator tracks.</p> |
| `report_frequency` | String | ✅ | <p>Frequency by which reports are generated.  Reports can be generated daily, monthly, or weekly.</p> |
| `description` | String |  | <p>Description of the report generator.</p> |
| `type` | Vec<String> | ✅ | <p>Type of reports to generate. The following report types an be generated:</p>
         <ul>
            <li>
               <p>License configuration report - Reports the number and details of consumed licenses for a license configuration.</p>
            </li>
            <li>
               <p>Resource report - Reports the tracked licenses and resource consumption for a license configuration.</p>
            </li>
         </ul> |
| `client_token` | String | ✅ | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p> |
| `tags` | Vec<String> |  | <p>Tags to add to the report generator.</p> |
| `report_generator_name` | String | ✅ | <p>Name of the report generator.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `report_generator` | String | <p>A report generator that creates periodic reports about your license configurations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create license_manager_report_generator
license_manager_report_generator = provider.license_manager.License_manager_report_generator {
    report_context = "value"  # <p>Defines the type of license configuration the report generator tracks.</p>
    report_frequency = "value"  # <p>Frequency by which reports are generated.  Reports can be generated daily, monthly, or weekly.</p>
    type = "value"  # <p>Type of reports to generate. The following report types an be generated:</p>
         <ul>
            <li>
               <p>License configuration report - Reports the number and details of consumed licenses for a license configuration.</p>
            </li>
            <li>
               <p>Resource report - Reports the tracked licenses and resource consumption for a license configuration.</p>
            </li>
         </ul>
    client_token = "value"  # <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    report_generator_name = "value"  # <p>Name of the report generator.</p>
}

# Access license_manager_report_generator outputs
license_manager_report_generator_id = license_manager_report_generator.id
license_manager_report_generator_report_generator = license_manager_report_generator.report_generator
```

---


### Grant_version

GrantVersion resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `allowed_operations` | Vec<String> |  | <p>Allowed operations for the grant.</p> |
| `source_version` | String |  | <p>Current version of the grant.</p> |
| `grant_arn` | String | ✅ | <p>Amazon Resource Name (ARN) of the grant.</p> |
| `grant_name` | String |  | <p>Grant name.</p> |
| `client_token` | String | ✅ | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p> |
| `status` | String |  | <p>Grant status.</p> |
| `status_reason` | String |  | <p>Grant status reason.</p> |
| `options` | String |  | <p>The options specified for the grant.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create grant_version
grant_version = provider.license_manager.Grant_version {
    grant_arn = "value"  # <p>Amazon Resource Name (ARN) of the grant.</p>
    client_token = "value"  # <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
}

```

---


### Grant

Grant resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `license_arn` | String | ✅ | <p>Amazon Resource Name (ARN) of the license.</p> |
| `tags` | Vec<String> |  | <p>Tags to add to the grant. For more information about tagging support in
         License Manager, see the <a href="https://docs.aws.amazon.com/license-manager/latest/APIReference/API_TagResource.html">TagResource</a> operation.</p> |
| `principals` | Vec<String> | ✅ | <p>The grant principals. You can specify one of the following as an Amazon Resource Name
         (ARN):</p>
         <ul>
            <li>
               <p>An Amazon Web Services account, which includes only the account specified.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>An organizational unit (OU), which includes all accounts in the OU.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>An organization, which will include all accounts across your organization.</p>
            </li>
         </ul> |
| `home_region` | String | ✅ | <p>Home Region of the grant.</p> |
| `allowed_operations` | Vec<String> | ✅ | <p>Allowed operations for the grant.</p> |
| `client_token` | String | ✅ | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p> |
| `grant_name` | String | ✅ | <p>Grant name.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `grant` | String | <p>Grant details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create grant
grant = provider.license_manager.Grant {
    license_arn = "value"  # <p>Amazon Resource Name (ARN) of the license.</p>
    principals = "value"  # <p>The grant principals. You can specify one of the following as an Amazon Resource Name
         (ARN):</p>
         <ul>
            <li>
               <p>An Amazon Web Services account, which includes only the account specified.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>An organizational unit (OU), which includes all accounts in the OU.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>An organization, which will include all accounts across your organization.</p>
            </li>
         </ul>
    home_region = "value"  # <p>Home Region of the grant.</p>
    allowed_operations = "value"  # <p>Allowed operations for the grant.</p>
    client_token = "value"  # <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    grant_name = "value"  # <p>Grant name.</p>
}

# Access grant outputs
grant_id = grant.id
grant_grant = grant.grant
```

---


### License_usage

LicenseUsage resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `license_usage` | String | <p>License usage details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access license_usage outputs
license_usage_id = license_usage.id
license_usage_license_usage = license_usage.license_usage
```

---


### License_specifications_for_resource

LicenseSpecificationsForResource resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `remove_license_specifications` | Vec<String> |  | <p>ARNs of the license configurations to remove.</p> |
| `resource_arn` | String | ✅ | <p>Amazon Resource Name (ARN) of the Amazon Web Services resource.</p> |
| `add_license_specifications` | Vec<String> |  | <p>ARNs of the license configurations to add.</p> |



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


### Access_token

AccessToken resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_token` | String | <p>Temporary access token.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access access_token outputs
access_token_id = access_token.id
access_token_access_token = access_token.access_token
```

---


### Service_settings

ServiceSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_cross_accounts_discovery` | bool |  | <p>Activates cross-account discovery.</p> |
| `sns_topic_arn` | String |  | <p>Amazon Resource Name (ARN) of the Amazon SNS topic used for License Manager alerts.</p> |
| `s3_bucket_arn` | String |  | <p>Amazon Resource Name (ARN) of the Amazon S3 bucket where the License Manager information is stored.</p> |
| `organization_configuration` | String |  | <p>Enables integration with Organizations for cross-account discovery.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `organization_configuration` | String | <p>Indicates whether Organizations is integrated with License Manager for
         cross-account discovery.</p> |
| `enable_cross_accounts_discovery` | bool | <p>Indicates whether cross-account discovery is enabled.</p> |
| `sns_topic_arn` | String | <p>SNS topic configured to receive notifications from License Manager.</p> |
| `license_manager_resource_share_arn` | String | <p>Amazon Resource Name (ARN) of the resource share. The License Manager management account 
         provides member accounts with access to this share.</p> |
| `s3_bucket_arn` | String | <p>Regional S3 bucket path for storing reports, license trail event data, discovery data,
         and so on.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_settings outputs
service_settings_id = service_settings.id
service_settings_organization_configuration = service_settings.organization_configuration
service_settings_enable_cross_accounts_discovery = service_settings.enable_cross_accounts_discovery
service_settings_sns_topic_arn = service_settings.sns_topic_arn
service_settings_license_manager_resource_share_arn = service_settings.license_manager_resource_share_arn
service_settings_s3_bucket_arn = service_settings.s3_bucket_arn
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple license_configuration resources
license_configuration_0 = provider.license_manager.License_configuration {
    name = "value-0"
    license_counting_type = "value-0"
}
license_configuration_1 = provider.license_manager.License_configuration {
    name = "value-1"
    license_counting_type = "value-1"
}
license_configuration_2 = provider.license_manager.License_configuration {
    name = "value-2"
    license_counting_type = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    license_configuration = provider.license_manager.License_configuration {
        name = "production-value"
        license_counting_type = "production-value"
    }
```

---

## Related Documentation

- [AWS License_manager Documentation](https://docs.aws.amazon.com/license_manager/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
