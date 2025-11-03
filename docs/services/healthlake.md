# Healthlake Service



**Resources**: 3

---

## Overview

The healthlake service provides access to 3 resource types:

- [Fhir_datastore](#fhir_datastore) [CRD]
- [Fhir_import_job](#fhir_import_job) [R]
- [Fhir_export_job](#fhir_export_job) [R]

---

## Resources


### Fhir_datastore

FHIRDatastore resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The resource tags applied to a data store when it is created.</p> |
| `identity_provider_configuration` | String |  | <p>The identity provider configuration to use for the data store.</p> |
| `preload_data_config` | String |  | <p>An optional parameter to preload (import) open source Synthea FHIR data upon creation of
         the data store.</p> |
| `sse_configuration` | String |  | <p>The server-side encryption key configuration for a customer-provided encryption key
         specified for creating a data store. </p> |
| `datastore_name` | String |  | <p>The data store name (user-generated).</p> |
| `client_token` | String |  | <p>An optional user-provided token to ensure API idempotency.</p> |
| `datastore_type_version` | String | ✅ | <p>The FHIR release version supported by the data store. Current support is for version
            <code>R4</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `datastore_properties` | String | <p>The data store properties.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create fhir_datastore
fhir_datastore = provider.healthlake.Fhir_datastore {
    datastore_type_version = "value"  # <p>The FHIR release version supported by the data store. Current support is for version
            <code>R4</code>.</p>
}

# Access fhir_datastore outputs
fhir_datastore_id = fhir_datastore.id
fhir_datastore_datastore_properties = fhir_datastore.datastore_properties
```

---


### Fhir_import_job

FHIRImportJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `import_job_properties` | String | <p>The import job properties.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fhir_import_job outputs
fhir_import_job_id = fhir_import_job.id
fhir_import_job_import_job_properties = fhir_import_job.import_job_properties
```

---


### Fhir_export_job

FHIRExportJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `export_job_properties` | String | <p>The export job properties.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fhir_export_job outputs
fhir_export_job_id = fhir_export_job.id
fhir_export_job_export_job_properties = fhir_export_job.export_job_properties
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple fhir_datastore resources
fhir_datastore_0 = provider.healthlake.Fhir_datastore {
    datastore_type_version = "value-0"
}
fhir_datastore_1 = provider.healthlake.Fhir_datastore {
    datastore_type_version = "value-1"
}
fhir_datastore_2 = provider.healthlake.Fhir_datastore {
    datastore_type_version = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    fhir_datastore = provider.healthlake.Fhir_datastore {
        datastore_type_version = "production-value"
    }
```

---

## Related Documentation

- [AWS Healthlake Documentation](https://docs.aws.amazon.com/healthlake/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
