# Mediapackage Service



**Resources**: 6

---

## Overview

The mediapackage service provides access to 6 resource types:

- [Origin_endpoint](#origin_endpoint) [CRUD]
- [Channel](#channel) [CRUD]
- [Harvest_job](#harvest_job) [CR]
- [Packaging_configuration](#packaging_configuration) [CRD]
- [Packaging_group](#packaging_group) [CRUD]
- [Asset](#asset) [CRD]

---

## Resources


### Origin_endpoint

OriginEndpoint resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dash_package` | String |  |  |
| `channel_id` | String | ✅ | The ID of the Channel that the OriginEndpoint will be associated with.
This cannot be changed after the OriginEndpoint is created. |
| `manifest_name` | String |  | A short string that will be used as the filename of the OriginEndpoint URL (defaults to "index"). |
| `tags` | HashMap<String, String> |  |  |
| `whitelist` | Vec<String> |  | A list of source IP CIDR blocks that will be allowed to access the OriginEndpoint. |
| `startover_window_seconds` | i64 |  | Maximum duration (seconds) of content to retain for startover playback.
If not specified, startover playback will be disabled for the OriginEndpoint. |
| `description` | String |  | A short text description of the OriginEndpoint. |
| `mss_package` | String |  |  |
| `time_delay_seconds` | i64 |  | Amount of delay (seconds) to enforce on the playback of live content.
If not specified, there will be no time delay in effect for the OriginEndpoint. |
| `origination` | String |  | Control whether origination of video is allowed for this OriginEndpoint. If set to ALLOW, the OriginEndpoint
may by requested, pursuant to any other form of access control. If set to DENY, the OriginEndpoint may not be
requested. This can be helpful for Live to VOD harvesting, or for temporarily disabling origination |
| `hls_package` | String |  |  |
| `cmaf_package` | String |  |  |
| `id` | String | ✅ | The ID of the OriginEndpoint.  The ID must be unique within the region
and it cannot be changed after the OriginEndpoint is created. |
| `authorization` | String |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | The Amazon Resource Name (ARN) assigned to the OriginEndpoint. |
| `manifest_name` | String | A short string appended to the end of the OriginEndpoint URL. |
| `dash_package` | String |  |
| `authorization` | String |  |
| `mss_package` | String |  |
| `cmaf_package` | String |  |
| `whitelist` | Vec<String> | A list of source IP CIDR blocks that will be allowed to access the OriginEndpoint. |
| `channel_id` | String | The ID of the Channel the OriginEndpoint is associated with. |
| `created_at` | String | The date and time the OriginEndpoint was created. |
| `tags` | HashMap<String, String> |  |
| `time_delay_seconds` | i64 | Amount of delay (seconds) to enforce on the playback of live content.
If not specified, there will be no time delay in effect for the OriginEndpoint. |
| `hls_package` | String |  |
| `origination` | String | Control whether origination of video is allowed for this OriginEndpoint. If set to ALLOW, the OriginEndpoint
may by requested, pursuant to any other form of access control. If set to DENY, the OriginEndpoint may not be
requested. This can be helpful for Live to VOD harvesting, or for temporarily disabling origination |
| `description` | String | A short text description of the OriginEndpoint. |
| `id` | String | The ID of the OriginEndpoint. |
| `url` | String | The URL of the packaged OriginEndpoint for consumption. |
| `startover_window_seconds` | i64 | Maximum duration (seconds) of content to retain for startover playback.
If not specified, startover playback will be disabled for the OriginEndpoint. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create origin_endpoint
origin_endpoint = provider.mediapackage.Origin_endpoint {
    channel_id = "value"  # The ID of the Channel that the OriginEndpoint will be associated with.
This cannot be changed after the OriginEndpoint is created.
    id = "value"  # The ID of the OriginEndpoint.  The ID must be unique within the region
and it cannot be changed after the OriginEndpoint is created.
}

# Access origin_endpoint outputs
origin_endpoint_id = origin_endpoint.id
origin_endpoint_arn = origin_endpoint.arn
origin_endpoint_manifest_name = origin_endpoint.manifest_name
origin_endpoint_dash_package = origin_endpoint.dash_package
origin_endpoint_authorization = origin_endpoint.authorization
origin_endpoint_mss_package = origin_endpoint.mss_package
origin_endpoint_cmaf_package = origin_endpoint.cmaf_package
origin_endpoint_whitelist = origin_endpoint.whitelist
origin_endpoint_channel_id = origin_endpoint.channel_id
origin_endpoint_created_at = origin_endpoint.created_at
origin_endpoint_tags = origin_endpoint.tags
origin_endpoint_time_delay_seconds = origin_endpoint.time_delay_seconds
origin_endpoint_hls_package = origin_endpoint.hls_package
origin_endpoint_origination = origin_endpoint.origination
origin_endpoint_description = origin_endpoint.description
origin_endpoint_id = origin_endpoint.id
origin_endpoint_url = origin_endpoint.url
origin_endpoint_startover_window_seconds = origin_endpoint.startover_window_seconds
```

---


### Channel

Channel resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  |  |
| `description` | String |  | A short text description of the Channel. |
| `id` | String | ✅ | The ID of the Channel. The ID must be unique within the region and it
cannot be changed after a Channel is created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ingress_access_logs` | String |  |
| `hls_ingest` | String |  |
| `tags` | HashMap<String, String> |  |
| `created_at` | String | The date and time the Channel was created. |
| `arn` | String | The Amazon Resource Name (ARN) assigned to the Channel. |
| `description` | String | A short text description of the Channel. |
| `egress_access_logs` | String |  |
| `id` | String | The ID of the Channel. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create channel
channel = provider.mediapackage.Channel {
    id = "value"  # The ID of the Channel. The ID must be unique within the region and it
cannot be changed after a Channel is created.
}

# Access channel outputs
channel_id = channel.id
channel_ingress_access_logs = channel.ingress_access_logs
channel_hls_ingest = channel.hls_ingest
channel_tags = channel.tags
channel_created_at = channel.created_at
channel_arn = channel.arn
channel_description = channel.description
channel_egress_access_logs = channel.egress_access_logs
channel_id = channel.id
```

---


### Harvest_job

HarvestJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `start_time` | String | ✅ | The start of the time-window which will be harvested |
| `end_time` | String | ✅ | The end of the time-window which will be harvested |
| `id` | String | ✅ | The ID of the HarvestJob. The ID must be unique within the region
and it cannot be changed after the HarvestJob is submitted |
| `origin_endpoint_id` | String | ✅ | The ID of the OriginEndpoint that the HarvestJob will harvest from.
This cannot be changed after the HarvestJob is submitted. |
| `s3_destination` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | The end of the time-window which will be harvested. |
| `id` | String | The ID of the HarvestJob. The ID must be unique within the region
and it cannot be changed after the HarvestJob is submitted. |
| `channel_id` | String | The ID of the Channel that the HarvestJob will harvest from. |
| `s3_destination` | String |  |
| `origin_endpoint_id` | String | The ID of the OriginEndpoint that the HarvestJob will harvest from.
This cannot be changed after the HarvestJob is submitted. |
| `arn` | String | The Amazon Resource Name (ARN) assigned to the HarvestJob. |
| `created_at` | String | The date and time the HarvestJob was submitted. |
| `start_time` | String | The start of the time-window which will be harvested. |
| `status` | String | The current status of the HarvestJob. Consider setting up a CloudWatch Event to listen for
HarvestJobs as they succeed or fail. In the event of failure, the CloudWatch Event will
include an explanation of why the HarvestJob failed. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create harvest_job
harvest_job = provider.mediapackage.Harvest_job {
    start_time = "value"  # The start of the time-window which will be harvested
    end_time = "value"  # The end of the time-window which will be harvested
    id = "value"  # The ID of the HarvestJob. The ID must be unique within the region
and it cannot be changed after the HarvestJob is submitted
    origin_endpoint_id = "value"  # The ID of the OriginEndpoint that the HarvestJob will harvest from.
This cannot be changed after the HarvestJob is submitted.
    s3_destination = "value"  # Required field
}

# Access harvest_job outputs
harvest_job_id = harvest_job.id
harvest_job_end_time = harvest_job.end_time
harvest_job_id = harvest_job.id
harvest_job_channel_id = harvest_job.channel_id
harvest_job_s3_destination = harvest_job.s3_destination
harvest_job_origin_endpoint_id = harvest_job.origin_endpoint_id
harvest_job_arn = harvest_job.arn
harvest_job_created_at = harvest_job.created_at
harvest_job_start_time = harvest_job.start_time
harvest_job_status = harvest_job.status
```

---


### Packaging_configuration

PackagingConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ | The ID of the PackagingConfiguration. |
| `hls_package` | String |  |  |
| `mss_package` | String |  |  |
| `tags` | HashMap<String, String> |  |  |
| `packaging_group_id` | String | ✅ | The ID of a PackagingGroup. |
| `cmaf_package` | String |  |  |
| `dash_package` | String |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `packaging_group_id` | String | The ID of a PackagingGroup. |
| `created_at` | String | The time the PackagingConfiguration was created. |
| `mss_package` | String |  |
| `dash_package` | String |  |
| `cmaf_package` | String |  |
| `arn` | String | The ARN of the PackagingConfiguration. |
| `id` | String | The ID of the PackagingConfiguration. |
| `tags` | HashMap<String, String> |  |
| `hls_package` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create packaging_configuration
packaging_configuration = provider.mediapackage.Packaging_configuration {
    id = "value"  # The ID of the PackagingConfiguration.
    packaging_group_id = "value"  # The ID of a PackagingGroup.
}

# Access packaging_configuration outputs
packaging_configuration_id = packaging_configuration.id
packaging_configuration_packaging_group_id = packaging_configuration.packaging_group_id
packaging_configuration_created_at = packaging_configuration.created_at
packaging_configuration_mss_package = packaging_configuration.mss_package
packaging_configuration_dash_package = packaging_configuration.dash_package
packaging_configuration_cmaf_package = packaging_configuration.cmaf_package
packaging_configuration_arn = packaging_configuration.arn
packaging_configuration_id = packaging_configuration.id
packaging_configuration_tags = packaging_configuration.tags
packaging_configuration_hls_package = packaging_configuration.hls_package
```

---


### Packaging_group

PackagingGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ | The ID of the PackagingGroup. |
| `tags` | HashMap<String, String> |  |  |
| `authorization` | String |  |  |
| `egress_access_logs` | String |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The ID of the PackagingGroup. |
| `arn` | String | The ARN of the PackagingGroup. |
| `egress_access_logs` | String |  |
| `domain_name` | String | The fully qualified domain name for Assets in the PackagingGroup. |
| `created_at` | String | The time the PackagingGroup was created. |
| `approximate_asset_count` | i64 | The approximate asset count of the PackagingGroup. |
| `authorization` | String |  |
| `tags` | HashMap<String, String> |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create packaging_group
packaging_group = provider.mediapackage.Packaging_group {
    id = "value"  # The ID of the PackagingGroup.
}

# Access packaging_group outputs
packaging_group_id = packaging_group.id
packaging_group_id = packaging_group.id
packaging_group_arn = packaging_group.arn
packaging_group_egress_access_logs = packaging_group.egress_access_logs
packaging_group_domain_name = packaging_group.domain_name
packaging_group_created_at = packaging_group.created_at
packaging_group_approximate_asset_count = packaging_group.approximate_asset_count
packaging_group_authorization = packaging_group.authorization
packaging_group_tags = packaging_group.tags
```

---


### Asset

Asset resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ | The unique identifier for the Asset. |
| `packaging_group_id` | String | ✅ | The ID of the PackagingGroup for the Asset. |
| `resource_id` | String |  | The resource ID to include in SPEKE key requests. |
| `tags` | HashMap<String, String> |  |  |
| `source_arn` | String | ✅ | ARN of the source object in S3. |
| `source_role_arn` | String | ✅ | The IAM role ARN used to access the source S3 bucket. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | The ARN of the Asset. |
| `packaging_group_id` | String | The ID of the PackagingGroup for the Asset. |
| `egress_endpoints` | Vec<String> | The list of egress endpoints available for the Asset. |
| `id` | String | The unique identifier for the Asset. |
| `source_arn` | String | ARN of the source object in S3. |
| `tags` | HashMap<String, String> |  |
| `source_role_arn` | String | The IAM role_arn used to access the source S3 bucket. |
| `created_at` | String | The time the Asset was initially submitted for Ingest. |
| `resource_id` | String | The resource ID to include in SPEKE key requests. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create asset
asset = provider.mediapackage.Asset {
    id = "value"  # The unique identifier for the Asset.
    packaging_group_id = "value"  # The ID of the PackagingGroup for the Asset.
    source_arn = "value"  # ARN of the source object in S3.
    source_role_arn = "value"  # The IAM role ARN used to access the source S3 bucket.
}

# Access asset outputs
asset_id = asset.id
asset_arn = asset.arn
asset_packaging_group_id = asset.packaging_group_id
asset_egress_endpoints = asset.egress_endpoints
asset_id = asset.id
asset_source_arn = asset.source_arn
asset_tags = asset.tags
asset_source_role_arn = asset.source_role_arn
asset_created_at = asset.created_at
asset_resource_id = asset.resource_id
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple origin_endpoint resources
origin_endpoint_0 = provider.mediapackage.Origin_endpoint {
    channel_id = "value-0"
    id = "value-0"
}
origin_endpoint_1 = provider.mediapackage.Origin_endpoint {
    channel_id = "value-1"
    id = "value-1"
}
origin_endpoint_2 = provider.mediapackage.Origin_endpoint {
    channel_id = "value-2"
    id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    origin_endpoint = provider.mediapackage.Origin_endpoint {
        channel_id = "production-value"
        id = "production-value"
    }
```

---

## Related Documentation

- [AWS Mediapackage Documentation](https://docs.aws.amazon.com/mediapackage/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
