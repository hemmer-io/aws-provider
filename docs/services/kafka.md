# Kafka Service



**Resources**: 21

---

## Overview

The kafka service provides access to 21 resource types:

- [Cluster_v2](#cluster_v2) [CR]
- [Connectivity](#connectivity) [U]
- [Configuration_revision](#configuration_revision) [R]
- [Cluster](#cluster) [CRD]
- [Bootstrap_brokers](#bootstrap_brokers) [R]
- [Broker_storage](#broker_storage) [U]
- [Security](#security) [U]
- [Monitoring](#monitoring) [U]
- [Cluster_operation_v2](#cluster_operation_v2) [R]
- [Cluster_kafka_version](#cluster_kafka_version) [U]
- [Broker_count](#broker_count) [U]
- [Compatible_kafka_versions](#compatible_kafka_versions) [R]
- [Cluster_policy](#cluster_policy) [CRD]
- [Replicator](#replicator) [CRD]
- [Cluster_operation](#cluster_operation) [R]
- [Configuration](#configuration) [CRUD]
- [Vpc_connection](#vpc_connection) [CRD]
- [Broker_type](#broker_type) [U]
- [Cluster_configuration](#cluster_configuration) [U]
- [Replication_info](#replication_info) [U]
- [Storage](#storage) [U]

---

## Resources


### Cluster_v2

ClusterV2 resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `serverless` | String |  | <p>Information about the serverless cluster.</p> |
| `tags` | HashMap<String, String> |  | <p>A map of tags that you want the cluster to have.</p> |
| `provisioned` | String |  | <p>Information about the provisioned cluster.</p> |
| `cluster_name` | String | ✅ | <p>The name of the cluster.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cluster_info` | String | <p>The cluster information.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster_v2
cluster_v2 = provider.kafka.Cluster_v2 {
    cluster_name = "value"  # <p>The name of the cluster.</p>
}

# Access cluster_v2 outputs
cluster_v2_id = cluster_v2.id
cluster_v2_cluster_info = cluster_v2.cluster_info
```

---


### Connectivity

Connectivity resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connectivity_info` | String | ✅ | <p>Information about the broker access configuration.</p> |
| `cluster_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the configuration.</p> |
| `current_version` | String | ✅ | <p>The version of the MSK cluster to update. Cluster versions aren't simple numbers. You can describe an MSK cluster to find its version. When this update operation is successful, it generates a new cluster version.</p> |



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


### Configuration_revision

ConfigurationRevision resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>The description of the configuration.</p> |
| `revision` | i64 | <p>The revision number.</p> |
| `server_properties` | String | <p>Contents of the <filename>server.properties</filename> file. When using the API, you must ensure that the contents of the file are base64 encoded. 
               When using the AWS Management Console, the SDK, or the AWS CLI, the contents of <filename>server.properties</filename> can be in plaintext.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the configuration.</p> |
| `creation_time` | String | <p>The time when the configuration was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access configuration_revision outputs
configuration_revision_id = configuration_revision.id
configuration_revision_description = configuration_revision.description
configuration_revision_revision = configuration_revision.revision
configuration_revision_server_properties = configuration_revision.server_properties
configuration_revision_arn = configuration_revision.arn
configuration_revision_creation_time = configuration_revision.creation_time
```

---


### Cluster

Cluster resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `logging_info` | String |  |  |
| `number_of_broker_nodes` | i64 | ✅ | <p>The number of broker nodes in the cluster.</p> |
| `tags` | HashMap<String, String> |  | <p>Create tags when creating the cluster.</p> |
| `storage_mode` | String |  | <p>This controls storage mode for supported storage tiers.</p> |
| `configuration_info` | String |  | <p>Represents the configuration that you want MSK to use for the brokers in a cluster.</p> |
| `broker_node_group_info` | String | ✅ | <p>Information about the broker nodes in the cluster.</p> |
| `enhanced_monitoring` | String |  | <p>Specifies the level of monitoring for the MSK cluster. The possible values are DEFAULT, PER_BROKER, PER_TOPIC_PER_BROKER, and PER_TOPIC_PER_PARTITION.</p> |
| `client_authentication` | String |  | <p>Includes all client authentication related information.</p> |
| `cluster_name` | String | ✅ | <p>The name of the cluster.</p> |
| `encryption_info` | String |  | <p>Includes all encryption-related information.</p> |
| `open_monitoring` | String |  | <p>The settings for open monitoring.</p> |
| `kafka_version` | String | ✅ | <p>The version of Apache Kafka.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cluster_info` | String | <p>The cluster information.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster
cluster = provider.kafka.Cluster {
    number_of_broker_nodes = "value"  # <p>The number of broker nodes in the cluster.</p>
    broker_node_group_info = "value"  # <p>Information about the broker nodes in the cluster.</p>
    cluster_name = "value"  # <p>The name of the cluster.</p>
    kafka_version = "value"  # <p>The version of Apache Kafka.</p>
}

# Access cluster outputs
cluster_id = cluster.id
cluster_cluster_info = cluster.cluster_info
```

---


### Bootstrap_brokers

BootstrapBrokers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bootstrap_broker_string_public_tls` | String | <p>A string containing one or more DNS names (or IP) and TLS port pairs.</p> |
| `bootstrap_broker_string_vpc_connectivity_sasl_scram` | String | <p>A string containing one or more DNS names (or IP) and SASL/SCRAM port pairs for VPC connectivity.</p> |
| `bootstrap_broker_string_sasl_iam` | String | <p>A string that contains one or more DNS names (or IP addresses) and SASL IAM port pairs.</p> |
| `bootstrap_broker_string_public_sasl_iam` | String | <p>A string that contains one or more DNS names (or IP addresses) and SASL IAM port pairs.</p> |
| `bootstrap_broker_string` | String | <p>A string containing one or more hostname:port pairs.</p> |
| `bootstrap_broker_string_tls` | String | <p>A string containing one or more DNS names (or IP) and TLS port pairs.</p> |
| `bootstrap_broker_string_sasl_scram` | String | <p>A string containing one or more DNS names (or IP) and Sasl Scram port pairs.</p> |
| `bootstrap_broker_string_vpc_connectivity_sasl_iam` | String | <p>A string containing one or more DNS names (or IP) and SASL/IAM port pairs for VPC connectivity.</p> |
| `bootstrap_broker_string_public_sasl_scram` | String | <p>A string containing one or more DNS names (or IP) and Sasl Scram port pairs.</p> |
| `bootstrap_broker_string_vpc_connectivity_tls` | String | <p>A string containing one or more DNS names (or IP) and TLS port pairs for VPC connectivity.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access bootstrap_brokers outputs
bootstrap_brokers_id = bootstrap_brokers.id
bootstrap_brokers_bootstrap_broker_string_public_tls = bootstrap_brokers.bootstrap_broker_string_public_tls
bootstrap_brokers_bootstrap_broker_string_vpc_connectivity_sasl_scram = bootstrap_brokers.bootstrap_broker_string_vpc_connectivity_sasl_scram
bootstrap_brokers_bootstrap_broker_string_sasl_iam = bootstrap_brokers.bootstrap_broker_string_sasl_iam
bootstrap_brokers_bootstrap_broker_string_public_sasl_iam = bootstrap_brokers.bootstrap_broker_string_public_sasl_iam
bootstrap_brokers_bootstrap_broker_string = bootstrap_brokers.bootstrap_broker_string
bootstrap_brokers_bootstrap_broker_string_tls = bootstrap_brokers.bootstrap_broker_string_tls
bootstrap_brokers_bootstrap_broker_string_sasl_scram = bootstrap_brokers.bootstrap_broker_string_sasl_scram
bootstrap_brokers_bootstrap_broker_string_vpc_connectivity_sasl_iam = bootstrap_brokers.bootstrap_broker_string_vpc_connectivity_sasl_iam
bootstrap_brokers_bootstrap_broker_string_public_sasl_scram = bootstrap_brokers.bootstrap_broker_string_public_sasl_scram
bootstrap_brokers_bootstrap_broker_string_vpc_connectivity_tls = bootstrap_brokers.bootstrap_broker_string_vpc_connectivity_tls
```

---


### Broker_storage

BrokerStorage resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `current_version` | String | ✅ | <p>The version of cluster to update from. A successful operation will then generate a new version.</p> |
| `cluster_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) that uniquely identifies the cluster.</p> |
| `target_broker_ebs_volume_info` | Vec<String> | ✅ | <p>Describes the target volume size and the ID of the broker to apply the update to.</p> |



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


### Security

Security resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_authentication` | String |  | <p>Includes all client authentication related information.</p> |
| `current_version` | String | ✅ | <p>The version of the MSK cluster to update. Cluster versions aren't simple numbers. You can describe an MSK cluster to find its version. When this update operation is successful, it generates a new cluster version.</p> |
| `cluster_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) that uniquely identifies the cluster.</p> |
| `encryption_info` | String |  | <p>Includes all encryption-related information.</p> |



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


### Monitoring

Monitoring resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enhanced_monitoring` | String |  | <p>Specifies which Apache Kafka metrics Amazon MSK gathers and sends to Amazon CloudWatch for this cluster.</p> |
| `open_monitoring` | String |  | <p>The settings for open monitoring.</p> |
| `logging_info` | String |  |  |
| `current_version` | String | ✅ | <p>The version of the MSK cluster to update. Cluster versions aren't simple numbers. You can describe an MSK cluster to find its version. When this update operation is successful, it generates a new cluster version.</p> |
| `cluster_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) that uniquely identifies the cluster.</p> |



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


### Cluster_operation_v2

ClusterOperationV2 resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cluster_operation_info` | String | <p>Cluster operation information</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cluster_operation_v2 outputs
cluster_operation_v2_id = cluster_operation_v2.id
cluster_operation_v2_cluster_operation_info = cluster_operation_v2.cluster_operation_info
```

---


### Cluster_kafka_version

ClusterKafkaVersion resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration_info` | String |  | <p>The custom configuration that should be applied on the new version of cluster.</p> |
| `current_version` | String | ✅ | <p>Current cluster version.</p> |
| `target_kafka_version` | String | ✅ | <p>Target Kafka version.</p> |
| `cluster_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the cluster to be updated.</p> |



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


### Broker_count

BrokerCount resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `current_version` | String | ✅ | <p>The version of cluster to update from. A successful operation will then generate a new version.</p> |
| `target_number_of_broker_nodes` | i64 | ✅ | <p>The number of broker nodes that you want the cluster to have after this operation completes successfully.</p> |
| `cluster_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) that uniquely identifies the cluster.</p> |



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


### Compatible_kafka_versions

CompatibleKafkaVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `compatible_kafka_versions` | Vec<String> | <p>A list of CompatibleKafkaVersion objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access compatible_kafka_versions outputs
compatible_kafka_versions_id = compatible_kafka_versions.id
compatible_kafka_versions_compatible_kafka_versions = compatible_kafka_versions.compatible_kafka_versions
```

---


### Cluster_policy

ClusterPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cluster_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the cluster.</p> |
| `policy` | String | ✅ | <p>The policy.</p> |
| `current_version` | String |  | <p>The policy version.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `current_version` | String | <p>The version of cluster policy.</p> |
| `policy` | String | <p>The cluster policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster_policy
cluster_policy = provider.kafka.Cluster_policy {
    cluster_arn = "value"  # <p>The Amazon Resource Name (ARN) of the cluster.</p>
    policy = "value"  # <p>The policy.</p>
}

# Access cluster_policy outputs
cluster_policy_id = cluster_policy.id
cluster_policy_current_version = cluster_policy.current_version
cluster_policy_policy = cluster_policy.policy
```

---


### Replicator

Replicator resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>List of tags to attach to created Replicator.</p> |
| `replication_info_list` | Vec<String> | ✅ | <p>A list of replication configurations, where each configuration targets a given source cluster to target cluster replication flow.</p> |
| `description` | String |  | <p>A summary description of the replicator.</p> |
| `kafka_clusters` | Vec<String> | ✅ | <p>Kafka Clusters to use in setting up sources / targets for replication.</p> |
| `replicator_name` | String | ✅ | <p>The name of the replicator. Alpha-numeric characters with '-' are allowed.</p> |
| `service_execution_role_arn` | String | ✅ | <p>The ARN of the IAM role used by the replicator to access resources in the customer's account (e.g source and target clusters)</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state_info` | String | <p>Details about the state of the replicator.</p> |
| `current_version` | String | <p>The current version number of the replicator.</p> |
| `replicator_arn` | String | <p>The Amazon Resource Name (ARN) of the replicator.</p> |
| `replicator_name` | String | <p>The name of the replicator.</p> |
| `replicator_resource_arn` | String | <p>The Amazon Resource Name (ARN) of the replicator resource in the region where the replicator was created.</p> |
| `service_execution_role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role used by the replicator to access resources in the customer's account (e.g source and target clusters)</p> |
| `creation_time` | String | <p>The time when the replicator was created.</p> |
| `replicator_state` | String | <p>State of the replicator.</p> |
| `tags` | HashMap<String, String> | <p>List of tags attached to the Replicator.</p> |
| `is_replicator_reference` | bool | <p>Whether this resource is a replicator reference.</p> |
| `kafka_clusters` | Vec<String> | <p>Kafka Clusters used in setting up sources / targets for replication.</p> |
| `replicator_description` | String | <p>The description of the replicator.</p> |
| `replication_info_list` | Vec<String> | <p>A list of replication configurations, where each configuration targets a given source cluster to target cluster replication flow.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create replicator
replicator = provider.kafka.Replicator {
    replication_info_list = "value"  # <p>A list of replication configurations, where each configuration targets a given source cluster to target cluster replication flow.</p>
    kafka_clusters = "value"  # <p>Kafka Clusters to use in setting up sources / targets for replication.</p>
    replicator_name = "value"  # <p>The name of the replicator. Alpha-numeric characters with '-' are allowed.</p>
    service_execution_role_arn = "value"  # <p>The ARN of the IAM role used by the replicator to access resources in the customer's account (e.g source and target clusters)</p>
}

# Access replicator outputs
replicator_id = replicator.id
replicator_state_info = replicator.state_info
replicator_current_version = replicator.current_version
replicator_replicator_arn = replicator.replicator_arn
replicator_replicator_name = replicator.replicator_name
replicator_replicator_resource_arn = replicator.replicator_resource_arn
replicator_service_execution_role_arn = replicator.service_execution_role_arn
replicator_creation_time = replicator.creation_time
replicator_replicator_state = replicator.replicator_state
replicator_tags = replicator.tags
replicator_is_replicator_reference = replicator.is_replicator_reference
replicator_kafka_clusters = replicator.kafka_clusters
replicator_replicator_description = replicator.replicator_description
replicator_replication_info_list = replicator.replication_info_list
```

---


### Cluster_operation

ClusterOperation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cluster_operation_info` | String | <p>Cluster operation information</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cluster_operation outputs
cluster_operation_id = cluster_operation.id
cluster_operation_cluster_operation_info = cluster_operation.cluster_operation_info
```

---


### Configuration

Configuration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kafka_versions` | Vec<String> |  | <p>The versions of Apache Kafka with which you can use this MSK configuration.</p> |
| `name` | String | ✅ | <p>The name of the configuration.</p> |
| `server_properties` | String | ✅ | <p>Contents of the <filename>server.properties</filename> file. When using the API, you must ensure that the contents of the file are base64 encoded. 
               When using the AWS Management Console, the SDK, or the AWS CLI, the contents of <filename>server.properties</filename> can be in plaintext.</p> |
| `description` | String |  | <p>The description of the configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kafka_versions` | Vec<String> | <p>The versions of Apache Kafka with which you can use this MSK configuration.</p> |
| `latest_revision` | String | <p>Latest revision of the configuration.</p> |
| `description` | String | <p>The description of the configuration.</p> |
| `name` | String | <p>The name of the configuration.</p> |
| `state` | String | <p>The state of the configuration. The possible states are ACTIVE, DELETING, and DELETE_FAILED. </p> |
| `creation_time` | String | <p>The time when the configuration was created.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration
configuration = provider.kafka.Configuration {
    name = "value"  # <p>The name of the configuration.</p>
    server_properties = "value"  # <p>Contents of the <filename>server.properties</filename> file. When using the API, you must ensure that the contents of the file are base64 encoded. 
               When using the AWS Management Console, the SDK, or the AWS CLI, the contents of <filename>server.properties</filename> can be in plaintext.</p>
}

# Access configuration outputs
configuration_id = configuration.id
configuration_kafka_versions = configuration.kafka_versions
configuration_latest_revision = configuration.latest_revision
configuration_description = configuration.description
configuration_name = configuration.name
configuration_state = configuration.state
configuration_creation_time = configuration.creation_time
configuration_arn = configuration.arn
```

---


### Vpc_connection

VpcConnection resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vpc_id` | String | ✅ | <p>The VPC ID of VPC connection.</p> |
| `security_groups` | Vec<String> | ✅ | <p>The list of security groups.</p> |
| `client_subnets` | Vec<String> | ✅ | <p>The list of client subnets.</p> |
| `authentication` | String | ✅ | <p>The authentication type of VPC connection.</p> |
| `target_cluster_arn` | String | ✅ | <p>The cluster Amazon Resource Name (ARN) for the VPC connection.</p> |
| `tags` | HashMap<String, String> |  | <p>A map of tags for the VPC connection.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `authentication` | String | <p>The authentication type of VPC connection.</p> |
| `tags` | HashMap<String, String> | <p>A map of tags for the VPC connection.</p> |
| `vpc_connection_arn` | String | <p>The Amazon Resource Name (ARN) that uniquely identifies a MSK VPC connection.</p> |
| `subnets` | Vec<String> | <p>The list of subnets for the VPC connection.</p> |
| `state` | String | <p>The state of VPC connection.</p> |
| `target_cluster_arn` | String | <p>The Amazon Resource Name (ARN) that uniquely identifies an MSK cluster.</p> |
| `creation_time` | String | <p>The creation time of the VPC connection.</p> |
| `vpc_id` | String | <p>The VPC Id for the VPC connection.</p> |
| `security_groups` | Vec<String> | <p>The list of security groups for the VPC connection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_connection
vpc_connection = provider.kafka.Vpc_connection {
    vpc_id = "value"  # <p>The VPC ID of VPC connection.</p>
    security_groups = "value"  # <p>The list of security groups.</p>
    client_subnets = "value"  # <p>The list of client subnets.</p>
    authentication = "value"  # <p>The authentication type of VPC connection.</p>
    target_cluster_arn = "value"  # <p>The cluster Amazon Resource Name (ARN) for the VPC connection.</p>
}

# Access vpc_connection outputs
vpc_connection_id = vpc_connection.id
vpc_connection_authentication = vpc_connection.authentication
vpc_connection_tags = vpc_connection.tags
vpc_connection_vpc_connection_arn = vpc_connection.vpc_connection_arn
vpc_connection_subnets = vpc_connection.subnets
vpc_connection_state = vpc_connection.state
vpc_connection_target_cluster_arn = vpc_connection.target_cluster_arn
vpc_connection_creation_time = vpc_connection.creation_time
vpc_connection_vpc_id = vpc_connection.vpc_id
vpc_connection_security_groups = vpc_connection.security_groups
```

---


### Broker_type

BrokerType resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `current_version` | String | ✅ | <p>The cluster version that you want to change. After this operation completes successfully, the cluster will have a new version.</p> |
| `cluster_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) that uniquely identifies the cluster.</p> |
| `target_instance_type` | String | ✅ | <p>The Amazon MSK broker type that you want all of the brokers in this cluster to be.</p> |



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


### Cluster_configuration

ClusterConfiguration resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `current_version` | String | ✅ | <p>The version of the cluster that needs to be updated.</p> |
| `configuration_info` | String | ✅ | <p>Represents the configuration that you want MSK to use for the brokers in a cluster.</p> |
| `cluster_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) that uniquely identifies the cluster.</p> |



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


### Replication_info

ReplicationInfo resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source_kafka_cluster_arn` | String | ✅ | <p>The ARN of the source Kafka cluster.</p> |
| `target_kafka_cluster_arn` | String | ✅ | <p>The ARN of the target Kafka cluster.</p> |
| `topic_replication` | String |  | <p>Updated topic replication information.</p> |
| `replicator_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the replicator to be updated.</p> |
| `consumer_group_replication` | String |  | <p>Updated consumer group replication information.</p> |
| `current_version` | String | ✅ | <p>Current replicator version.</p> |



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


### Storage

Storage resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `provisioned_throughput` | String |  | <p>EBS volume provisioned throughput information.</p> |
| `storage_mode` | String |  | <p>Controls storage mode for supported storage tiers.</p> |
| `cluster_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the cluster to be updated.</p> |
| `volume_size_gb` | i64 |  | <p>size of the EBS volume to update.</p> |
| `current_version` | String | ✅ | <p>The version of cluster to update from. A successful operation will then generate a new version.</p> |



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

# Create multiple cluster_v2 resources
cluster_v2_0 = provider.kafka.Cluster_v2 {
    cluster_name = "value-0"
}
cluster_v2_1 = provider.kafka.Cluster_v2 {
    cluster_name = "value-1"
}
cluster_v2_2 = provider.kafka.Cluster_v2 {
    cluster_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    cluster_v2 = provider.kafka.Cluster_v2 {
        cluster_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Kafka Documentation](https://docs.aws.amazon.com/kafka/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
