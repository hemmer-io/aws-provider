# Connectcampaigns Service



**Resources**: 10

---

## Overview

The connectcampaigns service provides access to 10 resource types:

- [Instance_onboarding_job_status](#instance_onboarding_job_status) [R]
- [Dial_request_batch](#dial_request_batch) [C]
- [Campaign_state](#campaign_state) [R]
- [Instance_onboarding_job](#instance_onboarding_job) [D]
- [Campaign](#campaign) [CRD]
- [Campaign_state_batch](#campaign_state_batch) [R]
- [Campaign_name](#campaign_name) [U]
- [Campaign_dialer_config](#campaign_dialer_config) [U]
- [Campaign_outbound_call_config](#campaign_outbound_call_config) [U]
- [Connect_instance_config](#connect_instance_config) [RD]

---

## Resources


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


### Dial_request_batch

DialRequestBatch resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dial_requests` | Vec<String> | ✅ |  |
| `id` | String | ✅ |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dial_request_batch
dial_request_batch = provider.connectcampaigns.Dial_request_batch {
    dial_requests = "value"  # Required field
    id = "value"  # Required field
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


### Campaign

Campaign resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `outbound_call_config` | String | ✅ |  |
| `tags` | HashMap<String, String> |  |  |
| `connect_instance_id` | String | ✅ |  |
| `name` | String | ✅ |  |
| `dialer_config` | String | ✅ |  |


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
campaign = provider.connectcampaigns.Campaign {
    outbound_call_config = "value"  # Required field
    connect_instance_id = "value"  # Required field
    name = "value"  # Required field
    dialer_config = "value"  # Required field
}

# Access campaign outputs
campaign_id = campaign.id
campaign_campaign = campaign.campaign
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
| `failed_requests` | Vec<String> |  |
| `successful_requests` | Vec<String> |  |


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
campaign_state_batch_failed_requests = campaign_state_batch.failed_requests
campaign_state_batch_successful_requests = campaign_state_batch.successful_requests
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


### Campaign_dialer_config

CampaignDialerConfig resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dialer_config` | String | ✅ |  |
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


### Campaign_outbound_call_config

CampaignOutboundCallConfig resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `answer_machine_detection_config` | String |  |  |
| `connect_contact_flow_id` | String |  |  |
| `id` | String | ✅ |  |
| `connect_source_phone_number` | String |  |  |



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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple instance_onboarding_job_status resources
instance_onboarding_job_status_0 = provider.connectcampaigns.Instance_onboarding_job_status {
}
instance_onboarding_job_status_1 = provider.connectcampaigns.Instance_onboarding_job_status {
}
instance_onboarding_job_status_2 = provider.connectcampaigns.Instance_onboarding_job_status {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    instance_onboarding_job_status = provider.connectcampaigns.Instance_onboarding_job_status {
    }
```

---

## Related Documentation

- [AWS Connectcampaigns Documentation](https://docs.aws.amazon.com/connectcampaigns/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
