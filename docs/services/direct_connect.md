# Direct_connect Service



**Resources**: 30

---

## Overview

The direct_connect service provides access to 30 resource types:

- [Transit_virtual_interface](#transit_virtual_interface) [C]
- [Direct_connect_gateway_associations](#direct_connect_gateway_associations) [R]
- [Router_configuration](#router_configuration) [R]
- [Interconnect](#interconnect) [CD]
- [Lag](#lag) [CUD]
- [Connections](#connections) [R]
- [Direct_connect_gateway_association_proposals](#direct_connect_gateway_association_proposals) [R]
- [Lags](#lags) [R]
- [Virtual_gateways](#virtual_gateways) [R]
- [Connection](#connection) [CUD]
- [Virtual_interfaces](#virtual_interfaces) [R]
- [Hosted_connections](#hosted_connections) [R]
- [Tags](#tags) [R]
- [Direct_connect_gateway_attachments](#direct_connect_gateway_attachments) [R]
- [Customer_metadata](#customer_metadata) [R]
- [Public_virtual_interface](#public_virtual_interface) [C]
- [Bgp_peer](#bgp_peer) [CD]
- [Virtual_interface_attributes](#virtual_interface_attributes) [U]
- [Connections_on_interconnect](#connections_on_interconnect) [R]
- [Virtual_interface](#virtual_interface) [D]
- [Interconnect_loa](#interconnect_loa) [R]
- [Loa](#loa) [R]
- [Private_virtual_interface](#private_virtual_interface) [C]
- [Locations](#locations) [R]
- [Direct_connect_gateway](#direct_connect_gateway) [CUD]
- [Interconnects](#interconnects) [R]
- [Connection_loa](#connection_loa) [R]
- [Direct_connect_gateways](#direct_connect_gateways) [R]
- [Direct_connect_gateway_association_proposal](#direct_connect_gateway_association_proposal) [CD]
- [Direct_connect_gateway_association](#direct_connect_gateway_association) [CUD]

---

## Resources


### Transit_virtual_interface

TransitVirtualInterface resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `new_transit_virtual_interface` | String | ✅ | <p>Information about the transit virtual interface.</p> |
| `connection_id` | String | ✅ | <p>The ID of the connection.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create transit_virtual_interface
transit_virtual_interface = provider.direct_connect.Transit_virtual_interface {
    new_transit_virtual_interface = "value"  # <p>Information about the transit virtual interface.</p>
    connection_id = "value"  # <p>The ID of the connection.</p>
}

```

---


### Direct_connect_gateway_associations

DirectConnectGatewayAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to retrieve the next page.</p> |
| `direct_connect_gateway_associations` | Vec<String> | <p>Information about the associations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access direct_connect_gateway_associations outputs
direct_connect_gateway_associations_id = direct_connect_gateway_associations.id
direct_connect_gateway_associations_next_token = direct_connect_gateway_associations.next_token
direct_connect_gateway_associations_direct_connect_gateway_associations = direct_connect_gateway_associations.direct_connect_gateway_associations
```

---


### Router_configuration

RouterConfiguration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `router` | String | <p>The details about the router.</p> |
| `virtual_interface_name` | String | <p>Provides the details about a virtual interface's router.</p> |
| `virtual_interface_id` | String | <p>The ID assigned to the virtual interface.</p> |
| `customer_router_config` | String | <p>The customer router configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access router_configuration outputs
router_configuration_id = router_configuration.id
router_configuration_router = router_configuration.router
router_configuration_virtual_interface_name = router_configuration.virtual_interface_name
router_configuration_virtual_interface_id = router_configuration.virtual_interface_id
router_configuration_customer_router_config = router_configuration.customer_router_config
```

---


### Interconnect

Interconnect resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bandwidth` | String | ✅ | <p>The port bandwidth, in Gbps. The possible values are 1, 10, and 100.</p> |
| `request_mac_sec` | bool |  | <p>Indicates whether you want the interconnect to support MAC Security (MACsec).</p> |
| `lag_id` | String |  | <p>The ID of the LAG.</p> |
| `location` | String | ✅ | <p>The location of the interconnect.</p> |
| `provider_name` | String |  | <p>The name of the service provider associated with the interconnect.</p> |
| `tags` | Vec<String> |  | <p>The tags to associate with the interconnect.</p> |
| `interconnect_name` | String | ✅ | <p>The name of the interconnect.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create interconnect
interconnect = provider.direct_connect.Interconnect {
    bandwidth = "value"  # <p>The port bandwidth, in Gbps. The possible values are 1, 10, and 100.</p>
    location = "value"  # <p>The location of the interconnect.</p>
    interconnect_name = "value"  # <p>The name of the interconnect.</p>
}

```

---


### Lag

Lag resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `lag_name` | String | ✅ | <p>The name of the LAG.</p> |
| `connection_id` | String |  | <p>The ID of an existing dedicated connection to migrate to the LAG.</p> |
| `tags` | Vec<String> |  | <p>The tags to associate with the LAG.</p> |
| `child_connection_tags` | Vec<String> |  | <p>The tags to associate with the automtically created LAGs.</p> |
| `number_of_connections` | i64 | ✅ | <p>The number of physical dedicated connections initially provisioned and bundled by the LAG.
      You can have a maximum of four connections when the port speed is 1Gbps or 10Gbps, or two when
      the port speed is 100Gbps or 400Gbps.</p> |
| `provider_name` | String |  | <p>The name of the service provider associated with the LAG.</p> |
| `request_mac_sec` | bool |  | <p>Indicates whether the connection will support MAC Security (MACsec).</p>
         <note>
            <p>All connections in the LAG must be capable of  supporting MAC Security (MACsec). For information about MAC Security (MACsec) prerequisties, see  <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/direct-connect-mac-sec-getting-started.html#mac-sec-prerequisites">MACsec prerequisties</a> in the <i>Direct Connect User Guide</i>.</p>
         </note> |
| `connections_bandwidth` | String | ✅ | <p>The bandwidth of the individual physical dedicated connections bundled by the LAG. The
      possible values are  1Gbps,10Gbps, 100Gbps, and 400Gbps. </p> |
| `location` | String | ✅ | <p>The location for the LAG.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create lag
lag = provider.direct_connect.Lag {
    lag_name = "value"  # <p>The name of the LAG.</p>
    number_of_connections = "value"  # <p>The number of physical dedicated connections initially provisioned and bundled by the LAG.
      You can have a maximum of four connections when the port speed is 1Gbps or 10Gbps, or two when
      the port speed is 100Gbps or 400Gbps.</p>
    connections_bandwidth = "value"  # <p>The bandwidth of the individual physical dedicated connections bundled by the LAG. The
      possible values are  1Gbps,10Gbps, 100Gbps, and 400Gbps. </p>
    location = "value"  # <p>The location for the LAG.</p>
}

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
| `connections` | Vec<String> | <p>The connections.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


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


### Direct_connect_gateway_association_proposals

DirectConnectGatewayAssociationProposals resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `direct_connect_gateway_association_proposals` | Vec<String> | <p>Describes the Direct Connect gateway association proposals.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access direct_connect_gateway_association_proposals outputs
direct_connect_gateway_association_proposals_id = direct_connect_gateway_association_proposals.id
direct_connect_gateway_association_proposals_next_token = direct_connect_gateway_association_proposals.next_token
direct_connect_gateway_association_proposals_direct_connect_gateway_association_proposals = direct_connect_gateway_association_proposals.direct_connect_gateway_association_proposals
```

---


### Lags

Lags resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lags` | Vec<String> | <p>The LAGs.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access lags outputs
lags_id = lags.id
lags_lags = lags.lags
lags_next_token = lags.next_token
```

---


### Virtual_gateways

VirtualGateways resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `virtual_gateways` | Vec<String> | <p>The virtual private gateways.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access virtual_gateways outputs
virtual_gateways_id = virtual_gateways.id
virtual_gateways_virtual_gateways = virtual_gateways.virtual_gateways
```

---


### Connection

Connection resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `location` | String | ✅ | <p>The location of the connection.</p> |
| `bandwidth` | String | ✅ | <p>The bandwidth of the connection.</p> |
| `lag_id` | String |  | <p>The ID of the LAG.</p> |
| `provider_name` | String |  | <p>The name of the service provider associated with the requested connection.</p> |
| `request_mac_sec` | bool |  | <p>Indicates whether you want the connection to support MAC Security (MACsec).</p>
         <p>MAC Security (MACsec) is unavailable on hosted connections. For information about MAC Security (MACsec) prerequisites, see  <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/MACSec.html">MAC Security in Direct Connect</a> in the <i>Direct Connect User Guide</i>.</p> |
| `tags` | Vec<String> |  | <p>The tags to associate with the lag.</p> |
| `connection_name` | String | ✅ | <p>The name of the connection.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connection
connection = provider.direct_connect.Connection {
    location = "value"  # <p>The location of the connection.</p>
    bandwidth = "value"  # <p>The bandwidth of the connection.</p>
    connection_name = "value"  # <p>The name of the connection.</p>
}

```

---


### Virtual_interfaces

VirtualInterfaces resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `virtual_interfaces` | Vec<String> | <p>The virtual interfaces</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access virtual_interfaces outputs
virtual_interfaces_id = virtual_interfaces.id
virtual_interfaces_virtual_interfaces = virtual_interfaces.virtual_interfaces
virtual_interfaces_next_token = virtual_interfaces.next_token
```

---


### Hosted_connections

HostedConnections resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connections` | Vec<String> | <p>The connections.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access hosted_connections outputs
hosted_connections_id = hosted_connections.id
hosted_connections_connections = hosted_connections.connections
hosted_connections_next_token = hosted_connections.next_token
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
| `resource_tags` | Vec<String> | <p>Information about the tags.</p> |


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
tags_resource_tags = tags.resource_tags
```

---


### Direct_connect_gateway_attachments

DirectConnectGatewayAttachments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `direct_connect_gateway_attachments` | Vec<String> | <p>The attachments.</p> |
| `next_token` | String | <p>The token to retrieve the next page.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access direct_connect_gateway_attachments outputs
direct_connect_gateway_attachments_id = direct_connect_gateway_attachments.id
direct_connect_gateway_attachments_direct_connect_gateway_attachments = direct_connect_gateway_attachments.direct_connect_gateway_attachments
direct_connect_gateway_attachments_next_token = direct_connect_gateway_attachments.next_token
```

---


### Customer_metadata

CustomerMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `agreements` | Vec<String> | <p>The list of customer agreements.</p> |
| `nni_partner_type` | String | <p>The type of network-to-network interface (NNI) partner. The partner type will be one of the following:</p>
         <ul>
            <li>
               <p>V1: This partner can only allocate 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, or 500Mbps subgigabit connections.</p>
            </li>
            <li>
               <p>V2: This partner can only allocate 1GB, 2GB, 5GB, or 10GB hosted connections.</p>
            </li>
            <li>
               <p>nonPartner: The customer is not a partner.</p>
            </li>
         </ul> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access customer_metadata outputs
customer_metadata_id = customer_metadata.id
customer_metadata_agreements = customer_metadata.agreements
customer_metadata_nni_partner_type = customer_metadata.nni_partner_type
```

---


### Public_virtual_interface

PublicVirtualInterface resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connection_id` | String | ✅ | <p>The ID of the connection.</p> |
| `new_public_virtual_interface` | String | ✅ | <p>Information about the public virtual interface.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create public_virtual_interface
public_virtual_interface = provider.direct_connect.Public_virtual_interface {
    connection_id = "value"  # <p>The ID of the connection.</p>
    new_public_virtual_interface = "value"  # <p>Information about the public virtual interface.</p>
}

```

---


### Bgp_peer

BGPPeer resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `virtual_interface_id` | String |  | <p>The ID of the virtual interface.</p> |
| `new_bgp_peer` | String |  | <p>Information about the BGP peer.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bgp_peer
bgp_peer = provider.direct_connect.Bgp_peer {
}

```

---


### Virtual_interface_attributes

VirtualInterfaceAttributes resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `virtual_interface_name` | String |  | <p>The name of the virtual private interface.</p> |
| `virtual_interface_id` | String | ✅ | <p>The ID of the virtual private interface.</p> |
| `enable_site_link` | bool |  | <p>Indicates whether to enable or disable SiteLink.</p> |
| `mtu` | i64 |  | <p>The maximum transmission unit (MTU), in bytes. The supported values are 1500 and 8500. The default value is 1500.</p> |



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


### Connections_on_interconnect

ConnectionsOnInterconnect resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connections` | Vec<String> | <p>The connections.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connections_on_interconnect outputs
connections_on_interconnect_id = connections_on_interconnect.id
connections_on_interconnect_connections = connections_on_interconnect.connections
connections_on_interconnect_next_token = connections_on_interconnect.next_token
```

---


### Virtual_interface

VirtualInterface resource

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


### Interconnect_loa

InterconnectLoa resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `loa` | String | <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA).</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access interconnect_loa outputs
interconnect_loa_id = interconnect_loa.id
interconnect_loa_loa = interconnect_loa.loa
```

---


### Loa

Loa resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `loa_content` | String | <p>The binary contents of the LOA-CFA document.</p> |
| `loa_content_type` | String | <p>The standard media type for the LOA-CFA document. The only supported value is application/pdf.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access loa outputs
loa_id = loa.id
loa_loa_content = loa.loa_content
loa_loa_content_type = loa.loa_content_type
```

---


### Private_virtual_interface

PrivateVirtualInterface resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `new_private_virtual_interface` | String | ✅ | <p>Information about the private virtual interface.</p> |
| `connection_id` | String | ✅ | <p>The ID of the connection.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create private_virtual_interface
private_virtual_interface = provider.direct_connect.Private_virtual_interface {
    new_private_virtual_interface = "value"  # <p>Information about the private virtual interface.</p>
    connection_id = "value"  # <p>The ID of the connection.</p>
}

```

---


### Locations

Locations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `locations` | Vec<String> | <p>The locations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access locations outputs
locations_id = locations.id
locations_locations = locations.locations
```

---


### Direct_connect_gateway

DirectConnectGateway resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `amazon_side_asn` | i64 |  | <p>The autonomous system number (ASN) for Border Gateway Protocol (BGP) to be configured
      on the Amazon side of the connection. The ASN must be in the private range of 64,512 to
      65,534 or 4,200,000,000 to 4,294,967,294. The default is 64512.</p> |
| `direct_connect_gateway_name` | String | ✅ | <p>The name of the Direct Connect gateway.</p> |
| `tags` | Vec<String> |  | <p>The key-value pair tags associated with the request.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create direct_connect_gateway
direct_connect_gateway = provider.direct_connect.Direct_connect_gateway {
    direct_connect_gateway_name = "value"  # <p>The name of the Direct Connect gateway.</p>
}

```

---


### Interconnects

Interconnects resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `interconnects` | Vec<String> | <p>The interconnects.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access interconnects outputs
interconnects_id = interconnects.id
interconnects_interconnects = interconnects.interconnects
interconnects_next_token = interconnects.next_token
```

---


### Connection_loa

ConnectionLoa resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `loa` | String | <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA).</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connection_loa outputs
connection_loa_id = connection_loa.id
connection_loa_loa = connection_loa.loa
```

---


### Direct_connect_gateways

DirectConnectGateways resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `direct_connect_gateways` | Vec<String> | <p>The Direct Connect gateways.</p> |
| `next_token` | String | <p>The token to retrieve the next page.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access direct_connect_gateways outputs
direct_connect_gateways_id = direct_connect_gateways.id
direct_connect_gateways_direct_connect_gateways = direct_connect_gateways.direct_connect_gateways
direct_connect_gateways_next_token = direct_connect_gateways.next_token
```

---


### Direct_connect_gateway_association_proposal

DirectConnectGatewayAssociationProposal resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `add_allowed_prefixes_to_direct_connect_gateway` | Vec<String> |  | <p>The Amazon VPC prefixes to advertise to the Direct Connect gateway.</p> |
| `remove_allowed_prefixes_to_direct_connect_gateway` | Vec<String> |  | <p>The Amazon VPC prefixes to no longer advertise to the Direct Connect gateway.</p> |
| `gateway_id` | String | ✅ | <p>The ID of the virtual private gateway or transit gateway.</p> |
| `direct_connect_gateway_id` | String | ✅ | <p>The ID of the Direct Connect gateway.</p> |
| `direct_connect_gateway_owner_account` | String | ✅ | <p>The ID of the Amazon Web Services account that owns the Direct Connect gateway.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create direct_connect_gateway_association_proposal
direct_connect_gateway_association_proposal = provider.direct_connect.Direct_connect_gateway_association_proposal {
    gateway_id = "value"  # <p>The ID of the virtual private gateway or transit gateway.</p>
    direct_connect_gateway_id = "value"  # <p>The ID of the Direct Connect gateway.</p>
    direct_connect_gateway_owner_account = "value"  # <p>The ID of the Amazon Web Services account that owns the Direct Connect gateway.</p>
}

```

---


### Direct_connect_gateway_association

DirectConnectGatewayAssociation resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gateway_id` | String |  | <p>The ID of the virtual private gateway or transit gateway.</p> |
| `add_allowed_prefixes_to_direct_connect_gateway` | Vec<String> |  | <p>The Amazon VPC prefixes to advertise to the Direct Connect gateway</p>
         <p>This parameter is required when you create an association to a transit gateway.</p>
         <p>For information about how to set the prefixes, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/multi-account-associate-vgw.html#allowed-prefixes">Allowed Prefixes</a> in the <i>Direct Connect User Guide</i>.</p> |
| `direct_connect_gateway_id` | String | ✅ | <p>The ID of the Direct Connect gateway.</p> |
| `virtual_gateway_id` | String |  | <p>The ID of the virtual private gateway.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create direct_connect_gateway_association
direct_connect_gateway_association = provider.direct_connect.Direct_connect_gateway_association {
    direct_connect_gateway_id = "value"  # <p>The ID of the Direct Connect gateway.</p>
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

# Create multiple transit_virtual_interface resources
transit_virtual_interface_0 = provider.direct_connect.Transit_virtual_interface {
    new_transit_virtual_interface = "value-0"
    connection_id = "value-0"
}
transit_virtual_interface_1 = provider.direct_connect.Transit_virtual_interface {
    new_transit_virtual_interface = "value-1"
    connection_id = "value-1"
}
transit_virtual_interface_2 = provider.direct_connect.Transit_virtual_interface {
    new_transit_virtual_interface = "value-2"
    connection_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    transit_virtual_interface = provider.direct_connect.Transit_virtual_interface {
        new_transit_virtual_interface = "production-value"
        connection_id = "production-value"
    }
```

---

## Related Documentation

- [AWS Direct_connect Documentation](https://docs.aws.amazon.com/direct_connect/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
