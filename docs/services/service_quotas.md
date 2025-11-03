# Service_quotas Service



**Resources**: 9

---

## Overview

The service_quotas service provides access to 9 resource types:

- [Service_quota](#service_quota) [R]
- [Service_quota_increase_request_into_template](#service_quota_increase_request_into_template) [C]
- [Aws_default_service_quota](#aws_default_service_quota) [R]
- [Requested_service_quota_change](#requested_service_quota_change) [R]
- [Service_quota_increase_request_from_template](#service_quota_increase_request_from_template) [RD]
- [Auto_management_configuration](#auto_management_configuration) [R]
- [Support_case](#support_case) [C]
- [Auto_management](#auto_management) [U]
- [Association_for_service_quota_template](#association_for_service_quota_template) [R]

---

## Resources


### Service_quota

ServiceQuota resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `quota` | String | <p>Information about the quota.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_quota outputs
service_quota_id = service_quota.id
service_quota_quota = service_quota.quota
```

---


### Service_quota_increase_request_into_template

ServiceQuotaIncreaseRequestIntoTemplate resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `desired_value` | f64 | ✅ | <p>Specifies the new, increased value for the quota.</p> |
| `service_code` | String | ✅ | <p>Specifies the service identifier. To find the service code value 
             for an Amazon Web Services service, use the <a>ListServices</a> operation.</p> |
| `aws_region` | String | ✅ | <p>Specifies the Amazon Web Services Region to which the template applies.</p> |
| `quota_code` | String | ✅ | <p>Specifies the quota identifier. To find the quota code for a specific 
             quota, use the <a>ListServiceQuotas</a> operation, and look for the
             <code>QuotaCode</code> response in the output for the quota you want.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create service_quota_increase_request_into_template
service_quota_increase_request_into_template = provider.service_quotas.Service_quota_increase_request_into_template {
    desired_value = "value"  # <p>Specifies the new, increased value for the quota.</p>
    service_code = "value"  # <p>Specifies the service identifier. To find the service code value 
             for an Amazon Web Services service, use the <a>ListServices</a> operation.</p>
    aws_region = "value"  # <p>Specifies the Amazon Web Services Region to which the template applies.</p>
    quota_code = "value"  # <p>Specifies the quota identifier. To find the quota code for a specific 
             quota, use the <a>ListServiceQuotas</a> operation, and look for the
             <code>QuotaCode</code> response in the output for the quota you want.</p>
}

```

---


### Aws_default_service_quota

AWSDefaultServiceQuota resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `quota` | String | <p>Information about the quota.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access aws_default_service_quota outputs
aws_default_service_quota_id = aws_default_service_quota.id
aws_default_service_quota_quota = aws_default_service_quota.quota
```

---


### Requested_service_quota_change

RequestedServiceQuotaChange resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `requested_quota` | String | <p>Information about the quota increase request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access requested_service_quota_change outputs
requested_service_quota_change_id = requested_service_quota_change.id
requested_service_quota_change_requested_quota = requested_service_quota_change.requested_quota
```

---


### Service_quota_increase_request_from_template

ServiceQuotaIncreaseRequestFromTemplate resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_quota_increase_request_in_template` | String | <p>Information about the quota increase request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_quota_increase_request_from_template outputs
service_quota_increase_request_from_template_id = service_quota_increase_request_from_template.id
service_quota_increase_request_from_template_service_quota_increase_request_in_template = service_quota_increase_request_from_template.service_quota_increase_request_in_template
```

---


### Auto_management_configuration

AutoManagementConfiguration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `opt_in_level` | String | <p>Information on the opt-in level for Automatic Management. Only Amazon Web Services account level is supported.</p> |
| `opt_in_type` | String | <p>Information on the opt-in type for Automatic Management. There are two modes: Notify only and Notify and Auto-Adjust. Currently, only
                                NotifyOnly is available.</p> |
| `notification_arn` | String | <p>The <a href="https://docs.aws.amazon.com/notifications/latest/userguide/resource-level-permissions.html#rlp-table">User Notifications</a> Amazon Resource Name (ARN) for Automatic Management notifications.</p> |
| `exclusion_list` | HashMap<String, Vec<String>> | <p>List of Amazon Web Services services excluded from Automatic Management.
            You won't be notified of Service Quotas utilization for Amazon Web Services services added to the
                Automatic Management exclusion list.
         </p> |
| `opt_in_status` | String | <p>Status on whether Automatic Management is started or stopped.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access auto_management_configuration outputs
auto_management_configuration_id = auto_management_configuration.id
auto_management_configuration_opt_in_level = auto_management_configuration.opt_in_level
auto_management_configuration_opt_in_type = auto_management_configuration.opt_in_type
auto_management_configuration_notification_arn = auto_management_configuration.notification_arn
auto_management_configuration_exclusion_list = auto_management_configuration.exclusion_list
auto_management_configuration_opt_in_status = auto_management_configuration.opt_in_status
```

---


### Support_case

SupportCase resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `request_id` | String | ✅ | <p>The ID of the pending quota increase request for which you want to open a Support case. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create support_case
support_case = provider.service_quotas.Support_case {
    request_id = "value"  # <p>The ID of the pending quota increase request for which you want to open a Support case. </p>
}

```

---


### Auto_management

AutoManagement resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `exclusion_list` | HashMap<String, Vec<String>> |  | <p>List of Amazon Web Services services you want to exclude from Automatic Management.
            You won't be notified of Service Quotas utilization for Amazon Web Services services added to the
                Automatic Management exclusion list.
         </p> |
| `notification_arn` | String |  | <p>The <a href="https://docs.aws.amazon.com/notifications/latest/userguide/resource-level-permissions.html#rlp-table">User Notifications</a> Amazon Resource Name (ARN) for Automatic Management notifications you want
            to update.</p> |
| `opt_in_type` | String |  | <p>Information on the opt-in type for your Automatic Management configuration.
            There are two modes: Notify only and Notify and Auto-Adjust. Currently, only
                                NotifyOnly is available.</p> |



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


### Association_for_service_quota_template

AssociationForServiceQuotaTemplate resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_quota_template_association_status` | String | <p>The association status. If the status is <code>ASSOCIATED</code>, the quota increase
            requests in the template are automatically applied to new Amazon Web Services accounts in your
            organization.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access association_for_service_quota_template outputs
association_for_service_quota_template_id = association_for_service_quota_template.id
association_for_service_quota_template_service_quota_template_association_status = association_for_service_quota_template.service_quota_template_association_status
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple service_quota resources
service_quota_0 = provider.service_quotas.Service_quota {
}
service_quota_1 = provider.service_quotas.Service_quota {
}
service_quota_2 = provider.service_quotas.Service_quota {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    service_quota = provider.service_quotas.Service_quota {
    }
```

---

## Related Documentation

- [AWS Service_quotas Documentation](https://docs.aws.amazon.com/service_quotas/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
