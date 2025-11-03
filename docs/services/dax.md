# Dax Service



**Resources**: 9

---

## Overview

The dax service provides access to 9 resource types:

- [Clusters](#clusters) [R]
- [Events](#events) [R]
- [Subnet_groups](#subnet_groups) [R]
- [Parameter_groups](#parameter_groups) [R]
- [Parameters](#parameters) [R]
- [Cluster](#cluster) [CUD]
- [Subnet_group](#subnet_group) [CUD]
- [Default_parameters](#default_parameters) [R]
- [Parameter_group](#parameter_group) [CUD]

---

## Resources


### Clusters

Clusters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `clusters` | Vec<String> | <p>The descriptions of your DAX clusters, in response to a
                <i>DescribeClusters</i> request.</p> |
| `next_token` | String | <p>Provides an identifier to allow retrieval of paginated results.</p> |


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
clusters_clusters = clusters.clusters
clusters_next_token = clusters.next_token
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
| `next_token` | String | <p>Provides an identifier to allow retrieval of paginated results.</p> |
| `events` | Vec<String> | <p>An array of events. Each element in the array represents one event.</p> |


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
events_next_token = events.next_token
events_events = events.events
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
| `next_token` | String | <p>Provides an identifier to allow retrieval of paginated results.</p> |
| `subnet_groups` | Vec<String> | <p>An array of subnet groups. Each element in the array represents a single subnet
            group.</p> |


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
subnet_groups_next_token = subnet_groups.next_token
subnet_groups_subnet_groups = subnet_groups.subnet_groups
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
| `next_token` | String | <p>Provides an identifier to allow retrieval of paginated results.</p> |
| `parameter_groups` | Vec<String> | <p>An array of parameter groups. Each element in the array represents one parameter
            group.</p> |


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


### Parameters

Parameters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>Provides an identifier to allow retrieval of paginated results.</p> |
| `parameters` | Vec<String> | <p>A list of parameters within a parameter group. Each element in the list represents
            one parameter.</p> |


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


### Cluster

Cluster resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cluster_endpoint_encryption_type` | String |  | <p>The type of encryption the cluster's endpoint should support. Values are:</p>
         <ul>
            <li>
               <p>
                  <code>NONE</code> for no encryption</p>
            </li>
            <li>
               <p>
                  <code>TLS</code> for Transport Layer Security</p>
            </li>
         </ul> |
| `availability_zones` | Vec<String> |  | <p>The Availability Zones (AZs) in which the cluster nodes will reside after the
            cluster has been created or updated. If provided, the length of this list must equal the
                <code>ReplicationFactor</code> parameter. If you omit this parameter, DAX will spread the nodes across Availability Zones for the highest
            availability.</p> |
| `node_type` | String | ✅ | <p>The compute and memory capacity of the nodes in the cluster.</p> |
| `preferred_maintenance_window` | String |  | <p>Specifies the weekly time range during which maintenance on the DAX cluster is
            performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock
            UTC). The minimum maintenance window is a 60 minute period. Valid values for
                <code>ddd</code> are:</p>
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
         <p>Example: <code>sun:05:00-sun:09:00</code>
         </p>
         <note>
            <p>If you don't specify a preferred maintenance window when you create or modify a
                cache cluster, DAX assigns a 60-minute maintenance window on a
                randomly selected day of the week.</p>
         </note> |
| `sse_specification` | String |  | <p>Represents the settings used to enable server-side encryption on the
            cluster.</p> |
| `network_type` | String |  | <p>Specifies the IP protocol(s) the cluster uses for network communications. Values
            are:</p>
         <ul>
            <li>
               <p>
                  <code>ipv4</code> - The cluster is accessible only through IPv4
                    addresses</p>
            </li>
            <li>
               <p>
                  <code>ipv6</code> - The cluster is accessible only through IPv6
                    addresses</p>
            </li>
            <li>
               <p>
                  <code>dual_stack</code> - The cluster is accessible through both IPv4 and
                    IPv6 addresses.</p>
            </li>
         </ul>
         <note>
            <p>If no explicit <code>NetworkType</code> is provided, the network type is
                derived based on the subnet group's configuration.</p>
         </note> |
| `description` | String |  | <p>A description of the cluster.</p> |
| `notification_topic_arn` | String |  | <p>The Amazon Resource Name (ARN) of the Amazon SNS topic to which
            notifications will be sent.</p>
         <note>
            <p>The Amazon SNS topic owner must be same as the DAX
                cluster owner.</p>
         </note> |
| `parameter_group_name` | String |  | <p>The parameter group to be associated with the DAX cluster.</p> |
| `cluster_name` | String | ✅ | <p>The cluster identifier. This parameter is stored as a lowercase string.</p>
         <p>
            <b>Constraints:</b>
         </p>
         <ul>
            <li>
               <p>A name must contain from 1 to 20 alphanumeric characters or
                    hyphens.</p>
            </li>
            <li>
               <p>The first character must be a letter.</p>
            </li>
            <li>
               <p>A name cannot end with a hyphen or contain two consecutive
                    hyphens.</p>
            </li>
         </ul> |
| `replication_factor` | i64 | ✅ | <p>The number of nodes in the DAX cluster. A replication factor of 1
            will create a single-node cluster, without any read replicas. For additional fault
            tolerance, you can create a multiple node cluster with one or more read replicas. To do
            this, set <code>ReplicationFactor</code> to a number between 3 (one primary and two read
            replicas) and 10 (one primary and nine read replicas). <code>If the
                AvailabilityZones</code> parameter is provided, its length must equal the
                <code>ReplicationFactor</code>.</p>
         <note>
            <p>Amazon Web Services recommends that you have at least two read replicas per
                cluster.</p>
         </note> |
| `iam_role_arn` | String | ✅ | <p>A valid Amazon Resource Name (ARN) that identifies an IAM role. At
            runtime, DAX will assume this role and use the role's permissions to
            access DynamoDB on your behalf.</p> |
| `security_group_ids` | Vec<String> |  | <p>A list of security group IDs to be assigned to each node in the DAX
            cluster. (Each of the security group ID is system-generated.)</p>
         <p>If this parameter is not specified, DAX assigns the default VPC
            security group to each node.</p> |
| `subnet_group_name` | String |  | <p>The name of the subnet group to be used for the replication group.</p>
         <important>
            <p>DAX clusters can only run in an Amazon VPC environment.
                All of the subnets that you specify in a subnet group must exist in the same
                VPC.</p>
         </important> |
| `tags` | Vec<String> |  | <p>A set of tags to associate with the DAX cluster.
            </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster
cluster = provider.dax.Cluster {
    node_type = "value"  # <p>The compute and memory capacity of the nodes in the cluster.</p>
    cluster_name = "value"  # <p>The cluster identifier. This parameter is stored as a lowercase string.</p>
         <p>
            <b>Constraints:</b>
         </p>
         <ul>
            <li>
               <p>A name must contain from 1 to 20 alphanumeric characters or
                    hyphens.</p>
            </li>
            <li>
               <p>The first character must be a letter.</p>
            </li>
            <li>
               <p>A name cannot end with a hyphen or contain two consecutive
                    hyphens.</p>
            </li>
         </ul>
    replication_factor = "value"  # <p>The number of nodes in the DAX cluster. A replication factor of 1
            will create a single-node cluster, without any read replicas. For additional fault
            tolerance, you can create a multiple node cluster with one or more read replicas. To do
            this, set <code>ReplicationFactor</code> to a number between 3 (one primary and two read
            replicas) and 10 (one primary and nine read replicas). <code>If the
                AvailabilityZones</code> parameter is provided, its length must equal the
                <code>ReplicationFactor</code>.</p>
         <note>
            <p>Amazon Web Services recommends that you have at least two read replicas per
                cluster.</p>
         </note>
    iam_role_arn = "value"  # <p>A valid Amazon Resource Name (ARN) that identifies an IAM role. At
            runtime, DAX will assume this role and use the role's permissions to
            access DynamoDB on your behalf.</p>
}

```

---


### Subnet_group

SubnetGroup resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `subnet_group_name` | String | ✅ | <p>A name for the subnet group. This value is stored as a lowercase string. </p> |
| `description` | String |  | <p>A description for the subnet group</p> |
| `subnet_ids` | Vec<String> | ✅ | <p>A list of VPC subnet IDs for the subnet group.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create subnet_group
subnet_group = provider.dax.Subnet_group {
    subnet_group_name = "value"  # <p>A name for the subnet group. This value is stored as a lowercase string. </p>
    subnet_ids = "value"  # <p>A list of VPC subnet IDs for the subnet group.</p>
}

```

---


### Default_parameters

DefaultParameters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parameters` | Vec<String> | <p>A list of parameters. Each element in the list represents one parameter.</p> |
| `next_token` | String | <p>Provides an identifier to allow retrieval of paginated results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access default_parameters outputs
default_parameters_id = default_parameters.id
default_parameters_parameters = default_parameters.parameters
default_parameters_next_token = default_parameters.next_token
```

---


### Parameter_group

ParameterGroup resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description of the parameter group.</p> |
| `parameter_group_name` | String | ✅ | <p>The name of the parameter group to apply to all of the clusters in this replication
            group.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create parameter_group
parameter_group = provider.dax.Parameter_group {
    parameter_group_name = "value"  # <p>The name of the parameter group to apply to all of the clusters in this replication
            group.</p>
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

# Create multiple clusters resources
clusters_0 = provider.dax.Clusters {
}
clusters_1 = provider.dax.Clusters {
}
clusters_2 = provider.dax.Clusters {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    clusters = provider.dax.Clusters {
    }
```

---

## Related Documentation

- [AWS Dax Documentation](https://docs.aws.amazon.com/dax/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
