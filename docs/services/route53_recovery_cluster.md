# Route53_recovery_cluster Service



**Resources**: 2

---

## Overview

The route53_recovery_cluster service provides access to 2 resource types:

- [Routing_control_state](#routing_control_state) [RU]
- [Routing_control_states](#routing_control_states) [U]

---

## Resources


### Routing_control_state

RoutingControlState resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `routing_control_state` | String | ✅ | <p>The state of the routing control. You can set the value to ON or OFF.</p> |
| `safety_rules_to_override` | Vec<String> |  | <p>The Amazon Resource Names (ARNs) for the safety rules that you want to override when you're updating the state of
			a routing control. You can override one safety rule or multiple safety rules by including one or more ARNs, separated 
			by commas.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/routing-control.override-safety-rule.html">
			Override safety rules to reroute traffic</a> in the Amazon Route 53 Application Recovery Controller Developer Guide.</p> |
| `routing_control_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) for the routing control that you want to update the state for.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `routing_control_state` | String | <p>The state of the routing control.</p> |
| `routing_control_name` | String | <p>The routing control name.</p> |
| `routing_control_arn` | String | <p>The Amazon Resource Name (ARN) of the response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access routing_control_state outputs
routing_control_state_id = routing_control_state.id
routing_control_state_routing_control_state = routing_control_state.routing_control_state
routing_control_state_routing_control_name = routing_control_state.routing_control_name
routing_control_state_routing_control_arn = routing_control_state.routing_control_arn
```

---


### Routing_control_states

RoutingControlStates resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `safety_rules_to_override` | Vec<String> |  | <p>The Amazon Resource Names (ARNs) for the safety rules that you want to override when you're updating routing
			control states. You can override one safety rule or multiple safety rules by including one or more ARNs, separated 
			by commas.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/routing-control.override-safety-rule.html">
			Override safety rules to reroute traffic</a> in the Amazon Route 53 Application Recovery Controller Developer Guide.</p> |
| `update_routing_control_state_entries` | Vec<String> | ✅ | <p>A set of routing control entries that you want to update.</p> |



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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple routing_control_state resources
routing_control_state_0 = provider.route53_recovery_cluster.Routing_control_state {
    routing_control_state = "value-0"
    routing_control_arn = "value-0"
}
routing_control_state_1 = provider.route53_recovery_cluster.Routing_control_state {
    routing_control_state = "value-1"
    routing_control_arn = "value-1"
}
routing_control_state_2 = provider.route53_recovery_cluster.Routing_control_state {
    routing_control_state = "value-2"
    routing_control_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    routing_control_state = provider.route53_recovery_cluster.Routing_control_state {
        routing_control_state = "production-value"
        routing_control_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Route53_recovery_cluster Documentation](https://docs.aws.amazon.com/route53_recovery_cluster/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
