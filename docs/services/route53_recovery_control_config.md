# Route53_recovery_control_config Service



**Resources**: 5

---

## Overview

The route53_recovery_control_config service provides access to 5 resource types:

- [Resource_policy](#resource_policy) [R]
- [Safety_rule](#safety_rule) [CRUD]
- [Control_panel](#control_panel) [CRUD]
- [Cluster](#cluster) [CRUD]
- [Routing_control](#routing_control) [CRUD]

---

## Resources


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The resource policy.</p> |


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
resource_policy_policy = resource_policy.policy
```

---


### Safety_rule

SafetyRule resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>The tags associated with the safety rule.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive string of up to 64 ASCII characters. To make an idempotent API request with an action, specify a client token in the request.</p> |
| `gating_rule` | String |  | <p>The gating rule requested.</p> |
| `assertion_rule` | String |  | <p>The assertion rule requested.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assertion_rule` | String | <p>The assertion rule in the response.</p> |
| `gating_rule` | String | <p>The gating rule in the response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create safety_rule
safety_rule = provider.route53_recovery_control_config.Safety_rule {
}

# Access safety_rule outputs
safety_rule_id = safety_rule.id
safety_rule_assertion_rule = safety_rule.assertion_rule
safety_rule_gating_rule = safety_rule.gating_rule
```

---


### Control_panel

ControlPanel resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cluster_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the cluster for the control panel.</p> |
| `control_panel_name` | String | ✅ | <p>The name of the control panel.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags associated with the control panel.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive string of up to 64 ASCII characters. To make an idempotent API request with an action, specify a client token in the request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `control_panel` | String | <p>Information about the control panel.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create control_panel
control_panel = provider.route53_recovery_control_config.Control_panel {
    cluster_arn = "value"  # <p>The Amazon Resource Name (ARN) of the cluster for the control panel.</p>
    control_panel_name = "value"  # <p>The name of the control panel.</p>
}

# Access control_panel outputs
control_panel_id = control_panel.id
control_panel_control_panel = control_panel.control_panel
```

---


### Cluster

Cluster resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cluster_name` | String | ✅ | <p>The name of the cluster.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags associated with the cluster.</p> |
| `network_type` | String |  | <p>The network type of the cluster. NetworkType can be one of the following: IPV4, DUALSTACK.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive string of up to 64 ASCII characters. To make an idempotent API request with an action, specify a client token in the request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cluster` | String | <p>The cluster for the DescribeCluster request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster
cluster = provider.route53_recovery_control_config.Cluster {
    cluster_name = "value"  # <p>The name of the cluster.</p>
}

# Access cluster outputs
cluster_id = cluster.id
cluster_cluster = cluster.cluster
```

---


### Routing_control

RoutingControl resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A unique, case-sensitive string of up to 64 ASCII characters. To make an idempotent API request with an action, specify a client token in the request.</p> |
| `cluster_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the cluster that includes the routing control.</p> |
| `control_panel_arn` | String |  | <p>The Amazon Resource Name (ARN) of the control panel that includes the routing control.</p> |
| `routing_control_name` | String | ✅ | <p>The name of the routing control.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `routing_control` | String | <p>Information about the routing control.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create routing_control
routing_control = provider.route53_recovery_control_config.Routing_control {
    cluster_arn = "value"  # <p>The Amazon Resource Name (ARN) of the cluster that includes the routing control.</p>
    routing_control_name = "value"  # <p>The name of the routing control.</p>
}

# Access routing_control outputs
routing_control_id = routing_control.id
routing_control_routing_control = routing_control.routing_control
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple resource_policy resources
resource_policy_0 = provider.route53_recovery_control_config.Resource_policy {
}
resource_policy_1 = provider.route53_recovery_control_config.Resource_policy {
}
resource_policy_2 = provider.route53_recovery_control_config.Resource_policy {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    resource_policy = provider.route53_recovery_control_config.Resource_policy {
    }
```

---

## Related Documentation

- [AWS Route53_recovery_control_config Documentation](https://docs.aws.amazon.com/route53_recovery_control_config/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
