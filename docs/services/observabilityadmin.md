# Observabilityadmin Service



**Resources**: 6

---

## Overview

The observabilityadmin service provides access to 6 resource types:

- [Centralization_rule_for_organization](#centralization_rule_for_organization) [CRUD]
- [Telemetry_enrichment_status](#telemetry_enrichment_status) [R]
- [Telemetry_rule_for_organization](#telemetry_rule_for_organization) [CRUD]
- [Telemetry_rule](#telemetry_rule) [CRUD]
- [Telemetry_evaluation_status](#telemetry_evaluation_status) [R]
- [Telemetry_evaluation_status_for_organization](#telemetry_evaluation_status_for_organization) [R]

---

## Resources


### Centralization_rule_for_organization

CentralizationRuleForOrganization resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>The key-value pairs to associate with the organization telemetry rule resource for categorization and management purposes.</p> |
| `rule_name` | String | ✅ | <p>A unique name for the organization-wide centralization rule being created.</p> |
| `rule` | String | ✅ | <p>The configuration details for the organization-wide centralization rule, including the source configuration and the destination configuration to centralize telemetry data across the organization.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_update_time_stamp` | i64 | <p>The timestamp when the organization centralization rule was last updated.</p> |
| `rule_name` | String | <p>The name of the organization centralization rule.</p> |
| `failure_reason` | String | <p>The reason why an organization centralization rule is marked UNHEALTHY.</p> |
| `rule_arn` | String | <p>The Amazon Resource Name (ARN) of the organization centralization rule.</p> |
| `created_time_stamp` | i64 | <p>The timestamp when the organization centralization rule was created.</p> |
| `created_region` | String | <p>The Amazon Web Services region where the organization centralization rule was created.</p> |
| `creator_account_id` | String | <p>The Amazon Web Services Account that created the organization centralization rule.</p> |
| `centralization_rule` | String | <p>The configuration details for the organization centralization rule.</p> |
| `rule_health` | String | <p>The health status of the organization centralization rule.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create centralization_rule_for_organization
centralization_rule_for_organization = provider.observabilityadmin.Centralization_rule_for_organization {
    rule_name = "value"  # <p>A unique name for the organization-wide centralization rule being created.</p>
    rule = "value"  # <p>The configuration details for the organization-wide centralization rule, including the source configuration and the destination configuration to centralize telemetry data across the organization.</p>
}

# Access centralization_rule_for_organization outputs
centralization_rule_for_organization_id = centralization_rule_for_organization.id
centralization_rule_for_organization_last_update_time_stamp = centralization_rule_for_organization.last_update_time_stamp
centralization_rule_for_organization_rule_name = centralization_rule_for_organization.rule_name
centralization_rule_for_organization_failure_reason = centralization_rule_for_organization.failure_reason
centralization_rule_for_organization_rule_arn = centralization_rule_for_organization.rule_arn
centralization_rule_for_organization_created_time_stamp = centralization_rule_for_organization.created_time_stamp
centralization_rule_for_organization_created_region = centralization_rule_for_organization.created_region
centralization_rule_for_organization_creator_account_id = centralization_rule_for_organization.creator_account_id
centralization_rule_for_organization_centralization_rule = centralization_rule_for_organization.centralization_rule
centralization_rule_for_organization_rule_health = centralization_rule_for_organization.rule_health
```

---


### Telemetry_enrichment_status

TelemetryEnrichmentStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `aws_resource_explorer_managed_view_arn` | String | <p> The Amazon Resource Name (ARN) of the Amazon Web Services Resource Explorer managed view used for resource tags for telemetry, if the feature is enabled. </p> |
| `status` | String | <p> The current status of the resource tags for telemetry feature (<code>Running</code>, <code>Stopped</code>, or <code>Impaired</code>). </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access telemetry_enrichment_status outputs
telemetry_enrichment_status_id = telemetry_enrichment_status.id
telemetry_enrichment_status_aws_resource_explorer_managed_view_arn = telemetry_enrichment_status.aws_resource_explorer_managed_view_arn
telemetry_enrichment_status_status = telemetry_enrichment_status.status
```

---


### Telemetry_rule_for_organization

TelemetryRuleForOrganization resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rule_name` | String | ✅ | <p> A unique name for the organization-wide telemetry rule being created. </p> |
| `tags` | HashMap<String, String> |  | <p> The key-value pairs to associate with the organization telemetry rule resource for categorization and management purposes. </p> |
| `rule` | String | ✅ | <p> The configuration details for the organization-wide telemetry rule, including the resource type, telemetry type, destination configuration, and selection criteria for which resources the rule applies to across the organization. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_update_time_stamp` | i64 | <p> The timestamp when the organization telemetry rule was last updated. </p> |
| `telemetry_rule` | String | <p> The configuration details of the organization telemetry rule. </p> |
| `rule_name` | String | <p> The name of the organization telemetry rule. </p> |
| `created_time_stamp` | i64 | <p> The timestamp when the organization telemetry rule was created. </p> |
| `rule_arn` | String | <p> The Amazon Resource Name (ARN) of the organization telemetry rule. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create telemetry_rule_for_organization
telemetry_rule_for_organization = provider.observabilityadmin.Telemetry_rule_for_organization {
    rule_name = "value"  # <p> A unique name for the organization-wide telemetry rule being created. </p>
    rule = "value"  # <p> The configuration details for the organization-wide telemetry rule, including the resource type, telemetry type, destination configuration, and selection criteria for which resources the rule applies to across the organization. </p>
}

# Access telemetry_rule_for_organization outputs
telemetry_rule_for_organization_id = telemetry_rule_for_organization.id
telemetry_rule_for_organization_last_update_time_stamp = telemetry_rule_for_organization.last_update_time_stamp
telemetry_rule_for_organization_telemetry_rule = telemetry_rule_for_organization.telemetry_rule
telemetry_rule_for_organization_rule_name = telemetry_rule_for_organization.rule_name
telemetry_rule_for_organization_created_time_stamp = telemetry_rule_for_organization.created_time_stamp
telemetry_rule_for_organization_rule_arn = telemetry_rule_for_organization.rule_arn
```

---


### Telemetry_rule

TelemetryRule resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rule_name` | String | ✅ | <p> A unique name for the telemetry rule being created. </p> |
| `tags` | HashMap<String, String> |  | <p> The key-value pairs to associate with the telemetry rule resource for categorization and management purposes. </p> |
| `rule` | String | ✅ | <p> The configuration details for the telemetry rule, including the resource type, telemetry type, destination configuration, and selection criteria for which resources the rule applies to. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rule_name` | String | <p> The name of the telemetry rule. </p> |
| `created_time_stamp` | i64 | <p> The timestamp when the telemetry rule was created. </p> |
| `telemetry_rule` | String | <p> The configuration details of the telemetry rule. </p> |
| `rule_arn` | String | <p> The Amazon Resource Name (ARN) of the telemetry rule. </p> |
| `last_update_time_stamp` | i64 | <p> The timestamp when the telemetry rule was last updated. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create telemetry_rule
telemetry_rule = provider.observabilityadmin.Telemetry_rule {
    rule_name = "value"  # <p> A unique name for the telemetry rule being created. </p>
    rule = "value"  # <p> The configuration details for the telemetry rule, including the resource type, telemetry type, destination configuration, and selection criteria for which resources the rule applies to. </p>
}

# Access telemetry_rule outputs
telemetry_rule_id = telemetry_rule.id
telemetry_rule_rule_name = telemetry_rule.rule_name
telemetry_rule_created_time_stamp = telemetry_rule.created_time_stamp
telemetry_rule_telemetry_rule = telemetry_rule.telemetry_rule
telemetry_rule_rule_arn = telemetry_rule.rule_arn
telemetry_rule_last_update_time_stamp = telemetry_rule.last_update_time_stamp
```

---


### Telemetry_evaluation_status

TelemetryEvaluationStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `failure_reason` | String | <p> Describes the reason for the failure status. The field will only be populated if <code>Status</code> is <code>FAILED_START</code> or <code>FAILED_STOP</code>. </p> |
| `status` | String | <p> The onboarding status of the telemetry config feature. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access telemetry_evaluation_status outputs
telemetry_evaluation_status_id = telemetry_evaluation_status.id
telemetry_evaluation_status_failure_reason = telemetry_evaluation_status.failure_reason
telemetry_evaluation_status_status = telemetry_evaluation_status.status
```

---


### Telemetry_evaluation_status_for_organization

TelemetryEvaluationStatusForOrganization resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p> The onboarding status of the telemetry config feature for the organization. </p> |
| `failure_reason` | String | <p> This field describes the reason for the failure status. The field will only be populated if <code>Status</code> is <code>FAILED_START</code> or <code>FAILED_STOP</code>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access telemetry_evaluation_status_for_organization outputs
telemetry_evaluation_status_for_organization_id = telemetry_evaluation_status_for_organization.id
telemetry_evaluation_status_for_organization_status = telemetry_evaluation_status_for_organization.status
telemetry_evaluation_status_for_organization_failure_reason = telemetry_evaluation_status_for_organization.failure_reason
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple centralization_rule_for_organization resources
centralization_rule_for_organization_0 = provider.observabilityadmin.Centralization_rule_for_organization {
    rule_name = "value-0"
    rule = "value-0"
}
centralization_rule_for_organization_1 = provider.observabilityadmin.Centralization_rule_for_organization {
    rule_name = "value-1"
    rule = "value-1"
}
centralization_rule_for_organization_2 = provider.observabilityadmin.Centralization_rule_for_organization {
    rule_name = "value-2"
    rule = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    centralization_rule_for_organization = provider.observabilityadmin.Centralization_rule_for_organization {
        rule_name = "production-value"
        rule = "production-value"
    }
```

---

## Related Documentation

- [AWS Observabilityadmin Documentation](https://docs.aws.amazon.com/observabilityadmin/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
