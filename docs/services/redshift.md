# Redshift Service



**Resources**: 64

---

## Overview

The redshift service provides access to 64 resource types:

- [Cluster_snapshots](#cluster_snapshots) [R]
- [Cluster_snapshot](#cluster_snapshot) [CD]
- [Cluster_db_revisions](#cluster_db_revisions) [R]
- [Default_cluster_parameters](#default_cluster_parameters) [R]
- [Scheduled_actions](#scheduled_actions) [R]
- [Snapshot_schedules](#snapshot_schedules) [R]
- [Usage_limits](#usage_limits) [R]
- [Cluster_credentials_with_iam](#cluster_credentials_with_iam) [R]
- [Clusters](#clusters) [R]
- [Partner_status](#partner_status) [U]
- [Authentication_profile](#authentication_profile) [CD]
- [Reserved_node_offerings](#reserved_node_offerings) [R]
- [Cluster_parameters](#cluster_parameters) [R]
- [Partners](#partners) [R]
- [Cluster_parameter_group](#cluster_parameter_group) [CD]
- [Reserved_node_exchange_offerings](#reserved_node_exchange_offerings) [R]
- [Snapshot_copy_grants](#snapshot_copy_grants) [R]
- [Integrations](#integrations) [R]
- [Hsm_client_certificate](#hsm_client_certificate) [CD]
- [Redshift_idc_application](#redshift_idc_application) [CD]
- [Event_subscriptions](#event_subscriptions) [R]
- [Cluster_tracks](#cluster_tracks) [R]
- [Integration](#integration) [CD]
- [Reserved_node_exchange_status](#reserved_node_exchange_status) [R]
- [Custom_domain_associations](#custom_domain_associations) [R]
- [Snapshot_copy_grant](#snapshot_copy_grant) [CD]
- [Endpoint_authorization](#endpoint_authorization) [R]
- [Cluster_versions](#cluster_versions) [R]
- [Orderable_cluster_options](#orderable_cluster_options) [R]
- [Cluster_credentials](#cluster_credentials) [R]
- [Cluster_parameter_groups](#cluster_parameter_groups) [R]
- [Tags](#tags) [CRD]
- [Cluster_security_group](#cluster_security_group) [CD]
- [Data_shares_for_producer](#data_shares_for_producer) [R]
- [Custom_domain_association](#custom_domain_association) [CD]
- [Usage_limit](#usage_limit) [CD]
- [Hsm_configurations](#hsm_configurations) [R]
- [Endpoint_access](#endpoint_access) [CRD]
- [Data_shares](#data_shares) [R]
- [Hsm_client_certificates](#hsm_client_certificates) [R]
- [Partner](#partner) [D]
- [Resize](#resize) [R]
- [Account_attributes](#account_attributes) [R]
- [Scheduled_action](#scheduled_action) [CD]
- [Resource_policy](#resource_policy) [CRD]
- [Reserved_nodes](#reserved_nodes) [R]
- [Data_shares_for_consumer](#data_shares_for_consumer) [R]
- [Cluster_security_groups](#cluster_security_groups) [R]
- [Events](#events) [R]
- [Hsm_configuration](#hsm_configuration) [CD]
- [Event_subscription](#event_subscription) [CD]
- [Inbound_integrations](#inbound_integrations) [R]
- [Logging_status](#logging_status) [R]
- [Cluster](#cluster) [CD]
- [Node_configuration_options](#node_configuration_options) [R]
- [Event_categories](#event_categories) [R]
- [Storage](#storage) [R]
- [Snapshot_schedule](#snapshot_schedule) [CD]
- [Table_restore_status](#table_restore_status) [R]
- [Cluster_subnet_group](#cluster_subnet_group) [CD]
- [Cluster_subnet_groups](#cluster_subnet_groups) [R]
- [Reserved_node_exchange_configuration_options](#reserved_node_exchange_configuration_options) [R]
- [Authentication_profiles](#authentication_profiles) [R]
- [Redshift_idc_applications](#redshift_idc_applications) [R]

---

## Resources


### Cluster_snapshots

ClusterSnapshots resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>Marker</code> parameter
            and retrying the command. If the <code>Marker</code> field is empty, all response
            records have been retrieved for the request. </p> |
| `snapshots` | Vec<String> | <p>A list of <a>Snapshot</a> instances. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cluster_snapshots outputs
cluster_snapshots_id = cluster_snapshots.id
cluster_snapshots_marker = cluster_snapshots.marker
cluster_snapshots_snapshots = cluster_snapshots.snapshots
```

---


### Cluster_snapshot

ClusterSnapshot resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `snapshot_identifier` | String | ✅ | <p>A unique identifier for the snapshot that you are requesting. This identifier must
            be unique for all snapshots within the Amazon Web Services account.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Cannot be null, empty, or blank</p>
            </li>
            <li>
               <p>Must contain from 1 to 255 alphanumeric characters or hyphens</p>
            </li>
            <li>
               <p>First character must be a letter</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens</p>
            </li>
         </ul>
         <p>Example: <code>my-snapshot-id</code>
         </p> |
| `manual_snapshot_retention_period` | i64 |  | <p>The number of days that a manual snapshot is retained. If the value is -1, the manual
            snapshot is retained indefinitely. </p>
         <p>The value must be either -1 or an integer between 1 and 3,653.</p>
         <p>The default value is -1.</p> |
| `tags` | Vec<String> |  | <p>A list of tag instances.</p> |
| `cluster_identifier` | String | ✅ | <p>The cluster identifier for which you want a snapshot.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster_snapshot
cluster_snapshot = provider.redshift.Cluster_snapshot {
    snapshot_identifier = "value"  # <p>A unique identifier for the snapshot that you are requesting. This identifier must
            be unique for all snapshots within the Amazon Web Services account.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Cannot be null, empty, or blank</p>
            </li>
            <li>
               <p>Must contain from 1 to 255 alphanumeric characters or hyphens</p>
            </li>
            <li>
               <p>First character must be a letter</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens</p>
            </li>
         </ul>
         <p>Example: <code>my-snapshot-id</code>
         </p>
    cluster_identifier = "value"  # <p>The cluster identifier for which you want a snapshot.</p>
}

```

---


### Cluster_db_revisions

ClusterDbRevisions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>A string representing the starting point for the next set of revisions. If a value is
            returned in a response, you can retrieve the next set of revisions by providing the
            value in the <code>marker</code> parameter and retrying the command. If the
                <code>marker</code> field is empty, all revisions have already been returned.</p> |
| `cluster_db_revisions` | Vec<String> | <p>A list of revisions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cluster_db_revisions outputs
cluster_db_revisions_id = cluster_db_revisions.id
cluster_db_revisions_marker = cluster_db_revisions.marker
cluster_db_revisions_cluster_db_revisions = cluster_db_revisions.cluster_db_revisions
```

---


### Default_cluster_parameters

DefaultClusterParameters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_cluster_parameters` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access default_cluster_parameters outputs
default_cluster_parameters_id = default_cluster_parameters.id
default_cluster_parameters_default_cluster_parameters = default_cluster_parameters.default_cluster_parameters
```

---


### Scheduled_actions

ScheduledActions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional parameter that specifies the starting point to return a set of response
            records. When the results of a <a>DescribeScheduledActions</a> request
            exceed the value specified in <code>MaxRecords</code>, Amazon Web Services returns a value in the
            <code>Marker</code> field of the response. You can retrieve the next set of response
            records by providing the returned marker value in the <code>Marker</code> parameter and
            retrying the request. </p> |
| `scheduled_actions` | Vec<String> | <p>List of retrieved scheduled actions. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access scheduled_actions outputs
scheduled_actions_id = scheduled_actions.id
scheduled_actions_marker = scheduled_actions.marker
scheduled_actions_scheduled_actions = scheduled_actions.scheduled_actions
```

---


### Snapshot_schedules

SnapshotSchedules resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `snapshot_schedules` | Vec<String> | <p>A list of SnapshotSchedules.</p> |
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>marker</code> parameter
            and retrying the command. If the <code>marker</code> field is empty, all response
            records have been retrieved for the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access snapshot_schedules outputs
snapshot_schedules_id = snapshot_schedules.id
snapshot_schedules_snapshot_schedules = snapshot_schedules.snapshot_schedules
snapshot_schedules_marker = snapshot_schedules.marker
```

---


### Usage_limits

UsageLimits resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `usage_limits` | Vec<String> | <p>Contains the output from the <a>DescribeUsageLimits</a>
            action. </p> |
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>Marker</code> parameter
            and retrying the command. If the <code>Marker</code> field is empty, all response
            records have been retrieved for the request. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access usage_limits outputs
usage_limits_id = usage_limits.id
usage_limits_usage_limits = usage_limits.usage_limits
usage_limits_marker = usage_limits.marker
```

---


### Cluster_credentials_with_iam

ClusterCredentialsWithIAM resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_password` | String | <p>A temporary password that you provide when you connect to a database.</p> |
| `next_refresh_time` | String | <p>Reserved for future use.</p> |
| `db_user` | String | <p>A database user name that you provide when you connect to a database. The database user is mapped 1:1 to the source IAM identity. </p> |
| `expiration` | String | <p>The time (UTC) when the temporary password expires. After this timestamp, a log in with the temporary password fails.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cluster_credentials_with_iam outputs
cluster_credentials_with_iam_id = cluster_credentials_with_iam.id
cluster_credentials_with_iam_db_password = cluster_credentials_with_iam.db_password
cluster_credentials_with_iam_next_refresh_time = cluster_credentials_with_iam.next_refresh_time
cluster_credentials_with_iam_db_user = cluster_credentials_with_iam.db_user
cluster_credentials_with_iam_expiration = cluster_credentials_with_iam.expiration
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
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>Marker</code> parameter
            and retrying the command. If the <code>Marker</code> field is empty, all response
            records have been retrieved for the request. </p> |
| `clusters` | Vec<String> | <p>A list of <code>Cluster</code> objects, where each object describes one cluster.
        </p> |


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
clusters_marker = clusters.marker
clusters_clusters = clusters.clusters
```

---


### Partner_status

PartnerStatus resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status_message` | String |  | <p>The status message provided by the partner.</p> |
| `database_name` | String | ✅ | <p>The name of the database whose partner integration status is being updated.</p> |
| `partner_name` | String | ✅ | <p>The name of the partner whose integration status is being updated.</p> |
| `cluster_identifier` | String | ✅ | <p>The cluster identifier of the cluster whose partner integration status is being updated.</p> |
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID that owns the cluster.</p> |
| `status` | String | ✅ | <p>The value of the updated status.</p> |



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


### Authentication_profile

AuthenticationProfile resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `authentication_profile_name` | String | ✅ | <p>The name of the authentication profile to be created.</p> |
| `authentication_profile_content` | String | ✅ | <p>The content of the authentication profile in JSON format. 
            The maximum length of the JSON string is determined by a quota for your account.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create authentication_profile
authentication_profile = provider.redshift.Authentication_profile {
    authentication_profile_name = "value"  # <p>The name of the authentication profile to be created.</p>
    authentication_profile_content = "value"  # <p>The content of the authentication profile in JSON format. 
            The maximum length of the JSON string is determined by a quota for your account.</p>
}

```

---


### Reserved_node_offerings

ReservedNodeOfferings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>Marker</code> parameter
            and retrying the command. If the <code>Marker</code> field is empty, all response
            records have been retrieved for the request. </p> |
| `reserved_node_offerings` | Vec<String> | <p>A list of <code>ReservedNodeOffering</code> objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_node_offerings outputs
reserved_node_offerings_id = reserved_node_offerings.id
reserved_node_offerings_marker = reserved_node_offerings.marker
reserved_node_offerings_reserved_node_offerings = reserved_node_offerings.reserved_node_offerings
```

---


### Cluster_parameters

ClusterParameters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parameters` | Vec<String> | <p>A list of <a>Parameter</a> instances. Each instance lists the parameters
            of one cluster parameter group. </p> |
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>Marker</code> parameter
            and retrying the command. If the <code>Marker</code> field is empty, all response
            records have been retrieved for the request. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cluster_parameters outputs
cluster_parameters_id = cluster_parameters.id
cluster_parameters_parameters = cluster_parameters.parameters
cluster_parameters_marker = cluster_parameters.marker
```

---


### Partners

Partners resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `partner_integration_info_list` | Vec<String> | <p>A list of partner integrations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access partners outputs
partners_id = partners.id
partners_partner_integration_info_list = partners.partner_integration_info_list
```

---


### Cluster_parameter_group

ClusterParameterGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameter_group_name` | String | ✅ | <p>The name of the cluster parameter group.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 1 to 255 alphanumeric characters or hyphens</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
            <li>
               <p>Must be unique withing your Amazon Web Services account.</p>
            </li>
         </ul>
         <note>
            <p>This value is stored as a lower-case string.</p>
         </note> |
| `description` | String | ✅ | <p>A description of the parameter group.</p> |
| `parameter_group_family` | String | ✅ | <p>The Amazon Redshift engine version to which the cluster parameter group applies. The
            cluster engine version determines the set of parameters.</p>
         <p>To get a list of valid parameter group family names, you can call <a>DescribeClusterParameterGroups</a>. By default, Amazon Redshift returns a list of
            all the parameter groups that are owned by your Amazon Web Services account, including the default
            parameter groups for each Amazon Redshift engine version. The parameter group family names
            associated with the default parameter groups provide you the valid values. For example,
            a valid family name is "redshift-1.0". </p> |
| `tags` | Vec<String> |  | <p>A list of tag instances.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster_parameter_group
cluster_parameter_group = provider.redshift.Cluster_parameter_group {
    parameter_group_name = "value"  # <p>The name of the cluster parameter group.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 1 to 255 alphanumeric characters or hyphens</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
            <li>
               <p>Must be unique withing your Amazon Web Services account.</p>
            </li>
         </ul>
         <note>
            <p>This value is stored as a lower-case string.</p>
         </note>
    description = "value"  # <p>A description of the parameter group.</p>
    parameter_group_family = "value"  # <p>The Amazon Redshift engine version to which the cluster parameter group applies. The
            cluster engine version determines the set of parameters.</p>
         <p>To get a list of valid parameter group family names, you can call <a>DescribeClusterParameterGroups</a>. By default, Amazon Redshift returns a list of
            all the parameter groups that are owned by your Amazon Web Services account, including the default
            parameter groups for each Amazon Redshift engine version. The parameter group family names
            associated with the default parameter groups provide you the valid values. For example,
            a valid family name is "redshift-1.0". </p>
}

```

---


### Reserved_node_exchange_offerings

ReservedNodeExchangeOfferings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reserved_node_offerings` | Vec<String> | <p>Returns an array of <a>ReservedNodeOffering</a> objects.</p> |
| `marker` | String | <p>An optional parameter that specifies the starting point for returning a set of
            response records. When the results of a <code>GetReservedNodeExchangeOfferings</code>
            request exceed the value specified in MaxRecords, Amazon Redshift returns a value in the
            marker field of the response. You can retrieve the next set of response records by
            providing the returned marker value in the marker parameter and retrying the request.
        </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_node_exchange_offerings outputs
reserved_node_exchange_offerings_id = reserved_node_exchange_offerings.id
reserved_node_exchange_offerings_reserved_node_offerings = reserved_node_exchange_offerings.reserved_node_offerings
reserved_node_exchange_offerings_marker = reserved_node_exchange_offerings.marker
```

---


### Snapshot_copy_grants

SnapshotCopyGrants resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `snapshot_copy_grants` | Vec<String> | <p>The list of <code>SnapshotCopyGrant</code> objects.</p> |
| `marker` | String | <p>An optional parameter that specifies the starting point to return a set of response
            records. When the results of a <code>DescribeSnapshotCopyGrant</code> request exceed the
            value specified in <code>MaxRecords</code>, Amazon Web Services returns a value in the
                <code>Marker</code> field of the response. You can retrieve the next set of response
            records by providing the returned marker value in the <code>Marker</code> parameter and
            retrying the request. </p>
         <p>Constraints: You can specify either the <b>SnapshotCopyGrantName</b> parameter or the <b>Marker</b> parameter, but not both. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access snapshot_copy_grants outputs
snapshot_copy_grants_id = snapshot_copy_grants.id
snapshot_copy_grants_snapshot_copy_grants = snapshot_copy_grants.snapshot_copy_grants
snapshot_copy_grants_marker = snapshot_copy_grants.marker
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
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a subsequent request. 
            If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>Marker</code> parameter and retrying the command. 
            If the <code>Marker</code> field is empty, all response records have been retrieved for the request.</p> |
| `integrations` | Vec<String> | <p>List of integrations that are described.</p> |


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
integrations_marker = integrations.marker
integrations_integrations = integrations.integrations
```

---


### Hsm_client_certificate

HsmClientCertificate resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of tag instances.</p> |
| `hsm_client_certificate_identifier` | String | ✅ | <p>The identifier to be assigned to the new HSM client certificate that the cluster
            will use to connect to the HSM to use the database encryption keys.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create hsm_client_certificate
hsm_client_certificate = provider.redshift.Hsm_client_certificate {
    hsm_client_certificate_identifier = "value"  # <p>The identifier to be assigned to the new HSM client certificate that the cluster
            will use to connect to the HSM to use the database encryption keys.</p>
}

```

---


### Redshift_idc_application

RedshiftIdcApplication resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_integrations` | Vec<String> |  | <p>A collection of service integrations for the Redshift IAM Identity Center application.</p> |
| `iam_role_arn` | String | ✅ | <p>The IAM role ARN for the Amazon Redshift IAM Identity Center application instance. It has the required permissions 
            to be assumed and invoke the IDC Identity Center API.</p> |
| `idc_instance_arn` | String | ✅ | <p>The Amazon resource name (ARN) of the IAM Identity Center instance where Amazon Redshift creates a new managed application.</p> |
| `identity_namespace` | String |  | <p>The namespace for the Amazon Redshift IAM Identity Center application instance. It determines which managed application 
            verifies the connection token.</p> |
| `sso_tag_keys` | Vec<String> |  | <p>A list of tags keys that Redshift Identity Center applications copy to IAM Identity
            Center. For each input key, the tag corresponding to the key-value pair is
            propagated.</p> |
| `redshift_idc_application_name` | String | ✅ | <p>The name of the Redshift application in IAM Identity Center.</p> |
| `idc_display_name` | String | ✅ | <p>The display name for the Amazon Redshift IAM Identity Center application instance. It appears in the console.</p> |
| `tags` | Vec<String> |  | <p>A list of tags.</p> |
| `authorized_token_issuer_list` | Vec<String> |  | <p>The token issuer list for the Amazon Redshift IAM Identity Center application instance.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create redshift_idc_application
redshift_idc_application = provider.redshift.Redshift_idc_application {
    iam_role_arn = "value"  # <p>The IAM role ARN for the Amazon Redshift IAM Identity Center application instance. It has the required permissions 
            to be assumed and invoke the IDC Identity Center API.</p>
    idc_instance_arn = "value"  # <p>The Amazon resource name (ARN) of the IAM Identity Center instance where Amazon Redshift creates a new managed application.</p>
    redshift_idc_application_name = "value"  # <p>The name of the Redshift application in IAM Identity Center.</p>
    idc_display_name = "value"  # <p>The display name for the Amazon Redshift IAM Identity Center application instance. It appears in the console.</p>
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
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>Marker</code> parameter
            and retrying the command. If the <code>Marker</code> field is empty, all response
            records have been retrieved for the request. </p> |
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


### Cluster_tracks

ClusterTracks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>The starting point to return a set of response tracklist records. You can retrieve the
            next set of response records by providing the returned marker value in the
                <code>Marker</code> parameter and retrying the request.</p> |
| `maintenance_tracks` | Vec<String> | <p>A list of maintenance tracks output by the <code>DescribeClusterTracks</code>
            operation. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cluster_tracks outputs
cluster_tracks_id = cluster_tracks.id
cluster_tracks_marker = cluster_tracks.marker
cluster_tracks_maintenance_tracks = cluster_tracks.maintenance_tracks
```

---


### Integration

Integration resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Amazon Redshift data warehouse to use as the target for replication.</p> |
| `tag_list` | Vec<String> |  | <p>A list of tags.</p> |
| `source_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the database to use as the source for replication.</p> |
| `integration_name` | String | ✅ | <p>The name of the integration.</p> |
| `description` | String |  | <p>A description of the integration.</p> |
| `kms_key_id` | String |  | <p>An Key Management Service (KMS) key identifier for the key to use to
            encrypt the integration. If you don't specify an encryption key, the default
            Amazon Web Services owned key is used.</p> |
| `additional_encryption_context` | HashMap<String, String> |  | <p>An optional set of non-secret key–value pairs that contains additional contextual
            information about the data. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption
                context</a> in the <i>Amazon Web Services Key Management Service Developer
                    Guide</i>.</p>
         <p>You can only include this parameter if you specify the <code>KMSKeyId</code> parameter.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create integration
integration = provider.redshift.Integration {
    target_arn = "value"  # <p>The Amazon Resource Name (ARN) of the Amazon Redshift data warehouse to use as the target for replication.</p>
    source_arn = "value"  # <p>The Amazon Resource Name (ARN) of the database to use as the source for replication.</p>
    integration_name = "value"  # <p>The name of the integration.</p>
}

```

---


### Reserved_node_exchange_status

ReservedNodeExchangeStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reserved_node_exchange_status_details` | Vec<String> | <p>The details of the reserved-node exchange request, including the status, request
            time, source reserved-node identifier, and additional details.</p> |
| `marker` | String | <p>A pagination token provided by a previous <code>DescribeReservedNodeExchangeStatus</code> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_node_exchange_status outputs
reserved_node_exchange_status_id = reserved_node_exchange_status.id
reserved_node_exchange_status_reserved_node_exchange_status_details = reserved_node_exchange_status.reserved_node_exchange_status_details
reserved_node_exchange_status_marker = reserved_node_exchange_status.marker
```

---


### Custom_domain_associations

CustomDomainAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `associations` | Vec<String> | <p>The associations for the custom domain.</p> |
| `marker` | String | <p>The marker for the custom domain association.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access custom_domain_associations outputs
custom_domain_associations_id = custom_domain_associations.id
custom_domain_associations_associations = custom_domain_associations.associations
custom_domain_associations_marker = custom_domain_associations.marker
```

---


### Snapshot_copy_grant

SnapshotCopyGrant resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of tag instances.</p> |
| `kms_key_id` | String |  | <p>The unique identifier of the encrypted symmetric key to which to grant Amazon Redshift
            permission. If no key is specified, the default key is used.</p> |
| `snapshot_copy_grant_name` | String | ✅ | <p>The name of the snapshot copy grant. This name must be unique in the region for the
            Amazon Web Services account.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>Alphabetic characters must be lowercase.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
            <li>
               <p>Must be unique for all clusters within an Amazon Web Services account.</p>
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

# Create snapshot_copy_grant
snapshot_copy_grant = provider.redshift.Snapshot_copy_grant {
    snapshot_copy_grant_name = "value"  # <p>The name of the snapshot copy grant. This name must be unique in the region for the
            Amazon Web Services account.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>Alphabetic characters must be lowercase.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
            <li>
               <p>Must be unique for all clusters within an Amazon Web Services account.</p>
            </li>
         </ul>
}

```

---


### Endpoint_authorization

EndpointAuthorization resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous
            <code>DescribeEndpointAuthorization</code> request. If this parameter is specified, the
            response includes only records beyond the marker, up to the value specified by the
            <code>MaxRecords</code> parameter.</p> |
| `endpoint_authorization_list` | Vec<String> | <p>The authorizations to an endpoint.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access endpoint_authorization outputs
endpoint_authorization_id = endpoint_authorization.id
endpoint_authorization_marker = endpoint_authorization.marker
endpoint_authorization_endpoint_authorization_list = endpoint_authorization.endpoint_authorization_list
```

---


### Cluster_versions

ClusterVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>Marker</code> parameter
            and retrying the command. If the <code>Marker</code> field is empty, all response
            records have been retrieved for the request. </p> |
| `cluster_versions` | Vec<String> | <p>A list of <code>Version</code> elements. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cluster_versions outputs
cluster_versions_id = cluster_versions.id
cluster_versions_marker = cluster_versions.marker
cluster_versions_cluster_versions = cluster_versions.cluster_versions
```

---


### Orderable_cluster_options

OrderableClusterOptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `orderable_cluster_options` | Vec<String> | <p>An <code>OrderableClusterOption</code> structure containing information about
            orderable options for the cluster.</p> |
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>Marker</code> parameter
            and retrying the command. If the <code>Marker</code> field is empty, all response
            records have been retrieved for the request. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access orderable_cluster_options outputs
orderable_cluster_options_id = orderable_cluster_options.id
orderable_cluster_options_orderable_cluster_options = orderable_cluster_options.orderable_cluster_options
orderable_cluster_options_marker = orderable_cluster_options.marker
```

---


### Cluster_credentials

ClusterCredentials resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_user` | String | <p>A database user name that is authorized to log on to the database <code>DbName</code>
            using the password <code>DbPassword</code>. If the specified DbUser exists in the
            database, the new user name has the same database permissions as the the user named in
            DbUser. By default, the user is added to PUBLIC. If the <code>DbGroups</code> parameter
            is specifed, <code>DbUser</code> is added to the listed groups for any sessions created
            using these credentials.</p> |
| `db_password` | String | <p>A temporary password that authorizes the user name returned by <code>DbUser</code>
            to log on to the database <code>DbName</code>. </p> |
| `expiration` | String | <p>The date and time the password in <code>DbPassword</code> expires.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cluster_credentials outputs
cluster_credentials_id = cluster_credentials.id
cluster_credentials_db_user = cluster_credentials.db_user
cluster_credentials_db_password = cluster_credentials.db_password
cluster_credentials_expiration = cluster_credentials.expiration
```

---


### Cluster_parameter_groups

ClusterParameterGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parameter_groups` | Vec<String> | <p>A list of <a>ClusterParameterGroup</a> instances. Each instance
            describes one cluster parameter group. </p> |
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>Marker</code> parameter
            and retrying the command. If the <code>Marker</code> field is empty, all response
            records have been retrieved for the request. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cluster_parameter_groups outputs
cluster_parameter_groups_id = cluster_parameter_groups.id
cluster_parameter_groups_parameter_groups = cluster_parameter_groups.parameter_groups
cluster_parameter_groups_marker = cluster_parameter_groups.marker
```

---


### Tags

Tags resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_name` | String | ✅ | <p>The Amazon Resource Name (ARN) to which you want to add the tag or tags. For
            example, <code>arn:aws:redshift:us-east-2:123456789:cluster:t1</code>. </p> |
| `tags` | Vec<String> | ✅ | <p>One or more name/value pairs to add as tags to the specified resource. Each tag
            name is passed in with the parameter <code>Key</code> and the corresponding value is
            passed in with the parameter <code>Value</code>. The <code>Key</code> and
                <code>Value</code> parameters are separated by a comma (,). Separate multiple tags
            with a space. For example, <code>--tags "Key"="owner","Value"="admin"
                "Key"="environment","Value"="test" "Key"="version","Value"="1.0"</code>. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>Marker</code> parameter
            and retrying the command. If the <code>Marker</code> field is empty, all response
            records have been retrieved for the request. </p> |
| `tagged_resources` | Vec<String> | <p>A list of tags with their associated resources.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create tags
tags = provider.redshift.Tags {
    resource_name = "value"  # <p>The Amazon Resource Name (ARN) to which you want to add the tag or tags. For
            example, <code>arn:aws:redshift:us-east-2:123456789:cluster:t1</code>. </p>
    tags = "value"  # <p>One or more name/value pairs to add as tags to the specified resource. Each tag
            name is passed in with the parameter <code>Key</code> and the corresponding value is
            passed in with the parameter <code>Value</code>. The <code>Key</code> and
                <code>Value</code> parameters are separated by a comma (,). Separate multiple tags
            with a space. For example, <code>--tags "Key"="owner","Value"="admin"
                "Key"="environment","Value"="test" "Key"="version","Value"="1.0"</code>. </p>
}

# Access tags outputs
tags_id = tags.id
tags_marker = tags.marker
tags_tagged_resources = tags.tagged_resources
```

---


### Cluster_security_group

ClusterSecurityGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String | ✅ | <p>A description for the security group.</p> |
| `cluster_security_group_name` | String | ✅ | <p>The name for the security group. Amazon Redshift stores the value as a lowercase
            string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain no more than 255 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>Must not be "Default".</p>
            </li>
            <li>
               <p>Must be unique for all security groups that are created by your Amazon Web Services account.</p>
            </li>
         </ul>
         <p>Example: <code>examplesecuritygroup</code>
         </p> |
| `tags` | Vec<String> |  | <p>A list of tag instances.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster_security_group
cluster_security_group = provider.redshift.Cluster_security_group {
    description = "value"  # <p>A description for the security group.</p>
    cluster_security_group_name = "value"  # <p>The name for the security group. Amazon Redshift stores the value as a lowercase
            string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain no more than 255 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>Must not be "Default".</p>
            </li>
            <li>
               <p>Must be unique for all security groups that are created by your Amazon Web Services account.</p>
            </li>
         </ul>
         <p>Example: <code>examplesecuritygroup</code>
         </p>
}

```

---


### Data_shares_for_producer

DataSharesForProducer resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_shares` | Vec<String> | <p>Shows the results of datashares available for producers.</p> |
| `marker` | String | <p>An optional parameter that specifies the starting point to return a set of response
            records. When the results of a <a>DescribeDataSharesForProducer</a> request
            exceed the value specified in <code>MaxRecords</code>, Amazon Web Services returns a value in the
            <code>Marker</code> field of the response. You can retrieve the next set of response
            records by providing the returned marker value in the <code>Marker</code> parameter and
            retrying the request. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_shares_for_producer outputs
data_shares_for_producer_id = data_shares_for_producer.id
data_shares_for_producer_data_shares = data_shares_for_producer.data_shares
data_shares_for_producer_marker = data_shares_for_producer.marker
```

---


### Custom_domain_association

CustomDomainAssociation resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `custom_domain_certificate_arn` | String | ✅ | <p>The certificate Amazon Resource Name (ARN) for the custom domain name association.</p> |
| `cluster_identifier` | String | ✅ | <p>The cluster identifier that the custom domain is associated with.</p> |
| `custom_domain_name` | String | ✅ | <p>The custom domain name for a custom domain association.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create custom_domain_association
custom_domain_association = provider.redshift.Custom_domain_association {
    custom_domain_certificate_arn = "value"  # <p>The certificate Amazon Resource Name (ARN) for the custom domain name association.</p>
    cluster_identifier = "value"  # <p>The cluster identifier that the custom domain is associated with.</p>
    custom_domain_name = "value"  # <p>The custom domain name for a custom domain association.</p>
}

```

---


### Usage_limit

UsageLimit resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `feature_type` | String | ✅ | <p>The Amazon Redshift feature that you want to limit.</p> |
| `cluster_identifier` | String | ✅ | <p>The identifier of the cluster that you want to limit usage.</p> |
| `period` | String |  | <p>The time period that the amount applies to. A <code>weekly</code> period begins on Sunday. The default is <code>monthly</code>. 
            </p> |
| `amount` | i64 | ✅ | <p>The limit amount. If time-based, this amount is in minutes. If data-based, this amount is in terabytes (TB).
            The value must be a positive number.
            </p> |
| `breach_action` | String |  | <p>The action that Amazon Redshift takes when the limit is reached. The default is log. 
            For more information about this parameter, see <a>UsageLimit</a>.</p> |
| `tags` | Vec<String> |  | <p>A list of tag instances.</p> |
| `limit_type` | String | ✅ | <p>The type of limit. Depending on the feature type, this can be based on a time duration or data size.
            If <code>FeatureType</code> is <code>spectrum</code>, then <code>LimitType</code> must be <code>data-scanned</code>.
            If <code>FeatureType</code> is <code>concurrency-scaling</code>, then <code>LimitType</code> must be <code>time</code>.
            If <code>FeatureType</code> is <code>cross-region-datasharing</code>, then <code>LimitType</code> must be <code>data-scanned</code>.
           </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create usage_limit
usage_limit = provider.redshift.Usage_limit {
    feature_type = "value"  # <p>The Amazon Redshift feature that you want to limit.</p>
    cluster_identifier = "value"  # <p>The identifier of the cluster that you want to limit usage.</p>
    amount = "value"  # <p>The limit amount. If time-based, this amount is in minutes. If data-based, this amount is in terabytes (TB).
            The value must be a positive number.
            </p>
    limit_type = "value"  # <p>The type of limit. Depending on the feature type, this can be based on a time duration or data size.
            If <code>FeatureType</code> is <code>spectrum</code>, then <code>LimitType</code> must be <code>data-scanned</code>.
            If <code>FeatureType</code> is <code>concurrency-scaling</code>, then <code>LimitType</code> must be <code>time</code>.
            If <code>FeatureType</code> is <code>cross-region-datasharing</code>, then <code>LimitType</code> must be <code>data-scanned</code>.
           </p>
}

```

---


### Hsm_configurations

HsmConfigurations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hsm_configurations` | Vec<String> | <p>A list of <code>HsmConfiguration</code> objects.</p> |
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>Marker</code> parameter
            and retrying the command. If the <code>Marker</code> field is empty, all response
            records have been retrieved for the request. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access hsm_configurations outputs
hsm_configurations_id = hsm_configurations.id
hsm_configurations_hsm_configurations = hsm_configurations.hsm_configurations
hsm_configurations_marker = hsm_configurations.marker
```

---


### Endpoint_access

EndpointAccess resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cluster_identifier` | String |  | <p>The cluster identifier of the cluster to access.</p> |
| `endpoint_name` | String | ✅ | <p>The Redshift-managed VPC endpoint name.</p>
         <p>An endpoint name must contain 1-30 characters.
          Valid characters are A-Z, a-z, 0-9, and hyphen(-).
          The first character must be a letter.
          The name can't contain two consecutive hyphens or end with a hyphen.</p> |
| `subnet_group_name` | String | ✅ | <p>The subnet group from which Amazon Redshift chooses the subnet to deploy the endpoint.</p> |
| `vpc_security_group_ids` | Vec<String> |  | <p>The security group that defines the ports, protocols, and sources for inbound traffic that you are authorizing into your endpoint.</p> |
| `resource_owner` | String |  | <p>The Amazon Web Services account ID of the owner of the cluster. This is only required if the cluster is in another Amazon Web Services account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoint_access_list` | Vec<String> | <p>The list of endpoints with access to the cluster.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous
            <code>DescribeEndpointAccess</code> request. If this parameter is specified, the
            response includes only records beyond the marker, up to the value specified by the
            <code>MaxRecords</code> parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create endpoint_access
endpoint_access = provider.redshift.Endpoint_access {
    endpoint_name = "value"  # <p>The Redshift-managed VPC endpoint name.</p>
         <p>An endpoint name must contain 1-30 characters.
          Valid characters are A-Z, a-z, 0-9, and hyphen(-).
          The first character must be a letter.
          The name can't contain two consecutive hyphens or end with a hyphen.</p>
    subnet_group_name = "value"  # <p>The subnet group from which Amazon Redshift chooses the subnet to deploy the endpoint.</p>
}

# Access endpoint_access outputs
endpoint_access_id = endpoint_access.id
endpoint_access_endpoint_access_list = endpoint_access.endpoint_access_list
endpoint_access_marker = endpoint_access.marker
```

---


### Data_shares

DataShares resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_shares` | Vec<String> | <p>The results returned from describing datashares.</p> |
| `marker` | String | <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <a>DescribeDataShares</a> request exceed the value specified in <code>MaxRecords</code>, Amazon Web Services returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_shares outputs
data_shares_id = data_shares.id
data_shares_data_shares = data_shares.data_shares
data_shares_marker = data_shares.marker
```

---


### Hsm_client_certificates

HsmClientCertificates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>Marker</code> parameter
            and retrying the command. If the <code>Marker</code> field is empty, all response
            records have been retrieved for the request. </p> |
| `hsm_client_certificates` | Vec<String> | <p>A list of the identifiers for one or more HSM client certificates used by Amazon Redshift
            clusters to store and retrieve database encryption keys in an HSM.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access hsm_client_certificates outputs
hsm_client_certificates_id = hsm_client_certificates.id
hsm_client_certificates_marker = hsm_client_certificates.marker
hsm_client_certificates_hsm_client_certificates = hsm_client_certificates.hsm_client_certificates
```

---


### Partner

Partner resource

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


### Resize

Resize resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `target_node_type` | String | <p>The node type that the cluster will have after the resize operation is
            complete.</p> |
| `status` | String | <p>The status of the resize operation.</p>
         <p>Valid Values: <code>NONE</code> | <code>IN_PROGRESS</code> | <code>FAILED</code> |
                <code>SUCCEEDED</code> | <code>CANCELLING</code>
         </p> |
| `estimated_time_to_completion_in_seconds` | i64 | <p>The estimated time remaining, in seconds, until the resize operation is complete.
            This value is calculated based on the average resize rate and the estimated amount of
            data remaining to be processed. Once the resize operation is complete, this value will
            be 0.</p> |
| `data_transfer_progress_percent` | f64 | <p>The percent of data transferred from source cluster to target cluster.</p> |
| `import_tables_not_started` | Vec<String> | <p>The names of tables that have not been yet imported.</p>
         <p>Valid Values: List of table names</p> |
| `import_tables_in_progress` | Vec<String> | <p>The names of tables that are being currently imported.</p>
         <p>Valid Values: List of table names.</p> |
| `total_resize_data_in_mega_bytes` | i64 | <p>The estimated total amount of data, in megabytes, on the cluster before the resize
            operation began.</p> |
| `target_encryption_type` | String | <p>The type of encryption for the cluster after the resize is complete.</p>
         <p>Possible values are <code>KMS</code> and <code>None</code>. </p> |
| `message` | String | <p>An optional string to provide additional details about the resize action.</p> |
| `target_number_of_nodes` | i64 | <p>The number of nodes that the cluster will have after the resize operation is
            complete.</p> |
| `progress_in_mega_bytes` | i64 | <p>While the resize operation is in progress, this value shows the current amount of
            data, in megabytes, that has been processed so far. When the resize operation is
            complete, this value shows the total amount of data, in megabytes, on the cluster, which
            may be more or less than TotalResizeDataInMegaBytes (the estimated total amount of data
            before resize).</p> |
| `elapsed_time_in_seconds` | i64 | <p>The amount of seconds that have elapsed since the resize operation began. After the
            resize operation completes, this value shows the total actual time, in seconds, for the
            resize operation.</p> |
| `target_cluster_type` | String | <p>The cluster type after the resize operation is complete.</p>
         <p>Valid Values: <code>multi-node</code> | <code>single-node</code>
         </p> |
| `avg_resize_rate_in_mega_bytes_per_second` | f64 | <p>The average rate of the resize operation over the last few minutes, measured in
            megabytes per second. After the resize operation completes, this value shows the average
            rate of the entire resize operation.</p> |
| `resize_type` | String | <p>An enum with possible values of <code>ClassicResize</code> and
                <code>ElasticResize</code>. These values describe the type of resize operation being
            performed. </p> |
| `import_tables_completed` | Vec<String> | <p>The names of tables that have been completely imported .</p>
         <p>Valid Values: List of table names.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resize outputs
resize_id = resize.id
resize_target_node_type = resize.target_node_type
resize_status = resize.status
resize_estimated_time_to_completion_in_seconds = resize.estimated_time_to_completion_in_seconds
resize_data_transfer_progress_percent = resize.data_transfer_progress_percent
resize_import_tables_not_started = resize.import_tables_not_started
resize_import_tables_in_progress = resize.import_tables_in_progress
resize_total_resize_data_in_mega_bytes = resize.total_resize_data_in_mega_bytes
resize_target_encryption_type = resize.target_encryption_type
resize_message = resize.message
resize_target_number_of_nodes = resize.target_number_of_nodes
resize_progress_in_mega_bytes = resize.progress_in_mega_bytes
resize_elapsed_time_in_seconds = resize.elapsed_time_in_seconds
resize_target_cluster_type = resize.target_cluster_type
resize_avg_resize_rate_in_mega_bytes_per_second = resize.avg_resize_rate_in_mega_bytes_per_second
resize_resize_type = resize.resize_type
resize_import_tables_completed = resize.import_tables_completed
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
| `account_attributes` | Vec<String> | <p>A list of attributes assigned to an account.</p> |


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
account_attributes_account_attributes = account_attributes.account_attributes
```

---


### Scheduled_action

ScheduledAction resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scheduled_action_description` | String |  | <p>The description of the scheduled action. 
            </p> |
| `iam_role` | String | ✅ | <p>The IAM role to assume to run the target action. 
            For more information about this parameter, see <a>ScheduledAction</a>. </p> |
| `enable` | bool |  | <p>If true, the schedule is enabled. If false, the scheduled action does not trigger. 
            For more information about <code>state</code> of the scheduled action, see <a>ScheduledAction</a>. </p> |
| `schedule` | String | ✅ | <p>The schedule in <code>at( )</code> or <code>cron( )</code> format. 
            For more information about this parameter, see <a>ScheduledAction</a>.</p> |
| `scheduled_action_name` | String | ✅ | <p>The name of the scheduled action. The name must be unique within an account. 
            For more information about this parameter, see <a>ScheduledAction</a>. </p> |
| `start_time` | String |  | <p>The start time in UTC of the scheduled action. 
            Before this time, the scheduled action does not trigger.
            For more information about this parameter, see <a>ScheduledAction</a>.</p> |
| `end_time` | String |  | <p>The end time in UTC of the scheduled action. After this time, the scheduled action does not trigger. 
            For more information about this parameter, see <a>ScheduledAction</a>. </p> |
| `target_action` | String | ✅ | <p>A JSON format string of the Amazon Redshift API operation with input parameters. 
            For more information about this parameter, see <a>ScheduledAction</a>. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create scheduled_action
scheduled_action = provider.redshift.Scheduled_action {
    iam_role = "value"  # <p>The IAM role to assume to run the target action. 
            For more information about this parameter, see <a>ScheduledAction</a>. </p>
    schedule = "value"  # <p>The schedule in <code>at( )</code> or <code>cron( )</code> format. 
            For more information about this parameter, see <a>ScheduledAction</a>.</p>
    scheduled_action_name = "value"  # <p>The name of the scheduled action. The name must be unique within an account. 
            For more information about this parameter, see <a>ScheduledAction</a>. </p>
    target_action = "value"  # <p>A JSON format string of the Amazon Redshift API operation with input parameters. 
            For more information about this parameter, see <a>ScheduledAction</a>. </p>
}

```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the resource of which its resource policy is updated.</p> |
| `policy` | String | ✅ | <p>The content of the resource policy being updated.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_policy` | String | <p>The content of the resource policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.redshift.Resource_policy {
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the resource of which its resource policy is updated.</p>
    policy = "value"  # <p>The content of the resource policy being updated.</p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_resource_policy = resource_policy.resource_policy
```

---


### Reserved_nodes

ReservedNodes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>Marker</code> parameter
            and retrying the command. If the <code>Marker</code> field is empty, all response
            records have been retrieved for the request. </p> |
| `reserved_nodes` | Vec<String> | <p>The list of <code>ReservedNode</code> objects.</p> |


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
reserved_nodes_marker = reserved_nodes.marker
reserved_nodes_reserved_nodes = reserved_nodes.reserved_nodes
```

---


### Data_shares_for_consumer

DataSharesForConsumer resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_shares` | Vec<String> | <p>Shows the results of datashares available for consumers.</p> |
| `marker` | String | <p>An optional parameter that specifies the starting point to return a set of response
            records. When the results of a <a>DescribeDataSharesForConsumer</a> request
            exceed the value specified in <code>MaxRecords</code>, Amazon Web Services returns a value in the
            <code>Marker</code> field of the response. You can retrieve the next set of response
            records by providing the returned marker value in the <code>Marker</code> parameter and
            retrying the request. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_shares_for_consumer outputs
data_shares_for_consumer_id = data_shares_for_consumer.id
data_shares_for_consumer_data_shares = data_shares_for_consumer.data_shares
data_shares_for_consumer_marker = data_shares_for_consumer.marker
```

---


### Cluster_security_groups

ClusterSecurityGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cluster_security_groups` | Vec<String> | <p>A list of <a>ClusterSecurityGroup</a> instances. </p> |
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>Marker</code> parameter
            and retrying the command. If the <code>Marker</code> field is empty, all response
            records have been retrieved for the request. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cluster_security_groups outputs
cluster_security_groups_id = cluster_security_groups.id
cluster_security_groups_cluster_security_groups = cluster_security_groups.cluster_security_groups
cluster_security_groups_marker = cluster_security_groups.marker
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
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>Marker</code> parameter
            and retrying the command. If the <code>Marker</code> field is empty, all response
            records have been retrieved for the request. </p> |
| `events` | Vec<String> | <p>A list of <code>Event</code> instances. </p> |


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


### Hsm_configuration

HsmConfiguration resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hsm_ip_address` | String | ✅ | <p>The IP address that the Amazon Redshift cluster must use to access the HSM.</p> |
| `hsm_partition_name` | String | ✅ | <p>The name of the partition in the HSM where the Amazon Redshift clusters will store their
            database encryption keys.</p> |
| `hsm_server_public_certificate` | String | ✅ | <p>The HSMs public certificate file. When using Cloud HSM, the file name is
            server.pem.</p> |
| `hsm_configuration_identifier` | String | ✅ | <p>The identifier to be assigned to the new Amazon Redshift HSM configuration.</p> |
| `tags` | Vec<String> |  | <p>A list of tag instances.</p> |
| `hsm_partition_password` | String | ✅ | <p>The password required to access the HSM partition.</p> |
| `description` | String | ✅ | <p>A text description of the HSM configuration to be created.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create hsm_configuration
hsm_configuration = provider.redshift.Hsm_configuration {
    hsm_ip_address = "value"  # <p>The IP address that the Amazon Redshift cluster must use to access the HSM.</p>
    hsm_partition_name = "value"  # <p>The name of the partition in the HSM where the Amazon Redshift clusters will store their
            database encryption keys.</p>
    hsm_server_public_certificate = "value"  # <p>The HSMs public certificate file. When using Cloud HSM, the file name is
            server.pem.</p>
    hsm_configuration_identifier = "value"  # <p>The identifier to be assigned to the new Amazon Redshift HSM configuration.</p>
    hsm_partition_password = "value"  # <p>The password required to access the HSM partition.</p>
    description = "value"  # <p>A text description of the HSM configuration to be created.</p>
}

```

---


### Event_subscription

EventSubscription resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of tag instances.</p> |
| `event_categories` | Vec<String> |  | <p>Specifies the Amazon Redshift event categories to be published by the event notification
            subscription.</p>
         <p>Values: configuration, management, monitoring, security, pending</p> |
| `subscription_name` | String | ✅ | <p>The name of the event subscription to be created.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Cannot be null, empty, or blank.</p>
            </li>
            <li>
               <p>Must contain from 1 to 255 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul> |
| `source_ids` | Vec<String> |  | <p>A list of one or more identifiers of Amazon Redshift source objects. All of the objects
            must be of the same type as was specified in the source type parameter. The event
            subscription will return only events generated by the specified objects. If not
            specified, then events are returned for all objects within the source type
            specified.</p>
         <p>Example: my-cluster-1, my-cluster-2</p>
         <p>Example: my-snapshot-20131010</p> |
| `sns_topic_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Amazon SNS topic used to transmit the event
            notifications. The ARN is created by Amazon SNS when you create a topic and subscribe to
            it.</p> |
| `enabled` | bool |  | <p>A boolean value; set to <code>true</code> to activate the subscription, and set to
                <code>false</code> to create the subscription but not activate it. </p> |
| `source_type` | String |  | <p>The type of source that will be generating the events. For example, if you want to
            be notified of events generated by a cluster, you would set this parameter to cluster.
            If this value is not specified, events are returned for all Amazon Redshift objects in your
            Amazon Web Services account. You must specify a source type in order to specify source IDs.</p>
         <p>Valid values: cluster, cluster-parameter-group, cluster-security-group, cluster-snapshot, and scheduled-action.</p> |
| `severity` | String |  | <p>Specifies the Amazon Redshift event severity to be published by the event notification
            subscription.</p>
         <p>Values: ERROR, INFO</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_subscription
event_subscription = provider.redshift.Event_subscription {
    subscription_name = "value"  # <p>The name of the event subscription to be created.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Cannot be null, empty, or blank.</p>
            </li>
            <li>
               <p>Must contain from 1 to 255 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
    sns_topic_arn = "value"  # <p>The Amazon Resource Name (ARN) of the Amazon SNS topic used to transmit the event
            notifications. The ARN is created by Amazon SNS when you create a topic and subscribe to
            it.</p>
}

```

---


### Inbound_integrations

InboundIntegrations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>Marker</code> parameter
            and retrying the command. If the <code>Marker</code> field is empty, all response
            records have been retrieved for the request. </p> |
| `inbound_integrations` | Vec<String> | <p>A list of <a>InboundIntegration</a> instances.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access inbound_integrations outputs
inbound_integrations_id = inbound_integrations.id
inbound_integrations_marker = inbound_integrations.marker
inbound_integrations_inbound_integrations = inbound_integrations.inbound_integrations
```

---


### Logging_status

LoggingStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_successful_delivery_time` | String | <p>The last time that logs were delivered.</p> |
| `bucket_name` | String | <p>The name of the S3 bucket where the log files are stored.</p> |
| `last_failure_time` | String | <p>The last time when logs failed to be delivered.</p> |
| `logging_enabled` | bool | <p>
            <code>true</code> if logging is on, <code>false</code> if logging is off.</p> |
| `last_failure_message` | String | <p>The message indicating that logs failed to be delivered.</p> |
| `log_exports` | Vec<String> | <p>The collection of exported log types. Possible values are <code>connectionlog</code>, <code>useractivitylog</code>, and 
            <code>userlog</code>.</p> |
| `s3_key_prefix` | String | <p>The prefix applied to the log file names.</p> |
| `log_destination_type` | String | <p>The log destination type. An enum with possible values of <code>s3</code> and <code>cloudwatch</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access logging_status outputs
logging_status_id = logging_status.id
logging_status_last_successful_delivery_time = logging_status.last_successful_delivery_time
logging_status_bucket_name = logging_status.bucket_name
logging_status_last_failure_time = logging_status.last_failure_time
logging_status_logging_enabled = logging_status.logging_enabled
logging_status_last_failure_message = logging_status.last_failure_message
logging_status_log_exports = logging_status.log_exports
logging_status_s3_key_prefix = logging_status.s3_key_prefix
logging_status_log_destination_type = logging_status.log_destination_type
```

---


### Cluster

Cluster resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `db_name` | String |  | <p>The name of the first database to be created when the cluster is created.</p>
         <p>To create additional databases after the cluster is created, connect to the cluster
            with a SQL client and use SQL commands to create a database. For more information, go to
                <a href="https://docs.aws.amazon.com/redshift/latest/dg/t_creating_database.html">Create
                a Database</a> in the Amazon Redshift Database Developer Guide. </p>
         <p>Default: <code>dev</code>
         </p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain 1 to 64 alphanumeric characters.</p>
            </li>
            <li>
               <p>Must contain only lowercase letters.</p>
            </li>
            <li>
               <p>Cannot be a word that is reserved by the service. A list of reserved words
                    can be found in <a href="https://docs.aws.amazon.com/redshift/latest/dg/r_pg_keywords.html">Reserved Words</a> in the
                    Amazon Redshift Database Developer Guide. </p>
            </li>
         </ul> |
| `multi_az` | bool |  | <p>If true, Amazon Redshift will deploy the cluster in two Availability Zones (AZ).</p> |
| `hsm_configuration_identifier` | String |  | <p>Specifies the name of the HSM configuration that contains the information the
            Amazon Redshift cluster can use to retrieve and store keys in an HSM.</p> |
| `availability_zone` | String |  | <p>The EC2 Availability Zone (AZ) in which you want Amazon Redshift to provision the
            cluster. For example, if you have several EC2 instances running in a specific
            Availability Zone, then you might want the cluster to be provisioned in the same zone in
            order to decrease network latency.</p>
         <p>Default: A random, system-chosen Availability Zone in the region that is specified
            by the endpoint.</p>
         <p>Example: <code>us-east-2d</code>
         </p>
         <p>Constraint: The specified Availability Zone must be in the same region as the
            current endpoint.</p> |
| `ip_address_type` | String |  | <p>The IP address types that the cluster supports. Possible values are <code>ipv4</code> and <code>dualstack</code>.</p> |
| `cluster_parameter_group_name` | String |  | <p>The name of the parameter group to be associated with this cluster.</p>
         <p>Default: The default Amazon Redshift cluster parameter group. For information about the
            default parameter group, go to <a href="https://docs.aws.amazon.com/redshift/latest/mgmt/working-with-parameter-groups.html">Working with Amazon
                Redshift Parameter Groups</a>
         </p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 1 to 255 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul> |
| `publicly_accessible` | bool |  | <p>If <code>true</code>, the cluster can be accessed from a public network. </p>
         <p>Default: false</p> |
| `port` | i64 |  | <p>The port number on which the cluster accepts incoming connections.</p>
         <p>The cluster is accessible only via the JDBC and ODBC connection strings. Part of
            the connection string requires the port on which the cluster will listen for incoming
            connections.</p>
         <p>Default: <code>5439</code>
         </p>
         <p>Valid Values:
        </p>
         <ul>
            <li>
               <p>For clusters with ra3 nodes - Select a port within the ranges <code>5431-5455</code> or <code>8191-8215</code>. (If you have an existing cluster 
                with ra3 nodes, it isn't required that you change the port to these ranges.)</p>
            </li>
            <li>
               <p>For clusters with dc2 nodes - Select a port within the range <code>1150-65535</code>.</p>
            </li>
         </ul> |
| `cluster_version` | String |  | <p>The version of the Amazon Redshift engine software that you want to deploy on the
            cluster.</p>
         <p>The version selected runs on all the nodes in the cluster.</p>
         <p>Constraints: Only version 1.0 is currently available.</p>
         <p>Example: <code>1.0</code>
         </p> |
| `load_sample_data` | String |  | <p>A flag that specifies whether to load sample data once the cluster is created.</p> |
| `cluster_subnet_group_name` | String |  | <p>The name of a cluster subnet group to be associated with this cluster.</p>
         <p>If this parameter is not provided the resulting cluster will be deployed outside
            virtual private cloud (VPC).</p> |
| `cluster_security_groups` | Vec<String> |  | <p>A list of security groups to be associated with this cluster.</p>
         <p>Default: The default cluster security group for Amazon Redshift.</p> |
| `allow_version_upgrade` | bool |  | <p>If <code>true</code>, major version upgrades can be applied during the maintenance
            window to the Amazon Redshift engine that is running on the cluster.</p>
         <p>When a new major version of the Amazon Redshift engine is released, you can request that
            the service automatically apply upgrades during the maintenance window to the Amazon Redshift
            engine that is running on your cluster.</p>
         <p>Default: <code>true</code>
         </p> |
| `number_of_nodes` | i64 |  | <p>The number of compute nodes in the cluster. This parameter is required when the
                <b>ClusterType</b> parameter is specified as
                <code>multi-node</code>. </p>
         <p>For information about determining how many nodes you need, go to <a href="https://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html#how-many-nodes"> Working with
                Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
         <p>If you don't specify this parameter, you get a single-node cluster. When requesting
            a multi-node cluster, you must specify the number of nodes that you want in the
            cluster.</p>
         <p>Default: <code>1</code>
         </p>
         <p>Constraints: Value must be at least 1 and no more than 100.</p> |
| `tags` | Vec<String> |  | <p>A list of tag instances.</p> |
| `kms_key_id` | String |  | <p>The Key Management Service (KMS) key ID of the encryption key that you want to
            use to encrypt data in the cluster.</p> |
| `snapshot_schedule_identifier` | String |  | <p>A unique identifier for the snapshot schedule.</p> |
| `aqua_configuration_status` | String |  | <p>This parameter is retired. It does not set the AQUA configuration status. Amazon Redshift automatically determines whether to use AQUA (Advanced Query Accelerator).</p> |
| `redshift_idc_application_arn` | String |  | <p>The Amazon resource name (ARN) of the Amazon Redshift IAM Identity Center application.</p> |
| `encrypted` | bool |  | <p>If <code>true</code>, the data in the cluster is encrypted at rest. 
            If you set the value on this parameter to <code>false</code>, the request will fail.</p>
         <p>Default: true</p> |
| `cluster_identifier` | String | ✅ | <p>A unique identifier for the cluster. You use this identifier to refer to the
            cluster for any subsequent cluster operations such as deleting or modifying. The
            identifier also appears in the Amazon Redshift console.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>Alphabetic characters must be lowercase.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
            <li>
               <p>Must be unique for all clusters within an Amazon Web Services account.</p>
            </li>
         </ul>
         <p>Example: <code>myexamplecluster</code>
         </p> |
| `iam_roles` | Vec<String> |  | <p>A list of Identity and Access Management (IAM) roles that can be used by the
            cluster to access other Amazon Web Services services. You must supply the IAM roles in their Amazon
            Resource Name (ARN) format. </p>
         <p>The maximum number of IAM roles that you can associate is subject to a quota.
            For more information, go to <a href="https://docs.aws.amazon.com/redshift/latest/mgmt/amazon-redshift-limits.html">Quotas and limits</a>
            in the <i>Amazon Redshift Cluster Management Guide</i>.</p> |
| `master_password_secret_kms_key_id` | String |  | <p>The ID of the Key Management Service (KMS) key used to encrypt and store the cluster's admin credentials secret. 
            You can only use this parameter if <code>ManageMasterPassword</code> is true.</p> |
| `master_username` | String | ✅ | <p>The user name associated with the admin user account for the cluster that is being
            created.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 1 - 128 alphanumeric characters or hyphens. The user name can't be
                        <code>PUBLIC</code>.</p>
            </li>
            <li>
               <p>Must contain only lowercase letters, numbers, underscore, plus sign, period (dot), at symbol (@), or hyphen.</p>
            </li>
            <li>
               <p>The first character must be a letter.</p>
            </li>
            <li>
               <p>Must not contain a colon (:) or a slash (/).</p>
            </li>
            <li>
               <p>Cannot be a reserved word. A list of reserved words can be found in <a href="https://docs.aws.amazon.com/redshift/latest/dg/r_pg_keywords.html">Reserved
                        Words</a> in the Amazon Redshift Database Developer Guide. </p>
            </li>
         </ul> |
| `preferred_maintenance_window` | String |  | <p>The weekly time range (in UTC) during which automated cluster maintenance can
            occur.</p>
         <p> Format: <code>ddd:hh24:mi-ddd:hh24:mi</code>
         </p>
         <p> Default: A 30-minute window selected at random from an 8-hour block of time per
            region, occurring on a random day of the week. For more information about the time
            blocks for each region, see <a href="https://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html#rs-maintenance-windows">Maintenance Windows</a> in Amazon Redshift Cluster Management Guide.</p>
         <p>Valid Days: Mon | Tue | Wed | Thu | Fri | Sat | Sun</p>
         <p>Constraints: Minimum 30-minute window.</p> |
| `automated_snapshot_retention_period` | i64 |  | <p>The number of days that automated snapshots are retained. If the value is 0,
            automated snapshots are disabled. Even if automated snapshots are disabled, you can
            still create manual snapshots when you want with <a>CreateClusterSnapshot</a>. </p>
         <p>You can't disable automated snapshots for RA3 node types. Set the automated retention period from 1-35 days.</p>
         <p>Default: <code>1</code>
         </p>
         <p>Constraints: Must be a value from 0 to 35.</p> |
| `availability_zone_relocation` | bool |  | <p>The option to enable relocation for an Amazon Redshift cluster between Availability Zones after the cluster is created.</p> |
| `additional_info` | String |  | <p>Reserved.</p> |
| `manage_master_password` | bool |  | <p>If <code>true</code>, Amazon Redshift uses Secrets Manager to manage this cluster's admin credentials. 
            You can't use <code>MasterUserPassword</code> if <code>ManageMasterPassword</code> is true. 
            If <code>ManageMasterPassword</code> is false or not set, Amazon Redshift uses 
            <code>MasterUserPassword</code> for the admin user account's password.
        </p> |
| `vpc_security_group_ids` | Vec<String> |  | <p>A list of Virtual Private Cloud (VPC) security groups to be associated with the
            cluster.</p>
         <p>Default: The default VPC security group is associated with the cluster.</p> |
| `hsm_client_certificate_identifier` | String |  | <p>Specifies the name of the HSM client certificate the Amazon Redshift cluster uses to
            retrieve the data encryption keys stored in an HSM.</p> |
| `manual_snapshot_retention_period` | i64 |  | <p>The default number of days to retain a manual snapshot. If the value is -1, the
            snapshot is retained indefinitely. This setting doesn't change the retention period
            of existing snapshots.</p>
         <p>The value must be either -1 or an integer between 1 and 3,653.</p> |
| `maintenance_track_name` | String |  | <p>An optional parameter for the name of the maintenance track for the cluster. If you
            don't provide a maintenance track name, the cluster is assigned to the
                <code>current</code> track.</p> |
| `node_type` | String | ✅ | <p>The node type to be provisioned for the cluster. For information about node types,
            go to <a href="https://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html#how-many-nodes"> Working with
                Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
         <p>Valid Values: 
            <code>dc2.large</code> | <code>dc2.8xlarge</code> | 
            <code>ra3.large</code> |  <code>ra3.xlplus</code> |  <code>ra3.4xlarge</code> | <code>ra3.16xlarge</code>
         </p> |
| `master_user_password` | String |  | <p>The password associated with the admin user account for the cluster that is being
            created.</p>
         <p>You can't use <code>MasterUserPassword</code> if <code>ManageMasterPassword</code> is <code>true</code>.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be between 8 and 64 characters in length.</p>
            </li>
            <li>
               <p>Must contain at least one uppercase letter.</p>
            </li>
            <li>
               <p>Must contain at least one lowercase letter.</p>
            </li>
            <li>
               <p>Must contain one number.</p>
            </li>
            <li>
               <p>Can be any printable ASCII character (ASCII code 33-126) except <code>'</code>
                    (single quote), <code>"</code> (double quote), <code>\</code>, <code>/</code>, or <code>@</code>.</p>
            </li>
         </ul> |
| `cluster_type` | String |  | <p>The type of the cluster. When cluster type is specified as</p>
         <ul>
            <li>
               <p>
                  <code>single-node</code>, the <b>NumberOfNodes</b>
                    parameter is not required.</p>
            </li>
            <li>
               <p>
                  <code>multi-node</code>, the <b>NumberOfNodes</b>
                    parameter is required.</p>
            </li>
         </ul>
         <p>Valid Values: <code>multi-node</code> | <code>single-node</code>
         </p>
         <p>Default: <code>multi-node</code>
         </p> |
| `default_iam_role_arn` | String |  | <p>The Amazon Resource Name (ARN) for the IAM role that was set as default for the cluster when the cluster was created. </p> |
| `elastic_ip` | String |  | <p>The Elastic IP (EIP) address for the cluster.</p>
         <p>Constraints: The cluster must be provisioned in EC2-VPC and publicly-accessible
            through an Internet gateway. Don't specify the Elastic IP address for a publicly accessible 
            cluster with availability zone relocation turned on. For more information about provisioning clusters in
            EC2-VPC, go to <a href="https://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html#cluster-platforms">Supported
                Platforms to Launch Your Cluster</a> in the Amazon Redshift Cluster Management Guide.</p> |
| `enhanced_vpc_routing` | bool |  | <p>An option that specifies whether to create the cluster with enhanced VPC routing
            enabled. To create a cluster that uses enhanced VPC routing, the cluster must be in a
            VPC. For more information, see <a href="https://docs.aws.amazon.com/redshift/latest/mgmt/enhanced-vpc-routing.html">Enhanced VPC Routing</a> in
            the Amazon Redshift Cluster Management Guide.</p>
         <p>If this option is <code>true</code>, enhanced VPC routing is enabled. </p>
         <p>Default: false</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster
cluster = provider.redshift.Cluster {
    cluster_identifier = "value"  # <p>A unique identifier for the cluster. You use this identifier to refer to the
            cluster for any subsequent cluster operations such as deleting or modifying. The
            identifier also appears in the Amazon Redshift console.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 1 to 63 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>Alphabetic characters must be lowercase.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
            <li>
               <p>Must be unique for all clusters within an Amazon Web Services account.</p>
            </li>
         </ul>
         <p>Example: <code>myexamplecluster</code>
         </p>
    master_username = "value"  # <p>The user name associated with the admin user account for the cluster that is being
            created.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be 1 - 128 alphanumeric characters or hyphens. The user name can't be
                        <code>PUBLIC</code>.</p>
            </li>
            <li>
               <p>Must contain only lowercase letters, numbers, underscore, plus sign, period (dot), at symbol (@), or hyphen.</p>
            </li>
            <li>
               <p>The first character must be a letter.</p>
            </li>
            <li>
               <p>Must not contain a colon (:) or a slash (/).</p>
            </li>
            <li>
               <p>Cannot be a reserved word. A list of reserved words can be found in <a href="https://docs.aws.amazon.com/redshift/latest/dg/r_pg_keywords.html">Reserved
                        Words</a> in the Amazon Redshift Database Developer Guide. </p>
            </li>
         </ul>
    node_type = "value"  # <p>The node type to be provisioned for the cluster. For information about node types,
            go to <a href="https://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html#how-many-nodes"> Working with
                Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
         <p>Valid Values: 
            <code>dc2.large</code> | <code>dc2.8xlarge</code> | 
            <code>ra3.large</code> |  <code>ra3.xlplus</code> |  <code>ra3.4xlarge</code> | <code>ra3.16xlarge</code>
         </p>
}

```

---


### Node_configuration_options

NodeConfigurationOptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>Marker</code> parameter
            and retrying the command. If the <code>Marker</code> field is empty, all response
            records have been retrieved for the request. </p> |
| `node_configuration_option_list` | Vec<String> | <p>A list of valid node configurations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access node_configuration_options outputs
node_configuration_options_id = node_configuration_options.id
node_configuration_options_marker = node_configuration_options.marker
node_configuration_options_node_configuration_option_list = node_configuration_options.node_configuration_option_list
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
| `event_categories_map_list` | Vec<String> | <p>A list of event categories descriptions.</p> |


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


### Storage

Storage resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_backup_size_in_mega_bytes` | f64 | <p>The total amount of storage currently used for snapshots.</p> |
| `total_provisioned_storage_in_mega_bytes` | f64 | <p>The total amount of storage currently provisioned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access storage outputs
storage_id = storage.id
storage_total_backup_size_in_mega_bytes = storage.total_backup_size_in_mega_bytes
storage_total_provisioned_storage_in_mega_bytes = storage.total_provisioned_storage_in_mega_bytes
```

---


### Snapshot_schedule

SnapshotSchedule resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>An optional set of tags you can use to search for the schedule.</p> |
| `dry_run` | bool |  | <p></p> |
| `schedule_definitions` | Vec<String> |  | <p>The definition of the snapshot schedule. The definition is made up of schedule
            expressions, for example "cron(30 12 *)" or "rate(12 hours)". </p> |
| `schedule_identifier` | String |  | <p>A unique identifier for a snapshot schedule. Only alphanumeric characters are allowed
            for the identifier.</p> |
| `schedule_description` | String |  | <p>The description of the snapshot schedule.</p> |
| `next_invocations` | i64 |  | <p></p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create snapshot_schedule
snapshot_schedule = provider.redshift.Snapshot_schedule {
}

```

---


### Table_restore_status

TableRestoreStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `table_restore_status_details` | Vec<String> | <p>A list of status details for one or more table restore requests.</p> |
| `marker` | String | <p>A pagination token that can be used in a subsequent <a>DescribeTableRestoreStatus</a> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access table_restore_status outputs
table_restore_status_id = table_restore_status.id
table_restore_status_table_restore_status_details = table_restore_status.table_restore_status_details
table_restore_status_marker = table_restore_status.marker
```

---


### Cluster_subnet_group

ClusterSubnetGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cluster_subnet_group_name` | String | ✅ | <p>The name for the subnet group. Amazon Redshift stores the value as a lowercase
            string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain no more than 255 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>Must not be "Default".</p>
            </li>
            <li>
               <p>Must be unique for all subnet groups that are created by your Amazon Web Services account.</p>
            </li>
         </ul>
         <p>Example: <code>examplesubnetgroup</code>
         </p> |
| `description` | String | ✅ | <p>A description for the subnet group.</p> |
| `subnet_ids` | Vec<String> | ✅ | <p>An array of VPC subnet IDs. A maximum of 20 subnets can be modified in a single
            request.</p> |
| `tags` | Vec<String> |  | <p>A list of tag instances.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster_subnet_group
cluster_subnet_group = provider.redshift.Cluster_subnet_group {
    cluster_subnet_group_name = "value"  # <p>The name for the subnet group. Amazon Redshift stores the value as a lowercase
            string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain no more than 255 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>Must not be "Default".</p>
            </li>
            <li>
               <p>Must be unique for all subnet groups that are created by your Amazon Web Services account.</p>
            </li>
         </ul>
         <p>Example: <code>examplesubnetgroup</code>
         </p>
    description = "value"  # <p>A description for the subnet group.</p>
    subnet_ids = "value"  # <p>An array of VPC subnet IDs. A maximum of 20 subnets can be modified in a single
            request.</p>
}

```

---


### Cluster_subnet_groups

ClusterSubnetGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cluster_subnet_groups` | Vec<String> | <p>A list of <a>ClusterSubnetGroup</a> instances. </p> |
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a
            subsequent request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the <code>Marker</code> parameter
            and retrying the command. If the <code>Marker</code> field is empty, all response
            records have been retrieved for the request. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cluster_subnet_groups outputs
cluster_subnet_groups_id = cluster_subnet_groups.id
cluster_subnet_groups_cluster_subnet_groups = cluster_subnet_groups.cluster_subnet_groups
cluster_subnet_groups_marker = cluster_subnet_groups.marker
```

---


### Reserved_node_exchange_configuration_options

ReservedNodeExchangeConfigurationOptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reserved_node_configuration_option_list` | Vec<String> | <p>the configuration options for the reserved-node
            exchange. These options include information about the source reserved node and target reserved
            node. Details include the node type, the price, the node count, and the offering
            type.</p> |
| `marker` | String | <p>A pagination token provided by a previous <code>GetReservedNodeExchangeConfigurationOptions</code> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_node_exchange_configuration_options outputs
reserved_node_exchange_configuration_options_id = reserved_node_exchange_configuration_options.id
reserved_node_exchange_configuration_options_reserved_node_configuration_option_list = reserved_node_exchange_configuration_options.reserved_node_configuration_option_list
reserved_node_exchange_configuration_options_marker = reserved_node_exchange_configuration_options.marker
```

---


### Authentication_profiles

AuthenticationProfiles resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `authentication_profiles` | Vec<String> | <p>The list of authentication profiles.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access authentication_profiles outputs
authentication_profiles_id = authentication_profiles.id
authentication_profiles_authentication_profiles = authentication_profiles.authentication_profiles
```

---


### Redshift_idc_applications

RedshiftIdcApplications resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a subsequent 
            request. If a value is returned in a response, you can retrieve the next set
            of records by providing this returned marker value in the Marker parameter
            and retrying the command. If the Marker field is empty, all response
            records have been retrieved for the request.
        </p> |
| `redshift_idc_applications` | Vec<String> | <p>The list of Amazon Redshift IAM Identity Center applications.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access redshift_idc_applications outputs
redshift_idc_applications_id = redshift_idc_applications.id
redshift_idc_applications_marker = redshift_idc_applications.marker
redshift_idc_applications_redshift_idc_applications = redshift_idc_applications.redshift_idc_applications
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple cluster_snapshots resources
cluster_snapshots_0 = provider.redshift.Cluster_snapshots {
}
cluster_snapshots_1 = provider.redshift.Cluster_snapshots {
}
cluster_snapshots_2 = provider.redshift.Cluster_snapshots {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    cluster_snapshots = provider.redshift.Cluster_snapshots {
    }
```

---

## Related Documentation

- [AWS Redshift Documentation](https://docs.aws.amazon.com/redshift/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
