# Docdb Service



**Resources**: 23

---

## Overview

The docdb service provides access to 23 resource types:

- [Db_cluster_parameter_groups](#db_cluster_parameter_groups) [R]
- [Db_cluster_snapshots](#db_cluster_snapshots) [R]
- [Db_subnet_groups](#db_subnet_groups) [R]
- [Engine_default_cluster_parameters](#engine_default_cluster_parameters) [R]
- [Db_cluster_parameter_group](#db_cluster_parameter_group) [CD]
- [Event_categories](#event_categories) [R]
- [Global_clusters](#global_clusters) [R]
- [Event_subscription](#event_subscription) [CD]
- [Pending_maintenance_actions](#pending_maintenance_actions) [R]
- [Certificates](#certificates) [R]
- [Db_clusters](#db_clusters) [R]
- [Orderable_db_instance_options](#orderable_db_instance_options) [R]
- [Db_engine_versions](#db_engine_versions) [R]
- [Db_cluster_parameters](#db_cluster_parameters) [R]
- [Db_instances](#db_instances) [R]
- [Events](#events) [R]
- [Db_instance](#db_instance) [CD]
- [Event_subscriptions](#event_subscriptions) [R]
- [Db_cluster_snapshot](#db_cluster_snapshot) [CD]
- [Global_cluster](#global_cluster) [CD]
- [Db_cluster_snapshot_attributes](#db_cluster_snapshot_attributes) [R]
- [Db_subnet_group](#db_subnet_group) [CD]
- [Db_cluster](#db_cluster) [CD]

---

## Resources


### Db_cluster_parameter_groups

DBClusterParameterGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_cluster_parameter_groups` | Vec<String> | <p>A list of cluster parameter groups.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is specified, the response
         includes only records beyond the marker, up to the value specified by
         <code>MaxRecords</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_cluster_parameter_groups outputs
db_cluster_parameter_groups_id = db_cluster_parameter_groups.id
db_cluster_parameter_groups_db_cluster_parameter_groups = db_cluster_parameter_groups.db_cluster_parameter_groups
db_cluster_parameter_groups_marker = db_cluster_parameter_groups.marker
```

---


### Db_cluster_snapshots

DBClusterSnapshots resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_cluster_snapshots` | Vec<String> | <p>Provides a list of cluster snapshots.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is specified, the response
         includes only records beyond the marker, up to the value specified by
         <code>MaxRecords</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_cluster_snapshots outputs
db_cluster_snapshots_id = db_cluster_snapshots.id
db_cluster_snapshots_db_cluster_snapshots = db_cluster_snapshots.db_cluster_snapshots
db_cluster_snapshots_marker = db_cluster_snapshots.marker
```

---


### Db_subnet_groups

DBSubnetGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_subnet_groups` | Vec<String> | <p>Detailed information about one or more subnet groups.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is specified, the response
         includes only records beyond the marker, up to the value specified by
         <code>MaxRecords</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_subnet_groups outputs
db_subnet_groups_id = db_subnet_groups.id
db_subnet_groups_db_subnet_groups = db_subnet_groups.db_subnet_groups
db_subnet_groups_marker = db_subnet_groups.marker
```

---


### Engine_default_cluster_parameters

EngineDefaultClusterParameters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `engine_defaults` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access engine_default_cluster_parameters outputs
engine_default_cluster_parameters_id = engine_default_cluster_parameters.id
engine_default_cluster_parameters_engine_defaults = engine_default_cluster_parameters.engine_defaults
```

---


### Db_cluster_parameter_group

DBClusterParameterGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String | ✅ | <p>The description for the cluster parameter group.</p> |
| `db_cluster_parameter_group_name` | String | ✅ | <p>The name of the cluster parameter group.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must not match the name of an existing
                    <code>DBClusterParameterGroup</code>.</p>
            </li>
         </ul>
         <note>
            <p>This value is stored as a lowercase string.</p>
         </note> |
| `db_parameter_group_family` | String | ✅ | <p>The cluster parameter group family name.</p> |
| `tags` | Vec<String> |  | <p>The tags to be assigned to the cluster parameter group.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_cluster_parameter_group
db_cluster_parameter_group = provider.docdb.Db_cluster_parameter_group {
    description = "value"  # <p>The description for the cluster parameter group.</p>
    db_cluster_parameter_group_name = "value"  # <p>The name of the cluster parameter group.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must not match the name of an existing
                    <code>DBClusterParameterGroup</code>.</p>
            </li>
         </ul>
         <note>
            <p>This value is stored as a lowercase string.</p>
         </note>
    db_parameter_group_family = "value"  # <p>The cluster parameter group family name.</p>
}

```

---


### Event_categories

EventCategories resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_categories_map_list` | Vec<String> | <p>A list of event category maps.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access event_categories outputs
event_categories_id = event_categories.id
event_categories_event_categories_map_list = event_categories.event_categories_map_list
```

---


### Global_clusters

GlobalClusters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `global_clusters` | Vec<String> | <p></p> |
| `marker` | String | <p></p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access global_clusters outputs
global_clusters_id = global_clusters.id
global_clusters_global_clusters = global_clusters.global_clusters
global_clusters_marker = global_clusters.marker
```

---


### Event_subscription

EventSubscription resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `subscription_name` | String | ✅ | <p>The name of the subscription.</p>
         <p>Constraints: The name must be fewer than 255 characters.</p> |
| `sns_topic_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the SNS topic created for event notification. Amazon SNS creates the ARN when you create a topic and subscribe to it.</p> |
| `source_ids` | Vec<String> |  | <p>The list of identifiers of the event sources for which events are returned. If not specified, then all sources are included in the response. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens; it can't end with a
            hyphen or contain two consecutive hyphens.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>If <code>SourceIds</code> are provided, <code>SourceType</code> must also be
                    provided.</p>
            </li>
            <li>
               <p>If the source type is an instance, a <code>DBInstanceIdentifier</code> must
                    be provided.</p>
            </li>
            <li>
               <p>If the source type is a security group, a <code>DBSecurityGroupName</code>
                    must be provided.</p>
            </li>
            <li>
               <p>If the source type is a parameter group, a
                        <code>DBParameterGroupName</code> must be provided.</p>
            </li>
            <li>
               <p>If the source type is a snapshot, a <code>DBSnapshotIdentifier</code> must
                    be provided.</p>
            </li>
         </ul> |
| `source_type` | String |  | <p>The type of source that is generating the events. For example, if you want to be notified of events generated by an instance, you would set this parameter to <code>db-instance</code>. If this value is not specified, all events are returned.</p>
         <p>Valid values: <code>db-instance</code>, <code>db-cluster</code>,
                <code>db-parameter-group</code>, <code>db-security-group</code>,
                <code>db-cluster-snapshot</code>
         </p> |
| `enabled` | bool |  | <p> A Boolean value; set to <code>true</code> to activate the subscription, set to <code>false</code> to create the subscription but not active it. </p> |
| `tags` | Vec<String> |  | <p>The tags to be assigned to the event subscription.</p> |
| `event_categories` | Vec<String> |  | <p> A list of event categories for a <code>SourceType</code> that you want to subscribe to. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_subscription
event_subscription = provider.docdb.Event_subscription {
    subscription_name = "value"  # <p>The name of the subscription.</p>
         <p>Constraints: The name must be fewer than 255 characters.</p>
    sns_topic_arn = "value"  # <p>The Amazon Resource Name (ARN) of the SNS topic created for event notification. Amazon SNS creates the ARN when you create a topic and subscribe to it.</p>
}

```

---


### Pending_maintenance_actions

PendingMaintenanceActions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is specified, the response
         includes only records beyond the marker, up to the value specified by
         <code>MaxRecords</code>.</p> |
| `pending_maintenance_actions` | Vec<String> | <p>The maintenance actions to be applied.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access pending_maintenance_actions outputs
pending_maintenance_actions_id = pending_maintenance_actions.id
pending_maintenance_actions_marker = pending_maintenance_actions.marker
pending_maintenance_actions_pending_maintenance_actions = pending_maintenance_actions.pending_maintenance_actions
```

---


### Certificates

Certificates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `certificates` | Vec<String> | <p>A list of certificates for this Amazon Web Services account.</p> |
| `marker` | String | <p>An optional pagination token provided if the number of records retrieved is greater than <code>MaxRecords</code>. If this parameter is specified, the marker specifies the next record in the list. Including the value of <code>Marker</code> in the next call to <code>DescribeCertificates</code> results in the next page of certificates.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access certificates outputs
certificates_id = certificates.id
certificates_certificates = certificates.certificates
certificates_marker = certificates.marker
```

---


### Db_clusters

DBClusters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is specified, the response
         includes only records beyond the marker, up to the value specified by
         <code>MaxRecords</code>.</p> |
| `db_clusters` | Vec<String> | <p>A list of clusters.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_clusters outputs
db_clusters_id = db_clusters.id
db_clusters_marker = db_clusters.marker
db_clusters_db_clusters = db_clusters.db_clusters
```

---


### Orderable_db_instance_options

OrderableDBInstanceOptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is specified, the response
         includes only records beyond the marker, up to the value specified by
         <code>MaxRecords</code>.</p> |
| `orderable_db_instance_options` | Vec<String> | <p>The options that are available for a particular orderable instance.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access orderable_db_instance_options outputs
orderable_db_instance_options_id = orderable_db_instance_options.id
orderable_db_instance_options_marker = orderable_db_instance_options.marker
orderable_db_instance_options_orderable_db_instance_options = orderable_db_instance_options.orderable_db_instance_options
```

---


### Db_engine_versions

DBEngineVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is specified, the response
         includes only records beyond the marker, up to the value specified by
         <code>MaxRecords</code>.</p> |
| `db_engine_versions` | Vec<String> | <p>Detailed information about one or more engine versions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_engine_versions outputs
db_engine_versions_id = db_engine_versions.id
db_engine_versions_marker = db_engine_versions.marker
db_engine_versions_db_engine_versions = db_engine_versions.db_engine_versions
```

---


### Db_cluster_parameters

DBClusterParameters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is specified, the response
         includes only records beyond the marker, up to the value specified by
         <code>MaxRecords</code>.</p> |
| `parameters` | Vec<String> | <p>Provides a list of parameters for the cluster parameter group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_cluster_parameters outputs
db_cluster_parameters_id = db_cluster_parameters.id
db_cluster_parameters_marker = db_cluster_parameters.marker
db_cluster_parameters_parameters = db_cluster_parameters.parameters
```

---


### Db_instances

DBInstances resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is specified, the response
         includes only records beyond the marker, up to the value specified by
         <code>MaxRecords</code>.</p> |
| `db_instances` | Vec<String> | <p>Detailed information about one or more instances. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_instances outputs
db_instances_id = db_instances.id
db_instances_marker = db_instances.marker
db_instances_db_instances = db_instances.db_instances
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
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is specified, the response
         includes only records beyond the marker, up to the value specified by
         <code>MaxRecords</code>.</p> |
| `events` | Vec<String> | <p>Detailed information about one or more events. </p> |


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
events_marker = events.marker
events_events = events.events
```

---


### Db_instance

DBInstance resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `availability_zone` | String |  | <p>The Amazon EC2 Availability Zone that the instance is created in. </p>
         <p>Default: A random, system-chosen Availability Zone in the endpoint's Amazon Web Services Region.</p>
         <p>Example: <code>us-east-1d</code>
         </p> |
| `copy_tags_to_snapshot` | bool |  | <p>A value that indicates whether to copy tags from the DB instance to snapshots of the DB instance. By default, tags are not copied.</p> |
| `engine` | String | ✅ | <p>The name of the database engine to be used for this instance.</p>
         <p>Valid value: <code>docdb</code>
         </p> |
| `auto_minor_version_upgrade` | bool |  | <p>This parameter does not apply to Amazon DocumentDB. Amazon DocumentDB does not perform minor version upgrades regardless of the value set.</p>
         <p>Default: <code>false</code>
         </p> |
| `preferred_maintenance_window` | String |  | <p>The time range each week during which system maintenance can occur, in Universal
            Coordinated Time (UTC).</p>
         <p> Format: <code>ddd:hh24:mi-ddd:hh24:mi</code>
         </p>
         <p>The default is a 30-minute window selected at random from an 8-hour block of time for
            each Amazon Web Services Region, occurring on a random day of the week. </p>
         <p>Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun</p>
         <p>Constraints: Minimum 30-minute window.</p> |
| `promotion_tier` | i64 |  | <p>A value that specifies the order in which an Amazon DocumentDB replica is promoted to the
            primary instance after a failure of the existing primary instance.</p>
         <p>Default: 1</p>
         <p>Valid values: 0-15</p> |
| `performance_insights_kms_key_id` | String |  | <p>The KMS key identifier for encryption of Performance Insights
            data.</p>
         <p>The KMS key identifier is the key ARN, key ID, alias ARN, or alias name
            for the KMS key.</p>
         <p>If you do not specify a value for PerformanceInsightsKMSKeyId, then Amazon DocumentDB uses your
            default KMS key. There is a default KMS key for your
            Amazon Web Services account. Your Amazon Web Services account has a different
            default KMS key for each Amazon Web Services region.</p> |
| `enable_performance_insights` | bool |  | <p>A value that indicates whether to enable Performance Insights for the DB Instance. For
            more information, see <a href="https://docs.aws.amazon.com/documentdb/latest/developerguide/performance-insights.html">Using Amazon
                Performance Insights</a>.</p> |
| `db_instance_class` | String | ✅ | <p>The compute and memory capacity of the instance; for example,
                <code>db.r5.large</code>. </p> |
| `ca_certificate_identifier` | String |  | <p>The CA certificate identifier to use for the DB instance's server certificate.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/documentdb/latest/developerguide/ca_cert_rotation.html">Updating Your Amazon DocumentDB TLS 
            Certificates</a> and 
            <a href="https://docs.aws.amazon.com/documentdb/latest/developerguide/security.encryption.ssl.html">
                Encrypting Data in Transit</a> in the <i>Amazon DocumentDB Developer 
                    Guide</i>.</p> |
| `tags` | Vec<String> |  | <p>The tags to be assigned to the instance. You can assign up to
           10 tags to an instance.</p> |
| `db_instance_identifier` | String | ✅ | <p>The instance identifier. This parameter is stored as a lowercase string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>The first character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
         <p>Example: <code>mydbinstance</code>
         </p> |
| `db_cluster_identifier` | String | ✅ | <p>The identifier of the cluster that the instance will belong to.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_instance
db_instance = provider.docdb.Db_instance {
    engine = "value"  # <p>The name of the database engine to be used for this instance.</p>
         <p>Valid value: <code>docdb</code>
         </p>
    db_instance_class = "value"  # <p>The compute and memory capacity of the instance; for example,
                <code>db.r5.large</code>. </p>
    db_instance_identifier = "value"  # <p>The instance identifier. This parameter is stored as a lowercase string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>The first character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
         <p>Example: <code>mydbinstance</code>
         </p>
    db_cluster_identifier = "value"  # <p>The identifier of the cluster that the instance will belong to.</p>
}

```

---


### Event_subscriptions

EventSubscriptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is specified, the response
         includes only records beyond the marker, up to the value specified by
         <code>MaxRecords</code>.</p> |
| `event_subscriptions_list` | Vec<String> | <p>A list of event subscriptions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access event_subscriptions outputs
event_subscriptions_id = event_subscriptions.id
event_subscriptions_marker = event_subscriptions.marker
event_subscriptions_event_subscriptions_list = event_subscriptions.event_subscriptions_list
```

---


### Db_cluster_snapshot

DBClusterSnapshot resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `db_cluster_identifier` | String | ✅ | <p>The identifier of the cluster to create a snapshot for. This
            parameter is not case sensitive.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must match the identifier of an existing
                    <code>DBCluster</code>.</p>
            </li>
         </ul>
         <p>Example: <code>my-cluster</code>
         </p> |
| `db_cluster_snapshot_identifier` | String | ✅ | <p>The identifier of the cluster snapshot. This parameter is stored
            as a lowercase string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>The first character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.
                    </p>
            </li>
         </ul>
         <p>Example: <code>my-cluster-snapshot1</code>
         </p> |
| `tags` | Vec<String> |  | <p>The tags to be assigned to the cluster snapshot.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_cluster_snapshot
db_cluster_snapshot = provider.docdb.Db_cluster_snapshot {
    db_cluster_identifier = "value"  # <p>The identifier of the cluster to create a snapshot for. This
            parameter is not case sensitive.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must match the identifier of an existing
                    <code>DBCluster</code>.</p>
            </li>
         </ul>
         <p>Example: <code>my-cluster</code>
         </p>
    db_cluster_snapshot_identifier = "value"  # <p>The identifier of the cluster snapshot. This parameter is stored
            as a lowercase string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>The first character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.
                    </p>
            </li>
         </ul>
         <p>Example: <code>my-cluster-snapshot1</code>
         </p>
}

```

---


### Global_cluster

GlobalCluster resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `storage_encrypted` | bool |  | <p>The storage encryption setting for the new global cluster. </p> |
| `database_name` | String |  | <p>The name for your database of up to 64 alpha-numeric characters. If you do not provide a name, Amazon DocumentDB will not create a database in the global cluster you are creating.</p> |
| `engine` | String |  | <p>The name of the database engine to be used for this cluster.</p> |
| `global_cluster_identifier` | String | ✅ | <p>The cluster identifier of the new global cluster.</p> |
| `source_db_cluster_identifier` | String |  | <p>The Amazon Resource Name (ARN) to use as the primary cluster of the global cluster. This parameter is optional.</p> |
| `engine_version` | String |  | <p>The engine version of the global cluster.</p> |
| `deletion_protection` | bool |  | <p>The deletion protection setting for the new global cluster. The global cluster can't be deleted when deletion protection is enabled. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create global_cluster
global_cluster = provider.docdb.Global_cluster {
    global_cluster_identifier = "value"  # <p>The cluster identifier of the new global cluster.</p>
}

```

---


### Db_cluster_snapshot_attributes

DBClusterSnapshotAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_cluster_snapshot_attributes_result` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_cluster_snapshot_attributes outputs
db_cluster_snapshot_attributes_id = db_cluster_snapshot_attributes.id
db_cluster_snapshot_attributes_db_cluster_snapshot_attributes_result = db_cluster_snapshot_attributes.db_cluster_snapshot_attributes_result
```

---


### Db_subnet_group

DBSubnetGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `db_subnet_group_description` | String | ✅ | <p>The description for the subnet group.</p> |
| `tags` | Vec<String> |  | <p>The tags to be assigned to the subnet group.</p> |
| `db_subnet_group_name` | String | ✅ | <p>The name for the subnet group. This value is stored as a lowercase string.</p>
         <p>Constraints: Must contain no more than 255 letters, numbers, periods, underscores,
            spaces, or hyphens. Must not be default.</p>
         <p>Example: <code>mySubnetgroup</code>
         </p> |
| `subnet_ids` | Vec<String> | ✅ | <p>The Amazon EC2 subnet IDs for the subnet group.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_subnet_group
db_subnet_group = provider.docdb.Db_subnet_group {
    db_subnet_group_description = "value"  # <p>The description for the subnet group.</p>
    db_subnet_group_name = "value"  # <p>The name for the subnet group. This value is stored as a lowercase string.</p>
         <p>Constraints: Must contain no more than 255 letters, numbers, periods, underscores,
            spaces, or hyphens. Must not be default.</p>
         <p>Example: <code>mySubnetgroup</code>
         </p>
    subnet_ids = "value"  # <p>The Amazon EC2 subnet IDs for the subnet group.</p>
}

```

---


### Db_cluster

DBCluster resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `storage_encrypted` | bool |  | <p>Specifies whether the cluster is encrypted.</p> |
| `engine_version` | String |  | <p>The version number of the database engine to use. The <code>--engine-version</code> will default to the latest major engine version. For production workloads, we recommend explicitly declaring this parameter with the intended major engine version.</p> |
| `serverless_v2_scaling_configuration` | String |  | <p>Contains the scaling configuration of an Amazon DocumentDB Serverless cluster.</p> |
| `db_subnet_group_name` | String |  | <p>A subnet group to associate with this cluster.</p>
         <p>Constraints: Must match the name of an existing
            <code>DBSubnetGroup</code>. Must not be default.</p>
         <p>Example: <code>mySubnetgroup</code>
         </p> |
| `master_user_secret_kms_key_id` | String |  | <p>The Amazon Web Services KMS key identifier to encrypt a secret that is automatically generated and managed in Amazon Web Services Secrets Manager.
            This setting is valid only if the master user password is managed by Amazon DocumentDB in Amazon Web Services Secrets Manager for the DB cluster.</p>
         <p>The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key. 
            To use a KMS key in a different Amazon Web Services account, specify the key ARN or alias ARN.</p>
         <p>If you don't specify <code>MasterUserSecretKmsKeyId</code>, then the <code>aws/secretsmanager</code> KMS key is used to encrypt the secret. 
            If the secret is in a different Amazon Web Services account, then you can't use the <code>aws/secretsmanager</code> KMS key to encrypt the secret, and you must use a customer managed KMS key.</p>
         <p>There is a default KMS key for your Amazon Web Services account. 
            Your Amazon Web Services account has a different default KMS key for each Amazon Web Services Region.</p> |
| `tags` | Vec<String> |  | <p>The tags to be assigned to the cluster.</p> |
| `pre_signed_url` | String |  | <p>Not currently supported.
            </p> |
| `availability_zones` | Vec<String> |  | <p>A list of Amazon EC2 Availability Zones that instances in the
            cluster can be created in.</p> |
| `vpc_security_group_ids` | Vec<String> |  | <p>A list of EC2 VPC security groups to associate with this cluster.
            </p> |
| `global_cluster_identifier` | String |  | <p>The cluster identifier of the new global cluster.</p> |
| `storage_type` | String |  | <p>The storage type to associate with the DB cluster.</p>
         <p>For information on storage types for Amazon DocumentDB clusters, see 
            Cluster storage configurations in the <i>Amazon DocumentDB Developer Guide</i>.</p>
         <p>Valid values for storage type - <code>standard | iopt1</code>
         </p>
         <p>Default value is <code>standard </code>
         </p>
         <note>
            <p>When you create an Amazon DocumentDB cluster with the storage type set to <code>iopt1</code>, the storage type is returned
                    in the response. The storage type isn't returned when you set it to <code>standard</code>.</p>
         </note> |
| `backup_retention_period` | i64 |  | <p>The number of days for which automated backups are retained. You
            must specify a minimum value of 1.</p>
         <p>Default: 1</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be a value from 1 to 35.</p>
            </li>
         </ul> |
| `master_username` | String |  | <p>The name of the master user for the cluster.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be from 1 to 63 letters or numbers.</p>
            </li>
            <li>
               <p>The first character must be a letter.</p>
            </li>
            <li>
               <p>Cannot be a reserved word for the chosen database engine.
                    </p>
            </li>
         </ul> |
| `port` | i64 |  | <p>The port number on which the instances in the cluster accept
            connections.</p> |
| `manage_master_user_password` | bool |  | <p>Specifies whether to manage the master user password with Amazon Web Services Secrets Manager.</p>
         <p>Constraint: You can't manage the master user password with Amazon Web Services Secrets Manager if <code>MasterUserPassword</code> is specified.</p> |
| `master_user_password` | String |  | <p>The password for the master database user. This password can
            contain any printable ASCII character except forward slash (/),
            double quote ("), or the "at" symbol (@).</p>
         <p>Constraints: Must contain from 8 to 100 characters.</p> |
| `db_cluster_parameter_group_name` | String |  | <p>The name of the cluster parameter group to associate with this
            cluster.</p> |
| `preferred_backup_window` | String |  | <p>The daily time range during which automated backups are created if
            automated backups are enabled using the <code>BackupRetentionPeriod</code> parameter. </p>
         <p>The default is a 30-minute window selected at random from an 8-hour block of time for each Amazon Web Services Region. </p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be in the format <code>hh24:mi-hh24:mi</code>.</p>
            </li>
            <li>
               <p>Must be in Universal Coordinated Time (UTC).</p>
            </li>
            <li>
               <p>Must not conflict with the preferred maintenance window.
                    </p>
            </li>
            <li>
               <p>Must be at least 30 minutes.</p>
            </li>
         </ul> |
| `engine` | String | ✅ | <p>The name of the database engine to be used for this cluster.</p>
         <p>Valid values: <code>docdb</code>
         </p> |
| `db_cluster_identifier` | String | ✅ | <p>The cluster identifier. This parameter is stored as a lowercase
            string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 letters, numbers, or hyphens.
                    </p>
            </li>
            <li>
               <p>The first character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.
                    </p>
            </li>
         </ul>
         <p>Example: <code>my-cluster</code>
         </p> |
| `network_type` | String |  | <p>The network type of the cluster.</p>
         <p>The network type is determined by the <code>DBSubnetGroup</code> specified for the cluster. 
            A <code>DBSubnetGroup</code> can support only the IPv4 protocol or the IPv4 and the IPv6 protocols (<code>DUAL</code>).</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/documentdb/latest/developerguide/vpc-clusters.html">DocumentDB clusters in a VPC</a> in the Amazon DocumentDB Developer Guide.</p>
         <p>Valid Values: <code>IPV4</code> | <code>DUAL</code>
         </p> |
| `deletion_protection` | bool |  | <p>Specifies whether this cluster can be deleted. If
            <code>DeletionProtection</code> is enabled, the cluster cannot be
            deleted unless it is modified and <code>DeletionProtection</code> is
            disabled. <code>DeletionProtection</code> protects clusters from
            being accidentally deleted.</p> |
| `kms_key_id` | String |  | <p>The KMS key identifier for an encrypted cluster.</p>
         <p>The KMS key identifier is the Amazon Resource Name (ARN) for the KMS encryption key. If you are creating a cluster using the same Amazon Web Services account that owns the KMS encryption key that is used to encrypt the new cluster, you can use the KMS key alias instead of the ARN for the KMS encryption key.</p>
         <p>If an encryption key is not specified in <code>KmsKeyId</code>:
            </p>
         <ul>
            <li>
               <p>If the <code>StorageEncrypted</code> parameter is
                    <code>true</code>, Amazon DocumentDB uses your default encryption key.
                    </p>
            </li>
         </ul>
         <p>KMS creates the default encryption key for your Amazon Web Services account. Your Amazon Web Services account has a different default encryption key for each Amazon Web Services Regions.</p> |
| `preferred_maintenance_window` | String |  | <p>The weekly time range during which system maintenance can occur,
            in Universal Coordinated Time (UTC).</p>
         <p>Format: <code>ddd:hh24:mi-ddd:hh24:mi</code>
         </p>
         <p>The default is a 30-minute window selected at random from an 8-hour block of time for each Amazon Web Services Region, occurring on a random day of the week.</p>
         <p>Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun</p>
         <p>Constraints: Minimum 30-minute window.</p> |
| `enable_cloudwatch_logs_exports` | Vec<String> |  | <p>A list of log types that need to be enabled for exporting to Amazon
            CloudWatch Logs. You can enable audit logs or profiler logs. For more
            information, see <a href="https://docs.aws.amazon.com/documentdb/latest/developerguide/event-auditing.html">
                Auditing Amazon DocumentDB Events</a>
            and <a href="https://docs.aws.amazon.com/documentdb/latest/developerguide/profiling.html">
                Profiling Amazon DocumentDB Operations</a>.
        </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_cluster
db_cluster = provider.docdb.Db_cluster {
    engine = "value"  # <p>The name of the database engine to be used for this cluster.</p>
         <p>Valid values: <code>docdb</code>
         </p>
    db_cluster_identifier = "value"  # <p>The cluster identifier. This parameter is stored as a lowercase
            string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 letters, numbers, or hyphens.
                    </p>
            </li>
            <li>
               <p>The first character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.
                    </p>
            </li>
         </ul>
         <p>Example: <code>my-cluster</code>
         </p>
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

# Create multiple db_cluster_parameter_groups resources
db_cluster_parameter_groups_0 = provider.docdb.Db_cluster_parameter_groups {
}
db_cluster_parameter_groups_1 = provider.docdb.Db_cluster_parameter_groups {
}
db_cluster_parameter_groups_2 = provider.docdb.Db_cluster_parameter_groups {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    db_cluster_parameter_groups = provider.docdb.Db_cluster_parameter_groups {
    }
```

---

## Related Documentation

- [AWS Docdb Documentation](https://docs.aws.amazon.com/docdb/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
