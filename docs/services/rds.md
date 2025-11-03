# Rds Service



**Resources**: 68

---

## Overview

The rds service provides access to 68 resource types:

- [Db_subnet_groups](#db_subnet_groups) [R]
- [Db_shard_groups](#db_shard_groups) [R]
- [Db_instance_read_replica](#db_instance_read_replica) [C]
- [Db_proxies](#db_proxies) [R]
- [Db_proxy_endpoints](#db_proxy_endpoints) [R]
- [Blue_green_deployment](#blue_green_deployment) [CD]
- [Event_categories](#event_categories) [R]
- [Integration](#integration) [CD]
- [Engine_default_cluster_parameters](#engine_default_cluster_parameters) [R]
- [Db_cluster_snapshots](#db_cluster_snapshots) [R]
- [Orderable_db_instance_options](#orderable_db_instance_options) [R]
- [Blue_green_deployments](#blue_green_deployments) [R]
- [Db_cluster_backtracks](#db_cluster_backtracks) [R]
- [Db_cluster_parameter_groups](#db_cluster_parameter_groups) [R]
- [Custom_db_engine_version](#custom_db_engine_version) [CD]
- [Db_subnet_group](#db_subnet_group) [CD]
- [Db_proxy_target_groups](#db_proxy_target_groups) [R]
- [Tenant_database](#tenant_database) [CD]
- [Db_shard_group](#db_shard_group) [CD]
- [Db_snapshots](#db_snapshots) [R]
- [Events](#events) [R]
- [Reserved_db_instances_offerings](#reserved_db_instances_offerings) [R]
- [Engine_default_parameters](#engine_default_parameters) [R]
- [Export_tasks](#export_tasks) [R]
- [Db_cluster_endpoint](#db_cluster_endpoint) [CD]
- [Db_security_groups](#db_security_groups) [R]
- [Db_security_group](#db_security_group) [CD]
- [Db_snapshot](#db_snapshot) [CD]
- [Account_attributes](#account_attributes) [R]
- [Source_regions](#source_regions) [R]
- [Certificates](#certificates) [R]
- [Db_cluster_snapshot_attributes](#db_cluster_snapshot_attributes) [R]
- [Db_instance_automated_backup](#db_instance_automated_backup) [D]
- [Db_cluster_endpoints](#db_cluster_endpoints) [R]
- [Db_engine_versions](#db_engine_versions) [R]
- [Tenant_databases](#tenant_databases) [R]
- [Db_proxy_endpoint](#db_proxy_endpoint) [CD]
- [Db_instance_automated_backups](#db_instance_automated_backups) [R]
- [Db_cluster_automated_backups](#db_cluster_automated_backups) [R]
- [Db_recommendations](#db_recommendations) [R]
- [Db_cluster_automated_backup](#db_cluster_automated_backup) [D]
- [Db_cluster_snapshot](#db_cluster_snapshot) [CD]
- [Db_log_files](#db_log_files) [R]
- [Db_proxy](#db_proxy) [CD]
- [Event_subscription](#event_subscription) [CD]
- [Db_cluster](#db_cluster) [CD]
- [Pending_maintenance_actions](#pending_maintenance_actions) [R]
- [Global_cluster](#global_cluster) [CD]
- [Db_cluster_parameters](#db_cluster_parameters) [R]
- [Db_parameters](#db_parameters) [R]
- [Option_group](#option_group) [CD]
- [Event_subscriptions](#event_subscriptions) [R]
- [Option_group_options](#option_group_options) [R]
- [Db_snapshot_tenant_databases](#db_snapshot_tenant_databases) [R]
- [Db_major_engine_versions](#db_major_engine_versions) [R]
- [Db_instance](#db_instance) [CD]
- [Db_cluster_parameter_group](#db_cluster_parameter_group) [CD]
- [Db_parameter_group](#db_parameter_group) [CD]
- [Db_clusters](#db_clusters) [R]
- [Db_proxy_targets](#db_proxy_targets) [R]
- [Db_parameter_groups](#db_parameter_groups) [R]
- [Integrations](#integrations) [R]
- [Reserved_db_instances](#reserved_db_instances) [R]
- [Db_instances](#db_instances) [R]
- [Option_groups](#option_groups) [R]
- [Valid_db_instance_modifications](#valid_db_instance_modifications) [R]
- [Global_clusters](#global_clusters) [R]
- [Db_snapshot_attributes](#db_snapshot_attributes) [R]

---

## Resources


### Db_subnet_groups

DBSubnetGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous request.
            If this parameter is specified, the response includes
            only records beyond the marker,
            up to the value specified by <code>MaxRecords</code>.</p> |
| `db_subnet_groups` | Vec<String> | <p>A list of <code>DBSubnetGroup</code> instances.</p> |


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
db_subnet_groups_marker = db_subnet_groups.marker
db_subnet_groups_db_subnet_groups = db_subnet_groups.db_subnet_groups
```

---


### Db_shard_groups

DBShardGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>A pagination token that can be used in a later <code>DescribeDBClusters</code> request.</p> |
| `db_shard_groups` | Vec<String> | <p>Contains a list of DB shard groups for the user.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_shard_groups outputs
db_shard_groups_id = db_shard_groups.id
db_shard_groups_marker = db_shard_groups.marker
db_shard_groups_db_shard_groups = db_shard_groups.db_shard_groups
```

---


### Db_instance_read_replica

DBInstanceReadReplica resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  |  |
| `pre_signed_url` | String |  | <p>When you are creating a read replica from one Amazon Web Services GovCloud (US) Region to another or
            from one China Amazon Web Services Region to another, the URL that contains a Signature Version 4
            signed request for the <code>CreateDBInstanceReadReplica</code> API operation in the
            source Amazon Web Services Region that contains the source DB instance.</p>
         <p>This setting applies only to Amazon Web Services GovCloud (US) Regions and 
            China Amazon Web Services Regions. It's ignored in other Amazon Web Services Regions.</p>
         <p>This setting applies only when replicating from a source DB
                <i>instance</i>. Source DB clusters aren't supported in Amazon Web Services GovCloud (US) Regions and China Amazon Web Services Regions.</p>
         <p>You must specify this parameter when you create an encrypted read replica from
            another Amazon Web Services Region by using the Amazon RDS API. Don't specify
                <code>PreSignedUrl</code> when you are creating an encrypted read replica in the
            same Amazon Web Services Region.</p>
         <p>The presigned URL must be a valid request for the
                <code>CreateDBInstanceReadReplica</code> API operation that can run in the
            source Amazon Web Services Region that contains the encrypted source DB instance. The presigned URL
            request must contain the following parameter values:</p>
         <ul>
            <li>
               <p>
                  <code>DestinationRegion</code> - The Amazon Web Services Region that the encrypted read
                    replica is created in. This Amazon Web Services Region is the same one where the
                        <code>CreateDBInstanceReadReplica</code> operation is called that contains
                    this presigned URL.</p>
               <p>For example, if you create an encrypted DB instance in the us-west-1
                    Amazon Web Services Region, from a source DB instance in the us-east-2 Amazon Web Services Region, then you
                    call the <code>CreateDBInstanceReadReplica</code> operation in the us-east-1
                    Amazon Web Services Region and provide a presigned URL that contains a call to the
                        <code>CreateDBInstanceReadReplica</code> operation in the us-west-2
                    Amazon Web Services Region. For this example, the <code>DestinationRegion</code> in the
                    presigned URL must be set to the us-east-1 Amazon Web Services Region.</p>
            </li>
            <li>
               <p>
                  <code>KmsKeyId</code> - The KMS key identifier for the key to use to
                    encrypt the read replica in the destination Amazon Web Services Region. This is the same
                    identifier for both the <code>CreateDBInstanceReadReplica</code> operation that
                    is called in the destination Amazon Web Services Region, and the operation contained in the
                    presigned URL.</p>
            </li>
            <li>
               <p>
                  <code>SourceDBInstanceIdentifier</code> - The DB instance identifier for the
                    encrypted DB instance to be replicated. This identifier must be in the Amazon
                    Resource Name (ARN) format for the source Amazon Web Services Region. For example, if you are
                    creating an encrypted read replica from a DB instance in the us-west-2
                    Amazon Web Services Region, then your <code>SourceDBInstanceIdentifier</code> looks like the
                    following example:
                        <code>arn:aws:rds:us-west-2:123456789012:instance:mysql-instance1-20161115</code>.</p>
            </li>
         </ul>
         <p>To learn how to generate a Signature Version 4 signed request, see 
            <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html">Authenticating Requests: Using Query Parameters (Amazon Web Services Signature Version 4)</a> and
            <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4 Signing Process</a>.</p>
         <note>
            <p>If you are using an Amazon Web Services SDK tool or the CLI, you can specify
                    <code>SourceRegion</code> (or <code>--source-region</code> for the CLI)
                instead of specifying <code>PreSignedUrl</code> manually. Specifying
                    <code>SourceRegion</code> autogenerates a presigned URL that is a valid request
                for the operation that can run in the source Amazon Web Services Region.</p>
         </note>
         <p>This setting doesn't apply to RDS Custom DB instances.</p> |
| `use_default_processor_features` | bool |  | <p>Specifies whether the DB instance class of the DB instance uses its default
            processor features.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p> |
| `port` | i64 |  | <p>The port number that the DB instance uses for connections.</p>
         <p>Valid Values: <code>1150-65535</code>
         </p>
         <p>Default: Inherits the value from the source DB instance.</p> |
| `deletion_protection` | bool |  | <p>Specifies whether to enable deletion protection for the DB instance. 
            The database can't be deleted when deletion protection is enabled. By default, 
            deletion protection isn't enabled. For more information, see 
            <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_DeleteInstance.html">
                Deleting a DB Instance</a>.</p> |
| `source_db_cluster_identifier` | String |  | <p>The identifier of the Multi-AZ DB cluster that will act as the source for the read
            replica. Each DB cluster can have up to 15 read replicas.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be the identifier of an existing Multi-AZ DB cluster.</p>
            </li>
            <li>
               <p>Can't be specified if the <code>SourceDBInstanceIdentifier</code> parameter is
                    also specified.</p>
            </li>
            <li>
               <p>The specified DB cluster must have automatic backups enabled, that is, its
                    backup retention period must be greater than 0.</p>
            </li>
            <li>
               <p>The source DB cluster must be in the same Amazon Web Services Region as the read replica.
                    Cross-Region replication isn't supported.</p>
            </li>
         </ul> |
| `domain_ou` | String |  | <p>The Active Directory organizational unit for your DB instance to join.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be in the distinguished name format.</p>
            </li>
            <li>
               <p>Can't be longer than 64 characters.</p>
            </li>
         </ul>
         <p>Example: <code>OU=mymanagedADtestOU,DC=mymanagedADtest,DC=mymanagedAD,DC=mydomain</code>
         </p> |
| `db_instance_identifier` | String | ✅ | <p>The DB instance identifier of the read replica. This identifier is the unique key
            that identifies a DB instance. This parameter is stored as a lowercase string.</p> |
| `domain_dns_ips` | String |  | <p>The IPv4 DNS IP addresses of your primary and secondary Active Directory domain controllers.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Two IP addresses must be provided.  If there isn't a secondary domain controller, use the IP address of the primary domain controller for both entries in the list.</p>
            </li>
         </ul>
         <p>Example: <code>123.124.125.126,234.235.236.237</code>
         </p> |
| `max_allocated_storage` | i64 |  | <p>The upper limit in gibibytes (GiB) to which Amazon RDS can automatically scale the storage of the DB instance.</p>
         <p>For more information about this setting, including limitations that apply to it, see 
            <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_PIOPS.StorageTypes.html#USER_PIOPS.Autoscaling">
                Managing capacity automatically with Amazon RDS storage autoscaling</a> 
            in the <i>Amazon RDS User Guide</i>.</p> |
| `db_subnet_group_name` | String |  | <p>A DB subnet group for the DB instance. The new DB instance is created in the VPC associated with the DB subnet group. If no DB subnet group is specified, then the new DB instance isn't created in a VPC.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>If supplied, must match the name of an existing DB subnet group.</p>
            </li>
            <li>
               <p>The specified DB subnet group must be in the same Amazon Web Services Region in which the operation is running.</p>
            </li>
            <li>
               <p>All read replicas in one Amazon Web Services Region that are created from the same source DB
                    instance must either:</p>
               <ul>
                  <li>
                     <p>Specify DB subnet groups from the same VPC. All these read replicas are created in the same
                            VPC.</p>
                  </li>
                  <li>
                     <p>Not specify a DB subnet group. All these read replicas are created outside of any
                            VPC.</p>
                  </li>
               </ul>
            </li>
         </ul>
         <p>Example: <code>mydbsubnetgroup</code>
         </p> |
| `upgrade_storage_config` | bool |  | <p>Whether to upgrade the storage file system configuration on the read replica. This option
            migrates the read replica from the old storage file system layout to the preferred layout.</p> |
| `enable_iam_database_authentication` | bool |  | <p>Specifies whether to enable mapping of Amazon Web Services Identity and Access Management
            (IAM) accounts to database accounts. By default, mapping isn't enabled.</p>
         <p>For more information about IAM database authentication, see 
          <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/UsingWithRDS.IAMDBAuth.html">
              IAM Database Authentication for MySQL and PostgreSQL</a> in the <i>Amazon RDS User Guide</i>.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p> |
| `db_instance_class` | String |  | <p>The compute and memory capacity of the read replica, for example
                db.m4.large. Not all DB instance classes are available in all Amazon Web Services
            Regions, or for all database engines. For the full list of DB instance classes, and
            availability for your engine, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Concepts.DBInstanceClass.html">DB Instance Class</a> in the <i>Amazon RDS User Guide</i>.</p>
         <p>Default: Inherits the value from the source DB instance.</p> |
| `storage_type` | String |  | <p>The storage type to associate with the read replica.</p>
         <p>If you specify <code>io1</code>, <code>io2</code>, or <code>gp3</code>, you must also include a value for the
            <code>Iops</code> parameter.</p>
         <p>Valid Values: <code>gp2 | gp3 | io1 | io2 | standard</code>
         </p>
         <p>Default: <code>io1</code> if the <code>Iops</code> parameter is specified. Otherwise,
                <code>gp3</code>.</p> |
| `auto_minor_version_upgrade` | bool |  | <p>Specifies whether to automatically apply minor engine upgrades to the
            read replica during the maintenance window.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p>
         <p>Default: Inherits the value from the source DB instance.</p>
         <p>For more information about automatic minor version upgrades, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_UpgradeDBInstance.Upgrading.html#USER_UpgradeDBInstance.Upgrading.AutoMinorVersionUpgrades">Automatically upgrading the minor engine version</a>.</p> |
| `network_type` | String |  | <p>The network type of the DB instance.</p>
         <p>Valid Values:</p>
         <ul>
            <li>
               <p>
                  <code>IPV4</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DUAL</code>
               </p>
            </li>
         </ul>
         <p>The network type is determined by the <code>DBSubnetGroup</code> specified for read replica. 
            A <code>DBSubnetGroup</code> can support only the IPv4 protocol or the IPv4 and the IPv6 
            protocols (<code>DUAL</code>).</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_VPC.WorkingWithRDSInstanceinaVPC.html">
            Working with a DB instance in a VPC</a> in the 
            <i>Amazon RDS User Guide.</i>
         </p> |
| `domain_auth_secret_arn` | String |  | <p>The ARN for the Secrets Manager secret with the credentials for the user joining the domain.</p>
         <p>Example: <code>arn:aws:secretsmanager:region:account-number:secret:myselfmanagedADtestsecret-123456</code>
         </p> |
| `performance_insights_retention_period` | i64 |  | <p>The number of days to retain Performance Insights data.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p>
         <p>Valid Values:</p>
         <ul>
            <li>
               <p>
                  <code>7</code>
               </p>
            </li>
            <li>
               <p>
                  <i>month</i> * 31, where <i>month</i> is a number of months from 1-23. 
                Examples: <code>93</code> (3 months * 31), <code>341</code> (11 months * 31), <code>589</code> (19 months * 31)</p>
            </li>
            <li>
               <p>
                  <code>731</code>
               </p>
            </li>
         </ul>
         <p>Default: <code>7</code> days</p>
         <p>If you specify a retention period that isn't valid, such as <code>94</code>,  Amazon RDS returns an error.</p> |
| `source_db_instance_identifier` | String |  | <p>The identifier of the DB instance that will act as the source for the read replica.
            Each DB instance can have up to 15 read replicas, except for the following
            engines:</p>
         <ul>
            <li>
               <p>Db2 - Can have up to three replicas.</p>
            </li>
            <li>
               <p>Oracle - Can have up to five read replicas.</p>
            </li>
            <li>
               <p>SQL Server - Can have up to five read replicas.</p>
            </li>
         </ul>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be the identifier of an existing Db2, MariaDB, MySQL, Oracle, PostgreSQL, or SQL Server DB
                    instance.</p>
            </li>
            <li>
               <p>Can't be specified if the <code>SourceDBClusterIdentifier</code> parameter is
                    also specified.</p>
            </li>
            <li>
               <p>For the limitations of Oracle read replicas, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/oracle-read-replicas.limitations.html#oracle-read-replicas.limitations.versions-and-licenses">Version and licensing considerations for RDS for Oracle replicas</a> in the
                  <i>Amazon RDS User Guide</i>.</p>
            </li>
            <li>
               <p>For the limitations of SQL Server read replicas, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/SQLServer.ReadReplicas.html#SQLServer.ReadReplicas.Limitations">Read replica limitations with SQL Server</a> in the <i>Amazon RDS User Guide</i>.</p>
            </li>
            <li>
               <p>The specified DB instance must have automatic backups enabled, that is, its backup
                    retention period must be greater than 0.</p>
            </li>
            <li>
               <p>If the source DB instance is in the same Amazon Web Services Region as the read replica, specify a valid DB
                    instance identifier.</p>
            </li>
            <li>
               <p>If the source DB instance is in a different Amazon Web Services Region from the read
                    replica, specify a valid DB instance ARN. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Tagging.ARN.html#USER_Tagging.ARN.Constructing">Constructing an ARN for Amazon RDS</a> in the <i>Amazon RDS User
                        Guide</i>. This doesn't apply to SQL Server or RDS Custom, which
                    don't support cross-Region replicas.</p>
            </li>
         </ul> |
| `kms_key_id` | String |  | <p>The Amazon Web Services KMS key identifier for an encrypted read replica.</p>
         <p>The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.</p>
         <p>If you create an encrypted read replica in the same Amazon Web Services Region as the source DB
            instance or Multi-AZ DB cluster, don't specify a value for this parameter. A read
            replica in the same Amazon Web Services Region is always encrypted with the same KMS key as the source
            DB instance or cluster.</p>
         <p>If you create an encrypted read replica in a different Amazon Web Services Region, then you must
            specify a KMS key identifier for the destination Amazon Web Services Region. KMS keys are specific to
            the Amazon Web Services Region that they are created in, and you can't use KMS keys from one
            Amazon Web Services Region in another Amazon Web Services Region.</p>
         <p>You can't create an encrypted read replica from an unencrypted DB instance or
            Multi-AZ DB cluster.</p>
         <p>This setting doesn't apply to RDS Custom, which uses the same KMS key as the primary 
            replica.</p> |
| `enable_cloudwatch_logs_exports` | Vec<String> |  | <p>The list of logs that the new DB instance is to export to CloudWatch Logs. The values
            in the list depend on the DB engine being used. For more information, see 
            <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_LogAccess.html#USER_LogAccess.Procedural.UploadtoCloudWatch">Publishing
                Database Logs to Amazon CloudWatch Logs </a> in the <i>Amazon RDS User Guide</i>.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p> |
| `enable_performance_insights` | bool |  | <p>Specifies whether to enable Performance Insights for the read replica.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_PerfInsights.html">Using
            Amazon Performance Insights</a> in the <i>Amazon RDS User Guide</i>.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p> |
| `monitoring_role_arn` | String |  | <p>The ARN for the IAM role that permits RDS to send enhanced monitoring metrics to Amazon CloudWatch Logs. For
      example, <code>arn:aws:iam:123456789012:role/emaccess</code>. For information on creating a monitoring role,
      go to <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Monitoring.html#USER_Monitoring.OS.IAMRole">To 
          create an IAM role for Amazon RDS Enhanced Monitoring</a> in the <i>Amazon RDS User Guide</i>.</p>
         <p>If <code>MonitoringInterval</code> is set to a value other than 0, then you must 
          supply a <code>MonitoringRoleArn</code> value.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p> |
| `processor_features` | Vec<String> |  | <p>The number of CPU cores and the number of threads per core for the DB instance class of the DB instance.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p> |
| `storage_throughput` | i64 |  | <p>Specifies the storage throughput value for the read replica.</p>
         <p>This setting doesn't apply to RDS Custom or Amazon Aurora DB instances.</p> |
| `dedicated_log_volume` | bool |  | <p>Indicates whether the DB instance has a dedicated log volume (DLV) enabled.</p> |
| `db_parameter_group_name` | String |  | <p>The name of the DB parameter group to associate with this read replica DB
            instance.</p>
         <p>For the Db2 DB engine, if your source DB instance uses the bring your own license
            (BYOL) model, then a custom parameter group must be associated with the replica. For a
            same Amazon Web Services Region replica, if you don't specify a custom parameter group, Amazon RDS
            associates the custom parameter group associated with the source DB instance. For a
            cross-Region replica, you must specify a custom parameter group. This custom parameter
            group must include your IBM Site ID and IBM Customer ID. For more information, see
                <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/db2-licensing.html#db2-prereqs-ibm-info">IBM IDs
                for bring your own license (BYOL) for Db2</a>. </p>
         <p>For Single-AZ or Multi-AZ DB instance read replica instances, if you don't specify a
            value for <code>DBParameterGroupName</code>, then Amazon RDS uses the
                <code>DBParameterGroup</code> of the source DB instance for a same Region read
            replica, or the default <code>DBParameterGroup</code> for the specified DB engine for a
            cross-Region read replica.</p>
         <p>For Multi-AZ DB cluster same Region read replica instances, if you don't specify a
            value for <code>DBParameterGroupName</code>, then Amazon RDS uses the default
                <code>DBParameterGroup</code>.</p>
         <p>Specifying a parameter group for this operation is only supported for MySQL DB
            instances for cross-Region read replicas, for Multi-AZ DB cluster read replica
            instances, for Db2 DB instances, and for Oracle DB instances. It isn't supported for
            MySQL DB instances for same Region read replicas or for RDS Custom.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 1 to 255 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Can't end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul> |
| `monitoring_interval` | i64 |  | <p>The interval, in seconds, between points when Enhanced Monitoring metrics are
            collected for the read replica. To disable collection of Enhanced Monitoring metrics,
            specify <code>0</code>. The default is <code>0</code>.</p>
         <p>If <code>MonitoringRoleArn</code> is specified, then you must set <code>MonitoringInterval</code>
      to a value other than <code>0</code>.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p>
         <p>Valid Values: <code>0, 1, 5, 10, 15, 30, 60</code>
         </p>
         <p>Default: <code>0</code>
         </p> |
| `option_group_name` | String |  | <p>The option group to associate the DB instance with. If not specified, RDS uses the option group
            associated with the source DB instance or cluster.</p>
         <note>
            <p>For SQL Server, you must use the option group associated with the source.</p>
         </note>
         <p>This setting doesn't apply to RDS Custom DB instances.</p> |
| `database_insights_mode` | String |  | <p>The mode of Database Insights to enable for the read replica.</p>
         <note>
            <p>This setting isn't supported.</p>
         </note> |
| `enable_customer_owned_ip` | bool |  | <p>Specifies whether to enable a customer-owned IP address (CoIP) for an RDS
            on Outposts read replica.</p>
         <p>A <i>CoIP</i> provides local or external connectivity to resources in
            your Outpost subnets through your on-premises network. For some use cases, a CoIP can
            provide lower latency for connections to the read replica from outside of its virtual
            private cloud (VPC) on your local network.</p>
         <p>For more information about RDS on Outposts, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/rds-on-outposts.html">Working with Amazon RDS on Amazon Web Services Outposts</a> 
            in the <i>Amazon RDS User Guide</i>.</p>
         <p>For more information about CoIPs, see <a href="https://docs.aws.amazon.com/outposts/latest/userguide/routing.html#ip-addressing">Customer-owned IP addresses</a> 
            in the <i>Amazon Web Services Outposts User Guide</i>.</p> |
| `domain` | String |  | <p>The Active Directory directory ID to create the DB instance in. Currently, only MySQL, Microsoft SQL 
            Server, Oracle, and PostgreSQL DB instances can be created in an Active Directory Domain.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/kerberos-authentication.html">
            Kerberos Authentication</a> in the <i>Amazon RDS User Guide</i>.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p> |
| `performance_insights_kms_key_id` | String |  | <p>The Amazon Web Services KMS key identifier for encryption of Performance Insights data.</p>
         <p>The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.</p>
         <p>If you do not specify a value for <code>PerformanceInsightsKMSKeyId</code>, then Amazon RDS 
            uses your default KMS key. There is a default KMS key for your Amazon Web Services account. 
            Your Amazon Web Services account has a different default KMS key for each Amazon Web Services Region.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p> |
| `vpc_security_group_ids` | Vec<String> |  | <p>A list of Amazon EC2 VPC security groups to associate with the read replica.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p>
         <p>Default: The default EC2 VPC security group for the DB subnet group's VPC.</p> |
| `replica_mode` | String |  | <p>The open mode of the replica database.</p>
         <p>This parameter is only supported for Db2 DB instances and Oracle DB
            instances.</p>
         <dl>
            <dt>Db2</dt>
            <dd>
               <p>Standby DB replicas are included in Db2 Advanced Edition (AE) and Db2
                        Standard Edition (SE). The main use case for standby replicas is
                        cross-Region disaster recovery. Because it doesn't accept user
                        connections, a standby replica can't serve a read-only workload.</p>
               <p>You can create a combination of standby and read-only DB replicas for the
                        same primary DB instance. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/db2-replication.html">Working with replicas for Amazon RDS for Db2</a> in the <i>Amazon RDS User
                            Guide</i>.</p>
               <p>To create standby DB replicas for RDS for Db2, set this parameter to
                            <code>mounted</code>.</p>
            </dd>
            <dt>Oracle</dt>
            <dd>
               <p>Mounted DB replicas are included in Oracle Database Enterprise Edition. The main use case for
                    mounted replicas is cross-Region disaster recovery. The primary database doesn't use Active
                    Data Guard to transmit information to the mounted replica. Because it doesn't accept
                    user connections, a mounted replica can't serve a read-only workload.</p>
               <p>You can create a combination of mounted and read-only DB replicas for the same primary DB instance.
                    For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/oracle-read-replicas.html">Working with read replicas for Amazon RDS for Oracle</a> 
                    in the <i>Amazon RDS User Guide</i>.</p>
               <p>For RDS Custom, you must specify this parameter and set it to
                        <code>mounted</code>. The value won't be set by default. After replica
                        creation, you can manage the open mode manually.</p>
            </dd>
         </dl> |
| `backup_target` | String |  | <p>The location where RDS stores automated backups and manual snapshots.</p>
         <p>Valid Values:</p>
         <ul>
            <li>
               <p>
                  <code>local</code> for Dedicated Local Zones</p>
            </li>
            <li>
               <p>
                  <code>region</code> for Amazon Web Services Region</p>
            </li>
         </ul> |
| `availability_zone` | String |  | <p>The Availability Zone (AZ) where the read replica will be created.</p>
         <p>Default: A random, system-chosen Availability Zone in the endpoint's Amazon Web Services Region.</p>
         <p>Example: <code>us-east-1d</code>
         </p> |
| `multi_az` | bool |  | <p>Specifies whether the read replica is in a Multi-AZ deployment.</p>
         <p>You can create a read replica as a Multi-AZ DB instance. RDS creates a standby of your
            replica in another Availability Zone for failover support for the replica. Creating your
            read replica as a Multi-AZ DB instance is independent of whether the source is a
            Multi-AZ DB instance or a Multi-AZ DB cluster.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p> |
| `publicly_accessible` | bool |  | <p>Specifies whether the DB instance is publicly accessible.</p>
         <p>When the DB cluster is publicly accessible, its Domain Name System (DNS) endpoint
          resolves to the private IP address from within the DB cluster's virtual private cloud
          (VPC). It resolves to the public IP address from outside of the DB cluster's VPC. Access
          to the DB cluster is ultimately controlled by the security group it uses. That public
          access isn't permitted if the security group assigned to the DB cluster doesn't permit
          it.</p>
         <p>When the DB instance isn't publicly accessible, it is an internal DB instance with a DNS name that resolves to a private IP address.</p>
         <p>For more information, see <a>CreateDBInstance</a>.</p> |
| `domain_fqdn` | String |  | <p>The fully qualified domain name (FQDN) of an Active Directory domain.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Can't be longer than 64 characters.</p>
            </li>
         </ul>
         <p>Example: <code>mymanagedADtest.mymanagedAD.mydomain</code>
         </p> |
| `copy_tags_to_snapshot` | bool |  | <p>Specifies whether to copy all tags from the read replica to snapshots of
            the read replica. By default, tags aren't copied.</p> |
| `domain_iam_role_name` | String |  | <p>The name of the IAM role to use when making API calls to the Directory
            Service.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p> |
| `custom_iam_instance_profile` | String |  | <p>The instance profile associated with the underlying Amazon EC2 instance of an 
            RDS Custom DB instance. The instance profile must meet the following requirements:</p>
         <ul>
            <li>
               <p>The profile must exist in your account.</p>
            </li>
            <li>
               <p>The profile must have an IAM role that Amazon EC2 has permissions to assume.</p>
            </li>
            <li>
               <p>The instance profile name and the associated IAM role name must start with the prefix <code>AWSRDSCustom</code>.</p>
            </li>
         </ul>
         <p>For the list of permissions required for the IAM role, see 
            <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/custom-setup-orcl.html#custom-setup-orcl.iam-vpc">
                Configure IAM and your VPC</a> in the <i>Amazon RDS User Guide</i>.</p>
         <p>This setting is required for RDS Custom DB instances.</p> |
| `allocated_storage` | i64 |  | <p>The amount of storage (in gibibytes) to allocate initially for the read replica.
            Follow the allocation rules specified in <code>CreateDBInstance</code>.</p>
         <p>This setting isn't valid for RDS for SQL Server.</p>
         <note>
            <p>Be sure to allocate enough storage for your read replica so that the create operation can succeed.
                You can also allocate additional storage for future growth.</p>
         </note> |
| `iops` | i64 |  | <p>The amount of Provisioned IOPS (input/output operations per second) to initially allocate for the DB instance.</p> |
| `ca_certificate_identifier` | String |  | <p>The CA certificate identifier to use for the read replica's server certificate.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/UsingWithRDS.SSL.html">Using SSL/TLS to encrypt a connection to a DB 
                instance</a> in the <i>Amazon RDS User Guide</i> and 
                <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/UsingWithRDS.SSL.html">
                    Using SSL/TLS to encrypt a connection to a DB cluster</a> in the <i>Amazon Aurora 
                        User Guide</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_instance_read_replica
db_instance_read_replica = provider.rds.Db_instance_read_replica {
    db_instance_identifier = "value"  # <p>The DB instance identifier of the read replica. This identifier is the unique key
            that identifies a DB instance. This parameter is stored as a lowercase string.</p>
}

```

---


### Db_proxies

DBProxies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous request.
        If this parameter is specified, the response includes only records beyond the marker,
        up to the value specified by <code>MaxRecords</code>.</p> |
| `db_proxies` | Vec<String> | <p>A return value representing an arbitrary number of <code>DBProxy</code> data structures.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_proxies outputs
db_proxies_id = db_proxies.id
db_proxies_marker = db_proxies.marker
db_proxies_db_proxies = db_proxies.db_proxies
```

---


### Db_proxy_endpoints

DBProxyEndpoints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous request.
        If this parameter is specified, the response includes only records beyond the marker,
        up to the value specified by <code>MaxRecords</code>.</p> |
| `db_proxy_endpoints` | Vec<String> | <p>The list of <code>ProxyEndpoint</code> objects returned by the API operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_proxy_endpoints outputs
db_proxy_endpoints_id = db_proxy_endpoints.id
db_proxy_endpoints_marker = db_proxy_endpoints.marker
db_proxy_endpoints_db_proxy_endpoints = db_proxy_endpoints.db_proxy_endpoints
```

---


### Blue_green_deployment

BlueGreenDeployment resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `upgrade_target_storage_config` | bool |  | <p>Whether to upgrade the storage file system configuration on the green database. This
            option migrates the green DB instance from the older 32-bit file system to the preferred
            configuration. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_PIOPS.StorageTypes.html#USER_PIOPS.UpgradeFileSystem">Upgrading the storage file system for a DB instance</a>.</p> |
| `target_storage_type` | String |  | <p>The storage type to associate with the green DB instance.</p>
         <p>Valid Values: <code>gp2 | gp3 | io1 | io2</code>
         </p>
         <p>This setting doesn't apply to Amazon Aurora blue/green deployments.</p> |
| `blue_green_deployment_name` | String | ✅ | <p>The name of the blue/green deployment.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Can't be the same as an existing blue/green deployment name in the same account and Amazon Web Services Region.</p>
            </li>
         </ul> |
| `tags` | Vec<String> |  | <p>Tags to assign to the blue/green deployment.</p> |
| `target_iops` | i64 |  | <p>The amount of Provisioned IOPS (input/output operations per second) to allocate for the green DB instance.
            For information about valid IOPS values, see 
            <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_Storage.html">Amazon RDS DB instance storage</a> 
            in the <i>Amazon RDS User Guide</i>.</p>
         <p>This setting doesn't apply to Amazon Aurora blue/green deployments.</p> |
| `target_storage_throughput` | i64 |  | <p>The storage throughput value for the green DB instance.</p>
         <p>This setting applies only to the <code>gp3</code> storage type.</p>
         <p>This setting doesn't apply to Amazon Aurora blue/green deployments.</p> |
| `target_db_parameter_group_name` | String |  | <p>The DB parameter group associated with the DB instance in the green environment.</p>
         <p>To test parameter changes, specify a DB parameter group that is different from the one associated 
            with the source DB instance.</p> |
| `target_engine_version` | String |  | <p>The engine version of the database in the green environment.</p>
         <p>Specify the engine version to upgrade to in the green environment.</p> |
| `target_db_cluster_parameter_group_name` | String |  | <p>The DB cluster parameter group associated with the Aurora DB cluster in the green environment.</p>
         <p>To test parameter changes, specify a DB cluster parameter group that is different from the one associated 
            with the source DB cluster.</p> |
| `source` | String | ✅ | <p>The Amazon Resource Name (ARN) of the source production database.</p>
         <p>Specify the database that you want to clone. The blue/green deployment creates this database in 
           the green environment. You can make updates to the database in the green environment, such as an engine 
           version upgrade. When you are ready, you can switch the database in the green environment to be the 
           production database.</p> |
| `target_allocated_storage` | i64 |  | <p>The amount of storage in gibibytes (GiB) to allocate for the green DB instance. You can choose to
            increase or decrease the allocated storage on the green DB instance.</p>
         <p>This setting doesn't apply to Amazon Aurora blue/green deployments.</p> |
| `target_db_instance_class` | String |  | <p>Specify the DB instance class for the databases in the green environment.</p>
         <p>This parameter only applies to RDS DB instances, because DB instances within an Aurora DB cluster can
        have multiple different instance classes. If you're creating a blue/green deployment from an Aurora DB cluster,
        don't specify this parameter. After the green environment is created, you can individually modify the instance classes 
        of the DB instances within the green DB cluster.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create blue_green_deployment
blue_green_deployment = provider.rds.Blue_green_deployment {
    blue_green_deployment_name = "value"  # <p>The name of the blue/green deployment.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Can't be the same as an existing blue/green deployment name in the same account and Amazon Web Services Region.</p>
            </li>
         </ul>
    source = "value"  # <p>The Amazon Resource Name (ARN) of the source production database.</p>
         <p>Specify the database that you want to clone. The blue/green deployment creates this database in 
           the green environment. You can make updates to the database in the green environment, such as an engine 
           version upgrade. When you are ready, you can switch the database in the green environment to be the 
           production database.</p>
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
| `event_categories_map_list` | Vec<String> | <p>A list of <code>EventCategoriesMap</code> data types.</p> |


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


### Integration

Integration resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description of the integration.</p> |
| `source_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the database to use as the source for
            replication.</p> |
| `kms_key_id` | String |  | <p>The Amazon Web Services Key Management System (Amazon Web Services KMS) key identifier for the key to use to
            encrypt the integration. If you don't specify an encryption key, RDS uses a default
            Amazon Web Services owned key. </p> |
| `additional_encryption_context` | HashMap<String, String> |  | <p>An optional set of non-secret key–value pairs that contains additional contextual
            information about the data. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption
                context</a> in the <i>Amazon Web Services Key Management Service Developer
                Guide</i>.</p>
         <p>You can only include this parameter if you specify the <code>KMSKeyId</code> parameter.</p> |
| `tags` | Vec<String> |  |  |
| `data_filter` | String |  | <p>Data filtering options for the integration. For more information, see 
            <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/zero-etl.filtering.html">Data filtering for Aurora zero-ETL integrations with Amazon Redshift</a>
            or
            <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/zero-etl.filtering.html">Data filtering for Amazon RDS zero-ETL integrations with Amazon Redshift</a>.
        </p> |
| `target_arn` | String | ✅ | <p>The ARN of the Redshift data warehouse to use as the target for replication.</p> |
| `integration_name` | String | ✅ | <p>The name of the integration.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create integration
integration = provider.rds.Integration {
    source_arn = "value"  # <p>The Amazon Resource Name (ARN) of the database to use as the source for
            replication.</p>
    target_arn = "value"  # <p>The ARN of the Redshift data warehouse to use as the target for replication.</p>
    integration_name = "value"  # <p>The name of the integration.</p>
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


### Db_cluster_snapshots

DBClusterSnapshots resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous
          <code>DescribeDBClusterSnapshots</code> request.
      If this parameter is specified, the response includes
      only records beyond the marker,
      up to the value specified by <code>MaxRecords</code>.</p> |
| `db_cluster_snapshots` | Vec<String> | <p>Provides a list of DB cluster snapshots for the user.</p> |


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
db_cluster_snapshots_marker = db_cluster_snapshots.marker
db_cluster_snapshots_db_cluster_snapshots = db_cluster_snapshots.db_cluster_snapshots
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
| `orderable_db_instance_options` | Vec<String> | <p>An <code>OrderableDBInstanceOption</code> structure containing information about orderable options for the DB instance.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous 
            OrderableDBInstanceOptions request.
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

# Access orderable_db_instance_options outputs
orderable_db_instance_options_id = orderable_db_instance_options.id
orderable_db_instance_options_orderable_db_instance_options = orderable_db_instance_options.orderable_db_instance_options
orderable_db_instance_options_marker = orderable_db_instance_options.marker
```

---


### Blue_green_deployments

BlueGreenDeployments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>A pagination token that can be used in a later
                <code>DescribeBlueGreenDeployments</code> request.</p> |
| `blue_green_deployments` | Vec<String> | <p>A list of blue/green deployments in the current account and Amazon Web Services Region.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access blue_green_deployments outputs
blue_green_deployments_id = blue_green_deployments.id
blue_green_deployments_marker = blue_green_deployments.marker
blue_green_deployments_blue_green_deployments = blue_green_deployments.blue_green_deployments
```

---


### Db_cluster_backtracks

DBClusterBacktracks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_cluster_backtracks` | Vec<String> | <p>Contains a list of backtracks for the user.</p> |
| `marker` | String | <p>A pagination token that can be used in a later <code>DescribeDBClusterBacktracks</code> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_cluster_backtracks outputs
db_cluster_backtracks_id = db_cluster_backtracks.id
db_cluster_backtracks_db_cluster_backtracks = db_cluster_backtracks.db_cluster_backtracks
db_cluster_backtracks_marker = db_cluster_backtracks.marker
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
| `marker` | String | <p>An optional pagination token provided by a previous
            <code>DescribeDBClusterParameterGroups</code> request.
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

# Access db_cluster_parameter_groups outputs
db_cluster_parameter_groups_id = db_cluster_parameter_groups.id
db_cluster_parameter_groups_db_cluster_parameter_groups = db_cluster_parameter_groups.db_cluster_parameter_groups
db_cluster_parameter_groups_marker = db_cluster_parameter_groups.marker
```

---


### Custom_db_engine_version

CustomDBEngineVersion resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `engine_version` | String | ✅ | <p>The name of your CEV. The name format is 19.<i>customized_string</i>.
            For example, a valid CEV name is <code>19.my_cev1</code>. This setting is required for RDS
            Custom for Oracle, but optional for Amazon RDS. The combination of <code>Engine</code>
            and <code>EngineVersion</code> is unique per customer per Region.</p> |
| `use_aws_provided_latest_image` | bool |  | <p>Specifies whether to use the latest service-provided Amazon Machine Image (AMI) for
            the CEV. If you specify <code>UseAwsProvidedLatestImage</code>, you can't also specify
                <code>ImageId</code>.</p> |
| `tags` | Vec<String> |  |  |
| `kms_key_id` | String |  | <p>The Amazon Web Services KMS key identifier for an encrypted CEV. A symmetric encryption KMS key is required for 
            RDS Custom, but optional for Amazon RDS.</p>
         <p>If you have an existing symmetric encryption KMS key in your account, you can use it with RDS Custom. 
            No further action is necessary. If you don't already have a symmetric encryption KMS key in your account, 
            follow the instructions in <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keys.html#create-symmetric-cmk">
                Creating a symmetric encryption KMS key</a> in the <i>Amazon Web Services Key Management Service
                    Developer Guide</i>.</p>
         <p>You can choose the same symmetric encryption key when you create a CEV and a DB instance, or choose different keys.</p> |
| `source_custom_db_engine_version_identifier` | String |  | <p>The ARN of a CEV to use as a source for creating a new CEV. You can specify a different
            Amazon Machine Imagine (AMI) by using either <code>Source</code> or
                <code>UseAwsProvidedLatestImage</code>. You can't specify a different JSON manifest
            when you specify <code>SourceCustomDbEngineVersionIdentifier</code>.</p> |
| `engine` | String | ✅ | <p>The database engine. RDS Custom for Oracle supports the following values:</p>
         <ul>
            <li>
               <p>
                  <code>custom-oracle-ee</code>
               </p>
            </li>
            <li>
               <p>
                  <code>custom-oracle-ee-cdb</code>
               </p>
            </li>
            <li>
               <p>
                  <code>custom-oracle-se2</code>
               </p>
            </li>
            <li>
               <p>
                  <code>custom-oracle-se2-cdb</code>
               </p>
            </li>
         </ul> |
| `database_installation_files_s3_bucket_name` | String |  | <p>The name of an Amazon S3 bucket that contains database installation files for your CEV. For example, a valid 
            bucket name is <code>my-custom-installation-files</code>.</p> |
| `description` | String |  | <p>An optional description of your CEV.</p> |
| `image_id` | String |  | <p>The ID of the Amazon Machine Image (AMI). For RDS Custom for SQL Server, an AMI ID is required 
            to create a CEV. For RDS Custom for Oracle, the default is the most recent AMI available, 
            but you can specify an AMI ID that was used in a different Oracle CEV. Find the AMIs 
            used by your CEVs by calling the <a href="https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_DescribeDBEngineVersions.html">DescribeDBEngineVersions</a> operation.</p> |
| `manifest` | String |  | <p>The CEV manifest, which is a JSON document that describes the installation .zip files stored in Amazon S3. 
            Specify the name/value pairs in a file or a quoted string. RDS Custom applies the patches in the order in which 
            they are listed.</p>
         <p>The following JSON fields are valid:</p>
         <dl>
            <dt>MediaImportTemplateVersion</dt>
            <dd>
               <p>Version of the CEV manifest. The date is in the format <code>YYYY-MM-DD</code>.</p>
            </dd>
            <dt>databaseInstallationFileNames</dt>
            <dd>
               <p>Ordered list of installation files for the CEV.</p>
            </dd>
            <dt>opatchFileNames</dt>
            <dd>
               <p>Ordered list of OPatch installers used for the Oracle DB engine.</p>
            </dd>
            <dt>psuRuPatchFileNames</dt>
            <dd>
               <p>The PSU and RU patches for this CEV.</p>
            </dd>
            <dt>OtherPatchFileNames</dt>
            <dd>
               <p>The patches that are not in the list of PSU and RU patches. 
                    Amazon RDS applies these patches after applying the PSU and RU patches.</p>
            </dd>
         </dl>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/custom-cev.html#custom-cev.preparing.manifest">
            Creating the CEV manifest</a> in the <i>Amazon RDS User Guide</i>.</p> |
| `database_installation_files_s3_prefix` | String |  | <p>The Amazon S3 directory that contains the database installation files for your CEV. For example, a valid 
            bucket name is <code>123456789012/cev1</code>. If this setting isn't specified, no prefix is assumed.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create custom_db_engine_version
custom_db_engine_version = provider.rds.Custom_db_engine_version {
    engine_version = "value"  # <p>The name of your CEV. The name format is 19.<i>customized_string</i>.
            For example, a valid CEV name is <code>19.my_cev1</code>. This setting is required for RDS
            Custom for Oracle, but optional for Amazon RDS. The combination of <code>Engine</code>
            and <code>EngineVersion</code> is unique per customer per Region.</p>
    engine = "value"  # <p>The database engine. RDS Custom for Oracle supports the following values:</p>
         <ul>
            <li>
               <p>
                  <code>custom-oracle-ee</code>
               </p>
            </li>
            <li>
               <p>
                  <code>custom-oracle-ee-cdb</code>
               </p>
            </li>
            <li>
               <p>
                  <code>custom-oracle-se2</code>
               </p>
            </li>
            <li>
               <p>
                  <code>custom-oracle-se2-cdb</code>
               </p>
            </li>
         </ul>
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
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain no more than 255 letters, numbers, periods, underscores, spaces, or hyphens.</p>
            </li>
            <li>
               <p>Must not be default.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
         </ul>
         <p>Example: <code>mydbsubnetgroup</code>
         </p> |
| `subnet_ids` | Vec<String> | ✅ | <p>The EC2 Subnet IDs for the DB subnet group.</p> |
| `tags` | Vec<String> |  | <p>Tags to assign to the DB subnet group.</p> |
| `db_subnet_group_description` | String | ✅ | <p>The description for the DB subnet group.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_subnet_group
db_subnet_group = provider.rds.Db_subnet_group {
    db_subnet_group_name = "value"  # <p>The name for the DB subnet group. This value is stored as a lowercase string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain no more than 255 letters, numbers, periods, underscores, spaces, or hyphens.</p>
            </li>
            <li>
               <p>Must not be default.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
         </ul>
         <p>Example: <code>mydbsubnetgroup</code>
         </p>
    subnet_ids = "value"  # <p>The EC2 Subnet IDs for the DB subnet group.</p>
    db_subnet_group_description = "value"  # <p>The description for the DB subnet group.</p>
}

```

---


### Db_proxy_target_groups

DBProxyTargetGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous request.
        If this parameter is specified, the response includes only records beyond the marker,
        up to the value specified by <code>MaxRecords</code>.</p> |
| `target_groups` | Vec<String> | <p>An arbitrary number of <code>DBProxyTargetGroup</code> objects, containing details of the corresponding target groups.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_proxy_target_groups outputs
db_proxy_target_groups_id = db_proxy_target_groups.id
db_proxy_target_groups_marker = db_proxy_target_groups.marker
db_proxy_target_groups_target_groups = db_proxy_target_groups.target_groups
```

---


### Tenant_database

TenantDatabase resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `nchar_character_set_name` | String |  | <p>The <code>NCHAR</code> value for the tenant database.</p> |
| `character_set_name` | String |  | <p>The character set for your tenant database. If you don't specify a value, the
            character set name defaults to <code>AL32UTF8</code>.</p> |
| `db_instance_identifier` | String | ✅ | <p>The user-supplied DB instance identifier. RDS creates your tenant database in this DB
            instance. This parameter isn't case-sensitive.</p> |
| `master_user_secret_kms_key_id` | String |  | <p>The Amazon Web Services KMS key identifier to encrypt a secret that is automatically generated and 
            managed in Amazon Web Services Secrets Manager.</p>
         <p>This setting is valid only if the master user password is managed by RDS in Amazon Web Services Secrets 
            Manager for the DB instance.</p>
         <p>The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.
            To use a KMS key in a different Amazon Web Services account, specify the key ARN or alias ARN.</p>
         <p>If you don't specify <code>MasterUserSecretKmsKeyId</code>, then the <code>aws/secretsmanager</code> 
            KMS key is used to encrypt the secret. If the secret is in a different Amazon Web Services account, then you can't 
            use the <code>aws/secretsmanager</code> KMS key to encrypt the secret, and you must use a customer 
            managed KMS key.</p>
         <p>There is a default KMS key for your Amazon Web Services account. Your Amazon Web Services account
            has a different default KMS key for each Amazon Web Services Region.</p> |
| `master_user_password` | String |  | <p>The password for the master user in your tenant database.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 8 to 30 characters.</p>
            </li>
            <li>
               <p>Can include any printable ASCII character except forward slash
                    (<code>/</code>), double quote (<code>"</code>), at symbol (<code>@</code>),
                    ampersand (<code>&</code>), or single quote (<code>'</code>).</p>
            </li>
            <li>
               <p>Can't be specified when <code>ManageMasterUserPassword</code> is
                    enabled.</p>
            </li>
         </ul> |
| `manage_master_user_password` | bool |  | <p>Specifies whether to manage the master user password with Amazon Web Services Secrets Manager.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/rds-secrets-manager.html">Password management with Amazon Web Services Secrets Manager</a> 
            in the <i>Amazon RDS User Guide.</i>
         </p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Can't manage the master user password with Amazon Web Services Secrets Manager if <code>MasterUserPassword</code> 
                    is specified.</p>
            </li>
         </ul> |
| `tags` | Vec<String> |  |  |
| `tenant_db_name` | String | ✅ | <p>The user-supplied name of the tenant database that you want to create in your DB
            instance. This parameter has the same constraints as <code>DBName</code> in
                <code>CreateDBInstance</code>.</p> |
| `master_username` | String | ✅ | <p>The name for the master user account in your tenant database. RDS creates this user
            account in the tenant database and grants privileges to the master user. This parameter
            is case-sensitive.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 1 to 16 letters, numbers, or underscores.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Can't be a reserved word for the chosen database engine.</p>
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

# Create tenant_database
tenant_database = provider.rds.Tenant_database {
    db_instance_identifier = "value"  # <p>The user-supplied DB instance identifier. RDS creates your tenant database in this DB
            instance. This parameter isn't case-sensitive.</p>
    tenant_db_name = "value"  # <p>The user-supplied name of the tenant database that you want to create in your DB
            instance. This parameter has the same constraints as <code>DBName</code> in
                <code>CreateDBInstance</code>.</p>
    master_username = "value"  # <p>The name for the master user account in your tenant database. RDS creates this user
            account in the tenant database and grants privileges to the master user. This parameter
            is case-sensitive.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 1 to 16 letters, numbers, or underscores.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Can't be a reserved word for the chosen database engine.</p>
            </li>
         </ul>
}

```

---


### Db_shard_group

DBShardGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `db_cluster_identifier` | String | ✅ | <p>The name of the primary DB cluster for the DB shard group.</p> |
| `db_shard_group_identifier` | String | ✅ | <p>The name of the DB shard group.</p> |
| `tags` | Vec<String> |  |  |
| `compute_redundancy` | i64 |  | <p>Specifies whether to create standby standby DB data access shard for the DB shard group. 
            Valid values are the following:</p>
         <ul>
            <li>
               <p>0 - Creates a DB shard group without a standby DB data access shard. This is the default value.</p>
            </li>
            <li>
               <p>1 - Creates a DB shard group with a standby DB data access shard in a different Availability Zone (AZ).</p>
            </li>
            <li>
               <p>2 - Creates a DB shard group with two standby DB data access shard in two different AZs.</p>
            </li>
         </ul> |
| `max_acu` | f64 | ✅ | <p>The maximum capacity of the DB shard group in Aurora capacity units (ACUs).</p> |
| `min_acu` | f64 |  | <p>The minimum capacity of the DB shard group in Aurora capacity units (ACUs).</p> |
| `publicly_accessible` | bool |  | <p>Specifies whether the DB shard group is publicly accessible.</p>
         <p>When the DB shard group is publicly accessible, its Domain Name System (DNS) endpoint resolves to the private IP address from 
            within the DB shard group's virtual private cloud (VPC). It resolves to the public IP address from outside of the DB shard group's VPC. 
            Access to the DB shard group is ultimately controlled by the security group it uses. 
            That public access is not permitted if the security group assigned to the DB shard group doesn't permit it.</p>
         <p>When the DB shard group isn't publicly accessible, it is an internal DB shard group with a DNS name that resolves to a private IP address.</p>
         <p>Default: The default behavior varies depending on whether <code>DBSubnetGroupName</code> is specified.</p>
         <p>If <code>DBSubnetGroupName</code> isn't specified, and <code>PubliclyAccessible</code> isn't specified, the following applies:</p>
         <ul>
            <li>
               <p>If the default VPC in the target Region doesn’t have an internet gateway attached to it, the DB shard group is private.</p>
            </li>
            <li>
               <p>If the default VPC in the target Region has an internet gateway attached to it, the DB shard group is public.</p>
            </li>
         </ul>
         <p>If <code>DBSubnetGroupName</code> is specified, and <code>PubliclyAccessible</code> isn't specified, the following applies:</p>
         <ul>
            <li>
               <p>If the subnets are part of a VPC that doesn’t have an internet gateway attached to it, the DB shard group is private.</p>
            </li>
            <li>
               <p>If the subnets are part of a VPC that has an internet gateway attached to it, the DB shard group is public.</p>
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

# Create db_shard_group
db_shard_group = provider.rds.Db_shard_group {
    db_cluster_identifier = "value"  # <p>The name of the primary DB cluster for the DB shard group.</p>
    db_shard_group_identifier = "value"  # <p>The name of the DB shard group.</p>
    max_acu = "value"  # <p>The maximum capacity of the DB shard group in Aurora capacity units (ACUs).</p>
}

```

---


### Db_snapshots

DBSnapshots resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous request.
            If this parameter is specified, the response includes
            only records beyond the marker,
            up to the value specified by <code>MaxRecords</code>.</p> |
| `db_snapshots` | Vec<String> | <p>A list of <code>DBSnapshot</code> instances.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_snapshots outputs
db_snapshots_id = db_snapshots.id
db_snapshots_marker = db_snapshots.marker
db_snapshots_db_snapshots = db_snapshots.db_snapshots
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
| `events` | Vec<String> | <p>A list of <code>Event</code> instances.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous 
            Events request.
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

# Access events outputs
events_id = events.id
events_events = events.events
events_marker = events.marker
```

---


### Reserved_db_instances_offerings

ReservedDBInstancesOfferings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reserved_db_instances_offerings` | Vec<String> | <p>A list of reserved DB instance offerings.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous request.
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

# Access reserved_db_instances_offerings outputs
reserved_db_instances_offerings_id = reserved_db_instances_offerings.id
reserved_db_instances_offerings_reserved_db_instances_offerings = reserved_db_instances_offerings.reserved_db_instances_offerings
reserved_db_instances_offerings_marker = reserved_db_instances_offerings.marker
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


### Export_tasks

ExportTasks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>A pagination token that can be used in a later <code>DescribeExportTasks</code>
            request. A marker is used for pagination to identify the location to begin output for
            the next response of <code>DescribeExportTasks</code>.</p> |
| `export_tasks` | Vec<String> | <p>Information about an export of a snapshot or cluster to Amazon S3.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access export_tasks outputs
export_tasks_id = export_tasks.id
export_tasks_marker = export_tasks.marker
export_tasks_export_tasks = export_tasks.export_tasks
```

---


### Db_cluster_endpoint

DBClusterEndpoint resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `db_cluster_identifier` | String | ✅ | <p>The DB cluster identifier of the DB cluster associated with the endpoint. This parameter is
            stored as a lowercase string.</p> |
| `static_members` | String |  | <p>List of DB instance identifiers that are part of the custom endpoint group.</p> |
| `db_cluster_endpoint_identifier` | String | ✅ | <p>The identifier to use for the new endpoint. This parameter is stored as a lowercase string.</p> |
| `endpoint_type` | String | ✅ | <p>The type of the endpoint, one of: <code>READER</code>, <code>WRITER</code>, <code>ANY</code>.</p> |
| `tags` | Vec<String> |  | <p>The tags to be assigned to the Amazon RDS resource.</p> |
| `excluded_members` | String |  | <p>List of DB instance identifiers that aren't part of the custom endpoint group.
            All other eligible instances are reachable through the custom endpoint.
            This parameter is relevant only if the list of static members is empty.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_cluster_endpoint
db_cluster_endpoint = provider.rds.Db_cluster_endpoint {
    db_cluster_identifier = "value"  # <p>The DB cluster identifier of the DB cluster associated with the endpoint. This parameter is
            stored as a lowercase string.</p>
    db_cluster_endpoint_identifier = "value"  # <p>The identifier to use for the new endpoint. This parameter is stored as a lowercase string.</p>
    endpoint_type = "value"  # <p>The type of the endpoint, one of: <code>READER</code>, <code>WRITER</code>, <code>ANY</code>.</p>
}

```

---


### Db_security_groups

DBSecurityGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous request.
            If this parameter is specified, the response includes
            only records beyond the marker,
            up to the value specified by <code>MaxRecords</code>.</p> |
| `db_security_groups` | Vec<String> | <p>A list of <code>DBSecurityGroup</code> instances.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_security_groups outputs
db_security_groups_id = db_security_groups.id
db_security_groups_marker = db_security_groups.marker
db_security_groups_db_security_groups = db_security_groups.db_security_groups
```

---


### Db_security_group

DBSecurityGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `db_security_group_name` | String | ✅ | <p>The name for the DB security group. This value is stored as a lowercase string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 1 to 255 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter</p>
            </li>
            <li>
               <p>Can't end with a hyphen or contain two consecutive hyphens</p>
            </li>
            <li>
               <p>Must not be "Default"</p>
            </li>
         </ul>
         <p>Example: <code>mysecuritygroup</code>
         </p> |
| `db_security_group_description` | String | ✅ | <p>The description for the DB security group.</p> |
| `tags` | Vec<String> |  | <p>Tags to assign to the DB security group.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_security_group
db_security_group = provider.rds.Db_security_group {
    db_security_group_name = "value"  # <p>The name for the DB security group. This value is stored as a lowercase string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 1 to 255 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter</p>
            </li>
            <li>
               <p>Can't end with a hyphen or contain two consecutive hyphens</p>
            </li>
            <li>
               <p>Must not be "Default"</p>
            </li>
         </ul>
         <p>Example: <code>mysecuritygroup</code>
         </p>
    db_security_group_description = "value"  # <p>The description for the DB security group.</p>
}

```

---


### Db_snapshot

DBSnapshot resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `db_instance_identifier` | String | ✅ | <p>The identifier of the DB instance that you want to create the snapshot of.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must match the identifier of an existing DBInstance.</p>
            </li>
         </ul> |
| `db_snapshot_identifier` | String | ✅ | <p>The identifier for the DB snapshot.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Can't be null, empty, or blank</p>
            </li>
            <li>
               <p>Must contain from 1 to 255 letters, numbers, or hyphens</p>
            </li>
            <li>
               <p>First character must be a letter</p>
            </li>
            <li>
               <p>Can't end with a hyphen or contain two consecutive hyphens</p>
            </li>
         </ul>
         <p>Example: <code>my-snapshot-id</code>
         </p> |
| `tags` | Vec<String> |  |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_snapshot
db_snapshot = provider.rds.Db_snapshot {
    db_instance_identifier = "value"  # <p>The identifier of the DB instance that you want to create the snapshot of.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must match the identifier of an existing DBInstance.</p>
            </li>
         </ul>
    db_snapshot_identifier = "value"  # <p>The identifier for the DB snapshot.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Can't be null, empty, or blank</p>
            </li>
            <li>
               <p>Must contain from 1 to 255 letters, numbers, or hyphens</p>
            </li>
            <li>
               <p>First character must be a letter</p>
            </li>
            <li>
               <p>Can't end with a hyphen or contain two consecutive hyphens</p>
            </li>
         </ul>
         <p>Example: <code>my-snapshot-id</code>
         </p>
}

```

---


### Account_attributes

AccountAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_quotas` | Vec<String> | <p>A list of <code>AccountQuota</code> objects. Within this list, each quota has a name, 
            a count of usage toward the quota maximum, and a maximum value for the quota.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_attributes outputs
account_attributes_id = account_attributes.id
account_attributes_account_quotas = account_attributes.account_quotas
```

---


### Source_regions

SourceRegions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `source_regions` | Vec<String> | <p>A list of <code>SourceRegion</code> instances that contains each source Amazon Web Services Region that the
            current Amazon Web Services Region can get a read replica or a DB snapshot from.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous request.
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

# Access source_regions outputs
source_regions_id = source_regions.id
source_regions_source_regions = source_regions.source_regions
source_regions_marker = source_regions.marker
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
| `marker` | String | <p>An optional pagination token provided by a previous
            <code>DescribeCertificates</code> request.
            If this parameter is specified, the response includes
            only records beyond the marker,
            up to the value specified by <code>MaxRecords</code> .</p> |
| `certificates` | Vec<String> | <p>The list of <code>Certificate</code> objects for the Amazon Web Services account.</p> |
| `default_certificate_for_new_launches` | String | <p>The default root CA for new databases created by your Amazon Web Services account. This is either the root CA override 
            set on your Amazon Web Services account or the system default CA for the Region if no override exists. To override the default CA, use the 
            <code>ModifyCertificates</code> operation.</p> |


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
certificates_marker = certificates.marker
certificates_certificates = certificates.certificates
certificates_default_certificate_for_new_launches = certificates.default_certificate_for_new_launches
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


### Db_instance_automated_backup

DBInstanceAutomatedBackup resource

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


### Db_cluster_endpoints

DBClusterEndpoints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous
            <code>DescribeDBClusterEndpoints</code> request.
            If this parameter is specified, the response includes
            only records beyond the marker,
            up to the value specified by <code>MaxRecords</code>.</p> |
| `db_cluster_endpoints` | Vec<String> | <p>Contains the details of the endpoints associated with the cluster
       and matching any filter conditions.</p> |


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
db_cluster_endpoints_marker = db_cluster_endpoints.marker
db_cluster_endpoints_db_cluster_endpoints = db_cluster_endpoints.db_cluster_endpoints
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
| `marker` | String | <p>An optional pagination token provided by a previous request.
            If this parameter is specified, the response includes
            only records beyond the marker,
            up to the value specified by <code>MaxRecords</code>.</p> |
| `db_engine_versions` | Vec<String> | <p>A list of <code>DBEngineVersion</code> elements.</p> |


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


### Tenant_databases

TenantDatabases resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tenant_databases` | Vec<String> | <p>An array of the tenant databases requested by the <code>DescribeTenantDatabases</code>
            operation.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous
                <code>DescribeTenantDatabases</code> request. If this parameter is specified, the
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

# Access tenant_databases outputs
tenant_databases_id = tenant_databases.id
tenant_databases_tenant_databases = tenant_databases.tenant_databases
tenant_databases_marker = tenant_databases.marker
```

---


### Db_proxy_endpoint

DBProxyEndpoint resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vpc_security_group_ids` | String |  | <p>The VPC security group IDs for the DB proxy endpoint that you create. You can
        specify a different set of security group IDs than for the original DB proxy.
        The default is the default security group for the VPC.</p> |
| `target_role` | String |  | <p>The role of the DB proxy endpoint. The role determines whether the endpoint can be used for read/write
        or only read operations. The default is <code>READ_WRITE</code>. The only role that proxies for RDS for Microsoft SQL Server 
        support is <code>READ_WRITE</code>.</p> |
| `vpc_subnet_ids` | String | ✅ | <p>The VPC subnet IDs for the DB proxy endpoint that you create. You can specify a
        different set of subnet IDs than for the original DB proxy.</p> |
| `endpoint_network_type` | String |  | <p>The network type of the DB proxy endpoint. The network type determines the IP version that the proxy endpoint supports.</p>
         <p>Valid values:</p>
         <ul>
            <li>
               <p>
                  <code>IPV4</code> - The proxy endpoint supports IPv4 only.</p>
            </li>
            <li>
               <p>
                  <code>IPV6</code> - The proxy endpoint supports IPv6 only.</p>
            </li>
            <li>
               <p>
                  <code>DUAL</code> - The proxy endpoint supports both IPv4 and IPv6.</p>
            </li>
         </ul>
         <p>Default: <code>IPV4</code>
         </p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>If you specify <code>IPV6</code> or <code>DUAL</code>, the VPC and all subnets must have an IPv6 CIDR block.</p>
            </li>
            <li>
               <p>If you specify <code>IPV6</code> or <code>DUAL</code>, the VPC tenancy cannot be <code>dedicated</code>.</p>
            </li>
         </ul> |
| `tags` | Vec<String> |  |  |
| `db_proxy_name` | String | ✅ | <p>The name of the DB proxy associated with the DB proxy endpoint that you create.</p> |
| `db_proxy_endpoint_name` | String | ✅ | <p>The name of the DB proxy endpoint to create.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_proxy_endpoint
db_proxy_endpoint = provider.rds.Db_proxy_endpoint {
    vpc_subnet_ids = "value"  # <p>The VPC subnet IDs for the DB proxy endpoint that you create. You can specify a
        different set of subnet IDs than for the original DB proxy.</p>
    db_proxy_name = "value"  # <p>The name of the DB proxy associated with the DB proxy endpoint that you create.</p>
    db_proxy_endpoint_name = "value"  # <p>The name of the DB proxy endpoint to create.</p>
}

```

---


### Db_instance_automated_backups

DBInstanceAutomatedBackups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_instance_automated_backups` | Vec<String> | <p>A list of <code>DBInstanceAutomatedBackup</code> instances.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous request.
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

# Access db_instance_automated_backups outputs
db_instance_automated_backups_id = db_instance_automated_backups.id
db_instance_automated_backups_db_instance_automated_backups = db_instance_automated_backups.db_instance_automated_backups
db_instance_automated_backups_marker = db_instance_automated_backups.marker
```

---


### Db_cluster_automated_backups

DBClusterAutomatedBackups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>The pagination token provided in the previous request. If this parameter is specified the response includes only 
            records beyond the marker, up to <code>MaxRecords</code>.</p> |
| `db_cluster_automated_backups` | Vec<String> | <p>A list of <code>DBClusterAutomatedBackup</code> backups.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_cluster_automated_backups outputs
db_cluster_automated_backups_id = db_cluster_automated_backups.id
db_cluster_automated_backups_marker = db_cluster_automated_backups.marker
db_cluster_automated_backups_db_cluster_automated_backups = db_cluster_automated_backups.db_cluster_automated_backups
```

---


### Db_recommendations

DBRecommendations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_recommendations` | Vec<String> | <p>A list of recommendations which is returned from <code>DescribeDBRecommendations</code> API request.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous <code>DBRecommendationsMessage</code> request.  This token can be used 
            later in a <code>DescribeDBRecomendations</code> request.
        </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_recommendations outputs
db_recommendations_id = db_recommendations.id
db_recommendations_db_recommendations = db_recommendations.db_recommendations
db_recommendations_marker = db_recommendations.marker
```

---


### Db_cluster_automated_backup

DBClusterAutomatedBackup resource

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


### Db_cluster_snapshot

DBClusterSnapshot resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `db_cluster_identifier` | String | ✅ | <p>The identifier of the DB cluster to create a snapshot for. This parameter isn't case-sensitive.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must match the identifier of an existing DBCluster.</p>
            </li>
         </ul>
         <p>Example: <code>my-cluster1</code>
         </p> |
| `tags` | Vec<String> |  | <p>The tags to be assigned to the DB cluster snapshot.</p> |
| `db_cluster_snapshot_identifier` | String | ✅ | <p>The identifier of the DB cluster snapshot. This parameter is stored as a lowercase string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Can't end with a hyphen or contain two consecutive hyphens.</p>
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
db_cluster_snapshot = provider.rds.Db_cluster_snapshot {
    db_cluster_identifier = "value"  # <p>The identifier of the DB cluster to create a snapshot for. This parameter isn't case-sensitive.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must match the identifier of an existing DBCluster.</p>
            </li>
         </ul>
         <p>Example: <code>my-cluster1</code>
         </p>
    db_cluster_snapshot_identifier = "value"  # <p>The identifier of the DB cluster snapshot. This parameter is stored as a lowercase string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Can't end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
         <p>Example: <code>my-cluster1-snapshot1</code>
         </p>
}

```

---


### Db_log_files

DBLogFiles resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>A pagination token that can be used in a later <code>DescribeDBLogFiles</code> request.</p> |
| `describe_db_log_files` | Vec<String> | <p>The DB log files returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_log_files outputs
db_log_files_id = db_log_files.id
db_log_files_marker = db_log_files.marker
db_log_files_describe_db_log_files = db_log_files.describe_db_log_files
```

---


### Db_proxy

DBProxy resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target_connection_network_type` | String |  | <p>The network type that the proxy uses to connect to the target database. The network type determines the IP version that the proxy uses for connections to the database.</p>
         <p>Valid values:</p>
         <ul>
            <li>
               <p>
                  <code>IPV4</code> - The proxy connects to the database using IPv4 only.</p>
            </li>
            <li>
               <p>
                  <code>IPV6</code> - The proxy connects to the database using IPv6 only.</p>
            </li>
         </ul>
         <p>Default: <code>IPV4</code>
         </p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>If you specify <code>IPV6</code>, the database must support dual-stack mode. RDS doesn't support IPv6-only databases.</p>
            </li>
            <li>
               <p>All targets registered with the proxy must be compatible with the specified network type.</p>
            </li>
         </ul> |
| `debug_logging` | bool |  | <p>Specifies whether the proxy logs detailed connection and query information. 
            When you enable <code>DebugLogging</code>, the proxy captures connection details 
            and connection pool behavior from your queries. Debug logging increases CloudWatch costs 
            and can impact proxy performance. Enable this option only when you need 
            to troubleshoot connection or performance issues.</p> |
| `default_auth_scheme` | String |  | <p>The default authentication scheme that the proxy uses for client connections to the proxy and connections from the proxy to the underlying database. 
            Valid values are <code>NONE</code> and <code>IAM_AUTH</code>. 
            When set to <code>IAM_AUTH</code>, the proxy uses end-to-end IAM authentication to connect to the database. 
            If you don't specify <code>DefaultAuthScheme</code> or specify this parameter 
            as <code>NONE</code>, you must specify the <code>Auth</code> option.</p> |
| `engine_family` | String | ✅ | <p>The kinds of databases that the proxy can connect to. 
          This value determines which database network protocol the proxy recognizes when it interprets
        network traffic to and from the database. For Aurora MySQL, RDS for MariaDB, and RDS for MySQL databases, specify <code>MYSQL</code>. 
        For Aurora PostgreSQL and RDS for PostgreSQL databases, specify <code>POSTGRESQL</code>. For RDS for Microsoft SQL Server, specify 
        <code>SQLSERVER</code>.</p> |
| `vpc_subnet_ids` | String | ✅ | <p>One or more VPC subnet IDs to associate with the new proxy.</p> |
| `db_proxy_name` | String | ✅ | <p>The identifier for the proxy. This name must be unique for all proxies owned by your Amazon Web Services account in the specified Amazon Web Services Region. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens; it can't end with a hyphen or contain two consecutive hyphens.</p> |
| `tags` | Vec<String> |  | <p>An optional set of key-value pairs to associate arbitrary data of your choosing with the proxy.</p> |
| `endpoint_network_type` | String |  | <p>The network type of the DB proxy endpoint. The network type determines the IP version that the proxy endpoint supports.</p>
         <p>Valid values:</p>
         <ul>
            <li>
               <p>
                  <code>IPV4</code> - The proxy endpoint supports IPv4 only.</p>
            </li>
            <li>
               <p>
                  <code>IPV6</code> - The proxy endpoint supports IPv6 only.</p>
            </li>
            <li>
               <p>
                  <code>DUAL</code> - The proxy endpoint supports both IPv4 and IPv6.</p>
            </li>
         </ul>
         <p>Default: <code>IPV4</code>
         </p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>If you specify <code>IPV6</code> or <code>DUAL</code>, the VPC and all subnets must have an IPv6 CIDR block.</p>
            </li>
            <li>
               <p>If you specify <code>IPV6</code> or <code>DUAL</code>, the VPC tenancy cannot be <code>dedicated</code>.</p>
            </li>
         </ul> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the IAM role that the proxy uses to access secrets in Amazon Web Services Secrets Manager.</p> |
| `idle_client_timeout` | i64 |  | <p>The number of seconds that a connection to the proxy can be inactive before the proxy disconnects it. You can set this
        value higher or lower than the connection timeout limit for the associated database.</p> |
| `vpc_security_group_ids` | String |  | <p>One or more VPC security group IDs to associate with the new proxy.</p> |
| `auth` | Vec<String> |  | <p>The authorization mechanism that the proxy uses.</p> |
| `require_tls` | bool |  | <p>Specifies whether Transport Layer Security (TLS) encryption is required for connections to the proxy.
        By enabling this setting, you can enforce encrypted TLS connections to the proxy.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_proxy
db_proxy = provider.rds.Db_proxy {
    engine_family = "value"  # <p>The kinds of databases that the proxy can connect to. 
          This value determines which database network protocol the proxy recognizes when it interprets
        network traffic to and from the database. For Aurora MySQL, RDS for MariaDB, and RDS for MySQL databases, specify <code>MYSQL</code>. 
        For Aurora PostgreSQL and RDS for PostgreSQL databases, specify <code>POSTGRESQL</code>. For RDS for Microsoft SQL Server, specify 
        <code>SQLSERVER</code>.</p>
    vpc_subnet_ids = "value"  # <p>One or more VPC subnet IDs to associate with the new proxy.</p>
    db_proxy_name = "value"  # <p>The identifier for the proxy. This name must be unique for all proxies owned by your Amazon Web Services account in the specified Amazon Web Services Region. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens; it can't end with a hyphen or contain two consecutive hyphens.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the IAM role that the proxy uses to access secrets in Amazon Web Services Secrets Manager.</p>
}

```

---


### Event_subscription

EventSubscription resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enabled` | bool |  | <p>Specifies whether to activate the subscription. If the event notification subscription isn't activated, the subscription is created but not active.</p> |
| `source_ids` | Vec<String> |  | <p>The list of identifiers of the event sources for which events are returned. If not specified, then all sources are included in the response. 
          An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens. It can't end with a hyphen or contain two consecutive hyphens.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>If <code>SourceIds</code> are supplied, <code>SourceType</code> must also be provided.</p>
            </li>
            <li>
               <p>If the source type is a DB instance, a <code>DBInstanceIdentifier</code> value must be supplied.</p>
            </li>
            <li>
               <p>If the source type is a DB cluster, a <code>DBClusterIdentifier</code> value must be supplied.</p>
            </li>
            <li>
               <p>If the source type is a DB parameter group, a <code>DBParameterGroupName</code> value must be supplied.</p>
            </li>
            <li>
               <p>If the source type is a DB security group, a <code>DBSecurityGroupName</code> value must be supplied.</p>
            </li>
            <li>
               <p>If the source type is a DB snapshot, a <code>DBSnapshotIdentifier</code> value must be supplied.</p>
            </li>
            <li>
               <p>If the source type is a DB cluster snapshot, a <code>DBClusterSnapshotIdentifier</code> value must be supplied.</p>
            </li>
            <li>
               <p>If the source type is an RDS Proxy, a <code>DBProxyName</code> value must be supplied.</p>
            </li>
         </ul> |
| `subscription_name` | String | ✅ | <p>The name of the subscription.</p>
         <p>Constraints: The name must be less than 255 characters.</p> |
| `source_type` | String |  | <p>The type of source that is generating the events. For example, if you want to be
            notified of events generated by a DB instance, you set this parameter to
                <code>db-instance</code>. For RDS Proxy events, specify <code>db-proxy</code>. If this value isn't specified, all events are
            returned.</p>
         <p>Valid Values:<code> db-instance | db-cluster | db-parameter-group | db-security-group | db-snapshot | db-cluster-snapshot | db-proxy | zero-etl | custom-engine-version | blue-green-deployment </code>
         </p> |
| `sns_topic_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the SNS topic created for event notification. SNS
            automatically creates the ARN when you create a topic and subscribe to it.</p>
         <note>
            <p>RDS doesn't support FIFO (first in, first out) topics. For more information, see
                    <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">Message
                    ordering and deduplication (FIFO topics)</a> in the <i>Amazon Simple
                    Notification Service Developer Guide</i>.</p>
         </note> |
| `event_categories` | Vec<String> |  | <p>A list of event categories for a particular source type (<code>SourceType</code>)
            that you want to subscribe to. You can see a list of the categories for a given source type in the "Amazon RDS event categories and event messages" section of the <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Events.Messages.html">
               <i>Amazon RDS User Guide</i>
            </a> or the
                <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/USER_Events.Messages.html">
               <i>Amazon Aurora User Guide</i>
            </a>.
                You can also see this list by using the <code>DescribeEventCategories</code> operation.</p> |
| `tags` | Vec<String> |  |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_subscription
event_subscription = provider.rds.Event_subscription {
    subscription_name = "value"  # <p>The name of the subscription.</p>
         <p>Constraints: The name must be less than 255 characters.</p>
    sns_topic_arn = "value"  # <p>The Amazon Resource Name (ARN) of the SNS topic created for event notification. SNS
            automatically creates the ARN when you create a topic and subscribe to it.</p>
         <note>
            <p>RDS doesn't support FIFO (first in, first out) topics. For more information, see
                    <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">Message
                    ordering and deduplication (FIFO topics)</a> in the <i>Amazon Simple
                    Notification Service Developer Guide</i>.</p>
         </note>
}

```

---


### Db_cluster

DBCluster resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `port` | i64 |  | <p>The port number on which the instances in the DB cluster accept connections.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>Valid Values: <code>1150-65535</code>
         </p>
         <p>Default:</p>
         <ul>
            <li>
               <p>RDS for MySQL and Aurora MySQL - <code>3306</code>
               </p>
            </li>
            <li>
               <p>RDS for PostgreSQL and Aurora PostgreSQL - <code>5432</code>
               </p>
            </li>
         </ul> |
| `pre_signed_url` | String |  | <p>When you are replicating a DB cluster from one Amazon Web Services GovCloud (US) Region to another,
            an URL that contains a Signature Version 4 signed request for the
                <code>CreateDBCluster</code> operation to be called in the source Amazon Web Services Region where
            the DB cluster is replicated from. Specify <code>PreSignedUrl</code> only when you are
            performing cross-Region replication from an encrypted DB cluster.</p>
         <p>The presigned URL must be a valid request for the <code>CreateDBCluster</code> API
            operation that can run in the source Amazon Web Services Region that contains the encrypted DB
            cluster to copy.</p>
         <p>The presigned URL request must contain the following parameter values:</p>
         <ul>
            <li>
               <p>
                  <code>KmsKeyId</code> - The KMS key identifier for the KMS key to use to
                    encrypt the copy of the DB cluster in the destination Amazon Web Services Region. This should
                    refer to the same KMS key for both the <code>CreateDBCluster</code> operation
                    that is called in the destination Amazon Web Services Region, and the operation contained in
                    the presigned URL.</p>
            </li>
            <li>
               <p>
                  <code>DestinationRegion</code> - The name of the Amazon Web Services Region that Aurora read replica will
                    be created in.</p>
            </li>
            <li>
               <p>
                  <code>ReplicationSourceIdentifier</code> - The DB cluster identifier for the encrypted DB cluster to be copied. 
                This identifier must be in the Amazon Resource Name (ARN) format for the source Amazon Web Services Region. For example, if you are copying an 
                encrypted DB cluster from the us-west-2 Amazon Web Services Region, then your <code>ReplicationSourceIdentifier</code> would look like
                Example: <code>arn:aws:rds:us-west-2:123456789012:cluster:aurora-cluster1</code>.</p>
            </li>
         </ul>
         <p>To learn how to generate a Signature Version 4 signed request, see 
            <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html">
                Authenticating Requests: Using Query Parameters (Amazon Web Services Signature Version 4)</a> and
            <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">
                Signature Version 4 Signing Process</a>.</p>
         <note>
            <p>If you are using an Amazon Web Services SDK tool or the CLI, you can specify
                    <code>SourceRegion</code> (or <code>--source-region</code> for the CLI)
                instead of specifying <code>PreSignedUrl</code> manually. Specifying
                    <code>SourceRegion</code> autogenerates a presigned URL that is a valid request
                for the operation that can run in the source Amazon Web Services Region.</p>
         </note>
         <p>Valid for Cluster Type: Aurora DB clusters only</p> |
| `auto_minor_version_upgrade` | bool |  | <p>Specifies whether minor engine upgrades are applied automatically to the DB cluster during the maintenance window. 
            By default, minor engine upgrades are applied automatically.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB cluster.</p>
         <p>For more information about automatic minor version upgrades, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_UpgradeDBInstance.Upgrading.html#USER_UpgradeDBInstance.Upgrading.AutoMinorVersionUpgrades">Automatically upgrading the minor engine version</a>.</p> |
| `enable_limitless_database` | bool |  | <p>Specifies whether to enable Aurora Limitless Database. You must enable Aurora Limitless Database to create a DB shard group.</p>
         <p>Valid for: Aurora DB clusters only</p>
         <note>
            <p>This setting is no longer used. Instead use the <code>ClusterScalabilityType</code> setting.</p>
         </note> |
| `db_cluster_identifier` | String | ✅ | <p>The identifier for this DB cluster. This parameter is stored as a lowercase string.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 (for Aurora DB clusters) or 1 to 52 (for Multi-AZ DB
                    clusters) letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Can't end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
         <p>Example: <code>my-cluster1</code>
         </p> |
| `serverless_v2_scaling_configuration` | String |  |  |
| `master_user_authentication_type` | String |  | <p>Specifies the authentication type for the master user. With IAM master user authentication, you can configure the master DB user with IAM database authentication when you create a DB cluster.</p>
         <p>You can specify one of the following values:</p>
         <ul>
            <li>
               <p>
                  <code>password</code> - Use standard database authentication with a password.</p>
            </li>
            <li>
               <p>
                  <code>iam-db-auth</code> - Use IAM database authentication for the master user.</p>
            </li>
         </ul>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>This option is only valid for RDS for PostgreSQL and Aurora PostgreSQL engines.</p> |
| `db_subnet_group_name` | String |  | <p>A DB subnet group to associate with this DB cluster.</p>
         <p>This setting is required to create a Multi-AZ DB cluster.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must match the name of an existing DB subnet group.</p>
            </li>
         </ul>
         <p>Example: <code>mydbsubnetgroup</code>
         </p> |
| `engine` | String | ✅ | <p>The database engine to use for this DB cluster.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>Valid Values:</p>
         <ul>
            <li>
               <p>
                  <code>aurora-mysql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>aurora-postgresql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>mysql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>postgres</code>
               </p>
            </li>
            <li>
               <p>
                  <code>neptune</code> - For information about using Amazon Neptune, see the
                        <a href="https://docs.aws.amazon.com/neptune/latest/userguide/intro.html">
                     <i>Amazon Neptune User Guide</i>
                  </a>.</p>
            </li>
         </ul> |
| `backtrack_window` | i64 |  | <p>The target backtrack window, in seconds. To disable backtracking, set this value to
            <code>0</code>.</p>
         <p>Valid for Cluster Type: Aurora MySQL DB clusters only</p>
         <p>Default: <code>0</code>
         </p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>If specified, this value must be set to a number from 0 to 259,200 (72 hours).</p>
            </li>
         </ul> |
| `master_user_password` | String |  | <p>The password for the master database user.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 8 to 41 characters.</p>
            </li>
            <li>
               <p>Can contain any printable ASCII character except "/", """, or "@".</p>
            </li>
            <li>
               <p>Can't be specified if <code>ManageMasterUserPassword</code> is turned on.</p>
            </li>
         </ul> |
| `storage_type` | String |  | <p>The storage type to associate with the DB cluster.</p>
         <p>For information on storage types for Aurora DB clusters, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/Aurora.Overview.StorageReliability.html#aurora-storage-type">Storage configurations for Amazon Aurora DB clusters</a>. For information on storage types for Multi-AZ DB
            clusters, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/create-multi-az-db-cluster.html#create-multi-az-db-cluster-settings">Settings for creating Multi-AZ DB clusters</a>.</p>
         <p>This setting is required to create a Multi-AZ DB cluster.</p>
         <p>When specified for a Multi-AZ DB cluster, a value for the <code>Iops</code> parameter is required.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>Valid Values:</p>
         <ul>
            <li>
               <p>Aurora DB clusters - <code>aurora | aurora-iopt1</code>
               </p>
            </li>
            <li>
               <p>Multi-AZ DB clusters - <code>io1 | io2 | gp3</code>
               </p>
            </li>
         </ul>
         <p>Default:</p>
         <ul>
            <li>
               <p>Aurora DB clusters - <code>aurora</code>
               </p>
            </li>
            <li>
               <p>Multi-AZ DB clusters - <code>io1</code>
               </p>
            </li>
         </ul>
         <note>
            <p>When you create an Aurora DB cluster with the storage type set to <code>aurora-iopt1</code>, the storage type is returned
                in the response. The storage type isn't returned when you set it to <code>aurora</code>.</p>
         </note> |
| `database_insights_mode` | String |  | <p>The mode of Database Insights to enable for the DB cluster.</p>
         <p>If you set this value to <code>advanced</code>, you must also set the <code>PerformanceInsightsEnabled</code>
            parameter to <code>true</code> and the <code>PerformanceInsightsRetentionPeriod</code> parameter to 465.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p> |
| `enable_performance_insights` | bool |  | <p>Specifies whether to turn on Performance Insights for the DB cluster.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_PerfInsights.html">
            Using Amazon Performance Insights</a> in the <i>Amazon RDS User Guide</i>.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p> |
| `db_cluster_instance_class` | String |  | <p>The compute and memory capacity of each DB instance in the Multi-AZ DB cluster, for example <code>db.m6gd.xlarge</code>.
            Not all DB instance classes are available in all Amazon Web Services Regions, or for all database engines.</p>
         <p>For the full list of DB instance classes and availability for your engine, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Concepts.DBInstanceClass.html">DB instance class</a> in the <i>Amazon RDS User Guide</i>.</p>
         <p>This setting is required to create a Multi-AZ DB cluster.</p>
         <p>Valid for Cluster Type: Multi-AZ DB clusters only</p> |
| `database_name` | String |  | <p>The name for your database of up to 64 alphanumeric characters. 
            A database named <code>postgres</code> is always created. If this parameter is specified, an additional database with this name is created.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p> |
| `global_cluster_identifier` | String |  | <p>The global cluster ID of an Aurora cluster that becomes the primary cluster
            in the new global database cluster.</p>
         <p>Valid for Cluster Type: Aurora DB clusters only</p> |
| `enable_global_write_forwarding` | bool |  | <p>Specifies whether to enable this DB cluster to forward write operations to the primary cluster of a global cluster
      (Aurora global database). By default, write operations are not allowed on Aurora DB clusters that
      are secondary clusters in an Aurora global database.</p>
         <p>You can set this value only on Aurora DB clusters that are members of an Aurora global database. With this parameter
      enabled, a secondary cluster can forward writes to the current primary cluster, and the resulting changes are replicated back to
      this cluster. For the primary DB cluster of an Aurora global database, this value is used immediately if the
        primary is demoted by a global cluster API operation, but it does nothing until then.</p>
         <p>Valid for Cluster Type: Aurora DB clusters only</p> |
| `domain` | String |  | <p>The Active Directory directory ID to create the DB cluster in.</p>
         <p>For Amazon Aurora DB clusters, Amazon RDS can use Kerberos authentication to authenticate users that connect to the DB cluster.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/kerberos-authentication.html">Kerberos authentication</a>
            in the <i>Amazon Aurora User Guide</i>.</p>
         <p>Valid for Cluster Type: Aurora DB clusters only</p> |
| `db_system_id` | String |  | <p>Reserved for future use.</p> |
| `option_group_name` | String |  | <p>The option group to associate the DB cluster with.</p>
         <p>DB clusters are associated with a default option group that can't be modified.</p> |
| `vpc_security_group_ids` | Vec<String> |  | <p>A list of EC2 VPC security groups to associate with this DB cluster.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p> |
| `engine_mode` | String |  | <p>The DB engine mode of the DB cluster, either <code>provisioned</code> or <code>serverless</code>.</p>
         <p>The <code>serverless</code> engine mode only applies for Aurora Serverless v1 DB clusters. Aurora Serverless v2 DB clusters use the 
        <code>provisioned</code> engine mode.</p>
         <p>For information about limitations and requirements for Serverless DB clusters, see the 
            following sections in the <i>Amazon Aurora User Guide</i>:</p>
         <ul>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-serverless.html#aurora-serverless.limitations">Limitations of Aurora
                        Serverless v1</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-serverless-v2.requirements.html">Requirements
                        for Aurora Serverless v2</a>
               </p>
            </li>
         </ul>
         <p>Valid for Cluster Type: Aurora DB clusters only</p> |
| `engine_version` | String |  | <p>The version number of the database engine to use.</p>
         <p>To list all of the available engine versions for Aurora MySQL version 2 (5.7-compatible) and version 3 (MySQL 8.0-compatible),
            use the following command:</p>
         <p>
            <code>aws rds describe-db-engine-versions --engine aurora-mysql --query "DBEngineVersions[].EngineVersion"</code>
         </p>
         <p>You can supply either <code>5.7</code> or <code>8.0</code> to use the default engine version for Aurora MySQL version 2 or
            version 3, respectively.</p>
         <p>To list all of the available engine versions for Aurora PostgreSQL, use the following command:</p>
         <p>
            <code>aws rds describe-db-engine-versions --engine aurora-postgresql --query "DBEngineVersions[].EngineVersion"</code>
         </p>
         <p>To list all of the available engine versions for RDS for MySQL, use the following command:</p>
         <p>
            <code>aws rds describe-db-engine-versions --engine mysql --query "DBEngineVersions[].EngineVersion"</code>
         </p>
         <p>To list all of the available engine versions for RDS for PostgreSQL, use the following command:</p>
         <p>
            <code>aws rds describe-db-engine-versions --engine postgres --query "DBEngineVersions[].EngineVersion"</code>
         </p>
         <p>For information about a specific engine, see the following topics:</p>
         <ul>
            <li>
               <p>Aurora MySQL - see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Updates.html">Database engine updates for Amazon Aurora MySQL</a> in the 
          <i>Amazon Aurora User Guide</i>.</p>
            </li>
            <li>
               <p>Aurora PostgreSQL - see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraPostgreSQL.Updates.20180305.html">Amazon Aurora PostgreSQL releases and engine versions</a> in the 
           <i>Amazon Aurora User Guide</i>.</p>
            </li>
            <li>
               <p>RDS for MySQL - see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_MySQL.html#MySQL.Concepts.VersionMgmt">Amazon RDS for MySQL</a> in the <i>Amazon RDS User Guide</i>.</p>
            </li>
            <li>
               <p>RDS for PostgreSQL - see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_PostgreSQL.html#PostgreSQL.Concepts">Amazon RDS for PostgreSQL</a> in the <i>Amazon RDS User Guide</i>.</p>
            </li>
         </ul>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p> |
| `master_username` | String |  | <p>The name of the master user for the DB cluster.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 1 to 16 letters or numbers.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Can't be a reserved word for the chosen database engine.</p>
            </li>
         </ul> |
| `iops` | i64 |  | <p>The amount of Provisioned IOPS (input/output operations per second) to be initially allocated 
            for each DB instance in the Multi-AZ DB cluster.</p>
         <p>For information about valid IOPS values, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_Storage.html#USER_PIOPS">Provisioned IOPS storage</a> in the <i>Amazon RDS
                User Guide</i>.</p>
         <p>This setting is required to create a Multi-AZ DB cluster.</p>
         <p>Valid for Cluster Type: Multi-AZ DB clusters only</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be a multiple between .5 and 50 of the storage amount for the DB cluster.</p>
            </li>
         </ul> |
| `availability_zones` | Vec<String> |  | <p>A list of Availability Zones (AZs) where you specifically want to create DB instances in the DB cluster.</p>
         <p>For the first three DB instances that you create, RDS distributes each DB instance to
            a different AZ that you specify. For additional DB instances that you create, RDS
            randomly distributes them to the AZs that you specified. For example, if you create a DB
            cluster with one writer instance and three reader instances, RDS might distribute the
            writer instance to AZ 1, the first reader instance to AZ 2, the second reader instance
            to AZ 3, and the third reader instance to either AZ 1, AZ 2, or AZ 3. </p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/Concepts.RegionsAndAvailabilityZones.html#Concepts.RegionsAndAvailabilityZones.AvailabilityZones">Availability Zones</a> and <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/Concepts.AuroraHighAvailability.html#Concepts.AuroraHighAvailability.Instances">High availability for Aurora DB instances</a> in the <i>Amazon Aurora
                User Guide</i>.</p>
         <p>Valid for Cluster Type: Aurora DB clusters only</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Can't specify more than three AZs.</p>
            </li>
         </ul> |
| `copy_tags_to_snapshot` | bool |  | <p>Specifies whether to copy all tags from the DB cluster to snapshots of the DB cluster. 
            The default is not to copy them.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p> |
| `monitoring_interval` | i64 |  | <p>The interval, in seconds, between points when Enhanced Monitoring metrics are collected for the DB cluster. To turn off 
            collecting Enhanced Monitoring metrics, specify <code>0</code>.</p>
         <p>If <code>MonitoringRoleArn</code> is specified, also set <code>MonitoringInterval</code>
            to a value other than <code>0</code>.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>Valid Values: <code>0 | 1 | 5 | 10 | 15 | 30 | 60</code>
         </p>
         <p>Default: <code>0</code>
         </p> |
| `replication_source_identifier` | String |  | <p>The Amazon Resource Name (ARN) of the source DB instance or DB cluster if this DB
            cluster is created as a read replica.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p> |
| `allocated_storage` | i64 |  | <p>The amount of storage in gibibytes (GiB) to allocate to each DB instance in the Multi-AZ DB cluster.</p>
         <p>Valid for Cluster Type: Multi-AZ DB clusters only</p>
         <p>This setting is required to create a Multi-AZ DB cluster.</p> |
| `enable_iam_database_authentication` | bool |  | <p>Specifies whether to enable mapping of Amazon Web Services Identity and Access
            Management (IAM) accounts to database accounts. By default, mapping isn't
            enabled.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/UsingWithRDS.IAMDBAuth.html"> IAM Database
                Authentication</a> in the <i>Amazon Aurora User Guide</i> or
                <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/UsingWithRDS.IAMDBAuth.html">IAM database
                authentication for MariaDB, MySQL, and PostgreSQL</a> in the <i>Amazon
                RDS User Guide</i>.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p> |
| `kms_key_id` | String |  | <p>The Amazon Web Services KMS key identifier for an encrypted DB cluster.</p>
         <p>The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.
                 To use a KMS key in a different Amazon Web Services account, specify the key ARN or alias ARN.</p>
         <p>When a KMS key isn't specified in <code>KmsKeyId</code>:</p>
         <ul>
            <li>
               <p>If <code>ReplicationSourceIdentifier</code> identifies an encrypted
                    source, then Amazon RDS uses the KMS key used to encrypt the
                    source. Otherwise, Amazon RDS uses your default KMS key.</p>
            </li>
            <li>
               <p>If the <code>StorageEncrypted</code> parameter is enabled and
                        <code>ReplicationSourceIdentifier</code> isn't specified, then Amazon RDS
                    uses your default KMS key.</p>
            </li>
         </ul>
         <p>There is a default KMS key for your Amazon Web Services account. Your Amazon Web Services account
            has a different default KMS key for each Amazon Web Services Region.</p>
         <p>If you create a read replica of an encrypted DB cluster in another Amazon Web Services Region, make
            sure to set <code>KmsKeyId</code> to a KMS key identifier that is valid in the destination Amazon Web Services
            Region. This KMS key is used to encrypt the read replica in that Amazon Web Services Region.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p> |
| `enable_http_endpoint` | bool |  | <p>Specifies whether to enable the HTTP endpoint for the DB cluster. By default, the HTTP endpoint 
            isn't enabled.</p>
         <p>When enabled, the HTTP endpoint provides a connectionless web service API (RDS Data API) for running
            SQL queries on the DB cluster. You can also query your database
            from inside the RDS console with the RDS query editor.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/data-api.html">Using RDS Data API</a> in the 
            <i>Amazon Aurora User Guide</i>.</p>
         <p>Valid for Cluster Type: Aurora DB clusters only</p> |
| `monitoring_role_arn` | String |  | <p>The Amazon Resource Name (ARN) for the IAM role that permits RDS to send Enhanced Monitoring metrics to Amazon CloudWatch Logs. 
            An example is <code>arn:aws:iam:123456789012:role/emaccess</code>. For information on creating a monitoring role,
            see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Monitoring.OS.html#USER_Monitoring.OS.Enabling">Setting 
                up and enabling Enhanced Monitoring</a> in the <i>Amazon RDS User Guide</i>.</p>
         <p>If <code>MonitoringInterval</code> is set to a value other than <code>0</code>, supply a <code>MonitoringRoleArn</code> value.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p> |
| `master_user_secret_kms_key_id` | String |  | <p>The Amazon Web Services KMS key identifier to encrypt a secret that is automatically generated and 
            managed in Amazon Web Services Secrets Manager.</p>
         <p>This setting is valid only if the master user password is managed by RDS in Amazon Web Services Secrets 
            Manager for the DB cluster.</p>
         <p>The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.
            To use a KMS key in a different Amazon Web Services account, specify the key ARN or alias ARN.</p>
         <p>If you don't specify <code>MasterUserSecretKmsKeyId</code>, then the <code>aws/secretsmanager</code> 
            KMS key is used to encrypt the secret. If the secret is in a different Amazon Web Services account, then you can't 
            use the <code>aws/secretsmanager</code> KMS key to encrypt the secret, and you must use a customer 
            managed KMS key.</p>
         <p>There is a default KMS key for your Amazon Web Services account. Your Amazon Web Services account
            has a different default KMS key for each Amazon Web Services Region.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p> |
| `preferred_backup_window` | String |  | <p>The daily time range during which automated backups are created
        if automated backups are enabled
        using the <code>BackupRetentionPeriod</code> parameter.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>The default is a 30-minute window selected at random from an
        8-hour block of time for each Amazon Web Services Region. 
        To view the time blocks available, see 
        <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/Aurora.Managing.Backups.html#Aurora.Managing.Backups.BackupWindow">
            Backup window</a> in the <i>Amazon Aurora User Guide</i>.</p>
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
| `enable_cloudwatch_logs_exports` | Vec<String> |  | <p>The list of log types that need to be enabled for exporting to CloudWatch Logs.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>The following values are valid for each DB engine:</p>
         <ul>
            <li>
               <p>Aurora MySQL - <code>audit | error | general | instance | slowquery | iam-db-auth-error</code>
               </p>
            </li>
            <li>
               <p>Aurora PostgreSQL - <code>instance | postgresql | iam-db-auth-error</code>
               </p>
            </li>
            <li>
               <p>RDS for MySQL - <code>error | general | slowquery | iam-db-auth-error</code>
               </p>
            </li>
            <li>
               <p>RDS for PostgreSQL - <code>postgresql | upgrade | iam-db-auth-error</code>
               </p>
            </li>
         </ul>
         <p>For more information about exporting CloudWatch Logs for Amazon RDS, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_LogAccess.html#USER_LogAccess.Procedural.UploadtoCloudWatch">Publishing Database Logs to Amazon CloudWatch Logs</a> in the <i>Amazon RDS User Guide</i>.</p>
         <p>For more information about exporting CloudWatch Logs for Amazon Aurora, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/USER_LogAccess.html#USER_LogAccess.Procedural.UploadtoCloudWatch">Publishing Database Logs to Amazon CloudWatch Logs</a> in the <i>Amazon Aurora User Guide</i>.</p> |
| `domain_iam_role_name` | String |  | <p>The name of the IAM role to use when making API calls to the Directory Service.</p>
         <p>Valid for Cluster Type: Aurora DB clusters only</p> |
| `ca_certificate_identifier` | String |  | <p>The CA certificate identifier to use for the DB cluster's server certificate.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/UsingWithRDS.SSL.html">Using SSL/TLS to encrypt a connection to a DB 
            instance</a> in the <i>Amazon RDS User Guide</i>.</p>
         <p>Valid for Cluster Type: Multi-AZ DB clusters</p> |
| `performance_insights_retention_period` | i64 |  | <p>The number of days to retain Performance Insights data.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>Valid Values:</p>
         <ul>
            <li>
               <p>
                  <code>7</code>
               </p>
            </li>
            <li>
               <p>
                  <i>month</i> * 31, where <i>month</i> is a number of months from 1-23. 
                Examples: <code>93</code> (3 months * 31), <code>341</code> (11 months * 31), <code>589</code> (19 months * 31)</p>
            </li>
            <li>
               <p>
                  <code>731</code>
               </p>
            </li>
         </ul>
         <p>Default: <code>7</code> days</p>
         <p>If you specify a retention period that isn't valid, such as <code>94</code>,  Amazon RDS issues an error.</p> |
| `engine_lifecycle_support` | String |  | <p>The life cycle type for this DB cluster.</p>
         <note>
            <p>By default, this value is set to <code>open-source-rds-extended-support</code>, which enrolls your DB cluster into Amazon RDS Extended Support. 
              At the end of standard support, you can avoid charges for Extended Support by setting the value to <code>open-source-rds-extended-support-disabled</code>. In this case, 
             creating the DB cluster will fail if the DB major version is past its end of standard support date.</p>
         </note>
         <p>You can use this setting to enroll your DB cluster into Amazon RDS Extended Support. With RDS Extended Support, 
        you can run the selected major engine version on your DB cluster past the end of standard support for that engine version. For more information, see the following sections:</p>
         <ul>
            <li>
               <p>Amazon Aurora - <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/extended-support.html">Amazon RDS Extended Support with Amazon Aurora</a> in the <i>Amazon Aurora User Guide</i>
               </p>
            </li>
            <li>
               <p>Amazon RDS - <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/extended-support.html">Amazon RDS Extended Support with Amazon RDS</a> in the <i>Amazon RDS User Guide</i>
               </p>
            </li>
         </ul>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>Valid Values: <code>open-source-rds-extended-support | open-source-rds-extended-support-disabled</code>
         </p>
         <p>Default: <code>open-source-rds-extended-support</code>
         </p> |
| `preferred_maintenance_window` | String |  | <p>The weekly time range during which system maintenance can occur.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>The default is a 30-minute window selected at random from an
            8-hour block of time for each Amazon Web Services Region, occurring on a random day of the
            week. To see the time blocks available, see 
            <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/USER_UpgradeDBInstance.Maintenance.html#AdjustingTheMaintenanceWindow.Aurora">
                Adjusting the Preferred DB Cluster Maintenance Window</a> in the <i>Amazon Aurora User Guide</i>.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be in the format <code>ddd:hh24:mi-ddd:hh24:mi</code>.</p>
            </li>
            <li>
               <p>Days must be one of <code>Mon | Tue | Wed | Thu | Fri | Sat | Sun</code>.</p>
            </li>
            <li>
               <p>Must be in Universal Coordinated Time (UTC).</p>
            </li>
            <li>
               <p>Must be at least 30 minutes.</p>
            </li>
         </ul> |
| `scaling_configuration` | String |  | <p>For DB clusters in <code>serverless</code> DB engine mode, the scaling properties of the DB cluster.</p>
         <p>Valid for Cluster Type: Aurora DB clusters only</p> |
| `manage_master_user_password` | bool |  | <p>Specifies whether to manage the master user password with Amazon Web Services Secrets Manager.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/rds-secrets-manager.html">Password management with Amazon Web Services Secrets Manager</a> 
            in the <i>Amazon RDS User Guide</i> and <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/rds-secrets-manager.html">Password management with Amazon Web Services Secrets Manager</a> 
            in the <i>Amazon Aurora User Guide.</i>
         </p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Can't manage the master user password with Amazon Web Services Secrets Manager if <code>MasterUserPassword</code> 
                    is specified.</p>
            </li>
         </ul> |
| `enable_local_write_forwarding` | bool |  | <p>Specifies whether read replicas can forward write operations to the writer DB instance in the DB cluster. By
            default, write operations aren't allowed on reader DB instances.</p>
         <p>Valid for: Aurora DB clusters only</p> |
| `rds_custom_cluster_configuration` | String |  | <p>Reserved for future use.</p> |
| `tags` | Vec<String> |  | <p>Tags to assign to the DB cluster.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p> |
| `publicly_accessible` | bool |  | <p>Specifies whether the DB cluster is publicly accessible.</p>
         <p>When the DB cluster is publicly accessible and you connect from outside of the DB cluster's virtual private cloud (VPC), 
              its Domain Name System (DNS) endpoint resolves to the public IP address. When you connect from within the same VPC as the DB cluster, 
              the endpoint resolves to the private IP address. Access to the DB cluster is ultimately controlled by the security group it uses. That public
              access isn't permitted if the security group assigned to the DB cluster doesn't permit it.</p>
         <p>When the DB cluster isn't publicly accessible, it is an internal DB cluster with a DNS name that resolves to a private IP address.</p>
         <p>Valid for Cluster Type: Multi-AZ DB clusters only</p>
         <p>Default: The default behavior varies depending on whether <code>DBSubnetGroupName</code> is specified.</p>
         <p>If <code>DBSubnetGroupName</code> isn't specified, and <code>PubliclyAccessible</code> isn't specified, the following applies:</p>
         <ul>
            <li>
               <p>If the default VPC in the target Region doesn’t have an internet gateway attached to it, the DB cluster is private.</p>
            </li>
            <li>
               <p>If the default VPC in the target Region has an internet gateway attached to it, the DB cluster is public.</p>
            </li>
         </ul>
         <p>If <code>DBSubnetGroupName</code> is specified, and <code>PubliclyAccessible</code> isn't specified, the following applies:</p>
         <ul>
            <li>
               <p>If the subnets are part of a VPC that doesn’t have an internet gateway attached to it, the DB cluster is private.</p>
            </li>
            <li>
               <p>If the subnets are part of a VPC that has an internet gateway attached to it, the DB cluster is public.</p>
            </li>
         </ul> |
| `deletion_protection` | bool |  | <p>Specifies whether the DB cluster has deletion protection enabled. 
            The database can't be deleted when deletion protection is enabled. By default, 
            deletion protection isn't enabled.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p> |
| `performance_insights_kms_key_id` | String |  | <p>The Amazon Web Services KMS key identifier for encryption of Performance Insights data.</p>
         <p>The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.</p>
         <p>If you don't specify a value for <code>PerformanceInsightsKMSKeyId</code>, then Amazon RDS 
            uses your default KMS key. There is a default KMS key for your Amazon Web Services account. 
            Your Amazon Web Services account has a different default KMS key for each Amazon Web Services Region.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p> |
| `network_type` | String |  | <p>The network type of the DB cluster.</p>
         <p>The network type is determined by the <code>DBSubnetGroup</code> specified for the DB cluster. 
            A <code>DBSubnetGroup</code> can support only the IPv4 protocol or the IPv4 and the IPv6 
            protocols (<code>DUAL</code>).</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/USER_VPC.WorkingWithRDSInstanceinaVPC.html">
            Working with a DB instance in a VPC</a> in the 
            <i>Amazon Aurora User Guide.</i>
         </p>
         <p>Valid for Cluster Type: Aurora DB clusters only</p>
         <p>Valid Values: <code>IPV4 | DUAL</code>
         </p> |
| `storage_encrypted` | bool |  | <p>Specifies whether the DB cluster is encrypted.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p> |
| `backup_retention_period` | i64 |  | <p>The number of days for which automated backups are retained.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>Default: <code>1</code>
         </p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be a value from 1 to 35.</p>
            </li>
         </ul> |
| `db_cluster_parameter_group_name` | String |  | <p>The name of the DB cluster parameter group to associate
            with this DB cluster. If you don't specify a value, then 
          the default DB cluster parameter group for the specified DB engine and version is used.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>If supplied, must match the name of an existing DB cluster parameter group.</p>
            </li>
         </ul> |
| `character_set_name` | String |  | <p>The name of the character set (<code>CharacterSet</code>) to associate the DB cluster with.</p>
         <p>Valid for Cluster Type: Aurora DB clusters only</p> |
| `cluster_scalability_type` | String |  | <p>Specifies the scalability mode of the Aurora DB cluster. When set to <code>limitless</code>, the cluster operates as an Aurora Limitless Database.
            When set to <code>standard</code> (the default), the cluster uses normal DB instance creation.</p>
         <p>Valid for: Aurora DB clusters only</p>
         <note>
            <p>You can't modify this setting after you create the DB cluster.</p>
         </note> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_cluster
db_cluster = provider.rds.Db_cluster {
    db_cluster_identifier = "value"  # <p>The identifier for this DB cluster. This parameter is stored as a lowercase string.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 (for Aurora DB clusters) or 1 to 52 (for Multi-AZ DB
                    clusters) letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Can't end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
         <p>Example: <code>my-cluster1</code>
         </p>
    engine = "value"  # <p>The database engine to use for this DB cluster.</p>
         <p>Valid for Cluster Type: Aurora DB clusters and Multi-AZ DB clusters</p>
         <p>Valid Values:</p>
         <ul>
            <li>
               <p>
                  <code>aurora-mysql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>aurora-postgresql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>mysql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>postgres</code>
               </p>
            </li>
            <li>
               <p>
                  <code>neptune</code> - For information about using Amazon Neptune, see the
                        <a href="https://docs.aws.amazon.com/neptune/latest/userguide/intro.html">
                     <i>Amazon Neptune User Guide</i>
                  </a>.</p>
            </li>
         </ul>
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
| `marker` | String | <p>An optional pagination token provided by a previous
            <code>DescribePendingMaintenanceActions</code> request.
            If this parameter is specified, the response includes
            only records beyond the marker,
            up to a number of records specified by <code>MaxRecords</code>.</p> |
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


### Global_cluster

GlobalCluster resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `storage_encrypted` | bool |  | <p>Specifies whether to enable storage encryption for the new global database cluster.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Can't be specified if <code>SourceDBClusterIdentifier</code> is specified. In this case, Amazon Aurora uses the setting from the source DB cluster.</p>
            </li>
         </ul> |
| `engine_version` | String |  | <p>The engine version to use for this global database cluster.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Can't be specified if <code>SourceDBClusterIdentifier</code> is specified. In this case, Amazon Aurora uses the engine version of the source DB cluster.</p>
            </li>
         </ul> |
| `engine_lifecycle_support` | String |  | <p>The life cycle type for this global database cluster.</p>
         <note>
            <p>By default, this value is set to <code>open-source-rds-extended-support</code>, which enrolls your global cluster into Amazon RDS Extended Support. 
              At the end of standard support, you can avoid charges for Extended Support by setting the value to <code>open-source-rds-extended-support-disabled</code>. In this case, 
              creating the global cluster will fail if the DB major version is past its end of standard support date.</p>
         </note>
         <p>This setting only applies to Aurora PostgreSQL-based global databases.</p>
         <p>You can use this setting to enroll your global cluster into Amazon RDS Extended Support. With RDS Extended Support, 
        you can run the selected major engine version on your global cluster past the end of standard support for that engine version. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/extended-support.html">Amazon RDS Extended Support with Amazon Aurora</a> in the <i>Amazon Aurora User Guide</i>.</p>
         <p>Valid Values: <code>open-source-rds-extended-support | open-source-rds-extended-support-disabled</code>
         </p>
         <p>Default: <code>open-source-rds-extended-support</code>
         </p> |
| `tags` | Vec<String> |  | <p>Tags to assign to the global cluster.</p> |
| `global_cluster_identifier` | String | ✅ | <p>The cluster identifier for this global database cluster. This parameter is stored as a lowercase string.</p> |
| `source_db_cluster_identifier` | String |  | <p>The Amazon Resource Name (ARN) to use as the primary cluster of the global database.</p>
         <p>If you provide a value for this parameter, don't specify values for the following settings because Amazon Aurora uses the values from the specified source DB cluster:</p>
         <ul>
            <li>
               <p>
                  <code>DatabaseName</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Engine</code>
               </p>
            </li>
            <li>
               <p>
                  <code>EngineVersion</code>
               </p>
            </li>
            <li>
               <p>
                  <code>StorageEncrypted</code>
               </p>
            </li>
         </ul> |
| `engine` | String |  | <p>The database engine to use for this global database cluster.</p>
         <p>Valid Values: <code>aurora-mysql | aurora-postgresql</code>
         </p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Can't be specified if <code>SourceDBClusterIdentifier</code> is specified. In this case, Amazon Aurora uses the engine of the source DB cluster.</p>
            </li>
         </ul> |
| `deletion_protection` | bool |  | <p>Specifies whether to enable deletion protection for the new global database cluster.
        The global database can't be deleted when deletion protection is enabled.</p> |
| `database_name` | String |  | <p>The name for your database of up to 64 alphanumeric characters. If you don't specify
            a name, Amazon Aurora doesn't create a database in the global database cluster.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Can't be specified if <code>SourceDBClusterIdentifier</code> is specified. In this case, Amazon Aurora uses the database name from the source DB cluster.</p>
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

# Create global_cluster
global_cluster = provider.rds.Global_cluster {
    global_cluster_identifier = "value"  # <p>The cluster identifier for this global database cluster. This parameter is stored as a lowercase string.</p>
}

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
| `parameters` | Vec<String> | <p>Provides a list of parameters for the DB cluster parameter group.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous
            <code>DescribeDBClusterParameters</code> request.
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

# Access db_cluster_parameters outputs
db_cluster_parameters_id = db_cluster_parameters.id
db_cluster_parameters_parameters = db_cluster_parameters.parameters
db_cluster_parameters_marker = db_cluster_parameters.marker
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
| `parameters` | Vec<String> | <p>A list of <code>Parameter</code> values.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous request.
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

# Access db_parameters outputs
db_parameters_id = db_parameters.id
db_parameters_parameters = db_parameters.parameters
db_parameters_marker = db_parameters.marker
```

---


### Option_group

OptionGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>Tags to assign to the option group.</p> |
| `engine_name` | String | ✅ | <p>The name of the engine to associate this option group with.</p>
         <p>Valid Values:</p>
         <ul>
            <li>
               <p>
                  <code>db2-ae</code>
               </p>
            </li>
            <li>
               <p>
                  <code>db2-se</code>
               </p>
            </li>
            <li>
               <p>
                  <code>mariadb</code>
               </p>
            </li>
            <li>
               <p>
                  <code>mysql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-ee</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-ee-cdb</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-se2</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-se2-cdb</code>
               </p>
            </li>
            <li>
               <p>
                  <code>postgres</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-ee</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-se</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-ex</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-web</code>
               </p>
            </li>
         </ul> |
| `major_engine_version` | String | ✅ | <p>Specifies the major version of the engine that this option group should be associated with.</p> |
| `option_group_name` | String | ✅ | <p>Specifies the name of the option group to be created.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 1 to 255 letters, numbers, or hyphens</p>
            </li>
            <li>
               <p>First character must be a letter</p>
            </li>
            <li>
               <p>Can't end with a hyphen or contain two consecutive hyphens</p>
            </li>
         </ul>
         <p>Example: <code>myoptiongroup</code>
         </p> |
| `option_group_description` | String | ✅ | <p>The description of the option group.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create option_group
option_group = provider.rds.Option_group {
    engine_name = "value"  # <p>The name of the engine to associate this option group with.</p>
         <p>Valid Values:</p>
         <ul>
            <li>
               <p>
                  <code>db2-ae</code>
               </p>
            </li>
            <li>
               <p>
                  <code>db2-se</code>
               </p>
            </li>
            <li>
               <p>
                  <code>mariadb</code>
               </p>
            </li>
            <li>
               <p>
                  <code>mysql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-ee</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-ee-cdb</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-se2</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-se2-cdb</code>
               </p>
            </li>
            <li>
               <p>
                  <code>postgres</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-ee</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-se</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-ex</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-web</code>
               </p>
            </li>
         </ul>
    major_engine_version = "value"  # <p>Specifies the major version of the engine that this option group should be associated with.</p>
    option_group_name = "value"  # <p>Specifies the name of the option group to be created.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 1 to 255 letters, numbers, or hyphens</p>
            </li>
            <li>
               <p>First character must be a letter</p>
            </li>
            <li>
               <p>Can't end with a hyphen or contain two consecutive hyphens</p>
            </li>
         </ul>
         <p>Example: <code>myoptiongroup</code>
         </p>
    option_group_description = "value"  # <p>The description of the option group.</p>
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
| `marker` | String | <p>An optional pagination token provided by a previous
            DescribeOrderableDBInstanceOptions request.
            If this parameter is specified, the response includes
            only records beyond the marker,
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


### Option_group_options

OptionGroupOptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `option_group_options` | Vec<String> |  |
| `marker` | String | <p>An optional pagination token provided by a previous request.
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

# Access option_group_options outputs
option_group_options_id = option_group_options.id
option_group_options_option_group_options = option_group_options.option_group_options
option_group_options_marker = option_group_options.marker
```

---


### Db_snapshot_tenant_databases

DBSnapshotTenantDatabases resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_snapshot_tenant_databases` | Vec<String> | <p>A list of DB snapshot tenant databases.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is
            specified, the response includes only records beyond the marker, up to the value
            specified by <code>MaxRecords</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_snapshot_tenant_databases outputs
db_snapshot_tenant_databases_id = db_snapshot_tenant_databases.id
db_snapshot_tenant_databases_db_snapshot_tenant_databases = db_snapshot_tenant_databases.db_snapshot_tenant_databases
db_snapshot_tenant_databases_marker = db_snapshot_tenant_databases.marker
```

---


### Db_major_engine_versions

DBMajorEngineVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_major_engine_versions` | Vec<String> | <p>A list of <code>DBMajorEngineVersion</code> elements.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is
            specified, the response includes only records beyond the marker, up to the value
            specified by <code>MaxRecords</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_major_engine_versions outputs
db_major_engine_versions_id = db_major_engine_versions.id
db_major_engine_versions_db_major_engine_versions = db_major_engine_versions.db_major_engine_versions
db_major_engine_versions_marker = db_major_engine_versions.marker
```

---


### Db_instance

DBInstance resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tde_credential_password` | String |  | <p>The password for the given ARN from the key store in order to access the device.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p> |
| `preferred_backup_window` | String |  | <p>The daily time range during which automated backups are created
        if automated backups are enabled,
        using the <code>BackupRetentionPeriod</code> parameter.
          The default is a 30-minute window selected at random from an
          8-hour block of time for each Amazon Web Services Region. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_WorkingWithAutomatedBackups.html#USER_WorkingWithAutomatedBackups.BackupWindow">Backup window</a> in the <i>Amazon RDS User Guide</i>.</p>
         <p>This setting doesn't apply to Amazon Aurora DB instances. The daily time range for creating automated backups is managed by
          the DB cluster.</p>
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
| `db_cluster_identifier` | String |  | <p>The identifier of the DB cluster that this DB instance will belong to.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p> |
| `preferred_maintenance_window` | String |  | <p>The time range each week during which system maintenance can occur. 
          For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_UpgradeDBInstance.Maintenance.html#Concepts.DBMaintenance">Amazon RDS Maintenance Window</a> 
          in the <i>Amazon RDS User Guide.</i>
         </p>
         <p>The default is a 30-minute window selected at random from an
            8-hour block of time for each Amazon Web Services Region, occurring on a random day of the
            week.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be in the format <code>ddd:hh24:mi-ddd:hh24:mi</code>.</p>
            </li>
            <li>
               <p>The day values must be <code>mon | tue | wed | thu | fri | sat | sun</code>. </p>
            </li>
            <li>
               <p>Must be in Universal Coordinated Time (UTC).</p>
            </li>
            <li>
               <p>Must not conflict with the preferred backup window.</p>
            </li>
            <li>
               <p>Must be at least 30 minutes.</p>
            </li>
         </ul> |
| `tde_credential_arn` | String |  | <p>The ARN from the key store with which to associate the instance for TDE encryption.</p>
         <p>This setting doesn't apply to Amazon Aurora or RDS Custom DB instances.</p> |
| `backup_retention_period` | i64 |  | <p>The number of days for which automated backups are retained. Setting this parameter to a positive number enables 
          backups. Setting this parameter to <code>0</code> disables automated backups.</p>
         <p>This setting doesn't apply to Amazon Aurora DB instances. The retention period for automated backups is managed by the DB cluster.</p>
         <p>Default: <code>1</code>
         </p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be a value from 0 to 35.</p>
            </li>
            <li>
               <p>Can't be set to 0 if the DB instance is a source to read replicas.</p>
            </li>
            <li>
               <p>Can't be set to 0 for an RDS Custom for Oracle DB instance.</p>
            </li>
         </ul> |
| `enable_iam_database_authentication` | bool |  | <p>Specifies whether to enable mapping of Amazon Web Services Identity and Access Management
            (IAM) accounts to database accounts. By default, mapping isn't enabled.</p>
         <p>For more information, see 
            <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/UsingWithRDS.IAMDBAuth.html">
                IAM Database Authentication for MySQL and PostgreSQL</a> in the <i>Amazon RDS User Guide</i>.</p>
         <p>This setting doesn't apply to the following DB instances:</p>
         <ul>
            <li>
               <p>Amazon Aurora (Mapping Amazon Web Services IAM accounts to database accounts is managed by the DB cluster.)</p>
            </li>
            <li>
               <p>RDS Custom</p>
            </li>
         </ul> |
| `publicly_accessible` | bool |  | <p>Specifies whether the DB instance is publicly accessible.</p>
         <p>When the DB instance is publicly accessible and you connect from outside of the DB instance's virtual private cloud (VPC), 
              its Domain Name System (DNS) endpoint resolves to the public IP address. When you connect from within the same VPC as the DB instance, 
              the endpoint resolves to the private IP address. Access to the DB instance is ultimately controlled by the security group it uses. 
              That public access is not permitted if the security group assigned to the DB instance doesn't permit it.</p>
         <p>When the DB instance isn't publicly accessible, it is an internal DB instance with a DNS name that resolves to a private IP address.</p>
         <p>Default: The default behavior varies depending on whether <code>DBSubnetGroupName</code> is specified.</p>
         <p>If <code>DBSubnetGroupName</code> isn't specified, and <code>PubliclyAccessible</code> isn't specified, the following applies:</p>
         <ul>
            <li>
               <p>If the default VPC in the target Region doesn’t have an internet gateway attached to it, the DB instance is private.</p>
            </li>
            <li>
               <p>If the default VPC in the target Region has an internet gateway attached to it, the DB instance is public.</p>
            </li>
         </ul>
         <p>If <code>DBSubnetGroupName</code> is specified, and <code>PubliclyAccessible</code> isn't specified, the following applies:</p>
         <ul>
            <li>
               <p>If the subnets are part of a VPC that doesn’t have an internet gateway attached to it, the DB instance is private.</p>
            </li>
            <li>
               <p>If the subnets are part of a VPC that has an internet gateway attached to it, the DB instance is public.</p>
            </li>
         </ul> |
| `auto_minor_version_upgrade` | bool |  | <p>Specifies whether minor engine upgrades are applied automatically to the DB instance during the maintenance window. 
          By default, minor engine upgrades are applied automatically.</p>
         <p>If you create an RDS Custom DB instance, you must set <code>AutoMinorVersionUpgrade</code> to 
          <code>false</code>.</p>
         <p>For more information about automatic minor version upgrades, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_UpgradeDBInstance.Upgrading.html#USER_UpgradeDBInstance.Upgrading.AutoMinorVersionUpgrades">Automatically upgrading the minor engine version</a>.</p> |
| `domain` | String |  | <p>The Active Directory directory ID to create the DB instance in. Currently, you can create only Db2, MySQL, Microsoft SQL 
          Server, Oracle, and PostgreSQL DB instances in an Active Directory Domain.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/kerberos-authentication.html">
           Kerberos Authentication</a> in the <i>Amazon RDS User Guide</i>.</p>
         <p>This setting doesn't apply to the following DB instances:</p>
         <ul>
            <li>
               <p>Amazon Aurora (The domain is managed by the DB cluster.)</p>
            </li>
            <li>
               <p>RDS Custom</p>
            </li>
         </ul> |
| `domain_ou` | String |  | <p>The Active Directory organizational unit for your DB instance to join.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be in the distinguished name format.</p>
            </li>
            <li>
               <p>Can't be longer than 64 characters.</p>
            </li>
         </ul>
         <p>Example: <code>OU=mymanagedADtestOU,DC=mymanagedADtest,DC=mymanagedAD,DC=mydomain</code>
         </p> |
| `performance_insights_retention_period` | i64 |  | <p>The number of days to retain Performance Insights data.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p>
         <p>Valid Values:</p>
         <ul>
            <li>
               <p>
                  <code>7</code>
               </p>
            </li>
            <li>
               <p>
                  <i>month</i> * 31, where <i>month</i> is a number of months from 1-23. 
                Examples: <code>93</code> (3 months * 31), <code>341</code> (11 months * 31), <code>589</code> (19 months * 31)</p>
            </li>
            <li>
               <p>
                  <code>731</code>
               </p>
            </li>
         </ul>
         <p>Default: <code>7</code> days</p>
         <p>If you specify a retention period that isn't valid, such as <code>94</code>,  Amazon RDS returns an error.</p> |
| `custom_iam_instance_profile` | String |  | <p>The instance profile associated with the underlying Amazon EC2 instance of an 
            RDS Custom DB instance.</p>
         <p>This setting is required for RDS Custom.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>The profile must exist in your account.</p>
            </li>
            <li>
               <p>The profile must have an IAM role that Amazon EC2 has permissions to assume.</p>
            </li>
            <li>
               <p>The instance profile name and the associated IAM role name must start with the prefix <code>AWSRDSCustom</code>.</p>
            </li>
         </ul>
         <p>For the list of permissions required for the IAM role, see 
            <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/custom-setup-orcl.html#custom-setup-orcl.iam-vpc">
                Configure IAM and your VPC</a> in the <i>Amazon RDS User Guide</i>.</p> |
| `enable_performance_insights` | bool |  | <p>Specifies whether to enable Performance Insights for the DB instance. For more information, see 
            <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_PerfInsights.html">Using Amazon Performance Insights</a> in the <i>Amazon RDS User Guide</i>.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p> |
| `enable_cloudwatch_logs_exports` | Vec<String> |  | <p>The list of log types to enable for exporting to CloudWatch Logs. For more information, see 
            <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_LogAccess.html#USER_LogAccess.Procedural.UploadtoCloudWatch">
            Publishing Database Logs to Amazon CloudWatch Logs</a> in the <i>Amazon RDS User Guide</i>.</p>
         <p>This setting doesn't apply to the following DB instances:</p>
         <ul>
            <li>
               <p>Amazon Aurora (CloudWatch Logs exports are managed by the DB cluster.)</p>
            </li>
            <li>
               <p>RDS Custom</p>
            </li>
         </ul>
         <p>The following values are valid for each DB engine:</p>
         <ul>
            <li>
               <p>RDS for Db2 - <code>diag.log | notify.log | iam-db-auth-error</code>
               </p>
            </li>
            <li>
               <p>RDS for MariaDB - <code>audit | error | general | slowquery | iam-db-auth-error</code>
               </p>
            </li>
            <li>
               <p>RDS for Microsoft SQL Server - <code>agent | error</code>
               </p>
            </li>
            <li>
               <p>RDS for MySQL - <code>audit | error | general | slowquery | iam-db-auth-error</code>
               </p>
            </li>
            <li>
               <p>RDS for Oracle - <code>alert | audit | listener | trace | oemagent</code>
               </p>
            </li>
            <li>
               <p>RDS for PostgreSQL - <code>postgresql | upgrade | iam-db-auth-error</code>
               </p>
            </li>
         </ul> |
| `dedicated_log_volume` | bool |  | <p>Indicates whether the DB instance has a dedicated log volume (DLV) enabled.</p> |
| `storage_encrypted` | bool |  | <p>Specifes whether the DB instance is encrypted. By default, it isn't encrypted.</p>
         <p>For RDS Custom DB instances, either enable this setting or leave it unset. Otherwise, Amazon RDS reports an error.</p>
         <p>This setting doesn't apply to Amazon Aurora DB instances. The encryption for DB instances is managed by the DB cluster.</p> |
| `master_user_secret_kms_key_id` | String |  | <p>The Amazon Web Services KMS key identifier to encrypt a secret that is automatically generated and 
            managed in Amazon Web Services Secrets Manager.</p>
         <p>This setting is valid only if the master user password is managed by RDS in Amazon Web Services Secrets 
            Manager for the DB instance.</p>
         <p>The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.
            To use a KMS key in a different Amazon Web Services account, specify the key ARN or alias ARN.</p>
         <p>If you don't specify <code>MasterUserSecretKmsKeyId</code>, then the <code>aws/secretsmanager</code> 
            KMS key is used to encrypt the secret. If the secret is in a different Amazon Web Services account, then you can't 
            use the <code>aws/secretsmanager</code> KMS key to encrypt the secret, and you must use a customer 
            managed KMS key.</p>
         <p>There is a default KMS key for your Amazon Web Services account. Your Amazon Web Services account
            has a different default KMS key for each Amazon Web Services Region.</p> |
| `db_instance_identifier` | String | ✅ | <p>The identifier for this DB instance. This parameter is stored as a lowercase string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Can't end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
         <p>Example: <code>mydbinstance</code>
         </p> |
| `monitoring_interval` | i64 |  | <p>The interval, in seconds, between points when Enhanced Monitoring metrics are collected for 
          the DB instance. To disable collection of Enhanced Monitoring metrics, specify <code>0</code>.</p>
         <p>If <code>MonitoringRoleArn</code> is specified, then you must set <code>MonitoringInterval</code>
      to a value other than <code>0</code>.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p>
         <p>Valid Values: <code>0 | 1 | 5 | 10 | 15 | 30 | 60</code>
         </p>
         <p>Default: <code>0</code>
         </p> |
| `domain_fqdn` | String |  | <p>The fully qualified domain name (FQDN) of an Active Directory domain.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Can't be longer than 64 characters.</p>
            </li>
         </ul>
         <p>Example: <code>mymanagedADtest.mymanagedAD.mydomain</code>
         </p> |
| `availability_zone` | String |  | <p>The Availability Zone (AZ) where the database will be created. For information on
        Amazon Web Services Regions and Availability Zones, see 
        <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Concepts.RegionsAndAvailabilityZones.html">Regions
        and Availability Zones</a>.</p>
         <p>For Amazon Aurora, each Aurora DB cluster hosts copies of its storage in three separate Availability Zones. Specify one of these 
            Availability Zones. Aurora automatically chooses an appropriate Availability Zone if you don't specify one.</p>
         <p>Default: A random, system-chosen Availability Zone in the endpoint's Amazon Web Services Region.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>The <code>AvailabilityZone</code> parameter can't be specified if the DB instance is a Multi-AZ deployment.</p>
            </li>
            <li>
               <p>The specified Availability Zone must be in the same Amazon Web Services Region as the current endpoint.</p>
            </li>
         </ul>
         <p>Example: <code>us-east-1d</code>
         </p> |
| `master_user_password` | String |  | <p>The password for the master user.</p>
         <p>This setting doesn't apply to Amazon Aurora DB instances. The password for the master user is managed by the DB
            cluster.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Can't be specified if <code>ManageMasterUserPassword</code> is turned on.</p>
            </li>
            <li>
               <p>Can include any printable ASCII character except "/", """, or "@". For RDS for Oracle, can't include the "&" (ampersand) or  the "'" (single quotes) character.</p>
            </li>
         </ul>
         <p>Length Constraints:</p>
         <ul>
            <li>
               <p>RDS for Db2 - Must contain from 8 to 255 characters.</p>
            </li>
            <li>
               <p>RDS for MariaDB - Must contain from 8 to 41 characters.</p>
            </li>
            <li>
               <p>RDS for Microsoft SQL Server - Must contain from 8 to 128 characters.</p>
            </li>
            <li>
               <p>RDS for MySQL - Must contain from 8 to 41 characters.</p>
            </li>
            <li>
               <p>RDS for Oracle - Must contain from 8 to 30 characters.</p>
            </li>
            <li>
               <p>RDS for PostgreSQL - Must contain from 8 to 128 characters.</p>
            </li>
         </ul> |
| `nchar_character_set_name` | String |  | <p>The name of the NCHAR character set for the Oracle DB instance.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p> |
| `port` | i64 |  | <p>The port number on which the database accepts connections.</p>
         <p>This setting doesn't apply to Aurora DB instances. The port number is managed by the cluster.</p>
         <p>Valid Values: <code>1150-65535</code>
         </p>
         <p>Default:</p>
         <ul>
            <li>
               <p>RDS for Db2 - <code>50000</code>
               </p>
            </li>
            <li>
               <p>RDS for MariaDB - <code>3306</code>
               </p>
            </li>
            <li>
               <p>RDS for Microsoft SQL Server - <code>1433</code>
               </p>
            </li>
            <li>
               <p>RDS for MySQL - <code>3306</code>
               </p>
            </li>
            <li>
               <p>RDS for Oracle - <code>1521</code>
               </p>
            </li>
            <li>
               <p>RDS for PostgreSQL - <code>5432</code>
               </p>
            </li>
         </ul>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>For RDS for Microsoft SQL Server, the value can't be <code>1234</code>, <code>1434</code>,
                <code>3260</code>, <code>3343</code>, <code>3389</code>, <code>47001</code>, or
                <code>49152-49156</code>.</p>
            </li>
         </ul> |
| `monitoring_role_arn` | String |  | <p>The ARN for the IAM role that permits RDS to send enhanced monitoring metrics to Amazon CloudWatch Logs. For
          example, <code>arn:aws:iam:123456789012:role/emaccess</code>. For information on creating a monitoring role,
      see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Monitoring.OS.html#USER_Monitoring.OS.Enabling">Setting Up and Enabling Enhanced Monitoring</a> 
          in the <i>Amazon RDS User Guide</i>.</p>
         <p>If <code>MonitoringInterval</code> is set to a value other than <code>0</code>, then you must supply a <code>MonitoringRoleArn</code> value.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p> |
| `domain_auth_secret_arn` | String |  | <p>The ARN for the Secrets Manager secret with the credentials for the user joining the domain.</p>
         <p>Example: <code>arn:aws:secretsmanager:region:account-number:secret:myselfmanagedADtestsecret-123456</code>
         </p> |
| `promotion_tier` | i64 |  | <p>The order of priority in which an Aurora Replica is promoted to the primary instance 
          after a failure of the existing primary instance. For more information, 
      see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/Concepts.AuroraHighAvailability.html#Aurora.Managing.FaultTolerance">
          Fault Tolerance for an Aurora DB Cluster</a> in the <i>Amazon Aurora User Guide</i>.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p>
         <p>Default: <code>1</code>
         </p>
         <p>Valid Values: <code>0 - 15</code>
         </p> |
| `network_type` | String |  | <p>The network type of the DB instance.</p>
         <p>The network type is determined by the <code>DBSubnetGroup</code> specified for the DB instance. 
            A <code>DBSubnetGroup</code> can support only the IPv4 protocol or the IPv4 and the IPv6 
            protocols (<code>DUAL</code>).</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_VPC.WorkingWithRDSInstanceinaVPC.html">
            Working with a DB instance in a VPC</a> in the 
            <i>Amazon RDS User Guide.</i>
         </p>
         <p>Valid Values: <code>IPV4 | DUAL</code>
         </p> |
| `storage_type` | String |  | <p>The storage type to associate with the DB instance.</p>
         <p>If you specify <code>io1</code>, <code>io2</code>, or <code>gp3</code>, you must also include a value for the
            <code>Iops</code> parameter.</p>
         <p>This setting doesn't apply to Amazon Aurora DB instances. Storage is managed by the DB cluster.</p>
         <p>Valid Values: <code>gp2 | gp3 | io1 | io2 | standard</code>
         </p>
         <p>Default: <code>io1</code>, if the <code>Iops</code> parameter is specified. Otherwise,
                <code>gp3</code>.</p> |
| `performance_insights_kms_key_id` | String |  | <p>The Amazon Web Services KMS key identifier for encryption of Performance Insights data.</p>
         <p>The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.</p>
         <p>If you don't specify a value for <code>PerformanceInsightsKMSKeyId</code>, then Amazon RDS 
            uses your default KMS key. There is a default KMS key for your Amazon Web Services account. 
            Your Amazon Web Services account has a different default KMS key for each Amazon Web Services Region.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p> |
| `db_name` | String |  | <p>The meaning of this parameter differs according to the database engine you use.</p>
         <dl>
            <dt>Amazon Aurora MySQL</dt>
            <dd>
               <p>The name of the database to create when the primary DB instance of the Aurora MySQL DB cluster is
                  created. If this parameter isn't specified for an Aurora MySQL DB cluster, no database is created 
                  in the DB cluster.</p>
               <p>Constraints:</p>
               <ul>
                  <li>
                     <p>Must contain 1 to 64 alphanumeric characters.</p>
                  </li>
                  <li>
                     <p>Must begin with a letter. Subsequent characters can be letters, underscores, or digits (0-9).</p>
                  </li>
                  <li>
                     <p>Can't be a word reserved by the database engine.</p>
                  </li>
               </ul>
            </dd>
            <dt>Amazon Aurora PostgreSQL</dt>
            <dd>
               <p>The name of the database to create when the primary DB instance of the Aurora PostgreSQL DB cluster is
                    created. A database named <code>postgres</code> is always created. If this parameter is specified, an additional database with this name is created.</p>
               <p>Constraints:</p>
               <ul>
                  <li>
                     <p>It must contain 1 to 63 alphanumeric characters.</p>
                  </li>
                  <li>
                     <p>Must begin with a letter. Subsequent characters can be letters, underscores, or digits
                          (0 to 9).</p>
                  </li>
                  <li>
                     <p>Can't be a word reserved by the database engine.</p>
                  </li>
               </ul>
            </dd>
            <dt>Amazon RDS Custom for Oracle</dt>
            <dd>
               <p>The Oracle System ID (SID) of the created RDS Custom DB instance. If you don't specify a value, the default value is <code>ORCL</code> for non-CDBs and
                <code>RDSCDB</code> for CDBs.</p>
               <p>Default: <code>ORCL</code>
               </p>
               <p>Constraints:</p>
               <ul>
                  <li>
                     <p>Must contain 1 to 8 alphanumeric characters.</p>
                  </li>
                  <li>
                     <p>Must contain a letter.</p>
                  </li>
                  <li>
                     <p>Can't be a word reserved by the database engine.</p>
                  </li>
               </ul>
            </dd>
            <dt>Amazon RDS Custom for SQL Server</dt>
            <dd>
               <p>Not applicable. Must be null.</p>
            </dd>
            <dt>RDS for Db2</dt>
            <dd>
               <p>The name of the database to create when the DB instance is created. If
                        this parameter isn't specified, no database is created in the DB instance.
                        In some cases, we recommend that you don't add a database name. For more
                        information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/db2-db-instance-prereqs.html#db2-prereqs-additional-considerations">Additional considerations</a> in the <i>Amazon RDS User
                            Guide</i>.</p>
               <p>Constraints:</p>
               <ul>
                  <li>
                     <p>Must contain 1 to 64 letters or numbers.</p>
                  </li>
                  <li>
                     <p>Must begin with a letter. Subsequent characters can be letters,
                                underscores, or digits (0-9).</p>
                  </li>
                  <li>
                     <p>Can't be a word reserved by the specified database engine.</p>
                  </li>
               </ul>
            </dd>
            <dt>RDS for MariaDB</dt>
            <dd>
               <p>The name of the database to create when the DB instance is created. If this parameter isn't specified, no database is created in the DB instance.</p>
               <p>Constraints:</p>
               <ul>
                  <li>
                     <p>Must contain 1 to 64 letters or numbers.</p>
                  </li>
                  <li>
                     <p>Must begin with a letter. Subsequent characters can be letters, underscores, or digits (0-9).</p>
                  </li>
                  <li>
                     <p>Can't be a word reserved by the specified database engine.</p>
                  </li>
               </ul>
            </dd>
            <dt>RDS for MySQL</dt>
            <dd>
               <p>The name of the database to create when the DB instance is created. If this parameter isn't specified, no database is created in the DB instance.</p>
               <p>Constraints:</p>
               <ul>
                  <li>
                     <p>Must contain 1 to 64 letters or numbers.</p>
                  </li>
                  <li>
                     <p>Must begin with a letter. Subsequent characters can be letters, underscores, or digits (0-9).</p>
                  </li>
                  <li>
                     <p>Can't be a word reserved by the specified database engine.</p>
                  </li>
               </ul>
            </dd>
            <dt>RDS for Oracle</dt>
            <dd>
               <p>The Oracle System ID (SID) of the created DB instance. If you don't specify a value, 
                    the default value is <code>ORCL</code>. You can't specify the 
                    string <code>null</code>, or any other reserved word, for <code>DBName</code>.</p>
               <p>Default: <code>ORCL</code>
               </p>
               <p>Constraints:</p>
               <ul>
                  <li>
                     <p>Can't be longer than 8 characters.</p>
                  </li>
               </ul>
            </dd>
            <dt>RDS for PostgreSQL</dt>
            <dd>
               <p>The name of the database to create when the DB instance is created. A database named <code>postgres</code> is always created. If this parameter is specified, an additional database with this name is created.</p>
               <p>Constraints:</p>
               <ul>
                  <li>
                     <p>Must contain 1 to 63 letters, numbers, or underscores.</p>
                  </li>
                  <li>
                     <p>Must begin with a letter. Subsequent characters can be letters, underscores, or digits (0-9).</p>
                  </li>
                  <li>
                     <p>Can't be a word reserved by the specified database engine.</p>
                  </li>
               </ul>
            </dd>
            <dt>RDS for SQL Server</dt>
            <dd>
               <p>Not applicable. Must be null.</p>
            </dd>
         </dl> |
| `license_model` | String |  | <p>The license model information for this DB instance.</p>
         <note>
            <p>License models for RDS for Db2 require additional configuration. The bring your
                own license (BYOL) model requires a custom parameter group and an Amazon Web Services License
                Manager self-managed license. The Db2 license through Amazon Web Services Marketplace model
                requires an Amazon Web Services Marketplace subscription. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/db2-licensing.html">Amazon
                    RDS for Db2 licensing options</a> in the <i>Amazon RDS User
                    Guide</i>.</p>
            <p>The default for RDS for Db2 is <code>bring-your-own-license</code>.</p>
         </note>
         <p>This setting doesn't apply to Amazon Aurora or RDS Custom DB instances.</p>
         <p>Valid Values:</p>
         <ul>
            <li>
               <p>RDS for Db2 - <code>bring-your-own-license | marketplace-license</code>
               </p>
            </li>
            <li>
               <p>RDS for MariaDB - <code>general-public-license</code>
               </p>
            </li>
            <li>
               <p>RDS for Microsoft SQL Server - <code>license-included</code>
               </p>
            </li>
            <li>
               <p>RDS for MySQL - <code>general-public-license</code>
               </p>
            </li>
            <li>
               <p>RDS for Oracle - <code>bring-your-own-license | license-included</code>
               </p>
            </li>
            <li>
               <p>RDS for PostgreSQL - <code>postgresql-license</code>
               </p>
            </li>
         </ul> |
| `vpc_security_group_ids` | Vec<String> |  | <p>A list of Amazon EC2 VPC security groups to associate with this DB instance.</p>
         <p>This setting doesn't apply to Amazon Aurora DB instances. The associated list of EC2 VPC security groups is managed by
          the DB cluster.</p>
         <p>Default: The default EC2 VPC security group for the DB subnet group's VPC.</p> |
| `multi_az` | bool |  | <p>Specifies whether the DB instance is a Multi-AZ deployment. You can't set 
          the <code>AvailabilityZone</code> parameter if the DB instance is a Multi-AZ deployment.</p>
         <p>This setting doesn't apply to Amazon Aurora because the DB instance Availability Zones (AZs)
          are managed by the DB cluster.</p> |
| `engine_lifecycle_support` | String |  | <p>The life cycle type for this DB instance.</p>
         <note>
            <p>By default, this value is set to <code>open-source-rds-extended-support</code>, which enrolls your DB instance into Amazon RDS Extended Support. 
              At the end of standard support, you can avoid charges for Extended Support by setting the value to <code>open-source-rds-extended-support-disabled</code>. In this case, 
              creating the DB instance will fail if the DB major version is past its end of standard support date.</p>
         </note>
         <p>This setting applies only to RDS for MySQL and RDS for PostgreSQL. For Amazon Aurora DB instances, the life cycle type is managed by the DB cluster.</p>
         <p>You can use this setting to enroll your DB instance into Amazon RDS Extended Support. With RDS Extended Support, 
        you can run the selected major engine version on your DB instance past the end of standard support for that engine version. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/extended-support.html">Amazon RDS Extended Support with Amazon RDS</a> in the <i>Amazon RDS User Guide</i>.</p>
         <p>Valid Values: <code>open-source-rds-extended-support | open-source-rds-extended-support-disabled</code>
         </p>
         <p>Default: <code>open-source-rds-extended-support</code>
         </p> |
| `db_subnet_group_name` | String |  | <p>A DB subnet group to associate with this DB instance.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must match the name of an existing DB subnet group.</p>
            </li>
         </ul>
         <p>Example: <code>mydbsubnetgroup</code>
         </p> |
| `domain_dns_ips` | String |  | <p>The IPv4 DNS IP addresses of your primary and secondary Active Directory domain controllers.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Two IP addresses must be provided.  If there isn't a secondary domain controller, use the IP address of the primary domain controller for both entries in the list.</p>
            </li>
         </ul>
         <p>Example: <code>123.124.125.126,234.235.236.237</code>
         </p> |
| `processor_features` | Vec<String> |  | <p>The number of CPU cores and the number of threads per core for the DB instance class of the DB instance.</p>
         <p>This setting doesn't apply to Amazon Aurora or RDS Custom DB instances.</p> |
| `ca_certificate_identifier` | String |  | <p>The CA certificate identifier to use for the DB instance's server certificate.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/UsingWithRDS.SSL.html">Using SSL/TLS to encrypt a connection to a DB 
            instance</a> in the <i>Amazon RDS User Guide</i> and 
            <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/UsingWithRDS.SSL.html">
            Using SSL/TLS to encrypt a connection to a DB cluster</a> in the <i>Amazon Aurora 
            User Guide</i>.</p> |
| `max_allocated_storage` | i64 |  | <p>The upper limit in gibibytes (GiB) to which Amazon RDS can automatically scale the storage of the DB instance.</p>
         <p>For more information about this setting, including limitations that apply to it, see 
            <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_PIOPS.StorageTypes.html#USER_PIOPS.Autoscaling">
                Managing capacity automatically with Amazon RDS storage autoscaling</a> 
            in the <i>Amazon RDS User Guide</i>.</p>
         <p>This setting doesn't apply to the following DB instances:</p>
         <ul>
            <li>
               <p>Amazon Aurora (Storage is managed by the DB cluster.)</p>
            </li>
            <li>
               <p>RDS Custom</p>
            </li>
         </ul> |
| `db_parameter_group_name` | String |  | <p>The name of the DB parameter group to associate with this DB instance. If you don't specify a value, then 
          Amazon RDS uses the default DB parameter group for the specified DB engine and version.</p>
         <p>This setting doesn't apply to RDS Custom DB instances.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 1 to 255 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>The first character must be a letter.</p>
            </li>
            <li>
               <p>Can't end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul> |
| `option_group_name` | String |  | <p>The option group to associate the DB instance with.</p>
         <p>Permanent options, such as the TDE option for Oracle Advanced Security TDE, can't be removed 
          from an option group. Also, that option group can't be removed from a DB instance after it is 
          associated with a DB instance.</p>
         <p>This setting doesn't apply to Amazon Aurora or RDS Custom DB instances.</p> |
| `enable_customer_owned_ip` | bool |  | <p>Specifies whether to enable a customer-owned IP address (CoIP) for an RDS
            on Outposts DB instance.</p>
         <p>A <i>CoIP</i> provides local or external connectivity to resources in
            your Outpost subnets through your on-premises network. For some use cases, a CoIP can
            provide lower latency for connections to the DB instance from outside of its virtual
            private cloud (VPC) on your local network.</p>
         <p>For more information about RDS on Outposts, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/rds-on-outposts.html">Working with Amazon RDS on Amazon Web Services Outposts</a> 
            in the <i>Amazon RDS User Guide</i>.</p>
         <p>For more information about CoIPs, see <a href="https://docs.aws.amazon.com/outposts/latest/userguide/routing.html#ip-addressing">Customer-owned IP addresses</a> 
            in the <i>Amazon Web Services Outposts User Guide</i>.</p> |
| `backup_target` | String |  | <p>The location for storing automated backups and manual snapshots.</p>
         <p>Valid Values:</p>
         <ul>
            <li>
               <p>
                  <code>local</code> (Dedicated Local Zone)</p>
            </li>
            <li>
               <p>
                  <code>outposts</code> (Amazon Web Services Outposts)</p>
            </li>
            <li>
               <p>
                  <code>region</code> (Amazon Web Services Region)</p>
            </li>
         </ul>
         <p>Default: <code>region</code>
         </p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/rds-on-outposts.html">Working 
            with Amazon RDS on Amazon Web Services Outposts</a> in the <i>Amazon RDS User Guide</i>.</p> |
| `manage_master_user_password` | bool |  | <p>Specifies whether to manage the master user password with Amazon Web Services Secrets Manager.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/rds-secrets-manager.html">Password management with Amazon Web Services Secrets Manager</a> 
            in the <i>Amazon RDS User Guide.</i>
         </p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Can't manage the master user password with Amazon Web Services Secrets Manager if <code>MasterUserPassword</code> 
                    is specified.</p>
            </li>
         </ul> |
| `deletion_protection` | bool |  | <p>Specifies whether the DB instance has deletion protection enabled. 
            The database can't be deleted when deletion protection is enabled. By default, 
            deletion protection isn't enabled. For more information, see 
            <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_DeleteInstance.html">
                Deleting a DB Instance</a>.</p>
         <p>This setting doesn't apply to Amazon Aurora DB instances. You can enable or disable deletion protection for the DB cluster. 
            For more information, see <code>CreateDBCluster</code>. DB instances in a DB 
            cluster can be deleted even when deletion protection is enabled for the DB cluster.</p> |
| `multi_tenant` | bool |  | <p>Specifies whether to use the multi-tenant configuration or the single-tenant
            configuration (default). This parameter only applies to RDS for Oracle container
            database (CDB) engines.</p>
         <p>Note the following restrictions: </p>
         <ul>
            <li>
               <p>The DB engine that you specify in the request must support the multi-tenant
                    configuration. If you attempt to enable the multi-tenant configuration on a DB
                    engine that doesn't support it, the request fails.</p>
            </li>
            <li>
               <p>If you specify the multi-tenant configuration when you create your DB instance,
                    you can't later modify this DB instance to use the single-tenant configuration.</p>
            </li>
         </ul> |
| `allocated_storage` | i64 |  | <p>The amount of storage in gibibytes (GiB) to allocate for the DB instance.</p>
         <p>This setting doesn't apply to Amazon Aurora DB instances. Aurora cluster volumes automatically grow as the amount of data in your 
                    database increases, though you are only charged for the space that you use in an Aurora cluster volume.</p>
         <dl>
            <dt>Amazon RDS Custom</dt>
            <dd>
               <p>Constraints to the amount of storage for each storage type are the following:</p>
               <ul>
                  <li>
                     <p>General Purpose (SSD) storage (gp2, gp3): Must be an integer from 40 to 65536 for RDS Custom for Oracle, 
                            16384 for RDS Custom for SQL Server.</p>
                  </li>
                  <li>
                     <p>Provisioned IOPS storage (io1, io2): Must be an integer from 40 to 65536 for RDS Custom for Oracle, 
                           16384 for RDS Custom for SQL Server.</p>
                  </li>
               </ul>
            </dd>
            <dt>RDS for Db2</dt>
            <dd>
               <p>Constraints to the amount of storage for each storage type are the following:</p>
               <ul>
                  <li>
                     <p>General Purpose (SSD) storage (gp3): Must be an integer from 20 to 65536.</p>
                  </li>
                  <li>
                     <p>Provisioned IOPS storage (io1, io2): Must be an integer from 100 to 65536.</p>
                  </li>
               </ul>
            </dd>
            <dt>RDS for MariaDB</dt>
            <dd>
               <p>Constraints to the amount of storage for each storage type are the following:</p>
               <ul>
                  <li>
                     <p>General Purpose (SSD) storage (gp2, gp3): Must be an integer from 20 to 65536.</p>
                  </li>
                  <li>
                     <p>Provisioned IOPS storage (io1, io2): Must be an integer from 100 to 65536.</p>
                  </li>
                  <li>
                     <p>Magnetic storage (standard): Must be an integer from 5 to 3072.</p>
                  </li>
               </ul>
            </dd>
            <dt>RDS for MySQL</dt>
            <dd>
               <p>Constraints to the amount of storage for each storage type are the following:</p>
               <ul>
                  <li>
                     <p>General Purpose (SSD) storage (gp2, gp3): Must be an integer from 20 to 65536.</p>
                  </li>
                  <li>
                     <p>Provisioned IOPS storage (io1, io2): Must be an integer from 100 to 65536.</p>
                  </li>
                  <li>
                     <p>Magnetic storage (standard): Must be an integer from 5 to 3072.</p>
                  </li>
               </ul>
            </dd>
            <dt>RDS for Oracle</dt>
            <dd>
               <p>Constraints to the amount of storage for each storage type are the following:</p>
               <ul>
                  <li>
                     <p>General Purpose (SSD) storage (gp2, gp3): Must be an integer from 20 to 65536.</p>
                  </li>
                  <li>
                     <p>Provisioned IOPS storage (io1, io2): Must be an integer from 100 to 65536.</p>
                  </li>
                  <li>
                     <p>Magnetic storage (standard): Must be an integer from 10 to 3072.</p>
                  </li>
               </ul>
            </dd>
            <dt>RDS for PostgreSQL</dt>
            <dd>
               <p>Constraints to the amount of storage for each storage type are the following:</p>
               <ul>
                  <li>
                     <p>General Purpose (SSD) storage (gp2, gp3): Must be an integer from 20 to 65536.</p>
                  </li>
                  <li>
                     <p>Provisioned IOPS storage (io1, io2): Must be an integer from 100 to 65536.</p>
                  </li>
                  <li>
                     <p>Magnetic storage (standard): Must be an integer from 5 to 3072.</p>
                  </li>
               </ul>
            </dd>
            <dt>RDS for SQL Server</dt>
            <dd>
               <p>Constraints to the amount of storage for each storage type are the following:</p>
               <ul>
                  <li>
                     <p>General Purpose (SSD) storage (gp2, gp3):</p>
                     <ul>
                        <li>
                           <p>Enterprise and Standard editions: Must be an integer from 20 to 16384.</p>
                        </li>
                        <li>
                           <p>Web and Express editions: Must be an integer from 20 to 16384.</p>
                        </li>
                     </ul>
                  </li>
                  <li>
                     <p>Provisioned IOPS storage (io1, io2):</p>
                     <ul>
                        <li>
                           <p>Enterprise and Standard editions: Must be an integer from 100 to 16384.</p>
                        </li>
                        <li>
                           <p>Web and Express editions: Must be an integer from 100 to 16384.</p>
                        </li>
                     </ul>
                  </li>
                  <li>
                     <p>Magnetic storage (standard):</p>
                     <ul>
                        <li>
                           <p>Enterprise and Standard editions: Must be an integer from 20 to 1024.</p>
                        </li>
                        <li>
                           <p>Web and Express editions: Must be an integer from 20 to 1024.</p>
                        </li>
                     </ul>
                  </li>
               </ul>
            </dd>
         </dl> |
| `master_user_authentication_type` | String |  | <p>Specifies the authentication type for the master user. With IAM master user authentication, you can configure the master DB user with IAM database authentication when you create a DB instance.</p>
         <p>You can specify one of the following values:</p>
         <ul>
            <li>
               <p>
                  <code>password</code> - Use standard database authentication with a password.</p>
            </li>
            <li>
               <p>
                  <code>iam-db-auth</code> - Use IAM database authentication for the master user.</p>
            </li>
         </ul>
         <p>This option is only valid for RDS for PostgreSQL and Aurora PostgreSQL engines.</p> |
| `iops` | i64 |  | <p>The amount of Provisioned IOPS (input/output operations per second) to initially allocate for the DB instance.
          For information about valid IOPS values, see 
          <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_Storage.html">Amazon RDS DB instance storage</a> 
          in the <i>Amazon RDS User Guide</i>.</p>
         <p>This setting doesn't apply to Amazon Aurora DB instances. Storage is managed by the DB cluster.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>For RDS for Db2, MariaDB, MySQL, Oracle, and PostgreSQL - Must be a multiple between .5 and 50 
          of the storage amount for the DB instance.</p>
            </li>
            <li>
               <p>For RDS for SQL Server - Must be a multiple between 1 and 50 of the storage amount for the DB instance.</p>
            </li>
         </ul> |
| `copy_tags_to_snapshot` | bool |  | <p>Specifies whether to copy tags from the DB instance to snapshots of the DB instance. By default, tags are not copied.</p>
         <p>This setting doesn't apply to Amazon Aurora DB instances. Copying tags to snapshots is managed by the DB cluster. Setting this
          value for an Aurora DB instance has no effect on the DB cluster setting.</p> |
| `timezone` | String |  | <p>The time zone of the DB instance. 
            The time zone parameter is currently supported only by <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/db2-time-zone">RDS for Db2</a> and
            <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_SQLServer.html#SQLServer.Concepts.General.TimeZone">RDS for SQL Server</a>.</p> |
| `domain_iam_role_name` | String |  | <p>The name of the IAM role to use when making API calls to the Directory Service.</p>
         <p>This setting doesn't apply to the following DB instances:</p>
         <ul>
            <li>
               <p>Amazon Aurora (The domain is managed by the DB cluster.)</p>
            </li>
            <li>
               <p>RDS Custom</p>
            </li>
         </ul> |
| `master_username` | String |  | <p>The name for the master user.</p>
         <p>This setting doesn't apply to Amazon Aurora DB instances. The name for the master user is managed by the DB cluster.</p>
         <p>This setting is required for RDS DB instances.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 1 to 16 letters, numbers, or underscores.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Can't be a reserved word for the chosen database engine.</p>
            </li>
         </ul> |
| `storage_throughput` | i64 |  | <p>The storage throughput value, in mebibyte per second (MiBps), for the DB instance.</p>
         <p>This setting applies only to the <code>gp3</code> storage type.</p>
         <p>This setting doesn't apply to Amazon Aurora or RDS Custom DB instances.</p> |
| `character_set_name` | String |  | <p>For supported engines, the character set (<code>CharacterSet</code>) to associate the DB instance with.</p>
         <p>This setting doesn't apply to the following DB instances:</p>
         <ul>
            <li>
               <p>Amazon Aurora - The character set is managed by
          the DB cluster. For more information, see <code>CreateDBCluster</code>.</p>
            </li>
            <li>
               <p>RDS Custom - However, if you need to change the character set, 
          you can change it on the database itself.</p>
            </li>
         </ul> |
| `tags` | Vec<String> |  | <p>Tags to assign to the DB instance.</p> |
| `db_instance_class` | String | ✅ | <p>The compute and memory capacity of the DB instance, for example <code>db.m5.large</code>.
          Not all DB instance classes are available in all Amazon Web Services Regions, or for all database engines.
          For the full list of DB instance classes, and availability for your engine, see
          <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Concepts.DBInstanceClass.html">DB instance 
          classes</a> in the <i>Amazon RDS User Guide</i> or 
          <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/Concepts.DBInstanceClass.html">Aurora 
          DB instance classes</a> in the <i>Amazon Aurora User Guide</i>.</p> |
| `db_security_groups` | Vec<String> |  | <p>A list of DB security groups to associate with this DB instance.</p>
         <p>This setting applies to the legacy EC2-Classic platform, which is no longer used to create 
            new DB instances. Use the <code>VpcSecurityGroupIds</code> setting instead.</p> |
| `database_insights_mode` | String |  | <p>The mode of Database Insights to enable for the DB instance.</p>
         <note>
            <p>Aurora DB instances inherit this value from the DB cluster, so you can't change this value.</p>
         </note> |
| `kms_key_id` | String |  | <p>The Amazon Web Services KMS key identifier for an encrypted DB instance.</p>
         <p>The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.
          To use a KMS key in a different Amazon Web Services account, specify the key ARN or alias ARN.</p>
         <p>This setting doesn't apply to Amazon Aurora DB instances. The Amazon Web Services KMS key identifier is managed by
          the DB cluster. For more information, see <code>CreateDBCluster</code>.</p>
         <p>If <code>StorageEncrypted</code> is enabled, and you do
        not specify a value for the <code>KmsKeyId</code> parameter, then
        Amazon RDS uses your default KMS key. There is a  
        default KMS key for your Amazon Web Services account. Your Amazon Web Services account has a different
        default KMS key for each Amazon Web Services Region.</p>
         <p>For Amazon RDS Custom, a KMS key is required for DB instances. For most RDS engines, if you leave this parameter empty 
          while enabling <code>StorageEncrypted</code>, the engine uses the default KMS key. However, RDS Custom 
          doesn't use the default key when this parameter is empty. You must explicitly specify a key.</p> |
| `engine` | String | ✅ | <p>The database engine to use for this DB instance.</p>
         <p>Not every database engine is available in every Amazon Web Services Region.</p>
         <p>Valid Values:</p>
         <ul>
            <li>
               <p>
                  <code>aurora-mysql</code> (for Aurora MySQL DB instances)</p>
            </li>
            <li>
               <p>
                  <code>aurora-postgresql</code> (for Aurora PostgreSQL DB instances)</p>
            </li>
            <li>
               <p>
                  <code>custom-oracle-ee</code> (for RDS Custom for Oracle DB instances)</p>
            </li>
            <li>
               <p>
                  <code>custom-oracle-ee-cdb</code> (for RDS Custom for Oracle DB instances)</p>
            </li>
            <li>
               <p>
                  <code>custom-oracle-se2</code> (for RDS Custom for Oracle DB instances)</p>
            </li>
            <li>
               <p>
                  <code>custom-oracle-se2-cdb</code> (for RDS Custom for Oracle DB
                    instances)</p>
            </li>
            <li>
               <p>
                  <code>custom-sqlserver-ee</code> (for RDS Custom for SQL Server DB instances)</p>
            </li>
            <li>
               <p>
                  <code>custom-sqlserver-se</code> (for RDS Custom for SQL Server DB instances)</p>
            </li>
            <li>
               <p>
                  <code>custom-sqlserver-web</code> (for RDS Custom for SQL Server DB instances)</p>
            </li>
            <li>
               <p>
                  <code>custom-sqlserver-dev</code> (for RDS Custom for SQL Server DB instances)</p>
            </li>
            <li>
               <p>
                  <code>db2-ae</code>
               </p>
            </li>
            <li>
               <p>
                  <code>db2-se</code>
               </p>
            </li>
            <li>
               <p>
                  <code>mariadb</code>
               </p>
            </li>
            <li>
               <p>
                  <code>mysql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-ee</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-ee-cdb</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-se2</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-se2-cdb</code>
               </p>
            </li>
            <li>
               <p>
                  <code>postgres</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-ee</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-se</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-ex</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-web</code>
               </p>
            </li>
         </ul> |
| `db_system_id` | String |  | <p>The Oracle system identifier (SID), which is the name of the Oracle database instance that 
            manages your database files. In this context, the term "Oracle database instance" refers exclusively 
            to the system global area (SGA) and Oracle background processes. If you don't specify a SID, 
            the value defaults to <code>RDSCDB</code>. The Oracle SID is also the name of your CDB.</p> |
| `engine_version` | String |  | <p>The version number of the database engine to use.</p>
         <p>This setting doesn't apply to Amazon Aurora DB instances. The version number of the database engine the DB
            instance uses is managed by the DB cluster.</p>
         <p>For a list of valid engine versions, use the <code>DescribeDBEngineVersions</code>
            operation.</p>
         <p>The following are the database engines and links to information about the major and minor versions that are available with 
          Amazon RDS. Not every database engine is available for every Amazon Web Services Region.</p>
         <dl>
            <dt>Amazon RDS Custom for Oracle</dt>
            <dd>
               <p>A custom engine version (CEV) that you have previously created. This setting is required for RDS Custom for Oracle. The CEV 
                name has the following format: 19.<i>customized_string</i>. A valid CEV name is  
                <code>19.my_cev1</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/custom-creating.html#custom-creating.create">
                Creating an RDS Custom for Oracle DB instance</a> in the <i>Amazon RDS User Guide</i>.</p>
            </dd>
            <dt>Amazon RDS Custom for SQL Server</dt>
            <dd>
               <p>See <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/custom-reqs-limits-MS.html">RDS Custom for SQL Server general requirements</a> 
                in the <i>Amazon RDS User Guide</i>.</p>
            </dd>
            <dt>RDS for Db2</dt>
            <dd>
               <p>For information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_Db2.html#Db2.Concepts.VersionMgmt">Db2 on Amazon RDS versions</a> in the 
                <i>Amazon RDS User Guide</i>.</p>
            </dd>
            <dt>RDS for MariaDB</dt>
            <dd>
               <p>For information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_MariaDB.html#MariaDB.Concepts.VersionMgmt">MariaDB on Amazon RDS versions</a> in the 
                <i>Amazon RDS User Guide</i>.</p>
            </dd>
            <dt>RDS for Microsoft SQL Server</dt>
            <dd>
               <p>For information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_SQLServer.html#SQLServer.Concepts.General.VersionSupport">Microsoft SQL Server versions on Amazon RDS</a> in the 
                  <i>Amazon RDS User Guide</i>.</p>
            </dd>
            <dt>RDS for MySQL</dt>
            <dd>
               <p>For information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_MySQL.html#MySQL.Concepts.VersionMgmt">MySQL on Amazon RDS versions</a> in the 
                <i>Amazon RDS User Guide</i>.</p>
            </dd>
            <dt>RDS for Oracle</dt>
            <dd>
               <p>For information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Appendix.Oracle.PatchComposition.html">Oracle Database Engine release notes</a> in the 
                <i>Amazon RDS User Guide</i>.</p>
            </dd>
            <dt>RDS for PostgreSQL</dt>
            <dd>
               <p>For information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_PostgreSQL.html#PostgreSQL.Concepts">Amazon RDS for PostgreSQL versions and extensions</a> in the 
                  <i>Amazon RDS User Guide</i>.</p>
            </dd>
         </dl> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_instance
db_instance = provider.rds.Db_instance {
    db_instance_identifier = "value"  # <p>The identifier for this DB instance. This parameter is stored as a lowercase string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Can't end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
         <p>Example: <code>mydbinstance</code>
         </p>
    db_instance_class = "value"  # <p>The compute and memory capacity of the DB instance, for example <code>db.m5.large</code>.
          Not all DB instance classes are available in all Amazon Web Services Regions, or for all database engines.
          For the full list of DB instance classes, and availability for your engine, see
          <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Concepts.DBInstanceClass.html">DB instance 
          classes</a> in the <i>Amazon RDS User Guide</i> or 
          <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/Concepts.DBInstanceClass.html">Aurora 
          DB instance classes</a> in the <i>Amazon Aurora User Guide</i>.</p>
    engine = "value"  # <p>The database engine to use for this DB instance.</p>
         <p>Not every database engine is available in every Amazon Web Services Region.</p>
         <p>Valid Values:</p>
         <ul>
            <li>
               <p>
                  <code>aurora-mysql</code> (for Aurora MySQL DB instances)</p>
            </li>
            <li>
               <p>
                  <code>aurora-postgresql</code> (for Aurora PostgreSQL DB instances)</p>
            </li>
            <li>
               <p>
                  <code>custom-oracle-ee</code> (for RDS Custom for Oracle DB instances)</p>
            </li>
            <li>
               <p>
                  <code>custom-oracle-ee-cdb</code> (for RDS Custom for Oracle DB instances)</p>
            </li>
            <li>
               <p>
                  <code>custom-oracle-se2</code> (for RDS Custom for Oracle DB instances)</p>
            </li>
            <li>
               <p>
                  <code>custom-oracle-se2-cdb</code> (for RDS Custom for Oracle DB
                    instances)</p>
            </li>
            <li>
               <p>
                  <code>custom-sqlserver-ee</code> (for RDS Custom for SQL Server DB instances)</p>
            </li>
            <li>
               <p>
                  <code>custom-sqlserver-se</code> (for RDS Custom for SQL Server DB instances)</p>
            </li>
            <li>
               <p>
                  <code>custom-sqlserver-web</code> (for RDS Custom for SQL Server DB instances)</p>
            </li>
            <li>
               <p>
                  <code>custom-sqlserver-dev</code> (for RDS Custom for SQL Server DB instances)</p>
            </li>
            <li>
               <p>
                  <code>db2-ae</code>
               </p>
            </li>
            <li>
               <p>
                  <code>db2-se</code>
               </p>
            </li>
            <li>
               <p>
                  <code>mariadb</code>
               </p>
            </li>
            <li>
               <p>
                  <code>mysql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-ee</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-ee-cdb</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-se2</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-se2-cdb</code>
               </p>
            </li>
            <li>
               <p>
                  <code>postgres</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-ee</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-se</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-ex</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-web</code>
               </p>
            </li>
         </ul>
}

```

---


### Db_cluster_parameter_group

DBClusterParameterGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String | ✅ | <p>The description for the DB cluster parameter group.</p> |
| `db_cluster_parameter_group_name` | String | ✅ | <p>The name of the DB cluster parameter group.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must not match the name of an existing DB cluster parameter group.</p>
            </li>
         </ul>
         <note>
            <p>This value is stored as a lowercase string.</p>
         </note> |
| `tags` | Vec<String> |  | <p>Tags to assign to the DB cluster parameter group.</p> |
| `db_parameter_group_family` | String | ✅ | <p>The DB cluster parameter group family name. A DB cluster parameter group can be associated with one and only one DB cluster 
          parameter group family, and can be applied only to a DB cluster running a database engine and engine version compatible with that DB cluster parameter group family.</p>
         <p>
            <b>Aurora MySQL</b>
         </p>
         <p>Example: <code>aurora-mysql5.7</code>, <code>aurora-mysql8.0</code>
         </p>
         <p>
            <b>Aurora PostgreSQL</b>
         </p>
         <p>Example: <code>aurora-postgresql14</code>
         </p>
         <p>
            <b>RDS for MySQL</b>
         </p>
         <p>Example: <code>mysql8.0</code>
         </p>
         <p>
            <b>RDS for PostgreSQL</b>
         </p>
         <p>Example: <code>postgres13</code>
         </p>
         <p>To list all of the available parameter group families for a DB engine, use the following command:</p>
         <p>
            <code>aws rds describe-db-engine-versions --query "DBEngineVersions[].DBParameterGroupFamily" --engine <engine></code>
         </p>
         <p>For example, to list all of the available parameter group families for the Aurora PostgreSQL DB engine, use the following command:</p>
         <p>
            <code>aws rds describe-db-engine-versions --query "DBEngineVersions[].DBParameterGroupFamily" --engine aurora-postgresql</code>
         </p>
         <note>
            <p>The output contains duplicates.</p>
         </note>
         <p>The following are the valid DB engine values:</p>
         <ul>
            <li>
               <p>
                  <code>aurora-mysql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>aurora-postgresql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>mysql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>postgres</code>
               </p>
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

# Create db_cluster_parameter_group
db_cluster_parameter_group = provider.rds.Db_cluster_parameter_group {
    description = "value"  # <p>The description for the DB cluster parameter group.</p>
    db_cluster_parameter_group_name = "value"  # <p>The name of the DB cluster parameter group.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must not match the name of an existing DB cluster parameter group.</p>
            </li>
         </ul>
         <note>
            <p>This value is stored as a lowercase string.</p>
         </note>
    db_parameter_group_family = "value"  # <p>The DB cluster parameter group family name. A DB cluster parameter group can be associated with one and only one DB cluster 
          parameter group family, and can be applied only to a DB cluster running a database engine and engine version compatible with that DB cluster parameter group family.</p>
         <p>
            <b>Aurora MySQL</b>
         </p>
         <p>Example: <code>aurora-mysql5.7</code>, <code>aurora-mysql8.0</code>
         </p>
         <p>
            <b>Aurora PostgreSQL</b>
         </p>
         <p>Example: <code>aurora-postgresql14</code>
         </p>
         <p>
            <b>RDS for MySQL</b>
         </p>
         <p>Example: <code>mysql8.0</code>
         </p>
         <p>
            <b>RDS for PostgreSQL</b>
         </p>
         <p>Example: <code>postgres13</code>
         </p>
         <p>To list all of the available parameter group families for a DB engine, use the following command:</p>
         <p>
            <code>aws rds describe-db-engine-versions --query "DBEngineVersions[].DBParameterGroupFamily" --engine <engine></code>
         </p>
         <p>For example, to list all of the available parameter group families for the Aurora PostgreSQL DB engine, use the following command:</p>
         <p>
            <code>aws rds describe-db-engine-versions --query "DBEngineVersions[].DBParameterGroupFamily" --engine aurora-postgresql</code>
         </p>
         <note>
            <p>The output contains duplicates.</p>
         </note>
         <p>The following are the valid DB engine values:</p>
         <ul>
            <li>
               <p>
                  <code>aurora-mysql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>aurora-postgresql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>mysql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>postgres</code>
               </p>
            </li>
         </ul>
}

```

---


### Db_parameter_group

DBParameterGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `db_parameter_group_family` | String | ✅ | <p>The DB parameter group family name. A DB parameter group can be associated with one and only one DB parameter group family, and can be applied only to a DB instance running a database engine and engine version compatible with that DB parameter group family.</p>
         <p>To list all of the available parameter group families for a DB engine, use the following command:</p>
         <p>
            <code>aws rds describe-db-engine-versions --query "DBEngineVersions[].DBParameterGroupFamily" --engine <engine></code>
         </p>
         <p>For example, to list all of the available parameter group families for the MySQL DB engine, use the following command:</p>
         <p>
            <code>aws rds describe-db-engine-versions --query "DBEngineVersions[].DBParameterGroupFamily" --engine mysql</code>
         </p>
         <note>
            <p>The output contains duplicates.</p>
         </note>
         <p>The following are the valid DB engine values:</p>
         <ul>
            <li>
               <p>
                  <code>aurora-mysql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>aurora-postgresql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>db2-ae</code>
               </p>
            </li>
            <li>
               <p>
                  <code>db2-se</code>
               </p>
            </li>
            <li>
               <p>
                  <code>mysql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-ee</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-ee-cdb</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-se2</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-se2-cdb</code>
               </p>
            </li>
            <li>
               <p>
                  <code>postgres</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-ee</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-se</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-ex</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-web</code>
               </p>
            </li>
         </ul> |
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
               <p>Can't end with a hyphen or contain two consecutive hyphens</p>
            </li>
         </ul>
         <note>
            <p>This value is stored as a lowercase string.</p>
         </note> |
| `description` | String | ✅ | <p>The description for the DB parameter group.</p> |
| `tags` | Vec<String> |  | <p>Tags to assign to the DB parameter group.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create db_parameter_group
db_parameter_group = provider.rds.Db_parameter_group {
    db_parameter_group_family = "value"  # <p>The DB parameter group family name. A DB parameter group can be associated with one and only one DB parameter group family, and can be applied only to a DB instance running a database engine and engine version compatible with that DB parameter group family.</p>
         <p>To list all of the available parameter group families for a DB engine, use the following command:</p>
         <p>
            <code>aws rds describe-db-engine-versions --query "DBEngineVersions[].DBParameterGroupFamily" --engine <engine></code>
         </p>
         <p>For example, to list all of the available parameter group families for the MySQL DB engine, use the following command:</p>
         <p>
            <code>aws rds describe-db-engine-versions --query "DBEngineVersions[].DBParameterGroupFamily" --engine mysql</code>
         </p>
         <note>
            <p>The output contains duplicates.</p>
         </note>
         <p>The following are the valid DB engine values:</p>
         <ul>
            <li>
               <p>
                  <code>aurora-mysql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>aurora-postgresql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>db2-ae</code>
               </p>
            </li>
            <li>
               <p>
                  <code>db2-se</code>
               </p>
            </li>
            <li>
               <p>
                  <code>mysql</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-ee</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-ee-cdb</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-se2</code>
               </p>
            </li>
            <li>
               <p>
                  <code>oracle-se2-cdb</code>
               </p>
            </li>
            <li>
               <p>
                  <code>postgres</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-ee</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-se</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-ex</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sqlserver-web</code>
               </p>
            </li>
         </ul>
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
               <p>Can't end with a hyphen or contain two consecutive hyphens</p>
            </li>
         </ul>
         <note>
            <p>This value is stored as a lowercase string.</p>
         </note>
    description = "value"  # <p>The description for the DB parameter group.</p>
}

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
| `marker` | String | <p>A pagination token that can be used in a later <code>DescribeDBClusters</code> request.</p> |
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


### Db_proxy_targets

DBProxyTargets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `targets` | Vec<String> | <p>An arbitrary number of <code>DBProxyTarget</code> objects, containing details of the corresponding targets.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous request.
        If this parameter is specified, the response includes only records beyond the marker,
        up to the value specified by <code>MaxRecords</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_proxy_targets outputs
db_proxy_targets_id = db_proxy_targets.id
db_proxy_targets_targets = db_proxy_targets.targets
db_proxy_targets_marker = db_proxy_targets.marker
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
| `marker` | String | <p>An optional pagination token provided by a previous request.
            If this parameter is specified, the response includes
            only records beyond the marker,
            up to the value specified by <code>MaxRecords</code>.</p> |
| `db_parameter_groups` | Vec<String> | <p>A list of <code>DBParameterGroup</code> instances.</p> |


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
db_parameter_groups_marker = db_parameter_groups.marker
db_parameter_groups_db_parameter_groups = db_parameter_groups.db_parameter_groups
```

---


### Integrations

Integrations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `integrations` | Vec<String> | <p>A list of integrations.</p> |
| `marker` | String | <p>A pagination token that can be used in a later <code>DescribeIntegrations</code>
            request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access integrations outputs
integrations_id = integrations.id
integrations_integrations = integrations.integrations
integrations_marker = integrations.marker
```

---


### Reserved_db_instances

ReservedDBInstances resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous request.
        If this parameter is specified, the response includes
        only records beyond the marker,
        up to the value specified by <code>MaxRecords</code>.</p> |
| `reserved_db_instances` | Vec<String> | <p>A list of reserved DB instances.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_db_instances outputs
reserved_db_instances_id = reserved_db_instances.id
reserved_db_instances_marker = reserved_db_instances.marker
reserved_db_instances_reserved_db_instances = reserved_db_instances.reserved_db_instances
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
| `db_instances` | Vec<String> | <p>A list of <code>DBInstance</code> instances.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous request.
            If this parameter is specified, the response includes
            only records beyond the marker,
            up to the value specified by <code>MaxRecords</code> .</p> |


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
db_instances_db_instances = db_instances.db_instances
db_instances_marker = db_instances.marker
```

---


### Option_groups

OptionGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous request.
            If this parameter is specified, the response includes
            only records beyond the marker,
            up to the value specified by <code>MaxRecords</code>.</p> |
| `option_groups_list` | Vec<String> | <p>List of option groups.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access option_groups outputs
option_groups_id = option_groups.id
option_groups_marker = option_groups.marker
option_groups_option_groups_list = option_groups.option_groups_list
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


### Global_clusters

GlobalClusters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous <code>DescribeGlobalClusters</code> request.
        If this parameter is specified, the response includes
        only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p> |
| `global_clusters` | Vec<String> | <p>The list of global clusters returned by this request.</p> |


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
global_clusters_marker = global_clusters.marker
global_clusters_global_clusters = global_clusters.global_clusters
```

---


### Db_snapshot_attributes

DBSnapshotAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_snapshot_attributes_result` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access db_snapshot_attributes outputs
db_snapshot_attributes_id = db_snapshot_attributes.id
db_snapshot_attributes_db_snapshot_attributes_result = db_snapshot_attributes.db_snapshot_attributes_result
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple db_subnet_groups resources
db_subnet_groups_0 = provider.rds.Db_subnet_groups {
}
db_subnet_groups_1 = provider.rds.Db_subnet_groups {
}
db_subnet_groups_2 = provider.rds.Db_subnet_groups {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    db_subnet_groups = provider.rds.Db_subnet_groups {
    }
```

---

## Related Documentation

- [AWS Rds Documentation](https://docs.aws.amazon.com/rds/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
