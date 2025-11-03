# Tnb Service



**Resources**: 9

---

## Overview

The tnb service provides access to 9 resource types:

- [Sol_function_instance](#sol_function_instance) [R]
- [Sol_network_package_descriptor](#sol_network_package_descriptor) [R]
- [Sol_network_operation](#sol_network_operation) [R]
- [Sol_function_package](#sol_function_package) [CRUD]
- [Sol_function_package_content](#sol_function_package_content) [CR]
- [Sol_network_instance](#sol_network_instance) [CRUD]
- [Sol_network_package_content](#sol_network_package_content) [CR]
- [Sol_function_package_descriptor](#sol_function_package_descriptor) [R]
- [Sol_network_package](#sol_network_package) [CRUD]

---

## Resources


### Sol_function_instance

SolFunctionInstance resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vnf_pkg_id` | String | <p>Function package ID.</p> |
| `vnfd_version` | String | <p>Function package descriptor version.</p> |
| `id` | String | <p>Network function instance ID.</p> |
| `arn` | String | <p>Network function instance ARN.</p> |
| `ns_instance_id` | String | <p>Network instance ID.</p> |
| `vnf_provider` | String | <p>Network function provider.</p> |
| `instantiation_state` | String | <p>Network function instantiation state.</p> |
| `vnfd_id` | String | <p>Function package descriptor ID.</p> |
| `metadata` | String |  |
| `instantiated_vnf_info` | String |  |
| `tags` | HashMap<String, String> | <p>A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value. You can use tags to search and filter your resources or track your Amazon Web Services costs.</p> |
| `vnf_product_name` | String | <p>Network function product name.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sol_function_instance outputs
sol_function_instance_id = sol_function_instance.id
sol_function_instance_vnf_pkg_id = sol_function_instance.vnf_pkg_id
sol_function_instance_vnfd_version = sol_function_instance.vnfd_version
sol_function_instance_id = sol_function_instance.id
sol_function_instance_arn = sol_function_instance.arn
sol_function_instance_ns_instance_id = sol_function_instance.ns_instance_id
sol_function_instance_vnf_provider = sol_function_instance.vnf_provider
sol_function_instance_instantiation_state = sol_function_instance.instantiation_state
sol_function_instance_vnfd_id = sol_function_instance.vnfd_id
sol_function_instance_metadata = sol_function_instance.metadata
sol_function_instance_instantiated_vnf_info = sol_function_instance.instantiated_vnf_info
sol_function_instance_tags = sol_function_instance.tags
sol_function_instance_vnf_product_name = sol_function_instance.vnf_product_name
```

---


### Sol_network_package_descriptor

SolNetworkPackageDescriptor resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `nsd` | String | <p>Contents of the network service descriptor in the network package.</p> |
| `content_type` | String | <p>Indicates the media type of the resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sol_network_package_descriptor outputs
sol_network_package_descriptor_id = sol_network_package_descriptor.id
sol_network_package_descriptor_nsd = sol_network_package_descriptor.nsd
sol_network_package_descriptor_content_type = sol_network_package_descriptor.content_type
```

---


### Sol_network_operation

SolNetworkOperation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `operation_state` | String | <p>The state of the network operation.</p> |
| `id` | String | <p>ID of this network operation occurrence.</p> |
| `tags` | HashMap<String, String> | <p>A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value. You can use tags to search and filter your resources or track your Amazon Web Services costs.</p> |
| `update_type` | String | <p>Type of the update. Only present if the network operation
         lcmOperationType is <code>UPDATE</code>.</p> |
| `lcm_operation_type` | String | <p>Type of the operation represented by this occurrence.</p> |
| `metadata` | String | <p>Metadata of this network operation occurrence.</p> |
| `arn` | String | <p>Network operation ARN.</p> |
| `ns_instance_id` | String | <p>ID of the network operation instance.</p> |
| `tasks` | Vec<String> | <p>All tasks associated with this operation occurrence.</p> |
| `error` | String | <p>Error related to this specific network operation occurrence.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sol_network_operation outputs
sol_network_operation_id = sol_network_operation.id
sol_network_operation_operation_state = sol_network_operation.operation_state
sol_network_operation_id = sol_network_operation.id
sol_network_operation_tags = sol_network_operation.tags
sol_network_operation_update_type = sol_network_operation.update_type
sol_network_operation_lcm_operation_type = sol_network_operation.lcm_operation_type
sol_network_operation_metadata = sol_network_operation.metadata
sol_network_operation_arn = sol_network_operation.arn
sol_network_operation_ns_instance_id = sol_network_operation.ns_instance_id
sol_network_operation_tasks = sol_network_operation.tasks
sol_network_operation_error = sol_network_operation.error
```

---


### Sol_function_package

SolFunctionPackage resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value. You can use tags to search and filter your resources or track your Amazon Web Services costs.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vnf_product_name` | String | <p>Network function product name.</p> |
| `arn` | String | <p>Function package ARN.</p> |
| `vnfd_version` | String | <p>Function package descriptor version.</p> |
| `metadata` | String |  |
| `usage_state` | String | <p>Function package usage state.</p> |
| `vnfd_id` | String | <p>Function package descriptor ID.</p> |
| `id` | String | <p>Function package ID.</p> |
| `operational_state` | String | <p>Function package operational state.</p> |
| `onboarding_state` | String | <p>Function package onboarding state.</p> |
| `vnf_provider` | String | <p>Network function provider.</p> |
| `tags` | HashMap<String, String> | <p>A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value. You can use tags to search and filter your resources or track your Amazon Web Services costs.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sol_function_package
sol_function_package = provider.tnb.Sol_function_package {
}

# Access sol_function_package outputs
sol_function_package_id = sol_function_package.id
sol_function_package_vnf_product_name = sol_function_package.vnf_product_name
sol_function_package_arn = sol_function_package.arn
sol_function_package_vnfd_version = sol_function_package.vnfd_version
sol_function_package_metadata = sol_function_package.metadata
sol_function_package_usage_state = sol_function_package.usage_state
sol_function_package_vnfd_id = sol_function_package.vnfd_id
sol_function_package_id = sol_function_package.id
sol_function_package_operational_state = sol_function_package.operational_state
sol_function_package_onboarding_state = sol_function_package.onboarding_state
sol_function_package_vnf_provider = sol_function_package.vnf_provider
sol_function_package_tags = sol_function_package.tags
```

---


### Sol_function_package_content

SolFunctionPackageContent resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file` | String | ✅ | <p>Function package file.</p> |
| `vnf_pkg_id` | String | ✅ | <p>Function package ID.</p> |
| `content_type` | String |  | <p>Function package content type.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `package_content` | String | <p>Contents of the function package.</p> |
| `content_type` | String | <p>Indicates the media type of the resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sol_function_package_content
sol_function_package_content = provider.tnb.Sol_function_package_content {
    file = "value"  # <p>Function package file.</p>
    vnf_pkg_id = "value"  # <p>Function package ID.</p>
}

# Access sol_function_package_content outputs
sol_function_package_content_id = sol_function_package_content.id
sol_function_package_content_package_content = sol_function_package_content.package_content
sol_function_package_content_content_type = sol_function_package_content.content_type
```

---


### Sol_network_instance

SolNetworkInstance resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value. You can use tags to search and filter your resources or track your Amazon Web Services costs.</p> |
| `ns_description` | String |  | <p>Network instance description.</p> |
| `ns_name` | String | ✅ | <p>Network instance name.</p> |
| `nsd_info_id` | String | ✅ | <p>ID for network service descriptor.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lcm_op_info` | String |  |
| `nsd_id` | String | <p>Network service descriptor ID.</p> |
| `tags` | HashMap<String, String> | <p>A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value. You can use tags to search and filter your resources or track your Amazon Web Services costs.</p> |
| `nsd_info_id` | String | <p>Network service descriptor info ID.</p> |
| `ns_instance_name` | String | <p>Network instance name.</p> |
| `ns_state` | String | <p>Network instance state.</p> |
| `metadata` | String |  |
| `id` | String | <p>Network instance ID.</p> |
| `arn` | String | <p>Network instance ARN.</p> |
| `ns_instance_description` | String | <p>Network instance description.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sol_network_instance
sol_network_instance = provider.tnb.Sol_network_instance {
    ns_name = "value"  # <p>Network instance name.</p>
    nsd_info_id = "value"  # <p>ID for network service descriptor.</p>
}

# Access sol_network_instance outputs
sol_network_instance_id = sol_network_instance.id
sol_network_instance_lcm_op_info = sol_network_instance.lcm_op_info
sol_network_instance_nsd_id = sol_network_instance.nsd_id
sol_network_instance_tags = sol_network_instance.tags
sol_network_instance_nsd_info_id = sol_network_instance.nsd_info_id
sol_network_instance_ns_instance_name = sol_network_instance.ns_instance_name
sol_network_instance_ns_state = sol_network_instance.ns_state
sol_network_instance_metadata = sol_network_instance.metadata
sol_network_instance_id = sol_network_instance.id
sol_network_instance_arn = sol_network_instance.arn
sol_network_instance_ns_instance_description = sol_network_instance.ns_instance_description
```

---


### Sol_network_package_content

SolNetworkPackageContent resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file` | String | ✅ | <p>Network package file.</p> |
| `nsd_info_id` | String | ✅ | <p>Network service descriptor info ID.</p> |
| `content_type` | String |  | <p>Network package content type.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `nsd_content` | String | <p>Content of the network service descriptor in the network package.</p> |
| `content_type` | String | <p>Indicates the media type of the resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sol_network_package_content
sol_network_package_content = provider.tnb.Sol_network_package_content {
    file = "value"  # <p>Network package file.</p>
    nsd_info_id = "value"  # <p>Network service descriptor info ID.</p>
}

# Access sol_network_package_content outputs
sol_network_package_content_id = sol_network_package_content.id
sol_network_package_content_nsd_content = sol_network_package_content.nsd_content
sol_network_package_content_content_type = sol_network_package_content.content_type
```

---


### Sol_function_package_descriptor

SolFunctionPackageDescriptor resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_type` | String | <p>Indicates the media type of the resource.</p> |
| `vnfd` | String | <p>Contents of the function package descriptor.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sol_function_package_descriptor outputs
sol_function_package_descriptor_id = sol_function_package_descriptor.id
sol_function_package_descriptor_content_type = sol_function_package_descriptor.content_type
sol_function_package_descriptor_vnfd = sol_function_package_descriptor.vnfd
```

---


### Sol_network_package

SolNetworkPackage resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value. You can use tags to search and filter your resources or track your Amazon Web Services costs.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `nsd_name` | String | <p>Network service descriptor name.</p> |
| `nsd_usage_state` | String | <p>Network service descriptor usage state.</p> |
| `nsd_id` | String | <p>Network service descriptor ID.</p> |
| `id` | String | <p>Network package ID.</p> |
| `nsd_onboarding_state` | String | <p>Network service descriptor onboarding state.</p> |
| `vnf_pkg_ids` | Vec<String> | <p>Identifies the function package for the function package descriptor referenced by the
         onboarded network package.</p> |
| `nsd_operational_state` | String | <p>Network service descriptor operational state.</p> |
| `arn` | String | <p>Network package ARN.</p> |
| `metadata` | String |  |
| `tags` | HashMap<String, String> | <p>A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value. You can use tags to search and filter your resources or track your Amazon Web Services costs.</p> |
| `nsd_version` | String | <p>Network service descriptor version.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sol_network_package
sol_network_package = provider.tnb.Sol_network_package {
}

# Access sol_network_package outputs
sol_network_package_id = sol_network_package.id
sol_network_package_nsd_name = sol_network_package.nsd_name
sol_network_package_nsd_usage_state = sol_network_package.nsd_usage_state
sol_network_package_nsd_id = sol_network_package.nsd_id
sol_network_package_id = sol_network_package.id
sol_network_package_nsd_onboarding_state = sol_network_package.nsd_onboarding_state
sol_network_package_vnf_pkg_ids = sol_network_package.vnf_pkg_ids
sol_network_package_nsd_operational_state = sol_network_package.nsd_operational_state
sol_network_package_arn = sol_network_package.arn
sol_network_package_metadata = sol_network_package.metadata
sol_network_package_tags = sol_network_package.tags
sol_network_package_nsd_version = sol_network_package.nsd_version
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple sol_function_instance resources
sol_function_instance_0 = provider.tnb.Sol_function_instance {
}
sol_function_instance_1 = provider.tnb.Sol_function_instance {
}
sol_function_instance_2 = provider.tnb.Sol_function_instance {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    sol_function_instance = provider.tnb.Sol_function_instance {
    }
```

---

## Related Documentation

- [AWS Tnb Documentation](https://docs.aws.amazon.com/tnb/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
