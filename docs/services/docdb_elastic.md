# Docdb_elastic Service



**Resources**: 3

---

## Overview

The docdb_elastic service provides access to 3 resource types:

- [Cluster_snapshot](#cluster_snapshot) [CRD]
- [Cluster](#cluster) [CRUD]
- [Pending_maintenance_action](#pending_maintenance_action) [R]

---

## Resources


### Cluster_snapshot

ClusterSnapshot resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>The tags to be assigned to the new elastic cluster snapshot.</p> |
| `snapshot_name` | String | ✅ | <p>The name of the new elastic cluster snapshot.</p> |
| `cluster_arn` | String | ✅ | <p>The ARN identifier of the elastic cluster of which you want to create a snapshot.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `snapshot` | String | <p>Returns information about a specific elastic cluster snapshot.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster_snapshot
cluster_snapshot = provider.docdb_elastic.Cluster_snapshot {
    snapshot_name = "value"  # <p>The name of the new elastic cluster snapshot.</p>
    cluster_arn = "value"  # <p>The ARN identifier of the elastic cluster of which you want to create a snapshot.</p>
}

# Access cluster_snapshot outputs
cluster_snapshot_id = cluster_snapshot.id
cluster_snapshot_snapshot = cluster_snapshot.snapshot
```

---


### Cluster

Cluster resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vpc_security_group_ids` | String |  | <p>A list of EC2 VPC security groups to associate with the new
      elastic cluster.</p> |
| `admin_user_password` | String | ✅ | <p>The password for the Amazon DocumentDB elastic clusters administrator. The password can contain any printable ASCII characters.</p>
         <p>
            <i>Constraints</i>:</p>
         <ul>
            <li>
               <p>Must contain from 8 to 100 characters.</p>
            </li>
            <li>
               <p>Cannot contain a forward slash (/), double quote ("), or the "at" symbol (@).</p>
            </li>
         </ul> |
| `auth_type` | String | ✅ | <p>The authentication type used to determine where to fetch the password used for accessing the elastic cluster. 
      Valid types are <code>PLAIN_TEXT</code> or <code>SECRET_ARN</code>.</p> |
| `kms_key_id` | String |  | <p>The KMS key identifier to use to encrypt the new elastic cluster.</p>
         <p>The KMS key identifier is the Amazon Resource Name (ARN) for the KMS
        encryption key. If you are creating a cluster using the same Amazon account
        that owns this KMS encryption key, you can use the KMS key alias instead
        of the ARN as the KMS encryption key.</p>
         <p>If an encryption key is not specified, Amazon DocumentDB uses the
        default encryption key that KMS creates for your account. Your account
        has a different default encryption key for each Amazon Region.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to be assigned to the new elastic cluster.</p> |
| `shard_count` | i64 | ✅ | <p>The number of shards assigned to the elastic cluster. Maximum is 32.</p> |
| `preferred_backup_window` | String |  | <p>The daily time range during which automated backups are created if automated backups are enabled, as determined by the <code>backupRetentionPeriod</code>.</p> |
| `preferred_maintenance_window` | String |  | <p>The weekly time range during which system maintenance can occur,
      in Universal Coordinated Time (UTC).</p>
         <p>
            <i>Format</i>: <code>ddd:hh24:mi-ddd:hh24:mi</code>
         </p>
         <p>
            <i>Default</i>: a 30-minute window selected at random from an 8-hour block of time for each Amazon Web Services Region, occurring on a random day of the week.</p>
         <p>
            <i>Valid days</i>: Mon, Tue, Wed, Thu, Fri, Sat, Sun</p>
         <p>
            <i>Constraints</i>: Minimum 30-minute window.</p> |
| `admin_user_name` | String | ✅ | <p>The name of the Amazon DocumentDB elastic clusters administrator.</p>
         <p>
            <i>Constraints</i>:</p>
         <ul>
            <li>
               <p>Must be from 1 to 63 letters or numbers.</p>
            </li>
            <li>
               <p>The first character must be a letter.</p>
            </li>
            <li>
               <p>Cannot be a reserved word.</p>
            </li>
         </ul> |
| `shard_capacity` | i64 | ✅ | <p>The number of vCPUs assigned to each elastic cluster shard. Maximum is 64. Allowed values are 2, 4, 8, 16, 32, 64.</p> |
| `cluster_name` | String | ✅ | <p>The name of the new elastic cluster. This parameter is stored as
      a lowercase string.</p>
         <p>
            <i>Constraints</i>:</p>
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
         <p>
            <i>Example</i>: <code>my-cluster</code>
         </p> |
| `subnet_ids` | String |  | <p>The Amazon EC2 subnet IDs for the new elastic cluster.</p> |
| `shard_instance_count` | i64 |  | <p>The number of replica instances applying to all shards in the elastic cluster. 
      A <code>shardInstanceCount</code> value of 1 means there is one writer instance, and any additional instances are replicas that can be used for reads and to improve availability.</p> |
| `backup_retention_period` | i64 |  | <p>The number of days for which automatic snapshots are retained.</p> |
| `client_token` | String |  | <p>The client token for the elastic cluster.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cluster` | String | <p>Returns information about a specific elastic cluster.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster
cluster = provider.docdb_elastic.Cluster {
    admin_user_password = "value"  # <p>The password for the Amazon DocumentDB elastic clusters administrator. The password can contain any printable ASCII characters.</p>
         <p>
            <i>Constraints</i>:</p>
         <ul>
            <li>
               <p>Must contain from 8 to 100 characters.</p>
            </li>
            <li>
               <p>Cannot contain a forward slash (/), double quote ("), or the "at" symbol (@).</p>
            </li>
         </ul>
    auth_type = "value"  # <p>The authentication type used to determine where to fetch the password used for accessing the elastic cluster. 
      Valid types are <code>PLAIN_TEXT</code> or <code>SECRET_ARN</code>.</p>
    shard_count = "value"  # <p>The number of shards assigned to the elastic cluster. Maximum is 32.</p>
    admin_user_name = "value"  # <p>The name of the Amazon DocumentDB elastic clusters administrator.</p>
         <p>
            <i>Constraints</i>:</p>
         <ul>
            <li>
               <p>Must be from 1 to 63 letters or numbers.</p>
            </li>
            <li>
               <p>The first character must be a letter.</p>
            </li>
            <li>
               <p>Cannot be a reserved word.</p>
            </li>
         </ul>
    shard_capacity = "value"  # <p>The number of vCPUs assigned to each elastic cluster shard. Maximum is 64. Allowed values are 2, 4, 8, 16, 32, 64.</p>
    cluster_name = "value"  # <p>The name of the new elastic cluster. This parameter is stored as
      a lowercase string.</p>
         <p>
            <i>Constraints</i>:</p>
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
         <p>
            <i>Example</i>: <code>my-cluster</code>
         </p>
}

# Access cluster outputs
cluster_id = cluster.id
cluster_cluster = cluster.cluster
```

---


### Pending_maintenance_action

PendingMaintenanceAction resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_pending_maintenance_action` | String | <p>Provides information about a pending maintenance action for a resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access pending_maintenance_action outputs
pending_maintenance_action_id = pending_maintenance_action.id
pending_maintenance_action_resource_pending_maintenance_action = pending_maintenance_action.resource_pending_maintenance_action
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple cluster_snapshot resources
cluster_snapshot_0 = provider.docdb_elastic.Cluster_snapshot {
    snapshot_name = "value-0"
    cluster_arn = "value-0"
}
cluster_snapshot_1 = provider.docdb_elastic.Cluster_snapshot {
    snapshot_name = "value-1"
    cluster_arn = "value-1"
}
cluster_snapshot_2 = provider.docdb_elastic.Cluster_snapshot {
    snapshot_name = "value-2"
    cluster_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    cluster_snapshot = provider.docdb_elastic.Cluster_snapshot {
        snapshot_name = "production-value"
        cluster_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Docdb_elastic Documentation](https://docs.aws.amazon.com/docdb_elastic/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
