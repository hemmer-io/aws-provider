# Connectcampaigns Service



**Resources**: 10

---

## Overview

The connectcampaigns service provides access to 10 resource types:

- [Campaign_dialer_config](#campaign_dialer_config) [U]
- [Campaign_outbound_call_config](#campaign_outbound_call_config) [U]
- [Instance_onboarding_job](#instance_onboarding_job) [D]
- [Connect_instance_config](#connect_instance_config) [RD]
- [Dial_request_batch](#dial_request_batch) [C]
- [Instance_onboarding_job_status](#instance_onboarding_job_status) [R]
- [Campaign_name](#campaign_name) [U]
- [Campaign](#campaign) [CRD]
- [Campaign_state_batch](#campaign_state_batch) [R]
- [Campaign_state](#campaign_state) [R]

---

## Resources


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
| `id` | String | ✅ |  |
| `connect_source_phone_number` | String |  |  |
| `connect_contact_flow_id` | String |  |  |



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


### Campaign

Campaign resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `outbound_call_config` | String | ✅ |  |
| `name` | String | ✅ |  |
| `connect_instance_id` | String | ✅ |  |
| `dialer_config` | String | ✅ |  |
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
campaign = provider.connectcampaigns.Campaign {
    outbound_call_config = "value"  # Required field
    name = "value"  # Required field
    connect_instance_id = "value"  # Required field
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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple campaign_dialer_config resources
campaign_dialer_config_0 = provider.connectcampaigns.Campaign_dialer_config {
    dialer_config = "value-0"
    id = "value-0"
}
campaign_dialer_config_1 = provider.connectcampaigns.Campaign_dialer_config {
    dialer_config = "value-1"
    id = "value-1"
}
campaign_dialer_config_2 = provider.connectcampaigns.Campaign_dialer_config {
    dialer_config = "value-2"
    id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    campaign_dialer_config = provider.connectcampaigns.Campaign_dialer_config {
        dialer_config = "production-value"
        id = "production-value"
    }
```

---

## Related Documentation

- [AWS Connectcampaigns Documentation](https://docs.aws.amazon.com/connectcampaigns/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
