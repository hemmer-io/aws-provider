# Medical_imaging Service



**Resources**: 4

---

## Overview

The medical_imaging service provides access to 4 resource types:

- [Dicom_import_job](#dicom_import_job) [R]
- [Image_set](#image_set) [RD]
- [Image_set_metadata](#image_set_metadata) [RU]
- [Image_frame](#image_frame) [R]

---

## Resources


### Dicom_import_job

DICOMImportJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_properties` | String | <p>The properties of the import job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dicom_import_job outputs
dicom_import_job_id = dicom_import_job.id
dicom_import_job_job_properties = dicom_import_job.job_properties
```

---


### Image_set

ImageSet resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `image_set_state` | String | <p>The image set state.</p> |
| `overrides` | String | <p>This object contains the details of any overrides used while creating a specific image set version. If an image set was copied or updated using the <code>force</code> flag, this object will contain the <code>forced</code> flag.</p> |
| `created_at` | String | <p>The timestamp when image set properties were created.</p> |
| `deleted_at` | String | <p>The timestamp when the image set properties were deleted.</p> |
| `message` | String | <p>The error message thrown if an image set action fails.</p> |
| `updated_at` | String | <p>The timestamp when image set properties were updated.</p> |
| `version_id` | String | <p>The image set version identifier.</p> |
| `datastore_id` | String | <p>The data store identifier.</p> |
| `image_set_workflow_status` | String | <p>The image set workflow status.</p> |
| `image_set_arn` | String | <p>The Amazon Resource Name (ARN) assigned to the image set.</p> |
| `is_primary` | bool | <p>The flag to determine whether the image set is primary or not.</p> |
| `image_set_id` | String | <p>The image set identifier.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access image_set outputs
image_set_id = image_set.id
image_set_image_set_state = image_set.image_set_state
image_set_overrides = image_set.overrides
image_set_created_at = image_set.created_at
image_set_deleted_at = image_set.deleted_at
image_set_message = image_set.message
image_set_updated_at = image_set.updated_at
image_set_version_id = image_set.version_id
image_set_datastore_id = image_set.datastore_id
image_set_image_set_workflow_status = image_set.image_set_workflow_status
image_set_image_set_arn = image_set.image_set_arn
image_set_is_primary = image_set.is_primary
image_set_image_set_id = image_set.image_set_id
```

---


### Image_set_metadata

ImageSetMetadata resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `force` | bool |  | <p>Setting this flag will force the <code>UpdateImageSetMetadata</code> operation for the following attributes:</p> <ul> <li> <p> <code>Tag.StudyInstanceUID</code>, <code>Tag.SeriesInstanceUID</code>, <code>Tag.SOPInstanceUID</code>, and <code>Tag.StudyID</code> </p> </li> <li> <p>Adding, removing, or updating private tags for an individual SOP Instance</p> </li> </ul> |
| `latest_version_id` | String | ✅ | <p>The latest image set version identifier.</p> |
| `update_image_set_metadata_updates` | String | ✅ | <p>Update image set metadata updates.</p> |
| `datastore_id` | String | ✅ | <p>The data store identifier.</p> |
| `image_set_id` | String | ✅ | <p>The image set identifier.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `image_set_metadata_blob` | String | <p>The blob containing the aggregated metadata information for the image set.</p> |
| `content_encoding` | String | <p>The compression format in which image set metadata attributes are returned.</p> |
| `content_type` | String | <p>The format in which the study metadata is returned to the customer. Default is <code>text/plain</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access image_set_metadata outputs
image_set_metadata_id = image_set_metadata.id
image_set_metadata_image_set_metadata_blob = image_set_metadata.image_set_metadata_blob
image_set_metadata_content_encoding = image_set_metadata.content_encoding
image_set_metadata_content_type = image_set_metadata.content_type
```

---


### Image_frame

ImageFrame resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `image_frame_blob` | String | <p>The blob containing the aggregated image frame information.</p> |
| `content_type` | String | <p>The format in which the image frame information is returned to the customer. Default is <code>application/octet-stream</code>.</p> <note> <ul> <li> <p>If the stored transfer syntax is <code>1.2.840.10008.1.2.1</code>, the returned <code>contentType</code> is <code>application/octet-stream</code>.</p> </li> </ul> <ul> <li> <p>If the stored transfer syntax is <code>1.2.840.10008.1.2.4.50</code>, the returned <code>contentType</code> is <code>image/jpeg</code>.</p> </li> </ul> <ul> <li> <p>If the stored transfer syntax is <code>1.2.840.10008.1.2.4.91</code>, the returned <code>contentType</code> is <code>image/j2c</code>.</p> </li> </ul> <ul> <li> <p>If the stored transfer syntax is MPEG2, <code>1.2.840.10008.1.2.4.100</code>, <code>1.2.840.10008.1.2.4.100.1</code>, <code>1.2.840.10008.1.2.4.101</code>, or <code>1.2.840.10008.1.2.4.101.1</code>, the returned <code>contentType</code> is <code>video/mpeg</code>.</p> </li> </ul> <ul> <li> <p>If the stored transfer syntax is MPEG-4 AVC/H.264, UID <code>1.2.840.10008.1.2.4.102</code>, <code>1.2.840.10008.1.2.4.102.1</code>, <code>1.2.840.10008.1.2.4.103</code>, <code>1.2.840.10008.1.2.4.103.1</code>, <code>1.2.840.10008.1.2.4.104</code>, <code>1.2.840.10008.1.2.4.104.1</code>, <code>1.2.840.10008.1.2.4.105</code>, <code>1.2.840.10008.1.2.4.105.1</code>, <code>1.2.840.10008.1.2.4.106</code>, or <code>1.2.840.10008.1.2.4.106.1</code>, the returned <code>contentType</code> is <code>video/mp4</code>.</p> </li> </ul> <ul> <li> <p>If the stored transfer syntax is HEVC/H.265, UID <code>1.2.840.10008.1.2.4.107</code> or <code>1.2.840.10008.1.2.4.108</code>, the returned <code>contentType</code> is <code>video/H256</code>.</p> </li> </ul> <ul> <li> <p>If the stored transfer syntax is <code>1.2.840.10008.1.2.4.202</code> or if the stored transfer syntax is <i>missing</i>, the returned <code>contentType</code> is <code>image/jph</code>.</p> </li> </ul> <ul> <li> <p>If the stored transfer syntax is <code>1.2.840.10008.1.2.4.203</code>, the returned contentType is <code>image/jphc</code>.</p> </li> </ul> </note> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access image_frame outputs
image_frame_id = image_frame.id
image_frame_image_frame_blob = image_frame.image_frame_blob
image_frame_content_type = image_frame.content_type
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple dicom_import_job resources
dicom_import_job_0 = provider.medical_imaging.Dicom_import_job {
}
dicom_import_job_1 = provider.medical_imaging.Dicom_import_job {
}
dicom_import_job_2 = provider.medical_imaging.Dicom_import_job {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    dicom_import_job = provider.medical_imaging.Dicom_import_job {
    }
```

---

## Related Documentation

- [AWS Medical_imaging Documentation](https://docs.aws.amazon.com/medical_imaging/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
