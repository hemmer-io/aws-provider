# Mailmanager Service



**Resources**: 7

---

## Overview

The mailmanager service provides access to 7 resource types:

- [Archive_message_content](#archive_message_content) [R]
- [Address_list_import_job](#address_list_import_job) [CR]
- [Archive_search_results](#archive_search_results) [R]
- [Archive_export](#archive_export) [R]
- [Archive_message](#archive_message) [R]
- [Member_of_address_list](#member_of_address_list) [R]
- [Archive_search](#archive_search) [R]

---

## Resources


### Archive_message_content

ArchiveMessageContent resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `body` | String | <p>The textual body content of the email message.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access archive_message_content outputs
archive_message_content_id = archive_message_content.id
archive_message_content_body = archive_message_content.body
```

---


### Address_list_import_job

AddressListImportJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A unique token that Amazon SES uses to recognize subsequent retries of the same request.</p> |
| `import_data_format` | String | ✅ | <p>The format of the input for an import job.</p> |
| `name` | String | ✅ | <p>A user-friendly name for the import job.</p> |
| `address_list_id` | String | ✅ | <p>The unique identifier of the address list for importing addresses to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `import_data_format` | String | <p>The format of the input for an import job.</p> |
| `completed_timestamp` | String | <p>The timestamp of when the import job was completed.</p> |
| `error` | String | <p>The reason for failure of an import job.</p> |
| `status` | String | <p>The status of the import job.</p> |
| `pre_signed_url` | String | <p>The pre-signed URL target for uploading the input file.</p> |
| `imported_items_count` | i64 | <p>The number of input addresses successfully imported into the address list.</p> |
| `address_list_id` | String | <p>The unique identifier of the address list the import job was created for.</p> |
| `start_timestamp` | String | <p>The timestamp of when the import job was started.</p> |
| `name` | String | <p>A user-friendly name for the import job.</p> |
| `job_id` | String | <p>The identifier of the import job.</p> |
| `failed_items_count` | i64 | <p>The number of input addresses that failed to be imported into the address list.</p> |
| `created_timestamp` | String | <p>The timestamp of when the import job was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create address_list_import_job
address_list_import_job = provider.mailmanager.Address_list_import_job {
    import_data_format = "value"  # <p>The format of the input for an import job.</p>
    name = "value"  # <p>A user-friendly name for the import job.</p>
    address_list_id = "value"  # <p>The unique identifier of the address list for importing addresses to.</p>
}

# Access address_list_import_job outputs
address_list_import_job_id = address_list_import_job.id
address_list_import_job_import_data_format = address_list_import_job.import_data_format
address_list_import_job_completed_timestamp = address_list_import_job.completed_timestamp
address_list_import_job_error = address_list_import_job.error
address_list_import_job_status = address_list_import_job.status
address_list_import_job_pre_signed_url = address_list_import_job.pre_signed_url
address_list_import_job_imported_items_count = address_list_import_job.imported_items_count
address_list_import_job_address_list_id = address_list_import_job.address_list_id
address_list_import_job_start_timestamp = address_list_import_job.start_timestamp
address_list_import_job_name = address_list_import_job.name
address_list_import_job_job_id = address_list_import_job.job_id
address_list_import_job_failed_items_count = address_list_import_job.failed_items_count
address_list_import_job_created_timestamp = address_list_import_job.created_timestamp
```

---


### Archive_search_results

ArchiveSearchResults resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rows` | Vec<String> | <p>The list of email result objects matching the search criteria.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access archive_search_results outputs
archive_search_results_id = archive_search_results.id
archive_search_results_rows = archive_search_results.rows
```

---


### Archive_export

ArchiveExport resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `max_results` | i64 | <p>The maximum number of email items included in the export.</p> |
| `export_destination_configuration` | String | <p>Where the exported emails are being delivered.</p> |
| `archive_id` | String | <p>The identifier of the archive the email export was performed from.</p> |
| `from_timestamp` | String | <p>The start of the timestamp range the exported emails cover.</p> |
| `to_timestamp` | String | <p>The end of the date range the exported emails cover.</p> |
| `filters` | String | <p>The criteria used to filter emails included in the export.</p> |
| `status` | String | <p>The current status of the export job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access archive_export outputs
archive_export_id = archive_export.id
archive_export_max_results = archive_export.max_results
archive_export_export_destination_configuration = archive_export.export_destination_configuration
archive_export_archive_id = archive_export.archive_id
archive_export_from_timestamp = archive_export.from_timestamp
archive_export_to_timestamp = archive_export.to_timestamp
archive_export_filters = archive_export.filters
archive_export_status = archive_export.status
```

---


### Archive_message

ArchiveMessage resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | String | <p>The metadata about the email.</p> |
| `envelope` | String | <p>The SMTP envelope information of the email.</p> |
| `message_download_link` | String | <p>A pre-signed URL to temporarily download the full message content.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access archive_message outputs
archive_message_id = archive_message.id
archive_message_metadata = archive_message.metadata
archive_message_envelope = archive_message.envelope
archive_message_message_download_link = archive_message.message_download_link
```

---


### Member_of_address_list

MemberOfAddressList resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `address` | String | <p>The address retrieved from the address list.</p> |
| `created_timestamp` | String | <p>The timestamp of when the address was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access member_of_address_list outputs
member_of_address_list_id = member_of_address_list.id
member_of_address_list_address = member_of_address_list.address
member_of_address_list_created_timestamp = member_of_address_list.created_timestamp
```

---


### Archive_search

ArchiveSearch resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `from_timestamp` | String | <p>The start timestamp of the range the searched emails cover.</p> |
| `to_timestamp` | String | <p>The end timestamp of the range the searched emails cover.</p> |
| `status` | String | <p>The current status of the search job.</p> |
| `filters` | String | <p>The criteria used to filter emails included in the search.</p> |
| `archive_id` | String | <p>The identifier of the archive the email search was performed in.</p> |
| `max_results` | i64 | <p>The maximum number of search results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access archive_search outputs
archive_search_id = archive_search.id
archive_search_from_timestamp = archive_search.from_timestamp
archive_search_to_timestamp = archive_search.to_timestamp
archive_search_status = archive_search.status
archive_search_filters = archive_search.filters
archive_search_archive_id = archive_search.archive_id
archive_search_max_results = archive_search.max_results
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple archive_message_content resources
archive_message_content_0 = provider.mailmanager.Archive_message_content {
}
archive_message_content_1 = provider.mailmanager.Archive_message_content {
}
archive_message_content_2 = provider.mailmanager.Archive_message_content {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    archive_message_content = provider.mailmanager.Archive_message_content {
    }
```

---

## Related Documentation

- [AWS Mailmanager Documentation](https://docs.aws.amazon.com/mailmanager/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
