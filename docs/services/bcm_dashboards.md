# Bcm_dashboards Service



**Resources**: 2

---

## Overview

The bcm_dashboards service provides access to 2 resource types:

- [Dashboard](#dashboard) [CRUD]
- [Resource_policy](#resource_policy) [R]

---

## Resources


### Dashboard

Dashboard resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `widgets` | Vec<String> | ✅ | <p>An array of widget configurations that define the visualizations to be displayed in the dashboard. Each dashboard can contain up to 20 widgets.</p> |
| `resource_tags` | Vec<String> |  | <p>The tags to apply to the dashboard resource for organization and management.</p> |
| `name` | String | ✅ | <p>The name of the dashboard. The name must be unique within your account.</p> |
| `description` | String |  | <p>A description of the dashboard's purpose or contents.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `widgets` | Vec<String> | <p>An array of widget configurations that make up the dashboard.</p> |
| `name` | String | <p>The name of the retrieved dashboard.</p> |
| `created_at` | String | <p>The timestamp when the dashboard was created.</p> |
| `updated_at` | String | <p>The timestamp when the dashboard was last modified.</p> |
| `arn` | String | <p>The ARN of the retrieved dashboard.</p> |
| `description` | String | <p>The description of the retrieved dashboard.</p> |
| `type` | String | <p>Indicates the dashboard type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dashboard
dashboard = provider.bcm_dashboards.Dashboard {
    widgets = "value"  # <p>An array of widget configurations that define the visualizations to be displayed in the dashboard. Each dashboard can contain up to 20 widgets.</p>
    name = "value"  # <p>The name of the dashboard. The name must be unique within your account.</p>
}

# Access dashboard outputs
dashboard_id = dashboard.id
dashboard_widgets = dashboard.widgets
dashboard_name = dashboard.name
dashboard_created_at = dashboard.created_at
dashboard_updated_at = dashboard.updated_at
dashboard_arn = dashboard.arn
dashboard_description = dashboard.description
dashboard_type = dashboard.type
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_document` | String | <p>The JSON policy document that represents the dashboard's resource-based policy.</p> |
| `resource_arn` | String | <p>The ARN of the dashboard for which the resource-based policy was retrieved.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_policy_document = resource_policy.policy_document
resource_policy_resource_arn = resource_policy.resource_arn
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple dashboard resources
dashboard_0 = provider.bcm_dashboards.Dashboard {
    widgets = "value-0"
    name = "value-0"
}
dashboard_1 = provider.bcm_dashboards.Dashboard {
    widgets = "value-1"
    name = "value-1"
}
dashboard_2 = provider.bcm_dashboards.Dashboard {
    widgets = "value-2"
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    dashboard = provider.bcm_dashboards.Dashboard {
        widgets = "production-value"
        name = "production-value"
    }
```

---

## Related Documentation

- [AWS Bcm_dashboards Documentation](https://docs.aws.amazon.com/bcm_dashboards/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
