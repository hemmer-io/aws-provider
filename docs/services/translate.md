# Translate Service



**Resources**: 3

---

## Overview

The translate service provides access to 3 resource types:

- [Parallel_data](#parallel_data) [CRUD]
- [Terminology](#terminology) [RD]
- [Text_translation_job](#text_translation_job) [R]

---

## Resources


### Parallel_data

ParallelData resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>A custom name for the parallel data resource in Amazon Translate. You must assign a name
      that is unique in the account and region.</p> |
| `parallel_data_config` | String | ✅ | <p>Specifies the format and S3 location of the parallel data input file.</p> |
| `encryption_key` | String |  |  |
| `client_token` | String | ✅ | <p>A unique identifier for the request. This token is automatically generated when you use
      Amazon Translate through an AWS SDK.</p> |
| `tags` | Vec<String> |  | <p>Tags to be associated with this resource. A tag is a key-value pair that
      adds metadata to a resource. Each tag key for the resource must be unique.
      For more information, see <a href="https://docs.aws.amazon.com/translate/latest/dg/tagging.html">
        Tagging your resources</a>.</p> |
| `description` | String |  | <p>A custom description for the parallel data resource in Amazon Translate.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auxiliary_data_location` | String | <p>The Amazon S3 location of a file that provides any errors or warnings that were produced
      by your input file. This file was created when Amazon Translate attempted to create a parallel
      data resource. The location is returned as a presigned URL to that has a 30-minute
      expiration.</p> |
| `parallel_data_properties` | String | <p>The properties of the parallel data resource that is being retrieved.</p> |
| `latest_update_attempt_auxiliary_data_location` | String | <p>The Amazon S3 location of a file that provides any errors or warnings that were produced
      by your input file. This file was created when Amazon Translate attempted to update a parallel
      data resource. The location is returned as a presigned URL to that has a 30-minute
      expiration.</p> |
| `data_location` | String | <p>The Amazon S3 location of the most recent parallel data input file that was successfully
      imported into Amazon Translate. The location is returned as a presigned URL that has a
      30-minute expiration.</p>
    
         <important>
            <p>Amazon Translate doesn't scan all input files for the risk of CSV injection
        attacks. </p>
            <p>CSV injection occurs when a .csv or .tsv file is altered so that a record contains
        malicious code. The record begins with a special character, such as =, +, -, or @. When the
        file is opened in a spreadsheet program, the program might interpret the record as a formula
        and run the code within it.</p>
            <p>Before you download an input file from Amazon S3, ensure that you recognize the file and trust its creator.</p>
         </important> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create parallel_data
parallel_data = provider.translate.Parallel_data {
    name = "value"  # <p>A custom name for the parallel data resource in Amazon Translate. You must assign a name
      that is unique in the account and region.</p>
    parallel_data_config = "value"  # <p>Specifies the format and S3 location of the parallel data input file.</p>
    client_token = "value"  # <p>A unique identifier for the request. This token is automatically generated when you use
      Amazon Translate through an AWS SDK.</p>
}

# Access parallel_data outputs
parallel_data_id = parallel_data.id
parallel_data_auxiliary_data_location = parallel_data.auxiliary_data_location
parallel_data_parallel_data_properties = parallel_data.parallel_data_properties
parallel_data_latest_update_attempt_auxiliary_data_location = parallel_data.latest_update_attempt_auxiliary_data_location
parallel_data_data_location = parallel_data.data_location
```

---


### Terminology

Terminology resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `terminology_properties` | String | <p>The properties of the custom terminology being retrieved.</p> |
| `terminology_data_location` | String | <p>The Amazon S3 location of the most recent custom terminology input file that was
      successfully imported into Amazon Translate. The location is returned as a presigned URL that
      has a 30-minute expiration.</p>
    
         <important>
            <p>Amazon Translate doesn't scan all input files for the risk of CSV injection
        attacks. </p>
            <p>CSV injection occurs when a .csv or .tsv file is altered so that a record contains
        malicious code. The record begins with a special character, such as =, +, -, or @. When the
        file is opened in a spreadsheet program, the program might interpret the record as a formula
        and run the code within it.</p>
            <p>Before you download an input file from Amazon S3, ensure that you recognize the file and trust its creator.</p>
         </important> |
| `auxiliary_data_location` | String | <p>The Amazon S3 location of a file that provides any errors or warnings that were produced
      by your input file. This file was created when Amazon Translate attempted to create a
      terminology resource. The location is returned as a presigned URL to that has a 30-minute
      expiration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access terminology outputs
terminology_id = terminology.id
terminology_terminology_properties = terminology.terminology_properties
terminology_terminology_data_location = terminology.terminology_data_location
terminology_auxiliary_data_location = terminology.auxiliary_data_location
```

---


### Text_translation_job

TextTranslationJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `text_translation_job_properties` | String | <p>An object that contains the properties associated with an asynchronous batch translation
      job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access text_translation_job outputs
text_translation_job_id = text_translation_job.id
text_translation_job_text_translation_job_properties = text_translation_job.text_translation_job_properties
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple parallel_data resources
parallel_data_0 = provider.translate.Parallel_data {
    name = "value-0"
    parallel_data_config = "value-0"
    client_token = "value-0"
}
parallel_data_1 = provider.translate.Parallel_data {
    name = "value-1"
    parallel_data_config = "value-1"
    client_token = "value-1"
}
parallel_data_2 = provider.translate.Parallel_data {
    name = "value-2"
    parallel_data_config = "value-2"
    client_token = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    parallel_data = provider.translate.Parallel_data {
        name = "production-value"
        parallel_data_config = "production-value"
        client_token = "production-value"
    }
```

---

## Related Documentation

- [AWS Translate Documentation](https://docs.aws.amazon.com/translate/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
