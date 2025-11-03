# Emr Service



**Resources**: 16

---

## Overview

The emr service provides access to 16 resource types:

- [Security_configuration](#security_configuration) [CRD]
- [Auto_termination_policy](#auto_termination_policy) [CR]
- [Cluster](#cluster) [R]
- [Persistent_app_ui](#persistent_app_ui) [CR]
- [Notebook_execution](#notebook_execution) [R]
- [Studio_session_mapping](#studio_session_mapping) [CRUD]
- [Persistent_app_ui_presigned_url](#persistent_app_ui_presigned_url) [R]
- [Managed_scaling_policy](#managed_scaling_policy) [CR]
- [Step](#step) [R]
- [On_cluster_app_ui_presigned_url](#on_cluster_app_ui_presigned_url) [R]
- [Studio](#studio) [CRUD]
- [Block_public_access_configuration](#block_public_access_configuration) [CR]
- [Cluster_session_credentials](#cluster_session_credentials) [R]
- [Auto_scaling_policy](#auto_scaling_policy) [C]
- [Release_label](#release_label) [R]
- [Job_flows](#job_flows) [R]

---

## Resources


### Security_configuration

SecurityConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `security_configuration` | String | ✅ | <p>The security configuration details in JSON format. For JSON parameters and examples, see
            <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-security-configurations.html">Use Security
            Configurations to Set Up Cluster Security</a> in the <i>Amazon EMR
            Management Guide</i>.</p> |
| `name` | String | ✅ | <p>The name of the security configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `security_configuration` | String | <p>The security configuration details in JSON format.</p> |
| `creation_date_time` | String | <p>The date and time the security configuration was created</p> |
| `name` | String | <p>The name of the security configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create security_configuration
security_configuration = provider.emr.Security_configuration {
    security_configuration = "value"  # <p>The security configuration details in JSON format. For JSON parameters and examples, see
            <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-security-configurations.html">Use Security
            Configurations to Set Up Cluster Security</a> in the <i>Amazon EMR
            Management Guide</i>.</p>
    name = "value"  # <p>The name of the security configuration.</p>
}

# Access security_configuration outputs
security_configuration_id = security_configuration.id
security_configuration_security_configuration = security_configuration.security_configuration
security_configuration_creation_date_time = security_configuration.creation_date_time
security_configuration_name = security_configuration.name
```

---


### Auto_termination_policy

AutoTerminationPolicy resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cluster_id` | String | ✅ | <p>Specifies the ID of the Amazon EMR cluster to which the auto-termination policy
         will be attached.</p> |
| `auto_termination_policy` | String |  | <p>Specifies the auto-termination policy to attach to the cluster.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auto_termination_policy` | String | <p>Specifies the auto-termination policy that is attached to an Amazon EMR cluster.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create auto_termination_policy
auto_termination_policy = provider.emr.Auto_termination_policy {
    cluster_id = "value"  # <p>Specifies the ID of the Amazon EMR cluster to which the auto-termination policy
         will be attached.</p>
}

# Access auto_termination_policy outputs
auto_termination_policy_id = auto_termination_policy.id
auto_termination_policy_auto_termination_policy = auto_termination_policy.auto_termination_policy
```

---


### Cluster

Cluster resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cluster` | String | <p>This output contains the details for the requested cluster.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cluster outputs
cluster_id = cluster.id
cluster_cluster = cluster.cluster
```

---


### Persistent_app_ui

PersistentAppUI resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target_resource_arn` | String | ✅ | <p>The unique Amazon Resource Name (ARN) of the target resource.</p> |
| `emr_containers_config` | String |  | <p>The EMR containers configuration.</p> |
| `tags` | Vec<String> |  | <p>Tags for the persistent application user interface.</p> |
| `x_referer` | String |  | <p>The cross reference for the persistent application user interface.</p> |
| `profiler_type` | String |  | <p>The profiler type for the persistent application user interface.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `persistent_app_ui` | String | <p>The persistent application user interface.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create persistent_app_ui
persistent_app_ui = provider.emr.Persistent_app_ui {
    target_resource_arn = "value"  # <p>The unique Amazon Resource Name (ARN) of the target resource.</p>
}

# Access persistent_app_ui outputs
persistent_app_ui_id = persistent_app_ui.id
persistent_app_ui_persistent_app_ui = persistent_app_ui.persistent_app_ui
```

---


### Notebook_execution

NotebookExecution resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notebook_execution` | String | <p>Properties of the notebook execution.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access notebook_execution outputs
notebook_execution_id = notebook_execution.id
notebook_execution_notebook_execution = notebook_execution.notebook_execution
```

---


### Studio_session_mapping

StudioSessionMapping resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `studio_id` | String | ✅ | <p>The ID of the Amazon EMR Studio to which the user or group will be
         mapped.</p> |
| `identity_type` | String | ✅ | <p>Specifies whether the identity to map to the Amazon EMR Studio is a user or a
         group.</p> |
| `session_policy_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) for the session policy that will be applied to the user
         or group. You should specify the ARN for the session policy that you want to apply, not the
         ARN of your user role. For more information, see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-studio-user-role.html">Create an Amazon EMR
            Studio User Role with Session Policies</a>.</p> |
| `identity_name` | String |  | <p>The name of the user or group. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/API_User.html#singlesignon-Type-User-UserName">UserName</a> and <a href="https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/API_Group.html#singlesignon-Type-Group-DisplayName">DisplayName</a> in the <i>IAM Identity Center Identity Store API
            Reference</i>. Either <code>IdentityName</code> or <code>IdentityId</code> must
         be specified, but not both.</p> |
| `identity_id` | String |  | <p>The globally unique identifier (GUID) of the user or group from the IAM Identity Center
         Identity Store. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/API_User.html#singlesignon-Type-User-UserId">UserId</a> and <a href="https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/API_Group.html#singlesignon-Type-Group-GroupId">GroupId</a> in the <i>IAM Identity Center Identity Store API
            Reference</i>. Either <code>IdentityName</code> or <code>IdentityId</code> must
         be specified, but not both.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `session_mapping` | String | <p>The session mapping details for the specified Amazon EMR Studio and identity,
         including session policy ARN and creation time.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create studio_session_mapping
studio_session_mapping = provider.emr.Studio_session_mapping {
    studio_id = "value"  # <p>The ID of the Amazon EMR Studio to which the user or group will be
         mapped.</p>
    identity_type = "value"  # <p>Specifies whether the identity to map to the Amazon EMR Studio is a user or a
         group.</p>
    session_policy_arn = "value"  # <p>The Amazon Resource Name (ARN) for the session policy that will be applied to the user
         or group. You should specify the ARN for the session policy that you want to apply, not the
         ARN of your user role. For more information, see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-studio-user-role.html">Create an Amazon EMR
            Studio User Role with Session Policies</a>.</p>
}

# Access studio_session_mapping outputs
studio_session_mapping_id = studio_session_mapping.id
studio_session_mapping_session_mapping = studio_session_mapping.session_mapping
```

---


### Persistent_app_ui_presigned_url

PersistentAppUIPresignedURL resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `presigned_url_ready` | bool | <p>Used to determine if the presigned URL is ready.</p> |
| `presigned_url` | String | <p>The returned presigned URL.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access persistent_app_ui_presigned_url outputs
persistent_app_ui_presigned_url_id = persistent_app_ui_presigned_url.id
persistent_app_ui_presigned_url_presigned_url_ready = persistent_app_ui_presigned_url.presigned_url_ready
persistent_app_ui_presigned_url_presigned_url = persistent_app_ui_presigned_url.presigned_url
```

---


### Managed_scaling_policy

ManagedScalingPolicy resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `managed_scaling_policy` | String | ✅ | <p>Specifies the constraints for the managed scaling policy. </p> |
| `cluster_id` | String | ✅ | <p>Specifies the ID of an Amazon EMR cluster where the managed scaling policy is
         attached. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `managed_scaling_policy` | String | <p>Specifies the managed scaling policy that is attached to an Amazon EMR cluster.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create managed_scaling_policy
managed_scaling_policy = provider.emr.Managed_scaling_policy {
    managed_scaling_policy = "value"  # <p>Specifies the constraints for the managed scaling policy. </p>
    cluster_id = "value"  # <p>Specifies the ID of an Amazon EMR cluster where the managed scaling policy is
         attached. </p>
}

# Access managed_scaling_policy outputs
managed_scaling_policy_id = managed_scaling_policy.id
managed_scaling_policy_managed_scaling_policy = managed_scaling_policy.managed_scaling_policy
```

---


### Step

Step resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `step` | String | <p>The step details for the requested step identifier.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access step outputs
step_id = step.id
step_step = step.step
```

---


### On_cluster_app_ui_presigned_url

OnClusterAppUIPresignedURL resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `presigned_url_ready` | bool | <p>Used to determine if the presigned URL is ready.</p> |
| `presigned_url` | String | <p>The cluster's generated presigned URL.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access on_cluster_app_ui_presigned_url outputs
on_cluster_app_ui_presigned_url_id = on_cluster_app_ui_presigned_url.id
on_cluster_app_ui_presigned_url_presigned_url_ready = on_cluster_app_ui_presigned_url.presigned_url_ready
on_cluster_app_ui_presigned_url_presigned_url = on_cluster_app_ui_presigned_url.presigned_url
```

---


### Studio

Studio resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vpc_id` | String | ✅ | <p>The ID of the Amazon Virtual Private Cloud (Amazon VPC) to associate with the
         Studio.</p> |
| `engine_security_group_id` | String | ✅ | <p>The ID of the Amazon EMR Studio Engine security group. The Engine security group
         allows inbound network traffic from the Workspace security group, and it must be in the
         same VPC specified by <code>VpcId</code>.</p> |
| `trusted_identity_propagation_enabled` | bool |  | <p>
         A Boolean indicating whether to enable Trusted identity propagation for the Studio. The default value is <code>false</code>.
      </p> |
| `idp_relay_state_parameter_name` | String |  | <p>The name that your identity provider (IdP) uses for its <code>RelayState</code>
         parameter. For example, <code>RelayState</code> or <code>TargetSource</code>. Specify this
         value when you use IAM authentication and want to let federated users log in
         to a Studio using the Studio URL. The <code>RelayState</code> parameter differs by
         IdP.</p> |
| `subnet_ids` | Vec<String> | ✅ | <p>A list of subnet IDs to associate with the Amazon EMR Studio. A Studio can have
         a maximum of 5 subnets. The subnets must belong to the VPC specified by <code>VpcId</code>.
         Studio users can create a Workspace in any of the specified subnets.</p> |
| `idp_auth_url` | String |  | <p>The authentication endpoint of your identity provider (IdP). Specify this value when you
         use IAM authentication and want to let federated users log in to a Studio
         with the Studio URL and credentials from your IdP. Amazon EMR Studio redirects
         users to this endpoint to enter credentials.</p> |
| `idc_instance_arn` | String |  | <p>
         The ARN of the IAM Identity Center instance to create the Studio application.
      </p> |
| `service_role` | String | ✅ | <p>The IAM role that the Amazon EMR Studio assumes. The service role
         provides a way for Amazon EMR Studio to interoperate with other Amazon Web Services
         services.</p> |
| `description` | String |  | <p>A detailed description of the Amazon EMR Studio.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to associate with the Amazon EMR Studio. Tags are user-defined
         key-value pairs that consist of a required key string with a maximum of 128 characters, and
         an optional value string with a maximum of 256 characters.</p> |
| `idc_user_assignment` | String |  | <p>
         Specifies whether IAM Identity Center user assignment is <code>REQUIRED</code> or <code>OPTIONAL</code>. If the value is set to <code>REQUIRED</code>, users must be explicitly assigned to the Studio application to access the Studio.
      </p> |
| `encryption_key_arn` | String |  | <p>The KMS key identifier (ARN) used to encrypt Amazon EMR Studio workspace and notebook files when backed up to Amazon S3.</p> |
| `user_role` | String |  | <p>The IAM user role that users and groups assume when logged in to an
            Amazon EMR Studio. Only specify a <code>UserRole</code> when you use IAM Identity Center authentication. The permissions attached to the <code>UserRole</code> can be
         scoped down for each user or group using session policies.</p> |
| `workspace_security_group_id` | String | ✅ | <p>The ID of the Amazon EMR Studio Workspace security group. The Workspace security
         group allows outbound network traffic to resources in the Engine security group, and it
         must be in the same VPC specified by <code>VpcId</code>.</p> |
| `name` | String | ✅ | <p>A descriptive name for the Amazon EMR Studio.</p> |
| `default_s3_location` | String | ✅ | <p>The Amazon S3 location to back up Amazon EMR Studio Workspaces and
         notebook files.</p> |
| `auth_mode` | String | ✅ | <p>Specifies whether the Studio authenticates users using IAM or IAM Identity Center.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `studio` | String | <p>The Amazon EMR Studio details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create studio
studio = provider.emr.Studio {
    vpc_id = "value"  # <p>The ID of the Amazon Virtual Private Cloud (Amazon VPC) to associate with the
         Studio.</p>
    engine_security_group_id = "value"  # <p>The ID of the Amazon EMR Studio Engine security group. The Engine security group
         allows inbound network traffic from the Workspace security group, and it must be in the
         same VPC specified by <code>VpcId</code>.</p>
    subnet_ids = "value"  # <p>A list of subnet IDs to associate with the Amazon EMR Studio. A Studio can have
         a maximum of 5 subnets. The subnets must belong to the VPC specified by <code>VpcId</code>.
         Studio users can create a Workspace in any of the specified subnets.</p>
    service_role = "value"  # <p>The IAM role that the Amazon EMR Studio assumes. The service role
         provides a way for Amazon EMR Studio to interoperate with other Amazon Web Services
         services.</p>
    workspace_security_group_id = "value"  # <p>The ID of the Amazon EMR Studio Workspace security group. The Workspace security
         group allows outbound network traffic to resources in the Engine security group, and it
         must be in the same VPC specified by <code>VpcId</code>.</p>
    name = "value"  # <p>A descriptive name for the Amazon EMR Studio.</p>
    default_s3_location = "value"  # <p>The Amazon S3 location to back up Amazon EMR Studio Workspaces and
         notebook files.</p>
    auth_mode = "value"  # <p>Specifies whether the Studio authenticates users using IAM or IAM Identity Center.</p>
}

# Access studio outputs
studio_id = studio.id
studio_studio = studio.studio
```

---


### Block_public_access_configuration

BlockPublicAccessConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `block_public_access_configuration` | String | ✅ | <p>A configuration for Amazon EMR block public access. The configuration applies to
         all clusters created in your account for the current Region. The configuration specifies
         whether block public access is enabled. If block public access is enabled, security groups
         associated with the cluster cannot have rules that allow inbound traffic from 0.0.0.0/0 or
         ::/0 on a port, unless the port is specified as an exception using
            <code>PermittedPublicSecurityGroupRuleRanges</code> in the
            <code>BlockPublicAccessConfiguration</code>. By default, Port 22 (SSH) is an exception,
         and public access is allowed on this port. You can change this by updating
            <code>BlockPublicSecurityGroupRules</code> to remove the exception.</p>
         <note>
            <p>For accounts that created clusters in a Region before November 25, 2019, block public
            access is disabled by default in that Region. To use this feature, you must manually
            enable and configure it. For accounts that did not create an Amazon EMR cluster
            in a Region before this date, block public access is enabled by default in that
            Region.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `block_public_access_configuration` | String | <p>A configuration for Amazon EMR block public access. The configuration applies to
         all clusters created in your account for the current Region. The configuration specifies
         whether block public access is enabled. If block public access is enabled, security groups
         associated with the cluster cannot have rules that allow inbound traffic from 0.0.0.0/0 or
         ::/0 on a port, unless the port is specified as an exception using
            <code>PermittedPublicSecurityGroupRuleRanges</code> in the
            <code>BlockPublicAccessConfiguration</code>. By default, Port 22 (SSH) is an exception,
         and public access is allowed on this port. You can change this by updating the block public
         access configuration to remove the exception.</p>
         <note>
            <p>For accounts that created clusters in a Region before November 25, 2019, block public
            access is disabled by default in that Region. To use this feature, you must manually
            enable and configure it. For accounts that did not create an Amazon EMR cluster
            in a Region before this date, block public access is enabled by default in that
            Region.</p>
         </note> |
| `block_public_access_configuration_metadata` | String | <p>Properties that describe the Amazon Web Services principal that created the
            <code>BlockPublicAccessConfiguration</code> using the
            <code>PutBlockPublicAccessConfiguration</code> action as well as the date and time that
         the configuration was created. Each time a configuration for block public access is
         updated, Amazon EMR updates this metadata.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create block_public_access_configuration
block_public_access_configuration = provider.emr.Block_public_access_configuration {
    block_public_access_configuration = "value"  # <p>A configuration for Amazon EMR block public access. The configuration applies to
         all clusters created in your account for the current Region. The configuration specifies
         whether block public access is enabled. If block public access is enabled, security groups
         associated with the cluster cannot have rules that allow inbound traffic from 0.0.0.0/0 or
         ::/0 on a port, unless the port is specified as an exception using
            <code>PermittedPublicSecurityGroupRuleRanges</code> in the
            <code>BlockPublicAccessConfiguration</code>. By default, Port 22 (SSH) is an exception,
         and public access is allowed on this port. You can change this by updating
            <code>BlockPublicSecurityGroupRules</code> to remove the exception.</p>
         <note>
            <p>For accounts that created clusters in a Region before November 25, 2019, block public
            access is disabled by default in that Region. To use this feature, you must manually
            enable and configure it. For accounts that did not create an Amazon EMR cluster
            in a Region before this date, block public access is enabled by default in that
            Region.</p>
         </note>
}

# Access block_public_access_configuration outputs
block_public_access_configuration_id = block_public_access_configuration.id
block_public_access_configuration_block_public_access_configuration = block_public_access_configuration.block_public_access_configuration
block_public_access_configuration_block_public_access_configuration_metadata = block_public_access_configuration.block_public_access_configuration_metadata
```

---


### Cluster_session_credentials

ClusterSessionCredentials resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expires_at` | String | <p>The time when the credentials that are returned by the
            <code>GetClusterSessionCredentials</code> API expire.</p> |
| `credentials` | String | <p>The credentials that you can use to connect to cluster endpoints that support username
         and password authentication.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cluster_session_credentials outputs
cluster_session_credentials_id = cluster_session_credentials.id
cluster_session_credentials_expires_at = cluster_session_credentials.expires_at
cluster_session_credentials_credentials = cluster_session_credentials.credentials
```

---


### Auto_scaling_policy

AutoScalingPolicy resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auto_scaling_policy` | String | ✅ | <p>Specifies the definition of the automatic scaling policy.</p> |
| `cluster_id` | String | ✅ | <p>Specifies the ID of a cluster. The instance group to which the automatic scaling policy
         is applied is within this cluster.</p> |
| `instance_group_id` | String | ✅ | <p>Specifies the ID of the instance group to which the automatic scaling policy is
         applied.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create auto_scaling_policy
auto_scaling_policy = provider.emr.Auto_scaling_policy {
    auto_scaling_policy = "value"  # <p>Specifies the definition of the automatic scaling policy.</p>
    cluster_id = "value"  # <p>Specifies the ID of a cluster. The instance group to which the automatic scaling policy
         is applied is within this cluster.</p>
    instance_group_id = "value"  # <p>Specifies the ID of the instance group to which the automatic scaling policy is
         applied.</p>
}

```

---


### Release_label

ReleaseLabel resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `applications` | Vec<String> | <p>The list of applications available for the target release label. <code>Name</code> is
         the name of the application. <code>Version</code> is the concise version of the
         application.</p> |
| `next_token` | String | <p>The pagination token. Reserved for future use. Currently set to null.</p> |
| `available_os_releases` | Vec<String> | <p>The list of available Amazon Linux release versions for an Amazon EMR release.
         Contains a Label field that is formatted as shown in <a href="https://docs.aws.amazon.com/AL2/latest/relnotes/relnotes-al2.html">
               <i>Amazon Linux 2 Release
               Notes</i>
            </a>. For example, <a href="https://docs.aws.amazon.com/AL2/latest/relnotes/relnotes-20220218.html">2.0.20220218.1</a>.</p> |
| `release_label` | String | <p>The target release label described in the response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access release_label outputs
release_label_id = release_label.id
release_label_applications = release_label.applications
release_label_next_token = release_label.next_token
release_label_available_os_releases = release_label.available_os_releases
release_label_release_label = release_label.release_label
```

---


### Job_flows

JobFlows resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_flows` | Vec<String> | <p>A list of job flows matching the parameters supplied.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access job_flows outputs
job_flows_id = job_flows.id
job_flows_job_flows = job_flows.job_flows
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple security_configuration resources
security_configuration_0 = provider.emr.Security_configuration {
    security_configuration = "value-0"
    name = "value-0"
}
security_configuration_1 = provider.emr.Security_configuration {
    security_configuration = "value-1"
    name = "value-1"
}
security_configuration_2 = provider.emr.Security_configuration {
    security_configuration = "value-2"
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    security_configuration = provider.emr.Security_configuration {
        security_configuration = "production-value"
        name = "production-value"
    }
```

---

## Related Documentation

- [AWS Emr Documentation](https://docs.aws.amazon.com/emr/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
