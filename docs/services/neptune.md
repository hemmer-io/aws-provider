# Neptune Service



**Resources**: 29

---

## Overview

The neptune service provides access to 29 resource types:

- [Db_instance](#db_instance) [CD]
- [Db_subnet_group](#db_subnet_group) [CD]
- [Engine_default_cluster_parameters](#engine_default_cluster_parameters) [R]
- [Orderable_db_instance_options](#orderable_db_instance_options) [R]
- [Db_parameter_group](#db_parameter_group) [CD]
- [Event_subscriptions](#event_subscriptions) [R]
- [Event_categories](#event_categories) [R]
- [Db_cluster_snapshot_attributes](#db_cluster_snapshot_attributes) [R]
- [Pending_maintenance_actions](#pending_maintenance_actions) [R]
- [Db_cluster_endpoints](#db_cluster_endpoints) [R]
- [Valid_db_instance_modifications](#valid_db_instance_modifications) [R]
- [Db_cluster_parameter_group](#db_cluster_parameter_group) [CD]
- [Db_cluster_snapshot](#db_cluster_snapshot) [CD]
- [Events](#events) [R]
- [Global_cluster](#global_cluster) [CD]
- [Db_engine_versions](#db_engine_versions) [R]
- [Global_clusters](#global_clusters) [R]
- [Db_instances](#db_instances) [R]
- [Db_cluster_endpoint](#db_cluster_endpoint) [CD]
- [Db_cluster_parameter_groups](#db_cluster_parameter_groups) [R]
- [Db_cluster_snapshots](#db_cluster_snapshots) [R]
- [Db_subnet_groups](#db_subnet_groups) [R]
- [Db_parameter_groups](#db_parameter_groups) [R]
- [Event_subscription](#event_subscription) [CD]
- [Db_cluster](#db_cluster) [CD]
- [Db_parameters](#db_parameters) [R]
- [Engine_default_parameters](#engine_default_parameters) [R]
- [Db_cluster_parameters](#db_cluster_parameters) [R]
- [Db_clusters](#db_clusters) [R]

---

## Resources


### Db_instance

DBInstance resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_performance_insights` | bool |  | <p>
            <i>(Not supported by Neptune)</i>
         </p> |
| `port` | i64 |  | <p>The port number on which the database accepts connections.</p>
         <p>Not applicable. The port is managed by the DB cluster. For more information, see <a>CreateDBCluster</a>.</p>
         <p> Default: <code>8182</code>
         </p>
         <p>Type: Integer</p> |
| `backup_retention_period` | i64 |  | <p>The number of days for which automated backups are
      retained.</p>
         <p>Not applicable. The retention period for automated backups is managed by the DB cluster.
      For more information, see <a>CreateDBCluster</a>.</p>
         <p>Default: 1</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be a value from 0 to 35</p>
            </li>
            <li>
               <p>Cannot be set to 0 if the DB instance is a source to Read Replicas</p>
            </li>
         </ul> |
| `copy_tags_to_snapshot` | bool |  | <p>True to copy all tags from the DB instance to snapshots of the DB instance, and otherwise
      false. The default is false.</p> |
| `db_parameter_group_name` | String |  | <p>The name of the DB parameter group to associate with this DB instance. If this argument is
      omitted, the default DBParameterGroup for the specified engine is used.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 1 to 255 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens</p>
            </li>
         </ul> |
| `monitoring_role_arn` | String |  | <p>The ARN for the IAM role that permits Neptune to send enhanced monitoring metrics to
      Amazon CloudWatch Logs. For example,
      <code>arn:aws:iam:123456789012:role/emaccess</code>.</p>
         <p>If <code>MonitoringInterval</code> is set to a value other than 0, then you must supply a
      <code>MonitoringRoleArn</code> value.</p> |
| `character_set_name` | String |  | <p>
            <i>(Not supported by Neptune)</i>
         </p> |
| `preferred_backup_window` | String |  | <p> The daily time range during which automated backups are created.</p>
         <p>Not applicable. The daily time range for creating automated backups is managed by the DB
      cluster. For more information, see <a>CreateDBCluster</a>.</p> |
| `db_name` | String |  | <p>Not supported.</p> |
| `db_instance_class` | String | ✅ | <p>The compute and memory capacity of the DB instance, for example, <code>db.m4.large</code>.
      Not all DB instance classes are available in all Amazon Regions.</p> |
| `engine` | String | ✅ | <p>The name of the database engine to be used for this instance.</p>
         <p>Valid Values: <code>neptune</code>
         </p> |
| `master_user_password` | String |  | <p>Not supported by Neptune.</p> |
| `db_security_groups` | Vec<String> |  | <p>A list of DB security groups to associate with this DB instance.</p>
         <p>Default: The default DB security group for the database engine.</p> |
| `preferred_maintenance_window` | String |  | <p>The time range each week during which system maintenance can occur, in Universal
      Coordinated Time (UTC).</p>
         <p> Format: <code>ddd:hh24:mi-ddd:hh24:mi</code>
         </p>
         <p>The default is a 30-minute window selected at random from an 8-hour block of time for each
      Amazon Region, occurring on a random day of the week.</p>
         <p>Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.</p>
         <p>Constraints: Minimum 30-minute window.</p> |
| `license_model` | String |  | <p>License model information for this DB instance.</p>
         <p> Valid values: <code>license-included</code> | <code>bring-your-own-license</code> |
      <code>general-public-license</code>
         </p> |
| `storage_encrypted` | bool |  | <p>Specifies whether the DB instance is encrypted.</p>
         <p>Not applicable. The encryption for DB instances is managed by the DB cluster. For more
      information, see <a>CreateDBCluster</a>.</p>
         <p>Default: false</p> |
| `enable_iam_database_authentication` | bool |  | <p>Not supported by Neptune (ignored).</p> |
| `option_group_name` | String |  | <p>
            <i>(Not supported by Neptune)</i>
         </p> |
| `db_cluster_identifier` | String | ✅ | <p>The identifier of the DB cluster that the instance will belong to.</p>
         <p>For information on creating a DB cluster, see <a>CreateDBCluster</a>.</p>
         <p>Type: String</p> |
| `timezone` | String |  | <p>The time zone of the DB instance.</p> |
| `db_instance_identifier` | String | ✅ | <p>The DB instance identifier. This parameter is stored as a lowercase string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
         <p>Example: <code>mydbinstance</code>
         </p> |
| `engine_version` | String |  | <p>The version number of the database engine to use. Currently, setting this
      parameter has no effect.</p> |
| `vpc_security_group_ids` | Vec<String> |  | <p>A list of EC2 VPC security groups to associate with this DB instance.</p>
         <p>Not applicable. The associated list of EC2 VPC security groups is managed by the DB
      cluster. For more information, see <a>CreateDBCluster</a>.</p>
         <p>Default: The default EC2 VPC security group for the DB subnet group's VPC.</p> |
| `publicly_accessible` | bool |  | <p>Indicates whether the DB instance is publicly accessible.</p>
         <p>When the DB instance is publicly accessible and you connect from outside of the DB instance's virtual private 
      cloud (VPC), its Domain Name System (DNS) endpoint resolves to the public IP address. When you connect from within 
      the same VPC as the DB instance, the endpoint resolves to the private IP address. Access to the DB instance is 
      ultimately controlled by the security group it uses. That public access isn't permitted if the security group assigned 
      to the DB cluster doesn't permit it.</p>
         <p>When the DB instance isn't publicly accessible, it is an internal DB instance with a DNS name that resolves to a 
      private IP address.</p> |
| `domain` | String |  | <p>Specify the Active Directory Domain to create the instance in.</p> |
| `db_subnet_group_name` | String |  | <p>A DB subnet group to associate with this DB instance.</p>
         <p>If there is no DB subnet group, then it is a non-VPC DB instance.</p> |
| `monitoring_interval` | i64 |  | <p>The interval, in seconds, between points when Enhanced Monitoring metrics are collected
      for the DB instance. To disable collecting Enhanced Monitoring metrics, specify 0. The default
      is 0.</p>
         <p>If <code>MonitoringRoleArn</code> is specified, then you must also set
      <code>MonitoringInterval</code> to a value other than 0.</p>
         <p>Valid Values: <code>0, 1, 5, 10, 15, 30, 60</code>
         </p> |
| `performance_insights_kms_key_id` | String |  | <p>
            <i>(Not supported by Neptune)</i>
         </p> |
| `domain_iam_role_name` | String |  | <p>Specify the name of the IAM role to be used when making API calls to the Directory
      Service.</p> |
| `availability_zone` | String |  | <p> The EC2 Availability Zone that the DB instance is created in</p>
         <p>Default: A random, system-chosen Availability Zone in the endpoint's Amazon Region.</p>
         <p> Example: <code>us-east-1d</code>
         </p>
         <p> Constraint: The AvailabilityZone parameter can't be specified if the MultiAZ parameter is
      set to <code>true</code>. The specified Availability Zone must be in the same Amazon Region as
      the current endpoint.</p> |
| `tde_credential_password` | String |  | <p>The password for the given ARN from the key store in order to access the device.</p> |
| `multi_az` | bool |  | <p>Specifies if the DB instance is a Multi-AZ deployment. You can't set the AvailabilityZone
      parameter if the MultiAZ parameter is set to true.</p> |
| `promotion_tier` | i64 |  | <p>A value that specifies the order in which an Read Replica is promoted to the primary
      instance after a failure of the existing primary instance.
      </p>
         <p>Default: 1</p>
         <p>Valid Values: 0 - 15</p> |
| `deletion_protection` | bool |  | <p>A value that indicates whether the DB instance has deletion protection enabled.
      The database can't be deleted when deletion protection is enabled. By default,
      deletion protection is disabled. See <a href="https://docs.aws.amazon.com/neptune/latest/userguide/manage-console-instances-delete.html">Deleting
      a DB Instance</a>.</p>
         <p>DB instances in a DB cluster can be deleted even when deletion
      protection is enabled in their parent DB cluster.</p> |
| `iops` | i64 |  | <p>The amount of Provisioned IOPS (input/output operations per second) to be initially
      allocated for the DB instance.</p> |
| `kms_key_id` | String |  | <p>The Amazon KMS key identifier for an encrypted DB instance.</p>
         <p>The KMS key identifier is the Amazon Resource Name (ARN) for the KMS encryption key. If
      you are creating a DB instance with the same Amazon account that owns the KMS encryption key used
      to encrypt the new DB instance, then you can use the KMS key alias instead of the ARN for the
      KM encryption key.</p>
         <p>Not applicable. The KMS key identifier is managed by the DB cluster. For more information,
      see <a>CreateDBCluster</a>.</p>
         <p>If the <code>StorageEncrypted</code> parameter is true, and you do not specify a value for
      the <code>KmsKeyId</code> parameter, then Amazon Neptune will use your default encryption key.
      Amazon KMS creates the default encryption key for your Amazon account. Your Amazon account has a
      different default encryption key for each Amazon Region.</p> |
| `enable_cloudwatch_logs_exports` | Vec<String> |  | <p>The list of log types that need to be enabled for exporting to CloudWatch Logs.</p> |
| `tags` | Vec<String> |  | <p>The tags to assign to the new instance.</p> |
| `allocated_storage` | i64 |  | <p>Not supported by Neptune.</p> |
| `auto_minor_version_upgrade` | bool |  | <p>Indicates that minor engine upgrades are applied automatically to the DB instance during
      the maintenance window.</p>
         <p>Default: <code>true</code>
         </p> |
| `tde_credential_arn` | String |  | <p>The ARN from the key store with which to associate the instance for TDE encryption.</p> |
| `master_username` | String |  | <p>Not supported by Neptune.</p> |
| `storage_type` | String |  | <p>Not applicable. In Neptune the storage type is managed at the DB Cluster level.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_instance
db_instance = provider.neptune.Db_instance {
    db_instance_class = "value"  # <p>The compute and memory capacity of the DB instance, for example, <code>db.m4.large</code>.
      Not all DB instance classes are available in all Amazon Regions.</p>
    engine = "value"  # <p>The name of the database engine to be used for this instance.</p>
         <p>Valid Values: <code>neptune</code>
         </p>
    db_cluster_identifier = "value"  # <p>The identifier of the DB cluster that the instance will belong to.</p>
         <p>For information on creating a DB cluster, see <a>CreateDBCluster</a>.</p>
         <p>Type: String</p>
    db_instance_identifier = "value"  # <p>The DB instance identifier. This parameter is stored as a lowercase string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
         <p>Example: <code>mydbinstance</code>
         </p>
}

```

---


### Db_subnet_group

DBSubnetGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `db_subnet_group_name` | String | ✅ | <p>The name for the DB subnet group. This value is stored as a lowercase string.</p>
         <p>Constraints: Must contain no more than 255 letters, numbers, periods, underscores, spaces,
      or hyphens. Must not be default.</p>
         <p>Example: <code>mySubnetgroup</code>
         </p> |
| `db_subnet_group_description` | String | ✅ | <p>The description for the DB subnet group.</p> |
| `subnet_ids` | Vec<String> | ✅ | <p>The EC2 Subnet IDs for the DB subnet group.</p> |
| `tags` | Vec<String> |  | <p>The tags to be assigned to the new DB subnet group.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_subnet_group
db_subnet_group = provider.neptune.Db_subnet_group {
    db_subnet_group_name = "value"  # <p>The name for the DB subnet group. This value is stored as a lowercase string.</p>
         <p>Constraints: Must contain no more than 255 letters, numbers, periods, underscores, spaces,
      or hyphens. Must not be default.</p>
         <p>Example: <code>mySubnetgroup</code>
         </p>
    db_subnet_group_description = "value"  # <p>The description for the DB subnet group.</p>
    subnet_ids = "value"  # <p>The EC2 Subnet IDs for the DB subnet group.</p>
}

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


### Orderable_db_instance_options

OrderableDBInstanceOptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `orderable_db_instance_options` | Vec<String> | <p>An <a>OrderableDBInstanceOption</a> structure
      containing information about orderable options for the DB instance.</p> |
| `marker` | String | <p> An optional pagination token provided by a previous OrderableDBInstanceOptions request.
      If this parameter is specified, the response includes only records beyond the marker, up to
      the value specified by <code>MaxRecords</code> .</p> |


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
orderable_db_instance_options_orderable_db_instance_options = orderable_db_instance_options.orderable_db_instance_options
orderable_db_instance_options_marker = orderable_db_instance_options.marker
```

---


### Db_parameter_group

DBParameterGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `db_parameter_group_family` | String | ✅ | <p>The DB parameter group family name. A DB parameter group can be associated with one and
      only one DB parameter group family, and can be applied only to a DB instance running a
      database engine and engine version compatible with that DB parameter group family.</p> |
| `description` | String | ✅ | <p>The description for the DB parameter group.</p> |
| `tags` | Vec<String> |  | <p>The tags to be assigned to the new DB parameter group.</p> |
| `db_parameter_group_name` | String | ✅ | <p>The name of the DB parameter group.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 1 to 255 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens</p>
            </li>
         </ul>
         <note>
            <p>This value is stored as a lowercase string.</p>
         </note> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_parameter_group
db_parameter_group = provider.neptune.Db_parameter_group {
    db_parameter_group_family = "value"  # <p>The DB parameter group family name. A DB parameter group can be associated with one and
      only one DB parameter group family, and can be applied only to a DB instance running a
      database engine and engine version compatible with that DB parameter group family.</p>
    description = "value"  # <p>The description for the DB parameter group.</p>
    db_parameter_group_name = "value"  # <p>The name of the DB parameter group.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 1 to 255 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens</p>
            </li>
         </ul>
         <note>
            <p>This value is stored as a lowercase string.</p>
         </note>
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
| `marker` | String | <p> An optional pagination token provided by a previous DescribeOrderableDBInstanceOptions
      request. If this parameter is specified, the response includes only records beyond the marker,
      up to the value specified by <code>MaxRecords</code>.</p> |
| `event_subscriptions_list` | Vec<String> | <p>A list of EventSubscriptions data types.</p> |


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


### Event_categories

EventCategories resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_categories_map_list` | Vec<String> | <p>A list of EventCategoriesMap data types.</p> |


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


### Pending_maintenance_actions

PendingMaintenanceActions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p> An optional pagination token provided by a previous
      <code>DescribePendingMaintenanceActions</code> request. If this parameter is specified, the
      response includes only records beyond the marker, up to a number of records specified by
      <code>MaxRecords</code>.</p> |
| `pending_maintenance_actions` | Vec<String> | <p>A list of the pending maintenance actions for the resource.</p> |


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


### Db_cluster_endpoints

DBClusterEndpoints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_cluster_endpoints` | Vec<String> | <p>Contains the details of the endpoints associated with the cluster
      and matching any filter conditions.</p> |
| `marker` | String | <p> n optional pagination token provided by a previous
      <code>DescribeDBClusterEndpoints</code> request.
      If this parameter is specified, the response includes
      only records beyond the marker,
      up to the value specified by <code>MaxRecords</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_cluster_endpoints outputs
db_cluster_endpoints_id = db_cluster_endpoints.id
db_cluster_endpoints_db_cluster_endpoints = db_cluster_endpoints.db_cluster_endpoints
db_cluster_endpoints_marker = db_cluster_endpoints.marker
```

---


### Valid_db_instance_modifications

ValidDBInstanceModifications resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `valid_db_instance_modifications_message` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access valid_db_instance_modifications outputs
valid_db_instance_modifications_id = valid_db_instance_modifications.id
valid_db_instance_modifications_valid_db_instance_modifications_message = valid_db_instance_modifications.valid_db_instance_modifications_message
```

---


### Db_cluster_parameter_group

DBClusterParameterGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The tags to be assigned to the new DB cluster parameter group.</p> |
| `db_cluster_parameter_group_name` | String | ✅ | <p>The name of the DB cluster parameter group.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must match the name of an existing DBClusterParameterGroup.</p>
            </li>
         </ul>
         <note>
            <p>This value is stored as a lowercase string.</p>
         </note> |
| `db_parameter_group_family` | String | ✅ | <p>The DB cluster parameter group family name. A DB cluster parameter group can be associated
      with one and only one DB cluster parameter group family, and can be applied only to a DB
      cluster running a database engine and engine version compatible with that DB cluster parameter
      group family.</p> |
| `description` | String | ✅ | <p>The description for the DB cluster parameter group.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_cluster_parameter_group
db_cluster_parameter_group = provider.neptune.Db_cluster_parameter_group {
    db_cluster_parameter_group_name = "value"  # <p>The name of the DB cluster parameter group.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must match the name of an existing DBClusterParameterGroup.</p>
            </li>
         </ul>
         <note>
            <p>This value is stored as a lowercase string.</p>
         </note>
    db_parameter_group_family = "value"  # <p>The DB cluster parameter group family name. A DB cluster parameter group can be associated
      with one and only one DB cluster parameter group family, and can be applied only to a DB
      cluster running a database engine and engine version compatible with that DB cluster parameter
      group family.</p>
    description = "value"  # <p>The description for the DB cluster parameter group.</p>
}

```

---


### Db_cluster_snapshot

DBClusterSnapshot resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `db_cluster_identifier` | String | ✅ | <p>The identifier of the DB cluster to create a snapshot for. This parameter is not
      case-sensitive.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must match the identifier of an existing DBCluster.</p>
            </li>
         </ul>
         <p>Example: <code>my-cluster1</code>
         </p> |
| `tags` | Vec<String> |  | <p>The tags to be assigned to the DB cluster snapshot.</p> |
| `db_cluster_snapshot_identifier` | String | ✅ | <p>The identifier of the DB cluster snapshot. This parameter is stored as a lowercase
      string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
         <p>Example: <code>my-cluster1-snapshot1</code>
         </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_cluster_snapshot
db_cluster_snapshot = provider.neptune.Db_cluster_snapshot {
    db_cluster_identifier = "value"  # <p>The identifier of the DB cluster to create a snapshot for. This parameter is not
      case-sensitive.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must match the identifier of an existing DBCluster.</p>
            </li>
         </ul>
         <p>Example: <code>my-cluster1</code>
         </p>
    db_cluster_snapshot_identifier = "value"  # <p>The identifier of the DB cluster snapshot. This parameter is stored as a lowercase
      string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
         <p>Example: <code>my-cluster1-snapshot1</code>
         </p>
}

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
| `events` | Vec<String> | <p> A list of <a>Event</a> instances.</p> |
| `marker` | String | <p> An optional pagination token provided by a previous Events request. If this parameter is
      specified, the response includes only records beyond the marker, up to the value specified by
      <code>MaxRecords</code> .</p> |


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
events_marker = events.marker
```

---


### Global_cluster

GlobalCluster resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `engine` | String |  | <p>The name of the database engine to be used in the global database.</p>
         <p>Valid values: <code>neptune</code>
         </p> |
| `global_cluster_identifier` | String | ✅ | <p>The cluster identifier of the new global database cluster.</p> |
| `engine_version` | String |  | <p>The Neptune engine version to be used by the global database.</p>
         <p>Valid values: <code>1.2.0.0</code> or above.</p> |
| `source_db_cluster_identifier` | String |  | <p>(<i>Optional</i>) The Amazon Resource Name (ARN) of
      an existing Neptune DB cluster to use as the primary cluster of the new
      global database.</p> |
| `deletion_protection` | bool |  | <p>The deletion protection setting for the new global database.
      The global database can't be deleted when deletion protection is
      enabled.</p> |
| `storage_encrypted` | bool |  | <p>The storage encryption setting for the new global database
      cluster.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create global_cluster
global_cluster = provider.neptune.Global_cluster {
    global_cluster_identifier = "value"  # <p>The cluster identifier of the new global database cluster.</p>
}

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
| `marker` | String | <p> An optional pagination token provided by a previous request. If this parameter is
      specified, the response includes only records beyond the marker, up to the value specified by
      <code>MaxRecords</code>.</p> |
| `db_engine_versions` | Vec<String> | <p> A list of <code>DBEngineVersion</code> elements.</p> |


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


### Global_clusters

GlobalClusters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `global_clusters` | Vec<String> | <p>The list of global clusters and instances returned by this request.</p> |
| `marker` | String | <p>A pagination token. If this parameter is returned in the response,
      more records are available, which can be retrieved by one or more additional
      calls to <code>DescribeGlobalClusters</code>.</p> |


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


### Db_instances

DBInstances resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p> An optional pagination token provided by a previous request. If this parameter is
      specified, the response includes only records beyond the marker, up to the value specified by
      <code>MaxRecords</code> .</p> |
| `db_instances` | Vec<String> | <p> A list of <a>DBInstance</a> instances.</p> |


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


### Db_cluster_endpoint

DBClusterEndpoint resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The tags to be assigned to the Amazon Neptune resource.</p> |
| `endpoint_type` | String | ✅ | <p>The type of the endpoint. One of: <code>READER</code>, <code>WRITER</code>, <code>ANY</code>.</p> |
| `db_cluster_identifier` | String | ✅ | <p>The DB cluster identifier of the DB cluster associated with the endpoint. This parameter is
      stored as a lowercase string.</p> |
| `excluded_members` | String |  | <p>List of DB instance identifiers that aren't part of the custom endpoint group.
      All other eligible instances are reachable through the custom endpoint.
      Only relevant if the list of static members is empty.</p> |
| `static_members` | String |  | <p>List of DB instance identifiers that are part of the custom endpoint group.</p> |
| `db_cluster_endpoint_identifier` | String | ✅ | <p>The identifier to use for the new endpoint. This parameter is stored as a lowercase string.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_cluster_endpoint
db_cluster_endpoint = provider.neptune.Db_cluster_endpoint {
    endpoint_type = "value"  # <p>The type of the endpoint. One of: <code>READER</code>, <code>WRITER</code>, <code>ANY</code>.</p>
    db_cluster_identifier = "value"  # <p>The DB cluster identifier of the DB cluster associated with the endpoint. This parameter is
      stored as a lowercase string.</p>
    db_cluster_endpoint_identifier = "value"  # <p>The identifier to use for the new endpoint. This parameter is stored as a lowercase string.</p>
}

```

---


### Db_cluster_parameter_groups

DBClusterParameterGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_cluster_parameter_groups` | Vec<String> | <p>A list of DB cluster parameter groups.</p> |
| `marker` | String | <p> An optional pagination token provided by a previous
      <code>DescribeDBClusterParameterGroups</code> request. If this parameter is specified, the
      response includes only records beyond the marker, up to the value specified by
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
| `db_cluster_snapshots` | Vec<String> | <p>Provides a list of DB cluster snapshots for the user.</p> |
| `marker` | String | <p> An optional pagination token provided by a previous <a>DescribeDBClusterSnapshots</a> request. If this parameter is specified, the response
      includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.
   </p> |


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
| `db_subnet_groups` | Vec<String> | <p> A list of <a>DBSubnetGroup</a> instances.</p> |
| `marker` | String | <p> An optional pagination token provided by a previous request. If this parameter is
      specified, the response includes only records beyond the marker, up to the value specified by
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


### Db_parameter_groups

DBParameterGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_parameter_groups` | Vec<String> | <p>A list of <a>DBParameterGroup</a> instances.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is
      specified, the response includes only records beyond the marker, up to the value specified by
      <code>MaxRecords</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_parameter_groups outputs
db_parameter_groups_id = db_parameter_groups.id
db_parameter_groups_db_parameter_groups = db_parameter_groups.db_parameter_groups
db_parameter_groups_marker = db_parameter_groups.marker
```

---


### Event_subscription

EventSubscription resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enabled` | bool |  | <p> A Boolean value; set to <b>true</b> to activate the
      subscription, set to <b>false</b> to create the subscription but not
      active it.</p> |
| `subscription_name` | String | ✅ | <p>The name of the subscription.</p>
         <p>Constraints: The name must be less than 255 characters.</p> |
| `sns_topic_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the SNS topic created for event notification. The ARN is
      created by Amazon SNS when you create a topic and subscribe to it.</p> |
| `event_categories` | Vec<String> |  | <p> A list of event categories for a SourceType that you want to subscribe to. You can see a
      list of the categories for a given SourceType by using the
      <b>DescribeEventCategories</b> action.</p> |
| `source_type` | String |  | <p>The type of source that is generating the events. For example, if you want to be notified
      of events generated by a DB instance, you would set this parameter to db-instance. if this
      value is not specified, all events are returned.</p>
         <p>Valid values: <code>db-instance</code> | <code>db-cluster</code> |
      <code>db-parameter-group</code> | <code>db-security-group</code> | <code>db-snapshot</code> |
      <code>db-cluster-snapshot</code>
         </p> |
| `source_ids` | Vec<String> |  | <p>The list of identifiers of the event sources for which events are returned. If not
      specified, then all sources are included in the response. An identifier must begin with a
      letter and must contain only ASCII letters, digits, and hyphens; it can't end with a hyphen or
      contain two consecutive hyphens.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>If SourceIds are supplied, SourceType must also be provided.</p>
            </li>
            <li>
               <p>If the source type is a DB instance, then a <code>DBInstanceIdentifier</code> must be
          supplied.</p>
            </li>
            <li>
               <p>If the source type is a DB security group, a <code>DBSecurityGroupName</code> must be
          supplied.</p>
            </li>
            <li>
               <p>If the source type is a DB parameter group, a <code>DBParameterGroupName</code> must
          be supplied.</p>
            </li>
            <li>
               <p>If the source type is a DB snapshot, a <code>DBSnapshotIdentifier</code> must be
          supplied.</p>
            </li>
         </ul> |
| `tags` | Vec<String> |  | <p>The tags to be applied to the new event subscription.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_subscription
event_subscription = provider.neptune.Event_subscription {
    subscription_name = "value"  # <p>The name of the subscription.</p>
         <p>Constraints: The name must be less than 255 characters.</p>
    sns_topic_arn = "value"  # <p>The Amazon Resource Name (ARN) of the SNS topic created for event notification. The ARN is
      created by Amazon SNS when you create a topic and subscribe to it.</p>
}

```

---


### Db_cluster

DBCluster resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deletion_protection` | bool |  | <p>A value that indicates whether the DB cluster has deletion protection enabled.
      The database can't be deleted when deletion protection is enabled. By default,
      deletion protection is enabled.</p> |
| `preferred_backup_window` | String |  | <p>The daily time range during which automated backups are created if automated backups are
      enabled using the <code>BackupRetentionPeriod</code> parameter.</p>
         <p>The default is a 30-minute window selected at random from an 8-hour block of time for each
      Amazon Region. To see the time blocks available, see <a href="https://docs.aws.amazon.com/neptune/latest/userguide/manage-console-maintaining.html#manage-console-maintaining-window">Neptune
      Maintenance Window</a> in the <i>Amazon Neptune User Guide.</i>
         </p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be in the format <code>hh24:mi-hh24:mi</code>.</p>
            </li>
            <li>
               <p>Must be in Universal Coordinated Time (UTC).</p>
            </li>
            <li>
               <p>Must not conflict with the preferred maintenance window.</p>
            </li>
            <li>
               <p>Must be at least 30 minutes.</p>
            </li>
         </ul> |
| `port` | i64 |  | <p>The port number on which the instances in the DB cluster accept connections.</p>
         <p> Default: <code>8182</code>
         </p> |
| `availability_zones` | Vec<String> |  | <p>A list of EC2 Availability Zones that instances in the DB cluster can be created
      in.</p> |
| `tags` | Vec<String> |  | <p>The tags to assign to the new DB cluster.</p> |
| `master_user_password` | String |  | <p>Not supported by Neptune.</p> |
| `database_name` | String |  | <p>The name for your database of up to 64 alpha-numeric characters. If you do not provide a
      name, Amazon Neptune will not create a database in the DB cluster you are creating.</p> |
| `kms_key_id` | String |  | <p>The Amazon KMS key identifier for an encrypted DB cluster.</p>
         <p>The KMS key identifier is the Amazon Resource Name (ARN) for the KMS encryption key. If
      you are creating a DB cluster with the same Amazon account that owns the KMS encryption key used
      to encrypt the new DB cluster, then you can use the KMS key alias instead of the ARN for the
      KMS encryption key.</p>
         <p>If an encryption key is not specified in <code>KmsKeyId</code>:</p>
         <ul>
            <li>
               <p>If <code>ReplicationSourceIdentifier</code> identifies an encrypted source, then
          Amazon Neptune will use the encryption key used to encrypt the source. Otherwise, Amazon
          Neptune will use your default encryption key.</p>
            </li>
            <li>
               <p>If the <code>StorageEncrypted</code> parameter is true and
          <code>ReplicationSourceIdentifier</code> is not specified, then Amazon Neptune will use
          your default encryption key.</p>
            </li>
         </ul>
         <p>Amazon KMS creates the default encryption key for your Amazon account. Your Amazon account has a
      different default encryption key for each Amazon Region.</p>
         <p>If you create a Read Replica of an encrypted DB cluster in another Amazon Region, you must
      set <code>KmsKeyId</code> to a KMS key ID that is valid in the destination Amazon Region. This
      key is used to encrypt the Read Replica in that Amazon Region.</p> |
| `enable_iam_database_authentication` | bool |  | <p>If set to <code>true</code>, enables Amazon Identity and Access Management
      (IAM) authentication for the entire DB cluster (this cannot be set at an
      instance level).</p>
         <p>Default: <code>false</code>.</p> |
| `pre_signed_url` | String |  | <p>This parameter is not currently supported.</p> |
| `global_cluster_identifier` | String |  | <p>The ID of the Neptune global database to which this new DB cluster
      should be added.</p> |
| `storage_type` | String |  | <p>The storage type for the new DB cluster.</p>
         <p>Valid Values:</p>
         <ul>
            <li>
               <p>
                  <b>
                     <code>standard</code>
                  </b>   –  
        ( <i>the default</i> ) Configures cost-effective database storage for applications
        with moderate to small I/O usage. When set to <code>standard</code>, the storage type
        is not returned in the response.</p>
            </li>
            <li>
               <p>
                  <b>
                     <code>iopt1</code>
                  </b>   –  
          Enables <a href="https://docs.aws.amazon.com/neptune/latest/userguide/storage-types.html#provisioned-iops-storage">I/O-Optimized storage</a>
          that's designed to meet the needs of I/O-intensive graph workloads that
          require predictable pricing with low I/O latency and consistent I/O throughput.</p>
               <p>Neptune I/O-Optimized storage is only available starting with engine release 1.3.0.0.</p>
            </li>
         </ul> |
| `copy_tags_to_snapshot` | bool |  | <p>
            <i>If set to <code>true</code>, tags are copied to any snapshot of
      the DB cluster that is created.</i>
         </p> |
| `master_username` | String |  | <p>Not supported by Neptune.</p> |
| `engine` | String | ✅ | <p>The name of the database engine to be used for this DB cluster.</p>
         <p>Valid Values: <code>neptune</code>
         </p> |
| `storage_encrypted` | bool |  | <p>Specifies whether the DB cluster is encrypted.</p> |
| `db_cluster_identifier` | String | ✅ | <p>The DB cluster identifier. This parameter is stored as a lowercase string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
         <p>Example: <code>my-cluster1</code>
         </p> |
| `vpc_security_group_ids` | Vec<String> |  | <p>A list of EC2 VPC security groups to associate with this DB cluster.</p> |
| `serverless_v2_scaling_configuration` | String |  | <p>Contains the scaling configuration of a Neptune Serverless DB cluster.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/neptune/latest/userguide/neptune-serverless-using.html">Using Amazon Neptune Serverless</a> in the
      <i>Amazon Neptune User Guide</i>.</p> |
| `backup_retention_period` | i64 |  | <p>The number of days for which automated backups are retained. You must specify a minimum
      value of 1.</p>
         <p>Default: 1</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be a value from 1 to 35</p>
            </li>
         </ul> |
| `engine_version` | String |  | <p>The version number of the database engine to use for the new DB cluster.</p>
         <p>Example: <code>1.2.1.0</code>
         </p> |
| `option_group_name` | String |  | <p>
            <i>(Not supported by Neptune)</i>
         </p> |
| `db_subnet_group_name` | String |  | <p>A DB subnet group to associate with this DB cluster.</p>
         <p>Constraints: Must match the name of an existing DBSubnetGroup. Must not be default.</p>
         <p>Example: <code>mySubnetgroup</code>
         </p> |
| `character_set_name` | String |  | <p>
            <i>(Not supported by Neptune)</i>
         </p> |
| `replication_source_identifier` | String |  | <p>The Amazon Resource Name (ARN) of the source DB instance or DB cluster if this DB cluster
      is created as a Read Replica.</p> |
| `enable_cloudwatch_logs_exports` | Vec<String> |  | <p>A list of the log types that this DB cluster should export to CloudWatch Logs.
      Valid log types are: <code>audit</code> (to publish audit logs) and
      <code>slowquery</code> (to publish slow-query logs). See <a href="https://docs.aws.amazon.com/neptune/latest/userguide/cloudwatch-logs.html">Publishing Neptune logs
      to Amazon CloudWatch logs</a>.</p> |
| `preferred_maintenance_window` | String |  | <p>The weekly time range during which system maintenance can occur, in Universal Coordinated
      Time (UTC).</p>
         <p>Format: <code>ddd:hh24:mi-ddd:hh24:mi</code>
         </p>
         <p>The default is a 30-minute window selected at random from an 8-hour block of time for each
      Amazon Region, occurring on a random day of the week. To see the time blocks available, see <a href="https://docs.aws.amazon.com/neptune/latest/userguide/manage-console-maintaining.html#manage-console-maintaining-window">Neptune
      Maintenance Window</a> in the <i>Amazon Neptune User Guide.</i>
         </p>
         <p>Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.</p>
         <p>Constraints: Minimum 30-minute window.</p> |
| `db_cluster_parameter_group_name` | String |  | <p> The name of the DB cluster parameter group to associate with this DB cluster. If this
      argument is omitted, the default is used.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>If supplied, must match the name of an existing DBClusterParameterGroup.</p>
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

# Create db_cluster
db_cluster = provider.neptune.Db_cluster {
    engine = "value"  # <p>The name of the database engine to be used for this DB cluster.</p>
         <p>Valid Values: <code>neptune</code>
         </p>
    db_cluster_identifier = "value"  # <p>The DB cluster identifier. This parameter is stored as a lowercase string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
         <p>Example: <code>my-cluster1</code>
         </p>
}

```

---


### Db_parameters

DBParameters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parameters` | Vec<String> | <p>A list of <a>Parameter</a> values.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is
      specified, the response includes only records beyond the marker, up to the value specified by
      <code>MaxRecords</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_parameters outputs
db_parameters_id = db_parameters.id
db_parameters_parameters = db_parameters.parameters
db_parameters_marker = db_parameters.marker
```

---


### Engine_default_parameters

EngineDefaultParameters resource

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

# Access engine_default_parameters outputs
engine_default_parameters_id = engine_default_parameters.id
engine_default_parameters_engine_defaults = engine_default_parameters.engine_defaults
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
| `marker` | String | <p> An optional pagination token provided by a previous DescribeDBClusterParameters request.
      If this parameter is specified, the response includes only records beyond the marker, up to
      the value specified by <code>MaxRecords</code> .</p> |
| `parameters` | Vec<String> | <p>Provides a list of parameters for the DB cluster parameter group.</p> |


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


### Db_clusters

DBClusters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>A pagination token that can be used in a subsequent DescribeDBClusters request.</p> |
| `db_clusters` | Vec<String> | <p>Contains a list of DB clusters for the user.</p> |


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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple db_instance resources
db_instance_0 = provider.neptune.Db_instance {
    db_instance_class = "value-0"
    engine = "value-0"
    db_cluster_identifier = "value-0"
    db_instance_identifier = "value-0"
}
db_instance_1 = provider.neptune.Db_instance {
    db_instance_class = "value-1"
    engine = "value-1"
    db_cluster_identifier = "value-1"
    db_instance_identifier = "value-1"
}
db_instance_2 = provider.neptune.Db_instance {
    db_instance_class = "value-2"
    engine = "value-2"
    db_cluster_identifier = "value-2"
    db_instance_identifier = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    db_instance = provider.neptune.Db_instance {
        db_instance_class = "production-value"
        engine = "production-value"
        db_cluster_identifier = "production-value"
        db_instance_identifier = "production-value"
    }
```

---

## Related Documentation

- [AWS Neptune Documentation](https://docs.aws.amazon.com/neptune/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
