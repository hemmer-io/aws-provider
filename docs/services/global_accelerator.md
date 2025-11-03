# Global_accelerator Service



**Resources**: 9

---

## Overview

The global_accelerator service provides access to 9 resource types:

- [Accelerator_attributes](#accelerator_attributes) [RU]
- [Listener](#listener) [CRUD]
- [Accelerator](#accelerator) [CRUD]
- [Custom_routing_accelerator_attributes](#custom_routing_accelerator_attributes) [RU]
- [Custom_routing_endpoint_group](#custom_routing_endpoint_group) [CRD]
- [Custom_routing_accelerator](#custom_routing_accelerator) [CRUD]
- [Endpoint_group](#endpoint_group) [CRUD]
- [Cross_account_attachment](#cross_account_attachment) [CRUD]
- [Custom_routing_listener](#custom_routing_listener) [CRUD]

---

## Resources


### Accelerator_attributes

AcceleratorAttributes resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `flow_logs_enabled` | bool |  | <p>Update whether flow logs are enabled. The default value is false. If the value is true,
				<code>FlowLogsS3Bucket</code> and <code>FlowLogsS3Prefix</code> must be specified.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/monitoring-global-accelerator.flow-logs.html">Flow Logs</a> in
		    the <i>Global Accelerator Developer Guide</i>.</p> |
| `accelerator_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the accelerator that you want to update.</p> |
| `flow_logs_s3_bucket` | String |  | <p>The name of the Amazon S3 bucket for the flow logs. Attribute is required if <code>FlowLogsEnabled</code> is
		    <code>true</code>. The bucket must exist and have a bucket policy that grants Global Accelerator permission to write to the
			bucket.</p> |
| `flow_logs_s3_prefix` | String |  | <p>Update the prefix for the location in the Amazon S3 bucket for the flow logs. Attribute is required if
				<code>FlowLogsEnabled</code> is <code>true</code>. </p>
         <p>If you specify slash (/) for the S3 bucket prefix, the log file bucket folder structure will include a double slash (//), 
			like the following:</p>
         <p>s3-bucket_name//AWSLogs/aws_account_id</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `accelerator_attributes` | String | <p>The attributes of the accelerator.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access accelerator_attributes outputs
accelerator_attributes_id = accelerator_attributes.id
accelerator_attributes_accelerator_attributes = accelerator_attributes.accelerator_attributes
```

---


### Listener

Listener resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_affinity` | String |  | <p>Client affinity lets you direct all requests from a user to the same endpoint, if you have stateful applications,
			regardless of the port and protocol of the client request. Client affinity gives you control over whether to always
			route each client to the same specific endpoint.</p>
         <p>Global Accelerator uses a consistent-flow hashing algorithm to choose the optimal endpoint for a connection. If client
	        affinity is <code>NONE</code>, Global Accelerator uses the "five-tuple" (5-tuple) properties—source IP address, source port,
			destination IP address, destination port, and protocol—to select the hash value, and then chooses the best
			endpoint. However, with this setting, if someone uses different ports to connect to Global Accelerator, their connections might not
			be always routed to the same endpoint because the hash value changes. </p>
         <p>If you want a given client to always be routed to the same endpoint, set client affinity to <code>SOURCE_IP</code>
		    instead. When you use the <code>SOURCE_IP</code> setting, Global Accelerator uses the "two-tuple" (2-tuple) properties—
			source (client) IP address and destination IP address—to select the hash value.</p>
         <p>The default value is <code>NONE</code>.</p> |
| `idempotency_token` | String | ✅ | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the
			uniqueness—of the request.</p> |
| `port_ranges` | Vec<String> | ✅ | <p>The list of port ranges to support for connections from clients to your accelerator.</p> |
| `accelerator_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of your accelerator.</p> |
| `protocol` | String | ✅ | <p>The protocol for connections from clients to your accelerator.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `listener` | String | <p>The description of a listener.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create listener
listener = provider.global_accelerator.Listener {
    idempotency_token = "value"  # <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the
			uniqueness—of the request.</p>
    port_ranges = "value"  # <p>The list of port ranges to support for connections from clients to your accelerator.</p>
    accelerator_arn = "value"  # <p>The Amazon Resource Name (ARN) of your accelerator.</p>
    protocol = "value"  # <p>The protocol for connections from clients to your accelerator.</p>
}

# Access listener outputs
listener_id = listener.id
listener_listener = listener.listener
```

---


### Accelerator

Accelerator resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enabled` | bool |  | <p>Indicates whether an accelerator is enabled. The value is true or false. The default value is true. </p>
         <p>If the value is set to true, an accelerator cannot be deleted. If set to false, the accelerator can be deleted.</p> |
| `idempotency_token` | String | ✅ | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the
			uniqueness—of an accelerator.</p> |
| `ip_address_type` | String |  | <p>The IP address type that an accelerator supports. For a standard accelerator, the value can be IPV4 or DUAL_STACK.</p> |
| `name` | String | ✅ | <p>The name of the accelerator. The name can have a maximum of 64 characters, must contain only alphanumeric characters, 
			periods (.), or hyphens (-), and must not begin or end with a hyphen or period.</p> |
| `tags` | Vec<String> |  | <p>Create tags for an accelerator.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/tagging-in-global-accelerator.html">Tagging
		    in Global Accelerator</a> in the <i>Global Accelerator Developer Guide</i>.</p> |
| `ip_addresses` | Vec<String> |  | <p>Optionally, if you've added your own IP address pool to Global Accelerator (BYOIP), you can choose an IPv4 address 
			from your own pool to use for the accelerator's static IPv4 address when you create an accelerator. </p>
         <p>After you bring an address range to Amazon Web Services, it appears in your account as an address pool. 
	    	When you create an accelerator, you can assign one IPv4 address from your range to it. Global Accelerator assigns 
	    	you a second static IPv4 address from an Amazon IP address range. If you bring two IPv4 address ranges 
	    	to Amazon Web Services, you can assign one IPv4 address from each range to your accelerator. This restriction is 
			because Global Accelerator assigns each address range to a different network zone, for high availability.</p>
         <p>You can specify one or two addresses, separated by a space. Do not include the /32 suffix.</p>
         <p>Note that you can't update IP addresses for an existing accelerator. To change them, you must create a new 
			accelerator with the new addresses.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/using-byoip.html">Bring 
		    your own IP addresses (BYOIP)</a> in the <i>Global Accelerator Developer Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `accelerator` | String | <p>The description of the accelerator.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create accelerator
accelerator = provider.global_accelerator.Accelerator {
    idempotency_token = "value"  # <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the
			uniqueness—of an accelerator.</p>
    name = "value"  # <p>The name of the accelerator. The name can have a maximum of 64 characters, must contain only alphanumeric characters, 
			periods (.), or hyphens (-), and must not begin or end with a hyphen or period.</p>
}

# Access accelerator outputs
accelerator_id = accelerator.id
accelerator_accelerator = accelerator.accelerator
```

---


### Custom_routing_accelerator_attributes

CustomRoutingAcceleratorAttributes resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `flow_logs_s3_prefix` | String |  | <p>Update the prefix for the location in the Amazon S3 bucket for the flow logs. Attribute is required if
		<code>FlowLogsEnabled</code> is <code>true</code>. </p>
         <p>If you don’t specify a prefix, the flow logs are stored in the
		root of the bucket. If you specify slash (/) for the S3 bucket prefix, the log file bucket folder structure will include a double slash (//), like the following:</p>
         <p>DOC-EXAMPLE-BUCKET//AWSLogs/aws_account_id</p> |
| `accelerator_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the custom routing accelerator to update attributes for.</p> |
| `flow_logs_enabled` | bool |  | <p>Update whether flow logs are enabled. The default value is false. If the value is true,
		<code>FlowLogsS3Bucket</code> and <code>FlowLogsS3Prefix</code> must be specified.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/monitoring-global-accelerator.flow-logs.html">Flow logs</a> in
	    the <i>Global Accelerator Developer Guide</i>.</p> |
| `flow_logs_s3_bucket` | String |  | <p>The name of the Amazon S3 bucket for the flow logs. Attribute is required if <code>FlowLogsEnabled</code> is
	    <code>true</code>. The bucket must exist and have a bucket policy that grants Global Accelerator permission to write to the
		bucket.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `accelerator_attributes` | String | <p>The attributes of the custom routing accelerator.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access custom_routing_accelerator_attributes outputs
custom_routing_accelerator_attributes_id = custom_routing_accelerator_attributes.id
custom_routing_accelerator_attributes_accelerator_attributes = custom_routing_accelerator_attributes.accelerator_attributes
```

---


### Custom_routing_endpoint_group

CustomRoutingEndpointGroup resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `endpoint_group_region` | String | ✅ | <p>The Amazon Web Services Region where the endpoint group is located. A listener can have only one endpoint group in a
		specific Region.</p> |
| `idempotency_token` | String | ✅ | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the
		uniqueness—of the request.</p> |
| `destination_configurations` | Vec<String> | ✅ | <p>Sets the port range and protocol for all endpoints (virtual private cloud subnets) in a custom routing endpoint group to accept 
		client traffic on.</p> |
| `listener_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the listener for a custom routing endpoint.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoint_group` | String | <p>The description of an endpoint group for a custom routing accelerator.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create custom_routing_endpoint_group
custom_routing_endpoint_group = provider.global_accelerator.Custom_routing_endpoint_group {
    endpoint_group_region = "value"  # <p>The Amazon Web Services Region where the endpoint group is located. A listener can have only one endpoint group in a
		specific Region.</p>
    idempotency_token = "value"  # <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the
		uniqueness—of the request.</p>
    destination_configurations = "value"  # <p>Sets the port range and protocol for all endpoints (virtual private cloud subnets) in a custom routing endpoint group to accept 
		client traffic on.</p>
    listener_arn = "value"  # <p>The Amazon Resource Name (ARN) of the listener for a custom routing endpoint.</p>
}

# Access custom_routing_endpoint_group outputs
custom_routing_endpoint_group_id = custom_routing_endpoint_group.id
custom_routing_endpoint_group_endpoint_group = custom_routing_endpoint_group.endpoint_group
```

---


### Custom_routing_accelerator

CustomRoutingAccelerator resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>Create tags for an accelerator.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/tagging-in-global-accelerator.html">Tagging
	    in Global Accelerator</a> in the <i>Global Accelerator Developer Guide</i>.</p> |
| `name` | String | ✅ | <p>The name of a custom routing accelerator. The name can have a maximum of 64 characters, must contain 
		only alphanumeric characters or hyphens (-), and must not begin or end with a hyphen.</p> |
| `ip_addresses` | Vec<String> |  | <p>Optionally, if you've added your own IP address pool to Global Accelerator (BYOIP), you can choose an IPv4 address 
			from your own pool to use for the accelerator's static IPv4 address when you create an accelerator. </p>
         <p>After you bring an address range to Amazon Web Services, it appears in your account as an address pool. 
			When you create an accelerator, you can assign one IPv4 address from your range to it. Global Accelerator assigns 
			you a second static IPv4 address from an Amazon IP address range. If you bring two IPv4 address ranges 
			to Amazon Web Services, you can assign one IPv4 address from each range to your accelerator. This restriction is 
			because Global Accelerator assigns each address range to a different network zone, for high availability.</p>
         <p>You can specify one or two addresses, separated by a space. Do not include the /32 suffix.</p>
         <p>Note that you can't update IP addresses for an existing accelerator. To change them, you must create a new 
			accelerator with the new addresses.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/using-byoip.html">Bring 
			your own IP addresses (BYOIP)</a> in the <i>Global Accelerator Developer Guide</i>.</p> |
| `ip_address_type` | String |  | <p>The IP address type that an accelerator supports. For a custom routing accelerator, the value must be IPV4.</p> |
| `idempotency_token` | String | ✅ | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that
			is, the uniqueness—of the request.</p> |
| `enabled` | bool |  | <p>Indicates whether an accelerator is enabled. The value is true or false. The default value is true. </p>
         <p>If the value is set to true, an accelerator cannot be deleted. If set to false, the accelerator can be deleted.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `accelerator` | String | <p>The description of the custom routing accelerator.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create custom_routing_accelerator
custom_routing_accelerator = provider.global_accelerator.Custom_routing_accelerator {
    name = "value"  # <p>The name of a custom routing accelerator. The name can have a maximum of 64 characters, must contain 
		only alphanumeric characters or hyphens (-), and must not begin or end with a hyphen.</p>
    idempotency_token = "value"  # <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that
			is, the uniqueness—of the request.</p>
}

# Access custom_routing_accelerator outputs
custom_routing_accelerator_id = custom_routing_accelerator.id
custom_routing_accelerator_accelerator = custom_routing_accelerator.accelerator
```

---


### Endpoint_group

EndpointGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `port_overrides` | Vec<String> |  | <p>Override specific listener ports used to route traffic to endpoints that are part of this endpoint group.
			For example, you can create a port override in which the listener 
			receives user traffic on ports 80 and 443, but your accelerator routes that traffic to ports 1080 
			and 1443, respectively, on the endpoints.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/about-endpoint-groups-port-override.html">
			Overriding listener ports</a> in the <i>Global Accelerator Developer Guide</i>.</p> |
| `traffic_dial_percentage` | String |  | <p>The percentage of traffic to send to an Amazon Web Services Region. Additional traffic is distributed to other endpoint groups for
			this listener. </p>
         <p>Use this action to increase (dial up) or decrease (dial down) traffic to a specific Region. The percentage is
			applied to the traffic that would otherwise have been routed to the Region based on optimal routing.</p>
         <p>The default value is 100.</p> |
| `health_check_port` | i64 |  | <p>The port that Global Accelerator uses to check the health of endpoints that are part of this endpoint group. The default port
	        is the listener port that this endpoint group is associated with. If listener port is a list of ports, Global Accelerator uses the
			first port in the list.</p> |
| `endpoint_configurations` | Vec<String> |  | <p>The list of endpoint objects.</p> |
| `endpoint_group_region` | String | ✅ | <p>The Amazon Web Services Region where the endpoint group is located. A listener can have only one endpoint group in a
			specific Region.</p> |
| `listener_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the listener.</p> |
| `health_check_interval_seconds` | i64 |  | <p>The time—10 seconds or 30 seconds—between each health check for an endpoint. The default value is 30.</p> |
| `health_check_protocol` | String |  | <p>The protocol that Global Accelerator uses to check the health of endpoints that are part of this endpoint group. The default
			value is TCP.</p> |
| `threshold_count` | i64 |  | <p>The number of consecutive health checks required to set the state of a healthy endpoint to unhealthy, or to set an
			unhealthy endpoint to healthy. The default value is 3.</p> |
| `idempotency_token` | String | ✅ | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the
			uniqueness—of the request.</p> |
| `health_check_path` | String |  | <p>If the protocol is HTTP/S, then this specifies the path that is the destination for health check targets. The
			default value is slash (/).</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoint_group` | String | <p>The description of an endpoint group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create endpoint_group
endpoint_group = provider.global_accelerator.Endpoint_group {
    endpoint_group_region = "value"  # <p>The Amazon Web Services Region where the endpoint group is located. A listener can have only one endpoint group in a
			specific Region.</p>
    listener_arn = "value"  # <p>The Amazon Resource Name (ARN) of the listener.</p>
    idempotency_token = "value"  # <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the
			uniqueness—of the request.</p>
}

# Access endpoint_group outputs
endpoint_group_id = endpoint_group.id
endpoint_group_endpoint_group = endpoint_group.endpoint_group
```

---


### Cross_account_attachment

CrossAccountAttachment resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the cross-account attachment. </p> |
| `idempotency_token` | String | ✅ | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the
			uniqueness—of the request.</p> |
| `tags` | Vec<String> |  | <p>Add tags for a cross-account attachment.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/tagging-in-global-accelerator.html">Tagging
			in Global Accelerator</a> in the <i>Global Accelerator Developer Guide</i>.</p> |
| `resources` | Vec<String> |  | <p>The Amazon Resource Names (ARNs) for the resources to include in the cross-account attachment. A resource can
			be any supported Amazon Web Services resource type for Global Accelerator or a CIDR range for a
			bring your own IP address (BYOIP) address pool. </p> |
| `principals` | Vec<String> |  | <p>The principals to include in the cross-account attachment. A principal can be an Amazon Web Services account
			number or the Amazon Resource Name (ARN) for an accelerator. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cross_account_attachment` | String | <p>Information about the cross-account attachment.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cross_account_attachment
cross_account_attachment = provider.global_accelerator.Cross_account_attachment {
    name = "value"  # <p>The name of the cross-account attachment. </p>
    idempotency_token = "value"  # <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the
			uniqueness—of the request.</p>
}

# Access cross_account_attachment outputs
cross_account_attachment_id = cross_account_attachment.id
cross_account_attachment_cross_account_attachment = cross_account_attachment.cross_account_attachment
```

---


### Custom_routing_listener

CustomRoutingListener resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `accelerator_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the accelerator for a custom routing listener.</p> |
| `port_ranges` | Vec<String> | ✅ | <p>The port range to support for connections from clients to your accelerator.</p>
         <p>Separately, you set port ranges for endpoints. For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/about-custom-routing-endpoints.html">About 
		endpoints for custom routing accelerators</a>.</p> |
| `idempotency_token` | String | ✅ | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the
		uniqueness—of the request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `listener` | String | <p>The description of a listener for a custom routing accelerator.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create custom_routing_listener
custom_routing_listener = provider.global_accelerator.Custom_routing_listener {
    accelerator_arn = "value"  # <p>The Amazon Resource Name (ARN) of the accelerator for a custom routing listener.</p>
    port_ranges = "value"  # <p>The port range to support for connections from clients to your accelerator.</p>
         <p>Separately, you set port ranges for endpoints. For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/about-custom-routing-endpoints.html">About 
		endpoints for custom routing accelerators</a>.</p>
    idempotency_token = "value"  # <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the
		uniqueness—of the request.</p>
}

# Access custom_routing_listener outputs
custom_routing_listener_id = custom_routing_listener.id
custom_routing_listener_listener = custom_routing_listener.listener
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple accelerator_attributes resources
accelerator_attributes_0 = provider.global_accelerator.Accelerator_attributes {
    accelerator_arn = "value-0"
}
accelerator_attributes_1 = provider.global_accelerator.Accelerator_attributes {
    accelerator_arn = "value-1"
}
accelerator_attributes_2 = provider.global_accelerator.Accelerator_attributes {
    accelerator_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    accelerator_attributes = provider.global_accelerator.Accelerator_attributes {
        accelerator_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Global_accelerator Documentation](https://docs.aws.amazon.com/global_accelerator/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
