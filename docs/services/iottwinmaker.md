# Iottwinmaker Service



**Resources**: 9

---

## Overview

The iottwinmaker service provides access to 9 resource types:

- [Property_value](#property_value) [R]
- [Workspace](#workspace) [CRUD]
- [Entity](#entity) [CRUD]
- [Scene](#scene) [CRUD]
- [Component_type](#component_type) [CRUD]
- [Property_value_history](#property_value_history) [R]
- [Pricing_plan](#pricing_plan) [RU]
- [Sync_job](#sync_job) [CRD]
- [Metadata_transfer_job](#metadata_transfer_job) [CR]

---

## Resources


### Property_value

PropertyValue resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `property_values` | HashMap<String, String> | <p>An object that maps strings to the properties and latest property values in the
         response. Each string in the mapping must be unique to this object.</p> |
| `next_token` | String | <p>The string that specifies the next page of results.</p> |
| `tabular_property_values` | Vec<Vec<HashMap<String, String>>> | <p>A table of property values.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access property_value outputs
property_value_id = property_value.id
property_value_property_values = property_value.property_values
property_value_next_token = property_value.next_token
property_value_tabular_property_values = property_value.tabular_property_values
```

---


### Workspace

Workspace resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description of the workspace.</p> |
| `workspace_id` | String | ✅ | <p>The ID of the workspace.</p> |
| `role` | String |  | <p>The ARN of the execution role associated with the workspace.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata that you can use to manage the workspace</p> |
| `s3_location` | String |  | <p>The ARN of the S3 bucket where resources associated with the workspace are
         stored.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `linked_services` | Vec<String> | <p>A list of services that are linked to the workspace.</p> |
| `description` | String | <p>The description of the workspace.</p> |
| `s3_location` | String | <p>The ARN of the S3 bucket where resources associated with the workspace are
         stored.</p> |
| `update_date_time` | String | <p>The date and time when the workspace was last updated.</p> |
| `arn` | String | <p>The ARN of the workspace.</p> |
| `workspace_id` | String | <p>The ID of the workspace.</p> |
| `creation_date_time` | String | <p>The date and time when the workspace was created.</p> |
| `role` | String | <p>The ARN of the execution role associated with the workspace.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create workspace
workspace = provider.iottwinmaker.Workspace {
    workspace_id = "value"  # <p>The ID of the workspace.</p>
}

# Access workspace outputs
workspace_id = workspace.id
workspace_linked_services = workspace.linked_services
workspace_description = workspace.description
workspace_s3_location = workspace.s3_location
workspace_update_date_time = workspace.update_date_time
workspace_arn = workspace.arn
workspace_workspace_id = workspace.workspace_id
workspace_creation_date_time = workspace.creation_date_time
workspace_role = workspace.role
```

---


### Entity

Entity resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entity_id` | String |  | <p>The ID of the entity.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata that you can use to manage the entity.</p> |
| `components` | HashMap<String, String> |  | <p>An object that maps strings to the components in the entity. Each string in the mapping
         must be unique to this object.</p> |
| `composite_components` | HashMap<String, String> |  | <p>This is an object that maps strings to <code>compositeComponent</code> updates in the request. 
          Each key of the map represents the <code>componentPath</code> of the <code>compositeComponent</code>.</p> |
| `description` | String |  | <p>The description of the entity.</p> |
| `workspace_id` | String | ✅ | <p>The ID of the workspace that contains the entity.</p> |
| `entity_name` | String | ✅ | <p>The name of the entity.</p> |
| `parent_entity_id` | String |  | <p>The ID of the entity's parent entity.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parent_entity_id` | String | <p>The ID of the parent entity for this entity.</p> |
| `status` | String | <p>The current status of the entity.</p> |
| `arn` | String | <p>The ARN of the entity.</p> |
| `has_child_entities` | bool | <p>A Boolean value that specifies whether the entity has associated child entities.</p> |
| `update_date_time` | String | <p>The date and time when the entity was last updated.</p> |
| `are_all_components_returned` | bool | <p>This flag notes whether all components are returned in the API response. The maximum number of components returned is 30.</p> |
| `sync_source` | String | <p>The syncSource of the sync job, if this entity was created by a sync job.</p> |
| `entity_id` | String | <p>The ID of the entity.</p> |
| `creation_date_time` | String | <p>The date and time when the entity was created.</p> |
| `description` | String | <p>The description of the entity.</p> |
| `components` | HashMap<String, String> | <p>An object that maps strings to the components in the entity. Each string in the mapping
         must be unique to this object.</p> |
| `entity_name` | String | <p>The name of the entity.</p> |
| `workspace_id` | String | <p>The ID of the workspace.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create entity
entity = provider.iottwinmaker.Entity {
    workspace_id = "value"  # <p>The ID of the workspace that contains the entity.</p>
    entity_name = "value"  # <p>The name of the entity.</p>
}

# Access entity outputs
entity_id = entity.id
entity_parent_entity_id = entity.parent_entity_id
entity_status = entity.status
entity_arn = entity.arn
entity_has_child_entities = entity.has_child_entities
entity_update_date_time = entity.update_date_time
entity_are_all_components_returned = entity.are_all_components_returned
entity_sync_source = entity.sync_source
entity_entity_id = entity.entity_id
entity_creation_date_time = entity.creation_date_time
entity_description = entity.description
entity_components = entity.components
entity_entity_name = entity.entity_name
entity_workspace_id = entity.workspace_id
```

---


### Scene

Scene resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description for this scene.</p> |
| `capabilities` | Vec<String> |  | <p>A list of capabilities that the scene uses to render itself.</p> |
| `scene_metadata` | HashMap<String, String> |  | <p>The request metadata.</p> |
| `scene_id` | String | ✅ | <p>The ID of the scene.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata that you can use to manage the scene.</p> |
| `workspace_id` | String | ✅ | <p>The ID of the workspace that contains the scene.</p> |
| `content_location` | String | ✅ | <p>The relative path that specifies the location of the content definition file.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scene_metadata` | HashMap<String, String> | <p>The response metadata.</p> |
| `generated_scene_metadata` | HashMap<String, String> | <p>The generated scene metadata.</p> |
| `scene_id` | String | <p>The ID of the scene.</p> |
| `creation_date_time` | String | <p>The date and time when the scene was created.</p> |
| `workspace_id` | String | <p>The ID of the workspace that contains the scene.</p> |
| `update_date_time` | String | <p>The date and time when the scene was last updated.</p> |
| `capabilities` | Vec<String> | <p>A list of capabilities that the scene uses to render.</p> |
| `description` | String | <p>The description of the scene.</p> |
| `error` | String | <p>The SceneResponse error.</p> |
| `content_location` | String | <p>The relative path that specifies the location of the content definition file.</p> |
| `arn` | String | <p>The ARN of the scene.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create scene
scene = provider.iottwinmaker.Scene {
    scene_id = "value"  # <p>The ID of the scene.</p>
    workspace_id = "value"  # <p>The ID of the workspace that contains the scene.</p>
    content_location = "value"  # <p>The relative path that specifies the location of the content definition file.</p>
}

# Access scene outputs
scene_id = scene.id
scene_scene_metadata = scene.scene_metadata
scene_generated_scene_metadata = scene.generated_scene_metadata
scene_scene_id = scene.scene_id
scene_creation_date_time = scene.creation_date_time
scene_workspace_id = scene.workspace_id
scene_update_date_time = scene.update_date_time
scene_capabilities = scene.capabilities
scene_description = scene.description
scene_error = scene.error
scene_content_location = scene.content_location
scene_arn = scene.arn
```

---


### Component_type

ComponentType resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>Metadata that you can use to manage the component type.</p> |
| `composite_component_types` | HashMap<String, String> |  | <p>This is an object that maps strings to <code>compositeComponentTypes</code> of the <code>componentType</code>. 
          <code>CompositeComponentType</code> is referenced by <code>componentTypeId</code>.</p> |
| `workspace_id` | String | ✅ | <p>The ID of the workspace that contains the component type.</p> |
| `property_groups` | HashMap<String, String> |  | <p/> |
| `is_singleton` | bool |  | <p>A Boolean value that specifies whether an entity can have more than one component of
         this type.</p> |
| `component_type_id` | String | ✅ | <p>The ID of the component type.</p> |
| `property_definitions` | HashMap<String, String> |  | <p>An object that maps strings to the property definitions in the component type. Each
         string in the mapping must be unique to this object.</p> |
| `extends_from` | Vec<String> |  | <p>Specifies the parent component type to extend.</p> |
| `description` | String |  | <p>The description of the component type.</p> |
| `functions` | HashMap<String, String> |  | <p>An object that maps strings to the functions in the component type. Each string in the
         mapping must be unique to this object.</p> |
| `component_type_name` | String |  | <p>A friendly name for the component type.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `is_schema_initialized` | bool | <p>A Boolean value that specifies whether the component type has a schema initializer and
         that the schema initializer has run.</p> |
| `status` | String | <p>The current status of the component type.</p> |
| `update_date_time` | String | <p>The date and time when the component was last updated.</p> |
| `functions` | HashMap<String, String> | <p>An object that maps strings to the functions in the component type. Each string in the
         mapping must be unique to this object.</p> |
| `description` | String | <p>The description of the component type.</p> |
| `property_groups` | HashMap<String, String> | <p>The maximum number of results to return at one time. The default is 25.</p>
         <p>Valid Range: Minimum value of 1. Maximum value of 250.</p> |
| `composite_component_types` | HashMap<String, String> | <p>This is an object that maps strings to <code>compositeComponentTypes</code> of the <code>componentType</code>. <code>CompositeComponentType</code> is referenced by <code>componentTypeId</code>.</p> |
| `extends_from` | Vec<String> | <p>The name of the parent component type that this component type extends.</p> |
| `component_type_name` | String | <p>The component type name.</p> |
| `arn` | String | <p>The ARN of the component type.</p> |
| `is_singleton` | bool | <p>A Boolean value that specifies whether an entity can have more than one component of
         this type.</p> |
| `workspace_id` | String | <p>The ID of the workspace that contains the component type.</p> |
| `property_definitions` | HashMap<String, String> | <p>An object that maps strings to the property definitions in the component type. Each
         string in the mapping must be unique to this object.</p> |
| `creation_date_time` | String | <p>The date and time when the component type was created.</p> |
| `is_abstract` | bool | <p>A Boolean value that specifies whether the component type is abstract.</p> |
| `sync_source` | String | <p>The syncSource of the SyncJob, if this entity was created by a SyncJob.</p> |
| `component_type_id` | String | <p>The ID of the component type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create component_type
component_type = provider.iottwinmaker.Component_type {
    workspace_id = "value"  # <p>The ID of the workspace that contains the component type.</p>
    component_type_id = "value"  # <p>The ID of the component type.</p>
}

# Access component_type outputs
component_type_id = component_type.id
component_type_is_schema_initialized = component_type.is_schema_initialized
component_type_status = component_type.status
component_type_update_date_time = component_type.update_date_time
component_type_functions = component_type.functions
component_type_description = component_type.description
component_type_property_groups = component_type.property_groups
component_type_composite_component_types = component_type.composite_component_types
component_type_extends_from = component_type.extends_from
component_type_component_type_name = component_type.component_type_name
component_type_arn = component_type.arn
component_type_is_singleton = component_type.is_singleton
component_type_workspace_id = component_type.workspace_id
component_type_property_definitions = component_type.property_definitions
component_type_creation_date_time = component_type.creation_date_time
component_type_is_abstract = component_type.is_abstract
component_type_sync_source = component_type.sync_source
component_type_component_type_id = component_type.component_type_id
```

---


### Property_value_history

PropertyValueHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `property_values` | Vec<String> | <p>An object that maps strings to the property definitions in the component type. Each
         string in the mapping must be unique to this object.</p> |
| `next_token` | String | <p>The string that specifies the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access property_value_history outputs
property_value_history_id = property_value_history.id
property_value_history_property_values = property_value_history.property_values
property_value_history_next_token = property_value_history.next_token
```

---


### Pricing_plan

PricingPlan resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pricing_mode` | String | ✅ | <p>The pricing mode.</p> |
| `bundle_names` | Vec<String> |  | <p>The bundle names.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `current_pricing_plan` | String | <p>The chosen pricing plan for the current billing cycle.</p> |
| `pending_pricing_plan` | String | <p>The pending pricing plan.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access pricing_plan outputs
pricing_plan_id = pricing_plan.id
pricing_plan_current_pricing_plan = pricing_plan.current_pricing_plan
pricing_plan_pending_pricing_plan = pricing_plan.pending_pricing_plan
```

---


### Sync_job

SyncJob resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `workspace_id` | String | ✅ | <p>The workspace ID.</p> |
| `tags` | HashMap<String, String> |  | <p>The SyncJob tags.</p> |
| `sync_source` | String | ✅ | <p>The sync source.</p>
         <note>
            <p>Currently the only supported syncSoource is <code>SITEWISE </code>.</p>
         </note> |
| `sync_role` | String | ✅ | <p>The SyncJob IAM role. This IAM role is used by the SyncJob to read from the syncSource,
         and create, update, or delete the corresponding resources.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sync_source` | String | <p>The sync soucre.</p>
         <note>
            <p>Currently the only supported syncSource is <code>SITEWISE </code>.</p>
         </note> |
| `status` | String | <p>The SyncJob response status.</p> |
| `workspace_id` | String | <p>The ID of the workspace that contains the sync job.</p> |
| `update_date_time` | String | <p>The update date and time.</p> |
| `sync_role` | String | <p>The sync IAM role.</p> |
| `arn` | String | <p>The sync job ARN.</p> |
| `creation_date_time` | String | <p>The creation date and time.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sync_job
sync_job = provider.iottwinmaker.Sync_job {
    workspace_id = "value"  # <p>The workspace ID.</p>
    sync_source = "value"  # <p>The sync source.</p>
         <note>
            <p>Currently the only supported syncSoource is <code>SITEWISE </code>.</p>
         </note>
    sync_role = "value"  # <p>The SyncJob IAM role. This IAM role is used by the SyncJob to read from the syncSource,
         and create, update, or delete the corresponding resources.</p>
}

# Access sync_job outputs
sync_job_id = sync_job.id
sync_job_sync_source = sync_job.sync_source
sync_job_status = sync_job.status
sync_job_workspace_id = sync_job.workspace_id
sync_job_update_date_time = sync_job.update_date_time
sync_job_sync_role = sync_job.sync_role
sync_job_arn = sync_job.arn
sync_job_creation_date_time = sync_job.creation_date_time
```

---


### Metadata_transfer_job

MetadataTransferJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination` | String | ✅ | <p>The metadata transfer job destination.</p> |
| `sources` | Vec<String> | ✅ | <p>The metadata transfer job sources.</p> |
| `description` | String |  | <p>The metadata transfer job description.</p> |
| `metadata_transfer_job_id` | String |  | <p>The metadata transfer job Id.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The metadata transfer job's status.</p> |
| `arn` | String | <p>The metadata transfer job ARN.</p> |
| `report_url` | String | <p>The metadata transfer job's report URL.</p> |
| `sources` | Vec<String> | <p>The metadata transfer job's sources.</p> |
| `creation_date_time` | String | <p>The metadata transfer job's creation DateTime property.</p> |
| `metadata_transfer_job_role` | String | <p>The metadata transfer job's role.</p> |
| `metadata_transfer_job_id` | String | <p>The metadata transfer job Id.</p> |
| `update_date_time` | String | <p>The metadata transfer job's update DateTime property.</p> |
| `progress` | String | <p>The metadata transfer job's progress.</p> |
| `description` | String | <p>The metadata transfer job description.</p> |
| `destination` | String | <p>The metadata transfer job's destination.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create metadata_transfer_job
metadata_transfer_job = provider.iottwinmaker.Metadata_transfer_job {
    destination = "value"  # <p>The metadata transfer job destination.</p>
    sources = "value"  # <p>The metadata transfer job sources.</p>
}

# Access metadata_transfer_job outputs
metadata_transfer_job_id = metadata_transfer_job.id
metadata_transfer_job_status = metadata_transfer_job.status
metadata_transfer_job_arn = metadata_transfer_job.arn
metadata_transfer_job_report_url = metadata_transfer_job.report_url
metadata_transfer_job_sources = metadata_transfer_job.sources
metadata_transfer_job_creation_date_time = metadata_transfer_job.creation_date_time
metadata_transfer_job_metadata_transfer_job_role = metadata_transfer_job.metadata_transfer_job_role
metadata_transfer_job_metadata_transfer_job_id = metadata_transfer_job.metadata_transfer_job_id
metadata_transfer_job_update_date_time = metadata_transfer_job.update_date_time
metadata_transfer_job_progress = metadata_transfer_job.progress
metadata_transfer_job_description = metadata_transfer_job.description
metadata_transfer_job_destination = metadata_transfer_job.destination
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple property_value resources
property_value_0 = provider.iottwinmaker.Property_value {
}
property_value_1 = provider.iottwinmaker.Property_value {
}
property_value_2 = provider.iottwinmaker.Property_value {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    property_value = provider.iottwinmaker.Property_value {
    }
```

---

## Related Documentation

- [AWS Iottwinmaker Documentation](https://docs.aws.amazon.com/iottwinmaker/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
