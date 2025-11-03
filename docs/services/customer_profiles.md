# Customer_profiles Service



**Resources**: 26

---

## Overview

The customer_profiles service provides access to 26 resource types:

- [Segment_snapshot](#segment_snapshot) [CR]
- [Upload_job](#upload_job) [CR]
- [Integration](#integration) [CRD]
- [Profile_object_type_template](#profile_object_type_template) [R]
- [Workflow_steps](#workflow_steps) [R]
- [Profile_object_type](#profile_object_type) [CRD]
- [Profile](#profile) [CUD]
- [Profile_object](#profile_object) [CD]
- [Event_trigger](#event_trigger) [CRUD]
- [Segment_membership](#segment_membership) [R]
- [Event_stream](#event_stream) [CRD]
- [Integration_workflow](#integration_workflow) [C]
- [Domain_layout](#domain_layout) [CRUD]
- [Upload_job_path](#upload_job_path) [R]
- [Segment_definition](#segment_definition) [CRD]
- [Calculated_attribute_for_profile](#calculated_attribute_for_profile) [R]
- [Domain](#domain) [CRUD]
- [Profile_history_record](#profile_history_record) [R]
- [Similar_profiles](#similar_profiles) [R]
- [Auto_merging_preview](#auto_merging_preview) [R]
- [Identity_resolution_job](#identity_resolution_job) [R]
- [Segment_estimate](#segment_estimate) [CR]
- [Matches](#matches) [R]
- [Profile_key](#profile_key) [D]
- [Calculated_attribute_definition](#calculated_attribute_definition) [CRUD]
- [Workflow](#workflow) [RD]

---

## Resources


### Segment_snapshot

SegmentSnapshot resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_name` | String | ✅ | <p>The unique name of the domain.</p> |
| `destination_uri` | String |  | <p>The destination to which the segment will be exported. This field must be provided if
         the request is not submitted from the Amazon Connect Admin Website.</p> |
| `segment_definition_name` | String | ✅ | <p>The name of the segment definition used in this snapshot request.</p> |
| `role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM role that allows Customer Profiles service
         principal to assume the role for conducting KMS and S3 operations.</p> |
| `data_format` | String | ✅ | <p>The format in which the segment will be exported.</p> |
| `encryption_key` | String |  | <p>The Amazon Resource Name (ARN) of the KMS key used to encrypt the exported
         segment.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status_message` | String | <p>The status message of the asynchronous job for exporting the segment snapshot.</p> |
| `snapshot_id` | String | <p>The unique identifier of the segment snapshot.</p> |
| `data_format` | String | <p>The format in which the segment will be exported.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role that allows Customer Profiles service
         principal to assume the role for conducting KMS and S3 operations.</p> |
| `encryption_key` | String | <p>The Amazon Resource Name (ARN) of the KMS key used to encrypt the exported
         segment.</p> |
| `destination_uri` | String | <p>The destination to which the segment will be exported. This field must be provided if
         the request is not submitted from the Amazon Connect Admin Website.</p> |
| `status` | String | <p>The status of the asynchronous job for exporting the segment snapshot.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create segment_snapshot
segment_snapshot = provider.customer_profiles.Segment_snapshot {
    domain_name = "value"  # <p>The unique name of the domain.</p>
    segment_definition_name = "value"  # <p>The name of the segment definition used in this snapshot request.</p>
    data_format = "value"  # <p>The format in which the segment will be exported.</p>
}

# Access segment_snapshot outputs
segment_snapshot_id = segment_snapshot.id
segment_snapshot_status_message = segment_snapshot.status_message
segment_snapshot_snapshot_id = segment_snapshot.snapshot_id
segment_snapshot_data_format = segment_snapshot.data_format
segment_snapshot_role_arn = segment_snapshot.role_arn
segment_snapshot_encryption_key = segment_snapshot.encryption_key
segment_snapshot_destination_uri = segment_snapshot.destination_uri
segment_snapshot_status = segment_snapshot.status
```

---


### Upload_job

UploadJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fields` | HashMap<String, String> | ✅ | <p>The mapping between CSV Columns and Profile Object attributes. A map of the name and
         ObjectType field.</p> |
| `domain_name` | String | ✅ | <p>The unique name of the domain. Domain should be exists for the upload job to be created.
      </p> |
| `display_name` | String | ✅ | <p>The unique name of the upload job. Could be a file name to identify the upload
         job.</p> |
| `unique_key` | String | ✅ | <p>The unique key columns for de-duping the profiles used to map data to the profile.
      </p> |
| `data_expiry` | i64 |  | <p>The expiry duration for the profiles ingested with the job. If not provided, the system
         default of 2 weeks is used. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status_reason` | String | <p>The reason for the current status of the upload job. Possible reasons: </p>
         <ul>
            <li>
               <p>
                  <b>VALIDATION_FAILURE</b>: The upload job has
               encountered an error or issue and was unable to complete the profile data ingestion.
            </p>
            </li>
            <li>
               <p>
                  <b>INTERNAL_FAILURE</b>: Failure caused from service
               side </p>
            </li>
         </ul> |
| `unique_key` | String | <p>The unique key columns used for de-duping the keys in the upload job. </p> |
| `data_expiry` | i64 | <p>The expiry duration for the profiles ingested with the upload job. </p> |
| `created_at` | String | <p>The timestamp when the upload job was created. </p> |
| `completed_at` | String | <p>The timestamp when the upload job was completed. </p> |
| `status` | String | <p>The status describing the status for the upload job. The following are Valid Values: </p>
         <ul>
            <li>
               <p>
                  <b>CREATED</b>: The upload job has been created, but has
               not started processing yet. </p>
            </li>
            <li>
               <p>
                  <b>IN_PROGRESS</b>: The upload job is currently in
               progress, ingesting and processing the profile data. </p>
            </li>
            <li>
               <p>
                  <b>PARTIALLY_SUCCEEDED</b>: The upload job has
               successfully completed the ingestion and processing of all profile data. </p>
            </li>
            <li>
               <p>
                  <b>SUCCEEDED</b>: The upload job has successfully
               completed the ingestion and processing of all profile data. </p>
            </li>
            <li>
               <p>
                  <b>FAILED</b>: The upload job has failed to complete.
            </p>
            </li>
            <li>
               <p>
                  <b>STOPPED</b>: The upload job has been manually stopped
               or terminated before completion. </p>
            </li>
         </ul> |
| `results_summary` | String | <p>The summary of results for the upload job, including the number of updated, created, and
         failed records. </p> |
| `job_id` | String | <p>The unique identifier of the upload job. </p> |
| `display_name` | String | <p>The unique name of the upload job. Could be a file name to identify the upload job.
      </p> |
| `fields` | HashMap<String, String> | <p>The mapping between CSV Columns and Profile Object attributes for the upload job.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create upload_job
upload_job = provider.customer_profiles.Upload_job {
    fields = "value"  # <p>The mapping between CSV Columns and Profile Object attributes. A map of the name and
         ObjectType field.</p>
    domain_name = "value"  # <p>The unique name of the domain. Domain should be exists for the upload job to be created.
      </p>
    display_name = "value"  # <p>The unique name of the upload job. Could be a file name to identify the upload
         job.</p>
    unique_key = "value"  # <p>The unique key columns for de-duping the profiles used to map data to the profile.
      </p>
}

# Access upload_job outputs
upload_job_id = upload_job.id
upload_job_status_reason = upload_job.status_reason
upload_job_unique_key = upload_job.unique_key
upload_job_data_expiry = upload_job.data_expiry
upload_job_created_at = upload_job.created_at
upload_job_completed_at = upload_job.completed_at
upload_job_status = upload_job.status
upload_job_results_summary = upload_job.results_summary
upload_job_job_id = upload_job.job_id
upload_job_display_name = upload_job.display_name
upload_job_fields = upload_job.fields
```

---


### Integration

Integration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uri` | String |  | <p>The URI of the S3 bucket or any other type of data source.</p> |
| `flow_definition` | String |  | <p>The configuration that controls how Customer Profiles retrieves data from the
         source.</p> |
| `role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM role. The Integration uses this role to make
         Customer Profiles requests on your behalf.</p> |
| `domain_name` | String | ✅ | <p>The unique name of the domain.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource.</p> |
| `event_trigger_names` | Vec<String> |  | <p>A list of unique names for active event triggers associated with the integration.</p> |
| `object_type_names` | HashMap<String, String> |  | <p>A map in which each key is an event type from an external application such as Segment or Shopify, and each value is an <code>ObjectTypeName</code> (template) used to ingest the event.
It supports the following event types: <code>SegmentIdentify</code>, <code>ShopifyCreateCustomers</code>, <code>ShopifyUpdateCustomers</code>, <code>ShopifyCreateDraftOrders</code>, 
<code>ShopifyUpdateDraftOrders</code>, <code>ShopifyCreateOrders</code>, and <code>ShopifyUpdatedOrders</code>.</p> |
| `object_type_name` | String |  | <p>The name of the profile object type.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_at` | String | <p>The timestamp of when the domain was created.</p> |
| `workflow_id` | String | <p>Unique identifier for the workflow.</p> |
| `is_unstructured` | bool | <p>Boolean that shows if the Flow that's associated with the Integration is created in
         Amazon Appflow, or with ObjectTypeName equals _unstructured via API/CLI in
         flowDefinition.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role. The Integration uses this role to make
         Customer Profiles requests on your behalf.</p> |
| `domain_name` | String | <p>The unique name of the domain.</p> |
| `uri` | String | <p>The URI of the S3 bucket or any other type of data source.</p> |
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource.</p> |
| `object_type_names` | HashMap<String, String> | <p>A map in which each key is an event type from an external application such as Segment or Shopify, and each value is an <code>ObjectTypeName</code> (template) used to ingest the event.
It supports the following event types: <code>SegmentIdentify</code>, <code>ShopifyCreateCustomers</code>, <code>ShopifyUpdateCustomers</code>, <code>ShopifyCreateDraftOrders</code>, 
<code>ShopifyUpdateDraftOrders</code>, <code>ShopifyCreateOrders</code>, and <code>ShopifyUpdatedOrders</code>.</p> |
| `event_trigger_names` | Vec<String> | <p>A list of unique names for active event triggers associated with the integration. This
         list would be empty if no Event Trigger is associated with the integration.</p> |
| `last_updated_at` | String | <p>The timestamp of when the domain was most recently edited.</p> |
| `object_type_name` | String | <p>The name of the profile object type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create integration
integration = provider.customer_profiles.Integration {
    domain_name = "value"  # <p>The unique name of the domain.</p>
}

# Access integration outputs
integration_id = integration.id
integration_created_at = integration.created_at
integration_workflow_id = integration.workflow_id
integration_is_unstructured = integration.is_unstructured
integration_role_arn = integration.role_arn
integration_domain_name = integration.domain_name
integration_uri = integration.uri
integration_tags = integration.tags
integration_object_type_names = integration.object_type_names
integration_event_trigger_names = integration.event_trigger_names
integration_last_updated_at = integration.last_updated_at
integration_object_type_name = integration.object_type_name
```

---


### Profile_object_type_template

ProfileObjectTypeTemplate resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `template_id` | String | <p>A unique identifier for the object template.</p> |
| `allow_profile_creation` | bool | <p>Indicates whether a profile should be created when data is received if one doesn’t exist
         for an object of this type. The default is <code>FALSE</code>. If the AllowProfileCreation
         flag is set to <code>FALSE</code>, then the service tries to fetch a standard profile and
         associate this object with the profile. If it is set to <code>TRUE</code>, and if no match
         is found, then the service creates a new standard profile.</p> |
| `source_name` | String | <p>The name of the source of the object template.</p> |
| `fields` | HashMap<String, String> | <p>A map of the name and ObjectType field.</p> |
| `source_object` | String | <p>The source of the object template.</p> |
| `keys` | HashMap<String, Vec<String>> | <p>A list of unique keys that can be used to map data to the profile.</p> |
| `source_last_updated_timestamp_format` | String | <p>The format of your <code>sourceLastUpdatedTimestamp</code> that was previously set
         up.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access profile_object_type_template outputs
profile_object_type_template_id = profile_object_type_template.id
profile_object_type_template_template_id = profile_object_type_template.template_id
profile_object_type_template_allow_profile_creation = profile_object_type_template.allow_profile_creation
profile_object_type_template_source_name = profile_object_type_template.source_name
profile_object_type_template_fields = profile_object_type_template.fields
profile_object_type_template_source_object = profile_object_type_template.source_object
profile_object_type_template_keys = profile_object_type_template.keys
profile_object_type_template_source_last_updated_timestamp_format = profile_object_type_template.source_last_updated_timestamp_format
```

---


### Workflow_steps

WorkflowSteps resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | <p>List containing workflow step details.</p> |
| `workflow_type` | String | <p>The type of workflow. The only supported value is APPFLOW_INTEGRATION.</p> |
| `workflow_id` | String | <p>Unique identifier for the workflow.</p> |
| `next_token` | String | <p>If there are additional results, this is the token for the next set of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workflow_steps outputs
workflow_steps_id = workflow_steps.id
workflow_steps_items = workflow_steps.items
workflow_steps_workflow_type = workflow_steps.workflow_type
workflow_steps_workflow_id = workflow_steps.workflow_id
workflow_steps_next_token = workflow_steps.next_token
```

---


### Profile_object_type

ProfileObjectType resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encryption_key` | String |  | <p>The customer-provided key to encrypt the profile object that will be created in this
         profile object type.</p> |
| `allow_profile_creation` | bool |  | <p>Indicates whether a profile should be created when data is received if one doesn’t exist
         for an object of this type. The default is <code>FALSE</code>. If the AllowProfileCreation
         flag is set to <code>FALSE</code>, then the service tries to fetch a standard profile and
         associate this object with the profile. If it is set to <code>TRUE</code>, and if no match
         is found, then the service creates a new standard profile.</p> |
| `fields` | HashMap<String, String> |  | <p>A map of the name and ObjectType field.</p> |
| `keys` | HashMap<String, Vec<String>> |  | <p>A list of unique keys that can be used to map data to the profile.</p> |
| `description` | String | ✅ | <p>Description of the profile object type.</p> |
| `source_last_updated_timestamp_format` | String |  | <p>The format of your <code>sourceLastUpdatedTimestamp</code> that was previously set up.
      </p> |
| `max_profile_object_count` | i64 |  | <p>The amount of profile object max count assigned to the object type</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource.</p> |
| `template_id` | String |  | <p>A unique identifier for the object template. For some attributes in the request, the
         service will use the default value from the object template when TemplateId is present. If
         these attributes are present in the request, the service may return a
            <code>BadRequestException</code>. These attributes include: AllowProfileCreation,
         SourceLastUpdatedTimestampFormat, Fields, and Keys. For example, if AllowProfileCreation is
         set to true when TemplateId is set, the service may return a
            <code>BadRequestException</code>.</p> |
| `expiration_days` | i64 |  | <p>The number of days until the data in the object expires.</p> |
| `domain_name` | String | ✅ | <p>The unique name of the domain.</p> |
| `object_type_name` | String | ✅ | <p>The name of the profile object type.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_updated_at` | String | <p>The timestamp of when the domain was most recently edited.</p> |
| `max_profile_object_count` | i64 | <p>The amount of profile object max count assigned to the object type.</p> |
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource.</p> |
| `created_at` | String | <p>The timestamp of when the domain was created.</p> |
| `object_type_name` | String | <p>The name of the profile object type.</p> |
| `description` | String | <p>The description of the profile object type.</p> |
| `expiration_days` | i64 | <p>The number of days until the data in the object expires.</p> |
| `allow_profile_creation` | bool | <p>Indicates whether a profile should be created when data is received if one doesn’t exist
         for an object of this type. The default is <code>FALSE</code>. If the AllowProfileCreation
         flag is set to <code>FALSE</code>, then the service tries to fetch a standard profile and
         associate this object with the profile. If it is set to <code>TRUE</code>, and if no match
         is found, then the service creates a new standard profile.</p> |
| `encryption_key` | String | <p>The customer-provided key to encrypt the profile object that will be created in this
         profile object type.</p> |
| `max_available_profile_object_count` | i64 | <p>The amount of provisioned profile object max count available.</p> |
| `template_id` | String | <p>A unique identifier for the object template.</p> |
| `fields` | HashMap<String, String> | <p>A map of the name and ObjectType field.</p> |
| `keys` | HashMap<String, Vec<String>> | <p>A list of unique keys that can be used to map data to the profile.</p> |
| `source_last_updated_timestamp_format` | String | <p>The format of your <code>sourceLastUpdatedTimestamp</code> that was previously set
         up.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create profile_object_type
profile_object_type = provider.customer_profiles.Profile_object_type {
    description = "value"  # <p>Description of the profile object type.</p>
    domain_name = "value"  # <p>The unique name of the domain.</p>
    object_type_name = "value"  # <p>The name of the profile object type.</p>
}

# Access profile_object_type outputs
profile_object_type_id = profile_object_type.id
profile_object_type_last_updated_at = profile_object_type.last_updated_at
profile_object_type_max_profile_object_count = profile_object_type.max_profile_object_count
profile_object_type_tags = profile_object_type.tags
profile_object_type_created_at = profile_object_type.created_at
profile_object_type_object_type_name = profile_object_type.object_type_name
profile_object_type_description = profile_object_type.description
profile_object_type_expiration_days = profile_object_type.expiration_days
profile_object_type_allow_profile_creation = profile_object_type.allow_profile_creation
profile_object_type_encryption_key = profile_object_type.encryption_key
profile_object_type_max_available_profile_object_count = profile_object_type.max_available_profile_object_count
profile_object_type_template_id = profile_object_type.template_id
profile_object_type_fields = profile_object_type.fields
profile_object_type_keys = profile_object_type.keys
profile_object_type_source_last_updated_timestamp_format = profile_object_type.source_last_updated_timestamp_format
```

---


### Profile

Profile resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_name` | String | ✅ | <p>The unique name of the domain.</p> |
| `additional_information` | String |  | <p>Any additional information relevant to the customer’s profile.</p> |
| `middle_name` | String |  | <p>The customer’s middle name.</p> |
| `business_name` | String |  | <p>The name of the customer’s business.</p> |
| `shipping_address` | String |  | <p>The customer’s shipping address.</p> |
| `party_type_string` | String |  | <p>An alternative to <code>PartyType</code> which accepts any string as input.</p> |
| `billing_address` | String |  | <p>The customer’s billing address.</p> |
| `personal_email_address` | String |  | <p>The customer’s personal email address.</p> |
| `mobile_phone_number` | String |  | <p>The customer’s mobile phone number.</p> |
| `phone_number` | String |  | <p>The customer’s phone number, which has not been specified as a mobile, home, or business
         number. </p> |
| `gender_string` | String |  | <p>An alternative to <code>Gender</code> which accepts any string as input.</p> |
| `attributes` | HashMap<String, String> |  | <p>A key value pair of attributes of a customer profile.</p> |
| `party_type` | String |  | <p>The type of profile used to describe the customer.</p> |
| `home_phone_number` | String |  | <p>The customer’s home phone number.</p> |
| `business_email_address` | String |  | <p>The customer’s business email address.</p> |
| `engagement_preferences` | String |  | <p>Object that defines the preferred methods of engagement, per channel.</p> |
| `mailing_address` | String |  | <p>The customer’s mailing address.</p> |
| `account_number` | String |  | <p>An account number that you have assigned to the customer.</p> |
| `first_name` | String |  | <p>The customer’s first name.</p> |
| `birth_date` | String |  | <p>The customer’s birth date. </p> |
| `last_name` | String |  | <p>The customer’s last name.</p> |
| `gender` | String |  | <p>The gender with which the customer identifies. </p> |
| `business_phone_number` | String |  | <p>The customer’s business phone number.</p> |
| `email_address` | String |  | <p>The customer’s email address, which has not been specified as a personal or business
         address. </p> |
| `address` | String |  | <p>A generic address associated with the customer that is not mailing, shipping, or
         billing.</p> |
| `profile_type` | String |  | <p>The type of the profile.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create profile
profile = provider.customer_profiles.Profile {
    domain_name = "value"  # <p>The unique name of the domain.</p>
}

```

---


### Profile_object

ProfileObject resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_name` | String | ✅ | <p>The unique name of the domain.</p> |
| `object` | String | ✅ | <p>A string that is serialized from a JSON object.</p> |
| `object_type_name` | String | ✅ | <p>The name of the profile object type.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create profile_object
profile_object = provider.customer_profiles.Profile_object {
    domain_name = "value"  # <p>The unique name of the domain.</p>
    object = "value"  # <p>A string that is serialized from a JSON object.</p>
    object_type_name = "value"  # <p>The name of the profile object type.</p>
}

```

---


### Event_trigger

EventTrigger resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `object_type_name` | String | ✅ | <p>The unique name of the object type.</p> |
| `segment_filter` | String |  | <p>The destination is triggered only for profiles that meet the criteria of a segment
         definition.</p> |
| `event_trigger_name` | String | ✅ | <p>The unique name of the event trigger.</p> |
| `event_trigger_limits` | String |  | <p>Defines limits controlling whether an event triggers the destination, based on ingestion
         latency and the number of invocations per profile over specific time periods.</p> |
| `event_trigger_conditions` | Vec<String> | ✅ | <p>A list of conditions that determine when an event should trigger the destination.</p> |
| `domain_name` | String | ✅ | <p>The unique name of the domain.</p> |
| `tags` | HashMap<String, String> |  | <p>An array of key-value pairs to apply to this resource.</p> |
| `description` | String |  | <p>The description of the event trigger.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `segment_filter` | String | <p>The destination is triggered only for profiles that meet the criteria of a segment
         definition.</p> |
| `event_trigger_limits` | String | <p>Defines limits controlling whether an event triggers the destination, based on ingestion
         latency and the number of invocations per profile over specific time periods.</p> |
| `event_trigger_conditions` | Vec<String> | <p>A list of conditions that determine when an event should trigger the destination.</p> |
| `description` | String | <p>The description of the event trigger.</p> |
| `object_type_name` | String | <p>The unique name of the object type.</p> |
| `event_trigger_name` | String | <p>The unique name of the event trigger.</p> |
| `tags` | HashMap<String, String> | <p>An array of key-value pairs to apply to this resource.</p> |
| `last_updated_at` | String | <p>The timestamp of when the event trigger was most recently updated.</p> |
| `created_at` | String | <p>The timestamp of when the event trigger was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_trigger
event_trigger = provider.customer_profiles.Event_trigger {
    object_type_name = "value"  # <p>The unique name of the object type.</p>
    event_trigger_name = "value"  # <p>The unique name of the event trigger.</p>
    event_trigger_conditions = "value"  # <p>A list of conditions that determine when an event should trigger the destination.</p>
    domain_name = "value"  # <p>The unique name of the domain.</p>
}

# Access event_trigger outputs
event_trigger_id = event_trigger.id
event_trigger_segment_filter = event_trigger.segment_filter
event_trigger_event_trigger_limits = event_trigger.event_trigger_limits
event_trigger_event_trigger_conditions = event_trigger.event_trigger_conditions
event_trigger_description = event_trigger.description
event_trigger_object_type_name = event_trigger.object_type_name
event_trigger_event_trigger_name = event_trigger.event_trigger_name
event_trigger_tags = event_trigger.tags
event_trigger_last_updated_at = event_trigger.last_updated_at
event_trigger_created_at = event_trigger.created_at
```

---


### Segment_membership

SegmentMembership resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `failures` | Vec<String> | <p>An array of maps where each contains a response per profile failed for the
         request.</p> |
| `segment_definition_name` | String | <p>The unique name of the segment definition.</p> |
| `profiles` | Vec<String> | <p>An array of maps where each contains a response per profile requested.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access segment_membership outputs
segment_membership_id = segment_membership.id
segment_membership_failures = segment_membership.failures
segment_membership_segment_definition_name = segment_membership.segment_definition_name
segment_membership_profiles = segment_membership.profiles
```

---


### Event_stream

EventStream resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uri` | String | ✅ | <p>The StreamARN of the destination to deliver profile events to. For example,
         arn:aws:kinesis:region:account-id:stream/stream-name</p> |
| `event_stream_name` | String | ✅ | <p>The name of the event stream.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource.</p> |
| `domain_name` | String | ✅ | <p>The unique name of the domain.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | <p>The operational state of destination stream for export.</p> |
| `destination_details` | String | <p>Details regarding the Kinesis stream.</p> |
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource.</p> |
| `event_stream_arn` | String | <p>A unique identifier for the event stream.</p> |
| `domain_name` | String | <p>The unique name of the domain.</p> |
| `stopped_since` | String | <p>The timestamp when the <code>State</code> changed to <code>STOPPED</code>.</p> |
| `created_at` | String | <p>The timestamp of when the export was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_stream
event_stream = provider.customer_profiles.Event_stream {
    uri = "value"  # <p>The StreamARN of the destination to deliver profile events to. For example,
         arn:aws:kinesis:region:account-id:stream/stream-name</p>
    event_stream_name = "value"  # <p>The name of the event stream.</p>
    domain_name = "value"  # <p>The unique name of the domain.</p>
}

# Access event_stream outputs
event_stream_id = event_stream.id
event_stream_state = event_stream.state
event_stream_destination_details = event_stream.destination_details
event_stream_tags = event_stream.tags
event_stream_event_stream_arn = event_stream.event_stream_arn
event_stream_domain_name = event_stream.domain_name
event_stream_stopped_since = event_stream.stopped_since
event_stream_created_at = event_stream.created_at
```

---


### Integration_workflow

IntegrationWorkflow resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `workflow_type` | String | ✅ | <p>The type of workflow. The only supported value is APPFLOW_INTEGRATION.</p> |
| `object_type_name` | String | ✅ | <p>The name of the profile object type.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the IAM role. Customer Profiles assumes this role to create resources on your behalf as part of workflow execution.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource.</p> |
| `integration_config` | String | ✅ | <p>Configuration data for integration workflow.</p> |
| `domain_name` | String | ✅ | <p>The unique name of the domain.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create integration_workflow
integration_workflow = provider.customer_profiles.Integration_workflow {
    workflow_type = "value"  # <p>The type of workflow. The only supported value is APPFLOW_INTEGRATION.</p>
    object_type_name = "value"  # <p>The name of the profile object type.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the IAM role. Customer Profiles assumes this role to create resources on your behalf as part of workflow execution.</p>
    integration_config = "value"  # <p>Configuration data for integration workflow.</p>
    domain_name = "value"  # <p>The unique name of the domain.</p>
}

```

---


### Domain_layout

DomainLayout resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_name` | String | ✅ | <p>The unique name of the domain.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource.</p> |
| `description` | String | ✅ | <p>The description of the layout</p> |
| `is_default` | bool |  | <p>If set to true for a layout, this layout will be used by default to view data. If set to
         false, then the layout will not be used by default, but it can be used to view data by
         explicitly selecting it in the console.</p> |
| `layout_type` | String | ✅ | <p>The type of layout that can be used to view data under a Customer Profiles domain.</p> |
| `layout_definition_name` | String | ✅ | <p>The unique name of the layout.</p> |
| `layout` | String | ✅ | <p>A customizable layout that can be used to view data under a Customer Profiles domain.</p> |
| `display_name` | String | ✅ | <p>The display name of the layout</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | <p>The display name of the layout</p> |
| `created_at` | String | <p>The timestamp of when the layout was created.</p> |
| `description` | String | <p>The description of the layout</p> |
| `layout_type` | String | <p>The type of layout that can be used to view data under a Customer Profiles domain.</p> |
| `version` | String | <p>The version used to create layout.</p> |
| `layout` | String | <p>A customizable layout that can be used to view data under a Customer Profiles domain.</p> |
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource.</p> |
| `layout_definition_name` | String | <p>The unique name of the layout.</p> |
| `is_default` | bool | <p>If set to true for a layout, this layout will be used by default to view data. If set to
         false, then the layout will not be used by default, but it can be used to view data by
         explicitly selecting it in the console.</p> |
| `last_updated_at` | String | <p>The timestamp of when the layout was most recently updated.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create domain_layout
domain_layout = provider.customer_profiles.Domain_layout {
    domain_name = "value"  # <p>The unique name of the domain.</p>
    description = "value"  # <p>The description of the layout</p>
    layout_type = "value"  # <p>The type of layout that can be used to view data under a Customer Profiles domain.</p>
    layout_definition_name = "value"  # <p>The unique name of the layout.</p>
    layout = "value"  # <p>A customizable layout that can be used to view data under a Customer Profiles domain.</p>
    display_name = "value"  # <p>The display name of the layout</p>
}

# Access domain_layout outputs
domain_layout_id = domain_layout.id
domain_layout_display_name = domain_layout.display_name
domain_layout_created_at = domain_layout.created_at
domain_layout_description = domain_layout.description
domain_layout_layout_type = domain_layout.layout_type
domain_layout_version = domain_layout.version
domain_layout_layout = domain_layout.layout
domain_layout_tags = domain_layout.tags
domain_layout_layout_definition_name = domain_layout.layout_definition_name
domain_layout_is_default = domain_layout.is_default
domain_layout_last_updated_at = domain_layout.last_updated_at
```

---


### Upload_job_path

UploadJobPath resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `url` | String | <p>The pre-signed S3 URL for uploading the CSV file associated with the upload job. </p> |
| `client_token` | String | <p>The plaintext data key used to encrypt the upload file. </p>
         <p>To persist to the pre-signed url, use the client token and MD5 client token as header.
         The required headers are as follows: </p>
         <ul>
            <li>
               <p>x-amz-server-side-encryption-customer-key: Client Token </p>
            </li>
            <li>
               <p>x-amz-server-side-encryption-customer-key-MD5: MD5 Client Token </p>
            </li>
            <li>
               <p>x-amz-server-side-encryption-customer-algorithm: AES256 </p>
            </li>
         </ul> |
| `valid_until` | String | <p>The expiry timestamp for the pre-signed URL, after which the URL will no longer be
         valid. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access upload_job_path outputs
upload_job_path_id = upload_job_path.id
upload_job_path_url = upload_job_path.url
upload_job_path_client_token = upload_job_path.client_token
upload_job_path_valid_until = upload_job_path.valid_until
```

---


### Segment_definition

SegmentDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description of the segment definition.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource.</p> |
| `display_name` | String | ✅ | <p>The display name of the segment definition.</p> |
| `domain_name` | String | ✅ | <p>The unique name of the domain.</p> |
| `segment_groups` | String | ✅ | <p>Specifies the base segments and dimensions for a segment definition along with their
         respective relationship.</p> |
| `segment_definition_name` | String | ✅ | <p>The unique name of the segment definition.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `segment_definition_arn` | String | <p>The arn of the segment definition.</p> |
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource.</p> |
| `display_name` | String | <p>The display name of the segment definition.</p> |
| `segment_groups` | String | <p>The segment criteria associated with this definition.</p> |
| `created_at` | String | <p>The timestamp of when the segment definition was created.</p> |
| `description` | String | <p>The description of the segment definition.</p> |
| `segment_definition_name` | String | <p>The name of the segment definition.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create segment_definition
segment_definition = provider.customer_profiles.Segment_definition {
    display_name = "value"  # <p>The display name of the segment definition.</p>
    domain_name = "value"  # <p>The unique name of the domain.</p>
    segment_groups = "value"  # <p>Specifies the base segments and dimensions for a segment definition along with their
         respective relationship.</p>
    segment_definition_name = "value"  # <p>The unique name of the segment definition.</p>
}

# Access segment_definition outputs
segment_definition_id = segment_definition.id
segment_definition_segment_definition_arn = segment_definition.segment_definition_arn
segment_definition_tags = segment_definition.tags
segment_definition_display_name = segment_definition.display_name
segment_definition_segment_groups = segment_definition.segment_groups
segment_definition_created_at = segment_definition.created_at
segment_definition_description = segment_definition.description
segment_definition_segment_definition_name = segment_definition.segment_definition_name
```

---


### Calculated_attribute_for_profile

CalculatedAttributeForProfile resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | <p>The display name of the calculated attribute.</p> |
| `value` | String | <p>The value of the calculated attribute.</p> |
| `is_data_partial` | String | <p>Indicates whether the calculated attribute’s value is based on partial data. If data is
         partial, it is set to true.</p> |
| `last_object_timestamp` | String | <p>The timestamp of the newest object included in the calculated attribute
         calculation.</p> |
| `calculated_attribute_name` | String | <p>The unique name of the calculated attribute.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access calculated_attribute_for_profile outputs
calculated_attribute_for_profile_id = calculated_attribute_for_profile.id
calculated_attribute_for_profile_display_name = calculated_attribute_for_profile.display_name
calculated_attribute_for_profile_value = calculated_attribute_for_profile.value
calculated_attribute_for_profile_is_data_partial = calculated_attribute_for_profile.is_data_partial
calculated_attribute_for_profile_last_object_timestamp = calculated_attribute_for_profile.last_object_timestamp
calculated_attribute_for_profile_calculated_attribute_name = calculated_attribute_for_profile.calculated_attribute_name
```

---


### Domain

Domain resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `matching` | String |  | <p>The process of matching duplicate profiles. If <code>Matching</code> = <code>true</code>, Amazon Connect Customer Profiles starts a weekly
batch process called Identity Resolution Job. If you do not specify a date and time for Identity Resolution Job to run, by default it runs every 
Saturday at 12AM UTC to detect duplicate profiles in your domains. </p>
         <p>After the Identity Resolution Job completes, use the 
<a href="https://docs.aws.amazon.com/customerprofiles/latest/APIReference/API_GetMatches.html">GetMatches</a>
API to return and review the results. Or, if you have configured <code>ExportingConfig</code> in the <code>MatchingRequest</code>, you can download the results from
S3.</p> |
| `domain_name` | String | ✅ | <p>The unique name of the domain.</p> |
| `default_encryption_key` | String |  | <p>The default encryption key, which is an AWS managed key, is used when no specific type
         of encryption key is specified. It is used to encrypt all data before it is placed in
         permanent or semi-permanent storage.</p> |
| `dead_letter_queue_url` | String |  | <p>The URL of the SQS dead letter queue, which is used for reporting errors associated with
         ingesting data from third party applications. You must set up a policy on the
         DeadLetterQueue for the SendMessage operation to enable Amazon Connect Customer Profiles to send
         messages to the DeadLetterQueue.</p> |
| `rule_based_matching` | String |  | <p>The process of matching duplicate profiles using the Rule-Based matching. If
            <code>RuleBasedMatching</code> = true, Amazon Connect Customer Profiles will start
         to match and merge your profiles according to your configuration in the
            <code>RuleBasedMatchingRequest</code>. You can use the <code>ListRuleBasedMatches</code>
         and <code>GetSimilarProfiles</code> API to return and review the results. Also, if you have
         configured <code>ExportingConfig</code> in the <code>RuleBasedMatchingRequest</code>, you
         can download the results from S3.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource.</p> |
| `default_expiration_days` | i64 | ✅ | <p>The default number of days until the data within the domain expires.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_at` | String | <p>The timestamp of when the domain was created.</p> |
| `default_encryption_key` | String | <p>The default encryption key, which is an AWS managed key, is used when no specific type
         of encryption key is specified. It is used to encrypt all data before it is placed in
         permanent or semi-permanent storage.</p> |
| `dead_letter_queue_url` | String | <p>The URL of the SQS dead letter queue, which is used for reporting errors associated with
         ingesting data from third party applications.</p> |
| `domain_name` | String | <p>The unique name of the domain.</p> |
| `matching` | String | <p>The process of matching duplicate profiles. If <code>Matching</code> = <code>true</code>, Amazon Connect Customer Profiles starts a weekly
batch process called Identity Resolution Job. If you do not specify a date and time for Identity Resolution Job to run, by default it runs every 
Saturday at 12AM UTC to detect duplicate profiles in your domains. </p>
         <p>After the Identity Resolution Job completes, use the 
<a href="https://docs.aws.amazon.com/customerprofiles/latest/APIReference/API_GetMatches.html">GetMatches</a>
API to return and review the results. Or, if you have configured <code>ExportingConfig</code> in the <code>MatchingRequest</code>, you can download the results from
S3.</p> |
| `stats` | String | <p>Usage-specific statistics about the domain.</p> |
| `rule_based_matching` | String | <p>The process of matching duplicate profiles using the Rule-Based matching. If
            <code>RuleBasedMatching</code> = true, Amazon Connect Customer Profiles will start
         to match and merge your profiles according to your configuration in the
            <code>RuleBasedMatchingRequest</code>. You can use the <code>ListRuleBasedMatches</code>
         and <code>GetSimilarProfiles</code> API to return and review the results. Also, if you have
         configured <code>ExportingConfig</code> in the <code>RuleBasedMatchingRequest</code>, you
         can download the results from S3.</p> |
| `default_expiration_days` | i64 | <p>The default number of days until the data within the domain expires.</p> |
| `last_updated_at` | String | <p>The timestamp of when the domain was most recently edited.</p> |
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create domain
domain = provider.customer_profiles.Domain {
    domain_name = "value"  # <p>The unique name of the domain.</p>
    default_expiration_days = "value"  # <p>The default number of days until the data within the domain expires.</p>
}

# Access domain outputs
domain_id = domain.id
domain_created_at = domain.created_at
domain_default_encryption_key = domain.default_encryption_key
domain_dead_letter_queue_url = domain.dead_letter_queue_url
domain_domain_name = domain.domain_name
domain_matching = domain.matching
domain_stats = domain.stats
domain_rule_based_matching = domain.rule_based_matching
domain_default_expiration_days = domain.default_expiration_days
domain_last_updated_at = domain.last_updated_at
domain_tags = domain.tags
```

---


### Profile_history_record

ProfileHistoryRecord resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `performed_by` | String | <p>The Amazon Resource Name (ARN) of the person or service principal who performed the action.</p> |
| `created_at` | String | <p>The timestamp of when the profile history record was created.</p> |
| `id` | String | <p>The unique identifier of the profile history record.</p> |
| `profile_object_unique_key` | String | <p>The unique identifier of the profile object generated by the service.</p> |
| `content` | String | <p>A string containing the customer profile, profile object, or profile key content.</p> |
| `last_updated_at` | String | <p>The timestamp of when the profile history record was last updated.</p> |
| `object_type_name` | String | <p>The name of the profile object type.</p> |
| `action_type` | String | <p>The action type of the profile history record.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access profile_history_record outputs
profile_history_record_id = profile_history_record.id
profile_history_record_performed_by = profile_history_record.performed_by
profile_history_record_created_at = profile_history_record.created_at
profile_history_record_id = profile_history_record.id
profile_history_record_profile_object_unique_key = profile_history_record.profile_object_unique_key
profile_history_record_content = profile_history_record.content
profile_history_record_last_updated_at = profile_history_record.last_updated_at
profile_history_record_object_type_name = profile_history_record.object_type_name
profile_history_record_action_type = profile_history_record.action_type
```

---


### Similar_profiles

SimilarProfiles resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rule_level` | i64 | <p>The integer rule level that the profiles matched on.</p> |
| `next_token` | String | <p>The pagination token from the previous <code>GetSimilarProfiles</code> API call.</p> |
| `profile_ids` | Vec<String> | <p>Set of <code>profileId</code>s that belong to the same matching group.</p> |
| `match_type` | String | <p>Specify the type of matching to get similar profiles for.</p> |
| `confidence_score` | f64 | <p>It only has value when the <code>MatchType</code> is <code>ML_BASED_MATCHING</code>.A
         number between 0 and 1, where a higher score means higher similarity. Examining match
         confidence scores lets you distinguish between groups of similar records in which the
         system is highly confident (which you may decide to merge), groups of similar records about
         which the system is uncertain (which you may decide to have reviewed by a human), and
         groups of similar records that the system deems to be unlikely (which you may decide to
         reject). Given confidence scores vary as per the data input, it should not be used as an
         absolute measure of matching quality.</p> |
| `match_id` | String | <p>The string <code>matchId</code> that the similar profiles belong to.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access similar_profiles outputs
similar_profiles_id = similar_profiles.id
similar_profiles_rule_level = similar_profiles.rule_level
similar_profiles_next_token = similar_profiles.next_token
similar_profiles_profile_ids = similar_profiles.profile_ids
similar_profiles_match_type = similar_profiles.match_type
similar_profiles_confidence_score = similar_profiles.confidence_score
similar_profiles_match_id = similar_profiles.match_id
```

---


### Auto_merging_preview

AutoMergingPreview resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `number_of_profiles_in_sample` | i64 | <p>The number of profiles found in this preview dry run.</p> |
| `number_of_profiles_will_be_merged` | i64 | <p>The number of profiles that would be merged if this wasn't a preview dry run.</p> |
| `number_of_matches_in_sample` | i64 | <p>The number of match groups in the domain that have been reviewed in this preview dry
         run.</p> |
| `domain_name` | String | <p>The unique name of the domain.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access auto_merging_preview outputs
auto_merging_preview_id = auto_merging_preview.id
auto_merging_preview_number_of_profiles_in_sample = auto_merging_preview.number_of_profiles_in_sample
auto_merging_preview_number_of_profiles_will_be_merged = auto_merging_preview.number_of_profiles_will_be_merged
auto_merging_preview_number_of_matches_in_sample = auto_merging_preview.number_of_matches_in_sample
auto_merging_preview_domain_name = auto_merging_preview.domain_name
```

---


### Identity_resolution_job

IdentityResolutionJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `exporting_location` | String | <p>The S3 location where the Identity Resolution Job writes result files.</p> |
| `job_stats` | String | <p>Statistics about the Identity Resolution Job.</p> |
| `status` | String | <p>The status of the Identity Resolution Job.</p>
         <ul>
            <li>
               <p>
                  <code>PENDING</code>: The Identity Resolution Job is scheduled but has not started yet. If you turn
               off the Identity Resolution feature in your domain, jobs in the <code>PENDING</code> state are
               deleted.</p>
            </li>
            <li>
               <p>
                  <code>PREPROCESSING</code>: The Identity Resolution Job is loading your data.</p>
            </li>
            <li>
               <p>
                  <code>FIND_MATCHING</code>: The Identity Resolution Job is using the machine learning model to
               identify profiles that belong to the same matching group.</p>
            </li>
            <li>
               <p>
                  <code>MERGING</code>: The Identity Resolution Job is merging duplicate profiles.</p>
            </li>
            <li>
               <p>
                  <code>COMPLETED</code>: The Identity Resolution Job completed successfully.</p>
            </li>
            <li>
               <p>
                  <code>PARTIAL_SUCCESS</code>: There's a system error and not all of the data is
               merged. The Identity Resolution Job writes a message indicating the source of the problem.</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code>: The Identity Resolution Job did not merge any data. It writes a message
               indicating the source of the problem.</p>
            </li>
         </ul> |
| `message` | String | <p>The error messages that are generated when the Identity Resolution Job runs.</p> |
| `domain_name` | String | <p>The unique name of the domain.</p> |
| `auto_merging` | String | <p>Configuration settings for how to perform the auto-merging of profiles.</p> |
| `job_start_time` | String | <p>The timestamp of when the Identity Resolution Job was started or will be started.</p> |
| `job_end_time` | String | <p>The timestamp of when the Identity Resolution Job was completed.</p> |
| `last_updated_at` | String | <p>The timestamp of when the Identity Resolution Job was most recently edited.</p> |
| `job_id` | String | <p>The unique identifier of the Identity Resolution Job.</p> |
| `job_expiration_time` | String | <p>The timestamp of when the Identity Resolution Job will expire.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access identity_resolution_job outputs
identity_resolution_job_id = identity_resolution_job.id
identity_resolution_job_exporting_location = identity_resolution_job.exporting_location
identity_resolution_job_job_stats = identity_resolution_job.job_stats
identity_resolution_job_status = identity_resolution_job.status
identity_resolution_job_message = identity_resolution_job.message
identity_resolution_job_domain_name = identity_resolution_job.domain_name
identity_resolution_job_auto_merging = identity_resolution_job.auto_merging
identity_resolution_job_job_start_time = identity_resolution_job.job_start_time
identity_resolution_job_job_end_time = identity_resolution_job.job_end_time
identity_resolution_job_last_updated_at = identity_resolution_job.last_updated_at
identity_resolution_job_job_id = identity_resolution_job.job_id
identity_resolution_job_job_expiration_time = identity_resolution_job.job_expiration_time
```

---


### Segment_estimate

SegmentEstimate resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_name` | String | ✅ | <p>The unique name of the domain.</p> |
| `segment_query` | String | ✅ | <p>The segment query for calculating a segment estimate.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The current status of the query.</p> |
| `message` | String | <p>The error message if there is any error.</p> |
| `domain_name` | String | <p>The unique name of the domain.</p> |
| `status_code` | i64 | <p>The status code of the segment estimate.</p> |
| `estimate` | String | <p>The estimated number of profiles contained in the segment.</p> |
| `estimate_id` | String | <p>The <code>QueryId</code> which is the same as the value passed in
         <code>QueryId</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create segment_estimate
segment_estimate = provider.customer_profiles.Segment_estimate {
    domain_name = "value"  # <p>The unique name of the domain.</p>
    segment_query = "value"  # <p>The segment query for calculating a segment estimate.</p>
}

# Access segment_estimate outputs
segment_estimate_id = segment_estimate.id
segment_estimate_status = segment_estimate.status
segment_estimate_message = segment_estimate.message
segment_estimate_domain_name = segment_estimate.domain_name
segment_estimate_status_code = segment_estimate.status_code
segment_estimate_estimate = segment_estimate.estimate
segment_estimate_estimate_id = segment_estimate.estimate_id
```

---


### Matches

Matches resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `match_generation_date` | String | <p>The timestamp this version of Match Result generated.</p> |
| `matches` | Vec<String> | <p>The list of matched profiles for this instance.</p> |
| `next_token` | String | <p>If there are additional results, this is the token for the next set of results.</p> |
| `potential_matches` | i64 | <p>The number of potential matches found.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access matches outputs
matches_id = matches.id
matches_match_generation_date = matches.match_generation_date
matches_matches = matches.matches
matches_next_token = matches.next_token
matches_potential_matches = matches.potential_matches
```

---


### Profile_key

ProfileKey resource

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


### Calculated_attribute_definition

CalculatedAttributeDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filter` | String |  | <p>Defines how to filter incoming objects to include part of the Calculated
         Attribute.</p> |
| `use_historical_data` | bool |  | <p>Whether historical data ingested before the Calculated Attribute was created should be
         included in calculations.</p> |
| `calculated_attribute_name` | String | ✅ | <p>The unique name of the calculated attribute.</p> |
| `attribute_details` | String | ✅ | <p>Mathematical expression and a list of attribute items specified in that
         expression.</p> |
| `description` | String |  | <p>The description of the calculated attribute.</p> |
| `statistic` | String | ✅ | <p>The aggregation operation to perform for the calculated attribute.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource.</p> |
| `domain_name` | String | ✅ | <p>The unique name of the domain.</p> |
| `conditions` | String |  | <p>The conditions including range, object count, and threshold for the calculated
         attribute.</p> |
| `display_name` | String |  | <p>The display name of the calculated attribute.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `readiness` | String | <p>Information indicating if the Calculated Attribute is ready for use by confirming all
         historical data has been processed and reflected.</p> |
| `description` | String | <p>The description of the calculated attribute.</p> |
| `use_historical_data` | bool | <p>Whether historical data ingested before the Calculated Attribute was created should be
         included in calculations.</p> |
| `display_name` | String | <p>The display name of the calculated attribute.</p> |
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource.</p> |
| `last_updated_at` | String | <p>The timestamp of when the calculated attribute definition was most recently
         edited.</p> |
| `calculated_attribute_name` | String | <p>The unique name of the calculated attribute.</p> |
| `statistic` | String | <p>The aggregation operation to perform for the calculated attribute.</p> |
| `conditions` | String | <p>The conditions including range, object count, and threshold for the calculated
         attribute.</p> |
| `created_at` | String | <p>The timestamp of when the calculated attribute definition was created.</p> |
| `attribute_details` | String | <p>Mathematical expression and a list of attribute items specified in that
         expression.</p> |
| `status` | String | <p>Status of the Calculated Attribute creation (whether all historical data has been
         indexed).</p> |
| `filter` | String | <p>The filter assigned to this calculated attribute definition.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create calculated_attribute_definition
calculated_attribute_definition = provider.customer_profiles.Calculated_attribute_definition {
    calculated_attribute_name = "value"  # <p>The unique name of the calculated attribute.</p>
    attribute_details = "value"  # <p>Mathematical expression and a list of attribute items specified in that
         expression.</p>
    statistic = "value"  # <p>The aggregation operation to perform for the calculated attribute.</p>
    domain_name = "value"  # <p>The unique name of the domain.</p>
}

# Access calculated_attribute_definition outputs
calculated_attribute_definition_id = calculated_attribute_definition.id
calculated_attribute_definition_readiness = calculated_attribute_definition.readiness
calculated_attribute_definition_description = calculated_attribute_definition.description
calculated_attribute_definition_use_historical_data = calculated_attribute_definition.use_historical_data
calculated_attribute_definition_display_name = calculated_attribute_definition.display_name
calculated_attribute_definition_tags = calculated_attribute_definition.tags
calculated_attribute_definition_last_updated_at = calculated_attribute_definition.last_updated_at
calculated_attribute_definition_calculated_attribute_name = calculated_attribute_definition.calculated_attribute_name
calculated_attribute_definition_statistic = calculated_attribute_definition.statistic
calculated_attribute_definition_conditions = calculated_attribute_definition.conditions
calculated_attribute_definition_created_at = calculated_attribute_definition.created_at
calculated_attribute_definition_attribute_details = calculated_attribute_definition.attribute_details
calculated_attribute_definition_status = calculated_attribute_definition.status
calculated_attribute_definition_filter = calculated_attribute_definition.filter
```

---


### Workflow

Workflow resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workflow_type` | String | <p>The type of workflow. The only supported value is APPFLOW_INTEGRATION.</p> |
| `status` | String | <p>Status of workflow execution.</p> |
| `start_date` | String | <p>The timestamp that represents when workflow execution started.</p> |
| `metrics` | String | <p>Workflow specific execution metrics.</p> |
| `workflow_id` | String | <p>Unique identifier for the workflow.</p> |
| `last_updated_at` | String | <p>The timestamp that represents when workflow execution last updated.</p> |
| `attributes` | String | <p>Attributes provided for workflow execution.</p> |
| `error_description` | String | <p>Workflow error messages during execution (if any).</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workflow outputs
workflow_id = workflow.id
workflow_workflow_type = workflow.workflow_type
workflow_status = workflow.status
workflow_start_date = workflow.start_date
workflow_metrics = workflow.metrics
workflow_workflow_id = workflow.workflow_id
workflow_last_updated_at = workflow.last_updated_at
workflow_attributes = workflow.attributes
workflow_error_description = workflow.error_description
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple segment_snapshot resources
segment_snapshot_0 = provider.customer_profiles.Segment_snapshot {
    domain_name = "value-0"
    segment_definition_name = "value-0"
    data_format = "value-0"
}
segment_snapshot_1 = provider.customer_profiles.Segment_snapshot {
    domain_name = "value-1"
    segment_definition_name = "value-1"
    data_format = "value-1"
}
segment_snapshot_2 = provider.customer_profiles.Segment_snapshot {
    domain_name = "value-2"
    segment_definition_name = "value-2"
    data_format = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    segment_snapshot = provider.customer_profiles.Segment_snapshot {
        domain_name = "production-value"
        segment_definition_name = "production-value"
        data_format = "production-value"
    }
```

---

## Related Documentation

- [AWS Customer_profiles Documentation](https://docs.aws.amazon.com/customer_profiles/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
