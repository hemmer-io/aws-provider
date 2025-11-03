# Migration_hub Service



**Resources**: 4

---

## Overview

The migration_hub service provides access to 4 resource types:

- [Progress_update_stream](#progress_update_stream) [CD]
- [Migration_task](#migration_task) [R]
- [Application_state](#application_state) [R]
- [Resource_attributes](#resource_attributes) [C]

---

## Resources


### Progress_update_stream

ProgressUpdateStream resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `progress_update_stream_name` | String | ✅ | <p>The name of the ProgressUpdateStream. <i>Do not store personal data in this
            field.</i>
         </p> |
| `dry_run` | bool |  | <p>Optional boolean flag to indicate whether any effect should take place. Used to test if
         the caller has permission to make the call.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create progress_update_stream
progress_update_stream = provider.migration_hub.Progress_update_stream {
    progress_update_stream_name = "value"  # <p>The name of the ProgressUpdateStream. <i>Do not store personal data in this
            field.</i>
         </p>
}

```

---


### Migration_task

MigrationTask resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `migration_task` | String | <p>Object encapsulating information about the migration task.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access migration_task outputs
migration_task_id = migration_task.id
migration_task_migration_task = migration_task.migration_task
```

---


### Application_state

ApplicationState resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_updated_time` | String | <p>The timestamp when the application status was last updated.</p> |
| `application_status` | String | <p>Status of the application - Not Started, In-Progress, Complete.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access application_state outputs
application_state_id = application_state.id
application_state_last_updated_time = application_state.last_updated_time
application_state_application_status = application_state.application_status
```

---


### Resource_attributes

ResourceAttributes resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `progress_update_stream` | String | ✅ | <p>The name of the ProgressUpdateStream. </p> |
| `dry_run` | bool |  | <p>Optional boolean flag to indicate whether any effect should take place. Used to test if
         the caller has permission to make the call.</p> |
| `migration_task_name` | String | ✅ | <p>Unique identifier that references the migration task. <i>Do not store personal
            data in this field.</i>
         </p> |
| `resource_attribute_list` | Vec<String> | ✅ | <p>Information about the resource that is being migrated. This data will be used to map the
         task to a resource in the Application Discovery Service repository.</p>
         <note>
            <p>Takes the object array of <code>ResourceAttribute</code> where the <code>Type</code>
            field is reserved for the following values: <code>IPV4_ADDRESS | IPV6_ADDRESS |
               MAC_ADDRESS | FQDN | VM_MANAGER_ID | VM_MANAGED_OBJECT_REFERENCE | VM_NAME | VM_PATH
               | BIOS_ID | MOTHERBOARD_SERIAL_NUMBER</code> where the identifying value can be a
            string up to 256 characters.</p>
         </note>
         <important>
            <ul>
               <li>
                  <p>If any "VM" related value is set for a <code>ResourceAttribute</code> object,
                  it is required that <code>VM_MANAGER_ID</code>, as a minimum, is always set. If
                     <code>VM_MANAGER_ID</code> is not set, then all "VM" fields will be discarded
                  and "VM" fields will not be used for matching the migration task to a server in
                  Application Discovery Service repository. See the <a href="https://docs.aws.amazon.com/migrationhub/latest/ug/API_PutResourceAttributes.html#API_PutResourceAttributes_Examples">Example</a> section below for a use case of specifying "VM" related
                  values.</p>
               </li>
               <li>
                  <p> If a server you are trying to match has multiple IP or MAC addresses, you
                  should provide as many as you know in separate type/value pairs passed to the
                     <code>ResourceAttributeList</code> parameter to maximize the chances of
                  matching.</p>
               </li>
            </ul>
         </important> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_attributes
resource_attributes = provider.migration_hub.Resource_attributes {
    progress_update_stream = "value"  # <p>The name of the ProgressUpdateStream. </p>
    migration_task_name = "value"  # <p>Unique identifier that references the migration task. <i>Do not store personal
            data in this field.</i>
         </p>
    resource_attribute_list = "value"  # <p>Information about the resource that is being migrated. This data will be used to map the
         task to a resource in the Application Discovery Service repository.</p>
         <note>
            <p>Takes the object array of <code>ResourceAttribute</code> where the <code>Type</code>
            field is reserved for the following values: <code>IPV4_ADDRESS | IPV6_ADDRESS |
               MAC_ADDRESS | FQDN | VM_MANAGER_ID | VM_MANAGED_OBJECT_REFERENCE | VM_NAME | VM_PATH
               | BIOS_ID | MOTHERBOARD_SERIAL_NUMBER</code> where the identifying value can be a
            string up to 256 characters.</p>
         </note>
         <important>
            <ul>
               <li>
                  <p>If any "VM" related value is set for a <code>ResourceAttribute</code> object,
                  it is required that <code>VM_MANAGER_ID</code>, as a minimum, is always set. If
                     <code>VM_MANAGER_ID</code> is not set, then all "VM" fields will be discarded
                  and "VM" fields will not be used for matching the migration task to a server in
                  Application Discovery Service repository. See the <a href="https://docs.aws.amazon.com/migrationhub/latest/ug/API_PutResourceAttributes.html#API_PutResourceAttributes_Examples">Example</a> section below for a use case of specifying "VM" related
                  values.</p>
               </li>
               <li>
                  <p> If a server you are trying to match has multiple IP or MAC addresses, you
                  should provide as many as you know in separate type/value pairs passed to the
                     <code>ResourceAttributeList</code> parameter to maximize the chances of
                  matching.</p>
               </li>
            </ul>
         </important>
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

# Create multiple progress_update_stream resources
progress_update_stream_0 = provider.migration_hub.Progress_update_stream {
    progress_update_stream_name = "value-0"
}
progress_update_stream_1 = provider.migration_hub.Progress_update_stream {
    progress_update_stream_name = "value-1"
}
progress_update_stream_2 = provider.migration_hub.Progress_update_stream {
    progress_update_stream_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    progress_update_stream = provider.migration_hub.Progress_update_stream {
        progress_update_stream_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Migration_hub Documentation](https://docs.aws.amazon.com/migration_hub/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
