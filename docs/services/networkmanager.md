# Networkmanager Service



**Resources**: 37

---

## Overview

The networkmanager service provides access to 37 resource types:

- [Sites](#sites) [R]
- [Core_network_policy](#core_network_policy) [CR]
- [Customer_gateway_associations](#customer_gateway_associations) [R]
- [Link](#link) [CUD]
- [Network_resource_metadata](#network_resource_metadata) [U]
- [Core_network_policy_version](#core_network_policy_version) [D]
- [Global_network](#global_network) [CUD]
- [Transit_gateway_route_table_attachment](#transit_gateway_route_table_attachment) [CR]
- [Peering](#peering) [D]
- [Connect_peer_associations](#connect_peer_associations) [R]
- [Resource_policy](#resource_policy) [CRD]
- [Network_resource_counts](#network_resource_counts) [R]
- [Transit_gateway_connect_peer_associations](#transit_gateway_connect_peer_associations) [R]
- [Connect_peer](#connect_peer) [CRD]
- [Vpc_attachment](#vpc_attachment) [CRU]
- [Global_networks](#global_networks) [R]
- [Devices](#devices) [R]
- [Link_associations](#link_associations) [R]
- [Device](#device) [CUD]
- [Connect_attachment](#connect_attachment) [CR]
- [Site_to_site_vpn_attachment](#site_to_site_vpn_attachment) [CR]
- [Core_network_change_events](#core_network_change_events) [R]
- [Links](#links) [R]
- [Network_routes](#network_routes) [R]
- [Connection](#connection) [CUD]
- [Direct_connect_gateway_attachment](#direct_connect_gateway_attachment) [CRU]
- [Connections](#connections) [R]
- [Core_network_change_set](#core_network_change_set) [R]
- [Attachment](#attachment) [D]
- [Network_resources](#network_resources) [R]
- [Route_analysis](#route_analysis) [R]
- [Core_network](#core_network) [CRUD]
- [Network_telemetry](#network_telemetry) [R]
- [Site](#site) [CUD]
- [Transit_gateway_peering](#transit_gateway_peering) [CR]
- [Network_resource_relationships](#network_resource_relationships) [R]
- [Transit_gateway_registrations](#transit_gateway_registrations) [R]

---

## Resources


### Sites

Sites resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token for the next page of results.</p> |
| `sites` | Vec<String> | <p>The sites.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sites outputs
sites_id = sites.id
sites_next_token = sites.next_token
sites_sites = sites.sites
```

---


### Core_network_policy

CoreNetworkPolicy resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `latest_version_id` | i64 |  | <p>The ID of a core network policy. </p> |
| `core_network_id` | String | ✅ | <p>The ID of a core network.</p> |
| `client_token` | String |  | <p>The client token associated with the request.</p> |
| `policy_document` | String | ✅ | <p>The policy document.</p> |
| `description` | String |  | <p>a core network policy description.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `core_network_policy` | String | <p>The details about a core network policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create core_network_policy
core_network_policy = provider.networkmanager.Core_network_policy {
    core_network_id = "value"  # <p>The ID of a core network.</p>
    policy_document = "value"  # <p>The policy document.</p>
}

# Access core_network_policy outputs
core_network_policy_id = core_network_policy.id
core_network_policy_core_network_policy = core_network_policy.core_network_policy
```

---


### Customer_gateway_associations

CustomerGatewayAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `customer_gateway_associations` | Vec<String> | <p>The customer gateway associations.</p> |
| `next_token` | String | <p>The token for the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access customer_gateway_associations outputs
customer_gateway_associations_id = customer_gateway_associations.id
customer_gateway_associations_customer_gateway_associations = customer_gateway_associations.customer_gateway_associations
customer_gateway_associations_next_token = customer_gateway_associations.next_token
```

---


### Link

Link resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `site_id` | String | ✅ | <p>The ID of the site.</p> |
| `global_network_id` | String | ✅ | <p>The ID of the global network.</p> |
| `tags` | Vec<String> |  | <p>The tags to apply to the resource during creation.</p> |
| `description` | String |  | <p>A description of the link.</p>
         <p>Constraints: Maximum length of 256 characters.</p> |
| `bandwidth` | String | ✅ | <p> The upload speed and download speed in Mbps. </p> |
| `provider` | String |  | <p>The provider of the link.</p>
         <p>Constraints: Maximum length of 128 characters. Cannot include the following characters: | \ ^</p> |
| `type` | String |  | <p>The type of the link.</p>
         <p>Constraints: Maximum length of 128 characters. Cannot include the following characters: | \ ^</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create link
link = provider.networkmanager.Link {
    site_id = "value"  # <p>The ID of the site.</p>
    global_network_id = "value"  # <p>The ID of the global network.</p>
    bandwidth = "value"  # <p> The upload speed and download speed in Mbps. </p>
}

```

---


### Network_resource_metadata

NetworkResourceMetadata resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `global_network_id` | String | ✅ | <p>The ID of the global network.</p> |
| `resource_arn` | String | ✅ | <p>The ARN of the resource.</p> |
| `metadata` | HashMap<String, String> | ✅ | <p>The resource metadata.</p> |



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


### Core_network_policy_version

CoreNetworkPolicyVersion resource

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


### Global_network

GlobalNetwork resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The tags to apply to the resource during creation.</p> |
| `description` | String |  | <p>A description of the global network.</p>
         <p>Constraints: Maximum length of 256 characters.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create global_network
global_network = provider.networkmanager.Global_network {
}

```

---


### Transit_gateway_route_table_attachment

TransitGatewayRouteTableAttachment resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>The client token associated with the request.</p> |
| `transit_gateway_route_table_arn` | String | ✅ | <p>The ARN of the transit gateway route table for the attachment request. For example, <code>"TransitGatewayRouteTableArn": "arn:aws:ec2:us-west-2:123456789012:transit-gateway-route-table/tgw-rtb-9876543210123456"</code>.</p> |
| `tags` | Vec<String> |  | <p>The list of key-value tags associated with the request.</p> |
| `peering_id` | String | ✅ | <p>The ID of the peer for the </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transit_gateway_route_table_attachment` | String | <p>Returns information about the transit gateway route table attachment.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create transit_gateway_route_table_attachment
transit_gateway_route_table_attachment = provider.networkmanager.Transit_gateway_route_table_attachment {
    transit_gateway_route_table_arn = "value"  # <p>The ARN of the transit gateway route table for the attachment request. For example, <code>"TransitGatewayRouteTableArn": "arn:aws:ec2:us-west-2:123456789012:transit-gateway-route-table/tgw-rtb-9876543210123456"</code>.</p>
    peering_id = "value"  # <p>The ID of the peer for the </p>
}

# Access transit_gateway_route_table_attachment outputs
transit_gateway_route_table_attachment_id = transit_gateway_route_table_attachment.id
transit_gateway_route_table_attachment_transit_gateway_route_table_attachment = transit_gateway_route_table_attachment.transit_gateway_route_table_attachment
```

---


### Peering

Peering resource

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


### Connect_peer_associations

ConnectPeerAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connect_peer_associations` | Vec<String> | <p>Displays a list of Connect peer associations.</p> |
| `next_token` | String | <p>The token for the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connect_peer_associations outputs
connect_peer_associations_id = connect_peer_associations.id
connect_peer_associations_connect_peer_associations = connect_peer_associations.connect_peer_associations
connect_peer_associations_next_token = connect_peer_associations.next_token
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_document` | String | ✅ | <p>The JSON resource policy document.</p> |
| `resource_arn` | String | ✅ | <p>The ARN of the resource policy. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_document` | String | <p>The resource policy document.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.networkmanager.Resource_policy {
    policy_document = "value"  # <p>The JSON resource policy document.</p>
    resource_arn = "value"  # <p>The ARN of the resource policy. </p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_policy_document = resource_policy.policy_document
```

---


### Network_resource_counts

NetworkResourceCounts resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `network_resource_counts` | Vec<String> | <p>The count of resources.</p> |
| `next_token` | String | <p>The token for the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access network_resource_counts outputs
network_resource_counts_id = network_resource_counts.id
network_resource_counts_network_resource_counts = network_resource_counts.network_resource_counts
network_resource_counts_next_token = network_resource_counts.next_token
```

---


### Transit_gateway_connect_peer_associations

TransitGatewayConnectPeerAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transit_gateway_connect_peer_associations` | Vec<String> | <p>Information about the transit gateway Connect peer associations.</p> |
| `next_token` | String | <p>The token to use for the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateway_connect_peer_associations outputs
transit_gateway_connect_peer_associations_id = transit_gateway_connect_peer_associations.id
transit_gateway_connect_peer_associations_transit_gateway_connect_peer_associations = transit_gateway_connect_peer_associations.transit_gateway_connect_peer_associations
transit_gateway_connect_peer_associations_next_token = transit_gateway_connect_peer_associations.next_token
```

---


### Connect_peer

ConnectPeer resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The tags associated with the peer request.</p> |
| `connect_attachment_id` | String | ✅ | <p>The ID of the connection attachment.</p> |
| `peer_address` | String | ✅ | <p>The Connect peer address.</p> |
| `subnet_arn` | String |  | <p>The subnet ARN for the Connect peer. This only applies only when the protocol is NO_ENCAP.</p> |
| `client_token` | String |  | <p>The client token associated with the request.</p> |
| `inside_cidr_blocks` | Vec<String> |  | <p>The inside IP addresses used for BGP peering.</p> |
| `core_network_address` | String |  | <p>A Connect peer core network address. This only applies only when the protocol is <code>GRE</code>.</p> |
| `bgp_options` | String |  | <p>The Connect peer BGP options. This only applies only when the protocol is <code>GRE</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connect_peer` | String | <p>Returns information about a core network Connect peer.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connect_peer
connect_peer = provider.networkmanager.Connect_peer {
    connect_attachment_id = "value"  # <p>The ID of the connection attachment.</p>
    peer_address = "value"  # <p>The Connect peer address.</p>
}

# Access connect_peer outputs
connect_peer_id = connect_peer.id
connect_peer_connect_peer = connect_peer.connect_peer
```

---


### Vpc_attachment

VpcAttachment resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `core_network_id` | String | ✅ | <p>The ID of a core network for the VPC attachment.</p> |
| `options` | String |  | <p>Options for the VPC attachment.</p> |
| `subnet_arns` | Vec<String> | ✅ | <p>The subnet ARN of the VPC attachment.</p> |
| `vpc_arn` | String | ✅ | <p>The ARN of the VPC.</p> |
| `tags` | Vec<String> |  | <p>The key-value tags associated with the request.</p> |
| `client_token` | String |  | <p>The client token associated with the request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpc_attachment` | String | <p>Returns details about a VPC attachment.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_attachment
vpc_attachment = provider.networkmanager.Vpc_attachment {
    core_network_id = "value"  # <p>The ID of a core network for the VPC attachment.</p>
    subnet_arns = "value"  # <p>The subnet ARN of the VPC attachment.</p>
    vpc_arn = "value"  # <p>The ARN of the VPC.</p>
}

# Access vpc_attachment outputs
vpc_attachment_id = vpc_attachment.id
vpc_attachment_vpc_attachment = vpc_attachment.vpc_attachment
```

---


### Global_networks

GlobalNetworks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `global_networks` | Vec<String> | <p>Information about the global networks.</p> |
| `next_token` | String | <p>The token for the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access global_networks outputs
global_networks_id = global_networks.id
global_networks_global_networks = global_networks.global_networks
global_networks_next_token = global_networks.next_token
```

---


### Devices

Devices resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `devices` | Vec<String> | <p>The devices.</p> |
| `next_token` | String | <p>The token for the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access devices outputs
devices_id = devices.id
devices_devices = devices.devices
devices_next_token = devices.next_token
```

---


### Link_associations

LinkAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `link_associations` | Vec<String> | <p>The link associations.</p> |
| `next_token` | String | <p>The token for the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access link_associations outputs
link_associations_id = link_associations.id
link_associations_link_associations = link_associations.link_associations
link_associations_next_token = link_associations.next_token
```

---


### Device

Device resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vendor` | String |  | <p>The vendor of the device.</p>
         <p>Constraints: Maximum length of 128 characters.</p> |
| `type` | String |  | <p>The type of the device.</p> |
| `site_id` | String |  | <p>The ID of the site.</p> |
| `description` | String |  | <p>A description of the device.</p>
         <p>Constraints: Maximum length of 256 characters.</p> |
| `aws_location` | String |  | <p>The Amazon Web Services location of the device, if applicable. For an on-premises device, you can omit this parameter.</p> |
| `location` | String |  | <p>The location of the device.</p> |
| `global_network_id` | String | ✅ | <p>The ID of the global network.</p> |
| `serial_number` | String |  | <p>The serial number of the device.</p>
         <p>Constraints: Maximum length of 128 characters.</p> |
| `tags` | Vec<String> |  | <p>The tags to apply to the resource during creation.</p> |
| `model` | String |  | <p>The model of the device.</p>
         <p>Constraints: Maximum length of 128 characters.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create device
device = provider.networkmanager.Device {
    global_network_id = "value"  # <p>The ID of the global network.</p>
}

```

---


### Connect_attachment

ConnectAttachment resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>The client token associated with the request.</p> |
| `transport_attachment_id` | String | ✅ | <p>The ID of the attachment between the two connections.</p> |
| `tags` | Vec<String> |  | <p>The list of key-value tags associated with the request.</p> |
| `core_network_id` | String | ✅ | <p>The ID of a core network where you want to create the attachment. </p> |
| `edge_location` | String | ✅ | <p>The Region where the edge is located.</p> |
| `options` | String | ✅ | <p>Options for creating an attachment.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connect_attachment` | String | <p>Details about the Connect attachment.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connect_attachment
connect_attachment = provider.networkmanager.Connect_attachment {
    transport_attachment_id = "value"  # <p>The ID of the attachment between the two connections.</p>
    core_network_id = "value"  # <p>The ID of a core network where you want to create the attachment. </p>
    edge_location = "value"  # <p>The Region where the edge is located.</p>
    options = "value"  # <p>Options for creating an attachment.</p>
}

# Access connect_attachment outputs
connect_attachment_id = connect_attachment.id
connect_attachment_connect_attachment = connect_attachment.connect_attachment
```

---


### Site_to_site_vpn_attachment

SiteToSiteVpnAttachment resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `core_network_id` | String | ✅ | <p>The ID of a core network where you're creating a site-to-site VPN attachment.</p> |
| `tags` | Vec<String> |  | <p>The tags associated with the request.</p> |
| `client_token` | String |  | <p>The client token associated with the request.</p> |
| `vpn_connection_arn` | String | ✅ | <p>The ARN identifying the VPN attachment.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `site_to_site_vpn_attachment` | String | <p>Describes the site-to-site attachment.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create site_to_site_vpn_attachment
site_to_site_vpn_attachment = provider.networkmanager.Site_to_site_vpn_attachment {
    core_network_id = "value"  # <p>The ID of a core network where you're creating a site-to-site VPN attachment.</p>
    vpn_connection_arn = "value"  # <p>The ARN identifying the VPN attachment.</p>
}

# Access site_to_site_vpn_attachment outputs
site_to_site_vpn_attachment_id = site_to_site_vpn_attachment.id
site_to_site_vpn_attachment_site_to_site_vpn_attachment = site_to_site_vpn_attachment.site_to_site_vpn_attachment
```

---


### Core_network_change_events

CoreNetworkChangeEvents resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `core_network_change_events` | Vec<String> | <p>The response to <code>GetCoreNetworkChangeEventsRequest</code>.</p> |
| `next_token` | String | <p>The token for the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access core_network_change_events outputs
core_network_change_events_id = core_network_change_events.id
core_network_change_events_core_network_change_events = core_network_change_events.core_network_change_events
core_network_change_events_next_token = core_network_change_events.next_token
```

---


### Links

Links resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token for the next page of results.</p> |
| `links` | Vec<String> | <p>The links.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access links outputs
links_id = links.id
links_next_token = links.next_token
links_links = links.links
```

---


### Network_routes

NetworkRoutes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `route_table_timestamp` | String | <p>The route table creation time.</p> |
| `core_network_segment_edge` | String | <p>Describes a core network segment edge.</p> |
| `network_routes` | Vec<String> | <p>The network routes.</p> |
| `route_table_arn` | String | <p>The ARN of the route table.</p> |
| `route_table_type` | String | <p>The route table type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access network_routes outputs
network_routes_id = network_routes.id
network_routes_route_table_timestamp = network_routes.route_table_timestamp
network_routes_core_network_segment_edge = network_routes.core_network_segment_edge
network_routes_network_routes = network_routes.network_routes
network_routes_route_table_arn = network_routes.route_table_arn
network_routes_route_table_type = network_routes.route_table_type
```

---


### Connection

Connection resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `device_id` | String | ✅ | <p>The ID of the first device in the connection.</p> |
| `tags` | Vec<String> |  | <p>The tags to apply to the resource during creation.</p> |
| `global_network_id` | String | ✅ | <p>The ID of the global network.</p> |
| `connected_device_id` | String | ✅ | <p>The ID of the second device in the connection.</p> |
| `connected_link_id` | String |  | <p>The ID of the link for the second device.</p> |
| `description` | String |  | <p>A description of the connection.</p>
         <p>Length Constraints: Maximum length of 256 characters.</p> |
| `link_id` | String |  | <p>The ID of the link for the first device.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connection
connection = provider.networkmanager.Connection {
    device_id = "value"  # <p>The ID of the first device in the connection.</p>
    global_network_id = "value"  # <p>The ID of the global network.</p>
    connected_device_id = "value"  # <p>The ID of the second device in the connection.</p>
}

```

---


### Direct_connect_gateway_attachment

DirectConnectGatewayAttachment resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `edge_locations` | Vec<String> | ✅ | <p>One or more core network edge locations that the Direct Connect gateway attachment is associated with. </p> |
| `core_network_id` | String | ✅ | <p>The ID of the Cloud WAN core network that the Direct Connect gateway attachment should be attached to.</p> |
| `direct_connect_gateway_arn` | String | ✅ | <p>The ARN of the Direct Connect gateway attachment.</p> |
| `client_token` | String |  | <p>client token</p> |
| `tags` | Vec<String> |  | <p>The key value tags to apply to the Direct Connect gateway attachment during creation.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `direct_connect_gateway_attachment` | String | <p>Shows details about the Direct Connect gateway attachment. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create direct_connect_gateway_attachment
direct_connect_gateway_attachment = provider.networkmanager.Direct_connect_gateway_attachment {
    edge_locations = "value"  # <p>One or more core network edge locations that the Direct Connect gateway attachment is associated with. </p>
    core_network_id = "value"  # <p>The ID of the Cloud WAN core network that the Direct Connect gateway attachment should be attached to.</p>
    direct_connect_gateway_arn = "value"  # <p>The ARN of the Direct Connect gateway attachment.</p>
}

# Access direct_connect_gateway_attachment outputs
direct_connect_gateway_attachment_id = direct_connect_gateway_attachment.id
direct_connect_gateway_attachment_direct_connect_gateway_attachment = direct_connect_gateway_attachment.direct_connect_gateway_attachment
```

---


### Connections

Connections resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connections` | Vec<String> | <p>Information about the connections.</p> |
| `next_token` | String | <p>The token to use for the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connections outputs
connections_id = connections.id
connections_connections = connections.connections
connections_next_token = connections.next_token
```

---


### Core_network_change_set

CoreNetworkChangeSet resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `core_network_changes` | Vec<String> | <p>Describes a core network changes.</p> |
| `next_token` | String | <p>The token for the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access core_network_change_set outputs
core_network_change_set_id = core_network_change_set.id
core_network_change_set_core_network_changes = core_network_change_set.core_network_changes
core_network_change_set_next_token = core_network_change_set.next_token
```

---


### Attachment

Attachment resource

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


### Network_resources

NetworkResources resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `network_resources` | Vec<String> | <p>The network resources.</p> |
| `next_token` | String | <p>The token for the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access network_resources outputs
network_resources_id = network_resources.id
network_resources_network_resources = network_resources.network_resources
network_resources_next_token = network_resources.next_token
```

---


### Route_analysis

RouteAnalysis resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `route_analysis` | String | <p>The route analysis.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access route_analysis outputs
route_analysis_id = route_analysis.id
route_analysis_route_analysis = route_analysis.route_analysis
```

---


### Core_network

CoreNetwork resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `global_network_id` | String | ✅ | <p>The ID of the global network that a core network will be a part of. </p> |
| `tags` | Vec<String> |  | <p>Key-value tags associated with a core network request.</p> |
| `client_token` | String |  | <p>The client token associated with a core network request.</p> |
| `description` | String |  | <p>The description of a core network.</p> |
| `policy_document` | String |  | <p>The policy document for creating a core network.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `core_network` | String | <p>Details about a core network.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create core_network
core_network = provider.networkmanager.Core_network {
    global_network_id = "value"  # <p>The ID of the global network that a core network will be a part of. </p>
}

# Access core_network outputs
core_network_id = core_network.id
core_network_core_network = core_network.core_network
```

---


### Network_telemetry

NetworkTelemetry resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token for the next page of results.</p> |
| `network_telemetry` | Vec<String> | <p>The network telemetry.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access network_telemetry outputs
network_telemetry_id = network_telemetry.id
network_telemetry_next_token = network_telemetry.next_token
network_telemetry_network_telemetry = network_telemetry.network_telemetry
```

---


### Site

Site resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `location` | String |  | <p>The site location. This information is used for visualization in the Network Manager console. If you specify the address, the latitude and longitude are automatically calculated.</p>
         <ul>
            <li>
               <p>
                  <code>Address</code>: The physical address of the site.</p>
            </li>
            <li>
               <p>
                  <code>Latitude</code>: The latitude of the site. </p>
            </li>
            <li>
               <p>
                  <code>Longitude</code>: The longitude of the site.</p>
            </li>
         </ul> |
| `tags` | Vec<String> |  | <p>The tags to apply to the resource during creation.</p> |
| `global_network_id` | String | ✅ | <p>The ID of the global network.</p> |
| `description` | String |  | <p>A description of your site.</p>
         <p>Constraints: Maximum length of 256 characters.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create site
site = provider.networkmanager.Site {
    global_network_id = "value"  # <p>The ID of the global network.</p>
}

```

---


### Transit_gateway_peering

TransitGatewayPeering resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `core_network_id` | String | ✅ | <p>The ID of a core network.</p> |
| `tags` | Vec<String> |  | <p>The list of key-value tags associated with the request.</p> |
| `transit_gateway_arn` | String | ✅ | <p>The ARN of the transit gateway for the peering request.</p> |
| `client_token` | String |  | <p>The client token associated with the request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transit_gateway_peering` | String | <p>Returns information about a transit gateway peering. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create transit_gateway_peering
transit_gateway_peering = provider.networkmanager.Transit_gateway_peering {
    core_network_id = "value"  # <p>The ID of a core network.</p>
    transit_gateway_arn = "value"  # <p>The ARN of the transit gateway for the peering request.</p>
}

# Access transit_gateway_peering outputs
transit_gateway_peering_id = transit_gateway_peering.id
transit_gateway_peering_transit_gateway_peering = transit_gateway_peering.transit_gateway_peering
```

---


### Network_resource_relationships

NetworkResourceRelationships resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token for the next page of results.</p> |
| `relationships` | Vec<String> | <p>The resource relationships.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access network_resource_relationships outputs
network_resource_relationships_id = network_resource_relationships.id
network_resource_relationships_next_token = network_resource_relationships.next_token
network_resource_relationships_relationships = network_resource_relationships.relationships
```

---


### Transit_gateway_registrations

TransitGatewayRegistrations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transit_gateway_registrations` | Vec<String> | <p>The transit gateway registrations.</p> |
| `next_token` | String | <p>The token for the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateway_registrations outputs
transit_gateway_registrations_id = transit_gateway_registrations.id
transit_gateway_registrations_transit_gateway_registrations = transit_gateway_registrations.transit_gateway_registrations
transit_gateway_registrations_next_token = transit_gateway_registrations.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple sites resources
sites_0 = provider.networkmanager.Sites {
}
sites_1 = provider.networkmanager.Sites {
}
sites_2 = provider.networkmanager.Sites {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    sites = provider.networkmanager.Sites {
    }
```

---

## Related Documentation

- [AWS Networkmanager Documentation](https://docs.aws.amazon.com/networkmanager/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
