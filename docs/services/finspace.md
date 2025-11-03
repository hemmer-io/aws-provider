# Finspace Service



**Resources**: 14

---

## Overview

The finspace service provides access to 14 resource types:

- [Kx_dataview](#kx_dataview) [CRUD]
- [Kx_cluster_code_configuration](#kx_cluster_code_configuration) [U]
- [Kx_database](#kx_database) [CRUD]
- [Kx_environment_network](#kx_environment_network) [U]
- [Kx_scaling_group](#kx_scaling_group) [CRD]
- [Kx_cluster_node](#kx_cluster_node) [D]
- [Kx_cluster](#kx_cluster) [CRD]
- [Kx_cluster_databases](#kx_cluster_databases) [U]
- [Kx_user](#kx_user) [CRUD]
- [Environment](#environment) [CRUD]
- [Kx_changeset](#kx_changeset) [CR]
- [Kx_environment](#kx_environment) [CRUD]
- [Kx_volume](#kx_volume) [CRUD]
- [Kx_connection_string](#kx_connection_string) [R]

---

## Resources


### Kx_dataview

KxDataview resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `az_mode` | String | ✅ | <p>The number of availability zones you want to assign per volume. Currently, FinSpace only supports <code>SINGLE</code> for volumes. This places dataview in a single AZ.</p> |
| `segment_configurations` | Vec<String> |  | <p>
   The configuration that contains the database path of the data that you want to place on each selected volume. Each segment must have a unique database path for each volume. If you do not explicitly specify any database path for a volume, they are accessible from the cluster through the default S3/object store segment. </p> |
| `description` | String |  | <p>A description of the dataview.</p> |
| `tags` | HashMap<String, String> |  | <p>
A list of key-value pairs to label the dataview. You can add up to 50 tags to a dataview.
</p> |
| `database_name` | String | ✅ | <p>
The name of the database where you want to create a dataview.
</p> |
| `availability_zone_id` | String |  | <p>
         The identifier of the availability zones.
      </p> |
| `auto_update` | bool |  | <p>The option to specify whether you want to apply all the future additions and corrections automatically to the dataview, when you ingest new changesets. The default value is false.</p> |
| `client_token` | String | ✅ | <p>A token that ensures idempotency. This token expires in 10 minutes.</p> |
| `read_write` | bool |  | <p>
         The option to specify whether you want to make the dataview writable to perform database maintenance. The following are some considerations related to writable dataviews.  </p>
         <ul>
            <li>
               <p>You cannot create partial writable dataviews. When you create writeable dataviews you must
               provide the entire database path.</p>
            </li>
            <li>
               <p>You cannot perform updates on a writeable dataview. Hence, <code>autoUpdate</code> must be set
            as <b>False</b> if <code>readWrite</code> is <b>True</b> for a dataview.</p>
            </li>
            <li>
               <p>You must also use a unique volume for creating a writeable dataview. So, if you choose a
               volume that is already in use by another dataview, the dataview creation
               fails.</p>
            </li>
            <li>
               <p>Once you create a dataview as writeable, you cannot change it to read-only. So, you cannot
               update the <code>readWrite</code> parameter later.</p>
            </li>
         </ul> |
| `dataview_name` | String | ✅ | <p>A unique identifier for the dataview.</p> |
| `environment_id` | String | ✅ | <p>A unique identifier for the kdb environment, where you want to create the dataview. </p> |
| `changeset_id` | String |  | <p>
A unique identifier of the changeset that you want to use to ingest data. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `active_versions` | Vec<String> | <p>
   The current active changeset versions of the database on the given dataview. 
   
</p> |
| `created_timestamp` | String | <p>The timestamp at which the dataview was created in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p> |
| `status_reason` | String | <p>
The error message when a failed state occurs. 
</p> |
| `database_name` | String | <p>
The name of the database where you created the dataview.</p> |
| `last_modified_timestamp` | String | <p>
   The last time that the dataview was updated in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.
</p> |
| `segment_configurations` | Vec<String> | <p>
      The configuration that contains the database path of the data that you want to place on each selected volume. Each segment must have a unique database path for each volume. If you do not explicitly specify any database path for a volume, they are accessible from the cluster through the default S3/object store segment. </p> |
| `availability_zone_id` | String | <p>
         The identifier of the availability zones.
      </p> |
| `az_mode` | String | <p>The number of availability zones you want to assign per volume. Currently, FinSpace only supports <code>SINGLE</code> for volumes. This places dataview in a single AZ.</p> |
| `changeset_id` | String | <p>
A unique identifier of the changeset that you want to use to ingest data. </p> |
| `dataview_name` | String | <p>A unique identifier for the dataview.</p> |
| `read_write` | bool | <p>Returns True if the dataview is created as writeable and False otherwise. </p> |
| `auto_update` | bool | <p>The option to specify whether you want to apply all the future additions and corrections automatically to the dataview when new changesets are ingested. The default value is false.</p> |
| `description` | String | <p>A description of the dataview.</p> |
| `environment_id` | String | <p>A unique identifier for the kdb environment, from where you want to retrieve the dataview details.</p> |
| `status` | String | <p>
      The status of dataview creation.</p>
         <ul>
            <li>
               <p>
                  <code>CREATING</code> – The dataview creation is in progress.</p>
            </li>
            <li>
               <p>
                  <code>UPDATING</code> – The dataview is in the process of being updated.</p>
            </li>
            <li>
               <p>
                  <code>ACTIVE</code> – The dataview is active.</p>
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

# Create kx_dataview
kx_dataview = provider.finspace.Kx_dataview {
    az_mode = "value"  # <p>The number of availability zones you want to assign per volume. Currently, FinSpace only supports <code>SINGLE</code> for volumes. This places dataview in a single AZ.</p>
    database_name = "value"  # <p>
The name of the database where you want to create a dataview.
</p>
    client_token = "value"  # <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
    dataview_name = "value"  # <p>A unique identifier for the dataview.</p>
    environment_id = "value"  # <p>A unique identifier for the kdb environment, where you want to create the dataview. </p>
}

# Access kx_dataview outputs
kx_dataview_id = kx_dataview.id
kx_dataview_active_versions = kx_dataview.active_versions
kx_dataview_created_timestamp = kx_dataview.created_timestamp
kx_dataview_status_reason = kx_dataview.status_reason
kx_dataview_database_name = kx_dataview.database_name
kx_dataview_last_modified_timestamp = kx_dataview.last_modified_timestamp
kx_dataview_segment_configurations = kx_dataview.segment_configurations
kx_dataview_availability_zone_id = kx_dataview.availability_zone_id
kx_dataview_az_mode = kx_dataview.az_mode
kx_dataview_changeset_id = kx_dataview.changeset_id
kx_dataview_dataview_name = kx_dataview.dataview_name
kx_dataview_read_write = kx_dataview.read_write
kx_dataview_auto_update = kx_dataview.auto_update
kx_dataview_description = kx_dataview.description
kx_dataview_environment_id = kx_dataview.environment_id
kx_dataview_status = kx_dataview.status
```

---


### Kx_cluster_code_configuration

KxClusterCodeConfiguration resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deployment_configuration` | String |  | <p>
         The configuration that allows you to choose how you want to update the code on a cluster.
      </p> |
| `client_token` | String |  | <p>A token that ensures idempotency. This token expires in 10 minutes.</p> |
| `initialization_script` | String |  | <p>Specifies a Q program that will be run at launch of a cluster. It is a relative path within
         <i>.zip</i> file that contains the custom code, which will be loaded on
         the cluster. It must include the file name itself. For example,
         <code>somedir/init.q</code>.</p>
         <p>You cannot update this parameter for a <code>NO_RESTART</code> deployment.</p> |
| `command_line_arguments` | Vec<String> |  | <p>Specifies the key-value pairs to make them available inside the cluster.</p>
         <p>You cannot update this parameter for a <code>NO_RESTART</code> deployment.</p> |
| `environment_id` | String | ✅ | <p>
         A unique identifier of the kdb environment.
      </p> |
| `cluster_name` | String | ✅ | <p>The name of the cluster.</p> |
| `code` | String | ✅ |  |



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


### Kx_database

KxDatabase resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description of the database.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs to label the kdb database. You can add up to 50 tags to your kdb database</p> |
| `client_token` | String | ✅ | <p>A token that ensures idempotency. This token expires in 10 minutes.</p> |
| `environment_id` | String | ✅ | <p>A unique identifier for the kdb environment.</p> |
| `database_name` | String | ✅ | <p>The name of the kdb database.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>A description of the database.</p> |
| `num_bytes` | i64 | <p>The total number of bytes in the database.</p> |
| `num_changesets` | i64 | <p>The total number of changesets in the database.</p> |
| `last_modified_timestamp` | String | <p>The last time that the database was modified. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p> |
| `database_arn` | String | <p>The ARN identifier of the database.</p> |
| `database_name` | String | <p>The name of the kdb database for which the information is retrieved.</p> |
| `environment_id` | String | <p>A unique identifier for the kdb environment.</p> |
| `created_timestamp` | String | <p>The timestamp at which the database is created in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p> |
| `last_completed_changeset_id` | String | <p>A unique identifier for the changeset.</p> |
| `num_files` | i64 | <p>The total number of files in the database.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create kx_database
kx_database = provider.finspace.Kx_database {
    client_token = "value"  # <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
    environment_id = "value"  # <p>A unique identifier for the kdb environment.</p>
    database_name = "value"  # <p>The name of the kdb database.</p>
}

# Access kx_database outputs
kx_database_id = kx_database.id
kx_database_description = kx_database.description
kx_database_num_bytes = kx_database.num_bytes
kx_database_num_changesets = kx_database.num_changesets
kx_database_last_modified_timestamp = kx_database.last_modified_timestamp
kx_database_database_arn = kx_database.database_arn
kx_database_database_name = kx_database.database_name
kx_database_environment_id = kx_database.environment_id
kx_database_created_timestamp = kx_database.created_timestamp
kx_database_last_completed_changeset_id = kx_database.last_completed_changeset_id
kx_database_num_files = kx_database.num_files
```

---


### Kx_environment_network

KxEnvironmentNetwork resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `custom_dns_configuration` | Vec<String> |  | <p>A list of DNS server name and server IP. This is used to set up Route-53 outbound resolvers.</p> |
| `environment_id` | String | ✅ | <p>A unique identifier for the kdb environment.</p> |
| `client_token` | String |  | <p>A token that ensures idempotency. This token expires in 10 minutes.</p> |
| `transit_gateway_configuration` | String |  | <p>Specifies the transit gateway and network configuration to connect the kdb environment to an internal network.</p> |



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


### Kx_scaling_group

KxScalingGroup resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `host_type` | String | ✅ | <p>
   The memory and CPU capabilities of the scaling group host on which FinSpace Managed kdb clusters will be placed.</p>
         <p>You can add one of the following values:</p>
         <ul>
            <li>
               <p>
                  <code>kx.sg.large</code> – The host type with a configuration of 16 GiB
            memory and 2 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg.xlarge</code> – The host type with a configuration of 32 GiB
               memory and 4 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg.2xlarge</code> – The host type with a configuration of 64 GiB
               memory and 8 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg.4xlarge</code> – The host type with a configuration of 108 GiB memory and 16 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg.8xlarge</code> – The host type with a configuration of 216 GiB memory and 32 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg.16xlarge</code> – The host type with a configuration of 432 GiB memory and 64 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg.32xlarge</code> – The host type with a configuration of 864 GiB memory and 128 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg1.16xlarge</code> – The host type with a configuration of 1949 GiB memory and 64 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg1.24xlarge</code> – The host type with a configuration of 2948 GiB memory and 96 vCPUs.</p>
            </li>
         </ul> |
| `environment_id` | String | ✅ | <p>A unique identifier for the kdb environment, where you want to create the scaling group. </p> |
| `client_token` | String | ✅ | <p>A token that ensures idempotency. This token expires in 10 minutes.</p> |
| `availability_zone_id` | String | ✅ | <p>The identifier of the availability zones.</p> |
| `scaling_group_name` | String | ✅ | <p>A unique identifier for the kdb scaling group. </p> |
| `tags` | HashMap<String, String> |  | <p>
A list of key-value pairs to label the scaling group. You can add up to 50 tags to a scaling group.
</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scaling_group_arn` | String | <p>
        The ARN identifier for the scaling group.
      </p> |
| `availability_zone_id` | String | <p>The identifier of the availability zones.</p> |
| `status` | String | <p>The status of scaling group.</p>
         <ul>
            <li>
               <p>CREATING – The scaling group creation is in progress.</p>
            </li>
            <li>
               <p>CREATE_FAILED – The scaling group creation has failed.</p>
            </li>
            <li>
               <p>ACTIVE – The scaling group is active.</p>
            </li>
            <li>
               <p>UPDATING – The scaling group is in the process of being updated.</p>
            </li>
            <li>
               <p>UPDATE_FAILED – The update action failed.</p>
            </li>
            <li>
               <p>DELETING – The scaling group is in the process of being deleted.</p>
            </li>
            <li>
               <p>DELETE_FAILED – The system failed to delete the scaling group.</p>
            </li>
            <li>
               <p>DELETED – The scaling group is successfully deleted.</p>
            </li>
         </ul> |
| `scaling_group_name` | String | <p>A unique identifier for the kdb scaling group. </p> |
| `last_modified_timestamp` | String | <p>
   The last time that the scaling group was updated in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.
</p> |
| `clusters` | Vec<String> | <p>
   The list of Managed kdb clusters that are currently active in the given scaling group.
   
</p> |
| `host_type` | String | <p>
      The memory and CPU capabilities of the scaling group host on which FinSpace Managed kdb clusters will be placed.</p>
         <p>It can have one of the following values:</p>
         <ul>
            <li>
               <p>
                  <code>kx.sg.large</code> – The host type with a configuration of 16 GiB
            memory and 2 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg.xlarge</code> – The host type with a configuration of 32 GiB
               memory and 4 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg.2xlarge</code> – The host type with a configuration of 64 GiB
               memory and 8 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg.4xlarge</code> – The host type with a configuration of 108 GiB memory and 16 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg.8xlarge</code> – The host type with a configuration of 216 GiB memory and 32 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg.16xlarge</code> – The host type with a configuration of 432 GiB memory and 64 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg.32xlarge</code> – The host type with a configuration of 864 GiB memory and 128 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg1.16xlarge</code> – The host type with a configuration of 1949 GiB memory and 64 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg1.24xlarge</code> – The host type with a configuration of 2948 GiB memory and 96 vCPUs.</p>
            </li>
         </ul> |
| `status_reason` | String | <p>
The error message when a failed state occurs. 
</p> |
| `created_timestamp` | String | <p>
   The timestamp at which the scaling group was created in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create kx_scaling_group
kx_scaling_group = provider.finspace.Kx_scaling_group {
    host_type = "value"  # <p>
   The memory and CPU capabilities of the scaling group host on which FinSpace Managed kdb clusters will be placed.</p>
         <p>You can add one of the following values:</p>
         <ul>
            <li>
               <p>
                  <code>kx.sg.large</code> – The host type with a configuration of 16 GiB
            memory and 2 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg.xlarge</code> – The host type with a configuration of 32 GiB
               memory and 4 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg.2xlarge</code> – The host type with a configuration of 64 GiB
               memory and 8 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg.4xlarge</code> – The host type with a configuration of 108 GiB memory and 16 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg.8xlarge</code> – The host type with a configuration of 216 GiB memory and 32 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg.16xlarge</code> – The host type with a configuration of 432 GiB memory and 64 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg.32xlarge</code> – The host type with a configuration of 864 GiB memory and 128 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg1.16xlarge</code> – The host type with a configuration of 1949 GiB memory and 64 vCPUs.</p>
            </li>
            <li>
               <p>
                  <code>kx.sg1.24xlarge</code> – The host type with a configuration of 2948 GiB memory and 96 vCPUs.</p>
            </li>
         </ul>
    environment_id = "value"  # <p>A unique identifier for the kdb environment, where you want to create the scaling group. </p>
    client_token = "value"  # <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
    availability_zone_id = "value"  # <p>The identifier of the availability zones.</p>
    scaling_group_name = "value"  # <p>A unique identifier for the kdb scaling group. </p>
}

# Access kx_scaling_group outputs
kx_scaling_group_id = kx_scaling_group.id
kx_scaling_group_scaling_group_arn = kx_scaling_group.scaling_group_arn
kx_scaling_group_availability_zone_id = kx_scaling_group.availability_zone_id
kx_scaling_group_status = kx_scaling_group.status
kx_scaling_group_scaling_group_name = kx_scaling_group.scaling_group_name
kx_scaling_group_last_modified_timestamp = kx_scaling_group.last_modified_timestamp
kx_scaling_group_clusters = kx_scaling_group.clusters
kx_scaling_group_host_type = kx_scaling_group.host_type
kx_scaling_group_status_reason = kx_scaling_group.status_reason
kx_scaling_group_created_timestamp = kx_scaling_group.created_timestamp
```

---


### Kx_cluster_node

KxClusterNode resource

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


### Kx_cluster

KxCluster resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auto_scaling_configuration` | String |  | <p>The configuration based on which FinSpace will scale in or scale out nodes in your cluster.</p> |
| `availability_zone_id` | String |  | <p>The availability zone identifiers for the requested regions.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs to label the cluster. You can add up to 50 tags to a cluster.</p> |
| `cache_storage_configurations` | Vec<String> |  | <p>The configurations for a read only cache storage associated with a cluster. This cache will be stored as an FSx Lustre that reads from the S3 store. </p> |
| `cluster_type` | String | ✅ | <p>Specifies the type of KDB database that is being created. The following types are available: </p>
         <ul>
            <li>
               <p>HDB – A Historical Database. The data is only accessible with read-only permissions from one of the FinSpace managed kdb databases mounted to the cluster.</p>
            </li>
            <li>
               <p>RDB – A Realtime Database. This type of database captures all the data from a ticker plant and stores it in memory until the end of day, after which it writes all of its data to a disk and reloads the HDB. This cluster type requires local storage for temporary storage of data during the savedown process. If you specify this field in your request, you must provide the <code>savedownStorageConfiguration</code> parameter.</p>
            </li>
            <li>
               <p>GATEWAY – A gateway cluster allows you to access data across processes in kdb systems. It allows you to create your own routing logic using the initialization scripts and custom code. This type of cluster does not require a  writable local storage.</p>
            </li>
            <li>
               <p>GP – A general purpose cluster allows you to quickly iterate on code during development by granting greater access to system commands and enabling a fast reload of custom code. This cluster type can optionally mount databases including cache and savedown storage. For this cluster type, the node count is fixed at 1. It does not support autoscaling and supports only <code>SINGLE</code> AZ mode.</p>
            </li>
            <li>
               <p>Tickerplant – A tickerplant cluster allows you to subscribe to feed handlers based on IAM permissions. It can publish to RDBs, other Tickerplants, and real-time subscribers (RTS). Tickerplants can persist messages to log, which is readable by any RDB environment. It supports only single-node that is only one kdb process.</p>
            </li>
         </ul> |
| `az_mode` | String | ✅ | <p>The number of availability zones you want to assign per cluster. This can be one of the following </p>
         <ul>
            <li>
               <p>
                  <code>SINGLE</code> – Assigns one availability zone per cluster.</p>
            </li>
            <li>
               <p>
                  <code>MULTI</code> – Assigns all the availability zones per cluster.</p>
            </li>
         </ul> |
| `cluster_description` | String |  | <p>A description of the cluster.</p> |
| `release_label` | String | ✅ | <p>The version of FinSpace managed kdb to run.</p> |
| `cluster_name` | String | ✅ | <p>A unique name for the cluster that you want to create.</p> |
| `client_token` | String |  | <p>A token that ensures idempotency. This token expires in 10 minutes.</p> |
| `capacity_configuration` | String |  | <p>A structure for the metadata of a cluster. It includes information like the CPUs needed, memory of instances, and number of instances.</p> |
| `vpc_configuration` | String | ✅ | <p>Configuration details about the network where the Privatelink endpoint of the cluster resides.</p> |
| `scaling_group_configuration` | String |  | <p>The structure that stores the configuration details of a scaling group.</p> |
| `command_line_arguments` | Vec<String> |  | <p>Defines the key-value pairs to make them available inside the cluster.</p> |
| `environment_id` | String | ✅ | <p>A unique identifier for the kdb environment.</p> |
| `databases` | Vec<String> |  | <p>A list of databases that will be available for querying.</p> |
| `tickerplant_log_configuration` | String |  | <p>
A configuration to store Tickerplant logs. It consists of
a list of volumes that will be mounted to your cluster. For the cluster type <code>Tickerplant</code>, the location of the TP volume on the cluster will be available by using the global variable <code>.aws.tp_log_path</code>. 
</p> |
| `code` | String |  | <p>The details of the custom code that you want to use inside a cluster when analyzing a data. It consists of the S3 source bucket, location, S3 object version, and the relative path from where the custom code is loaded into the cluster. </p> |
| `execution_role` | String |  | <p>An IAM role that defines a set of permissions associated with a cluster. These permissions are assumed when a cluster attempts to access another cluster.</p> |
| `initialization_script` | String |  | <p>Specifies a Q program that will be run at launch of a cluster. It is a relative path within
            <i>.zip</i> file that contains the custom code, which will be loaded on
         the cluster. It must include the file name itself. For example,
         <code>somedir/init.q</code>.</p> |
| `savedown_storage_configuration` | String |  | <p>The size and type of the temporary storage that is used to hold data during the savedown process. This parameter is required when you choose <code>clusterType</code> as RDB. All the data written to this storage space is lost when the cluster node is restarted.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `az_mode` | String | <p>The number of availability zones you want to assign per cluster. This can be one of the following </p>
         <ul>
            <li>
               <p>
                  <code>SINGLE</code> – Assigns one availability zone per cluster.</p>
            </li>
            <li>
               <p>
                  <code>MULTI</code> – Assigns all the availability zones per cluster.</p>
            </li>
         </ul> |
| `command_line_arguments` | Vec<String> | <p>Defines key-value pairs to make them available inside the cluster.</p> |
| `cache_storage_configurations` | Vec<String> | <p>The configurations for a read only cache storage associated with a cluster. This cache will be stored as an FSx Lustre that reads from the S3 store. </p> |
| `status` | String | <p>The status of cluster creation.</p>
         <ul>
            <li>
               <p>PENDING – The cluster is pending creation.</p>
            </li>
            <li>
               <p>CREATING – The cluster creation process is in progress.</p>
            </li>
            <li>
               <p>CREATE_FAILED – The cluster creation process has failed.</p>
            </li>
            <li>
               <p>RUNNING – The cluster creation process is running.</p>
            </li>
            <li>
               <p>UPDATING – The cluster is in the process of being updated.</p>
            </li>
            <li>
               <p>DELETING – The cluster is in the process of being deleted.</p>
            </li>
            <li>
               <p>DELETED – The cluster has been deleted.</p>
            </li>
            <li>
               <p>DELETE_FAILED – The cluster failed to delete.</p>
            </li>
         </ul> |
| `cluster_type` | String | <p>Specifies the type of KDB database that is being created. The following types are available: </p>
         <ul>
            <li>
               <p>HDB – A Historical Database. The data is only accessible with read-only permissions from one of the FinSpace managed kdb databases mounted to the cluster.</p>
            </li>
            <li>
               <p>RDB – A Realtime Database. This type of database captures all the data from a ticker plant and stores it in memory until the end of day, after which it writes all of its data to a disk and reloads the HDB. This cluster type requires local storage for temporary storage of data during the savedown process. If you specify this field in your request, you must provide the <code>savedownStorageConfiguration</code> parameter.</p>
            </li>
            <li>
               <p>GATEWAY – A gateway cluster allows you to access data across processes in kdb systems. It allows you to create your own routing logic using the initialization scripts and custom code. This type of cluster does not require a  writable local storage.</p>
            </li>
            <li>
               <p>GP – A general purpose cluster allows you to quickly iterate on code during development by granting greater access to system commands and enabling a fast reload of custom code. This cluster type can optionally mount databases including cache and savedown storage. For this cluster type, the node count is fixed at 1. It does not support autoscaling and supports only <code>SINGLE</code> AZ mode.</p>
            </li>
            <li>
               <p>Tickerplant – A tickerplant cluster allows you to subscribe to feed handlers based on IAM permissions. It can publish to RDBs, other Tickerplants, and real-time subscribers (RTS). Tickerplants can persist messages to log, which is readable by any RDB environment. It supports only single-node that is only one kdb process.</p>
            </li>
         </ul> |
| `cluster_description` | String | <p>A description of the cluster.</p> |
| `last_modified_timestamp` | String | <p>The last time that the cluster was modified. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p> |
| `cluster_name` | String | <p>A unique name for the cluster.</p> |
| `vpc_configuration` | String | <p>Configuration details about the network where the Privatelink endpoint of the cluster resides.</p> |
| `release_label` | String | <p>The version of FinSpace managed kdb to run.</p> |
| `tickerplant_log_configuration` | String |  |
| `initialization_script` | String | <p>Specifies a Q program that will be run at launch of a cluster. It is a relative path within
      <i>.zip</i> file that contains the custom code, which will be loaded on
      the cluster. It must include the file name itself. For example,
      <code>somedir/init.q</code>.</p> |
| `status_reason` | String | <p>The error message when a failed state occurs. </p> |
| `databases` | Vec<String> | <p> A list of databases mounted on the cluster.</p> |
| `auto_scaling_configuration` | String | <p>The configuration based on which FinSpace will scale in or scale out nodes in your cluster.</p> |
| `capacity_configuration` | String | <p>A structure for the metadata of a cluster. It includes information like the CPUs needed, memory of instances, and number of instances.</p> |
| `code` | String | <p>The details of the custom code that you want to use inside a cluster when analyzing a data. It consists of the S3 source bucket, location, S3 object version, and the relative path from where the custom code is loaded into the cluster. </p> |
| `execution_role` | String | <p>
            An IAM role that defines a set of permissions associated with a cluster. These permissions are assumed when a cluster attempts to access another cluster.
         </p> |
| `savedown_storage_configuration` | String | <p>The size and type of the temporary storage that is used to hold data during the savedown process. This parameter is required when you choose <code>clusterType</code> as RDB. All the data written to this storage space is lost when the cluster node is restarted.</p> |
| `availability_zone_id` | String | <p>
            The availability zone identifiers for the requested regions.
         </p> |
| `created_timestamp` | String | <p>The timestamp at which the cluster was created in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p> |
| `scaling_group_configuration` | String |  |
| `volumes` | Vec<String> | <p>
A list of volumes attached to the cluster.
</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create kx_cluster
kx_cluster = provider.finspace.Kx_cluster {
    cluster_type = "value"  # <p>Specifies the type of KDB database that is being created. The following types are available: </p>
         <ul>
            <li>
               <p>HDB – A Historical Database. The data is only accessible with read-only permissions from one of the FinSpace managed kdb databases mounted to the cluster.</p>
            </li>
            <li>
               <p>RDB – A Realtime Database. This type of database captures all the data from a ticker plant and stores it in memory until the end of day, after which it writes all of its data to a disk and reloads the HDB. This cluster type requires local storage for temporary storage of data during the savedown process. If you specify this field in your request, you must provide the <code>savedownStorageConfiguration</code> parameter.</p>
            </li>
            <li>
               <p>GATEWAY – A gateway cluster allows you to access data across processes in kdb systems. It allows you to create your own routing logic using the initialization scripts and custom code. This type of cluster does not require a  writable local storage.</p>
            </li>
            <li>
               <p>GP – A general purpose cluster allows you to quickly iterate on code during development by granting greater access to system commands and enabling a fast reload of custom code. This cluster type can optionally mount databases including cache and savedown storage. For this cluster type, the node count is fixed at 1. It does not support autoscaling and supports only <code>SINGLE</code> AZ mode.</p>
            </li>
            <li>
               <p>Tickerplant – A tickerplant cluster allows you to subscribe to feed handlers based on IAM permissions. It can publish to RDBs, other Tickerplants, and real-time subscribers (RTS). Tickerplants can persist messages to log, which is readable by any RDB environment. It supports only single-node that is only one kdb process.</p>
            </li>
         </ul>
    az_mode = "value"  # <p>The number of availability zones you want to assign per cluster. This can be one of the following </p>
         <ul>
            <li>
               <p>
                  <code>SINGLE</code> – Assigns one availability zone per cluster.</p>
            </li>
            <li>
               <p>
                  <code>MULTI</code> – Assigns all the availability zones per cluster.</p>
            </li>
         </ul>
    release_label = "value"  # <p>The version of FinSpace managed kdb to run.</p>
    cluster_name = "value"  # <p>A unique name for the cluster that you want to create.</p>
    vpc_configuration = "value"  # <p>Configuration details about the network where the Privatelink endpoint of the cluster resides.</p>
    environment_id = "value"  # <p>A unique identifier for the kdb environment.</p>
}

# Access kx_cluster outputs
kx_cluster_id = kx_cluster.id
kx_cluster_az_mode = kx_cluster.az_mode
kx_cluster_command_line_arguments = kx_cluster.command_line_arguments
kx_cluster_cache_storage_configurations = kx_cluster.cache_storage_configurations
kx_cluster_status = kx_cluster.status
kx_cluster_cluster_type = kx_cluster.cluster_type
kx_cluster_cluster_description = kx_cluster.cluster_description
kx_cluster_last_modified_timestamp = kx_cluster.last_modified_timestamp
kx_cluster_cluster_name = kx_cluster.cluster_name
kx_cluster_vpc_configuration = kx_cluster.vpc_configuration
kx_cluster_release_label = kx_cluster.release_label
kx_cluster_tickerplant_log_configuration = kx_cluster.tickerplant_log_configuration
kx_cluster_initialization_script = kx_cluster.initialization_script
kx_cluster_status_reason = kx_cluster.status_reason
kx_cluster_databases = kx_cluster.databases
kx_cluster_auto_scaling_configuration = kx_cluster.auto_scaling_configuration
kx_cluster_capacity_configuration = kx_cluster.capacity_configuration
kx_cluster_code = kx_cluster.code
kx_cluster_execution_role = kx_cluster.execution_role
kx_cluster_savedown_storage_configuration = kx_cluster.savedown_storage_configuration
kx_cluster_availability_zone_id = kx_cluster.availability_zone_id
kx_cluster_created_timestamp = kx_cluster.created_timestamp
kx_cluster_scaling_group_configuration = kx_cluster.scaling_group_configuration
kx_cluster_volumes = kx_cluster.volumes
```

---


### Kx_cluster_databases

KxClusterDatabases resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deployment_configuration` | String |  | <p>
         The configuration that allows you to choose how you want to update the databases on a cluster.
      </p> |
| `environment_id` | String | ✅ | <p>The unique identifier of a kdb environment.</p> |
| `databases` | Vec<String> | ✅ | <p> The structure of databases mounted on the cluster.</p> |
| `cluster_name` | String | ✅ | <p>A unique name for the cluster that you want to modify.</p> |
| `client_token` | String |  | <p>A token that ensures idempotency. This token expires in 10 minutes.</p> |



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


### Kx_user

KxUser resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_name` | String | ✅ | <p>A unique identifier for the user.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs to label the user. You can add up to 50 tags to a user.</p> |
| `iam_role` | String | ✅ | <p>The IAM role ARN that will be associated with the user.</p> |
| `client_token` | String |  | <p>A token that ensures idempotency. This token expires in 10 minutes.</p> |
| `environment_id` | String | ✅ | <p>A unique identifier for the kdb environment where you want to create a user.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_arn` | String | <p> The Amazon Resource Name (ARN) that identifies the user. For more information about ARNs and
      how to use ARNs in policies, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM Identifiers</a> in the
      <i>IAM User Guide</i>. </p> |
| `environment_id` | String | <p>A unique identifier for the kdb environment.</p> |
| `iam_role` | String | <p>The IAM role ARN that is associated with the user.</p> |
| `user_name` | String | <p>A unique identifier for the user.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create kx_user
kx_user = provider.finspace.Kx_user {
    user_name = "value"  # <p>A unique identifier for the user.</p>
    iam_role = "value"  # <p>The IAM role ARN that will be associated with the user.</p>
    environment_id = "value"  # <p>A unique identifier for the kdb environment where you want to create a user.</p>
}

# Access kx_user outputs
kx_user_id = kx_user.id
kx_user_user_arn = kx_user.user_arn
kx_user_environment_id = kx_user.environment_id
kx_user_iam_role = kx_user.iam_role
kx_user_user_name = kx_user.user_name
```

---


### Environment

Environment resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `federation_mode` | String |  | <p>Authentication mode for the environment.</p>
         <ul>
            <li>
               <p>
                  <code>FEDERATED</code> - Users access FinSpace through Single Sign On (SSO) via your Identity provider.</p>
            </li>
            <li>
               <p>
                  <code>LOCAL</code> - Users access FinSpace via email and password managed within the FinSpace environment.</p>
            </li>
         </ul> |
| `tags` | HashMap<String, String> |  | <p>Add tags to your FinSpace environment.</p> |
| `federation_parameters` | String |  | <p>Configuration information when authentication mode is FEDERATED.</p> |
| `description` | String |  | <p>The description of the FinSpace environment to be created.</p> |
| `name` | String | ✅ | <p>The name of the FinSpace environment to be created.</p> |
| `kms_key_id` | String |  | <p>The KMS key id to encrypt your data in the FinSpace environment.</p> |
| `superuser_parameters` | String |  | <p>Configuration information for the superuser.</p> |
| `data_bundles` | Vec<String> |  | <p>The list of Amazon Resource Names (ARN) of the data bundles to install. Currently supported data bundle ARNs:</p>
         <ul>
            <li>
               <p>
                  <code>arn:aws:finspace:${Region}::data-bundle/capital-markets-sample</code> - Contains sample Capital Markets datasets, categories and controlled vocabularies.</p>
            </li>
            <li>
               <p>
                  <code>arn:aws:finspace:${Region}::data-bundle/taq</code> (default) - Contains trades and quotes data in addition to sample Capital Markets data.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `environment` | String | <p>The name of the FinSpace environment.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create environment
environment = provider.finspace.Environment {
    name = "value"  # <p>The name of the FinSpace environment to be created.</p>
}

# Access environment outputs
environment_id = environment.id
environment_environment = environment.environment
```

---


### Kx_changeset

KxChangeset resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `database_name` | String | ✅ | <p>The name of the kdb database.</p> |
| `change_requests` | Vec<String> | ✅ | <p>A list of change request objects that are run in order. A change request object consists of <code>changeType</code> , <code>s3Path</code>, and <code>dbPath</code>. 
      A changeType can have the following values: </p>
         <ul>
            <li>
               <p>PUT – Adds or updates files in a database.</p>
            </li>
            <li>
               <p>DELETE – Deletes files in a database.</p>
            </li>
         </ul>
         <p>All the change requests require a mandatory <code>dbPath</code> attribute that defines the
         path within the database directory. All database paths must start with a leading / and end
         with a trailing /. The <code>s3Path</code> attribute defines the s3 source file path and is
         required for a PUT change type. The <code>s3path</code> must end with a trailing / if it is
         a directory and must end without a trailing / if it is a file. </p>
         <p>Here are few examples of how you can use the change request object:</p>
         <ol>
            <li>
               <p>This request adds a single sym file at database root location.   </p>
               <p>
                  <code>{ "changeType": "PUT", "s3Path":"s3://bucket/db/sym",
               "dbPath":"/"}</code>
               </p>
            </li>
            <li>
               <p>This request adds files in the given <code>s3Path</code> under the 2020.01.02
               partition of the database.</p>
               <p>
                  <code>{ "changeType": "PUT", "s3Path":"s3://bucket/db/2020.01.02/",
               "dbPath":"/2020.01.02/"}</code>
               </p>
            </li>
            <li>
               <p>This request adds files in the given <code>s3Path</code> under the
                  <i>taq</i> table partition of the database.</p>
               <p>
                  <code>[ { "changeType": "PUT", "s3Path":"s3://bucket/db/2020.01.02/taq/",
                  "dbPath":"/2020.01.02/taq/"}]</code>
               </p>
            </li>
            <li>
               <p>This request deletes the 2020.01.02 partition of the database.</p>
               <p>
                  <code>[{ "changeType": "DELETE", "dbPath": "/2020.01.02/"} ]</code>
               </p>
            </li>
            <li>
               <p>The <i>DELETE</i> request allows you to delete the existing files under the
               2020.01.02 partition of the database, and the <i>PUT</i> request adds a
               new taq table under it.</p>
               <p>
                  <code>[ {"changeType": "DELETE", "dbPath":"/2020.01.02/"}, {"changeType": "PUT",
                  "s3Path":"s3://bucket/db/2020.01.02/taq/",
               "dbPath":"/2020.01.02/taq/"}]</code>
               </p>
            </li>
         </ol> |
| `environment_id` | String | ✅ | <p>A unique identifier of the kdb environment.</p> |
| `client_token` | String | ✅ | <p>A token that ensures idempotency. This token expires in 10 minutes.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>Status of the changeset creation process.</p>
         <ul>
            <li>
               <p>Pending – Changeset creation is pending.</p>
            </li>
            <li>
               <p>Processing – Changeset creation is running.</p>
            </li>
            <li>
               <p>Failed – Changeset creation has failed.</p>
            </li>
            <li>
               <p>Complete – Changeset creation has succeeded.</p>
            </li>
         </ul> |
| `change_requests` | Vec<String> | <p>A list of change request objects that are run in order.</p> |
| `active_from_timestamp` | String | <p>Beginning time from which the changeset is active. The value is determined as epoch time in
      milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as
      1635768000000.</p> |
| `database_name` | String | <p>The name of the kdb database.</p> |
| `changeset_id` | String | <p>A unique identifier for the changeset.</p> |
| `created_timestamp` | String | <p>The timestamp at which the changeset was created in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p> |
| `error_info` | String | <p>Provides details in the event of a failed flow, including the error type and the related error message.</p> |
| `environment_id` | String | <p>A unique identifier for the kdb environment.</p> |
| `last_modified_timestamp` | String | <p>The timestamp at which the changeset was updated in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create kx_changeset
kx_changeset = provider.finspace.Kx_changeset {
    database_name = "value"  # <p>The name of the kdb database.</p>
    change_requests = "value"  # <p>A list of change request objects that are run in order. A change request object consists of <code>changeType</code> , <code>s3Path</code>, and <code>dbPath</code>. 
      A changeType can have the following values: </p>
         <ul>
            <li>
               <p>PUT – Adds or updates files in a database.</p>
            </li>
            <li>
               <p>DELETE – Deletes files in a database.</p>
            </li>
         </ul>
         <p>All the change requests require a mandatory <code>dbPath</code> attribute that defines the
         path within the database directory. All database paths must start with a leading / and end
         with a trailing /. The <code>s3Path</code> attribute defines the s3 source file path and is
         required for a PUT change type. The <code>s3path</code> must end with a trailing / if it is
         a directory and must end without a trailing / if it is a file. </p>
         <p>Here are few examples of how you can use the change request object:</p>
         <ol>
            <li>
               <p>This request adds a single sym file at database root location.   </p>
               <p>
                  <code>{ "changeType": "PUT", "s3Path":"s3://bucket/db/sym",
               "dbPath":"/"}</code>
               </p>
            </li>
            <li>
               <p>This request adds files in the given <code>s3Path</code> under the 2020.01.02
               partition of the database.</p>
               <p>
                  <code>{ "changeType": "PUT", "s3Path":"s3://bucket/db/2020.01.02/",
               "dbPath":"/2020.01.02/"}</code>
               </p>
            </li>
            <li>
               <p>This request adds files in the given <code>s3Path</code> under the
                  <i>taq</i> table partition of the database.</p>
               <p>
                  <code>[ { "changeType": "PUT", "s3Path":"s3://bucket/db/2020.01.02/taq/",
                  "dbPath":"/2020.01.02/taq/"}]</code>
               </p>
            </li>
            <li>
               <p>This request deletes the 2020.01.02 partition of the database.</p>
               <p>
                  <code>[{ "changeType": "DELETE", "dbPath": "/2020.01.02/"} ]</code>
               </p>
            </li>
            <li>
               <p>The <i>DELETE</i> request allows you to delete the existing files under the
               2020.01.02 partition of the database, and the <i>PUT</i> request adds a
               new taq table under it.</p>
               <p>
                  <code>[ {"changeType": "DELETE", "dbPath":"/2020.01.02/"}, {"changeType": "PUT",
                  "s3Path":"s3://bucket/db/2020.01.02/taq/",
               "dbPath":"/2020.01.02/taq/"}]</code>
               </p>
            </li>
         </ol>
    environment_id = "value"  # <p>A unique identifier of the kdb environment.</p>
    client_token = "value"  # <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
}

# Access kx_changeset outputs
kx_changeset_id = kx_changeset.id
kx_changeset_status = kx_changeset.status
kx_changeset_change_requests = kx_changeset.change_requests
kx_changeset_active_from_timestamp = kx_changeset.active_from_timestamp
kx_changeset_database_name = kx_changeset.database_name
kx_changeset_changeset_id = kx_changeset.changeset_id
kx_changeset_created_timestamp = kx_changeset.created_timestamp
kx_changeset_error_info = kx_changeset.error_info
kx_changeset_environment_id = kx_changeset.environment_id
kx_changeset_last_modified_timestamp = kx_changeset.last_modified_timestamp
```

---


### Kx_environment

KxEnvironment resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description for the kdb environment.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs to label the kdb environment. You can add up to 50 tags to your kdb environment.</p> |
| `name` | String | ✅ | <p>The name of the kdb environment that you want to create.</p> |
| `kms_key_id` | String | ✅ | <p>The KMS key ID to encrypt your data in the FinSpace environment.</p> |
| `client_token` | String |  | <p>A token that ensures idempotency. This token expires in 10 minutes.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `custom_dns_configuration` | Vec<String> | <p>A list of DNS server name and server IP. This is used to set up Route-53 outbound resolvers.</p> |
| `dns_status` | String | <p>The status of DNS configuration.</p> |
| `creation_timestamp` | String | <p>The timestamp at which the kdb environment was created in FinSpace. </p> |
| `environment_arn` | String | <p>The ARN identifier of the environment.</p> |
| `update_timestamp` | String | <p>The timestamp at which the kdb environment was updated. </p> |
| `dedicated_service_account_id` | String | <p>A unique identifier for the AWS environment infrastructure account.</p> |
| `kms_key_id` | String | <p>The KMS key ID to encrypt your data in the FinSpace environment.</p> |
| `availability_zone_ids` | Vec<String> | <p>The identifier of the availability zones where subnets for the environment are created.</p> |
| `description` | String | <p>A description for the kdb environment.</p> |
| `name` | String | <p>The name of the kdb environment.</p> |
| `tgw_status` | String | <p>The status of the network configuration.</p> |
| `transit_gateway_configuration` | String |  |
| `status` | String | <p>The status of the kdb environment.</p> |
| `aws_account_id` | String | <p>The unique identifier of the AWS account that is used to create the kdb environment.</p> |
| `error_message` | String | <p>Specifies the error message that appears if a flow fails.</p> |
| `certificate_authority_arn` | String | <p>The Amazon Resource Name (ARN) of the certificate authority of the 
         kdb environment.</p> |
| `environment_id` | String | <p>A unique identifier for the kdb environment.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create kx_environment
kx_environment = provider.finspace.Kx_environment {
    name = "value"  # <p>The name of the kdb environment that you want to create.</p>
    kms_key_id = "value"  # <p>The KMS key ID to encrypt your data in the FinSpace environment.</p>
}

# Access kx_environment outputs
kx_environment_id = kx_environment.id
kx_environment_custom_dns_configuration = kx_environment.custom_dns_configuration
kx_environment_dns_status = kx_environment.dns_status
kx_environment_creation_timestamp = kx_environment.creation_timestamp
kx_environment_environment_arn = kx_environment.environment_arn
kx_environment_update_timestamp = kx_environment.update_timestamp
kx_environment_dedicated_service_account_id = kx_environment.dedicated_service_account_id
kx_environment_kms_key_id = kx_environment.kms_key_id
kx_environment_availability_zone_ids = kx_environment.availability_zone_ids
kx_environment_description = kx_environment.description
kx_environment_name = kx_environment.name
kx_environment_tgw_status = kx_environment.tgw_status
kx_environment_transit_gateway_configuration = kx_environment.transit_gateway_configuration
kx_environment_status = kx_environment.status
kx_environment_aws_account_id = kx_environment.aws_account_id
kx_environment_error_message = kx_environment.error_message
kx_environment_certificate_authority_arn = kx_environment.certificate_authority_arn
kx_environment_environment_id = kx_environment.environment_id
```

---


### Kx_volume

KxVolume resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `environment_id` | String | ✅ | <p>A unique identifier for the kdb environment, whose clusters can attach to the volume. </p> |
| `az_mode` | String | ✅ | <p>The number of availability zones you want to assign per volume. Currently, FinSpace only supports <code>SINGLE</code> for volumes. This places dataview in a single AZ.</p> |
| `availability_zone_ids` | Vec<String> | ✅ | <p>The identifier of the availability zones.</p> |
| `description` | String |  | <p>
A description of the volume.
</p> |
| `client_token` | String |  | <p>A token that ensures idempotency. This token expires in 10 minutes.</p> |
| `volume_type` | String | ✅ | <p>
   The type of file system volume. Currently, FinSpace only supports <code>NAS_1</code> volume type. When you select <code>NAS_1</code> volume type, you must also provide <code>nas1Configuration</code>.
</p> |
| `volume_name` | String | ✅ | <p>A unique identifier for the volume.</p> |
| `nas1_configuration` | String |  | <p> Specifies the configuration for the Network attached storage (NAS_1) file system volume. This
         parameter is required when you choose <code>volumeType</code> as
         <i>NAS_1</i>.</p> |
| `tags` | HashMap<String, String> |  | <p>
A list of key-value pairs to label the volume. You can add up to 50 tags to a volume.
</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The status of volume creation.</p>
         <ul>
            <li>
               <p>CREATING – The volume creation is in progress.</p>
            </li>
            <li>
               <p>CREATE_FAILED – The volume creation has failed.</p>
            </li>
            <li>
               <p>ACTIVE – The volume is active.</p>
            </li>
            <li>
               <p>UPDATING – The volume is in the process of being updated.</p>
            </li>
            <li>
               <p>UPDATE_FAILED – The update action failed.</p>
            </li>
            <li>
               <p>UPDATED – The volume is successfully updated.</p>
            </li>
            <li>
               <p>DELETING – The volume is in the process of being deleted.</p>
            </li>
            <li>
               <p>DELETE_FAILED – The system failed to delete the volume.</p>
            </li>
            <li>
               <p>DELETED – The volume is successfully deleted.</p>
            </li>
         </ul> |
| `attached_clusters` | Vec<String> | <p>
A list of cluster identifiers that a volume is attached to. 
</p> |
| `volume_name` | String | <p>
A unique identifier for the volume.</p> |
| `volume_arn` | String | <p>
The ARN identifier of the volume.
</p> |
| `status_reason` | String | <p>The error message when a failed state occurs. </p> |
| `created_timestamp` | String | <p>
The timestamp at which the volume was created in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.
</p> |
| `environment_id` | String | <p>A unique identifier for the kdb environment, whose clusters can attach to the volume. </p> |
| `nas1_configuration` | String | <p> Specifies the configuration for the Network attached storage (NAS_1) file system volume.</p> |
| `az_mode` | String | <p>The number of availability zones you want to assign per volume. Currently, FinSpace only supports <code>SINGLE</code> for volumes. This places dataview in a single AZ.</p> |
| `availability_zone_ids` | Vec<String> | <p>The identifier of the availability zones.</p> |
| `description` | String | <p>
A description of the volume.
</p> |
| `last_modified_timestamp` | String | <p>The last time that the volume was updated in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p> |
| `volume_type` | String | <p>
      The type of file system volume. Currently, FinSpace only supports <code>NAS_1</code> volume type.
   </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create kx_volume
kx_volume = provider.finspace.Kx_volume {
    environment_id = "value"  # <p>A unique identifier for the kdb environment, whose clusters can attach to the volume. </p>
    az_mode = "value"  # <p>The number of availability zones you want to assign per volume. Currently, FinSpace only supports <code>SINGLE</code> for volumes. This places dataview in a single AZ.</p>
    availability_zone_ids = "value"  # <p>The identifier of the availability zones.</p>
    volume_type = "value"  # <p>
   The type of file system volume. Currently, FinSpace only supports <code>NAS_1</code> volume type. When you select <code>NAS_1</code> volume type, you must also provide <code>nas1Configuration</code>.
</p>
    volume_name = "value"  # <p>A unique identifier for the volume.</p>
}

# Access kx_volume outputs
kx_volume_id = kx_volume.id
kx_volume_status = kx_volume.status
kx_volume_attached_clusters = kx_volume.attached_clusters
kx_volume_volume_name = kx_volume.volume_name
kx_volume_volume_arn = kx_volume.volume_arn
kx_volume_status_reason = kx_volume.status_reason
kx_volume_created_timestamp = kx_volume.created_timestamp
kx_volume_environment_id = kx_volume.environment_id
kx_volume_nas1_configuration = kx_volume.nas1_configuration
kx_volume_az_mode = kx_volume.az_mode
kx_volume_availability_zone_ids = kx_volume.availability_zone_ids
kx_volume_description = kx_volume.description
kx_volume_last_modified_timestamp = kx_volume.last_modified_timestamp
kx_volume_volume_type = kx_volume.volume_type
```

---


### Kx_connection_string

KxConnectionString resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `signed_connection_string` | String | <p>The signed connection string that you can use to connect to clusters.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access kx_connection_string outputs
kx_connection_string_id = kx_connection_string.id
kx_connection_string_signed_connection_string = kx_connection_string.signed_connection_string
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple kx_dataview resources
kx_dataview_0 = provider.finspace.Kx_dataview {
    az_mode = "value-0"
    database_name = "value-0"
    client_token = "value-0"
    dataview_name = "value-0"
    environment_id = "value-0"
}
kx_dataview_1 = provider.finspace.Kx_dataview {
    az_mode = "value-1"
    database_name = "value-1"
    client_token = "value-1"
    dataview_name = "value-1"
    environment_id = "value-1"
}
kx_dataview_2 = provider.finspace.Kx_dataview {
    az_mode = "value-2"
    database_name = "value-2"
    client_token = "value-2"
    dataview_name = "value-2"
    environment_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    kx_dataview = provider.finspace.Kx_dataview {
        az_mode = "production-value"
        database_name = "production-value"
        client_token = "production-value"
        dataview_name = "production-value"
        environment_id = "production-value"
    }
```

---

## Related Documentation

- [AWS Finspace Documentation](https://docs.aws.amazon.com/finspace/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
