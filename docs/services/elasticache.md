# Elasticache Service



**Resources**: 30

---

## Overview

The elasticache service provides access to 30 resource types:

- [Serverless_caches](#serverless_caches) [R]
- [Serverless_cache_snapshots](#serverless_cache_snapshots) [R]
- [Snapshot](#snapshot) [CD]
- [User_group](#user_group) [CD]
- [Replication_group](#replication_group) [CD]
- [Cache_security_group](#cache_security_group) [CD]
- [Cache_security_groups](#cache_security_groups) [R]
- [Cache_engine_versions](#cache_engine_versions) [R]
- [Cache_clusters](#cache_clusters) [R]
- [Serverless_cache](#serverless_cache) [CD]
- [User_groups](#user_groups) [R]
- [Serverless_cache_snapshot](#serverless_cache_snapshot) [CD]
- [Global_replication_group](#global_replication_group) [CD]
- [Cache_cluster](#cache_cluster) [CD]
- [Cache_subnet_groups](#cache_subnet_groups) [R]
- [Cache_parameters](#cache_parameters) [R]
- [User](#user) [CD]
- [Reserved_cache_nodes](#reserved_cache_nodes) [R]
- [Service_updates](#service_updates) [R]
- [Replication_groups](#replication_groups) [R]
- [Engine_default_parameters](#engine_default_parameters) [R]
- [Events](#events) [R]
- [Global_replication_groups](#global_replication_groups) [R]
- [Reserved_cache_nodes_offerings](#reserved_cache_nodes_offerings) [R]
- [Cache_subnet_group](#cache_subnet_group) [CD]
- [Users](#users) [R]
- [Cache_parameter_group](#cache_parameter_group) [CD]
- [Cache_parameter_groups](#cache_parameter_groups) [R]
- [Update_actions](#update_actions) [R]
- [Snapshots](#snapshots) [R]

---

## Resources


### Serverless_caches

ServerlessCaches resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `serverless_caches` | Vec<String> | <p>The serverless caches associated with a given description request.</p> |
| `next_token` | String | <p>An optional marker returned from a prior request to support pagination of results from this operation. 
           If this parameter is specified, the response includes only records beyond the marker, 
           up to the value specified by MaxResults.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access serverless_caches outputs
serverless_caches_id = serverless_caches.id
serverless_caches_serverless_caches = serverless_caches.serverless_caches
serverless_caches_next_token = serverless_caches.next_token
```

---


### Serverless_cache_snapshots

ServerlessCacheSnapshots resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `serverless_cache_snapshots` | Vec<String> | <p>The serverless caches snapshots associated with a given description request. Available for Valkey, Redis OSS and Serverless Memcached only.</p> |
| `next_token` | String | <p>An optional marker returned from a prior request to support pagination of results from this operation. 
           If this parameter is specified, the response includes only records beyond the marker, 
           up to the value specified by max-results. Available for Valkey, Redis OSS and Serverless Memcached only.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access serverless_cache_snapshots outputs
serverless_cache_snapshots_id = serverless_cache_snapshots.id
serverless_cache_snapshots_serverless_cache_snapshots = serverless_cache_snapshots.serverless_cache_snapshots
serverless_cache_snapshots_next_token = serverless_cache_snapshots.next_token
```

---


### Snapshot

Snapshot resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `snapshot_name` | String | ✅ | <p>A name for the snapshot being created.</p> |
| `replication_group_id` | String |  | <p>The identifier of an existing replication group. The snapshot is created from this
            replication group.</p> |
| `kms_key_id` | String |  | <p>The ID of the KMS key used to encrypt the snapshot.</p> |
| `cache_cluster_id` | String |  | <p>The identifier of an existing cluster. The snapshot is created from this
            cluster.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to be added to this resource. A tag is a key-value pair. A tag key must
            be accompanied by a tag value, although null is accepted.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create snapshot
snapshot = provider.elasticache.Snapshot {
    snapshot_name = "value"  # <p>A name for the snapshot being created.</p>
}

```

---


### User_group

UserGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_ids` | Vec<String> |  | <p>The list of user IDs that belong to the user group.</p> |
| `user_group_id` | String | ✅ | <p>The ID of the user group.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to be added to this resource. A tag is a key-value pair. A tag key must
            be accompanied by a tag value, although null is accepted. Available for Valkey and Redis OSS only.</p> |
| `engine` | String | ✅ | <p>Sets the engine listed in a user group. The options are valkey or redis.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user_group
user_group = provider.elasticache.User_group {
    user_group_id = "value"  # <p>The ID of the user group.</p>
    engine = "value"  # <p>Sets the engine listed in a user group. The options are valkey or redis.</p>
}

```

---


### Replication_group

ReplicationGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `primary_cluster_id` | String |  | <p>The identifier of the cluster that serves as the primary for this replication group.
            This cluster must already exist and have a status of <code>available</code>.</p>
         <p>This parameter is not required if <code>NumCacheClusters</code>,
                <code>NumNodeGroups</code>, or <code>ReplicasPerNodeGroup</code> is
            specified.</p> |
| `preferred_cache_cluster_a_zs` | Vec<String> |  | <p>A list of EC2 Availability Zones in which the replication group's clusters are
            created. The order of the Availability Zones in the list is the order in which clusters
            are allocated. The primary cluster is created in the first AZ in the list.</p>
         <p>This parameter is not used if there is more than one node group (shard). You should
            use <code>NodeGroupConfiguration</code> instead.</p>
         <note>
            <p>If you are creating your replication group in an Amazon VPC (recommended), you can
                only locate clusters in Availability Zones associated with the subnets in the
                selected subnet group.</p>
            <p>The number of Availability Zones listed must equal the value of
                    <code>NumCacheClusters</code>.</p>
         </note>
         <p>Default: system chosen Availability Zones.</p> |
| `replication_group_id` | String | ✅ | <p>The replication group identifier. This parameter is stored as a lowercase
            string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>A name must contain from 1 to 40 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>The first character must be a letter.</p>
            </li>
            <li>
               <p>A name cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul> |
| `cache_security_group_names` | Vec<String> |  | <p>A list of cache security group names to associate with this replication group.</p> |
| `log_delivery_configurations` | Vec<String> |  | <p>Specifies the destination, format and type of the logs.</p> |
| `auto_minor_version_upgrade` | bool |  | <p> If you are running Valkey 7.2 and above or Redis OSS engine version 6.0 and above, set this parameter to yes 
            to opt-in to the next auto minor version upgrade campaign. This parameter is
            disabled for previous versions.  </p> |
| `snapshot_window` | String |  | <p>The daily time range (in UTC) during which ElastiCache begins taking a daily snapshot
            of your node group (shard).</p>
         <p>Example: <code>05:00-09:00</code>
         </p>
         <p>If you do not specify this parameter, ElastiCache automatically chooses an appropriate
            time range.</p> |
| `serverless_cache_snapshot_name` | String |  | <p>The name of the snapshot used to create a replication group. Available for Valkey, Redis OSS only.</p> |
| `security_group_ids` | Vec<String> |  | <p>One or more Amazon VPC security groups associated with this replication group.</p>
         <p>Use this parameter only when you are creating a replication group in an Amazon Virtual
            Private Cloud (Amazon VPC).</p> |
| `data_tiering_enabled` | bool |  | <p>Enables data tiering. Data tiering is only supported for replication groups using the
            r6gd node type. This parameter must be set to true when using r6gd nodes. For more
            information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/dg/data-tiering.html">Data tiering</a>.</p> |
| `kms_key_id` | String |  | <p>The ID of the KMS key used to encrypt the disk in the cluster.</p> |
| `snapshot_retention_limit` | i64 |  | <p>The number of days for which ElastiCache retains automatic snapshots before deleting
            them. For example, if you set <code>SnapshotRetentionLimit</code> to 5, a snapshot that
            was taken today is retained for 5 days before being deleted.</p>
         <p>Default: 0 (i.e., automatic backups are disabled for this cluster).</p> |
| `automatic_failover_enabled` | bool |  | <p>Specifies whether a read-only replica is automatically promoted to read/write primary
            if the existing primary fails.</p>
         <p>
            <code>AutomaticFailoverEnabled</code> must be enabled for Valkey or Redis OSS (cluster mode enabled)
            replication groups.</p>
         <p>Default: false</p> |
| `tags` | Vec<String> |  | <p>A list of tags to be added to this resource. Tags are comma-separated key,value pairs
            (e.g. Key=<code>myKey</code>, Value=<code>myKeyValue</code>. You can include multiple
            tags as shown following: Key=<code>myKey</code>, Value=<code>myKeyValue</code>
                Key=<code>mySecondKey</code>, Value=<code>mySecondKeyValue</code>. Tags on
            replication groups will be replicated to all nodes.</p> |
| `snapshot_name` | String |  | <p>The name of a snapshot from which to restore data into the new replication group. The
            snapshot status changes to <code>restoring</code> while the new replication group is
            being created.</p> |
| `notification_topic_arn` | String |  | <p>The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS) topic
            to which notifications are sent.</p>
         <note>
            <p>The Amazon SNS topic owner must be the same as the cluster owner.</p>
         </note> |
| `at_rest_encryption_enabled` | bool |  | <p>A flag that enables encryption at rest when set to <code>true</code>.</p>
         <p>You cannot modify the value of <code>AtRestEncryptionEnabled</code> after the
            replication group is created. To enable encryption at rest on a replication group you
            must set <code>AtRestEncryptionEnabled</code> to <code>true</code> when you create the
            replication group. </p>
         <p>
            <b>Required:</b> Only available when creating a replication
            group in an Amazon VPC using Valkey 7.2 and later, Redis OSS version <code>3.2.6</code>, or Redis OSS <code>4.x</code> and
            later.</p>
         <p>Default: <code>true</code> when using Valkey, <code>false</code> when using Redis OSS</p> |
| `auth_token` | String |  | <p>
            <b>Reserved parameter.</b> The password used to access a
            password protected server.</p>
         <p>
            <code>AuthToken</code> can be specified only on replication groups where
                <code>TransitEncryptionEnabled</code> is <code>true</code>.</p>
         <important>
            <p>For HIPAA compliance, you must specify <code>TransitEncryptionEnabled</code> as
                    <code>true</code>, an <code>AuthToken</code>, and a
                    <code>CacheSubnetGroup</code>.</p>
         </important>
         <p>Password constraints:</p>
         <ul>
            <li>
               <p>Must be only printable ASCII characters.</p>
            </li>
            <li>
               <p>Must be at least 16 characters and no more than 128 characters in
                    length.</p>
            </li>
            <li>
               <p>The only permitted printable special characters are !, &, #, $, ^, <,
                    >, and -. Other printable special characters cannot be used in the AUTH
                    token.</p>
            </li>
         </ul>
         <p>For more information, see <a href="http://redis.io/commands/AUTH">AUTH
                password</a> at http://redis.io/commands/AUTH.</p> |
| `preferred_maintenance_window` | String |  | <p>Specifies the weekly time range during which maintenance on the cluster is performed.
            It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The
            minimum maintenance window is a 60 minute period.</p>
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
| `snapshot_arns` | Vec<String> |  | <p>A list of Amazon Resource Names (ARN) that uniquely identify the Valkey or Redis OSS RDB snapshot
            files stored in Amazon S3. The snapshot files are used to populate the new replication
            group. The Amazon S3 object name in the ARN cannot contain any commas. The new
            replication group will have the number of node groups (console: shards) specified by the
            parameter <i>NumNodeGroups</i> or the number of node groups configured by
                <i>NodeGroupConfiguration</i> regardless of the number of ARNs
            specified here.</p>
         <p>Example of an Amazon S3 ARN: <code>arn:aws:s3:::my_bucket/snapshot1.rdb</code>
         </p> |
| `replicas_per_node_group` | i64 |  | <p>An optional parameter that specifies the number of replica nodes in each node group
            (shard). Valid values are 0 to 5.</p> |
| `cache_node_type` | String |  | <p>The compute and memory capacity of the nodes in the node group (shard).</p>
         <p>The following node types are supported by ElastiCache. Generally speaking, the current
            generation types provide more memory and computational power at lower cost when compared
            to their equivalent previous generation counterparts.</p>
         <ul>
            <li>
               <p>General purpose:</p>
               <ul>
                  <li>
                     <p>Current generation: </p>
                     <p>
                        <b>M7g node types</b>:
    					<code>cache.m7g.large</code>,
    					<code>cache.m7g.xlarge</code>,
    					<code>cache.m7g.2xlarge</code>,
    					<code>cache.m7g.4xlarge</code>,
    					<code>cache.m7g.8xlarge</code>,
    					<code>cache.m7g.12xlarge</code>,
    					<code>cache.m7g.16xlarge</code>
                     </p>
                     <note>
                        <p>For region availability, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/dg/CacheNodes.SupportedTypes.html#CacheNodes.SupportedTypesByRegion">Supported Node Types</a>
                        </p>
                     </note>
                     <p>
                        <b>M6g node types</b> (available only for Redis OSS engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward):
					    
					 	<code>cache.m6g.large</code>,
							<code>cache.m6g.xlarge</code>,
							<code>cache.m6g.2xlarge</code>,
							<code>cache.m6g.4xlarge</code>,
							<code>cache.m6g.8xlarge</code>,
							<code>cache.m6g.12xlarge</code>,
							<code>cache.m6g.16xlarge</code>
                     </p>
                     <p>
                        <b>M5 node types:</b>
                        <code>cache.m5.large</code>,
    						<code>cache.m5.xlarge</code>,
    						<code>cache.m5.2xlarge</code>,
    						<code>cache.m5.4xlarge</code>,
    						<code>cache.m5.12xlarge</code>,
    						<code>cache.m5.24xlarge</code>
                     </p>
                     <p>
                        <b>M4 node types:</b>
                        <code>cache.m4.large</code>,
    						<code>cache.m4.xlarge</code>,
    						<code>cache.m4.2xlarge</code>,
    						<code>cache.m4.4xlarge</code>,
    						<code>cache.m4.10xlarge</code>
                     </p>
                     <p>
                        <b>T4g node types</b> (available only for Redis OSS engine version 5.0.6 onward and Memcached engine version 1.5.16 onward):
					        <code>cache.t4g.micro</code>,
					        <code>cache.t4g.small</code>,
					        <code>cache.t4g.medium</code>
                     </p>
                     <p>
                        <b>T3 node types:</b>
                        <code>cache.t3.micro</code>, 
    						<code>cache.t3.small</code>,
    						<code>cache.t3.medium</code>
                     </p>
                     <p>
                        <b>T2 node types:</b>
                        <code>cache.t2.micro</code>, 
    						<code>cache.t2.small</code>,
    						<code>cache.t2.medium</code>
                     </p>
                  </li>
                  <li>
                     <p>Previous generation: (not recommended. Existing clusters are still supported but creation of new clusters is not supported for these types.)</p>
                     <p>
                        <b>T1 node types:</b>
                        <code>cache.t1.micro</code>
                     </p>
                     <p>
                        <b>M1 node types:</b>
                        <code>cache.m1.small</code>, 
						   <code>cache.m1.medium</code>, 
						   <code>cache.m1.large</code>,
						   <code>cache.m1.xlarge</code>
                     </p>
                     <p>
                        <b>M3 node types:</b>
                        <code>cache.m3.medium</code>,
    						<code>cache.m3.large</code>, 
    						<code>cache.m3.xlarge</code>,
    						<code>cache.m3.2xlarge</code>
                     </p>
                  </li>
               </ul>
            </li>
            <li>
               <p>Compute optimized:</p>
               <ul>
                  <li>
                     <p>Previous generation: (not recommended. Existing clusters are still supported but creation of new clusters is not supported for these types.)</p>
                     <p>
                        <b>C1 node types:</b>
                        <code>cache.c1.xlarge</code>
                     </p>
                  </li>
               </ul>
            </li>
            <li>
               <p>Memory optimized:</p>
               <ul>
                  <li>
                     <p>Current generation: </p>
                     <p>
                        <b>R7g node types</b>:	
							<code>cache.r7g.large</code>,
							<code>cache.r7g.xlarge</code>,
							<code>cache.r7g.2xlarge</code>,
							<code>cache.r7g.4xlarge</code>,
							<code>cache.r7g.8xlarge</code>,
							<code>cache.r7g.12xlarge</code>,
							<code>cache.r7g.16xlarge</code>
                     </p>
                     <note>
                        <p>For region availability, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/dg/CacheNodes.SupportedTypes.html#CacheNodes.SupportedTypesByRegion">Supported Node Types</a>
                        </p>
                     </note>
                     <p>
                        <b>R6g node types</b> (available only for Redis OSS engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward):
							<code>cache.r6g.large</code>,
							<code>cache.r6g.xlarge</code>,
							<code>cache.r6g.2xlarge</code>,
							<code>cache.r6g.4xlarge</code>,
							<code>cache.r6g.8xlarge</code>,
							<code>cache.r6g.12xlarge</code>,
							<code>cache.r6g.16xlarge</code>
                     </p>
                     <p>
                        <b>R5 node types:</b>
                        <code>cache.r5.large</code>,
    					   <code>cache.r5.xlarge</code>,
    					   <code>cache.r5.2xlarge</code>,
    					   <code>cache.r5.4xlarge</code>,
    					   <code>cache.r5.12xlarge</code>,
    					   <code>cache.r5.24xlarge</code>
                     </p>
                     <p>
                        <b>R4 node types:</b>
                        <code>cache.r4.large</code>,
    					   <code>cache.r4.xlarge</code>,
    					   <code>cache.r4.2xlarge</code>,
    					   <code>cache.r4.4xlarge</code>,
    					   <code>cache.r4.8xlarge</code>,
    					   <code>cache.r4.16xlarge</code>
                     </p>
                  </li>
                  <li>
                     <p>Previous generation: (not recommended. Existing clusters are still supported but creation of new clusters is not supported for these types.)</p>
                     <p>
                        <b>M2 node types:</b>
                        <code>cache.m2.xlarge</code>, 
    						<code>cache.m2.2xlarge</code>,
    						<code>cache.m2.4xlarge</code>
                     </p>
                     <p>
                        <b>R3 node types:</b>
                        <code>cache.r3.large</code>, 
    						<code>cache.r3.xlarge</code>,
    						<code>cache.r3.2xlarge</code>,  
    						<code>cache.r3.4xlarge</code>,
    						<code>cache.r3.8xlarge</code>
                     </p>
                  </li>
               </ul>
            </li>
         </ul>
         <p>
            <b>Additional node type info</b>
         </p>
         <ul>
            <li>
               <p>All current generation instance types are created in Amazon VPC by
                    default.</p>
            </li>
            <li>
               <p>Valkey or Redis OSS append-only files (AOF) are not supported for T1 or T2 instances.</p>
            </li>
            <li>
               <p>Valkey or Redis OSS Multi-AZ with automatic failover is not supported on T1
                    instances.</p>
            </li>
            <li>
               <p>The configuration variables <code>appendonly</code> and
                        <code>appendfsync</code> are not supported on Valkey, or on Redis OSS version 2.8.22 and
                    later.</p>
            </li>
         </ul> |
| `network_type` | String |  | <p>Must be either <code>ipv4</code> | <code>ipv6</code> | <code>dual_stack</code>. IPv6
            is supported for workloads using Valkey 7.2 and above, Redis OSS engine version 6.2
            to 7.1 and Memcached engine version 1.6.6 and above on all instances built on the <a href="http://aws.amazon.com/ec2/nitro/">Nitro system</a>.</p> |
| `port` | i64 |  | <p>The port number on which each member of the replication group accepts
            connections.</p> |
| `num_cache_clusters` | i64 |  | <p>The number of clusters this replication group initially has.</p>
         <p>This parameter is not used if there is more than one node group (shard). You should
            use <code>ReplicasPerNodeGroup</code> instead.</p>
         <p>If <code>AutomaticFailoverEnabled</code> is <code>true</code>, the value of this
            parameter must be at least 2. If <code>AutomaticFailoverEnabled</code> is
                <code>false</code> you can omit this parameter (it will default to 1), or you can
            explicitly set it to a value between 2 and 6.</p>
         <p>The maximum permitted value for <code>NumCacheClusters</code> is 6 (1 primary plus 5
            replicas).</p> |
| `node_group_configuration` | Vec<String> |  | <p>A list of node group (shard) configuration options. Each node group (shard)
            configuration has the following members: <code>PrimaryAvailabilityZone</code>,
                <code>ReplicaAvailabilityZones</code>, <code>ReplicaCount</code>, and
                <code>Slots</code>.</p>
         <p>If you're creating a Valkey or Redis OSS (cluster mode disabled) or a Valkey or Redis OSS (cluster mode enabled)
            replication group, you can use this parameter to individually configure each node group
            (shard), or you can omit this parameter. However, it is required when seeding a Valkey or Redis OSS (cluster mode enabled) cluster from a S3 rdb file. You must configure each node group
            (shard) using this parameter because you must specify the slots for each node
            group.</p> |
| `user_group_ids` | Vec<String> |  | <p>The user group to associate with the replication group.</p> |
| `transit_encryption_mode` | String |  | <p>A setting that allows you to migrate your clients to use in-transit encryption, with
            no downtime.</p>
         <p>When setting <code>TransitEncryptionEnabled</code> to <code>true</code>, you can set
            your <code>TransitEncryptionMode</code> to <code>preferred</code> in the same request,
            to allow both encrypted and unencrypted connections at the same time. Once you migrate
            all your Valkey or Redis OSS clients to use encrypted connections you can modify the value to
                <code>required</code> to allow encrypted connections only.</p>
         <p>Setting <code>TransitEncryptionMode</code> to <code>required</code> is a two-step
            process that requires you to first set the <code>TransitEncryptionMode</code> to
                <code>preferred</code>, after that you can set <code>TransitEncryptionMode</code> to
                <code>required</code>.</p>
         <p>This process will not trigger the replacement of the replication group.</p> |
| `cluster_mode` | String |  | <p>Enabled or Disabled. To modify cluster mode from Disabled to Enabled, you must first
            set the cluster mode to Compatible. Compatible mode allows your Valkey or Redis OSS clients to connect
            using both cluster mode enabled and cluster mode disabled. After you migrate all Valkey or Redis OSS 
            clients to use cluster mode enabled, you can then complete cluster mode configuration
            and set the cluster mode to Enabled.</p> |
| `engine_version` | String |  | <p>The version number of the cache engine to be used for the clusters in this replication
            group. To view the supported cache engine versions, use the
                <code>DescribeCacheEngineVersions</code> operation.</p>
         <p>
            <b>Important:</b> You can upgrade to a newer engine version
            (see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/dg/SelectEngine.html#VersionManagement">Selecting
                a Cache Engine and Version</a>) in the <i>ElastiCache User
                Guide</i>, but you cannot downgrade to an earlier engine version. If you want
            to use an earlier engine version, you must delete the existing cluster or replication
            group and create it anew with the earlier engine version. </p> |
| `transit_encryption_enabled` | bool |  | <p>A flag that enables in-transit encryption when set to <code>true</code>.</p>
         <p>This parameter is valid only if the <code>Engine</code> parameter is
                <code>redis</code>, the <code>EngineVersion</code> parameter is <code>3.2.6</code>,
                <code>4.x</code> or later, and the cluster is being created in an Amazon VPC.</p>
         <p>If you enable in-transit encryption, you must also specify a value for
                <code>CacheSubnetGroup</code>.</p>
         <p>
            <b>Required:</b> Only available when creating a replication
            group in an Amazon VPC using Redis OSS version <code>3.2.6</code>, <code>4.x</code> or
            later.</p>
         <p>Default: <code>false</code>
         </p>
         <important>
            <p>For HIPAA compliance, you must specify <code>TransitEncryptionEnabled</code> as
                    <code>true</code>, an <code>AuthToken</code>, and a
                    <code>CacheSubnetGroup</code>.</p>
         </important> |
| `ip_discovery` | String |  | <p>The network type you choose when creating a replication group, either
            <code>ipv4</code> | <code>ipv6</code>. IPv6 is supported for workloads using Valkey 7.2 and above, Redis OSS engine version 6.2
            to 7.1 or Memcached engine version 1.6.6 and above on all instances built on
            the <a href="http://aws.amazon.com/ec2/nitro/">Nitro system</a>.</p> |
| `global_replication_group_id` | String |  | <p>The name of the Global datastore</p> |
| `cache_parameter_group_name` | String |  | <p>The name of the parameter group to associate with this replication group. If this
            argument is omitted, the default cache parameter group for the specified engine is
            used.</p>
         <p>If you are running Valkey or Redis OSS version 3.2.4 or later, only one node group (shard), and want
            to use a default parameter group, we recommend that you specify the parameter group by
            name. </p>
         <ul>
            <li>
               <p>To create a Valkey or Redis OSS (cluster mode disabled) replication group, use
                        <code>CacheParameterGroupName=default.redis3.2</code>.</p>
            </li>
            <li>
               <p>To create a Valkey or Redis OSS (cluster mode enabled) replication group, use
                        <code>CacheParameterGroupName=default.redis3.2.cluster.on</code>.</p>
            </li>
         </ul> |
| `multi_az_enabled` | bool |  | <p>A flag indicating if you have Multi-AZ enabled to enhance fault tolerance. For more
            information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/dg/AutoFailover.html">Minimizing Downtime: Multi-AZ</a>.</p> |
| `cache_subnet_group_name` | String |  | <p>The name of the cache subnet group to be used for the replication group.</p>
         <important>
            <p>If you're going to launch your cluster in an Amazon VPC, you need to create a
                subnet group before you start creating a cluster. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/dg/SubnetGroups.html">Subnets and Subnet Groups</a>.</p>
         </important> |
| `replication_group_description` | String | ✅ | <p>A user-created description for the replication group.</p> |
| `engine` | String |  | <p>The name of the cache engine to be used for the clusters in this replication group.
            The value must be set to <code>valkey</code> or <code>redis</code>.</p> |
| `num_node_groups` | i64 |  | <p>An optional parameter that specifies the number of node groups (shards) for this Valkey or Redis OSS (cluster mode enabled) replication group. For Valkey or Redis OSS (cluster mode disabled) either omit
            this parameter or set it to 1.</p>
         <p>Default: 1</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create replication_group
replication_group = provider.elasticache.Replication_group {
    replication_group_id = "value"  # <p>The replication group identifier. This parameter is stored as a lowercase
            string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>A name must contain from 1 to 40 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>The first character must be a letter.</p>
            </li>
            <li>
               <p>A name cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
    replication_group_description = "value"  # <p>A user-created description for the replication group.</p>
}

```

---


### Cache_security_group

CacheSecurityGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cache_security_group_name` | String | ✅ | <p>A name for the cache security group. This value is stored as a lowercase
            string.</p>
         <p>Constraints: Must contain no more than 255 alphanumeric characters. Cannot be the word
            "Default".</p>
         <p>Example: <code>mysecuritygroup</code>
         </p> |
| `description` | String | ✅ | <p>A description for the cache security group.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to be added to this resource. A tag is a key-value pair. A tag key must
            be accompanied by a tag value, although null is accepted.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cache_security_group
cache_security_group = provider.elasticache.Cache_security_group {
    cache_security_group_name = "value"  # <p>A name for the cache security group. This value is stored as a lowercase
            string.</p>
         <p>Constraints: Must contain no more than 255 alphanumeric characters. Cannot be the word
            "Default".</p>
         <p>Example: <code>mysecuritygroup</code>
         </p>
    description = "value"  # <p>A description for the cache security group.</p>
}

```

---


### Cache_security_groups

CacheSecurityGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cache_security_groups` | Vec<String> | <p>A list of cache security groups. Each element in the list contains detailed
            information about one group.</p> |
| `marker` | String | <p>Provides an identifier to allow retrieval of paginated results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cache_security_groups outputs
cache_security_groups_id = cache_security_groups.id
cache_security_groups_cache_security_groups = cache_security_groups.cache_security_groups
cache_security_groups_marker = cache_security_groups.marker
```

---


### Cache_engine_versions

CacheEngineVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cache_engine_versions` | Vec<String> | <p>A list of cache engine version details. Each element in the list contains detailed
            information about one cache engine version.</p> |
| `marker` | String | <p>Provides an identifier to allow retrieval of paginated results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cache_engine_versions outputs
cache_engine_versions_id = cache_engine_versions.id
cache_engine_versions_cache_engine_versions = cache_engine_versions.cache_engine_versions
cache_engine_versions_marker = cache_engine_versions.marker
```

---


### Cache_clusters

CacheClusters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cache_clusters` | Vec<String> | <p>A list of clusters. Each item in the list contains detailed information about one
            cluster.</p> |
| `marker` | String | <p>Provides an identifier to allow retrieval of paginated results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cache_clusters outputs
cache_clusters_id = cache_clusters.id
cache_clusters_cache_clusters = cache_clusters.cache_clusters
cache_clusters_marker = cache_clusters.marker
```

---


### Serverless_cache

ServerlessCache resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The list of tags (key, value) pairs to be added to the serverless cache resource. Default is NULL.</p> |
| `snapshot_retention_limit` | i64 |  | <p>The number of snapshots that will be retained for the serverless cache that is being created. 
           As new snapshots beyond this limit are added, the oldest snapshots will be deleted on a rolling basis. Available for Valkey, Redis OSS and Serverless Memcached only.</p> |
| `cache_usage_limits` | String |  | <p>Sets the cache usage limits for storage and ElastiCache Processing Units for the cache.</p> |
| `description` | String |  | <p>User-provided description for the serverless cache.  
           The default is NULL, i.e. if no description is provided then an empty string will be returned. 
           The maximum length is 255 characters. </p> |
| `user_group_id` | String |  | <p>The identifier of the UserGroup to be associated with the serverless cache.  Available for Valkey and Redis OSS only. Default is NULL.</p> |
| `kms_key_id` | String |  | <p>ARN of the customer managed key for encrypting the data at rest. If no KMS key is provided, a default service key is used.</p> |
| `major_engine_version` | String |  | <p>The version of the cache engine that will be used to create the serverless cache.</p> |
| `daily_snapshot_time` | String |  | <p>The daily time that snapshots will be created from the new serverless cache. By default this number is populated with 
           0, i.e. no snapshots will be created on an automatic daily basis. Available for Valkey, Redis OSS and Serverless Memcached only.</p> |
| `subnet_ids` | Vec<String> |  | <p>A list of the identifiers of the subnets where the VPC endpoint for the serverless cache will be deployed. 
           All the subnetIds must belong to the same VPC.</p> |
| `security_group_ids` | Vec<String> |  | <p>A list of the one or more VPC security groups to be associated with the serverless cache. 
           The security group will authorize traffic access for the VPC end-point (private-link). 
           If no other information is given this will be the VPC’s Default Security Group that is associated with the cluster VPC 
           end-point.</p> |
| `engine` | String | ✅ | <p>The name of the cache engine to be used for creating the serverless cache.</p> |
| `serverless_cache_name` | String | ✅ | <p>User-provided identifier for the serverless cache. This parameter is stored as a lowercase string.</p> |
| `snapshot_arns_to_restore` | Vec<String> |  | <p>The ARN(s) of the snapshot that the new serverless cache will be created from. Available for Valkey, Redis OSS and Serverless Memcached only.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create serverless_cache
serverless_cache = provider.elasticache.Serverless_cache {
    engine = "value"  # <p>The name of the cache engine to be used for creating the serverless cache.</p>
    serverless_cache_name = "value"  # <p>User-provided identifier for the serverless cache. This parameter is stored as a lowercase string.</p>
}

```

---


### User_groups

UserGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_groups` | Vec<String> | <p>Returns a list of user groups.</p> |
| `marker` | String | <p>An optional marker returned from a prior request. Use this marker for pagination of
            results from this operation. If this parameter is specified, the response includes only
            records beyond the marker, up to the value specified by MaxRecords.></p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access user_groups outputs
user_groups_id = user_groups.id
user_groups_user_groups = user_groups.user_groups
user_groups_marker = user_groups.marker
```

---


### Serverless_cache_snapshot

ServerlessCacheSnapshot resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of tags to be added to the snapshot resource. A tag is a key-value pair. Available for Valkey, Redis OSS and Serverless Memcached only.</p> |
| `serverless_cache_snapshot_name` | String | ✅ | <p>The name for the snapshot being created. Must be unique for the customer account. Available for Valkey, Redis OSS and Serverless Memcached only.
           Must be between 1 and 255 characters.</p> |
| `serverless_cache_name` | String | ✅ | <p>The name of an existing serverless cache. The snapshot is created from this cache. Available for Valkey, Redis OSS and Serverless Memcached only.</p> |
| `kms_key_id` | String |  | <p>The ID of the KMS key used to encrypt the snapshot.  Available for Valkey, Redis OSS and Serverless Memcached only. Default: NULL</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create serverless_cache_snapshot
serverless_cache_snapshot = provider.elasticache.Serverless_cache_snapshot {
    serverless_cache_snapshot_name = "value"  # <p>The name for the snapshot being created. Must be unique for the customer account. Available for Valkey, Redis OSS and Serverless Memcached only.
           Must be between 1 and 255 characters.</p>
    serverless_cache_name = "value"  # <p>The name of an existing serverless cache. The snapshot is created from this cache. Available for Valkey, Redis OSS and Serverless Memcached only.</p>
}

```

---


### Global_replication_group

GlobalReplicationGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `global_replication_group_id_suffix` | String | ✅ | <p>The suffix name of a Global datastore. Amazon ElastiCache automatically applies a
            prefix to the Global datastore ID when it is created. Each Amazon Region has its own
            prefix. For instance, a Global datastore ID created in the US-West-1 region will begin
            with "dsdfu" along with the suffix name you provide. The suffix, combined with the
            auto-generated prefix, guarantees uniqueness of the Global datastore name across
            multiple regions. </p>
         <p>For a full list of Amazon Regions and their respective Global datastore iD prefixes,
            see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/dg/Redis-Global-Datastores-CLI.html">Using the Amazon CLI with Global datastores </a>.</p> |
| `global_replication_group_description` | String |  | <p>Provides details of the Global datastore</p> |
| `primary_replication_group_id` | String | ✅ | <p>The name of the primary cluster that accepts writes and will replicate updates to the
            secondary cluster.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create global_replication_group
global_replication_group = provider.elasticache.Global_replication_group {
    global_replication_group_id_suffix = "value"  # <p>The suffix name of a Global datastore. Amazon ElastiCache automatically applies a
            prefix to the Global datastore ID when it is created. Each Amazon Region has its own
            prefix. For instance, a Global datastore ID created in the US-West-1 region will begin
            with "dsdfu" along with the suffix name you provide. The suffix, combined with the
            auto-generated prefix, guarantees uniqueness of the Global datastore name across
            multiple regions. </p>
         <p>For a full list of Amazon Regions and their respective Global datastore iD prefixes,
            see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/dg/Redis-Global-Datastores-CLI.html">Using the Amazon CLI with Global datastores </a>.</p>
    primary_replication_group_id = "value"  # <p>The name of the primary cluster that accepts writes and will replicate updates to the
            secondary cluster.</p>
}

```

---


### Cache_cluster

CacheCluster resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cache_parameter_group_name` | String |  | <p>The name of the parameter group to associate with this cluster. If this argument is
            omitted, the default parameter group for the specified engine is used. You cannot use
            any parameter group which has <code>cluster-enabled='yes'</code> when creating a
            cluster.</p> |
| `num_cache_nodes` | i64 |  | <p>The initial number of cache nodes that the cluster has.</p>
         <p>For clusters running Valkey or Redis OSS, this value must be 1. For clusters running Memcached, this
            value must be between 1 and 40.</p>
         <p>If you need more than 40 nodes for your Memcached cluster, please fill out the
            ElastiCache Limit Increase Request form at <a href="http://aws.amazon.com/contact-us/elasticache-node-limit-request/">http://aws.amazon.com/contact-us/elasticache-node-limit-request/</a>.</p> |
| `snapshot_window` | String |  | <p>The daily time range (in UTC) during which ElastiCache begins taking a daily snapshot
            of your node group (shard).</p>
         <p>Example: <code>05:00-09:00</code>
         </p>
         <p>If you do not specify this parameter, ElastiCache automatically chooses an appropriate
            time range.</p>
         <note>
            <p>This parameter is only valid if the <code>Engine</code> parameter is
                    <code>redis</code>.</p>
         </note> |
| `snapshot_retention_limit` | i64 |  | <p>The number of days for which ElastiCache retains automatic snapshots before deleting
            them. For example, if you set <code>SnapshotRetentionLimit</code> to 5, a snapshot taken
            today is retained for 5 days before being deleted.</p>
         <note>
            <p>This parameter is only valid if the <code>Engine</code> parameter is
                    <code>redis</code>.</p>
         </note>
         <p>Default: 0 (i.e., automatic backups are disabled for this cache cluster).</p> |
| `ip_discovery` | String |  | <p>The network type you choose when modifying a cluster, either <code>ipv4</code> |
            <code>ipv6</code>. IPv6 is supported for workloads using Valkey 7.2 and above, Redis OSS engine version 6.2
            to 7.1 and Memcached engine version 1.6.6 and above on all instances built on the <a href="http://aws.amazon.com/ec2/nitro/">Nitro system</a>.</p> |
| `engine` | String |  | <p>The name of the cache engine to be used for this cluster.</p>
         <p>Valid values for this parameter are: <code>memcached</code> |
            <code>redis</code>
         </p> |
| `preferred_outpost_arns` | Vec<String> |  | <p>The outpost ARNs in which the cache cluster is created.</p> |
| `auth_token` | String |  | <p>
            <b>Reserved parameter.</b> The password used to access a
            password protected server.</p>
         <p>Password constraints:</p>
         <ul>
            <li>
               <p>Must be only printable ASCII characters.</p>
            </li>
            <li>
               <p>Must be at least 16 characters and no more than 128 characters in
                    length.</p>
            </li>
            <li>
               <p>The only permitted printable special characters are !, &, #, $, ^, <,
                    >, and -. Other printable special characters cannot be used in the AUTH
                    token.</p>
            </li>
         </ul>
         <p>For more information, see <a href="http://redis.io/commands/AUTH">AUTH
                password</a> at http://redis.io/commands/AUTH.</p> |
| `cache_subnet_group_name` | String |  | <p>The name of the subnet group to be used for the cluster.</p>
         <p>Use this parameter only when you are creating a cluster in an Amazon Virtual Private
            Cloud (Amazon VPC).</p>
         <important>
            <p>If you're going to launch your cluster in an Amazon VPC, you need to create a
                subnet group before you start creating a cluster. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/dg/SubnetGroups.html">Subnets and Subnet Groups</a>.</p>
         </important> |
| `preferred_availability_zone` | String |  | <p>The EC2 Availability Zone in which the cluster is created.</p>
         <p>All nodes belonging to this cluster are placed in the preferred Availability Zone. If
            you want to create your nodes across multiple Availability Zones, use
                <code>PreferredAvailabilityZones</code>.</p>
         <p>Default: System chosen Availability Zone.</p> |
| `engine_version` | String |  | <p>The version number of the cache engine to be used for this cluster. To view the
            supported cache engine versions, use the DescribeCacheEngineVersions operation.</p>
         <p>
            <b>Important:</b> You can upgrade to a newer engine version
            (see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/dg/SelectEngine.html#VersionManagement">Selecting
                a Cache Engine and Version</a>), but you cannot downgrade to an earlier engine
            version. If you want to use an earlier engine version, you must delete the existing
            cluster or replication group and create it anew with the earlier engine version. </p> |
| `preferred_maintenance_window` | String |  | <p>Specifies the weekly time range during which maintenance on the cluster is performed.
            It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The
            minimum maintenance window is a 60 minute period. </p> |
| `cache_security_group_names` | Vec<String> |  | <p>A list of security group names to associate with this cluster.</p>
         <p>Use this parameter only when you are creating a cluster outside of an Amazon Virtual
            Private Cloud (Amazon VPC).</p> |
| `outpost_mode` | String |  | <p>Specifies whether the nodes in the cluster are created in a single outpost or across
            multiple outposts.</p> |
| `security_group_ids` | Vec<String> |  | <p>One or more VPC security groups associated with the cluster.</p>
         <p>Use this parameter only when you are creating a cluster in an Amazon Virtual Private
            Cloud (Amazon VPC).</p> |
| `port` | i64 |  | <p>The port number on which each of the cache nodes accepts connections.</p> |
| `auto_minor_version_upgrade` | bool |  | <p> If you are running Valkey 7.2 and above or Redis OSS engine version 6.0 and above, set this parameter to yes 
            to opt-in to the next auto minor version upgrade campaign. This parameter is
            disabled for previous versions.  </p> |
| `transit_encryption_enabled` | bool |  | <p>A flag that enables in-transit encryption when set to true.</p> |
| `cache_cluster_id` | String | ✅ | <p>The node group (shard) identifier. This parameter is stored as a lowercase
            string.</p>
         <p>
            <b>Constraints:</b>
         </p>
         <ul>
            <li>
               <p>A name must contain from 1 to 50 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>The first character must be a letter.</p>
            </li>
            <li>
               <p>A name cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul> |
| `preferred_availability_zones` | Vec<String> |  | <p>A list of the Availability Zones in which cache nodes are created. The order of the
            zones in the list is not important.</p>
         <p>This option is only supported on Memcached.</p>
         <note>
            <p>If you are creating your cluster in an Amazon VPC (recommended) you can only
                locate nodes in Availability Zones that are associated with the subnets in the
                selected subnet group.</p>
            <p>The number of Availability Zones listed must equal the value of
                    <code>NumCacheNodes</code>.</p>
         </note>
         <p>If you want all the nodes in the same Availability Zone, use
                <code>PreferredAvailabilityZone</code> instead, or repeat the Availability Zone
            multiple times in the list.</p>
         <p>Default: System chosen Availability Zones.</p> |
| `log_delivery_configurations` | Vec<String> |  | <p>Specifies the destination, format and type of the logs. </p> |
| `network_type` | String |  | <p>Must be either <code>ipv4</code> | <code>ipv6</code> | <code>dual_stack</code>. IPv6
            is supported for workloads using Valkey 7.2 and above, Redis OSS engine version 6.2 to 7.1
            and Memcached engine version 1.6.6 and above on all instances built on the <a href="http://aws.amazon.com/ec2/nitro/">Nitro system</a>. </p> |
| `snapshot_arns` | Vec<String> |  | <p>A single-element string list containing an Amazon Resource Name (ARN) that uniquely
            identifies a Valkey or Redis OSS RDB snapshot file stored in Amazon S3. The snapshot file is used to
            populate the node group (shard). The Amazon S3 object name in the ARN cannot contain any
            commas.</p>
         <note>
            <p>This parameter is only valid if the <code>Engine</code> parameter is
                    <code>redis</code>.</p>
         </note>
         <p>Example of an Amazon S3 ARN: <code>arn:aws:s3:::my_bucket/snapshot1.rdb</code>
         </p> |
| `cache_node_type` | String |  | <p>The compute and memory capacity of the nodes in the node group (shard).</p>
         <p>The following node types are supported by ElastiCache. Generally speaking, the current
            generation types provide more memory and computational power at lower cost when compared
            to their equivalent previous generation counterparts.</p>
         <ul>
            <li>
               <p>General purpose:</p>
               <ul>
                  <li>
                     <p>Current generation: </p>
                     <p>
                        <b>M7g node types</b>:
    					<code>cache.m7g.large</code>,
    					<code>cache.m7g.xlarge</code>,
    					<code>cache.m7g.2xlarge</code>,
    					<code>cache.m7g.4xlarge</code>,
    					<code>cache.m7g.8xlarge</code>,
    					<code>cache.m7g.12xlarge</code>,
    					<code>cache.m7g.16xlarge</code>
                     </p>
                     <note>
                        <p>For region availability, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/dg/CacheNodes.SupportedTypes.html#CacheNodes.SupportedTypesByRegion">Supported Node Types</a>
                        </p>
                     </note>
                     <p>
                        <b>M6g node types</b> (available only for Redis OSS engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward):
					    
					 	<code>cache.m6g.large</code>,
							<code>cache.m6g.xlarge</code>,
							<code>cache.m6g.2xlarge</code>,
							<code>cache.m6g.4xlarge</code>,
							<code>cache.m6g.8xlarge</code>,
							<code>cache.m6g.12xlarge</code>,
							<code>cache.m6g.16xlarge</code>
                     </p>
                     <p>
                        <b>M5 node types:</b>
                        <code>cache.m5.large</code>,
    						<code>cache.m5.xlarge</code>,
    						<code>cache.m5.2xlarge</code>,
    						<code>cache.m5.4xlarge</code>,
    						<code>cache.m5.12xlarge</code>,
    						<code>cache.m5.24xlarge</code>
                     </p>
                     <p>
                        <b>M4 node types:</b>
                        <code>cache.m4.large</code>,
    						<code>cache.m4.xlarge</code>,
    						<code>cache.m4.2xlarge</code>,
    						<code>cache.m4.4xlarge</code>,
    						<code>cache.m4.10xlarge</code>
                     </p>
                     <p>
                        <b>T4g node types</b> (available only for Redis OSS engine version 5.0.6 onward and Memcached engine version 1.5.16 onward):
					        <code>cache.t4g.micro</code>,
					        <code>cache.t4g.small</code>,
					        <code>cache.t4g.medium</code>
                     </p>
                     <p>
                        <b>T3 node types:</b>
                        <code>cache.t3.micro</code>, 
    						<code>cache.t3.small</code>,
    						<code>cache.t3.medium</code>
                     </p>
                     <p>
                        <b>T2 node types:</b>
                        <code>cache.t2.micro</code>, 
    						<code>cache.t2.small</code>,
    						<code>cache.t2.medium</code>
                     </p>
                  </li>
                  <li>
                     <p>Previous generation: (not recommended. Existing clusters are still supported but creation of new clusters is not supported for these types.)</p>
                     <p>
                        <b>T1 node types:</b>
                        <code>cache.t1.micro</code>
                     </p>
                     <p>
                        <b>M1 node types:</b>
                        <code>cache.m1.small</code>, 
						   <code>cache.m1.medium</code>, 
						   <code>cache.m1.large</code>,
						   <code>cache.m1.xlarge</code>
                     </p>
                     <p>
                        <b>M3 node types:</b>
                        <code>cache.m3.medium</code>,
    						<code>cache.m3.large</code>, 
    						<code>cache.m3.xlarge</code>,
    						<code>cache.m3.2xlarge</code>
                     </p>
                  </li>
               </ul>
            </li>
            <li>
               <p>Compute optimized:</p>
               <ul>
                  <li>
                     <p>Previous generation: (not recommended. Existing clusters are still supported but creation of new clusters is not supported for these types.)</p>
                     <p>
                        <b>C1 node types:</b>
                        <code>cache.c1.xlarge</code>
                     </p>
                  </li>
               </ul>
            </li>
            <li>
               <p>Memory optimized:</p>
               <ul>
                  <li>
                     <p>Current generation: </p>
                     <p>
                        <b>R7g node types</b>:	
							<code>cache.r7g.large</code>,
							<code>cache.r7g.xlarge</code>,
							<code>cache.r7g.2xlarge</code>,
							<code>cache.r7g.4xlarge</code>,
							<code>cache.r7g.8xlarge</code>,
							<code>cache.r7g.12xlarge</code>,
							<code>cache.r7g.16xlarge</code>
                     </p>
                     <note>
                        <p>For region availability, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/dg/CacheNodes.SupportedTypes.html#CacheNodes.SupportedTypesByRegion">Supported Node Types</a>
                        </p>
                     </note>
                     <p>
                        <b>R6g node types</b> (available only for Redis OSS engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward):
							<code>cache.r6g.large</code>,
							<code>cache.r6g.xlarge</code>,
							<code>cache.r6g.2xlarge</code>,
							<code>cache.r6g.4xlarge</code>,
							<code>cache.r6g.8xlarge</code>,
							<code>cache.r6g.12xlarge</code>,
							<code>cache.r6g.16xlarge</code>
                     </p>
                     <p>
                        <b>R5 node types:</b>
                        <code>cache.r5.large</code>,
    					   <code>cache.r5.xlarge</code>,
    					   <code>cache.r5.2xlarge</code>,
    					   <code>cache.r5.4xlarge</code>,
    					   <code>cache.r5.12xlarge</code>,
    					   <code>cache.r5.24xlarge</code>
                     </p>
                     <p>
                        <b>R4 node types:</b>
                        <code>cache.r4.large</code>,
    					   <code>cache.r4.xlarge</code>,
    					   <code>cache.r4.2xlarge</code>,
    					   <code>cache.r4.4xlarge</code>,
    					   <code>cache.r4.8xlarge</code>,
    					   <code>cache.r4.16xlarge</code>
                     </p>
                  </li>
                  <li>
                     <p>Previous generation: (not recommended. Existing clusters are still supported but creation of new clusters is not supported for these types.)</p>
                     <p>
                        <b>M2 node types:</b>
                        <code>cache.m2.xlarge</code>, 
    						<code>cache.m2.2xlarge</code>,
    						<code>cache.m2.4xlarge</code>
                     </p>
                     <p>
                        <b>R3 node types:</b>
                        <code>cache.r3.large</code>, 
    						<code>cache.r3.xlarge</code>,
    						<code>cache.r3.2xlarge</code>,  
    						<code>cache.r3.4xlarge</code>,
    						<code>cache.r3.8xlarge</code>
                     </p>
                  </li>
               </ul>
            </li>
         </ul>
         <p>
            <b>Additional node type info</b>
         </p>
         <ul>
            <li>
               <p>All current generation instance types are created in Amazon VPC by
                    default.</p>
            </li>
            <li>
               <p>Valkey or Redis OSS append-only files (AOF) are not supported for T1 or T2 instances.</p>
            </li>
            <li>
               <p>Valkey or Redis OSS Multi-AZ with automatic failover is not supported on T1
                    instances.</p>
            </li>
            <li>
               <p>The configuration variables <code>appendonly</code> and
                        <code>appendfsync</code> are not supported on Valkey, or on Redis OSS version 2.8.22 and
                    later.</p>
            </li>
         </ul> |
| `snapshot_name` | String |  | <p>The name of a Valkey or Redis OSS snapshot from which to restore data into the new node group
            (shard). The snapshot status changes to <code>restoring</code> while the new node group
            (shard) is being created.</p>
         <note>
            <p>This parameter is only valid if the <code>Engine</code> parameter is
                    <code>redis</code>.</p>
         </note> |
| `notification_topic_arn` | String |  | <p>The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS) topic
            to which notifications are sent.</p>
         <note>
            <p>The Amazon SNS topic owner must be the same as the cluster owner.</p>
         </note> |
| `az_mode` | String |  | <p>Specifies whether the nodes in this Memcached cluster are created in a single
            Availability Zone or created across multiple Availability Zones in the cluster's
            region.</p>
         <p>This parameter is only supported for Memcached clusters.</p>
         <p>If the <code>AZMode</code> and <code>PreferredAvailabilityZones</code> are not
            specified, ElastiCache assumes <code>single-az</code> mode.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to be added to this resource.</p> |
| `preferred_outpost_arn` | String |  | <p>The outpost ARN in which the cache cluster is created.</p> |
| `replication_group_id` | String |  | <p>The ID of the replication group to which this cluster should belong. If this parameter
            is specified, the cluster is added to the specified replication group as a read replica;
            otherwise, the cluster is a standalone primary that is not part of any replication
            group.</p>
         <p>If the specified replication group is Multi-AZ enabled and the Availability Zone is
            not specified, the cluster is created in Availability Zones that provide the best spread
            of read replicas across Availability Zones.</p>
         <note>
            <p>This parameter is only valid if the <code>Engine</code> parameter is
                    <code>redis</code>.</p>
         </note> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cache_cluster
cache_cluster = provider.elasticache.Cache_cluster {
    cache_cluster_id = "value"  # <p>The node group (shard) identifier. This parameter is stored as a lowercase
            string.</p>
         <p>
            <b>Constraints:</b>
         </p>
         <ul>
            <li>
               <p>A name must contain from 1 to 50 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>The first character must be a letter.</p>
            </li>
            <li>
               <p>A name cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
}

```

---


### Cache_subnet_groups

CacheSubnetGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cache_subnet_groups` | Vec<String> | <p>A list of cache subnet groups. Each element in the list contains detailed information
            about one group.</p> |
| `marker` | String | <p>Provides an identifier to allow retrieval of paginated results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cache_subnet_groups outputs
cache_subnet_groups_id = cache_subnet_groups.id
cache_subnet_groups_cache_subnet_groups = cache_subnet_groups.cache_subnet_groups
cache_subnet_groups_marker = cache_subnet_groups.marker
```

---


### Cache_parameters

CacheParameters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parameters` | Vec<String> | <p>A list of <a>Parameter</a> instances.</p> |
| `cache_node_type_specific_parameters` | Vec<String> | <p>A list of parameters specific to a particular cache node type. Each element in the
            list contains detailed information about one parameter.</p> |
| `marker` | String | <p>Provides an identifier to allow retrieval of paginated results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cache_parameters outputs
cache_parameters_id = cache_parameters.id
cache_parameters_parameters = cache_parameters.parameters
cache_parameters_cache_node_type_specific_parameters = cache_parameters.cache_node_type_specific_parameters
cache_parameters_marker = cache_parameters.marker
```

---


### User

User resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of tags to be added to this resource. A tag is a key-value pair. A tag key must
            be accompanied by a tag value, although null is accepted.</p> |
| `authentication_mode` | String |  | <p>Specifies how to authenticate the user.</p> |
| `user_id` | String | ✅ | <p>The ID of the user.</p> |
| `engine` | String | ✅ | <p>The options are valkey or redis. </p> |
| `user_name` | String | ✅ | <p>The username of the user.</p> |
| `no_password_required` | bool |  | <p>Indicates a password is not required for this user.</p> |
| `passwords` | Vec<String> |  | <p>Passwords used for this user. You can create up to two passwords for each user.</p> |
| `access_string` | String | ✅ | <p>Access permissions string used for this user.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user
user = provider.elasticache.User {
    user_id = "value"  # <p>The ID of the user.</p>
    engine = "value"  # <p>The options are valkey or redis. </p>
    user_name = "value"  # <p>The username of the user.</p>
    access_string = "value"  # <p>Access permissions string used for this user.</p>
}

```

---


### Reserved_cache_nodes

ReservedCacheNodes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>Provides an identifier to allow retrieval of paginated results.</p> |
| `reserved_cache_nodes` | Vec<String> | <p>A list of reserved cache nodes. Each element in the list contains detailed information
            about one node.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_cache_nodes outputs
reserved_cache_nodes_id = reserved_cache_nodes.id
reserved_cache_nodes_marker = reserved_cache_nodes.marker
reserved_cache_nodes_reserved_cache_nodes = reserved_cache_nodes.reserved_cache_nodes
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
| `service_updates` | Vec<String> | <p>A list of service updates</p> |
| `marker` | String | <p>An optional marker returned from a prior request. Use this marker for pagination of
            results from this operation. If this parameter is specified, the response includes only
            records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p> |


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
service_updates_service_updates = service_updates.service_updates
service_updates_marker = service_updates.marker
```

---


### Replication_groups

ReplicationGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>Provides an identifier to allow retrieval of paginated results.</p> |
| `replication_groups` | Vec<String> | <p>A list of replication groups. Each item in the list contains detailed information
            about one replication group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access replication_groups outputs
replication_groups_id = replication_groups.id
replication_groups_marker = replication_groups.marker
replication_groups_replication_groups = replication_groups.replication_groups
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


### Events

Events resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>Provides an identifier to allow retrieval of paginated results.</p> |
| `events` | Vec<String> | <p>A list of events. Each element in the list contains detailed information about one
            event.</p> |


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


### Global_replication_groups

GlobalReplicationGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional marker returned from a prior request. Use this marker for pagination of
            results from this operation. If this parameter is specified, the response includes only
            records beyond the marker, up to the value specified by MaxRecords. ></p> |
| `global_replication_groups` | Vec<String> | <p>Indicates the slot configuration and global identifier for each slice group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access global_replication_groups outputs
global_replication_groups_id = global_replication_groups.id
global_replication_groups_marker = global_replication_groups.marker
global_replication_groups_global_replication_groups = global_replication_groups.global_replication_groups
```

---


### Reserved_cache_nodes_offerings

ReservedCacheNodesOfferings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reserved_cache_nodes_offerings` | Vec<String> | <p>A list of reserved cache node offerings. Each element in the list contains detailed
            information about one offering.</p> |
| `marker` | String | <p>Provides an identifier to allow retrieval of paginated results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_cache_nodes_offerings outputs
reserved_cache_nodes_offerings_id = reserved_cache_nodes_offerings.id
reserved_cache_nodes_offerings_reserved_cache_nodes_offerings = reserved_cache_nodes_offerings.reserved_cache_nodes_offerings
reserved_cache_nodes_offerings_marker = reserved_cache_nodes_offerings.marker
```

---


### Cache_subnet_group

CacheSubnetGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cache_subnet_group_description` | String | ✅ | <p>A description for the cache subnet group.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to be added to this resource. A tag is a key-value pair. A tag key must
            be accompanied by a tag value, although null is accepted.</p> |
| `subnet_ids` | Vec<String> | ✅ | <p>A list of VPC subnet IDs for the cache subnet group.</p> |
| `cache_subnet_group_name` | String | ✅ | <p>A name for the cache subnet group. This value is stored as a lowercase string.</p>
         <p>Constraints: Must contain no more than 255 alphanumeric characters or hyphens.</p>
         <p>Example: <code>mysubnetgroup</code>
         </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cache_subnet_group
cache_subnet_group = provider.elasticache.Cache_subnet_group {
    cache_subnet_group_description = "value"  # <p>A description for the cache subnet group.</p>
    subnet_ids = "value"  # <p>A list of VPC subnet IDs for the cache subnet group.</p>
    cache_subnet_group_name = "value"  # <p>A name for the cache subnet group. This value is stored as a lowercase string.</p>
         <p>Constraints: Must contain no more than 255 alphanumeric characters or hyphens.</p>
         <p>Example: <code>mysubnetgroup</code>
         </p>
}

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
| `marker` | String | <p>An optional marker returned from a prior request. Use this marker for pagination of
            results from this operation. If this parameter is specified, the response includes only
            records beyond the marker, up to the value specified by MaxRecords. ></p> |
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
users_marker = users.marker
users_users = users.users
```

---


### Cache_parameter_group

CacheParameterGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cache_parameter_group_family` | String | ✅ | <p>The name of the cache parameter group family that the cache parameter group can be
            used with.</p>
         <p>Valid values are: <code>valkey8</code> | <code>valkey7</code> | <code>memcached1.4</code> | <code>memcached1.5</code> | <code>memcached1.6</code> | <code>redis2.6</code> | <code>redis2.8</code> |
<code>redis3.2</code> | <code>redis4.0</code> | <code>redis5.0</code> | <code>redis6.x</code> | <code>redis7</code>
         </p> |
| `description` | String | ✅ | <p>A user-specified description for the cache parameter group.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to be added to this resource. A tag is a key-value pair. A tag key must
            be accompanied by a tag value, although null is accepted.</p> |
| `cache_parameter_group_name` | String | ✅ | <p>A user-specified name for the cache parameter group.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cache_parameter_group
cache_parameter_group = provider.elasticache.Cache_parameter_group {
    cache_parameter_group_family = "value"  # <p>The name of the cache parameter group family that the cache parameter group can be
            used with.</p>
         <p>Valid values are: <code>valkey8</code> | <code>valkey7</code> | <code>memcached1.4</code> | <code>memcached1.5</code> | <code>memcached1.6</code> | <code>redis2.6</code> | <code>redis2.8</code> |
<code>redis3.2</code> | <code>redis4.0</code> | <code>redis5.0</code> | <code>redis6.x</code> | <code>redis7</code>
         </p>
    description = "value"  # <p>A user-specified description for the cache parameter group.</p>
    cache_parameter_group_name = "value"  # <p>A user-specified name for the cache parameter group.</p>
}

```

---


### Cache_parameter_groups

CacheParameterGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cache_parameter_groups` | Vec<String> | <p>A list of cache parameter groups. Each element in the list contains detailed
            information about one cache parameter group.</p> |
| `marker` | String | <p>Provides an identifier to allow retrieval of paginated results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cache_parameter_groups outputs
cache_parameter_groups_id = cache_parameter_groups.id
cache_parameter_groups_cache_parameter_groups = cache_parameter_groups.cache_parameter_groups
cache_parameter_groups_marker = cache_parameter_groups.marker
```

---


### Update_actions

UpdateActions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_actions` | Vec<String> | <p>Returns a list of update actions</p> |
| `marker` | String | <p>An optional marker returned from a prior request. Use this marker for pagination of
            results from this operation. If this parameter is specified, the response includes only
            records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access update_actions outputs
update_actions_id = update_actions.id
update_actions_update_actions = update_actions.update_actions
update_actions_marker = update_actions.marker
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
| `marker` | String | <p>An optional marker returned from a prior request. Use this marker for pagination of
            results from this operation. If this parameter is specified, the response includes only
            records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p> |
| `snapshots` | Vec<String> | <p>A list of snapshots. Each item in the list contains detailed information about one
            snapshot.</p> |


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
snapshots_marker = snapshots.marker
snapshots_snapshots = snapshots.snapshots
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple serverless_caches resources
serverless_caches_0 = provider.elasticache.Serverless_caches {
}
serverless_caches_1 = provider.elasticache.Serverless_caches {
}
serverless_caches_2 = provider.elasticache.Serverless_caches {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    serverless_caches = provider.elasticache.Serverless_caches {
    }
```

---

## Related Documentation

- [AWS Elasticache Documentation](https://docs.aws.amazon.com/elasticache/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
