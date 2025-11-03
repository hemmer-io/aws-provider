# Pinpoint Service



**Resources**: 53

---

## Overview

The pinpoint service provides access to 53 resource types:

- [App](#app) [CRD]
- [Segment_versions](#segment_versions) [R]
- [Email_channel](#email_channel) [RUD]
- [Recommender_configuration](#recommender_configuration) [CRUD]
- [Apns_channel](#apns_channel) [RUD]
- [Export_jobs](#export_jobs) [R]
- [Push_template](#push_template) [CRUD]
- [Segment](#segment) [CRUD]
- [Gcm_channel](#gcm_channel) [RUD]
- [Application_date_range_kpi](#application_date_range_kpi) [R]
- [Endpoint](#endpoint) [RUD]
- [Export_job](#export_job) [CR]
- [User_endpoints](#user_endpoints) [RD]
- [Campaigns](#campaigns) [R]
- [Journey_run_execution_metrics](#journey_run_execution_metrics) [R]
- [Journey](#journey) [CRUD]
- [Journey_state](#journey_state) [U]
- [Voice_channel](#voice_channel) [RUD]
- [Journey_runs](#journey_runs) [R]
- [Segments](#segments) [R]
- [Application_settings](#application_settings) [RU]
- [Import_job](#import_job) [CR]
- [Campaign](#campaign) [CRUD]
- [Voice_template](#voice_template) [CRUD]
- [Apns_sandbox_channel](#apns_sandbox_channel) [RUD]
- [Apns_voip_channel](#apns_voip_channel) [RUD]
- [Apps](#apps) [R]
- [Import_jobs](#import_jobs) [R]
- [Apns_voip_sandbox_channel](#apns_voip_sandbox_channel) [RUD]
- [Sms_template](#sms_template) [CRUD]
- [Campaign_date_range_kpi](#campaign_date_range_kpi) [R]
- [Campaign_version](#campaign_version) [R]
- [Journey_execution_metrics](#journey_execution_metrics) [R]
- [Segment_version](#segment_version) [R]
- [Events](#events) [C]
- [Template_active_version](#template_active_version) [U]
- [Campaign_versions](#campaign_versions) [R]
- [Journey_date_range_kpi](#journey_date_range_kpi) [R]
- [Recommender_configurations](#recommender_configurations) [R]
- [Segment_import_jobs](#segment_import_jobs) [R]
- [In_app_template](#in_app_template) [CRUD]
- [Endpoints_batch](#endpoints_batch) [U]
- [Adm_channel](#adm_channel) [RUD]
- [Channels](#channels) [R]
- [Segment_export_jobs](#segment_export_jobs) [R]
- [Baidu_channel](#baidu_channel) [RUD]
- [Event_stream](#event_stream) [CRD]
- [Sms_channel](#sms_channel) [RUD]
- [Campaign_activities](#campaign_activities) [R]
- [In_app_messages](#in_app_messages) [R]
- [Journey_execution_activity_metrics](#journey_execution_activity_metrics) [R]
- [Journey_run_execution_activity_metrics](#journey_run_execution_activity_metrics) [R]
- [Email_template](#email_template) [CRUD]

---

## Resources


### App

App resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_application_request` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `application_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app
app = provider.pinpoint.App {
    create_application_request = "value"  # Required field
}

# Access app outputs
app_id = app.id
app_application_response = app.application_response
```

---


### Segment_versions

SegmentVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `segments_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access segment_versions outputs
segment_versions_id = segment_versions.id
segment_versions_segments_response = segment_versions.segments_response
```

---


### Email_channel

EmailChannel resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email_channel_request` | String | ✅ |  |
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `email_channel_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access email_channel outputs
email_channel_id = email_channel.id
email_channel_email_channel_response = email_channel.email_channel_response
```

---


### Recommender_configuration

RecommenderConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_recommender_configuration` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recommender_configuration_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create recommender_configuration
recommender_configuration = provider.pinpoint.Recommender_configuration {
    create_recommender_configuration = "value"  # Required field
}

# Access recommender_configuration outputs
recommender_configuration_id = recommender_configuration.id
recommender_configuration_recommender_configuration_response = recommender_configuration.recommender_configuration_response
```

---


### Apns_channel

ApnsChannel resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |
| `apns_channel_request` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `apns_channel_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access apns_channel outputs
apns_channel_id = apns_channel.id
apns_channel_apns_channel_response = apns_channel.apns_channel_response
```

---


### Export_jobs

ExportJobs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `export_jobs_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access export_jobs outputs
export_jobs_id = export_jobs.id
export_jobs_export_jobs_response = export_jobs.export_jobs_response
```

---


### Push_template

PushTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `push_notification_template_request` | String | ✅ |  |
| `template_name` | String | ✅ | <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `push_notification_template_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create push_template
push_template = provider.pinpoint.Push_template {
    push_notification_template_request = "value"  # Required field
    template_name = "value"  # <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
}

# Access push_template outputs
push_template_id = push_template.id
push_template_push_notification_template_response = push_template.push_notification_template_response
```

---


### Segment

Segment resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |
| `write_segment_request` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `segment_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create segment
segment = provider.pinpoint.Segment {
    application_id = "value"  # <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    write_segment_request = "value"  # Required field
}

# Access segment outputs
segment_id = segment.id
segment_segment_response = segment.segment_response
```

---


### Gcm_channel

GcmChannel resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gcm_channel_request` | String | ✅ |  |
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `gcm_channel_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access gcm_channel outputs
gcm_channel_id = gcm_channel.id
gcm_channel_gcm_channel_response = gcm_channel.gcm_channel_response
```

---


### Application_date_range_kpi

ApplicationDateRangeKpi resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `application_date_range_kpi_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access application_date_range_kpi outputs
application_date_range_kpi_id = application_date_range_kpi.id
application_date_range_kpi_application_date_range_kpi_response = application_date_range_kpi.application_date_range_kpi_response
```

---


### Endpoint

Endpoint resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `endpoint_request` | String | ✅ |  |
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |
| `endpoint_id` | String | ✅ | <p>The case insensitive unique identifier for the endpoint. The identifier can't contain <code>$</code>, <code>{</code> or <code>}</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoint_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access endpoint outputs
endpoint_id = endpoint.id
endpoint_endpoint_response = endpoint.endpoint_response
```

---


### Export_job

ExportJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |
| `export_job_request` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `export_job_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create export_job
export_job = provider.pinpoint.Export_job {
    application_id = "value"  # <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    export_job_request = "value"  # Required field
}

# Access export_job outputs
export_job_id = export_job.id
export_job_export_job_response = export_job.export_job_response
```

---


### User_endpoints

UserEndpoints resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoints_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access user_endpoints outputs
user_endpoints_id = user_endpoints.id
user_endpoints_endpoints_response = user_endpoints.endpoints_response
```

---


### Campaigns

Campaigns resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `campaigns_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access campaigns outputs
campaigns_id = campaigns.id
campaigns_campaigns_response = campaigns.campaigns_response
```

---


### Journey_run_execution_metrics

JourneyRunExecutionMetrics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `journey_run_execution_metrics_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access journey_run_execution_metrics outputs
journey_run_execution_metrics_id = journey_run_execution_metrics.id
journey_run_execution_metrics_journey_run_execution_metrics_response = journey_run_execution_metrics.journey_run_execution_metrics_response
```

---


### Journey

Journey resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |
| `write_journey_request` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `journey_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create journey
journey = provider.pinpoint.Journey {
    application_id = "value"  # <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    write_journey_request = "value"  # Required field
}

# Access journey outputs
journey_id = journey.id
journey_journey_response = journey.journey_response
```

---


### Journey_state

JourneyState resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `journey_id` | String | ✅ | <p>The unique identifier for the journey.</p> |
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |
| `journey_state_request` | String | ✅ |  |



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


### Voice_channel

VoiceChannel resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `voice_channel_request` | String | ✅ |  |
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `voice_channel_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access voice_channel outputs
voice_channel_id = voice_channel.id
voice_channel_voice_channel_response = voice_channel.voice_channel_response
```

---


### Journey_runs

JourneyRuns resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `journey_runs_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access journey_runs outputs
journey_runs_id = journey_runs.id
journey_runs_journey_runs_response = journey_runs.journey_runs_response
```

---


### Segments

Segments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `segments_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access segments outputs
segments_id = segments.id
segments_segments_response = segments.segments_response
```

---


### Application_settings

ApplicationSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |
| `write_application_settings_request` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `application_settings_resource` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access application_settings outputs
application_settings_id = application_settings.id
application_settings_application_settings_resource = application_settings.application_settings_resource
```

---


### Import_job

ImportJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `import_job_request` | String | ✅ |  |
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `import_job_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create import_job
import_job = provider.pinpoint.Import_job {
    import_job_request = "value"  # Required field
    application_id = "value"  # <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
}

# Access import_job outputs
import_job_id = import_job.id
import_job_import_job_response = import_job.import_job_response
```

---


### Campaign

Campaign resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `write_campaign_request` | String | ✅ |  |
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `campaign_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create campaign
campaign = provider.pinpoint.Campaign {
    write_campaign_request = "value"  # Required field
    application_id = "value"  # <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
}

# Access campaign outputs
campaign_id = campaign.id
campaign_campaign_response = campaign.campaign_response
```

---


### Voice_template

VoiceTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `voice_template_request` | String | ✅ |  |
| `template_name` | String | ✅ | <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `voice_template_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create voice_template
voice_template = provider.pinpoint.Voice_template {
    voice_template_request = "value"  # Required field
    template_name = "value"  # <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
}

# Access voice_template outputs
voice_template_id = voice_template.id
voice_template_voice_template_response = voice_template.voice_template_response
```

---


### Apns_sandbox_channel

ApnsSandboxChannel resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `apns_sandbox_channel_request` | String | ✅ |  |
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `apns_sandbox_channel_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access apns_sandbox_channel outputs
apns_sandbox_channel_id = apns_sandbox_channel.id
apns_sandbox_channel_apns_sandbox_channel_response = apns_sandbox_channel.apns_sandbox_channel_response
```

---


### Apns_voip_channel

ApnsVoipChannel resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |
| `apns_voip_channel_request` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `apns_voip_channel_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access apns_voip_channel outputs
apns_voip_channel_id = apns_voip_channel.id
apns_voip_channel_apns_voip_channel_response = apns_voip_channel.apns_voip_channel_response
```

---


### Apps

Apps resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `applications_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access apps outputs
apps_id = apps.id
apps_applications_response = apps.applications_response
```

---


### Import_jobs

ImportJobs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `import_jobs_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access import_jobs outputs
import_jobs_id = import_jobs.id
import_jobs_import_jobs_response = import_jobs.import_jobs_response
```

---


### Apns_voip_sandbox_channel

ApnsVoipSandboxChannel resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `apns_voip_sandbox_channel_request` | String | ✅ |  |
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `apns_voip_sandbox_channel_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access apns_voip_sandbox_channel outputs
apns_voip_sandbox_channel_id = apns_voip_sandbox_channel.id
apns_voip_sandbox_channel_apns_voip_sandbox_channel_response = apns_voip_sandbox_channel.apns_voip_sandbox_channel_response
```

---


### Sms_template

SmsTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sms_template_request` | String | ✅ |  |
| `template_name` | String | ✅ | <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sms_template_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sms_template
sms_template = provider.pinpoint.Sms_template {
    sms_template_request = "value"  # Required field
    template_name = "value"  # <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
}

# Access sms_template outputs
sms_template_id = sms_template.id
sms_template_sms_template_response = sms_template.sms_template_response
```

---


### Campaign_date_range_kpi

CampaignDateRangeKpi resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `campaign_date_range_kpi_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access campaign_date_range_kpi outputs
campaign_date_range_kpi_id = campaign_date_range_kpi.id
campaign_date_range_kpi_campaign_date_range_kpi_response = campaign_date_range_kpi.campaign_date_range_kpi_response
```

---


### Campaign_version

CampaignVersion resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `campaign_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access campaign_version outputs
campaign_version_id = campaign_version.id
campaign_version_campaign_response = campaign_version.campaign_response
```

---


### Journey_execution_metrics

JourneyExecutionMetrics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `journey_execution_metrics_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access journey_execution_metrics outputs
journey_execution_metrics_id = journey_execution_metrics.id
journey_execution_metrics_journey_execution_metrics_response = journey_execution_metrics.journey_execution_metrics_response
```

---


### Segment_version

SegmentVersion resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `segment_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access segment_version outputs
segment_version_id = segment_version.id
segment_version_segment_response = segment_version.segment_response
```

---


### Events

Events resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |
| `events_request` | String | ✅ |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create events
events = provider.pinpoint.Events {
    application_id = "value"  # <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    events_request = "value"  # Required field
}

```

---


### Template_active_version

TemplateActiveVersion resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `template_name` | String | ✅ | <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p> |
| `template_active_version_request` | String | ✅ |  |
| `template_type` | String | ✅ | <p>The type of channel that the message template is designed for. Valid values are: EMAIL, PUSH, SMS, and VOICE.</p> |



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


### Campaign_versions

CampaignVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `campaigns_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access campaign_versions outputs
campaign_versions_id = campaign_versions.id
campaign_versions_campaigns_response = campaign_versions.campaigns_response
```

---


### Journey_date_range_kpi

JourneyDateRangeKpi resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `journey_date_range_kpi_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access journey_date_range_kpi outputs
journey_date_range_kpi_id = journey_date_range_kpi.id
journey_date_range_kpi_journey_date_range_kpi_response = journey_date_range_kpi.journey_date_range_kpi_response
```

---


### Recommender_configurations

RecommenderConfigurations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `list_recommender_configurations_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access recommender_configurations outputs
recommender_configurations_id = recommender_configurations.id
recommender_configurations_list_recommender_configurations_response = recommender_configurations.list_recommender_configurations_response
```

---


### Segment_import_jobs

SegmentImportJobs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `import_jobs_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access segment_import_jobs outputs
segment_import_jobs_id = segment_import_jobs.id
segment_import_jobs_import_jobs_response = segment_import_jobs.import_jobs_response
```

---


### In_app_template

InAppTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `in_app_template_request` | String | ✅ |  |
| `template_name` | String | ✅ | <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `in_app_template_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create in_app_template
in_app_template = provider.pinpoint.In_app_template {
    in_app_template_request = "value"  # Required field
    template_name = "value"  # <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
}

# Access in_app_template outputs
in_app_template_id = in_app_template.id
in_app_template_in_app_template_response = in_app_template.in_app_template_response
```

---


### Endpoints_batch

EndpointsBatch resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |
| `endpoint_batch_request` | String | ✅ |  |



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


### Adm_channel

AdmChannel resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `adm_channel_request` | String | ✅ |  |
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `adm_channel_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access adm_channel outputs
adm_channel_id = adm_channel.id
adm_channel_adm_channel_response = adm_channel.adm_channel_response
```

---


### Channels

Channels resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channels_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access channels outputs
channels_id = channels.id
channels_channels_response = channels.channels_response
```

---


### Segment_export_jobs

SegmentExportJobs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `export_jobs_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access segment_export_jobs outputs
segment_export_jobs_id = segment_export_jobs.id
segment_export_jobs_export_jobs_response = segment_export_jobs.export_jobs_response
```

---


### Baidu_channel

BaiduChannel resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |
| `baidu_channel_request` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `baidu_channel_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access baidu_channel outputs
baidu_channel_id = baidu_channel.id
baidu_channel_baidu_channel_response = baidu_channel.baidu_channel_response
```

---


### Event_stream

EventStream resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `write_event_stream` | String | ✅ |  |
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_stream` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_stream
event_stream = provider.pinpoint.Event_stream {
    write_event_stream = "value"  # Required field
    application_id = "value"  # <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
}

# Access event_stream outputs
event_stream_id = event_stream.id
event_stream_event_stream = event_stream.event_stream
```

---


### Sms_channel

SmsChannel resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sms_channel_request` | String | ✅ |  |
| `application_id` | String | ✅ | <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sms_channel_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sms_channel outputs
sms_channel_id = sms_channel.id
sms_channel_sms_channel_response = sms_channel.sms_channel_response
```

---


### Campaign_activities

CampaignActivities resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `activities_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access campaign_activities outputs
campaign_activities_id = campaign_activities.id
campaign_activities_activities_response = campaign_activities.activities_response
```

---


### In_app_messages

InAppMessages resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `in_app_messages_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access in_app_messages outputs
in_app_messages_id = in_app_messages.id
in_app_messages_in_app_messages_response = in_app_messages.in_app_messages_response
```

---


### Journey_execution_activity_metrics

JourneyExecutionActivityMetrics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `journey_execution_activity_metrics_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access journey_execution_activity_metrics outputs
journey_execution_activity_metrics_id = journey_execution_activity_metrics.id
journey_execution_activity_metrics_journey_execution_activity_metrics_response = journey_execution_activity_metrics.journey_execution_activity_metrics_response
```

---


### Journey_run_execution_activity_metrics

JourneyRunExecutionActivityMetrics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `journey_run_execution_activity_metrics_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access journey_run_execution_activity_metrics outputs
journey_run_execution_activity_metrics_id = journey_run_execution_activity_metrics.id
journey_run_execution_activity_metrics_journey_run_execution_activity_metrics_response = journey_run_execution_activity_metrics.journey_run_execution_activity_metrics_response
```

---


### Email_template

EmailTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `template_name` | String | ✅ | <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p> |
| `email_template_request` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `email_template_response` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create email_template
email_template = provider.pinpoint.Email_template {
    template_name = "value"  # <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    email_template_request = "value"  # Required field
}

# Access email_template outputs
email_template_id = email_template.id
email_template_email_template_response = email_template.email_template_response
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple app resources
app_0 = provider.pinpoint.App {
    create_application_request = "value-0"
}
app_1 = provider.pinpoint.App {
    create_application_request = "value-1"
}
app_2 = provider.pinpoint.App {
    create_application_request = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    app = provider.pinpoint.App {
        create_application_request = "production-value"
    }
```

---

## Related Documentation

- [AWS Pinpoint Documentation](https://docs.aws.amazon.com/pinpoint/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
