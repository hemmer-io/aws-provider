# Cloudhsm Service



**Resources**: 10

---

## Overview

The cloudhsm service provides access to 10 resource types:

- [Hsm](#hsm) [CRD]
- [Config](#config) [R]
- [Luna_client](#luna_client) [CRD]
- [Hapg](#hapg) [CRD]
- [Resource_policy](#resource_policy) [CRD]
- [Hsm](#hsm) [CD]
- [Clusters](#clusters) [R]
- [Backup](#backup) [D]
- [Backups](#backups) [R]
- [Cluster](#cluster) [CD]

---

## Resources


### Hsm

Hsm resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `syslog_ip` | String |  | <p>The IP address for the syslog monitoring server. The AWS CloudHSM service only supports one
      syslog monitoring server.</p> |
| `subscription_type` | String | ✅ |  |
| `ssh_key` | String | ✅ | <p>The SSH public key to install on the HSM.</p> |
| `external_id` | String |  | <p>The external ID from <code>IamRoleArn</code>, if present.</p> |
| `client_token` | String |  | <p>A user-defined token to ensure idempotence. Subsequent calls to this operation with the
      same token will be ignored.</p> |
| `eni_ip` | String |  | <p>The IP address to assign to the HSM's ENI.</p>
         <p>If an IP address is not specified, an IP address will be randomly chosen from the CIDR
      range of the subnet.</p> |
| `subnet_id` | String | ✅ | <p>The identifier of the subnet in your VPC in which to place the HSM.</p> |
| `iam_role_arn` | String | ✅ | <p>The ARN of an IAM role to enable the AWS CloudHSM service to allocate an ENI on your
      behalf.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The status of the HSM.</p> |
| `subnet_id` | String | <p>The identifier of the subnet that the HSM is in.</p> |
| `serial_number` | String | <p>The serial number of the HSM.</p> |
| `status_details` | String | <p>Contains additional information about the status of the HSM.</p> |
| `server_cert_uri` | String | <p>The URI of the certificate server.</p> |
| `partitions` | Vec<String> | <p>The list of partitions on the HSM.</p> |
| `subscription_type` | String |  |
| `eni_ip` | String | <p>The IP address assigned to the HSM's ENI.</p> |
| `subscription_start_date` | String | <p>The subscription start date.</p> |
| `hsm_arn` | String | <p>The ARN of the HSM.</p> |
| `vpc_id` | String | <p>The identifier of the VPC that the HSM is in.</p> |
| `server_cert_last_updated` | String | <p>The date and time that the server certificate was last updated.</p> |
| `availability_zone` | String | <p>The Availability Zone that the HSM is in.</p> |
| `hsm_type` | String | <p>The HSM model type.</p> |
| `eni_id` | String | <p>The identifier of the elastic network interface (ENI) attached to the HSM.</p> |
| `vendor_name` | String | <p>The name of the HSM vendor.</p> |
| `ssh_public_key` | String | <p>The public SSH key.</p> |
| `software_version` | String | <p>The HSM software version.</p> |
| `iam_role_arn` | String | <p>The ARN of the IAM role assigned to the HSM.</p> |
| `subscription_end_date` | String | <p>The subscription end date.</p> |
| `ssh_key_last_updated` | String | <p>The date and time that the SSH key was last updated.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create hsm
hsm = provider.cloudhsm.Hsm {
    subscription_type = "value"  # Required field
    ssh_key = "value"  # <p>The SSH public key to install on the HSM.</p>
    subnet_id = "value"  # <p>The identifier of the subnet in your VPC in which to place the HSM.</p>
    iam_role_arn = "value"  # <p>The ARN of an IAM role to enable the AWS CloudHSM service to allocate an ENI on your
      behalf.</p>
}

# Access hsm outputs
hsm_id = hsm.id
hsm_status = hsm.status
hsm_subnet_id = hsm.subnet_id
hsm_serial_number = hsm.serial_number
hsm_status_details = hsm.status_details
hsm_server_cert_uri = hsm.server_cert_uri
hsm_partitions = hsm.partitions
hsm_subscription_type = hsm.subscription_type
hsm_eni_ip = hsm.eni_ip
hsm_subscription_start_date = hsm.subscription_start_date
hsm_hsm_arn = hsm.hsm_arn
hsm_vpc_id = hsm.vpc_id
hsm_server_cert_last_updated = hsm.server_cert_last_updated
hsm_availability_zone = hsm.availability_zone
hsm_hsm_type = hsm.hsm_type
hsm_eni_id = hsm.eni_id
hsm_vendor_name = hsm.vendor_name
hsm_ssh_public_key = hsm.ssh_public_key
hsm_software_version = hsm.software_version
hsm_iam_role_arn = hsm.iam_role_arn
hsm_subscription_end_date = hsm.subscription_end_date
hsm_ssh_key_last_updated = hsm.ssh_key_last_updated
```

---


### Config

Config resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `config_type` | String | <p>The type of credentials.</p> |
| `config_file` | String | <p>The chrystoki.conf configuration file.</p> |
| `config_cred` | String | <p>The certificate file containing the server.pem files of the HSMs.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access config outputs
config_id = config.id
config_config_type = config.config_type
config_config_file = config.config_file
config_config_cred = config.config_cred
```

---


### Luna_client

LunaClient resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `certificate` | String | ✅ | <p>The contents of a Base64-Encoded X.509 v3 certificate to be installed on the HSMs used
      by this client.</p> |
| `label` | String |  | <p>The label for the client.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `certificate_fingerprint` | String | <p>The certificate fingerprint.</p> |
| `label` | String | <p>The label of the client.</p> |
| `last_modified_timestamp` | String | <p>The date and time the client was last modified.</p> |
| `certificate` | String | <p>The certificate installed on the HSMs used by this client.</p> |
| `client_arn` | String | <p>The ARN of the client.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create luna_client
luna_client = provider.cloudhsm.Luna_client {
    certificate = "value"  # <p>The contents of a Base64-Encoded X.509 v3 certificate to be installed on the HSMs used
      by this client.</p>
}

# Access luna_client outputs
luna_client_id = luna_client.id
luna_client_certificate_fingerprint = luna_client.certificate_fingerprint
luna_client_label = luna_client.label
luna_client_last_modified_timestamp = luna_client.last_modified_timestamp
luna_client_certificate = luna_client.certificate
luna_client_client_arn = luna_client.client_arn
```

---


### Hapg

Hapg resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `label` | String | ✅ | <p>The label of the new high-availability partition group.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `partition_serial_list` | Vec<String> | <p>The list of partition serial numbers that belong to the high-availability partition
      group.</p> |
| `hsms_last_action_failed` | Vec<String> | <p></p> |
| `last_modified_timestamp` | String | <p>The date and time the high-availability partition group was last modified.</p> |
| `hsms_pending_deletion` | Vec<String> | <p></p> |
| `hsms_pending_registration` | Vec<String> | <p></p> |
| `label` | String | <p>The label for the high-availability partition group.</p> |
| `hapg_arn` | String | <p>The ARN of the high-availability partition group.</p> |
| `hapg_serial` | String | <p>The serial number of the high-availability partition group.</p> |
| `state` | String | <p>The state of the high-availability partition group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create hapg
hapg = provider.cloudhsm.Hapg {
    label = "value"  # <p>The label of the new high-availability partition group.</p>
}

# Access hapg outputs
hapg_id = hapg.id
hapg_partition_serial_list = hapg.partition_serial_list
hapg_hsms_last_action_failed = hapg.hsms_last_action_failed
hapg_last_modified_timestamp = hapg.last_modified_timestamp
hapg_hsms_pending_deletion = hapg.hsms_pending_deletion
hapg_hsms_pending_registration = hapg.hsms_pending_registration
hapg_label = hapg.label
hapg_hapg_arn = hapg.hapg_arn
hapg_hapg_serial = hapg.hapg_serial
hapg_state = hapg.state
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String |  | <p>Amazon Resource Name (ARN) of the resource to which you want to attach a policy. </p> |
| `policy` | String |  | <p>The policy you want to associate with a resource. </p>
         <p>For an example policy, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/sharing.html"> Working with shared backups</a> in the CloudHSM User Guide</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The policy attached to a resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.cloudhsm.Resource_policy {
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_policy = resource_policy.policy
```

---


### Hsm

Hsm resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cluster_id` | String | ✅ | <p>The identifier (ID) of the HSM's cluster. To find the cluster ID, use <a>DescribeClusters</a>.</p> |
| `availability_zone` | String | ✅ | <p>The Availability Zone where you are creating the HSM. To find the cluster's
      Availability Zones, use <a>DescribeClusters</a>.</p> |
| `ip_address` | String |  | <p>The HSM's IP address. If you specify an IP address, use an available address from the
      subnet that maps to the Availability Zone where you are creating the HSM. If you don't specify
      an IP address, one is chosen for you from that subnet.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create hsm
hsm = provider.cloudhsm.Hsm {
    cluster_id = "value"  # <p>The identifier (ID) of the HSM's cluster. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    availability_zone = "value"  # <p>The Availability Zone where you are creating the HSM. To find the cluster's
      Availability Zones, use <a>DescribeClusters</a>.</p>
}

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
| `next_token` | String | <p>An opaque string that indicates that the response contains only a subset of clusters.
      Use this value in a subsequent <code>DescribeClusters</code> request to get more
      clusters.</p> |
| `clusters` | Vec<String> | <p>A list of clusters.</p> |


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
clusters_next_token = clusters.next_token
clusters_clusters = clusters.clusters
```

---


### Backup

Backup resource

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


### Backups

Backups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>An opaque string that indicates that the response contains only a subset of backups.
      Use this value in a subsequent <code>DescribeBackups</code> request to get more
      backups.</p> |
| `backups` | Vec<String> | <p>A list of backups.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access backups outputs
backups_id = backups.id
backups_next_token = backups.next_token
backups_backups = backups.backups
```

---


### Cluster

Cluster resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `mode` | String |  | <p>The mode to use in the cluster. The allowed values are
      <code>FIPS</code> and <code>NON_FIPS</code>.</p> |
| `source_backup_id` | String |  | <p>The identifier (ID) or the Amazon Resource Name (ARN) of the cluster backup to restore. Use this value to restore the
      cluster from a backup instead of creating a new cluster. To find the backup ID or ARN, use <a>DescribeBackups</a>. <i>If using a backup in another account, the full ARN must be supplied.</i>
         </p> |
| `backup_retention_policy` | String |  | <p>A policy that defines how the service retains backups.</p> |
| `hsm_type` | String | ✅ | <p>The type of HSM to use in the cluster. The allowed values are
      <code>hsm1.medium</code> and <code>hsm2m.medium</code>.</p> |
| `subnet_ids` | Vec<String> | ✅ | <p>The identifiers (IDs) of the subnets where you are creating the cluster. You must
      specify at least one subnet. If you specify multiple subnets, they must meet the following
      criteria:</p>
         <ul>
            <li>
               <p>All subnets must be in the same virtual private cloud (VPC).</p>
            </li>
            <li>
               <p>You can specify only one subnet per Availability Zone.</p>
            </li>
         </ul> |
| `network_type` | String |  | <p>The NetworkType to create a cluster with. The allowed values are
          <code>IPV4</code> and <code>DUALSTACK</code>.
      </p> |
| `tag_list` | Vec<String> |  | <p>Tags to apply to the CloudHSM cluster during creation.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster
cluster = provider.cloudhsm.Cluster {
    hsm_type = "value"  # <p>The type of HSM to use in the cluster. The allowed values are
      <code>hsm1.medium</code> and <code>hsm2m.medium</code>.</p>
    subnet_ids = "value"  # <p>The identifiers (IDs) of the subnets where you are creating the cluster. You must
      specify at least one subnet. If you specify multiple subnets, they must meet the following
      criteria:</p>
         <ul>
            <li>
               <p>All subnets must be in the same virtual private cloud (VPC).</p>
            </li>
            <li>
               <p>You can specify only one subnet per Availability Zone.</p>
            </li>
         </ul>
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

# Create multiple hsm resources
hsm_0 = provider.cloudhsm.Hsm {
    subscription_type = "value-0"
    ssh_key = "value-0"
    subnet_id = "value-0"
    iam_role_arn = "value-0"
}
hsm_1 = provider.cloudhsm.Hsm {
    subscription_type = "value-1"
    ssh_key = "value-1"
    subnet_id = "value-1"
    iam_role_arn = "value-1"
}
hsm_2 = provider.cloudhsm.Hsm {
    subscription_type = "value-2"
    ssh_key = "value-2"
    subnet_id = "value-2"
    iam_role_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    hsm = provider.cloudhsm.Hsm {
        subscription_type = "production-value"
        ssh_key = "production-value"
        subnet_id = "production-value"
        iam_role_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Cloudhsm Documentation](https://docs.aws.amazon.com/cloudhsm/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
