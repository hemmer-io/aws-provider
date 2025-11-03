# Panorama Service



**Resources**: 11

---

## Overview

The panorama service provides access to 11 resource types:

- [Node_from_template_job](#node_from_template_job) [CR]
- [Device_metadata](#device_metadata) [U]
- [Application_instance](#application_instance) [CR]
- [Package_import_job](#package_import_job) [CR]
- [Device_job](#device_job) [R]
- [Package](#package) [CRD]
- [Application_instance_details](#application_instance_details) [R]
- [Package_version](#package_version) [R]
- [Node](#node) [R]
- [Job_for_devices](#job_for_devices) [C]
- [Device](#device) [RD]

---

## Resources


### Node_from_template_job

NodeFromTemplateJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `template_type` | String | ✅ | <p>The type of node.</p> |
| `node_description` | String |  | <p>A description for the node.</p> |
| `node_name` | String | ✅ | <p>A name for the node.</p> |
| `template_parameters` | HashMap<String, String> | ✅ | <p>Template parameters for the node.</p> |
| `job_tags` | Vec<String> |  | <p>Tags for the job.</p> |
| `output_package_name` | String | ✅ | <p>An output package name for the node.</p> |
| `output_package_version` | String | ✅ | <p>An output package version for the node.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The job's status.</p> |
| `node_description` | String | <p>The node's description.</p> |
| `created_time` | String | <p>When the job was created.</p> |
| `last_updated_time` | String | <p>When the job was updated.</p> |
| `output_package_version` | String | <p>The job's output package version.</p> |
| `template_parameters` | HashMap<String, String> | <p>The job's template parameters.</p> |
| `job_id` | String | <p>The job's ID.</p> |
| `output_package_name` | String | <p>The job's output package name.</p> |
| `job_tags` | Vec<String> | <p>The job's tags.</p> |
| `status_message` | String | <p>The job's status message.</p> |
| `node_name` | String | <p>The node's name.</p> |
| `template_type` | String | <p>The job's template type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create node_from_template_job
node_from_template_job = provider.panorama.Node_from_template_job {
    template_type = "value"  # <p>The type of node.</p>
    node_name = "value"  # <p>A name for the node.</p>
    template_parameters = "value"  # <p>Template parameters for the node.</p>
    output_package_name = "value"  # <p>An output package name for the node.</p>
    output_package_version = "value"  # <p>An output package version for the node.</p>
}

# Access node_from_template_job outputs
node_from_template_job_id = node_from_template_job.id
node_from_template_job_status = node_from_template_job.status
node_from_template_job_node_description = node_from_template_job.node_description
node_from_template_job_created_time = node_from_template_job.created_time
node_from_template_job_last_updated_time = node_from_template_job.last_updated_time
node_from_template_job_output_package_version = node_from_template_job.output_package_version
node_from_template_job_template_parameters = node_from_template_job.template_parameters
node_from_template_job_job_id = node_from_template_job.job_id
node_from_template_job_output_package_name = node_from_template_job.output_package_name
node_from_template_job_job_tags = node_from_template_job.job_tags
node_from_template_job_status_message = node_from_template_job.status_message
node_from_template_job_node_name = node_from_template_job.node_name
node_from_template_job_template_type = node_from_template_job.template_type
```

---


### Device_metadata

DeviceMetadata resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `device_id` | String | ✅ | <p>The device's ID.</p> |
| `description` | String |  | <p>A description for the device.</p> |



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


### Application_instance

ApplicationInstance resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `manifest_overrides_payload` | String |  | <p>Setting overrides for the application manifest.</p> |
| `runtime_role_arn` | String |  | <p>The ARN of a runtime role for the application instance.</p> |
| `default_runtime_context_device` | String | ✅ | <p>A device's ID.</p> |
| `tags` | HashMap<String, String> |  | <p>Tags for the application instance.</p> |
| `name` | String |  | <p>A name for the application instance.</p> |
| `manifest_payload` | String | ✅ | <p>The application's manifest document.</p> |
| `description` | String |  | <p>A description for the application instance.</p> |
| `application_instance_id_to_replace` | String |  | <p>The ID of an application instance to replace with the new instance.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_runtime_context_device` | String | <p>The device's ID.</p> |
| `arn` | String | <p>The application instance's ARN.</p> |
| `runtime_role_arn` | String | <p>The application instance's runtime role ARN.</p> |
| `default_runtime_context_device_name` | String | <p>The device's bane.</p> |
| `status` | String | <p>The application instance's status.</p> |
| `tags` | HashMap<String, String> | <p>The application instance's tags.</p> |
| `name` | String | <p>The application instance's name.</p> |
| `created_time` | String | <p>When the application instance was created.</p> |
| `runtime_context_states` | Vec<String> | <p>The application instance's state.</p> |
| `application_instance_id_to_replace` | String | <p>The ID of the application instance that this instance replaced.</p> |
| `last_updated_time` | String | <p>The application instance was updated.</p> |
| `health_status` | String | <p>The application instance's health status.</p> |
| `status_description` | String | <p>The application instance's status description.</p> |
| `description` | String | <p>The application instance's description.</p> |
| `application_instance_id` | String | <p>The application instance's ID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application_instance
application_instance = provider.panorama.Application_instance {
    default_runtime_context_device = "value"  # <p>A device's ID.</p>
    manifest_payload = "value"  # <p>The application's manifest document.</p>
}

# Access application_instance outputs
application_instance_id = application_instance.id
application_instance_default_runtime_context_device = application_instance.default_runtime_context_device
application_instance_arn = application_instance.arn
application_instance_runtime_role_arn = application_instance.runtime_role_arn
application_instance_default_runtime_context_device_name = application_instance.default_runtime_context_device_name
application_instance_status = application_instance.status
application_instance_tags = application_instance.tags
application_instance_name = application_instance.name
application_instance_created_time = application_instance.created_time
application_instance_runtime_context_states = application_instance.runtime_context_states
application_instance_application_instance_id_to_replace = application_instance.application_instance_id_to_replace
application_instance_last_updated_time = application_instance.last_updated_time
application_instance_health_status = application_instance.health_status
application_instance_status_description = application_instance.status_description
application_instance_description = application_instance.description
application_instance_application_instance_id = application_instance.application_instance_id
```

---


### Package_import_job

PackageImportJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `job_type` | String | ✅ | <p>A job type for the package import job.</p> |
| `client_token` | String | ✅ | <p>A client token for the package import job.</p> |
| `input_config` | String | ✅ | <p>An input config for the package import job.</p> |
| `output_config` | String | ✅ | <p>An output config for the package import job.</p> |
| `job_tags` | Vec<String> |  | <p>Tags for the package import job.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `input_config` | String | <p>The job's input config.</p> |
| `job_id` | String | <p>The job's ID.</p> |
| `status_message` | String | <p>The job's status message.</p> |
| `job_tags` | Vec<String> | <p>The job's tags.</p> |
| `client_token` | String | <p>The job's client token.</p> |
| `output_config` | String | <p>The job's output config.</p> |
| `created_time` | String | <p>When the job was created.</p> |
| `last_updated_time` | String | <p>When the job was updated.</p> |
| `status` | String | <p>The job's status.</p> |
| `job_type` | String | <p>The job's type.</p> |
| `output` | String | <p>The job's output.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create package_import_job
package_import_job = provider.panorama.Package_import_job {
    job_type = "value"  # <p>A job type for the package import job.</p>
    client_token = "value"  # <p>A client token for the package import job.</p>
    input_config = "value"  # <p>An input config for the package import job.</p>
    output_config = "value"  # <p>An output config for the package import job.</p>
}

# Access package_import_job outputs
package_import_job_id = package_import_job.id
package_import_job_input_config = package_import_job.input_config
package_import_job_job_id = package_import_job.job_id
package_import_job_status_message = package_import_job.status_message
package_import_job_job_tags = package_import_job.job_tags
package_import_job_client_token = package_import_job.client_token
package_import_job_output_config = package_import_job.output_config
package_import_job_created_time = package_import_job.created_time
package_import_job_last_updated_time = package_import_job.last_updated_time
package_import_job_status = package_import_job.status
package_import_job_job_type = package_import_job.job_type
package_import_job_output = package_import_job.output
```

---


### Device_job

DeviceJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `image_version` | String | <p>For an OTA job, the target version of the device software.</p> |
| `created_time` | String | <p>When the job was created.</p> |
| `job_type` | String | <p>The job's type.</p> |
| `job_id` | String | <p>The job's ID.</p> |
| `device_type` | String | <p>The device's type.</p> |
| `device_id` | String | <p>The device's ID.</p> |
| `status` | String | <p>The job's status.</p> |
| `device_arn` | String | <p>The device's ARN.</p> |
| `device_name` | String | <p>The device's name.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access device_job outputs
device_job_id = device_job.id
device_job_image_version = device_job.image_version
device_job_created_time = device_job.created_time
device_job_job_type = device_job.job_type
device_job_job_id = device_job.job_id
device_job_device_type = device_job.device_type
device_job_device_id = device_job.device_id
device_job_status = device_job.status
device_job_device_arn = device_job.device_arn
device_job_device_name = device_job.device_name
```

---


### Package

Package resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `package_name` | String | ✅ | <p>A name for the package.</p> |
| `tags` | HashMap<String, String> |  | <p>Tags for the package.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `storage_location` | String | <p>The package's storage location.</p> |
| `package_id` | String | <p>The package's ID.</p> |
| `package_name` | String | <p>The package's name.</p> |
| `read_access_principal_arns` | Vec<String> | <p>ARNs of accounts that have read access to the package.</p> |
| `write_access_principal_arns` | Vec<String> | <p>ARNs of accounts that have write access to the package.</p> |
| `created_time` | String | <p>When the package was created.</p> |
| `tags` | HashMap<String, String> | <p>The package's tags.</p> |
| `arn` | String | <p>The package's ARN.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create package
package = provider.panorama.Package {
    package_name = "value"  # <p>A name for the package.</p>
}

# Access package outputs
package_id = package.id
package_storage_location = package.storage_location
package_package_id = package.package_id
package_package_name = package.package_name
package_read_access_principal_arns = package.read_access_principal_arns
package_write_access_principal_arns = package.write_access_principal_arns
package_created_time = package.created_time
package_tags = package.tags
package_arn = package.arn
```

---


### Application_instance_details

ApplicationInstanceDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_runtime_context_device` | String | <p>The application instance's default runtime context device.</p> |
| `created_time` | String | <p>When the application instance was created.</p> |
| `manifest_payload` | String | <p>The application instance's configuration manifest.</p> |
| `description` | String | <p>The application instance's description.</p> |
| `manifest_overrides_payload` | String | <p>Parameter overrides for the configuration manifest.</p> |
| `application_instance_id` | String | <p>The application instance's ID.</p> |
| `name` | String | <p>The application instance's name.</p> |
| `application_instance_id_to_replace` | String | <p>The ID of the application instance that this instance replaced.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access application_instance_details outputs
application_instance_details_id = application_instance_details.id
application_instance_details_default_runtime_context_device = application_instance_details.default_runtime_context_device
application_instance_details_created_time = application_instance_details.created_time
application_instance_details_manifest_payload = application_instance_details.manifest_payload
application_instance_details_description = application_instance_details.description
application_instance_details_manifest_overrides_payload = application_instance_details.manifest_overrides_payload
application_instance_details_application_instance_id = application_instance_details.application_instance_id
application_instance_details_name = application_instance_details.name
application_instance_details_application_instance_id_to_replace = application_instance_details.application_instance_id_to_replace
```

---


### Package_version

PackageVersion resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `package_name` | String | <p>The version's name.</p> |
| `status_description` | String | <p>The version's status description.</p> |
| `package_version` | String | <p>The version's version.</p> |
| `registered_time` | String | <p>The version's registered time.</p> |
| `status` | String | <p>The version's status.</p> |
| `owner_account` | String | <p>The account ID of the version's owner.</p> |
| `package_arn` | String | <p>The ARN of the package.</p> |
| `package_id` | String | <p>The version's ID.</p> |
| `patch_version` | String | <p>The version's patch version.</p> |
| `is_latest_patch` | bool | <p>Whether the version is the latest available.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access package_version outputs
package_version_id = package_version.id
package_version_package_name = package_version.package_name
package_version_status_description = package_version.status_description
package_version_package_version = package_version.package_version
package_version_registered_time = package_version.registered_time
package_version_status = package_version.status
package_version_owner_account = package_version.owner_account
package_version_package_arn = package_version.package_arn
package_version_package_id = package_version.package_id
package_version_patch_version = package_version.patch_version
package_version_is_latest_patch = package_version.is_latest_patch
```

---


### Node

Node resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `patch_version` | String | <p>The node's patch version.</p> |
| `node_id` | String | <p>The node's ID.</p> |
| `package_name` | String | <p>The node's package name.</p> |
| `package_version` | String | <p>The node's package version.</p> |
| `category` | String | <p>The node's category.</p> |
| `node_interface` | String | <p>The node's interface.</p> |
| `asset_name` | String | <p>The node's asset name.</p> |
| `last_updated_time` | String | <p>When the node was updated.</p> |
| `package_id` | String | <p>The node's package ID.</p> |
| `package_arn` | String | <p>The node's ARN.</p> |
| `owner_account` | String | <p>The account ID of the node's owner.</p> |
| `description` | String | <p>The node's description.</p> |
| `name` | String | <p>The node's name.</p> |
| `created_time` | String | <p>When the node was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access node outputs
node_id = node.id
node_patch_version = node.patch_version
node_node_id = node.node_id
node_package_name = node.package_name
node_package_version = node.package_version
node_category = node.category
node_node_interface = node.node_interface
node_asset_name = node.asset_name
node_last_updated_time = node.last_updated_time
node_package_id = node.package_id
node_package_arn = node.package_arn
node_owner_account = node.owner_account
node_description = node.description
node_name = node.name
node_created_time = node.created_time
```

---


### Job_for_devices

JobForDevices resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `device_job_config` | String |  | <p>Configuration settings for a software update job.</p> |
| `device_ids` | Vec<String> | ✅ | <p>ID of target device.</p> |
| `job_type` | String | ✅ | <p>The type of job to run.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create job_for_devices
job_for_devices = provider.panorama.Job_for_devices {
    device_ids = "value"  # <p>ID of target device.</p>
    job_type = "value"  # <p>The type of job to run.</p>
}

```

---


### Device

Device resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | <p>The device's type.</p> |
| `lease_expiration_time` | String | <p>The device's lease expiration time.</p> |
| `alternate_softwares` | Vec<String> | <p>Beta software releases available for the device.</p> |
| `current_networking_status` | String | <p>The device's networking status.</p> |
| `serial_number` | String | <p>The device's serial number.</p> |
| `created_time` | String | <p>When the device was created.</p> |
| `device_aggregated_status` | String | <p>A device's aggregated status. Including the device's connection status, provisioning status, and lease status.</p> |
| `latest_alternate_software` | String | <p>The most recent beta software release.</p> |
| `description` | String | <p>The device's description.</p> |
| `name` | String | <p>The device's name.</p> |
| `latest_software` | String | <p>The latest software version available for the device.</p> |
| `current_software` | String | <p>The device's current software version.</p> |
| `networking_configuration` | String | <p>The device's networking configuration.</p> |
| `provisioning_status` | String | <p>The device's provisioning status.</p> |
| `arn` | String | <p>The device's ARN.</p> |
| `device_id` | String | <p>The device's ID.</p> |
| `device_connection_status` | String | <p>The device's connection status.</p> |
| `tags` | HashMap<String, String> | <p>The device's tags.</p> |
| `brand` | String | <p>The device's maker.</p> |
| `latest_device_job` | String | <p>A device's latest job. Includes the target image version, and the job status.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access device outputs
device_id = device.id
device_type = device.type
device_lease_expiration_time = device.lease_expiration_time
device_alternate_softwares = device.alternate_softwares
device_current_networking_status = device.current_networking_status
device_serial_number = device.serial_number
device_created_time = device.created_time
device_device_aggregated_status = device.device_aggregated_status
device_latest_alternate_software = device.latest_alternate_software
device_description = device.description
device_name = device.name
device_latest_software = device.latest_software
device_current_software = device.current_software
device_networking_configuration = device.networking_configuration
device_provisioning_status = device.provisioning_status
device_arn = device.arn
device_device_id = device.device_id
device_device_connection_status = device.device_connection_status
device_tags = device.tags
device_brand = device.brand
device_latest_device_job = device.latest_device_job
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple node_from_template_job resources
node_from_template_job_0 = provider.panorama.Node_from_template_job {
    template_type = "value-0"
    node_name = "value-0"
    template_parameters = "value-0"
    output_package_name = "value-0"
    output_package_version = "value-0"
}
node_from_template_job_1 = provider.panorama.Node_from_template_job {
    template_type = "value-1"
    node_name = "value-1"
    template_parameters = "value-1"
    output_package_name = "value-1"
    output_package_version = "value-1"
}
node_from_template_job_2 = provider.panorama.Node_from_template_job {
    template_type = "value-2"
    node_name = "value-2"
    template_parameters = "value-2"
    output_package_name = "value-2"
    output_package_version = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    node_from_template_job = provider.panorama.Node_from_template_job {
        template_type = "production-value"
        node_name = "production-value"
        template_parameters = "production-value"
        output_package_name = "production-value"
        output_package_version = "production-value"
    }
```

---

## Related Documentation

- [AWS Panorama Documentation](https://docs.aws.amazon.com/panorama/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
