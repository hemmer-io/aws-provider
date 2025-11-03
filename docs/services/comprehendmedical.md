# Comprehendmedical Service



**Resources**: 5

---

## Overview

The comprehendmedical service provides access to 5 resource types:

- [Icd10_cm_inference_job](#icd10_cm_inference_job) [R]
- [Snomedct_inference_job](#snomedct_inference_job) [R]
- [Phi_detection_job](#phi_detection_job) [R]
- [Rx_norm_inference_job](#rx_norm_inference_job) [R]
- [Entities_detection_v2_job](#entities_detection_v2_job) [R]

---

## Resources


### Icd10_cm_inference_job

ICD10CMInferenceJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `comprehend_medical_async_job_properties` | String | <p>An object that contains the properties associated with a detection job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access icd10_cm_inference_job outputs
icd10_cm_inference_job_id = icd10_cm_inference_job.id
icd10_cm_inference_job_comprehend_medical_async_job_properties = icd10_cm_inference_job.comprehend_medical_async_job_properties
```

---


### Snomedct_inference_job

SNOMEDCTInferenceJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `comprehend_medical_async_job_properties` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access snomedct_inference_job outputs
snomedct_inference_job_id = snomedct_inference_job.id
snomedct_inference_job_comprehend_medical_async_job_properties = snomedct_inference_job.comprehend_medical_async_job_properties
```

---


### Phi_detection_job

PHIDetectionJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `comprehend_medical_async_job_properties` | String | <p>An object that contains the properties associated with a detection job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access phi_detection_job outputs
phi_detection_job_id = phi_detection_job.id
phi_detection_job_comprehend_medical_async_job_properties = phi_detection_job.comprehend_medical_async_job_properties
```

---


### Rx_norm_inference_job

RxNormInferenceJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `comprehend_medical_async_job_properties` | String | <p>An object that contains the properties associated with a detection job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access rx_norm_inference_job outputs
rx_norm_inference_job_id = rx_norm_inference_job.id
rx_norm_inference_job_comprehend_medical_async_job_properties = rx_norm_inference_job.comprehend_medical_async_job_properties
```

---


### Entities_detection_v2_job

EntitiesDetectionV2Job resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `comprehend_medical_async_job_properties` | String | <p>An object that contains the properties associated with a detection job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access entities_detection_v2_job outputs
entities_detection_v2_job_id = entities_detection_v2_job.id
entities_detection_v2_job_comprehend_medical_async_job_properties = entities_detection_v2_job.comprehend_medical_async_job_properties
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple icd10_cm_inference_job resources
icd10_cm_inference_job_0 = provider.comprehendmedical.Icd10_cm_inference_job {
}
icd10_cm_inference_job_1 = provider.comprehendmedical.Icd10_cm_inference_job {
}
icd10_cm_inference_job_2 = provider.comprehendmedical.Icd10_cm_inference_job {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    icd10_cm_inference_job = provider.comprehendmedical.Icd10_cm_inference_job {
    }
```

---

## Related Documentation

- [AWS Comprehendmedical Documentation](https://docs.aws.amazon.com/comprehendmedical/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
