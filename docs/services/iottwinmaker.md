# Iottwinmaker Service



**Resources**: 9

---

## Overview

The iottwinmaker service provides access to 9 resource types:

- [Property_value_history](#property_value_history) [R]
- [Component_type](#component_type) [CRUD]
- [Sync_job](#sync_job) [CRD]
- [Metadata_transfer_job](#metadata_transfer_job) [CR]
- [Pricing_plan](#pricing_plan) [RU]
- [Workspace](#workspace) [CRUD]
- [Entity](#entity) [CRUD]
- [Property_value](#property_value) [R]
- [Scene](#scene) [CRUD]

---

## Resources


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


### Component_type

ComponentType resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `functions` | HashMap<String, String> |  | <p>An object that maps strings to the functions in the component type. Each string in the
         mapping must be unique to this object.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata that you can use to manage the component type.</p> |
| `description` | String |  | <p>The description of the component type.</p> |
| `workspace_id` | String | ✅ | <p>The ID of the workspace that contains the component type.</p> |
| `component_type_name` | String |  | <p>A friendly name for the component type.</p> |
| `property_groups` | HashMap<String, String> |  | <p/> |
| `composite_component_types` | HashMap<String, String> |  | <p>This is an object that maps strings to <code>compositeComponentTypes</code> of the <code>componentType</code>. 
          <code>CompositeComponentType</code> is referenced by <code>componentTypeId</code>.</p> |
| `component_type_id` | String | ✅ | <p>The ID of the component type.</p> |
| `extends_from` | Vec<String> |  | <p>Specifies the parent component type to extend.</p> |
| `property_definitions` | HashMap<String, String> |  | <p>An object that maps strings to the property definitions in the component type. Each
         string in the mapping must be unique to this object.</p> |
| `is_singleton` | bool |  | <p>A Boolean value that specifies whether an entity can have more than one component of
         this type.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `property_definitions` | HashMap<String, String> | <p>An object that maps strings to the property definitions in the component type. Each
         string in the mapping must be unique to this object.</p> |
| `creation_date_time` | String | <p>The date and time when the component type was created.</p> |
| `is_schema_initialized` | bool | <p>A Boolean value that specifies whether the component type has a schema initializer and
         that the schema initializer has run.</p> |
| `status` | String | <p>The current status of the component type.</p> |
| `component_type_name` | String | <p>The component type name.</p> |
| `property_groups` | HashMap<String, String> | <p>The maximum number of results to return at one time. The default is 25.</p>
         <p>Valid Range: Minimum value of 1. Maximum value of 250.</p> |
| `is_abstract` | bool | <p>A Boolean value that specifies whether the component type is abstract.</p> |
| `workspace_id` | String | <p>The ID of the workspace that contains the component type.</p> |
| `component_type_id` | String | <p>The ID of the component type.</p> |
| `is_singleton` | bool | <p>A Boolean value that specifies whether an entity can have more than one component of
         this type.</p> |
| `sync_source` | String | <p>The syncSource of the SyncJob, if this entity was created by a SyncJob.</p> |
| `extends_from` | Vec<String> | <p>The name of the parent component type that this component type extends.</p> |
| `composite_component_types` | HashMap<String, String> | <p>This is an object that maps strings to <code>compositeComponentTypes</code> of the <code>componentType</code>. <code>CompositeComponentType</code> is referenced by <code>componentTypeId</code>.</p> |
| `description` | String | <p>The description of the component type.</p> |
| `functions` | HashMap<String, String> | <p>An object that maps strings to the functions in the component type. Each string in the
         mapping must be unique to this object.</p> |
| `update_date_time` | String | <p>The date and time when the component was last updated.</p> |
| `arn` | String | <p>The ARN of the component type.</p> |


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
component_type_property_definitions = component_type.property_definitions
component_type_creation_date_time = component_type.creation_date_time
component_type_is_schema_initialized = component_type.is_schema_initialized
component_type_status = component_type.status
component_type_component_type_name = component_type.component_type_name
component_type_property_groups = component_type.property_groups
component_type_is_abstract = component_type.is_abstract
component_type_workspace_id = component_type.workspace_id
component_type_component_type_id = component_type.component_type_id
component_type_is_singleton = component_type.is_singleton
component_type_sync_source = component_type.sync_source
component_type_extends_from = component_type.extends_from
component_type_composite_component_types = component_type.composite_component_types
component_type_description = component_type.description
component_type_functions = component_type.functions
component_type_update_date_time = component_type.update_date_time
component_type_arn = component_type.arn
```

---


### Sync_job

SyncJob resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>The SyncJob tags.</p> |
| `sync_source` | String | ✅ | <p>The sync source.</p>
         <note>
            <p>Currently the only supported syncSoource is <code>SITEWISE </code>.</p>
         </note> |
| `workspace_id` | String | ✅ | <p>The workspace ID.</p> |
| `sync_role` | String | ✅ | <p>The SyncJob IAM role. This IAM role is used by the SyncJob to read from the syncSource,
         and create, update, or delete the corresponding resources.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sync_source` | String | <p>The sync soucre.</p>
         <note>
            <p>Currently the only supported syncSource is <code>SITEWISE </code>.</p>
         </note> |
| `sync_role` | String | <p>The sync IAM role.</p> |
| `workspace_id` | String | <p>The ID of the workspace that contains the sync job.</p> |
| `arn` | String | <p>The sync job ARN.</p> |
| `status` | String | <p>The SyncJob response status.</p> |
| `creation_date_time` | String | <p>The creation date and time.</p> |
| `update_date_time` | String | <p>The update date and time.</p> |


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
    sync_source = "value"  # <p>The sync source.</p>
         <note>
            <p>Currently the only supported syncSoource is <code>SITEWISE </code>.</p>
         </note>
    workspace_id = "value"  # <p>The workspace ID.</p>
    sync_role = "value"  # <p>The SyncJob IAM role. This IAM role is used by the SyncJob to read from the syncSource,
         and create, update, or delete the corresponding resources.</p>
}

# Access sync_job outputs
sync_job_id = sync_job.id
sync_job_sync_source = sync_job.sync_source
sync_job_sync_role = sync_job.sync_role
sync_job_workspace_id = sync_job.workspace_id
sync_job_arn = sync_job.arn
sync_job_status = sync_job.status
sync_job_creation_date_time = sync_job.creation_date_time
sync_job_update_date_time = sync_job.update_date_time
```

---


### Metadata_transfer_job

MetadataTransferJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `metadata_transfer_job_id` | String |  | <p>The metadata transfer job Id.</p> |
| `description` | String |  | <p>The metadata transfer job description.</p> |
| `sources` | Vec<String> | ✅ | <p>The metadata transfer job sources.</p> |
| `destination` | String | ✅ | <p>The metadata transfer job destination.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `destination` | String | <p>The metadata transfer job's destination.</p> |
| `report_url` | String | <p>The metadata transfer job's report URL.</p> |
| `arn` | String | <p>The metadata transfer job ARN.</p> |
| `description` | String | <p>The metadata transfer job description.</p> |
| `sources` | Vec<String> | <p>The metadata transfer job's sources.</p> |
| `update_date_time` | String | <p>The metadata transfer job's update DateTime property.</p> |
| `metadata_transfer_job_id` | String | <p>The metadata transfer job Id.</p> |
| `creation_date_time` | String | <p>The metadata transfer job's creation DateTime property.</p> |
| `status` | String | <p>The metadata transfer job's status.</p> |
| `progress` | String | <p>The metadata transfer job's progress.</p> |
| `metadata_transfer_job_role` | String | <p>The metadata transfer job's role.</p> |


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
    sources = "value"  # <p>The metadata transfer job sources.</p>
    destination = "value"  # <p>The metadata transfer job destination.</p>
}

# Access metadata_transfer_job outputs
metadata_transfer_job_id = metadata_transfer_job.id
metadata_transfer_job_destination = metadata_transfer_job.destination
metadata_transfer_job_report_url = metadata_transfer_job.report_url
metadata_transfer_job_arn = metadata_transfer_job.arn
metadata_transfer_job_description = metadata_transfer_job.description
metadata_transfer_job_sources = metadata_transfer_job.sources
metadata_transfer_job_update_date_time = metadata_transfer_job.update_date_time
metadata_transfer_job_metadata_transfer_job_id = metadata_transfer_job.metadata_transfer_job_id
metadata_transfer_job_creation_date_time = metadata_transfer_job.creation_date_time
metadata_transfer_job_status = metadata_transfer_job.status
metadata_transfer_job_progress = metadata_transfer_job.progress
metadata_transfer_job_metadata_transfer_job_role = metadata_transfer_job.metadata_transfer_job_role
```

---


### Pricing_plan

PricingPlan resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bundle_names` | Vec<String> |  | <p>The bundle names.</p> |
| `pricing_mode` | String | ✅ | <p>The pricing mode.</p> |


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


### Workspace

Workspace resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `workspace_id` | String | ✅ | <p>The ID of the workspace.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata that you can use to manage the workspace</p> |
| `description` | String |  | <p>The description of the workspace.</p> |
| `role` | String |  | <p>The ARN of the execution role associated with the workspace.</p> |
| `s3_location` | String |  | <p>The ARN of the S3 bucket where resources associated with the workspace are
         stored.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workspace_id` | String | <p>The ID of the workspace.</p> |
| `creation_date_time` | String | <p>The date and time when the workspace was created.</p> |
| `s3_location` | String | <p>The ARN of the S3 bucket where resources associated with the workspace are
         stored.</p> |
| `role` | String | <p>The ARN of the execution role associated with the workspace.</p> |
| `description` | String | <p>The description of the workspace.</p> |
| `arn` | String | <p>The ARN of the workspace.</p> |
| `linked_services` | Vec<String> | <p>A list of services that are linked to the workspace.</p> |
| `update_date_time` | String | <p>The date and time when the workspace was last updated.</p> |


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
workspace_workspace_id = workspace.workspace_id
workspace_creation_date_time = workspace.creation_date_time
workspace_s3_location = workspace.s3_location
workspace_role = workspace.role
workspace_description = workspace.description
workspace_arn = workspace.arn
workspace_linked_services = workspace.linked_services
workspace_update_date_time = workspace.update_date_time
```

---


### Entity

Entity resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `composite_components` | HashMap<String, String> |  | <p>This is an object that maps strings to <code>compositeComponent</code> updates in the request. 
          Each key of the map represents the <code>componentPath</code> of the <code>compositeComponent</code>.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata that you can use to manage the entity.</p> |
| `entity_id` | String |  | <p>The ID of the entity.</p> |
| `description` | String |  | <p>The description of the entity.</p> |
| `parent_entity_id` | String |  | <p>The ID of the entity's parent entity.</p> |
| `components` | HashMap<String, String> |  | <p>An object that maps strings to the components in the entity. Each string in the mapping
         must be unique to this object.</p> |
| `entity_name` | String | ✅ | <p>The name of the entity.</p> |
| `workspace_id` | String | ✅ | <p>The ID of the workspace that contains the entity.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | <p>The ARN of the entity.</p> |
| `entity_id` | String | <p>The ID of the entity.</p> |
| `components` | HashMap<String, String> | <p>An object that maps strings to the components in the entity. Each string in the mapping
         must be unique to this object.</p> |
| `parent_entity_id` | String | <p>The ID of the parent entity for this entity.</p> |
| `description` | String | <p>The description of the entity.</p> |
| `creation_date_time` | String | <p>The date and time when the entity was created.</p> |
| `workspace_id` | String | <p>The ID of the workspace.</p> |
| `update_date_time` | String | <p>The date and time when the entity was last updated.</p> |
| `sync_source` | String | <p>The syncSource of the sync job, if this entity was created by a sync job.</p> |
| `are_all_components_returned` | bool | <p>This flag notes whether all components are returned in the API response. The maximum number of components returned is 30.</p> |
| `entity_name` | String | <p>The name of the entity.</p> |
| `status` | String | <p>The current status of the entity.</p> |
| `has_child_entities` | bool | <p>A Boolean value that specifies whether the entity has associated child entities.</p> |


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
    entity_name = "value"  # <p>The name of the entity.</p>
    workspace_id = "value"  # <p>The ID of the workspace that contains the entity.</p>
}

# Access entity outputs
entity_id = entity.id
entity_arn = entity.arn
entity_entity_id = entity.entity_id
entity_components = entity.components
entity_parent_entity_id = entity.parent_entity_id
entity_description = entity.description
entity_creation_date_time = entity.creation_date_time
entity_workspace_id = entity.workspace_id
entity_update_date_time = entity.update_date_time
entity_sync_source = entity.sync_source
entity_are_all_components_returned = entity.are_all_components_returned
entity_entity_name = entity.entity_name
entity_status = entity.status
entity_has_child_entities = entity.has_child_entities
```

---


### Property_value

PropertyValue resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The string that specifies the next page of results.</p> |
| `tabular_property_values` | Vec<Vec<HashMap<String, String>>> | <p>A table of property values.</p> |
| `property_values` | HashMap<String, String> | <p>An object that maps strings to the properties and latest property values in the
         response. Each string in the mapping must be unique to this object.</p> |


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
property_value_next_token = property_value.next_token
property_value_tabular_property_values = property_value.tabular_property_values
property_value_property_values = property_value.property_values
```

---


### Scene

Scene resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>Metadata that you can use to manage the scene.</p> |
| `scene_metadata` | HashMap<String, String> |  | <p>The request metadata.</p> |
| `content_location` | String | ✅ | <p>The relative path that specifies the location of the content definition file.</p> |
| `workspace_id` | String | ✅ | <p>The ID of the workspace that contains the scene.</p> |
| `description` | String |  | <p>The description for this scene.</p> |
| `scene_id` | String | ✅ | <p>The ID of the scene.</p> |
| `capabilities` | Vec<String> |  | <p>A list of capabilities that the scene uses to render itself.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | <p>The SceneResponse error.</p> |
| `scene_metadata` | HashMap<String, String> | <p>The response metadata.</p> |
| `arn` | String | <p>The ARN of the scene.</p> |
| `scene_id` | String | <p>The ID of the scene.</p> |
| `update_date_time` | String | <p>The date and time when the scene was last updated.</p> |
| `workspace_id` | String | <p>The ID of the workspace that contains the scene.</p> |
| `generated_scene_metadata` | HashMap<String, String> | <p>The generated scene metadata.</p> |
| `creation_date_time` | String | <p>The date and time when the scene was created.</p> |
| `description` | String | <p>The description of the scene.</p> |
| `content_location` | String | <p>The relative path that specifies the location of the content definition file.</p> |
| `capabilities` | Vec<String> | <p>A list of capabilities that the scene uses to render.</p> |


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
    content_location = "value"  # <p>The relative path that specifies the location of the content definition file.</p>
    workspace_id = "value"  # <p>The ID of the workspace that contains the scene.</p>
    scene_id = "value"  # <p>The ID of the scene.</p>
}

# Access scene outputs
scene_id = scene.id
scene_error = scene.error
scene_scene_metadata = scene.scene_metadata
scene_arn = scene.arn
scene_scene_id = scene.scene_id
scene_update_date_time = scene.update_date_time
scene_workspace_id = scene.workspace_id
scene_generated_scene_metadata = scene.generated_scene_metadata
scene_creation_date_time = scene.creation_date_time
scene_description = scene.description
scene_content_location = scene.content_location
scene_capabilities = scene.capabilities
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple property_value_history resources
property_value_history_0 = provider.iottwinmaker.Property_value_history {
}
property_value_history_1 = provider.iottwinmaker.Property_value_history {
}
property_value_history_2 = provider.iottwinmaker.Property_value_history {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    property_value_history = provider.iottwinmaker.Property_value_history {
    }
```

---

## Related Documentation

- [AWS Iottwinmaker Documentation](https://docs.aws.amazon.com/iottwinmaker/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
