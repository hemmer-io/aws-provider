# Memorydb Service



**Resources**: 22

---

## Overview

The memorydb service provides access to 22 resource types:

- [Reserved_nodes](#reserved_nodes) [R]
- [Reserved_nodes_offerings](#reserved_nodes_offerings) [R]
- [Ac_ls](#ac_ls) [R]
- [Multi_region_clusters](#multi_region_clusters) [R]
- [Multi_region_cluster](#multi_region_cluster) [CUD]
- [Parameter_groups](#parameter_groups) [R]
- [Snapshots](#snapshots) [R]
- [Subnet_groups](#subnet_groups) [R]
- [Users](#users) [R]
- [Cluster](#cluster) [CUD]
- [Snapshot](#snapshot) [CD]
- [Engine_versions](#engine_versions) [R]
- [Multi_region_parameters](#multi_region_parameters) [R]
- [Service_updates](#service_updates) [R]
- [Subnet_group](#subnet_group) [CUD]
- [Multi_region_parameter_groups](#multi_region_parameter_groups) [R]
- [Events](#events) [R]
- [Parameter_group](#parameter_group) [CUD]
- [Acl](#acl) [CUD]
- [User](#user) [CUD]
- [Clusters](#clusters) [R]
- [Parameters](#parameters) [R]

---

## Resources


### Reserved_nodes

ReservedNodes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reserved_nodes` | Vec<String> | <p>Returns information about reserved nodes for this account, or about a specified reserved node.</p> |
| `next_token` | String | <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by MaxRecords.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_nodes outputs
reserved_nodes_id = reserved_nodes.id
reserved_nodes_reserved_nodes = reserved_nodes.reserved_nodes
reserved_nodes_next_token = reserved_nodes.next_token
```

---


### Reserved_nodes_offerings

ReservedNodesOfferings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by MaxRecords.</p> |
| `reserved_nodes_offerings` | Vec<String> | <p>Lists available reserved node offerings.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_nodes_offerings outputs
reserved_nodes_offerings_id = reserved_nodes_offerings.id
reserved_nodes_offerings_next_token = reserved_nodes_offerings.next_token
reserved_nodes_offerings_reserved_nodes_offerings = reserved_nodes_offerings.reserved_nodes_offerings
```

---


### Ac_ls

ACLs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ac_ls` | Vec<String> | <p>The list of ACLs.</p> |
| `next_token` | String | <p>If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ac_ls outputs
ac_ls_id = ac_ls.id
ac_ls_ac_ls = ac_ls.ac_ls
ac_ls_next_token = ac_ls.next_token
```

---


### Multi_region_clusters

MultiRegionClusters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `multi_region_clusters` | Vec<String> | <p>A list of multi-Region clusters.</p> |
| `next_token` | String | <p>A token to use to retrieve the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access multi_region_clusters outputs
multi_region_clusters_id = multi_region_clusters.id
multi_region_clusters_multi_region_clusters = multi_region_clusters.multi_region_clusters
multi_region_clusters_next_token = multi_region_clusters.next_token
```

---


### Multi_region_cluster

MultiRegionCluster resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `node_type` | String | ✅ | <p>The node type to be used for the multi-Region cluster.</p> |
| `multi_region_parameter_group_name` | String |  | <p>The name of the multi-Region parameter group to be associated with the cluster.</p> |
| `multi_region_cluster_name_suffix` | String | ✅ | <p>A suffix to be added to the Multi-Region cluster name. Amazon MemoryDB automatically applies a prefix to the Multi-Region cluster Name when it is created. Each Amazon Region has its own prefix. For instance, a Multi-Region cluster Name created in the US-West-1 region will begin with "virxk", along with the suffix name you provide. The suffix guarantees uniqueness of the Multi-Region cluster name across multiple regions.</p> |
| `description` | String |  | <p>A description for the multi-Region cluster.</p> |
| `engine` | String |  | <p>The name of the engine to be used for the multi-Region cluster.</p> |
| `engine_version` | String |  | <p>The version of the engine to be used for the multi-Region cluster.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to be applied to the multi-Region cluster.</p> |
| `num_shards` | i64 |  | <p>The number of shards for the multi-Region cluster.</p> |
| `tls_enabled` | bool |  | <p>Whether to enable TLS encryption for the multi-Region cluster.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multi_region_cluster
multi_region_cluster = provider.memorydb.Multi_region_cluster {
    node_type = "value"  # <p>The node type to be used for the multi-Region cluster.</p>
    multi_region_cluster_name_suffix = "value"  # <p>A suffix to be added to the Multi-Region cluster name. Amazon MemoryDB automatically applies a prefix to the Multi-Region cluster Name when it is created. Each Amazon Region has its own prefix. For instance, a Multi-Region cluster Name created in the US-West-1 region will begin with "virxk", along with the suffix name you provide. The suffix guarantees uniqueness of the Multi-Region cluster name across multiple regions.</p>
}

```

---


### Parameter_groups

ParameterGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p> |
| `parameter_groups` | Vec<String> | <p>A list of parameter groups. Each element in the list contains detailed information about one parameter group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access parameter_groups outputs
parameter_groups_id = parameter_groups.id
parameter_groups_next_token = parameter_groups.next_token
parameter_groups_parameter_groups = parameter_groups.parameter_groups
```

---


### Snapshots

Snapshots resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p> |
| `snapshots` | Vec<String> | <p>A list of snapshots. Each item in the list contains detailed information about one snapshot.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access snapshots outputs
snapshots_id = snapshots.id
snapshots_next_token = snapshots.next_token
snapshots_snapshots = snapshots.snapshots
```

---


### Subnet_groups

SubnetGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subnet_groups` | Vec<String> | <p>A list of subnet groups. Each element in the list contains detailed information about one group.</p> |
| `next_token` | String | <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access subnet_groups outputs
subnet_groups_id = subnet_groups.id
subnet_groups_subnet_groups = subnet_groups.subnet_groups
subnet_groups_next_token = subnet_groups.next_token
```

---


### Users

Users resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p> |
| `users` | Vec<String> | <p>A list of users.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access users outputs
users_id = users.id
users_next_token = users.next_token
users_users = users.users
```

---


### Cluster

Cluster resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `snapshot_retention_limit` | i64 |  | <p>The number of days for which MemoryDB retains automatic snapshots before deleting them. For example, if you set SnapshotRetentionLimit to 5, a snapshot that was taken today is retained for 5 days before being deleted.</p> |
| `security_group_ids` | Vec<String> |  | <p>A list of security group names to associate with this cluster.</p> |
| `node_type` | String | ✅ | <p>The compute and memory capacity of the nodes in the cluster.</p> |
| `multi_region_cluster_name` | String |  | <p>The name of the multi-Region cluster to be created.</p> |
| `parameter_group_name` | String |  | <p>The name of the parameter group associated with the cluster.</p> |
| `maintenance_window` | String |  | <p>Specifies the weekly time range during which maintenance
         on the cluster is performed. It is specified as a range in
         the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum
         maintenance window is a 60 minute period.</p>
         <p>Valid values for <code>ddd</code> are:</p>
         <ul>
            <li>
               <p>
                  <code>sun</code>
               </p>
            </li>
            <li>
               <p>
                  <code>mon</code>
               </p>
            </li>
            <li>
               <p>
                  <code>tue</code>
               </p>
            </li>
            <li>
               <p>
                  <code>wed</code>
               </p>
            </li>
            <li>
               <p>
                  <code>thu</code>
               </p>
            </li>
            <li>
               <p>
                  <code>fri</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sat</code>
               </p>
            </li>
         </ul>
         <p>Example: <code>sun:23:00-mon:01:30</code>
         </p> |
| `num_replicas_per_shard` | i64 |  | <p>The number of replicas to apply to each shard. The default value is 1. The maximum is 5. </p> |
| `tls_enabled` | bool |  | <p>A flag to enable in-transit encryption on the cluster.</p> |
| `cluster_name` | String | ✅ | <p>The name of the cluster. This value must be unique as it also serves as the cluster identifier.</p> |
| `snapshot_arns` | Vec<String> |  | <p>A list of Amazon Resource Names (ARN) that uniquely identify the RDB snapshot files stored in Amazon S3. The snapshot files are used to populate the new cluster. The Amazon S3 object name in the ARN cannot contain any commas.</p> |
| `snapshot_name` | String |  | <p>The name of a snapshot from which to restore data into the new cluster. The snapshot status changes to restoring while the new cluster is being created.</p> |
| `engine_version` | String |  | <p>The version number of the Redis OSS engine to be used for the cluster.</p> |
| `data_tiering` | bool |  | <p>Enables data tiering. Data tiering is only supported for clusters using the r6gd node type. 
            This parameter must be set when using r6gd nodes. For more information, see <a href="https://docs.aws.amazon.com/memorydb/latest/devguide/data-tiering.html">Data tiering</a>.</p> |
| `description` | String |  | <p>An optional description of the cluster.</p> |
| `port` | i64 |  | <p>The port number on which each of the nodes accepts connections.</p> |
| `engine` | String |  | <p>The name of the engine to be used for the cluster.</p> |
| `acl_name` | String | ✅ | <p>The name of the Access Control List to associate with the cluster.</p> |
| `snapshot_window` | String |  | <p>The daily time range (in UTC) during which MemoryDB begins taking a daily snapshot of your shard.</p>
         <p>    Example: 05:00-09:00</p>
         <p>    If you do not specify this parameter, MemoryDB automatically chooses an appropriate time range.</p> |
| `kms_key_id` | String |  | <p>The ID of the KMS key used to encrypt the cluster.</p> |
| `auto_minor_version_upgrade` | bool |  | <p>When set to true, the cluster will automatically receive minor engine version upgrades after launch.</p> |
| `network_type` | String |  | <p>Specifies the IP address type for the cluster. Valid values are 'ipv4', 'ipv6', or 'dual_stack'. When set to 'ipv4', the cluster will only be accessible via IPv4 addresses. When set to 'ipv6', the cluster will only be accessible via IPv6 addresses. When set to 'dual_stack', the cluster will be accessible via both IPv4 and IPv6 addresses. If not specified, the default is 'ipv4'.</p> |
| `subnet_group_name` | String |  | <p>The name of the subnet group to be used for the cluster.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to be added to this resource. Tags are comma-separated key,value pairs (e.g. Key=myKey, Value=myKeyValue. You can include multiple tags as shown following: Key=myKey, Value=myKeyValue Key=mySecondKey, Value=mySecondKeyValue.</p> |
| `num_shards` | i64 |  | <p>The number of shards the cluster will contain. The default value is 1. </p> |
| `sns_topic_arn` | String |  | <p>The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS) topic to which notifications are sent.</p> |
| `ip_discovery` | String |  | <p>The mechanism for discovering IP addresses for the cluster discovery protocol. Valid values are 'ipv4' or 'ipv6'. When set to 'ipv4', cluster discovery functions such as cluster slots, cluster shards, and cluster nodes return IPv4 addresses for cluster nodes. When set to 'ipv6', the cluster discovery functions return IPv6 addresses for cluster nodes. The value must be compatible with the NetworkType parameter. If not specified, the default is 'ipv4'.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster
cluster = provider.memorydb.Cluster {
    node_type = "value"  # <p>The compute and memory capacity of the nodes in the cluster.</p>
    cluster_name = "value"  # <p>The name of the cluster. This value must be unique as it also serves as the cluster identifier.</p>
    acl_name = "value"  # <p>The name of the Access Control List to associate with the cluster.</p>
}

```

---


### Snapshot

Snapshot resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `snapshot_name` | String | ✅ | <p>A name for the snapshot being created.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to be added to this resource. A tag is a key-value pair. A tag key must be accompanied by a tag value, although null is accepted.</p> |
| `cluster_name` | String | ✅ | <p>The snapshot is created from this cluster.</p> |
| `kms_key_id` | String |  | <p>The ID of the KMS key used to encrypt the snapshot.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create snapshot
snapshot = provider.memorydb.Snapshot {
    snapshot_name = "value"  # <p>A name for the snapshot being created.</p>
    cluster_name = "value"  # <p>The snapshot is created from this cluster.</p>
}

```

---


### Engine_versions

EngineVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `engine_versions` | Vec<String> | <p>A list of engine version details. Each element in the list contains detailed information about one engine version.</p> |
| `next_token` | String | <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access engine_versions outputs
engine_versions_id = engine_versions.id
engine_versions_engine_versions = engine_versions.engine_versions
engine_versions_next_token = engine_versions.next_token
```

---


### Multi_region_parameters

MultiRegionParameters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `multi_region_parameters` | Vec<String> | <p>A list of parameters specific to a particular multi-region parameter group. Each element in the list contains detailed information about one parameter.</p> |
| `next_token` | String | <p>An optional token to include in the response. If this token is provided, the response includes only results beyond the token, up to the value specified by MaxResults.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access multi_region_parameters outputs
multi_region_parameters_id = multi_region_parameters.id
multi_region_parameters_multi_region_parameters = multi_region_parameters.multi_region_parameters
multi_region_parameters_next_token = multi_region_parameters.next_token
```

---


### Service_updates

ServiceUpdates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p> |
| `service_updates` | Vec<String> | <p>A list of service updates</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_updates outputs
service_updates_id = service_updates.id
service_updates_next_token = service_updates.next_token
service_updates_service_updates = service_updates.service_updates
```

---


### Subnet_group

SubnetGroup resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description for the subnet group.</p> |
| `subnet_group_name` | String | ✅ | <p>The name of the subnet group.</p> |
| `subnet_ids` | Vec<String> | ✅ | <p>A list of VPC subnet IDs for the subnet group.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to be added to this resource. A tag is a key-value pair. A tag key must be accompanied by a tag value, although null is accepted.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create subnet_group
subnet_group = provider.memorydb.Subnet_group {
    subnet_group_name = "value"  # <p>The name of the subnet group.</p>
    subnet_ids = "value"  # <p>A list of VPC subnet IDs for the subnet group.</p>
}

```

---


### Multi_region_parameter_groups

MultiRegionParameterGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>An optional token to include in the response. If this token is provided, the response includes only results beyond the token, up to the value specified by MaxResults.</p> |
| `multi_region_parameter_groups` | Vec<String> | <p>A list of multi-region parameter groups. Each element in the list contains detailed information about one parameter group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access multi_region_parameter_groups outputs
multi_region_parameter_groups_id = multi_region_parameter_groups.id
multi_region_parameter_groups_next_token = multi_region_parameter_groups.next_token
multi_region_parameter_groups_multi_region_parameter_groups = multi_region_parameter_groups.multi_region_parameter_groups
```

---


### Events

Events resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `events` | Vec<String> | <p>A list of events. Each element in the list contains detailed information about one event.</p> |
| `next_token` | String | <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access events outputs
events_id = events.id
events_events = events.events
events_next_token = events.next_token
```

---


### Parameter_group

ParameterGroup resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `family` | String | ✅ | <p>The name of the parameter group family that the parameter group can be used with.</p> |
| `parameter_group_name` | String | ✅ | <p>The name of the parameter group.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to be added to this resource. A tag is a key-value pair. A tag key must be accompanied by a tag value, although null is accepted.</p> |
| `description` | String |  | <p>An optional description of the parameter group.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create parameter_group
parameter_group = provider.memorydb.Parameter_group {
    family = "value"  # <p>The name of the parameter group family that the parameter group can be used with.</p>
    parameter_group_name = "value"  # <p>The name of the parameter group.</p>
}

```

---


### Acl

ACL resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `acl_name` | String | ✅ | <p>The name of the Access Control List.</p> |
| `user_names` | Vec<String> |  | <p>The list of users that belong to the Access Control List.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to be added to this resource. A tag is a key-value pair. A tag key must be accompanied by a tag value, although null is accepted.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create acl
acl = provider.memorydb.Acl {
    acl_name = "value"  # <p>The name of the Access Control List.</p>
}

```

---


### User

User resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of tags to be added to this resource. A tag is a key-value pair. A tag key must be accompanied by a tag value, although null is accepted.</p> |
| `user_name` | String | ✅ | <p>The name of the user. This value must be unique as it also serves as the user identifier.</p> |
| `access_string` | String | ✅ | <p>Access permissions string used for this user.</p> |
| `authentication_mode` | String | ✅ | <p>Denotes the user's authentication properties, such as whether it requires a password to authenticate.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user
user = provider.memorydb.User {
    user_name = "value"  # <p>The name of the user. This value must be unique as it also serves as the user identifier.</p>
    access_string = "value"  # <p>Access permissions string used for this user.</p>
    authentication_mode = "value"  # <p>Denotes the user's authentication properties, such as whether it requires a password to authenticate.</p>
}

```

---


### Clusters

Clusters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p> |
| `clusters` | Vec<String> | <p>A list of clusters</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access clusters outputs
clusters_id = clusters.id
clusters_next_token = clusters.next_token
clusters_clusters = clusters.clusters
```

---


### Parameters

Parameters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p> |
| `parameters` | Vec<String> | <p>A list of parameters specific to a particular parameter group. Each element in the list contains detailed information about one parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access parameters outputs
parameters_id = parameters.id
parameters_next_token = parameters.next_token
parameters_parameters = parameters.parameters
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple reserved_nodes resources
reserved_nodes_0 = provider.memorydb.Reserved_nodes {
}
reserved_nodes_1 = provider.memorydb.Reserved_nodes {
}
reserved_nodes_2 = provider.memorydb.Reserved_nodes {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    reserved_nodes = provider.memorydb.Reserved_nodes {
    }
```

---

## Related Documentation

- [AWS Memorydb Documentation](https://docs.aws.amazon.com/memorydb/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
