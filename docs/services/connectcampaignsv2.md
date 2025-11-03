# Connectcampaignsv2 Service



**Resources**: 17

---

## Overview

The connectcampaignsv2 service provides access to 17 resource types:

- [Instance_onboarding_job](#instance_onboarding_job) [D]
- [Campaign_state](#campaign_state) [R]
- [Instance_onboarding_job_status](#instance_onboarding_job_status) [R]
- [Campaign_state_batch](#campaign_state_batch) [R]
- [Campaign_communication_limits](#campaign_communication_limits) [UD]
- [Campaign_schedule](#campaign_schedule) [U]
- [Instance_communication_limits](#instance_communication_limits) [CR]
- [Campaign_communication_time](#campaign_communication_time) [UD]
- [Campaign](#campaign) [CRD]
- [Campaign_source](#campaign_source) [U]
- [Outbound_request_batch](#outbound_request_batch) [C]
- [Profile_outbound_request_batch](#profile_outbound_request_batch) [C]
- [Connect_instance_config](#connect_instance_config) [RD]
- [Campaign_flow_association](#campaign_flow_association) [U]
- [Campaign_name](#campaign_name) [U]
- [Campaign_channel_subtype_config](#campaign_channel_subtype_config) [UD]
- [Connect_instance_integration](#connect_instance_integration) [CD]

---

## Resources


### Instance_onboarding_job

InstanceOnboardingJob resource

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


### Campaign_state

CampaignState resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access campaign_state outputs
campaign_state_id = campaign_state.id
campaign_state_state = campaign_state.state
```

---


### Instance_onboarding_job_status

InstanceOnboardingJobStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connect_instance_onboarding_job_status` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_onboarding_job_status outputs
instance_onboarding_job_status_id = instance_onboarding_job_status.id
instance_onboarding_job_status_connect_instance_onboarding_job_status = instance_onboarding_job_status.connect_instance_onboarding_job_status
```

---


### Campaign_state_batch

CampaignStateBatch resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `successful_requests` | Vec<String> |  |
| `failed_requests` | Vec<String> |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access campaign_state_batch outputs
campaign_state_batch_id = campaign_state_batch.id
campaign_state_batch_successful_requests = campaign_state_batch.successful_requests
campaign_state_batch_failed_requests = campaign_state_batch.failed_requests
```

---


### Campaign_communication_limits

CampaignCommunicationLimits resource

**Operations**: ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `communication_limits_override` | String | ✅ |  |
| `id` | String | ✅ |  |



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


### Campaign_schedule

CampaignSchedule resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ |  |
| `schedule` | String | ✅ |  |



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


### Instance_communication_limits

InstanceCommunicationLimits resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connect_instance_id` | String | ✅ |  |
| `communication_limits_config` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `communication_limits_config` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create instance_communication_limits
instance_communication_limits = provider.connectcampaignsv2.Instance_communication_limits {
    connect_instance_id = "value"  # Required field
    communication_limits_config = "value"  # Required field
}

# Access instance_communication_limits outputs
instance_communication_limits_id = instance_communication_limits.id
instance_communication_limits_communication_limits_config = instance_communication_limits.communication_limits_config
```

---


### Campaign_communication_time

CampaignCommunicationTime resource

**Operations**: ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ |  |
| `communication_time_config` | String | ✅ |  |



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


### Campaign

Campaign resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source` | String |  |  |
| `connect_campaign_flow_arn` | String |  |  |
| `schedule` | String |  |  |
| `communication_time_config` | String |  |  |
| `communication_limits_override` | String |  |  |
| `channel_subtype_config` | String | ✅ |  |
| `name` | String | ✅ |  |
| `connect_instance_id` | String | ✅ |  |
| `tags` | HashMap<String, String> |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `campaign` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create campaign
campaign = provider.connectcampaignsv2.Campaign {
    channel_subtype_config = "value"  # Required field
    name = "value"  # Required field
    connect_instance_id = "value"  # Required field
}

# Access campaign outputs
campaign_id = campaign.id
campaign_campaign = campaign.campaign
```

---


### Campaign_source

CampaignSource resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ |  |
| `source` | String | ✅ |  |



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


### Outbound_request_batch

OutboundRequestBatch resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `outbound_requests` | Vec<String> | ✅ |  |
| `id` | String | ✅ |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create outbound_request_batch
outbound_request_batch = provider.connectcampaignsv2.Outbound_request_batch {
    outbound_requests = "value"  # Required field
    id = "value"  # Required field
}

```

---


### Profile_outbound_request_batch

ProfileOutboundRequestBatch resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ |  |
| `profile_outbound_requests` | Vec<String> | ✅ |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create profile_outbound_request_batch
profile_outbound_request_batch = provider.connectcampaignsv2.Profile_outbound_request_batch {
    id = "value"  # Required field
    profile_outbound_requests = "value"  # Required field
}

```

---


### Connect_instance_config

ConnectInstanceConfig resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connect_instance_config` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connect_instance_config outputs
connect_instance_config_id = connect_instance_config.id
connect_instance_config_connect_instance_config = connect_instance_config.connect_instance_config
```

---


### Campaign_flow_association

CampaignFlowAssociation resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ |  |
| `connect_campaign_flow_arn` | String | ✅ |  |



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


### Campaign_name

CampaignName resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ |  |
| `name` | String | ✅ |  |



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


### Campaign_channel_subtype_config

CampaignChannelSubtypeConfig resource

**Operations**: ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ |  |
| `channel_subtype_config` | String | ✅ |  |



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


### Connect_instance_integration

ConnectInstanceIntegration resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connect_instance_id` | String | ✅ |  |
| `integration_config` | String | ✅ |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connect_instance_integration
connect_instance_integration = provider.connectcampaignsv2.Connect_instance_integration {
    connect_instance_id = "value"  # Required field
    integration_config = "value"  # Required field
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

# Create multiple instance_onboarding_job resources
instance_onboarding_job_0 = provider.connectcampaignsv2.Instance_onboarding_job {
}
instance_onboarding_job_1 = provider.connectcampaignsv2.Instance_onboarding_job {
}
instance_onboarding_job_2 = provider.connectcampaignsv2.Instance_onboarding_job {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    instance_onboarding_job = provider.connectcampaignsv2.Instance_onboarding_job {
    }
```

---

## Related Documentation

- [AWS Connectcampaignsv2 Documentation](https://docs.aws.amazon.com/connectcampaignsv2/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
