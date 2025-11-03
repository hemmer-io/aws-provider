# Elastic_load_balancing Service



**Resources**: 37

---

## Overview

The elastic_load_balancing service provides access to 37 resource types:

- [Trust_store](#trust_store) [CD]
- [Trust_store_associations](#trust_store_associations) [R]
- [Trust_stores](#trust_stores) [R]
- [Listener_attributes](#listener_attributes) [R]
- [Shared_trust_store_association](#shared_trust_store_association) [D]
- [Capacity_reservation](#capacity_reservation) [R]
- [Listener_certificates](#listener_certificates) [R]
- [Load_balancers](#load_balancers) [R]
- [Ssl_policies](#ssl_policies) [R]
- [Load_balancer_attributes](#load_balancer_attributes) [R]
- [Rules](#rules) [R]
- [Load_balancer](#load_balancer) [CD]
- [Account_limits](#account_limits) [R]
- [Target_groups](#target_groups) [R]
- [Tags](#tags) [R]
- [Trust_store_revocations](#trust_store_revocations) [R]
- [Listeners](#listeners) [R]
- [Target_health](#target_health) [R]
- [Rule](#rule) [CD]
- [Target_group_attributes](#target_group_attributes) [R]
- [Trust_store_ca_certificates_bundle](#trust_store_ca_certificates_bundle) [R]
- [Target_group](#target_group) [CD]
- [Trust_store_revocation_content](#trust_store_revocation_content) [R]
- [Listener](#listener) [CD]
- [Resource_policy](#resource_policy) [R]
- [Load_balancer_policies](#load_balancer_policies) [R]
- [App_cookie_stickiness_policy](#app_cookie_stickiness_policy) [C]
- [Load_balancer_policy_types](#load_balancer_policy_types) [R]
- [Account_limits](#account_limits) [R]
- [Load_balancers](#load_balancers) [R]
- [Load_balancer](#load_balancer) [CD]
- [Load_balancer_attributes](#load_balancer_attributes) [R]
- [Instance_health](#instance_health) [R]
- [Load_balancer_listeners](#load_balancer_listeners) [CD]
- [Tags](#tags) [R]
- [Lb_cookie_stickiness_policy](#lb_cookie_stickiness_policy) [C]
- [Load_balancer_policy](#load_balancer_policy) [CD]

---

## Resources


### Trust_store

TrustStore resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ca_certificates_bundle_s3_key` | String | ✅ | <p>The Amazon S3 path for the ca certificates bundle.</p> |
| `name` | String | ✅ | <p>The name of the trust store.</p>
         <p>This name must be unique per region and can't be changed after creation.</p> |
| `ca_certificates_bundle_s3_object_version` | String |  | <p>The Amazon S3 object version for the ca certificates bundle. If undefined the current version is used.</p> |
| `tags` | Vec<String> |  | <p>The tags to assign to the trust store.</p> |
| `ca_certificates_bundle_s3_bucket` | String | ✅ | <p>The Amazon S3 bucket for the ca certificates bundle.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create trust_store
trust_store = provider.elastic_load_balancing.Trust_store {
    ca_certificates_bundle_s3_key = "value"  # <p>The Amazon S3 path for the ca certificates bundle.</p>
    name = "value"  # <p>The name of the trust store.</p>
         <p>This name must be unique per region and can't be changed after creation.</p>
    ca_certificates_bundle_s3_bucket = "value"  # <p>The Amazon S3 bucket for the ca certificates bundle.</p>
}

```

---


### Trust_store_associations

TrustStoreAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_marker` | String | <p>If there are additional results, this is the marker for the next set of results.
      Otherwise, this is null.</p> |
| `trust_store_associations` | Vec<String> | <p>Information about the resources the trust store is associated to.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access trust_store_associations outputs
trust_store_associations_id = trust_store_associations.id
trust_store_associations_next_marker = trust_store_associations.next_marker
trust_store_associations_trust_store_associations = trust_store_associations.trust_store_associations
```

---


### Trust_stores

TrustStores resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_marker` | String | <p>If there are additional results, this is the marker for the next set of results.
      Otherwise, this is null.</p> |
| `trust_stores` | Vec<String> | <p>Information about the trust stores.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access trust_stores outputs
trust_stores_id = trust_stores.id
trust_stores_next_marker = trust_stores.next_marker
trust_stores_trust_stores = trust_stores.trust_stores
```

---


### Listener_attributes

ListenerAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attributes` | Vec<String> | <p>Information about the listener attributes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access listener_attributes outputs
listener_attributes_id = listener_attributes.id
listener_attributes_attributes = listener_attributes.attributes
```

---


### Shared_trust_store_association

SharedTrustStoreAssociation resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



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


### Capacity_reservation

CapacityReservation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `capacity_reservation_state` | Vec<String> | <p>The state of the capacity reservation.</p> |
| `decrease_requests_remaining` | i64 | <p>The amount of daily capacity decreases remaining.</p> |
| `minimum_load_balancer_capacity` | String | <p>The requested minimum capacity reservation for the load balancer</p> |
| `last_modified_time` | String | <p>The last time the capacity reservation was modified.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access capacity_reservation outputs
capacity_reservation_id = capacity_reservation.id
capacity_reservation_capacity_reservation_state = capacity_reservation.capacity_reservation_state
capacity_reservation_decrease_requests_remaining = capacity_reservation.decrease_requests_remaining
capacity_reservation_minimum_load_balancer_capacity = capacity_reservation.minimum_load_balancer_capacity
capacity_reservation_last_modified_time = capacity_reservation.last_modified_time
```

---


### Listener_certificates

ListenerCertificates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `certificates` | Vec<String> | <p>Information about the certificates.</p> |
| `next_marker` | String | <p>If there are additional results, this is the marker for the next set of results.
      Otherwise, this is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access listener_certificates outputs
listener_certificates_id = listener_certificates.id
listener_certificates_certificates = listener_certificates.certificates
listener_certificates_next_marker = listener_certificates.next_marker
```

---


### Load_balancers

LoadBalancers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `load_balancers` | Vec<String> | <p>Information about the load balancers.</p> |
| `next_marker` | String | <p>If there are additional results, this is the marker for the next set of results.
      Otherwise, this is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access load_balancers outputs
load_balancers_id = load_balancers.id
load_balancers_load_balancers = load_balancers.load_balancers
load_balancers_next_marker = load_balancers.next_marker
```

---


### Ssl_policies

SSLPolicies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_marker` | String | <p>If there are additional results, this is the marker for the next set of results.
      Otherwise, this is null.</p> |
| `ssl_policies` | Vec<String> | <p>Information about the security policies.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ssl_policies outputs
ssl_policies_id = ssl_policies.id
ssl_policies_next_marker = ssl_policies.next_marker
ssl_policies_ssl_policies = ssl_policies.ssl_policies
```

---


### Load_balancer_attributes

LoadBalancerAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attributes` | Vec<String> | <p>Information about the load balancer attributes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access load_balancer_attributes outputs
load_balancer_attributes_id = load_balancer_attributes.id
load_balancer_attributes_attributes = load_balancer_attributes.attributes
```

---


### Rules

Rules resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rules` | Vec<String> | <p>Information about the rules.</p> |
| `next_marker` | String | <p>If there are additional results, this is the marker for the next set of results.
      Otherwise, this is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access rules outputs
rules_id = rules.id
rules_rules = rules.rules
rules_next_marker = rules.next_marker
```

---


### Load_balancer

LoadBalancer resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the load balancer.</p>
         <p>This name must be unique per region per account, can have a maximum of 32 characters, must
      contain only alphanumeric characters or hyphens, must not begin or end with a hyphen, and must
      not begin with "internal-".</p> |
| `subnets` | Vec<String> |  | <p>The IDs of the subnets. You can specify only one subnet per Availability Zone. You
      must specify either subnets or subnet mappings, but not both. To specify an Elastic IP
      address, specify subnet mappings instead of subnets.</p>
         <p>[Application Load Balancers] You must specify subnets from at least two Availability
      Zones.</p>
         <p>[Application Load Balancers on Outposts] You must specify one Outpost subnet.</p>
         <p>[Application Load Balancers on Local Zones] You can specify subnets from one or more Local
      Zones.</p>
         <p>[Network Load Balancers and Gateway Load Balancers] You can specify subnets from one or more 
      Availability Zones.</p> |
| `ip_address_type` | String |  | <p>The IP address type. Internal load balancers must use <code>ipv4</code>.</p>
         <p>[Application Load Balancers] The possible values are <code>ipv4</code> (IPv4 addresses), 
      <code>dualstack</code> (IPv4 and IPv6 addresses), and <code>dualstack-without-public-ipv4</code> 
      (public IPv6 addresses and private IPv4 and IPv6 addresses).</p>
         <p>[Network Load Balancers and Gateway Load Balancers] The possible values are <code>ipv4</code> 
      (IPv4 addresses) and <code>dualstack</code> (IPv4 and IPv6 addresses).</p> |
| `type` | String |  | <p>The type of load balancer. The default is <code>application</code>.</p> |
| `customer_owned_ipv4_pool` | String |  | <p>[Application Load Balancers on Outposts] The ID of the customer-owned address pool (CoIP
      pool).</p> |
| `enable_prefix_for_ipv6_source_nat` | String |  | <p>[Network Load Balancers with UDP listeners] Indicates whether to use an IPv6 prefix 
      from each subnet for source NAT. The IP address type must be <code>dualstack</code>. 
      The default value is <code>off</code>.</p> |
| `subnet_mappings` | Vec<String> |  | <p>The IDs of the subnets. You can specify only one subnet per Availability Zone. You
      must specify either subnets or subnet mappings, but not both.</p>
         <p>[Application Load Balancers] You must specify subnets from at least two Availability
      Zones. You can't specify Elastic IP addresses for your subnets.</p>
         <p>[Application Load Balancers on Outposts] You must specify one Outpost subnet.</p>
         <p>[Application Load Balancers on Local Zones] You can specify subnets from one or more Local
      Zones.</p>
         <p>[Network Load Balancers] You can specify subnets from one or more Availability Zones. You
      can specify one Elastic IP address per subnet if you need static IP addresses for your
      internet-facing load balancer. For internal load balancers, you can specify one private IP
      address per subnet from the IPv4 range of the subnet. For internet-facing load balancer, you
      can specify one IPv6 address per subnet.</p>
         <p>[Gateway Load Balancers] You can specify subnets from one or more Availability Zones. You
      can't specify Elastic IP addresses for your subnets.</p> |
| `security_groups` | Vec<String> |  | <p>[Application Load Balancers and Network Load Balancers] The IDs of the security groups for
      the load balancer.</p> |
| `scheme` | String |  | <p>The nodes of an Internet-facing load balancer have public IP addresses. The DNS name of an
      Internet-facing load balancer is publicly resolvable to the public IP addresses of the nodes.
      Therefore, Internet-facing load balancers can route requests from clients over the
      internet.</p>
         <p>The nodes of an internal load balancer have only private IP addresses. The DNS name of an
      internal load balancer is publicly resolvable to the private IP addresses of the nodes.
      Therefore, internal load balancers can route requests only from clients with access to the VPC
      for the load balancer.</p>
         <p>The default is an Internet-facing load balancer.</p>
         <p>You can't specify a scheme for a Gateway Load Balancer.</p> |
| `ipam_pools` | String |  | <p>[Application Load Balancers] The IPAM pools to use with the load balancer.</p> |
| `tags` | Vec<String> |  | <p>The tags to assign to the load balancer.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create load_balancer
load_balancer = provider.elastic_load_balancing.Load_balancer {
    name = "value"  # <p>The name of the load balancer.</p>
         <p>This name must be unique per region per account, can have a maximum of 32 characters, must
      contain only alphanumeric characters or hyphens, must not begin or end with a hyphen, and must
      not begin with "internal-".</p>
}

```

---


### Account_limits

AccountLimits resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_marker` | String | <p>If there are additional results, this is the marker for the next set of results.
      Otherwise, this is null.</p> |
| `limits` | Vec<String> | <p>Information about the limits.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_limits outputs
account_limits_id = account_limits.id
account_limits_next_marker = account_limits.next_marker
account_limits_limits = account_limits.limits
```

---


### Target_groups

TargetGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `target_groups` | Vec<String> | <p>Information about the target groups.</p> |
| `next_marker` | String | <p>If there are additional results, this is the marker for the next set of results.
      Otherwise, this is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access target_groups outputs
target_groups_id = target_groups.id
target_groups_target_groups = target_groups.target_groups
target_groups_next_marker = target_groups.next_marker
```

---


### Tags

Tags resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tag_descriptions` | Vec<String> | <p>Information about the tags.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access tags outputs
tags_id = tags.id
tags_tag_descriptions = tags.tag_descriptions
```

---


### Trust_store_revocations

TrustStoreRevocations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `trust_store_revocations` | Vec<String> | <p>Information about the revocation file in the trust store.</p> |
| `next_marker` | String | <p>If there are additional results, this is the marker for the next set of results.
      Otherwise, this is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access trust_store_revocations outputs
trust_store_revocations_id = trust_store_revocations.id
trust_store_revocations_trust_store_revocations = trust_store_revocations.trust_store_revocations
trust_store_revocations_next_marker = trust_store_revocations.next_marker
```

---


### Listeners

Listeners resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `listeners` | Vec<String> | <p>Information about the listeners.</p> |
| `next_marker` | String | <p>If there are additional results, this is the marker for the next set of results.
      Otherwise, this is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access listeners outputs
listeners_id = listeners.id
listeners_listeners = listeners.listeners
listeners_next_marker = listeners.next_marker
```

---


### Target_health

TargetHealth resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `target_health_descriptions` | Vec<String> | <p>Information about the health of the targets.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access target_health outputs
target_health_id = target_health.id
target_health_target_health_descriptions = target_health.target_health_descriptions
```

---


### Rule

Rule resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `priority` | i64 | ✅ | <p>The rule priority. A listener can't have multiple rules with the same priority.</p> |
| `tags` | Vec<String> |  | <p>The tags to assign to the rule.</p> |
| `transforms` | Vec<String> |  | <p>The transforms to apply to requests that match this rule. You can add one host header rewrite transform 
      and one URL rewrite transform.</p> |
| `actions` | Vec<String> | ✅ | <p>The actions.</p> |
| `listener_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the listener.</p> |
| `conditions` | Vec<String> | ✅ | <p>The conditions.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create rule
rule = provider.elastic_load_balancing.Rule {
    priority = "value"  # <p>The rule priority. A listener can't have multiple rules with the same priority.</p>
    actions = "value"  # <p>The actions.</p>
    listener_arn = "value"  # <p>The Amazon Resource Name (ARN) of the listener.</p>
    conditions = "value"  # <p>The conditions.</p>
}

```

---


### Target_group_attributes

TargetGroupAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attributes` | Vec<String> | <p>Information about the target group attributes</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access target_group_attributes outputs
target_group_attributes_id = target_group_attributes.id
target_group_attributes_attributes = target_group_attributes.attributes
```

---


### Trust_store_ca_certificates_bundle

TrustStoreCaCertificatesBundle resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location` | String | <p>The ca certificate bundles Amazon S3 URI.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access trust_store_ca_certificates_bundle outputs
trust_store_ca_certificates_bundle_id = trust_store_ca_certificates_bundle.id
trust_store_ca_certificates_bundle_location = trust_store_ca_certificates_bundle.location
```

---


### Target_group

TargetGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target_type` | String |  | <p>The type of target that you must specify when registering targets with this target group.
      You can't specify targets for a target group using more than one target type.</p>
         <ul>
            <li>
               <p>
                  <code>instance</code> - Register targets by instance ID. This is the default
          value.</p>
            </li>
            <li>
               <p>
                  <code>ip</code> - Register targets by IP address. You can specify IP addresses from
          the subnets of the virtual private cloud (VPC) for the target group, the RFC 1918 range
          (10.0.0.0/8, 172.16.0.0/12, and 192.168.0.0/16), and the RFC 6598 range (100.64.0.0/10).
          You can't specify publicly routable IP addresses.</p>
            </li>
            <li>
               <p>
                  <code>lambda</code> - Register a single Lambda function as a target.</p>
            </li>
            <li>
               <p>
                  <code>alb</code> - Register a single Application Load Balancer as a target.</p>
            </li>
         </ul> |
| `name` | String | ✅ | <p>The name of the target group.</p>
         <p>This name must be unique per region per account, can have a maximum of 32 characters, must
      contain only alphanumeric characters or hyphens, and must not begin or end with a
      hyphen.</p> |
| `port` | i64 |  | <p>The port on which the targets receive traffic. This port is used unless you specify a port
      override when registering the target. If the target is a Lambda function, this parameter does
      not apply. If the protocol is GENEVE, the supported port is 6081.</p> |
| `healthy_threshold_count` | i64 |  | <p>The number of consecutive health check successes required before considering a target healthy. The range is 
      2-10. If the target group protocol is TCP, TCP_UDP, UDP, TLS, HTTP or HTTPS, the default is 5. For target groups 
      with a protocol of GENEVE, the default is 5. If the target type 
      is <code>lambda</code>, the default is 5.</p> |
| `health_check_path` | String |  | <p>[HTTP/HTTPS health checks] The destination for health checks on the targets.</p>
         <p>[HTTP1 or HTTP2 protocol version] The ping path. The default is /.</p>
         <p>[GRPC protocol version] The path of a custom health check method with the format
      /package.service/method. The default is /Amazon Web Services.ALB/healthcheck.</p> |
| `vpc_id` | String |  | <p>The identifier of the virtual private cloud (VPC). If the target is a Lambda function,
      this parameter does not apply. Otherwise, this parameter is required.</p> |
| `health_check_port` | String |  | <p>The port the load balancer uses when performing health checks on targets. If the protocol
      is HTTP, HTTPS, TCP, TLS, UDP, or TCP_UDP, the default is <code>traffic-port</code>, which is
      the port on which each target receives traffic from the load balancer. If the protocol is
      GENEVE, the default is port 80.</p> |
| `health_check_interval_seconds` | i64 |  | <p>The approximate amount of time, in seconds, between health checks of an individual target. The range is 5-300.
      If the target group protocol is TCP, TLS, UDP, TCP_UDP, HTTP or HTTPS, the default is 30 seconds. 
      If the target group protocol is GENEVE, the default is 10 seconds. 
      If the target type is <code>lambda</code>, the default is 35 seconds.</p> |
| `unhealthy_threshold_count` | i64 |  | <p>The number of consecutive health check failures required before considering a target unhealthy. The range is 
      2-10. If the target group protocol is TCP, TCP_UDP, UDP, TLS, HTTP or HTTPS, the default is 2. For target groups 
      with a protocol of GENEVE, the default is 2. If the target type 
      is <code>lambda</code>, the default is 5.</p> |
| `ip_address_type` | String |  | <p>The IP address type. The default value is <code>ipv4</code>.</p> |
| `tags` | Vec<String> |  | <p>The tags to assign to the target group.</p> |
| `health_check_protocol` | String |  | <p>The protocol the load balancer uses when performing health checks on targets. For
      Application Load Balancers, the default is HTTP. For Network Load Balancers and Gateway Load
      Balancers, the default is TCP. The TCP protocol is not supported for health checks if the
      protocol of the target group is HTTP or HTTPS. The GENEVE, TLS, UDP, and TCP_UDP protocols are
      not supported for health checks.</p> |
| `matcher` | String |  | <p>[HTTP/HTTPS health checks] The HTTP or gRPC codes to use when checking for a successful 
      response from a target. For target groups with a protocol of TCP, TCP_UDP, UDP or TLS the range 
      is 200-599. For target groups with a protocol of HTTP or HTTPS, the range is 200-499. For target 
      groups with a protocol of GENEVE, the range is 200-399.</p> |
| `health_check_timeout_seconds` | i64 |  | <p>The amount of time, in seconds, during which no response from a target means a failed 
      health check. The range is 2–120 seconds. For target groups with a protocol of HTTP, the 
      default is 6 seconds. For target groups with a protocol of TCP, TLS or HTTPS, the default 
      is 10 seconds. For target groups with a protocol of GENEVE, the default is 5 seconds. If 
      the target type is <code>lambda</code>, the default is 30 seconds.</p> |
| `protocol` | String |  | <p>The protocol to use for routing traffic to the targets. For Application Load Balancers,
      the supported protocols are HTTP and HTTPS. For Network Load Balancers, the supported
      protocols are TCP, TLS, UDP, or TCP_UDP. For Gateway Load Balancers, the supported protocol is
      GENEVE. A TCP_UDP listener must be associated with a TCP_UDP target group. If the target is a
      Lambda function, this parameter does not apply.</p> |
| `protocol_version` | String |  | <p>[HTTP/HTTPS protocol] The protocol version. Specify <code>GRPC</code> to send requests to
      targets using gRPC. Specify <code>HTTP2</code> to send requests to targets using HTTP/2. The
      default is <code>HTTP1</code>, which sends requests to targets using HTTP/1.1.</p> |
| `health_check_enabled` | bool |  | <p>Indicates whether health checks are enabled. If the target type is <code>lambda</code>,
      health checks are disabled by default but can be enabled. If the target type is
        <code>instance</code>, <code>ip</code>, or <code>alb</code>, health checks are always
      enabled and can't be disabled.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create target_group
target_group = provider.elastic_load_balancing.Target_group {
    name = "value"  # <p>The name of the target group.</p>
         <p>This name must be unique per region per account, can have a maximum of 32 characters, must
      contain only alphanumeric characters or hyphens, and must not begin or end with a
      hyphen.</p>
}

```

---


### Trust_store_revocation_content

TrustStoreRevocationContent resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location` | String | <p>The revocation files Amazon S3 URI.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access trust_store_revocation_content outputs
trust_store_revocation_content_id = trust_store_revocation_content.id
trust_store_revocation_content_location = trust_store_revocation_content.location
```

---


### Listener

Listener resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `port` | i64 |  | <p>The port on which the load balancer is listening. You can't specify a port for a Gateway
      Load Balancer.</p> |
| `load_balancer_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the load balancer.</p> |
| `certificates` | Vec<String> |  | <p>[HTTPS and TLS listeners] The default certificate for the listener. You must provide
      exactly one certificate. Set <code>CertificateArn</code> to the certificate ARN but do not set
        <code>IsDefault</code>.</p> |
| `ssl_policy` | String |  | <p>[HTTPS and TLS listeners] The security policy that defines which protocols and ciphers are
      supported.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/describe-ssl-policies.html">Security policies</a> in the <i>Application Load Balancers Guide</i> and
        <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/describe-ssl-policies.html">Security policies</a> in the <i>Network Load Balancers Guide</i>.</p> |
| `default_actions` | Vec<String> | ✅ | <p>The actions for the default rule.</p> |
| `mutual_authentication` | String |  | <p>[HTTPS listeners] The mutual authentication configuration information.</p> |
| `tags` | Vec<String> |  | <p>The tags to assign to the listener.</p> |
| `alpn_policy` | Vec<String> |  | <p>[TLS listeners] The name of the Application-Layer Protocol Negotiation (ALPN) policy. You
      can specify one policy name. The following are the possible values:</p>
         <ul>
            <li>
               <p>
                  <code>HTTP1Only</code>
               </p>
            </li>
            <li>
               <p>
                  <code>HTTP2Only</code>
               </p>
            </li>
            <li>
               <p>
                  <code>HTTP2Optional</code>
               </p>
            </li>
            <li>
               <p>
                  <code>HTTP2Preferred</code>
               </p>
            </li>
            <li>
               <p>
                  <code>None</code>
               </p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-listeners.html#alpn-policies">ALPN
        policies</a> in the <i>Network Load Balancers Guide</i>.</p> |
| `protocol` | String |  | <p>The protocol for connections from clients to the load balancer. For Application Load
      Balancers, the supported protocols are HTTP and HTTPS. For Network Load Balancers, the
      supported protocols are TCP, TLS, UDP, and TCP_UDP. You can’t specify the UDP or TCP_UDP
      protocol if dual-stack mode is enabled. You can't specify a protocol for a Gateway Load
      Balancer.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create listener
listener = provider.elastic_load_balancing.Listener {
    load_balancer_arn = "value"  # <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    default_actions = "value"  # <p>The actions for the default rule.</p>
}

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
| `policy` | String | <p>The content of the resource policy.</p> |


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


### Load_balancer_policies

LoadBalancerPolicies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_descriptions` | Vec<String> | <p>Information about the policies.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access load_balancer_policies outputs
load_balancer_policies_id = load_balancer_policies.id
load_balancer_policies_policy_descriptions = load_balancer_policies.policy_descriptions
```

---


### App_cookie_stickiness_policy

AppCookieStickinessPolicy resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cookie_name` | String | ✅ | <p>The name of the application cookie used for stickiness.</p> |
| `load_balancer_name` | String | ✅ | <p>The name of the load balancer.</p> |
| `policy_name` | String | ✅ | <p>The name of the policy being created. Policy names must consist of alphanumeric characters and dashes (-). This name must be unique within the set of policies for this load balancer.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app_cookie_stickiness_policy
app_cookie_stickiness_policy = provider.elastic_load_balancing.App_cookie_stickiness_policy {
    cookie_name = "value"  # <p>The name of the application cookie used for stickiness.</p>
    load_balancer_name = "value"  # <p>The name of the load balancer.</p>
    policy_name = "value"  # <p>The name of the policy being created. Policy names must consist of alphanumeric characters and dashes (-). This name must be unique within the set of policies for this load balancer.</p>
}

```

---


### Load_balancer_policy_types

LoadBalancerPolicyTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_type_descriptions` | Vec<String> | <p>Information about the policy types.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access load_balancer_policy_types outputs
load_balancer_policy_types_id = load_balancer_policy_types.id
load_balancer_policy_types_policy_type_descriptions = load_balancer_policy_types.policy_type_descriptions
```

---


### Account_limits

AccountLimits resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `limits` | Vec<String> | <p>Information about the limits.</p> |
| `next_marker` | String | <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_limits outputs
account_limits_id = account_limits.id
account_limits_limits = account_limits.limits
account_limits_next_marker = account_limits.next_marker
```

---


### Load_balancers

LoadBalancers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `load_balancer_descriptions` | Vec<String> | <p>Information about the load balancers.</p> |
| `next_marker` | String | <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access load_balancers outputs
load_balancers_id = load_balancers.id
load_balancers_load_balancer_descriptions = load_balancers.load_balancer_descriptions
load_balancers_next_marker = load_balancers.next_marker
```

---


### Load_balancer

LoadBalancer resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `security_groups` | Vec<String> |  | <p>The IDs of the security groups to assign to the load balancer.</p> |
| `scheme` | String |  | <p>The type of a load balancer. Valid only for load balancers in a VPC.</p>          
        <p>By default, Elastic Load Balancing creates an Internet-facing load balancer with a DNS name that resolves to public IP addresses.
            For more information about Internet-facing and Internal load balancers, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/userguide/how-elastic-load-balancing-works.html#load-balancer-scheme">Load Balancer Scheme</a>
            in the <i>Elastic Load Balancing User Guide</i>.</p>
        <p>Specify <code>internal</code> to create a load balancer with a DNS name that resolves to private IP addresses.</p> |
| `listeners` | Vec<String> | ✅ | <p>The listeners.</p>
        <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-listener-config.html">Listeners for Your Classic Load Balancer</a>
            in the <i>Classic Load Balancers Guide</i>.</p> |
| `availability_zones` | Vec<String> |  | <p>One or more Availability Zones from the same region as the load balancer.</p>
        <p>You must specify at least one Availability Zone.</p>
        <p>You can add more Availability Zones after you create the load balancer using 
            <a>EnableAvailabilityZonesForLoadBalancer</a>.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to assign to the load balancer.</p>
        <p>For more information about tagging your load balancer, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/add-remove-tags.html">Tag Your Classic Load Balancer</a>
            in the <i>Classic Load Balancers Guide</i>.</p> |
| `subnets` | Vec<String> |  | <p>The IDs of the subnets in your VPC to attach to the load balancer.
            Specify one subnet per Availability Zone specified in <code>AvailabilityZones</code>.</p> |
| `load_balancer_name` | String | ✅ | <p>The name of the load balancer.</p>
        <p>This name must be unique within your set of load balancers for the region, must have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and cannot begin or end with a hyphen.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create load_balancer
load_balancer = provider.elastic_load_balancing.Load_balancer {
    listeners = "value"  # <p>The listeners.</p>
        <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-listener-config.html">Listeners for Your Classic Load Balancer</a>
            in the <i>Classic Load Balancers Guide</i>.</p>
    load_balancer_name = "value"  # <p>The name of the load balancer.</p>
        <p>This name must be unique within your set of load balancers for the region, must have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and cannot begin or end with a hyphen.</p>
}

```

---


### Load_balancer_attributes

LoadBalancerAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `load_balancer_attributes` | String | <p>Information about the load balancer attributes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access load_balancer_attributes outputs
load_balancer_attributes_id = load_balancer_attributes.id
load_balancer_attributes_load_balancer_attributes = load_balancer_attributes.load_balancer_attributes
```

---


### Instance_health

InstanceHealth resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_states` | Vec<String> | <p>Information about the health of the instances.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_health outputs
instance_health_id = instance_health.id
instance_health_instance_states = instance_health.instance_states
```

---


### Load_balancer_listeners

LoadBalancerListeners resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `listeners` | Vec<String> | ✅ | <p>The listeners.</p> |
| `load_balancer_name` | String | ✅ | <p>The name of the load balancer.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create load_balancer_listeners
load_balancer_listeners = provider.elastic_load_balancing.Load_balancer_listeners {
    listeners = "value"  # <p>The listeners.</p>
    load_balancer_name = "value"  # <p>The name of the load balancer.</p>
}

```

---


### Tags

Tags resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tag_descriptions` | Vec<String> | <p>Information about the tags.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access tags outputs
tags_id = tags.id
tags_tag_descriptions = tags.tag_descriptions
```

---


### Lb_cookie_stickiness_policy

LBCookieStickinessPolicy resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_name` | String | ✅ | <p>The name of the policy being created. Policy names must consist of alphanumeric characters and dashes (-). This name must be unique within the set of policies for this load balancer.</p> |
| `cookie_expiration_period` | i64 |  | <p>The time period, in seconds, after which the cookie should be considered stale. If you do not specify this parameter, the default value is 0, which indicates that the sticky session should last for the duration of the browser session.</p> |
| `load_balancer_name` | String | ✅ | <p>The name of the load balancer.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create lb_cookie_stickiness_policy
lb_cookie_stickiness_policy = provider.elastic_load_balancing.Lb_cookie_stickiness_policy {
    policy_name = "value"  # <p>The name of the policy being created. Policy names must consist of alphanumeric characters and dashes (-). This name must be unique within the set of policies for this load balancer.</p>
    load_balancer_name = "value"  # <p>The name of the load balancer.</p>
}

```

---


### Load_balancer_policy

LoadBalancerPolicy resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_type_name` | String | ✅ | <p>The name of the base policy type.
   	   To get the list of policy types, use <a>DescribeLoadBalancerPolicyTypes</a>.</p> |
| `policy_name` | String | ✅ | <p>The name of the load balancer policy to be created. This name must be unique within the set of policies for this load balancer.</p> |
| `load_balancer_name` | String | ✅ | <p>The name of the load balancer.</p> |
| `policy_attributes` | Vec<String> |  | <p>The policy attributes.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create load_balancer_policy
load_balancer_policy = provider.elastic_load_balancing.Load_balancer_policy {
    policy_type_name = "value"  # <p>The name of the base policy type.
   	   To get the list of policy types, use <a>DescribeLoadBalancerPolicyTypes</a>.</p>
    policy_name = "value"  # <p>The name of the load balancer policy to be created. This name must be unique within the set of policies for this load balancer.</p>
    load_balancer_name = "value"  # <p>The name of the load balancer.</p>
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

# Create multiple trust_store resources
trust_store_0 = provider.elastic_load_balancing.Trust_store {
    ca_certificates_bundle_s3_key = "value-0"
    name = "value-0"
    ca_certificates_bundle_s3_bucket = "value-0"
}
trust_store_1 = provider.elastic_load_balancing.Trust_store {
    ca_certificates_bundle_s3_key = "value-1"
    name = "value-1"
    ca_certificates_bundle_s3_bucket = "value-1"
}
trust_store_2 = provider.elastic_load_balancing.Trust_store {
    ca_certificates_bundle_s3_key = "value-2"
    name = "value-2"
    ca_certificates_bundle_s3_bucket = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    trust_store = provider.elastic_load_balancing.Trust_store {
        ca_certificates_bundle_s3_key = "production-value"
        name = "production-value"
        ca_certificates_bundle_s3_bucket = "production-value"
    }
```

---

## Related Documentation

- [AWS Elastic_load_balancing Documentation](https://docs.aws.amazon.com/elastic_load_balancing/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
