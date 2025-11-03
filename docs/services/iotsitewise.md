# Iotsitewise Service



**Resources**: 26

---

## Overview

The iotsitewise service provides access to 26 resource types:

- [Time_series](#time_series) [RD]
- [Execution](#execution) [R]
- [Asset_property](#asset_property) [RU]
- [Gateway](#gateway) [CRUD]
- [Computation_model_execution_summary](#computation_model_execution_summary) [R]
- [Asset_property_value](#asset_property_value) [R]
- [Dashboard](#dashboard) [CRUD]
- [Action](#action) [R]
- [Project](#project) [CRUD]
- [Asset_model](#asset_model) [CRUD]
- [Asset_model_composite_model](#asset_model_composite_model) [CRUD]
- [Logging_options](#logging_options) [CR]
- [Bulk_import_job](#bulk_import_job) [CR]
- [Computation_model](#computation_model) [CRUD]
- [Default_encryption_configuration](#default_encryption_configuration) [CR]
- [Dataset](#dataset) [CRUD]
- [Asset](#asset) [CRUD]
- [Asset_model_interface_relationship](#asset_model_interface_relationship) [CRD]
- [Asset_composite_model](#asset_composite_model) [R]
- [Interpolated_asset_property_values](#interpolated_asset_property_values) [R]
- [Gateway_capability_configuration](#gateway_capability_configuration) [RU]
- [Storage_configuration](#storage_configuration) [CR]
- [Asset_property_value_history](#asset_property_value_history) [R]
- [Portal](#portal) [CRUD]
- [Asset_property_aggregates](#asset_property_aggregates) [R]
- [Access_policy](#access_policy) [CRUD]

---

## Resources


### Time_series

TimeSeries resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `asset_id` | String | <p>The ID of the asset in which the asset property was created.</p> |
| `time_series_arn` | String | <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the time series, which has the following format.</p>
         <p>
            <code>arn:${Partition}:iotsitewise:${Region}:${Account}:time-series/${TimeSeriesId}</code>
         </p> |
| `data_type` | String | <p>The data type of the time series.</p>
         <p>If you specify <code>STRUCT</code>, you must also specify <code>dataTypeSpec</code> to identify the type of the structure for this time series.</p> |
| `time_series_last_update_date` | String | <p>The date that the time series was last updated, in Unix epoch time.</p> |
| `alias` | String | <p>The alias that identifies the time series.</p> |
| `time_series_creation_date` | String | <p>The date that the time series was created, in Unix epoch time.</p> |
| `property_id` | String | <p>The ID of the asset property, in UUID format.</p> |
| `data_type_spec` | String | <p>The data type of the structure for this time series. This parameter is required for time series
      that have the <code>STRUCT</code> data type.</p>
         <p>The options for this parameter depend on the type of the composite model
      in which you created the asset property that is associated with your time series.
      Use <code>AWS/ALARM_STATE</code> for alarm state in alarm composite models.</p> |
| `time_series_id` | String | <p>The ID of the time series.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access time_series outputs
time_series_id = time_series.id
time_series_asset_id = time_series.asset_id
time_series_time_series_arn = time_series.time_series_arn
time_series_data_type = time_series.data_type
time_series_time_series_last_update_date = time_series.time_series_last_update_date
time_series_alias = time_series.alias
time_series_time_series_creation_date = time_series.time_series_creation_date
time_series_property_id = time_series.property_id
time_series_data_type_spec = time_series.data_type_spec
time_series_time_series_id = time_series.time_series_id
```

---


### Execution

Execution resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `action_type` | String | <p>The type of action exectued.</p> |
| `execution_end_time` | String | <p>The time the process ended.</p> |
| `execution_start_time` | String | <p>The time the process started.</p> |
| `execution_status` | String | <p>The status of the execution process.</p> |
| `execution_details` | HashMap<String, String> | <p>Provides detailed information about the execution of your anomaly detection models. This
      includes model metrics and training timestamps for both training and inference actions.</p>
         <ul>
            <li>
               <p> The training action (Amazon Web Services/ANOMALY_DETECTION_TRAINING), includes performance metrics
          that help you compare different versions of your anomaly detection models. These metrics
          provide insights into the model's performance during the training process. </p>
            </li>
            <li>
               <p> The inference action (Amazon Web Services/ANOMALY_DETECTION_INFERENCE), includes information about
          the results of executing your anomaly detection models. This helps you understand the
          output of your models and assess their performance. </p>
            </li>
         </ul> |
| `execution_entity_version` | String | <p>Entity version used for the execution.</p> |
| `execution_result` | HashMap<String, String> | <p>The result of the execution.</p> |
| `resolve_to` | String | <p>The detailed resource this execution resolves to.</p> |
| `target_resource` | String |  |
| `execution_id` | String | <p>The ID of the execution.</p> |
| `target_resource_version` | String | <p>The version of the target resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access execution outputs
execution_id = execution.id
execution_action_type = execution.action_type
execution_execution_end_time = execution.execution_end_time
execution_execution_start_time = execution.execution_start_time
execution_execution_status = execution.execution_status
execution_execution_details = execution.execution_details
execution_execution_entity_version = execution.execution_entity_version
execution_execution_result = execution.execution_result
execution_resolve_to = execution.resolve_to
execution_target_resource = execution.target_resource
execution_execution_id = execution.execution_id
execution_target_resource_version = execution.target_resource_version
```

---


### Asset_property

AssetProperty resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `asset_id` | String | ✅ | <p>The ID of the asset to be updated. This can be either the actual ID in UUID format, or else <code>externalId:</code> followed by the external ID, if it has one.
    For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-id-references">Referencing objects with external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `property_unit` | String |  | <p>The unit of measure (such as Newtons or RPM) of the asset property. If you don't specify a
      value for this parameter, the service uses the value of the <code>assetModelProperty</code> in
      the asset model.</p> |
| `property_id` | String | ✅ | <p>The ID of the asset property to be updated. This can be either the actual ID in UUID format, or else <code>externalId:</code> followed by the external ID, if it has one.
    For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-id-references">Referencing objects with external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `property_notification_state` | String |  | <p>The MQTT notification state (enabled or disabled) for this asset property.
      When the notification state is enabled, IoT SiteWise publishes property value
      updates to a unique MQTT topic. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/interact-with-other-services.html">Interacting with other services</a> in the <i>IoT SiteWise User Guide</i>.</p>
         <p>If you omit this parameter, the notification state is set to <code>DISABLED</code>.</p> |
| `property_alias` | String |  | <p>The alias that identifies the property, such as an OPC-UA server data stream path
        (for example, <code>/company/windfarm/3/turbine/7/temperature</code>). For more information, see
        <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/connect-data-streams.html">Mapping industrial data streams to asset properties</a> in the
        <i>IoT SiteWise User Guide</i>.</p>
         <p>If you omit this parameter, the alias is removed from the property.</p> |
| `client_token` | String |  | <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `composite_model` | String | <p>The composite model that declares this asset property, if this asset property exists in a
      composite model.</p> |
| `asset_model_id` | String | <p>The ID of the asset model, in UUID format.</p> |
| `asset_id` | String | <p>The ID of the asset, in UUID format.</p> |
| `asset_external_id` | String | <p>The external ID of the asset. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-ids">Using external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `asset_name` | String | <p>The name of the asset.</p> |
| `asset_property` | String | <p>The asset property's definition, alias, and notification state.</p>
         <p>This response includes this object for normal asset properties. If you describe an asset
      property in a composite model, this response includes the asset property information in
        <code>compositeModel</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access asset_property outputs
asset_property_id = asset_property.id
asset_property_composite_model = asset_property.composite_model
asset_property_asset_model_id = asset_property.asset_model_id
asset_property_asset_id = asset_property.asset_id
asset_property_asset_external_id = asset_property.asset_external_id
asset_property_asset_name = asset_property.asset_name
asset_property_asset_property = asset_property.asset_property
```

---


### Gateway

Gateway resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs that contain metadata for the gateway. For more information, see
        <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/tag-resources.html">Tagging your IoT SiteWise
        resources</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `gateway_platform` | String | ✅ | <p>The gateway's platform. You can only specify one platform in a gateway.</p> |
| `gateway_version` | String |  | <p>The version of the gateway to create. Specify <code>3</code> to create an MQTT-enabled, V3
      gateway and <code>2</code> to create a Classic streams, V2 gateway. If not specified, the
      default is <code>2</code> (Classic streams, V2 gateway).</p>
         <note>
            <p>When creating a V3 gateway (<code>gatewayVersion=3</code>) with the
          <code>GreengrassV2</code> platform, you must also specify the
          <code>coreDeviceOperatingSystem</code> parameter.</p>
         </note>
         <p> We recommend creating an MQTT-enabled gateway for self-hosted gateways and Siemens
      Industrial Edge gateways. For more information on gateway versions, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/gateways.html">Use Amazon Web Services IoT SiteWise Edge Edge
        gateways</a>.</p> |
| `gateway_name` | String | ✅ | <p>A unique name for the gateway.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `gateway_id` | String | <p>The ID of the gateway device.</p> |
| `gateway_version` | String | <p>The version of the gateway. A value of <code>3</code> indicates an MQTT-enabled, V3
      gateway, while <code>2</code> indicates a Classic streams, V2 gateway.</p> |
| `gateway_arn` | String | <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the gateway, which has the following format.</p>
         <p>
            <code>arn:${Partition}:iotsitewise:${Region}:${Account}:gateway/${GatewayId}</code>
         </p> |
| `gateway_platform` | String | <p>The gateway's platform.</p> |
| `last_update_date` | String | <p>The date the gateway was last updated, in Unix epoch time.</p> |
| `gateway_capability_summaries` | Vec<String> | <p>A list of gateway capability summaries that each contain a namespace and status. Each
      gateway capability defines data sources for the gateway. To retrieve a capability
      configuration's definition, use <a href="https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/API_DescribeGatewayCapabilityConfiguration.html">DescribeGatewayCapabilityConfiguration</a>.</p> |
| `creation_date` | String | <p>The date the gateway was created, in Unix epoch time.</p> |
| `gateway_name` | String | <p>The name of the gateway.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create gateway
gateway = provider.iotsitewise.Gateway {
    gateway_platform = "value"  # <p>The gateway's platform. You can only specify one platform in a gateway.</p>
    gateway_name = "value"  # <p>A unique name for the gateway.</p>
}

# Access gateway outputs
gateway_id = gateway.id
gateway_gateway_id = gateway.gateway_id
gateway_gateway_version = gateway.gateway_version
gateway_gateway_arn = gateway.gateway_arn
gateway_gateway_platform = gateway.gateway_platform
gateway_last_update_date = gateway.last_update_date
gateway_gateway_capability_summaries = gateway.gateway_capability_summaries
gateway_creation_date = gateway.creation_date
gateway_gateway_name = gateway.gateway_name
```

---


### Computation_model_execution_summary

ComputationModelExecutionSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `computation_model_id` | String | <p>The ID of the computation model.</p> |
| `computation_model_execution_summary` | HashMap<String, String> | <p>Contains the execution summary of the computation model.</p> |
| `resolve_to` | String | <p>The detailed resource this execution summary resolves to.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access computation_model_execution_summary outputs
computation_model_execution_summary_id = computation_model_execution_summary.id
computation_model_execution_summary_computation_model_id = computation_model_execution_summary.computation_model_id
computation_model_execution_summary_computation_model_execution_summary = computation_model_execution_summary.computation_model_execution_summary
computation_model_execution_summary_resolve_to = computation_model_execution_summary.resolve_to
```

---


### Asset_property_value

AssetPropertyValue resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `property_value` | String | <p>The current asset property value.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access asset_property_value outputs
asset_property_value_id = asset_property_value.id
asset_property_value_property_value = asset_property_value.property_value
```

---


### Dashboard

Dashboard resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dashboard_name` | String | ✅ | <p>A friendly name for the dashboard.</p> |
| `dashboard_description` | String |  | <p>A description for the dashboard.</p> |
| `dashboard_definition` | String | ✅ | <p>The dashboard definition specified in a JSON literal.</p>
         <ul>
            <li>
               <p>IoT SiteWise Monitor (Classic) see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/create-dashboards-using-aws-cli.html">Create dashboards (CLI)</a>
               </p>
            </li>
            <li>
               <p>IoT SiteWise Monitor (AI-aware) see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/create-dashboards-ai-dashboard-cli.html">Create dashboards (CLI)</a>
               </p>
            </li>
         </ul>
         <p>in the <i>IoT SiteWise User Guide</i>
         </p> |
| `client_token` | String |  | <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs that contain metadata for the dashboard. For more information,
      see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/tag-resources.html">Tagging your IoT SiteWise
        resources</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `project_id` | String | ✅ | <p>The ID of the project in which to create the dashboard.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dashboard_creation_date` | String | <p>The date the dashboard was created, in Unix epoch time.</p> |
| `dashboard_id` | String | <p>The ID of the dashboard.</p> |
| `dashboard_last_update_date` | String | <p>The date the dashboard was last updated, in Unix epoch time.</p> |
| `dashboard_arn` | String | <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the dashboard, which has the following format.</p>
         <p>
            <code>arn:${Partition}:iotsitewise:${Region}:${Account}:dashboard/${DashboardId}</code>
         </p> |
| `project_id` | String | <p>The ID of the project that the dashboard is in.</p> |
| `dashboard_name` | String | <p>The name of the dashboard.</p> |
| `dashboard_definition` | String | <p>The dashboard's definition JSON literal. For detailed information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/create-dashboards-using-aws-cli.html">Creating
        dashboards (CLI)</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `dashboard_description` | String | <p>The dashboard's description.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dashboard
dashboard = provider.iotsitewise.Dashboard {
    dashboard_name = "value"  # <p>A friendly name for the dashboard.</p>
    dashboard_definition = "value"  # <p>The dashboard definition specified in a JSON literal.</p>
         <ul>
            <li>
               <p>IoT SiteWise Monitor (Classic) see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/create-dashboards-using-aws-cli.html">Create dashboards (CLI)</a>
               </p>
            </li>
            <li>
               <p>IoT SiteWise Monitor (AI-aware) see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/create-dashboards-ai-dashboard-cli.html">Create dashboards (CLI)</a>
               </p>
            </li>
         </ul>
         <p>in the <i>IoT SiteWise User Guide</i>
         </p>
    project_id = "value"  # <p>The ID of the project in which to create the dashboard.</p>
}

# Access dashboard outputs
dashboard_id = dashboard.id
dashboard_dashboard_creation_date = dashboard.dashboard_creation_date
dashboard_dashboard_id = dashboard.dashboard_id
dashboard_dashboard_last_update_date = dashboard.dashboard_last_update_date
dashboard_dashboard_arn = dashboard.dashboard_arn
dashboard_project_id = dashboard.project_id
dashboard_dashboard_name = dashboard.dashboard_name
dashboard_dashboard_definition = dashboard.dashboard_definition
dashboard_dashboard_description = dashboard.dashboard_description
```

---


### Action

Action resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resolve_to` | String | <p>The detailed resource this action resolves to.</p> |
| `action_id` | String | <p>The ID of the action.</p> |
| `target_resource` | String | <p>The resource the action will be taken on.</p> |
| `action_definition_id` | String | <p>The ID of the action definition.</p> |
| `action_payload` | String | <p>The JSON payload of the action.</p> |
| `execution_time` | String | <p>The time the action was executed.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access action outputs
action_id = action.id
action_resolve_to = action.resolve_to
action_action_id = action.action_id
action_target_resource = action.target_resource
action_action_definition_id = action.action_definition_id
action_action_payload = action.action_payload
action_execution_time = action.execution_time
```

---


### Project

Project resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project_description` | String |  | <p>A description for the project.</p> |
| `portal_id` | String | ✅ | <p>The ID of the portal in which to create the project.</p> |
| `project_name` | String | ✅ | <p>A friendly name for the project.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs that contain metadata for the project. For more information, see
        <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/tag-resources.html">Tagging your IoT SiteWise
        resources</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `client_token` | String |  | <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `project_id` | String | <p>The ID of the project.</p> |
| `project_description` | String | <p>The project's description.</p> |
| `portal_id` | String | <p>The ID of the portal that the project is in.</p> |
| `project_last_update_date` | String | <p>The date the project was last updated, in Unix epoch time.</p> |
| `project_arn` | String | <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the project, which has the following format.</p>
         <p>
            <code>arn:${Partition}:iotsitewise:${Region}:${Account}:project/${ProjectId}</code>
         </p> |
| `project_name` | String | <p>The name of the project.</p> |
| `project_creation_date` | String | <p>The date the project was created, in Unix epoch time.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create project
project = provider.iotsitewise.Project {
    portal_id = "value"  # <p>The ID of the portal in which to create the project.</p>
    project_name = "value"  # <p>A friendly name for the project.</p>
}

# Access project outputs
project_id = project.id
project_project_id = project.project_id
project_project_description = project.project_description
project_portal_id = project.portal_id
project_project_last_update_date = project.project_last_update_date
project_project_arn = project.project_arn
project_project_name = project.project_name
project_project_creation_date = project.project_creation_date
```

---


### Asset_model

AssetModel resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `asset_model_description` | String |  | <p>A description for the asset model.</p> |
| `asset_model_id` | String |  | <p>The ID to assign to the asset model, if desired. IoT SiteWise automatically generates a unique ID for you, so this parameter is never required.
    However, if you prefer to supply your own ID instead, you can specify it here in UUID format.
    If you specify your own ID, it must be globally unique.</p> |
| `asset_model_properties` | Vec<String> |  | <p>The property definitions of the asset model. For more information, see
      <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/asset-properties.html">Asset properties</a> in the <i>IoT SiteWise User Guide</i>.</p>
         <p>You can specify up to 200 properties per asset model. For more
      information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/quotas.html">Quotas</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `asset_model_hierarchies` | Vec<String> |  | <p>The hierarchy definitions of the asset model. Each hierarchy specifies an asset model
      whose assets can be children of any other assets created from this asset model. For more
      information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/asset-hierarchies.html">Asset hierarchies</a> in the <i>IoT SiteWise User Guide</i>.</p>
         <p>You can specify up to 10 hierarchies per asset model. For more
      information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/quotas.html">Quotas</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `asset_model_composite_models` | Vec<String> |  | <p>The composite models that are part of this asset model. It groups properties
  (such as attributes, measurements, transforms, and metrics) and child composite models that
      model parts of your industrial equipment. Each composite model has a type that defines the
      properties that the composite model supports. Use composite models to define alarms on this asset model.</p>
         <note>
            <p>When creating custom composite models, you need to use <a href="https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/API_CreateAssetModelCompositeModel.html">CreateAssetModelCompositeModel</a>. For more information,
      see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/create-custom-composite-models.html">Creating custom composite models (Components)</a> in the
      <i>IoT SiteWise User Guide</i>.</p>
         </note> |
| `asset_model_external_id` | String |  | <p>An external ID to assign to the asset model. The external ID must be unique within your Amazon Web Services account. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-ids">Using external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `asset_model_name` | String | ✅ | <p>A unique name for the asset model.</p> |
| `asset_model_type` | String |  | <p>The type of asset model.</p>
         <ul>
            <li>
               <p>
                  <b>ASSET_MODEL</b> – (default) An asset model that you can use to create assets.
   Can't be included as a component in another asset model.</p>
            </li>
            <li>
               <p>
                  <b>COMPONENT_MODEL</b> – A reusable component that you can include in the composite
   models of other asset models. You can't create assets directly from this type of asset model. </p>
            </li>
         </ul> |
| `client_token` | String |  | <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs that contain metadata for the asset model. For more information,
      see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/tag-resources.html">Tagging your IoT SiteWise
        resources</a> in the <i>IoT SiteWise User Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `asset_model_type` | String | <p>The type of asset model.</p>
         <ul>
            <li>
               <p>
                  <b>ASSET_MODEL</b> – (default) An asset model that you can use to create assets.
   Can't be included as a component in another asset model.</p>
            </li>
            <li>
               <p>
                  <b>COMPONENT_MODEL</b> – A reusable component that you can include in the composite
   models of other asset models. You can't create assets directly from this type of asset model. </p>
            </li>
         </ul> |
| `asset_model_external_id` | String | <p>The external ID of the asset model, if any.</p> |
| `asset_model_description` | String | <p>The asset model's description.</p> |
| `e_tag` | String | <p>The entity tag (ETag) is a hash of the retrieved version of the asset model. It's used to make
    concurrent updates safely to the resource. See <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/opt-locking-for-model.html">Optimistic locking for asset model writes</a>
    in the <i>IoT SiteWise User Guide</i>.
    </p>
         <p>See <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/opt-locking-for-model.html"> Optimistic locking for asset
        model writes</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `asset_model_name` | String | <p>The name of the asset model.</p> |
| `asset_model_composite_models` | Vec<String> | <p>The list of built-in composite models for the asset model, such as those with those of
      type <code>AWS/ALARMS</code>.</p> |
| `asset_model_version` | String | <p>The version of the asset model. See <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/model-active-version.html">
          Asset model versions</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `asset_model_status` | String | <p>The current status of the asset model, which contains a state and any error
      message.</p> |
| `asset_model_composite_model_summaries` | Vec<String> | <p>The list of the immediate child custom composite model summaries for the asset
      model.</p> |
| `asset_model_hierarchies` | Vec<String> | <p>A list of asset model hierarchies that each contain a <code>childAssetModelId</code> and a
        <code>hierarchyId</code> (named <code>id</code>). A hierarchy specifies allowed parent/child
      asset relationships for an asset model.</p> |
| `interface_details` | Vec<String> | <p>A list of interface details that describe the interfaces implemented by this asset model,
      including interface asset model IDs and property mappings.</p> |
| `asset_model_arn` | String | <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the asset model, which has the following format.</p>
         <p>
            <code>arn:${Partition}:iotsitewise:${Region}:${Account}:asset-model/${AssetModelId}</code>
         </p> |
| `asset_model_id` | String | <p>The ID of the asset model, in UUID format.</p> |
| `asset_model_creation_date` | String | <p>The date the asset model was created, in Unix epoch time.</p> |
| `asset_model_properties` | Vec<String> | <p>The list of asset properties for the asset model.</p>
         <p>This object doesn't include properties that you define in composite models. You can find
      composite model properties in the <code>assetModelCompositeModels</code> object.</p> |
| `asset_model_last_update_date` | String | <p>The date the asset model was last updated, in Unix epoch time.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create asset_model
asset_model = provider.iotsitewise.Asset_model {
    asset_model_name = "value"  # <p>A unique name for the asset model.</p>
}

# Access asset_model outputs
asset_model_id = asset_model.id
asset_model_asset_model_type = asset_model.asset_model_type
asset_model_asset_model_external_id = asset_model.asset_model_external_id
asset_model_asset_model_description = asset_model.asset_model_description
asset_model_e_tag = asset_model.e_tag
asset_model_asset_model_name = asset_model.asset_model_name
asset_model_asset_model_composite_models = asset_model.asset_model_composite_models
asset_model_asset_model_version = asset_model.asset_model_version
asset_model_asset_model_status = asset_model.asset_model_status
asset_model_asset_model_composite_model_summaries = asset_model.asset_model_composite_model_summaries
asset_model_asset_model_hierarchies = asset_model.asset_model_hierarchies
asset_model_interface_details = asset_model.interface_details
asset_model_asset_model_arn = asset_model.asset_model_arn
asset_model_asset_model_id = asset_model.asset_model_id
asset_model_asset_model_creation_date = asset_model.asset_model_creation_date
asset_model_asset_model_properties = asset_model.asset_model_properties
asset_model_asset_model_last_update_date = asset_model.asset_model_last_update_date
```

---


### Asset_model_composite_model

AssetModelCompositeModel resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `asset_model_id` | String | ✅ | <p>The ID of the asset model this composite model is a part of.</p> |
| `composed_asset_model_id` | String |  | <p>The ID of a component model which is reused to create this composite model.</p> |
| `parent_asset_model_composite_model_id` | String |  | <p>The ID of the parent composite model in this asset model relationship.</p> |
| `if_none_match` | String |  | <p>Accepts <b>*</b> to reject the create request if an active version 
    (specified using <code>matchForVersionType</code> as <code>ACTIVE</code>) already exists for the asset model.</p> |
| `asset_model_composite_model_name` | String | ✅ | <p>A unique name for the composite model.</p> |
| `if_match` | String |  | <p>The expected current entity tag (ETag) for the asset model’s latest or active version (specified using <code>matchForVersionType</code>).   
    The create request is rejected if the tag does not match the latest or active version's current entity tag.
    See <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/opt-locking-for-model.html">Optimistic locking for asset model writes</a>
    in the <i>IoT SiteWise User Guide</i>.</p> |
| `asset_model_composite_model_id` | String |  | <p>The ID of the composite model. IoT SiteWise automatically generates a unique ID for you, so this
      parameter is never required. However, if you prefer to supply your own ID instead, you can
      specify it here in UUID format. If you specify your own ID, it must be globally unique.</p> |
| `asset_model_composite_model_type` | String | ✅ | <p>The composite model type. Valid values are <code>AWS/ALARM</code>, <code>CUSTOM</code>, or <code> AWS/L4E_ANOMALY</code>.</p> |
| `client_token` | String |  | <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p> |
| `asset_model_composite_model_external_id` | String |  | <p>An external ID to assign to the composite model.</p>
         <p>If the composite model is a derived composite model, or one nested inside a component
      model, you can only set the external ID using <code>UpdateAssetModelCompositeModel</code> and
      specifying the derived ID of the model or property from the created model it's a part
      of.</p> |
| `asset_model_composite_model_description` | String |  | <p>A description for the composite model.</p> |
| `asset_model_composite_model_properties` | Vec<String> |  | <p>The property definitions of the composite model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/custom-composite-models.html#inline-composite-models">
        Inline custom composite models</a> in the <i>IoT SiteWise User Guide</i>.</p>
         <p>You can specify up to 200 properties per composite model. For more
      information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/quotas.html">Quotas</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `match_for_version_type` | String |  | <p>Specifies the asset model version type (<code>LATEST</code> or <code>ACTIVE</code>) used in 
  conjunction with <code>If-Match</code> or <code>If-None-Match</code> headers to determine the target ETag for the create operation.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `asset_model_composite_model_description` | String | <p>The description for the composite model.</p> |
| `asset_model_id` | String | <p>The ID of the asset model, in UUID format.</p> |
| `asset_model_composite_model_id` | String | <p>The ID of a composite model on this asset model.</p> |
| `asset_model_composite_model_external_id` | String | <p>The external ID of a composite model on this asset model.</p> |
| `asset_model_composite_model_properties` | Vec<String> | <p>The property definitions of the composite model.</p> |
| `asset_model_composite_model_summaries` | Vec<String> | <p>The list of composite model summaries for the composite model.</p> |
| `action_definitions` | Vec<String> | <p>The available actions for a composite model on this asset model.</p> |
| `asset_model_composite_model_name` | String | <p>The unique, friendly name for the composite model.</p> |
| `asset_model_composite_model_type` | String | <p>The composite model type. Valid values are <code>AWS/ALARM</code>, <code>CUSTOM</code>, or
        <code> AWS/L4E_ANOMALY</code>.</p> |
| `asset_model_composite_model_path` | Vec<String> | <p>The path to the composite model listing the parent composite models.</p> |
| `composition_details` | String | <p>Metadata for the composition relationship established by using
        <code>composedAssetModelId</code> in <a href="https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/API_CreateAssetModelCompositeModel.html">
               <code>CreateAssetModelCompositeModel</code>
            </a>. For instance, an array detailing the
      path of the composition relationship for this composite model.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create asset_model_composite_model
asset_model_composite_model = provider.iotsitewise.Asset_model_composite_model {
    asset_model_id = "value"  # <p>The ID of the asset model this composite model is a part of.</p>
    asset_model_composite_model_name = "value"  # <p>A unique name for the composite model.</p>
    asset_model_composite_model_type = "value"  # <p>The composite model type. Valid values are <code>AWS/ALARM</code>, <code>CUSTOM</code>, or <code> AWS/L4E_ANOMALY</code>.</p>
}

# Access asset_model_composite_model outputs
asset_model_composite_model_id = asset_model_composite_model.id
asset_model_composite_model_asset_model_composite_model_description = asset_model_composite_model.asset_model_composite_model_description
asset_model_composite_model_asset_model_id = asset_model_composite_model.asset_model_id
asset_model_composite_model_asset_model_composite_model_id = asset_model_composite_model.asset_model_composite_model_id
asset_model_composite_model_asset_model_composite_model_external_id = asset_model_composite_model.asset_model_composite_model_external_id
asset_model_composite_model_asset_model_composite_model_properties = asset_model_composite_model.asset_model_composite_model_properties
asset_model_composite_model_asset_model_composite_model_summaries = asset_model_composite_model.asset_model_composite_model_summaries
asset_model_composite_model_action_definitions = asset_model_composite_model.action_definitions
asset_model_composite_model_asset_model_composite_model_name = asset_model_composite_model.asset_model_composite_model_name
asset_model_composite_model_asset_model_composite_model_type = asset_model_composite_model.asset_model_composite_model_type
asset_model_composite_model_asset_model_composite_model_path = asset_model_composite_model.asset_model_composite_model_path
asset_model_composite_model_composition_details = asset_model_composite_model.composition_details
```

---


### Logging_options

LoggingOptions resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `logging_options` | String | ✅ | <p>The logging options to set.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `logging_options` | String | <p>The current logging options.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create logging_options
logging_options = provider.iotsitewise.Logging_options {
    logging_options = "value"  # <p>The logging options to set.</p>
}

# Access logging_options outputs
logging_options_id = logging_options.id
logging_options_logging_options = logging_options.logging_options
```

---


### Bulk_import_job

BulkImportJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `job_role_arn` | String | ✅ | <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the IAM role that allows IoT SiteWise to read Amazon S3 data.</p> |
| `job_name` | String | ✅ | <p>The unique name that helps identify the job request.</p> |
| `job_configuration` | String | ✅ | <p>Contains the configuration information of a job, such as the file format used to save data in Amazon S3.</p> |
| `delete_files_after_import` | bool |  | <p>If set to true, your data files is deleted from S3, after ingestion into IoT SiteWise storage.</p> |
| `files` | Vec<String> | ✅ | <p>The files in the specified Amazon S3 bucket that contain your data.</p> |
| `adaptive_ingestion` | bool |  | <p>If set to true, ingest new data into IoT SiteWise storage. Measurements with notifications, metrics and transforms are 
   computed. If set to false, historical data is ingested into IoT SiteWise as is.</p> |
| `error_report_location` | String | ✅ | <p>The Amazon S3 destination where errors associated with the job creation request are saved.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_creation_date` | String | <p>The date the job was created, in Unix epoch TIME.</p> |
| `delete_files_after_import` | bool | <p>If set to true, your data files is deleted from S3, after ingestion into IoT SiteWise storage.</p> |
| `adaptive_ingestion` | bool | <p>If set to true, ingest new data into IoT SiteWise storage. Measurements with notifications, metrics and transforms are 
   computed. If set to false, historical data is ingested into IoT SiteWise as is.</p> |
| `job_status` | String | <p>The status of the bulk import job can be one of following values:</p>
         <ul>
            <li>
               <p>
                  <code>PENDING</code> – IoT SiteWise is waiting for the current bulk import job to finish.</p>
            </li>
            <li>
               <p>
                  <code>CANCELLED</code> – The bulk import job has been canceled.</p>
            </li>
            <li>
               <p>
                  <code>RUNNING</code> – IoT SiteWise is processing your request to import your data from Amazon S3.</p>
            </li>
            <li>
               <p>
                  <code>COMPLETED</code> – IoT SiteWise successfully completed your request to import data from Amazon S3.</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code> – IoT SiteWise couldn't process your request to import data from Amazon S3.
        You can use logs saved in the specified error report location in Amazon S3 to troubleshoot issues.</p>
            </li>
            <li>
               <p>
                  <code>COMPLETED_WITH_FAILURES</code> – IoT SiteWise completed your request to import data from Amazon S3 with errors.
        You can use logs saved in the specified error report location in Amazon S3 to troubleshoot issues.</p>
            </li>
         </ul> |
| `job_role_arn` | String | <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the IAM role that allows IoT SiteWise to read Amazon S3 data.</p> |
| `job_id` | String | <p>The ID of the job.</p> |
| `job_last_update_date` | String | <p>The date the job was last updated, in Unix epoch time.</p> |
| `job_configuration` | String | <p>Contains the configuration information of a job, such as the file format used to save data in Amazon S3.</p> |
| `job_name` | String | <p>The unique name that helps identify the job request.</p> |
| `files` | Vec<String> | <p>The files in the specified Amazon S3 bucket that contain your data.</p> |
| `error_report_location` | String | <p>The Amazon S3 destination where errors associated with the job creation request are saved.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bulk_import_job
bulk_import_job = provider.iotsitewise.Bulk_import_job {
    job_role_arn = "value"  # <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the IAM role that allows IoT SiteWise to read Amazon S3 data.</p>
    job_name = "value"  # <p>The unique name that helps identify the job request.</p>
    job_configuration = "value"  # <p>Contains the configuration information of a job, such as the file format used to save data in Amazon S3.</p>
    files = "value"  # <p>The files in the specified Amazon S3 bucket that contain your data.</p>
    error_report_location = "value"  # <p>The Amazon S3 destination where errors associated with the job creation request are saved.</p>
}

# Access bulk_import_job outputs
bulk_import_job_id = bulk_import_job.id
bulk_import_job_job_creation_date = bulk_import_job.job_creation_date
bulk_import_job_delete_files_after_import = bulk_import_job.delete_files_after_import
bulk_import_job_adaptive_ingestion = bulk_import_job.adaptive_ingestion
bulk_import_job_job_status = bulk_import_job.job_status
bulk_import_job_job_role_arn = bulk_import_job.job_role_arn
bulk_import_job_job_id = bulk_import_job.job_id
bulk_import_job_job_last_update_date = bulk_import_job.job_last_update_date
bulk_import_job_job_configuration = bulk_import_job.job_configuration
bulk_import_job_job_name = bulk_import_job.job_name
bulk_import_job_files = bulk_import_job.files
bulk_import_job_error_report_location = bulk_import_job.error_report_location
```

---


### Computation_model

ComputationModel resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `computation_model_description` | String |  | <p>The description of the computation model.</p> |
| `computation_model_configuration` | String | ✅ | <p>The configuration for the computation model.</p> |
| `client_token` | String |  | <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs that contain metadata for the asset. For more information, see
        <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/tag-resources.html">Tagging your IoT SiteWise
        resources</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `computation_model_name` | String | ✅ | <p>The name of the computation model.</p> |
| `computation_model_data_binding` | HashMap<String, String> | ✅ | <p>The data binding for the computation model. Key is a variable name defined in configuration. 
  Value is a <code>ComputationModelDataBindingValue</code> referenced by the variable.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `action_definitions` | Vec<String> | <p>The available actions for this computation model.</p> |
| `computation_model_configuration` | String | <p>The configuration for the computation model.</p> |
| `computation_model_name` | String | <p>The name of the computation model.</p> |
| `computation_model_id` | String | <p>The ID of the computation model.</p> |
| `computation_model_description` | String | <p>The description of the computation model.</p> |
| `computation_model_data_binding` | HashMap<String, String> | <p>The data binding for the computation model. Key is a variable name defined in configuration. 
  Value is a <code>ComputationModelDataBindingValue</code> referenced by the variable.</p> |
| `computation_model_arn` | String | <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the computation model, which has the following format.</p>
         <p>
            <code>arn:${Partition}:iotsitewise:${Region}:${Account}:computation-model/${ComputationModelId}</code>
         </p> |
| `computation_model_creation_date` | String | <p>The model creation date, in Unix epoch time.</p> |
| `computation_model_last_update_date` | String | <p>The date the model was last updated, in Unix epoch time.</p> |
| `computation_model_status` | String | <p>The current status of the asset model, which contains a state and an error message if
      any.</p> |
| `computation_model_version` | String | <p>The version of the computation model.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create computation_model
computation_model = provider.iotsitewise.Computation_model {
    computation_model_configuration = "value"  # <p>The configuration for the computation model.</p>
    computation_model_name = "value"  # <p>The name of the computation model.</p>
    computation_model_data_binding = "value"  # <p>The data binding for the computation model. Key is a variable name defined in configuration. 
  Value is a <code>ComputationModelDataBindingValue</code> referenced by the variable.</p>
}

# Access computation_model outputs
computation_model_id = computation_model.id
computation_model_action_definitions = computation_model.action_definitions
computation_model_computation_model_configuration = computation_model.computation_model_configuration
computation_model_computation_model_name = computation_model.computation_model_name
computation_model_computation_model_id = computation_model.computation_model_id
computation_model_computation_model_description = computation_model.computation_model_description
computation_model_computation_model_data_binding = computation_model.computation_model_data_binding
computation_model_computation_model_arn = computation_model.computation_model_arn
computation_model_computation_model_creation_date = computation_model.computation_model_creation_date
computation_model_computation_model_last_update_date = computation_model.computation_model_last_update_date
computation_model_computation_model_status = computation_model.computation_model_status
computation_model_computation_model_version = computation_model.computation_model_version
```

---


### Default_encryption_configuration

DefaultEncryptionConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kms_key_id` | String |  | <p>The Key ID of the customer managed key used for KMS encryption. This is required if you
      use <code>KMS_BASED_ENCRYPTION</code>.</p> |
| `encryption_type` | String | ✅ | <p>The type of encryption used for the encryption configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kms_key_arn` | String | <p>The key ARN of the customer managed key used for KMS encryption if you use
        <code>KMS_BASED_ENCRYPTION</code>.</p> |
| `encryption_type` | String | <p>The type of encryption used for the encryption configuration.</p> |
| `configuration_status` | String | <p>The status of the account configuration. This contains the
      <code>ConfigurationState</code>. If there's an error, it also contains the
        <code>ErrorDetails</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create default_encryption_configuration
default_encryption_configuration = provider.iotsitewise.Default_encryption_configuration {
    encryption_type = "value"  # <p>The type of encryption used for the encryption configuration.</p>
}

# Access default_encryption_configuration outputs
default_encryption_configuration_id = default_encryption_configuration.id
default_encryption_configuration_kms_key_arn = default_encryption_configuration.kms_key_arn
default_encryption_configuration_encryption_type = default_encryption_configuration.encryption_type
default_encryption_configuration_configuration_status = default_encryption_configuration.configuration_status
```

---


### Dataset

Dataset resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dataset_id` | String |  | <p>The ID of the dataset.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs that contain metadata for the access policy. For more
      information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/tag-resources.html">Tagging your
        IoT SiteWise resources</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `dataset_description` | String |  | <p>A description about the dataset, and its functionality.</p> |
| `client_token` | String |  | <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p> |
| `dataset_name` | String | ✅ | <p>The name of the dataset.</p> |
| `dataset_source` | String | ✅ | <p>The data source for the dataset.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dataset_name` | String | <p>The name of the dataset.</p> |
| `dataset_status` | String | <p>The status of the dataset. This contains the state and any error messages. 
  State is <code>CREATING</code> after a successfull call to this API, and any associated error message. The state is 
  <code>ACTIVE</code> when ready to use.</p> |
| `dataset_last_update_date` | String | <p>The date the dataset was last updated, in Unix epoch time.</p> |
| `dataset_version` | String | <p>The version of the dataset.</p> |
| `dataset_description` | String | <p>A description about the dataset, and its functionality.</p> |
| `dataset_creation_date` | String | <p>The dataset creation date, in Unix epoch time.</p> |
| `dataset_id` | String | <p>The ID of the dataset.</p> |
| `dataset_arn` | String | <p>The <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference-arns.html">ARN</a> of the dataset. 
  The format is <code>arn:${Partition}:iotsitewise:${Region}:${Account}:dataset/${DatasetId}</code>.</p> |
| `dataset_source` | String | <p>The data source for the dataset.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dataset
dataset = provider.iotsitewise.Dataset {
    dataset_name = "value"  # <p>The name of the dataset.</p>
    dataset_source = "value"  # <p>The data source for the dataset.</p>
}

# Access dataset outputs
dataset_id = dataset.id
dataset_dataset_name = dataset.dataset_name
dataset_dataset_status = dataset.dataset_status
dataset_dataset_last_update_date = dataset.dataset_last_update_date
dataset_dataset_version = dataset.dataset_version
dataset_dataset_description = dataset.dataset_description
dataset_dataset_creation_date = dataset.dataset_creation_date
dataset_dataset_id = dataset.dataset_id
dataset_dataset_arn = dataset.dataset_arn
dataset_dataset_source = dataset.dataset_source
```

---


### Asset

Asset resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `asset_id` | String |  | <p>The ID to assign to the asset, if desired. IoT SiteWise automatically generates a unique ID for you, so this parameter is never required.
    However, if you prefer to supply your own ID instead, you can specify it here in UUID format.
    If you specify your own ID, it must be globally unique.</p> |
| `asset_description` | String |  | <p>A description for the asset.</p> |
| `asset_name` | String | ✅ | <p>A friendly name for the asset.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs that contain metadata for the asset. For more information, see
        <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/tag-resources.html">Tagging your IoT SiteWise
        resources</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `asset_model_id` | String | ✅ | <p>The ID of the asset model from which to create the asset. This can be either the actual ID in UUID format, or else <code>externalId:</code> followed by the external ID, if it has one.
    For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-id-references">Referencing objects with external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `asset_external_id` | String |  | <p>An external ID to assign to the asset. The external ID must be unique within your Amazon Web Services account. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-ids">Using external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `client_token` | String |  | <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `asset_external_id` | String | <p>The external ID of the asset, if any.</p> |
| `asset_composite_models` | Vec<String> | <p>The composite models for the asset.</p> |
| `asset_status` | String | <p>The current status of the asset, which contains a state and any error message.</p> |
| `asset_id` | String | <p>The ID of the asset, in UUID format.</p> |
| `asset_model_id` | String | <p>The ID of the asset model that was used to create the asset.</p> |
| `asset_creation_date` | String | <p>The date the asset was created, in Unix epoch time.</p> |
| `asset_description` | String | <p>A description for the asset.</p> |
| `asset_last_update_date` | String | <p>The date the asset was last updated, in Unix epoch time.</p> |
| `asset_composite_model_summaries` | Vec<String> | <p>The list of the immediate child custom composite model summaries for the asset.</p> |
| `asset_arn` | String | <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the asset, which has the following format.</p>
         <p>
            <code>arn:${Partition}:iotsitewise:${Region}:${Account}:asset/${AssetId}</code>
         </p> |
| `asset_name` | String | <p>The name of the asset.</p> |
| `asset_properties` | Vec<String> | <p>The list of asset properties for the asset.</p>
         <p>This object doesn't include properties that you define in composite models. You can find
      composite model properties in the <code>assetCompositeModels</code> object.</p> |
| `asset_hierarchies` | Vec<String> | <p>A list of asset hierarchies that each contain a <code>hierarchyId</code>. A hierarchy specifies allowed parent/child asset relationships.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create asset
asset = provider.iotsitewise.Asset {
    asset_name = "value"  # <p>A friendly name for the asset.</p>
    asset_model_id = "value"  # <p>The ID of the asset model from which to create the asset. This can be either the actual ID in UUID format, or else <code>externalId:</code> followed by the external ID, if it has one.
    For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-id-references">Referencing objects with external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p>
}

# Access asset outputs
asset_id = asset.id
asset_asset_external_id = asset.asset_external_id
asset_asset_composite_models = asset.asset_composite_models
asset_asset_status = asset.asset_status
asset_asset_id = asset.asset_id
asset_asset_model_id = asset.asset_model_id
asset_asset_creation_date = asset.asset_creation_date
asset_asset_description = asset.asset_description
asset_asset_last_update_date = asset.asset_last_update_date
asset_asset_composite_model_summaries = asset.asset_composite_model_summaries
asset_asset_arn = asset.asset_arn
asset_asset_name = asset.asset_name
asset_asset_properties = asset.asset_properties
asset_asset_hierarchies = asset.asset_hierarchies
```

---


### Asset_model_interface_relationship

AssetModelInterfaceRelationship resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `interface_asset_model_id` | String | ✅ | <p>The ID of the interface asset model. This can be either the actual ID in UUID format, or
      else externalId: followed by the external ID.</p> |
| `client_token` | String |  | <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the
      request. Don't reuse this client token if a new idempotent request is required.</p> |
| `asset_model_id` | String | ✅ | <p>The ID of the asset model. This can be either the actual ID in UUID format, or else
      externalId: followed by the external ID.</p> |
| `property_mapping_configuration` | String | ✅ | <p>The configuration for mapping properties from the interface asset model to the asset model
      where the interface is applied. This configuration controls how properties are matched and
      created during the interface application process.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hierarchy_mappings` | Vec<String> | <p>A list of hierarchy mappings between the interface asset model and the asset model where
      the interface is applied.</p> |
| `asset_model_id` | String | <p>The ID of the asset model.</p> |
| `property_mappings` | Vec<String> | <p>A list of property mappings between the interface asset model and the asset model where
      the interface is applied.</p> |
| `interface_asset_model_id` | String | <p>The ID of the interface asset model.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create asset_model_interface_relationship
asset_model_interface_relationship = provider.iotsitewise.Asset_model_interface_relationship {
    interface_asset_model_id = "value"  # <p>The ID of the interface asset model. This can be either the actual ID in UUID format, or
      else externalId: followed by the external ID.</p>
    asset_model_id = "value"  # <p>The ID of the asset model. This can be either the actual ID in UUID format, or else
      externalId: followed by the external ID.</p>
    property_mapping_configuration = "value"  # <p>The configuration for mapping properties from the interface asset model to the asset model
      where the interface is applied. This configuration controls how properties are matched and
      created during the interface application process.</p>
}

# Access asset_model_interface_relationship outputs
asset_model_interface_relationship_id = asset_model_interface_relationship.id
asset_model_interface_relationship_hierarchy_mappings = asset_model_interface_relationship.hierarchy_mappings
asset_model_interface_relationship_asset_model_id = asset_model_interface_relationship.asset_model_id
asset_model_interface_relationship_property_mappings = asset_model_interface_relationship.property_mappings
asset_model_interface_relationship_interface_asset_model_id = asset_model_interface_relationship.interface_asset_model_id
```

---


### Asset_composite_model

AssetCompositeModel resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `asset_composite_model_type` | String | <p>The composite model type. Valid values are <code>AWS/ALARM</code>, <code>CUSTOM</code>, or
        <code> AWS/L4E_ANOMALY</code>.</p> |
| `asset_composite_model_properties` | Vec<String> | <p>The property definitions of the composite model that was used to create the asset.</p> |
| `asset_composite_model_summaries` | Vec<String> | <p>The list of composite model summaries.</p> |
| `asset_composite_model_external_id` | String | <p>An external ID to assign to the asset model.</p>
         <p>If the composite model is a component-based composite model, or one nested inside a
      component model, you can only set the external ID using
        <code>UpdateAssetModelCompositeModel</code> and specifying the derived ID of the model or
      property from the created model it's a part of.</p> |
| `asset_composite_model_description` | String | <p>A description for the composite model.</p> |
| `asset_composite_model_id` | String | <p>The ID of a composite model on this asset.</p> |
| `action_definitions` | Vec<String> | <p>The available actions for a composite model on this asset.</p> |
| `asset_composite_model_path` | Vec<String> | <p>The path to the composite model listing the parent composite models.</p> |
| `asset_id` | String | <p>The ID of the asset, in UUID format. This ID uniquely identifies the asset within IoT SiteWise and can be used with other
      IoT SiteWise APIs.</p> |
| `asset_composite_model_name` | String | <p>The unique, friendly name for the composite model.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access asset_composite_model outputs
asset_composite_model_id = asset_composite_model.id
asset_composite_model_asset_composite_model_type = asset_composite_model.asset_composite_model_type
asset_composite_model_asset_composite_model_properties = asset_composite_model.asset_composite_model_properties
asset_composite_model_asset_composite_model_summaries = asset_composite_model.asset_composite_model_summaries
asset_composite_model_asset_composite_model_external_id = asset_composite_model.asset_composite_model_external_id
asset_composite_model_asset_composite_model_description = asset_composite_model.asset_composite_model_description
asset_composite_model_asset_composite_model_id = asset_composite_model.asset_composite_model_id
asset_composite_model_action_definitions = asset_composite_model.action_definitions
asset_composite_model_asset_composite_model_path = asset_composite_model.asset_composite_model_path
asset_composite_model_asset_id = asset_composite_model.asset_id
asset_composite_model_asset_composite_model_name = asset_composite_model.asset_composite_model_name
```

---


### Interpolated_asset_property_values

InterpolatedAssetPropertyValues resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token for the next set of results, or null if there are no additional results.</p> |
| `interpolated_asset_property_values` | Vec<String> | <p>The requested interpolated values.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access interpolated_asset_property_values outputs
interpolated_asset_property_values_id = interpolated_asset_property_values.id
interpolated_asset_property_values_next_token = interpolated_asset_property_values.next_token
interpolated_asset_property_values_interpolated_asset_property_values = interpolated_asset_property_values.interpolated_asset_property_values
```

---


### Gateway_capability_configuration

GatewayCapabilityConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `capability_configuration` | String | ✅ | <p>The JSON document that defines the configuration for the gateway capability. For more
      information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/configure-sources.html#configure-source-cli">Configuring data sources (CLI)</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `gateway_id` | String | ✅ | <p>The ID of the gateway to be updated.</p> |
| `capability_namespace` | String | ✅ | <p>The namespace of the gateway capability configuration to be updated.
      For example, if you configure OPC UA
      sources for an MQTT-enabled gateway, your OPC-UA capability configuration has the namespace
        <code>iotsitewise:opcuacollector:3</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `capability_configuration` | String | <p>The JSON document that defines the gateway capability's configuration. For more
      information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/configure-sources.html#configure-source-cli">Configuring data sources (CLI)</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `gateway_id` | String | <p>The ID of the gateway that defines the capability configuration.</p> |
| `capability_sync_status` | String | <p>The synchronization status of the gateway capability configuration. The sync status can be one of the following:</p>
         <ul>
            <li>
               <p>
                  <code>IN_SYNC</code> - The gateway is running with the latest configuration.</p>
            </li>
            <li>
               <p>
                  <code>OUT_OF_SYNC</code> - The gateway hasn't received the latest configuration.</p>
            </li>
            <li>
               <p>
                  <code>SYNC_FAILED</code> - The gateway rejected the latest configuration.</p>
            </li>
            <li>
               <p>
                  <code>UNKNOWN</code> - The gateway hasn't reported its sync status.</p>
            </li>
            <li>
               <p>
                  <code>NOT_APPLICABLE</code> - The gateway doesn't support this capability. This is most common when integrating partner data sources, because the data integration is handled externally by the partner.</p>
            </li>
         </ul> |
| `capability_namespace` | String | <p>The namespace of the gateway capability.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access gateway_capability_configuration outputs
gateway_capability_configuration_id = gateway_capability_configuration.id
gateway_capability_configuration_capability_configuration = gateway_capability_configuration.capability_configuration
gateway_capability_configuration_gateway_id = gateway_capability_configuration.gateway_id
gateway_capability_configuration_capability_sync_status = gateway_capability_configuration.capability_sync_status
gateway_capability_configuration_capability_namespace = gateway_capability_configuration.capability_namespace
```

---


### Storage_configuration

StorageConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `warm_tier_retention_period` | String |  | <p>Set this period to specify how long your data is stored in the warm tier before it is deleted. You can set this only if cold tier is enabled.</p> |
| `multi_layer_storage` | String |  | <p>Identifies a storage destination. If you specified <code>MULTI_LAYER_STORAGE</code> for the storage type,
      you must specify a <code>MultiLayerStorage</code> object.</p> |
| `disallow_ingest_null_na_n` | bool |  | <p>Describes the configuration for ingesting NULL and NaN data. By default the feature is
      allowed. The feature is disallowed if the value is <code>true</code>.</p> |
| `disassociated_data_storage` | String |  | <p>Contains the storage configuration for time series (data streams) that aren't associated with asset properties.
      The <code>disassociatedDataStorage</code> can be one of the following values:</p>
         <ul>
            <li>
               <p>
                  <code>ENABLED</code> – IoT SiteWise accepts time series that aren't associated with asset properties.</p>
               <important>
                  <p>After the <code>disassociatedDataStorage</code> is enabled, you can't disable it.</p>
               </important>
            </li>
            <li>
               <p>
                  <code>DISABLED</code> – IoT SiteWise doesn't accept time series (data streams) that aren't associated with asset properties.</p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/data-streams.html">Data streams</a>
    in the <i>IoT SiteWise User Guide</i>.</p> |
| `warm_tier` | String |  | <p>A service managed storage tier optimized for analytical queries. It stores periodically uploaded, buffered and historical data ingested with the CreaeBulkImportJob API.</p> |
| `storage_type` | String | ✅ | <p>The storage tier that you specified for your data.
      The <code>storageType</code> parameter can be one of the following values:</p>
         <ul>
            <li>
               <p>
                  <code>SITEWISE_DEFAULT_STORAGE</code> – IoT SiteWise saves your data into the hot tier.
          The hot tier is a service-managed database.</p>
            </li>
            <li>
               <p>
                  <code>MULTI_LAYER_STORAGE</code> – IoT SiteWise saves your data in both the cold tier and the hot tier.
          The cold tier is a customer-managed Amazon S3 bucket.</p>
            </li>
         </ul> |
| `retention_period` | String |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `disassociated_data_storage` | String | <p>Contains the storage configuration for time series (data streams) that aren't associated with asset properties.
      The <code>disassociatedDataStorage</code> can be one of the following values:</p>
         <ul>
            <li>
               <p>
                  <code>ENABLED</code> – IoT SiteWise accepts time series that aren't associated with asset properties.</p>
               <important>
                  <p>After the <code>disassociatedDataStorage</code> is enabled, you can't disable it.</p>
               </important>
            </li>
            <li>
               <p>
                  <code>DISABLED</code> – IoT SiteWise doesn't accept time series (data streams) that aren't associated with asset properties.</p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/data-streams.html">Data streams</a>
    in the <i>IoT SiteWise User Guide</i>.</p> |
| `disallow_ingest_null_na_n` | bool | <p>Describes the configuration for ingesting NULL and NaN data. By default the feature is
      allowed. The feature is disallowed if the value is <code>true</code>.</p> |
| `multi_layer_storage` | String | <p>Contains information about the storage destination.</p> |
| `retention_period` | String | <p>The number of days your data is kept in the hot tier. By default, your data is kept indefinitely in the hot tier.</p> |
| `warm_tier_retention_period` | String | <p>Set this period to specify how long your data is stored in the warm tier before it is deleted. You can set this only if cold tier is enabled.</p> |
| `configuration_status` | String |  |
| `last_update_date` | String | <p>The date the storage configuration was last updated, in Unix epoch time.</p> |
| `warm_tier` | String | <p>A service managed storage tier optimized for analytical queries. It stores periodically uploaded, buffered and historical data ingested with the CreaeBulkImportJob API.</p> |
| `storage_type` | String | <p>The storage tier that you specified for your data.
      The <code>storageType</code> parameter can be one of the following values:</p>
         <ul>
            <li>
               <p>
                  <code>SITEWISE_DEFAULT_STORAGE</code> – IoT SiteWise saves your data into the hot tier.
          The hot tier is a service-managed database.</p>
            </li>
            <li>
               <p>
                  <code>MULTI_LAYER_STORAGE</code> – IoT SiteWise saves your data in both the cold tier and the hot tier.
          The cold tier is a customer-managed Amazon S3 bucket.</p>
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

# Create storage_configuration
storage_configuration = provider.iotsitewise.Storage_configuration {
    storage_type = "value"  # <p>The storage tier that you specified for your data.
      The <code>storageType</code> parameter can be one of the following values:</p>
         <ul>
            <li>
               <p>
                  <code>SITEWISE_DEFAULT_STORAGE</code> – IoT SiteWise saves your data into the hot tier.
          The hot tier is a service-managed database.</p>
            </li>
            <li>
               <p>
                  <code>MULTI_LAYER_STORAGE</code> – IoT SiteWise saves your data in both the cold tier and the hot tier.
          The cold tier is a customer-managed Amazon S3 bucket.</p>
            </li>
         </ul>
}

# Access storage_configuration outputs
storage_configuration_id = storage_configuration.id
storage_configuration_disassociated_data_storage = storage_configuration.disassociated_data_storage
storage_configuration_disallow_ingest_null_na_n = storage_configuration.disallow_ingest_null_na_n
storage_configuration_multi_layer_storage = storage_configuration.multi_layer_storage
storage_configuration_retention_period = storage_configuration.retention_period
storage_configuration_warm_tier_retention_period = storage_configuration.warm_tier_retention_period
storage_configuration_configuration_status = storage_configuration.configuration_status
storage_configuration_last_update_date = storage_configuration.last_update_date
storage_configuration_warm_tier = storage_configuration.warm_tier
storage_configuration_storage_type = storage_configuration.storage_type
```

---


### Asset_property_value_history

AssetPropertyValueHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token for the next set of results, or null if there are no additional results.</p> |
| `asset_property_value_history` | Vec<String> | <p>The asset property's value history.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access asset_property_value_history outputs
asset_property_value_history_id = asset_property_value_history.id
asset_property_value_history_next_token = asset_property_value_history.next_token
asset_property_value_history_asset_property_value_history = asset_property_value_history.asset_property_value_history
```

---


### Portal

Portal resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `portal_logo_image_file` | String |  | <p>A logo image to display in the portal. Upload a square, high-resolution image. The
      image is displayed on a dark background.</p> |
| `portal_contact_email` | String | ✅ | <p>The Amazon Web Services administrator's contact email address.</p> |
| `notification_sender_email` | String |  | <p>The email address that sends alarm notifications.</p>
         <important>
            <p>If you use the <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/lambda-support.html">IoT Events managed Lambda
          function</a> to manage your emails, you must <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-email-addresses.html">verify the sender email
          address in Amazon SES</a>.</p>
         </important> |
| `portal_type` | String |  | <p>Define the type of portal. The value for IoT SiteWise Monitor (Classic) is <code>SITEWISE_PORTAL_V1</code>. The value for IoT SiteWise Monitor (AI-aware) is <code>SITEWISE_PORTAL_V2</code>.</p> |
| `portal_type_configuration` | HashMap<String, String> |  | <p>The configuration entry associated with the specific portal type. The value for IoT SiteWise Monitor (Classic) is <code>SITEWISE_PORTAL_V1</code>. The value for IoT SiteWise Monitor (AI-aware) is <code>SITEWISE_PORTAL_V2</code>.</p> |
| `portal_name` | String | ✅ | <p>A friendly name for the portal.</p> |
| `portal_description` | String |  | <p>A description for the portal.</p> |
| `client_token` | String |  | <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p> |
| `alarms` | String |  | <p>Contains the configuration information of an alarm created in an IoT SiteWise Monitor portal.
  You can use the alarm to monitor an asset property and get notified when the asset property value is outside a specified range.
  For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/appguide/monitor-alarms.html">Monitoring with alarms</a> in the <i>IoT SiteWise Application Guide</i>.</p> |
| `role_arn` | String | ✅ | <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of a service role that allows the portal's users to access your IoT SiteWise
      resources on your behalf. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/monitor-service-role.html">Using service roles for IoT SiteWise Monitor</a> in the
        <i>IoT SiteWise User Guide</i>.</p> |
| `portal_auth_mode` | String |  | <p>The service to use to authenticate users to the portal. Choose from the following
      options:</p>
         <ul>
            <li>
               <p>
                  <code>SSO</code> – The portal uses IAM Identity Center to authenticate users and manage
          user permissions. Before you can create a portal that uses IAM Identity Center, you must enable IAM Identity Center.
          For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/monitor-get-started.html#mon-gs-sso">Enabling IAM Identity Center</a> in the
            <i>IoT SiteWise User Guide</i>. This option is only available in Amazon Web Services Regions other than
          the China Regions.</p>
            </li>
            <li>
               <p>
                  <code>IAM</code> – The portal uses Identity and Access Management to authenticate users and manage
          user permissions.</p>
            </li>
         </ul>
         <p>You can't change this value after you create a portal.</p>
         <p>Default: <code>SSO</code>
         </p> |
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs that contain metadata for the portal. For more information, see
        <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/tag-resources.html">Tagging your IoT SiteWise
        resources</a> in the <i>IoT SiteWise User Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `portal_type` | String | <p>Define the type of portal. The value for IoT SiteWise Monitor (Classic) is <code>SITEWISE_PORTAL_V1</code>. The value for IoT SiteWise Monitor (AI-aware) is <code>SITEWISE_PORTAL_V2</code>.</p> |
| `portal_auth_mode` | String | <p>The service to use to authenticate users to the portal.</p> |
| `notification_sender_email` | String | <p>The email address that sends alarm notifications.</p> |
| `portal_start_url` | String | <p>The URL for the IoT SiteWise Monitor portal. You can use this URL to access portals that
      use IAM Identity Center for authentication. For portals that use IAM for authentication, you must use the
      IoT SiteWise console to get a URL that you can use to access the portal.</p> |
| `portal_creation_date` | String | <p>The date the portal was created, in Unix epoch time.</p> |
| `portal_last_update_date` | String | <p>The date the portal was last updated, in Unix epoch time.</p> |
| `alarms` | String | <p>Contains the configuration information of an alarm created in an IoT SiteWise Monitor portal.</p> |
| `portal_id` | String | <p>The ID of the portal.</p> |
| `role_arn` | String | <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the service role that allows the portal's users to access your IoT SiteWise
      resources on your behalf. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/monitor-service-role.html">Using service roles for IoT SiteWise Monitor</a> in the
        <i>IoT SiteWise User Guide</i>.</p> |
| `portal_description` | String | <p>The portal's description.</p> |
| `portal_contact_email` | String | <p>The Amazon Web Services administrator's contact email address.</p> |
| `portal_type_configuration` | HashMap<String, String> | <p>The configuration entry associated with the specific portal type. The value for IoT SiteWise Monitor (Classic) is <code>SITEWISE_PORTAL_V1</code>. The value for IoT SiteWise Monitor (AI-aware) is <code>SITEWISE_PORTAL_V2</code>.</p> |
| `portal_arn` | String | <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the portal, which has the following format.</p>
         <p>
            <code>arn:${Partition}:iotsitewise:${Region}:${Account}:portal/${PortalId}</code>
         </p> |
| `portal_client_id` | String | <p>The IAM Identity Center application generated client ID (used with IAM Identity Center API operations). IoT SiteWise includes
        <code>portalClientId</code> for only portals that use IAM Identity Center to authenticate users.</p> |
| `portal_logo_image_location` | String | <p>The portal's logo image, which is available at a URL.</p> |
| `portal_name` | String | <p>The name of the portal.</p> |
| `portal_status` | String | <p>The current status of the portal, which contains a state and any error message.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create portal
portal = provider.iotsitewise.Portal {
    portal_contact_email = "value"  # <p>The Amazon Web Services administrator's contact email address.</p>
    portal_name = "value"  # <p>A friendly name for the portal.</p>
    role_arn = "value"  # <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of a service role that allows the portal's users to access your IoT SiteWise
      resources on your behalf. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/monitor-service-role.html">Using service roles for IoT SiteWise Monitor</a> in the
        <i>IoT SiteWise User Guide</i>.</p>
}

# Access portal outputs
portal_id = portal.id
portal_portal_type = portal.portal_type
portal_portal_auth_mode = portal.portal_auth_mode
portal_notification_sender_email = portal.notification_sender_email
portal_portal_start_url = portal.portal_start_url
portal_portal_creation_date = portal.portal_creation_date
portal_portal_last_update_date = portal.portal_last_update_date
portal_alarms = portal.alarms
portal_portal_id = portal.portal_id
portal_role_arn = portal.role_arn
portal_portal_description = portal.portal_description
portal_portal_contact_email = portal.portal_contact_email
portal_portal_type_configuration = portal.portal_type_configuration
portal_portal_arn = portal.portal_arn
portal_portal_client_id = portal.portal_client_id
portal_portal_logo_image_location = portal.portal_logo_image_location
portal_portal_name = portal.portal_name
portal_portal_status = portal.portal_status
```

---


### Asset_property_aggregates

AssetPropertyAggregates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `aggregated_values` | Vec<String> | <p>The requested aggregated values.</p> |
| `next_token` | String | <p>The token for the next set of results, or null if there are no additional results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access asset_property_aggregates outputs
asset_property_aggregates_id = asset_property_aggregates.id
asset_property_aggregates_aggregated_values = asset_property_aggregates.aggregated_values
asset_property_aggregates_next_token = asset_property_aggregates.next_token
```

---


### Access_policy

AccessPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p> |
| `access_policy_permission` | String | ✅ | <p>The permission level for this access policy. Note that a project <code>ADMINISTRATOR</code> is also known as a project owner.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs that contain metadata for the access policy. For more
      information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/tag-resources.html">Tagging your
        IoT SiteWise resources</a> in the <i>IoT SiteWise User Guide</i>.</p> |
| `access_policy_identity` | String | ✅ | <p>The identity for this access policy. Choose an IAM Identity Center user, an IAM Identity Center group, or an IAM user.</p> |
| `access_policy_resource` | String | ✅ | <p>The IoT SiteWise Monitor resource for this access policy. Choose either a portal or a project.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_policy_last_update_date` | String | <p>The date the access policy was last updated, in Unix epoch time.</p> |
| `access_policy_resource` | String | <p>The IoT SiteWise Monitor resource (portal or project) to which this access policy provides
      access.</p> |
| `access_policy_arn` | String | <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the access policy, which has the following format.</p>
         <p>
            <code>arn:${Partition}:iotsitewise:${Region}:${Account}:access-policy/${AccessPolicyId}</code>
         </p> |
| `access_policy_identity` | String | <p>The identity (IAM Identity Center user, IAM Identity Center group, or IAM user) to which this access policy
      applies.</p> |
| `access_policy_permission` | String | <p>The access policy permission. Note that a project <code>ADMINISTRATOR</code> is also known
      as a project owner.</p> |
| `access_policy_id` | String | <p>The ID of the access policy.</p> |
| `access_policy_creation_date` | String | <p>The date the access policy was created, in Unix epoch time.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create access_policy
access_policy = provider.iotsitewise.Access_policy {
    access_policy_permission = "value"  # <p>The permission level for this access policy. Note that a project <code>ADMINISTRATOR</code> is also known as a project owner.</p>
    access_policy_identity = "value"  # <p>The identity for this access policy. Choose an IAM Identity Center user, an IAM Identity Center group, or an IAM user.</p>
    access_policy_resource = "value"  # <p>The IoT SiteWise Monitor resource for this access policy. Choose either a portal or a project.</p>
}

# Access access_policy outputs
access_policy_id = access_policy.id
access_policy_access_policy_last_update_date = access_policy.access_policy_last_update_date
access_policy_access_policy_resource = access_policy.access_policy_resource
access_policy_access_policy_arn = access_policy.access_policy_arn
access_policy_access_policy_identity = access_policy.access_policy_identity
access_policy_access_policy_permission = access_policy.access_policy_permission
access_policy_access_policy_id = access_policy.access_policy_id
access_policy_access_policy_creation_date = access_policy.access_policy_creation_date
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple time_series resources
time_series_0 = provider.iotsitewise.Time_series {
}
time_series_1 = provider.iotsitewise.Time_series {
}
time_series_2 = provider.iotsitewise.Time_series {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    time_series = provider.iotsitewise.Time_series {
    }
```

---

## Related Documentation

- [AWS Iotsitewise Documentation](https://docs.aws.amazon.com/iotsitewise/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
