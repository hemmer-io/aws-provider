# Opensearch Service



**Resources**: 29

---

## Overview

The opensearch service provides access to 29 resource types:

- [Domain_health](#domain_health) [R]
- [Reserved_instance_offerings](#reserved_instance_offerings) [R]
- [Upgrade_status](#upgrade_status) [R]
- [Reserved_instances](#reserved_instances) [R]
- [Outbound_connections](#outbound_connections) [R]
- [Domain_config](#domain_config) [RU]
- [Domain](#domain) [CRD]
- [Packages](#packages) [R]
- [Inbound_connections](#inbound_connections) [R]
- [Domain_change_progress](#domain_change_progress) [R]
- [Instance_type_limits](#instance_type_limits) [R]
- [Outbound_connection](#outbound_connection) [CD]
- [Scheduled_action](#scheduled_action) [U]
- [Package](#package) [CUD]
- [Dry_run_progress](#dry_run_progress) [R]
- [Domain_auto_tunes](#domain_auto_tunes) [R]
- [Direct_query_data_source](#direct_query_data_source) [RUD]
- [Inbound_connection](#inbound_connection) [D]
- [Data_source](#data_source) [RUD]
- [Domains](#domains) [R]
- [Vpc_endpoint](#vpc_endpoint) [CUD]
- [Application](#application) [CRUD]
- [Compatible_versions](#compatible_versions) [R]
- [Vpc_endpoints](#vpc_endpoints) [R]
- [Package_scope](#package_scope) [U]
- [Domain_nodes](#domain_nodes) [R]
- [Package_version_history](#package_version_history) [R]
- [Upgrade_history](#upgrade_history) [R]
- [Domain_maintenance_status](#domain_maintenance_status) [R]

---

## Resources


### Domain_health

DomainHealth resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_state` | String | <p>The current state of the domain.</p>
         <ul>
            <li>
               <p>
                  <code>Processing</code> - The domain has updates in progress.</p>
            </li>
            <li>
               <p>
                  <code>Active</code> - Requested changes have been processed and deployed to the domain.</p>
            </li>
         </ul> |
| `total_shards` | String | <p>The total number of primary and replica shards for the domain.</p> |
| `cluster_health` | String | <p>The current health status of your cluster.</p>
         <ul>
            <li>
               <p>
                  <code>Red</code> - At least one primary shard is not allocated to any node.</p>
            </li>
            <li>
               <p>
                  <code>Yellow</code> - All primary shards are allocated to nodes, but some replicas aren’t.</p>
            </li>
            <li>
               <p>
                  <code>Green</code> - All primary shards and their replicas are allocated to nodes.</p>
            </li>
            <li>
               <p>
                  <code>NotAvailable</code> - Unable to retrieve cluster health.</p>
            </li>
         </ul> |
| `total_un_assigned_shards` | String | <p>The total number of primary and replica shards not allocated to any of the nodes for the cluster.</p> |
| `dedicated_master` | bool | <p>A boolean that indicates if dedicated master nodes are activated for the domain.</p> |
| `active_availability_zone_count` | String | <p>The number of active Availability Zones configured for the domain. If the service is unable to fetch this information, it will return <code>NotAvailable</code>.</p> |
| `availability_zone_count` | String | <p>The number of Availability Zones configured for the domain. If the service is unable to fetch this information, it will return <code>NotAvailable</code>.</p> |
| `environment_information` | Vec<String> | <p>A list of <code>EnvironmentInfo</code> for the domain. </p> |
| `master_eligible_node_count` | String | <p>The number of nodes that can be elected as a master node. If dedicated master nodes is turned on, this value is the number of dedicated master nodes configured for the domain.
   If the service is unable to fetch this information, it will return <code>NotAvailable</code>.</p> |
| `warm_node_count` | String | <p>The number of warm nodes configured for the domain.</p> |
| `stand_by_availability_zone_count` | String | <p>The number of standby Availability Zones configured for the domain. If the service is unable to fetch this information, it will return <code>NotAvailable</code>.</p> |
| `data_node_count` | String | <p>The number of data nodes configured for the domain. If the service is unable to fetch this information, it will return <code>NotAvailable</code>.</p> |
| `master_node` | String | <p>Indicates whether the domain has an elected master node.</p>
         <ul>
            <li>
               <p>
                  <b>Available</b> - The domain has an elected master node.</p>
            </li>
            <li>
               <p>
                  <b>UnAvailable</b> - The master node hasn't yet been elected, and a quorum to elect a new master node hasn't been reached.</p>
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

# Access domain_health outputs
domain_health_id = domain_health.id
domain_health_domain_state = domain_health.domain_state
domain_health_total_shards = domain_health.total_shards
domain_health_cluster_health = domain_health.cluster_health
domain_health_total_un_assigned_shards = domain_health.total_un_assigned_shards
domain_health_dedicated_master = domain_health.dedicated_master
domain_health_active_availability_zone_count = domain_health.active_availability_zone_count
domain_health_availability_zone_count = domain_health.availability_zone_count
domain_health_environment_information = domain_health.environment_information
domain_health_master_eligible_node_count = domain_health.master_eligible_node_count
domain_health_warm_node_count = domain_health.warm_node_count
domain_health_stand_by_availability_zone_count = domain_health.stand_by_availability_zone_count
domain_health_data_node_count = domain_health.data_node_count
domain_health_master_node = domain_health.master_node
```

---


### Reserved_instance_offerings

ReservedInstanceOfferings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>When <code>nextToken</code> is returned, there are more results available. The value of
    <code>nextToken</code> is a unique pagination token for each page. Send the request again using the
   returned token to retrieve the next page.</p> |
| `reserved_instance_offerings` | Vec<String> | <p>List of Reserved Instance offerings.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_instance_offerings outputs
reserved_instance_offerings_id = reserved_instance_offerings.id
reserved_instance_offerings_next_token = reserved_instance_offerings.next_token
reserved_instance_offerings_reserved_instance_offerings = reserved_instance_offerings.reserved_instance_offerings
```

---


### Upgrade_status

UpgradeStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `upgrade_name` | String | <p>A string that describes the update.</p> |
| `upgrade_step` | String | <p>One of three steps that an upgrade or upgrade eligibility check goes through.</p> |
| `step_status` | String | <p>The status of the current step that an upgrade is on.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access upgrade_status outputs
upgrade_status_id = upgrade_status.id
upgrade_status_upgrade_name = upgrade_status.upgrade_name
upgrade_status_upgrade_step = upgrade_status.upgrade_step
upgrade_status_step_status = upgrade_status.step_status
```

---


### Reserved_instances

ReservedInstances resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reserved_instances` | Vec<String> | <p>List of Reserved Instances in the current Region.</p> |
| `next_token` | String | <p>When <code>nextToken</code> is returned, there are more results available. The value of
    <code>nextToken</code> is a unique pagination token for each page. Send the request again using the
   returned token to retrieve the next page.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_instances outputs
reserved_instances_id = reserved_instances.id
reserved_instances_reserved_instances = reserved_instances.reserved_instances
reserved_instances_next_token = reserved_instances.next_token
```

---


### Outbound_connections

OutboundConnections resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connections` | Vec<String> | <p>List of outbound connections that match the filter criteria.</p> |
| `next_token` | String | <p>When <code>nextToken</code> is returned, there are more results available. The value of
    <code>nextToken</code> is a unique pagination token for each page. Send the request again using the
   returned token to retrieve the next page.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access outbound_connections outputs
outbound_connections_id = outbound_connections.id
outbound_connections_connections = outbound_connections.connections
outbound_connections_next_token = outbound_connections.next_token
```

---


### Domain_config

DomainConfig resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `snapshot_options` | String |  | <p>Option to set the time, in UTC format, for the daily automated snapshot. Default value is <code>0</code> hours.
  </p> |
| `off_peak_window_options` | String |  | <p>Off-peak window options for the domain.</p> |
| `aiml_options` | String |  | <p>Options for all machine learning features for the specified domain.</p> |
| `log_publishing_options` | HashMap<String, String> |  | <p>Options to publish OpenSearch logs to Amazon CloudWatch Logs.</p> |
| `ip_address_type` | String |  | <p>Specify either dual stack or IPv4 as your IP address type. Dual stack allows you to share domain resources across
   IPv4 and IPv6 address types, and is the recommended option. 
   If your IP address type is currently set to dual stack, you can't change it.
  </p> |
| `domain_endpoint_options` | String |  | <p>Additional options for the domain endpoint, such as whether to require HTTPS for all
   traffic.</p> |
| `identity_center_options` | String |  |  |
| `cluster_config` | String |  | <p>Changes that you want to make to the cluster configuration, such as the instance type and
   number of EC2 instances.</p> |
| `node_to_node_encryption_options` | String |  | <p>Node-to-node encryption options for the domain.</p> |
| `software_update_options` | String |  | <p>Service software update options for the domain.</p> |
| `ebs_options` | String |  | <p>The type and size of the EBS volume to attach to instances in the domain.</p> |
| `advanced_security_options` | String |  | <p>Options for fine-grained access control.</p> |
| `cognito_options` | String |  | <p>Key-value pairs to configure Amazon Cognito authentication for OpenSearch Dashboards.</p> |
| `advanced_options` | HashMap<String, String> |  | <p>Key-value pairs to specify advanced configuration options. The following key-value pairs are
   supported:</p>
         <ul>
            <li>
               <p>
                  <code>"rest.action.multi.allow_explicit_index": "true" | "false"</code> - Note the use of
     a string rather than a boolean. Specifies whether explicit references to indexes are allowed
     inside the body of HTTP requests. If you want to configure access policies for domain
     sub-resources, such as specific indexes and domain APIs, you must disable this property.
     Default is true.</p>
            </li>
            <li>
               <p>
                  <code>"indices.fielddata.cache.size": "80" </code> - Note the use of a string rather than
     a boolean. Specifies the percentage of heap space allocated to field data. Default is
     unbounded.</p>
            </li>
            <li>
               <p>
                  <code>"indices.query.bool.max_clause_count": "1024"</code> - Note the use of a string
     rather than a boolean. Specifies the maximum number of clauses allowed in a Lucene boolean
     query. Default is 1,024. Queries with more than the permitted number of clauses result in a
     <code>TooManyClauses</code> error.</p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/opensearch-service/latest/developerguide/createupdatedomains.html#createdomain-configure-advanced-options">Advanced cluster parameters</a>.</p> |
| `auto_tune_options` | String |  | <p>Options for Auto-Tune.</p> |
| `domain_name` | String | ✅ | <p>The name of the domain that you're updating.</p> |
| `dry_run` | bool |  | <p>This flag, when set to True, specifies whether the <code>UpdateDomain</code> request should
   return the results of a dry run analysis without actually applying the change. A dry run
   determines what type of deployment the update will cause.</p> |
| `dry_run_mode` | String |  | <p>The type of dry run to perform.</p>
         <ul>
            <li>
               <p>
                  <code>Basic</code> only returns the type of deployment (blue/green or dynamic) that the update
     will cause.</p>
            </li>
            <li>
               <p>
                  <code>Verbose</code> runs an additional check to validate the changes you're making. For
     more information, see <a href="https://docs.aws.amazon.com/opensearch-service/latest/developerguide/managedomains-configuration-changes#validation-check">Validating a domain update</a>.</p>
            </li>
         </ul> |
| `vpc_options` | String |  | <p>Options to specify the subnets and security groups for a VPC endpoint. For more information,
   see <a href="https://docs.aws.amazon.com/opensearch-service/latest/developerguide/vpc.html">Launching
    your Amazon OpenSearch Service domains using a VPC</a>.</p> |
| `encryption_at_rest_options` | String |  | <p>Encryption at rest options for the domain.</p> |
| `access_policies` | String |  | <p>Identity and Access Management (IAM) access policy as a JSON-formatted string.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_config` | String | <p>Container for the configuration of the OpenSearch Service domain.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access domain_config outputs
domain_config_id = domain_config.id
domain_config_domain_config = domain_config.domain_config
```

---


### Domain

Domain resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `identity_center_options` | String |  | <p>Configuration options for enabling and managing IAM Identity Center integration within a domain.</p> |
| `cluster_config` | String |  | <p>Container for the cluster configuration of a domain.</p> |
| `advanced_options` | HashMap<String, String> |  | <p>Key-value pairs to specify advanced configuration options. The following key-value pairs are
   supported:</p>
         <ul>
            <li>
               <p>
                  <code>"rest.action.multi.allow_explicit_index": "true" | "false"</code> - Note the use of
     a string rather than a boolean. Specifies whether explicit references to indexes are allowed
     inside the body of HTTP requests. If you want to configure access policies for domain
     sub-resources, such as specific indexes and domain APIs, you must disable this property.
     Default is true.</p>
            </li>
            <li>
               <p>
                  <code>"indices.fielddata.cache.size": "80" </code> - Note the use of a string rather than
     a boolean. Specifies the percentage of heap space allocated to field data. Default is
     unbounded.</p>
            </li>
            <li>
               <p>
                  <code>"indices.query.bool.max_clause_count": "1024"</code> - Note the use of a string
     rather than a boolean. Specifies the maximum number of clauses allowed in a Lucene boolean
     query. Default is 1,024. Queries with more than the permitted number of clauses result in a
      <code>TooManyClauses</code> error.</p>
            </li>
            <li>
               <p>
                  <code>"override_main_response_version": "true" | "false"</code> - Note the use of a string
     rather than a boolean. Specifies whether the domain reports its version as 7.10 to allow
     Elasticsearch OSS clients and plugins to continue working with it. Default is false when
     creating a domain and true when upgrading a domain.</p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/opensearch-service/latest/developerguide/createupdatedomains.html#createdomain-configure-advanced-options">Advanced cluster parameters</a>.</p> |
| `encryption_at_rest_options` | String |  | <p>Key-value pairs to enable encryption at rest.</p> |
| `domain_endpoint_options` | String |  | <p>Additional options for the domain endpoint, such as whether to require HTTPS for all
   traffic.</p> |
| `ebs_options` | String |  | <p>Container for the parameters required to enable EBS-based storage for an OpenSearch Service
   domain.</p> |
| `ip_address_type` | String |  | <p>Specify either dual stack or IPv4 as your IP address type. Dual stack allows you to share
   domain resources across IPv4 and IPv6 address types, and is the recommended option. 
   If you set your IP address type to dual stack, you can't change your address type later.</p> |
| `access_policies` | String |  | <p>Identity and Access Management (IAM) policy document specifying the access policies for the
   new domain.</p> |
| `tag_list` | Vec<String> |  | <p>List of tags to add to the domain upon creation.</p> |
| `engine_version` | String |  | <p>String of format Elasticsearch_X.Y or OpenSearch_X.Y to specify the engine version for the
   OpenSearch Service domain. For example, <code>OpenSearch_1.0</code> or
    <code>Elasticsearch_7.9</code>. For more information, see <a href="https://docs.aws.amazon.com/opensearch-service/latest/developerguide/createupdatedomains.html#createdomains">Creating
    and managing Amazon OpenSearch Service domains</a>.</p> |
| `log_publishing_options` | HashMap<String, String> |  | <p>Key-value pairs to configure log publishing.</p> |
| `advanced_security_options` | String |  | <p>Options for fine-grained access control.</p> |
| `software_update_options` | String |  | <p>Software update options for the domain.</p> |
| `off_peak_window_options` | String |  | <p>Specifies a daily 10-hour time block during which OpenSearch Service can perform
   configuration changes on the domain, including service software updates and Auto-Tune
   enhancements that require a blue/green deployment. If no options are specified, the default start
   time of 10:00 P.M. local time (for the Region that the domain is created in) is used.</p> |
| `auto_tune_options` | String |  | <p>Options for Auto-Tune.</p> |
| `aiml_options` | String |  | <p>Options for all machine learning features for the specified domain.</p> |
| `node_to_node_encryption_options` | String |  | <p>Enables node-to-node encryption.</p> |
| `snapshot_options` | String |  | <p>DEPRECATED. Container for the parameters required to configure automated snapshots of domain
   indexes.</p> |
| `domain_name` | String | ✅ | <p>Name of the OpenSearch Service domain to create. Domain names are unique across the domains
   owned by an account within an Amazon Web Services Region.</p> |
| `vpc_options` | String |  | <p>Container for the values required to configure VPC access domains. If you don't specify
   these values, OpenSearch Service creates the domain with a public endpoint. For more information,
   see <a href="https://docs.aws.amazon.com/opensearch-service/latest/developerguide/vpc.html">Launching
    your Amazon OpenSearch Service domains using a VPC</a>.</p> |
| `cognito_options` | String |  | <p>Key-value pairs to configure Amazon Cognito authentication. For more information, see <a href="https://docs.aws.amazon.com/opensearch-service/latest/developerguide/cognito-auth.html">Configuring Amazon Cognito authentication for OpenSearch Dashboards</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_status` | String | <p>List that contains the status of each specified OpenSearch Service domain.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create domain
domain = provider.opensearch.Domain {
    domain_name = "value"  # <p>Name of the OpenSearch Service domain to create. Domain names are unique across the domains
   owned by an account within an Amazon Web Services Region.</p>
}

# Access domain outputs
domain_id = domain.id
domain_domain_status = domain.domain_status
```

---


### Packages

Packages resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>When <code>nextToken</code> is returned, there are more results available. The value of
   <code>nextToken</code> is a unique pagination token for each page. Send the request again using the
   returned token to retrieve the next page.</p> |
| `package_details_list` | Vec<String> | <p>Basic information about a package.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access packages outputs
packages_id = packages.id
packages_next_token = packages.next_token
packages_package_details_list = packages.package_details_list
```

---


### Inbound_connections

InboundConnections resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>When <code>nextToken</code> is returned, there are more results available. The value of
    <code>nextToken</code> is a unique pagination token for each page. Send the request again using the
   returned token to retrieve the next page.</p> |
| `connections` | Vec<String> | <p>List of inbound connections.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access inbound_connections outputs
inbound_connections_id = inbound_connections.id
inbound_connections_next_token = inbound_connections.next_token
inbound_connections_connections = inbound_connections.connections
```

---


### Domain_change_progress

DomainChangeProgress resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `change_progress_status` | String | <p>Container for information about the stages of a configuration change happening on a domain.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access domain_change_progress outputs
domain_change_progress_id = domain_change_progress.id
domain_change_progress_change_progress_status = domain_change_progress.change_progress_status
```

---


### Instance_type_limits

InstanceTypeLimits resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `limits_by_role` | HashMap<String, String> | <p>Map that contains all applicable instance type limits.<code>data</code> refers to data
   nodes.<code>master</code> refers to dedicated master nodes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_type_limits outputs
instance_type_limits_id = instance_type_limits.id
instance_type_limits_limits_by_role = instance_type_limits.limits_by_role
```

---


### Outbound_connection

OutboundConnection resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connection_alias` | String | ✅ | <p>Name of the connection.</p> |
| `remote_domain_info` | String | ✅ | <p>Name and Region of the destination (remote) domain.</p> |
| `connection_mode` | String |  | <p>The connection mode.</p> |
| `local_domain_info` | String | ✅ | <p>Name and Region of the source (local) domain.</p> |
| `connection_properties` | String |  | <p>The <code>ConnectionProperties</code> for the outbound connection.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create outbound_connection
outbound_connection = provider.opensearch.Outbound_connection {
    connection_alias = "value"  # <p>Name of the connection.</p>
    remote_domain_info = "value"  # <p>Name and Region of the destination (remote) domain.</p>
    local_domain_info = "value"  # <p>Name and Region of the source (local) domain.</p>
}

```

---


### Scheduled_action

ScheduledAction resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_name` | String | ✅ | <p>The name of the domain to reschedule an action for.</p> |
| `action_id` | String | ✅ | <p>The unique identifier of the action to reschedule. To retrieve this ID, send a <a href="https://docs.aws.amazon.com/opensearch-service/latest/APIReference/API_ListScheduledActions.html">ListScheduledActions</a> request.</p> |
| `action_type` | String | ✅ | <p>The type of action to reschedule. Can be one of <code>SERVICE_SOFTWARE_UPDATE</code>,
   <code>JVM_HEAP_SIZE_TUNING</code>, or <code>JVM_YOUNG_GEN_TUNING</code>. To retrieve this value, send a <a href="https://docs.aws.amazon.com/opensearch-service/latest/APIReference/API_ListScheduledActions.html">ListScheduledActions</a> request.</p> |
| `schedule_at` | String | ✅ | <p>When to schedule the action.</p>
         <ul>
            <li>
               <p>
                  <code>NOW</code> - Immediately schedules the update to happen in the current hour if
     there's capacity available.</p>
            </li>
            <li>
               <p>
                  <code>TIMESTAMP</code> - Lets you specify a custom date and time to apply the update. If
     you specify this value, you must also provide a value for <code>DesiredStartTime</code>.</p>
            </li>
            <li>
               <p>
                  <code>OFF_PEAK_WINDOW</code> - Marks the action to be picked up during an upcoming
     off-peak window. There's no guarantee that the change will be implemented during the next
     immediate window. Depending on capacity, it might happen in subsequent days.</p>
            </li>
         </ul> |
| `desired_start_time` | i64 |  | <p>The time to implement the change, in Coordinated Universal Time (UTC). Only specify this
   parameter if you set <code>ScheduleAt</code> to <code>TIMESTAMP</code>.</p> |



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


### Package

Package resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `package_source` | String | ✅ | <p>The Amazon S3 location from which to import the package.</p> |
| `package_vending_options` | String |  | <p> The vending options for the package being created. They determine if the package can be vended to other users.</p> |
| `package_description` | String |  | <p>Description of the package.</p> |
| `package_configuration` | String |  | <p> The configuration parameters for the package being created.</p> |
| `package_encryption_options` | String |  | <p>The encryption parameters for the package being created.</p> |
| `engine_version` | String |  | <p>The version of the Amazon OpenSearch Service engine for which is compatible with the package. This can only be specified for package type <code>ZIP-PLUGIN</code>
         </p> |
| `package_name` | String | ✅ | <p>Unique name for the package.</p> |
| `package_type` | String | ✅ | <p>The type of package.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create package
package = provider.opensearch.Package {
    package_source = "value"  # <p>The Amazon S3 location from which to import the package.</p>
    package_name = "value"  # <p>Unique name for the package.</p>
    package_type = "value"  # <p>The type of package.</p>
}

```

---


### Dry_run_progress

DryRunProgress resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dry_run_results` | String | <p>The results of the dry run. </p> |
| `dry_run_config` | String | <p>Details about the changes you're planning to make on the domain.</p> |
| `dry_run_progress_status` | String | <p>The current status of the dry run, including any validation errors.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dry_run_progress outputs
dry_run_progress_id = dry_run_progress.id
dry_run_progress_dry_run_results = dry_run_progress.dry_run_results
dry_run_progress_dry_run_config = dry_run_progress.dry_run_config
dry_run_progress_dry_run_progress_status = dry_run_progress.dry_run_progress_status
```

---


### Domain_auto_tunes

DomainAutoTunes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auto_tunes` | Vec<String> | <p>The list of setting adjustments that Auto-Tune has made to the domain.</p> |
| `next_token` | String | <p>When <code>nextToken</code> is returned, there are more results available. The value of
    <code>nextToken</code> is a unique pagination token for each page. Send the request again using the
   returned token to retrieve the next page.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access domain_auto_tunes outputs
domain_auto_tunes_id = domain_auto_tunes.id
domain_auto_tunes_auto_tunes = domain_auto_tunes.auto_tunes
domain_auto_tunes_next_token = domain_auto_tunes.next_token
```

---


### Direct_query_data_source

DirectQueryDataSource resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_source_type` | String | ✅ | <p>
   The supported Amazon Web Services service that you want to use as the source for 
   direct queries in OpenSearch Service. 
  </p> |
| `description` | String |  | <p>
   An optional text field for providing additional context and 
   details about the data source.
  </p> |
| `open_search_arns` | Vec<String> | ✅ | <p>
   A list of Amazon Resource Names (ARNs) for the OpenSearch 
   collections that are associated with the direct query data source.
  </p> |
| `data_source_name` | String | ✅ | <p>
   A unique, user-defined label to identify the data 
   source within your OpenSearch Service environment.
  </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>
   A description that provides additional context and details about the data source.
  </p> |
| `data_source_name` | String | <p>
   A unique, user-defined label to identify the data source 
   within your OpenSearch Service environment.
  </p> |
| `open_search_arns` | Vec<String> | <p>
   A list of Amazon Resource Names (ARNs) for the OpenSearch 
   collections that are associated with the direct query data source.
  </p> |
| `data_source_type` | String | <p>
   The supported Amazon Web Services service that is used as the source for 
   direct queries in OpenSearch Service. 
  </p> |
| `data_source_arn` | String | <p>
   The unique, system-generated identifier that represents the data source.
  </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access direct_query_data_source outputs
direct_query_data_source_id = direct_query_data_source.id
direct_query_data_source_description = direct_query_data_source.description
direct_query_data_source_data_source_name = direct_query_data_source.data_source_name
direct_query_data_source_open_search_arns = direct_query_data_source.open_search_arns
direct_query_data_source_data_source_type = direct_query_data_source.data_source_type
direct_query_data_source_data_source_arn = direct_query_data_source.data_source_arn
```

---


### Inbound_connection

InboundConnection resource

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


### Data_source

DataSource resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A new description of the data source.</p> |
| `data_source_type` | String | ✅ | <p>The type of data source.</p> |
| `domain_name` | String | ✅ | <p>The name of the domain.</p> |
| `name` | String | ✅ | <p>The name of the data source to modify.</p> |
| `status` | String |  | <p>The status of the data source update.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The status of the data source.</p> |
| `data_source_type` | String | <p>The type of data source.</p> |
| `description` | String | <p>A description of the data source.</p> |
| `name` | String | <p>The name of the data source.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_source outputs
data_source_id = data_source.id
data_source_status = data_source.status
data_source_data_source_type = data_source.data_source_type
data_source_description = data_source.description
data_source_name = data_source.name
```

---


### Domains

Domains resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_status_list` | Vec<String> | <p>The status of the requested domains.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access domains outputs
domains_id = domains.id
domains_domain_status_list = domains.domain_status_list
```

---


### Vpc_endpoint

VpcEndpoint resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>Unique, case-sensitive identifier to ensure idempotency of the request.</p> |
| `domain_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the domain to create the endpoint for.</p> |
| `vpc_options` | String | ✅ | <p>Options to specify the subnets and security groups for the endpoint.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_endpoint
vpc_endpoint = provider.opensearch.Vpc_endpoint {
    domain_arn = "value"  # <p>The Amazon Resource Name (ARN) of the domain to create the endpoint for.</p>
    vpc_options = "value"  # <p>Options to specify the subnets and security groups for the endpoint.</p>
}

```

---


### Application

Application resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_configs` | Vec<String> |  | <p>Configuration settings for the OpenSearch application, including administrative options.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier to ensure idempotency of the request.</p> |
| `data_sources` | Vec<String> |  | <p>The data sources to link to the OpenSearch application.</p> |
| `tag_list` | Vec<String> |  |  |
| `name` | String | ✅ | <p>The unique name of the OpenSearch application. Names must be unique within an Amazon Web Services Region for each account.</p> |
| `iam_identity_center_options` | String |  | <p>Configuration settings for integrating Amazon Web Services IAM Identity Center with the OpenSearch application.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_at` | String | <p>The timestamp when the OpenSearch application was created.</p> |
| `iam_identity_center_options` | String | <p>The IAM Identity Center settings configured for the OpenSearch application.</p> |
| `name` | String | <p>The name of the OpenSearch application.</p> |
| `id` | String | <p>The unique identifier of the OpenSearch application.</p> |
| `endpoint` | String | <p>The endpoint URL of the OpenSearch application.</p> |
| `arn` | String |  |
| `status` | String | <p>The current status of the OpenSearch application. Possible values: <code>CREATING</code>, <code>UPDATING</code>, <code>DELETING</code>, <code>FAILED</code>, <code>ACTIVE</code>, and <code>DELETED</code>.</p> |
| `app_configs` | Vec<String> | <p>The configuration settings of the OpenSearch application.</p> |
| `last_updated_at` | String | <p>The timestamp of the last update to the OpenSearch application.</p> |
| `data_sources` | Vec<String> | <p>The data sources associated with the OpenSearch application.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application
application = provider.opensearch.Application {
    name = "value"  # <p>The unique name of the OpenSearch application. Names must be unique within an Amazon Web Services Region for each account.</p>
}

# Access application outputs
application_id = application.id
application_created_at = application.created_at
application_iam_identity_center_options = application.iam_identity_center_options
application_name = application.name
application_id = application.id
application_endpoint = application.endpoint
application_arn = application.arn
application_status = application.status
application_app_configs = application.app_configs
application_last_updated_at = application.last_updated_at
application_data_sources = application.data_sources
```

---


### Compatible_versions

CompatibleVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `compatible_versions` | Vec<String> | <p>A map of OpenSearch or Elasticsearch versions and the versions you can upgrade them
   to.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access compatible_versions outputs
compatible_versions_id = compatible_versions.id
compatible_versions_compatible_versions = compatible_versions.compatible_versions
```

---


### Vpc_endpoints

VpcEndpoints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpc_endpoints` | Vec<String> | <p>Information about each requested VPC endpoint.</p> |
| `vpc_endpoint_errors` | Vec<String> | <p>Any errors associated with the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpc_endpoints outputs
vpc_endpoints_id = vpc_endpoints.id
vpc_endpoints_vpc_endpoints = vpc_endpoints.vpc_endpoints
vpc_endpoints_vpc_endpoint_errors = vpc_endpoints.vpc_endpoint_errors
```

---


### Package_scope

PackageScope resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `package_id` | String | ✅ | <p>ID of the package whose scope is being updated.</p> |
| `operation` | String | ✅ | <p> The operation to perform on the package scope (e.g., add/remove/override users).</p> |
| `package_user_list` | Vec<String> | ✅ | <p> List of users to be added or removed from the package scope.</p> |



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


### Domain_nodes

DomainNodes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_nodes_status_list` | Vec<String> | <p>Contains nodes information list <code>DomainNodesStatusList</code> with
   details about the all nodes on the requested domain.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access domain_nodes outputs
domain_nodes_id = domain_nodes.id
domain_nodes_domain_nodes_status_list = domain_nodes.domain_nodes_status_list
```

---


### Package_version_history

PackageVersionHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `package_version_history_list` | Vec<String> | <p>A list of package versions, along with their creation time and commit message.</p> |
| `package_id` | String | <p>The unique identifier of the package.</p> |
| `next_token` | String | <p>When <code>nextToken</code> is returned, there are more results available. The value of
    <code>nextToken</code> is a unique pagination token for each page. Send the request again using the
   returned token to retrieve the next page.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access package_version_history outputs
package_version_history_id = package_version_history.id
package_version_history_package_version_history_list = package_version_history.package_version_history_list
package_version_history_package_id = package_version_history.package_id
package_version_history_next_token = package_version_history.next_token
```

---


### Upgrade_history

UpgradeHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `upgrade_histories` | Vec<String> | <p>A list of objects corresponding to each upgrade or upgrade eligibility check performed on a
   domain.</p> |
| `next_token` | String | <p>When <code>nextToken</code> is returned, there are more results available. The value of
    <code>nextToken</code> is a unique pagination token for each page. Send the request again using the
   returned token to retrieve the next page.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access upgrade_history outputs
upgrade_history_id = upgrade_history.id
upgrade_history_upgrade_histories = upgrade_history.upgrade_histories
upgrade_history_next_token = upgrade_history.next_token
```

---


### Domain_maintenance_status

DomainMaintenanceStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `updated_at` | String | <p>The time at which the action was updated.</p> |
| `status` | String | <p>The status of the maintenance action.</p> |
| `status_message` | String | <p>The status message of the maintenance action.</p> |
| `node_id` | String | <p>The node ID of the maintenance action.</p> |
| `created_at` | String | <p>The time at which the action was created.</p> |
| `action` | String | <p>The action name.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access domain_maintenance_status outputs
domain_maintenance_status_id = domain_maintenance_status.id
domain_maintenance_status_updated_at = domain_maintenance_status.updated_at
domain_maintenance_status_status = domain_maintenance_status.status
domain_maintenance_status_status_message = domain_maintenance_status.status_message
domain_maintenance_status_node_id = domain_maintenance_status.node_id
domain_maintenance_status_created_at = domain_maintenance_status.created_at
domain_maintenance_status_action = domain_maintenance_status.action
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple domain_health resources
domain_health_0 = provider.opensearch.Domain_health {
}
domain_health_1 = provider.opensearch.Domain_health {
}
domain_health_2 = provider.opensearch.Domain_health {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    domain_health = provider.opensearch.Domain_health {
    }
```

---

## Related Documentation

- [AWS Opensearch Documentation](https://docs.aws.amazon.com/opensearch/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
