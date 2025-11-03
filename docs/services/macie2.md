# Macie2 Service



**Resources**: 29

---

## Overview

The macie2 service provides access to 29 resource types:

- [Sample_findings](#sample_findings) [C]
- [Custom_data_identifier](#custom_data_identifier) [CRD]
- [Macie_session](#macie_session) [RU]
- [Findings](#findings) [R]
- [Sensitive_data_occurrences_availability](#sensitive_data_occurrences_availability) [R]
- [Member_session](#member_session) [U]
- [Invitations](#invitations) [CD]
- [Sensitive_data_occurrences](#sensitive_data_occurrences) [R]
- [Master_account](#master_account) [R]
- [Member](#member) [CRD]
- [Resource_profile](#resource_profile) [RU]
- [Classification_job](#classification_job) [CRU]
- [Automated_discovery_configuration](#automated_discovery_configuration) [RU]
- [Organization_configuration](#organization_configuration) [RU]
- [Findings_publication_configuration](#findings_publication_configuration) [CR]
- [Findings_filter](#findings_filter) [CRUD]
- [Resource_profile_detections](#resource_profile_detections) [U]
- [Usage_statistics](#usage_statistics) [R]
- [Sensitivity_inspection_template](#sensitivity_inspection_template) [RU]
- [Bucket_statistics](#bucket_statistics) [R]
- [Buckets](#buckets) [R]
- [Classification_scope](#classification_scope) [RU]
- [Classification_export_configuration](#classification_export_configuration) [CR]
- [Allow_list](#allow_list) [CRUD]
- [Reveal_configuration](#reveal_configuration) [RU]
- [Administrator_account](#administrator_account) [R]
- [Finding_statistics](#finding_statistics) [R]
- [Invitations_count](#invitations_count) [R]
- [Usage_totals](#usage_totals) [R]

---

## Resources


### Sample_findings

SampleFindings resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `finding_types` | Vec<String> |  | <p>An array of finding types, one for each type of sample finding to create. To create a sample of every type of finding that Amazon Macie supports, don't include this array in your request.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sample_findings
sample_findings = provider.macie2.Sample_findings {
}

```

---


### Custom_data_identifier

CustomDataIdentifier resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `severity_levels` | Vec<String> |  | <p>The severity to assign to findings that the custom data identifier produces, based on the number of occurrences of text that match the custom data identifier's detection criteria. You can specify as many as three SeverityLevel objects in this array, one for each severity: LOW, MEDIUM, or HIGH. If you specify more than one, the occurrences thresholds must be in ascending order by severity, moving from LOW to HIGH. For example, 1 for LOW, 50 for MEDIUM, and 100 for HIGH. If an S3 object contains fewer occurrences than the lowest specified threshold, Amazon Macie doesn't create a finding.</p> <p>If you don't specify any values for this array, Macie creates findings for S3 objects that contain at least one occurrence of text that matches the detection criteria, and Macie assigns the MEDIUM severity to those findings.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive token that you provide to ensure the idempotency of the request.</p> |
| `maximum_match_distance` | i64 |  | <p>The maximum number of characters that can exist between the end of at least one complete character sequence specified by the keywords array and the end of the text that matches the regex pattern. If a complete keyword precedes all the text that matches the pattern and the keyword is within the specified distance, Amazon Macie includes the result. The distance can be 1-300 characters. The default value is 50.</p> |
| `ignore_words` | Vec<String> |  | <p>An array that lists specific character sequences (<i>ignore words</i>) to exclude from the results. If the text matched by the regular expression contains any string in this array, Amazon Macie ignores it. The array can contain as many as 10 ignore words. Each ignore word can contain 4-90 UTF-8 characters. Ignore words are case sensitive.</p> |
| `keywords` | Vec<String> |  | <p>An array that lists specific character sequences (<i>keywords</i>), one of which must precede and be within proximity (maximumMatchDistance) of the regular expression to match. The array can contain as many as 50 keywords. Each keyword can contain 3-90 UTF-8 characters. Keywords aren't case sensitive.</p> |
| `regex` | String | ✅ | <p>The regular expression (<i>regex</i>) that defines the pattern to match. The expression can contain as many as 512 characters.</p> |
| `name` | String | ✅ | <p>A custom name for the custom data identifier. The name can contain as many as 128 characters.</p> <p>We strongly recommend that you avoid including any sensitive data in the name of a custom data identifier. Other users of your account might be able to see this name, depending on the actions that they're allowed to perform in Amazon Macie.</p> |
| `tags` | HashMap<String, String> |  | <p>A map of key-value pairs that specifies the tags to associate with the custom data identifier.</p> <p>A custom data identifier can have a maximum of 50 tags. Each tag consists of a tag key and an associated tag value. The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.</p> |
| `description` | String |  | <p>A custom description of the custom data identifier. The description can contain as many as 512 characters.</p> <p>We strongly recommend that you avoid including any sensitive data in the description of a custom data identifier. Other users of your account might be able to see this description, depending on the actions that they're allowed to perform in Amazon Macie.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | HashMap<String, String> | <p>A map of key-value pairs that identifies the tags (keys and values) that are associated with the custom data identifier.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the custom data identifier.</p> |
| `description` | String | <p>The custom description of the custom data identifier.</p> |
| `deleted` | bool | <p>Specifies whether the custom data identifier was deleted. If you delete a custom data identifier, Amazon Macie doesn't delete it permanently. Instead, it soft deletes the identifier.</p> |
| `id` | String | <p>The unique identifier for the custom data identifier.</p> |
| `keywords` | Vec<String> | <p>An array that lists specific character sequences (<i>keywords</i>), one of which must precede and be within proximity (maximumMatchDistance) of the regular expression to match. Keywords aren't case sensitive.</p> |
| `maximum_match_distance` | i64 | <p>The maximum number of characters that can exist between the end of at least one complete character sequence specified by the keywords array and the end of the text that matches the regex pattern. If a complete keyword precedes all the text that matches the pattern and the keyword is within the specified distance, Amazon Macie includes the result. Otherwise, Macie excludes the result.</p> |
| `name` | String | <p>The custom name of the custom data identifier.</p> |
| `ignore_words` | Vec<String> | <p>An array that lists specific character sequences (<i>ignore words</i>) to exclude from the results. If the text matched by the regular expression contains any string in this array, Amazon Macie ignores it. Ignore words are case sensitive.</p> |
| `severity_levels` | Vec<String> | <p>Specifies the severity that's assigned to findings that the custom data identifier produces, based on the number of occurrences of text that match the custom data identifier's detection criteria. By default, Amazon Macie creates findings for S3 objects that contain at least one occurrence of text that matches the detection criteria, and Macie assigns the MEDIUM severity to those findings.</p> |
| `created_at` | String | <p>The date and time, in UTC and extended ISO 8601 format, when the custom data identifier was created.</p> |
| `regex` | String | <p>The regular expression (<i>regex</i>) that defines the pattern to match.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create custom_data_identifier
custom_data_identifier = provider.macie2.Custom_data_identifier {
    regex = "value"  # <p>The regular expression (<i>regex</i>) that defines the pattern to match. The expression can contain as many as 512 characters.</p>
    name = "value"  # <p>A custom name for the custom data identifier. The name can contain as many as 128 characters.</p> <p>We strongly recommend that you avoid including any sensitive data in the name of a custom data identifier. Other users of your account might be able to see this name, depending on the actions that they're allowed to perform in Amazon Macie.</p>
}

# Access custom_data_identifier outputs
custom_data_identifier_id = custom_data_identifier.id
custom_data_identifier_tags = custom_data_identifier.tags
custom_data_identifier_arn = custom_data_identifier.arn
custom_data_identifier_description = custom_data_identifier.description
custom_data_identifier_deleted = custom_data_identifier.deleted
custom_data_identifier_id = custom_data_identifier.id
custom_data_identifier_keywords = custom_data_identifier.keywords
custom_data_identifier_maximum_match_distance = custom_data_identifier.maximum_match_distance
custom_data_identifier_name = custom_data_identifier.name
custom_data_identifier_ignore_words = custom_data_identifier.ignore_words
custom_data_identifier_severity_levels = custom_data_identifier.severity_levels
custom_data_identifier_created_at = custom_data_identifier.created_at
custom_data_identifier_regex = custom_data_identifier.regex
```

---


### Macie_session

MacieSession resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `finding_publishing_frequency` | String |  | <p>Specifies how often to publish updates to policy findings for the account. This includes publishing updates to Security Hub and Amazon EventBridge (formerly Amazon CloudWatch Events).</p> |
| `status` | String |  | <p>Specifies a new status for the account. Valid values are: ENABLED, resume all Amazon Macie activities for the account; and, PAUSED, suspend all Macie activities for the account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `finding_publishing_frequency` | String | <p>The frequency with which Amazon Macie publishes updates to policy findings for the account. This includes publishing updates to Security Hub and Amazon EventBridge (formerly Amazon CloudWatch Events).</p> |
| `updated_at` | String | <p>The date and time, in UTC and extended ISO 8601 format, of the most recent change to the status or configuration settings for the Amazon Macie account.</p> |
| `service_role` | String | <p>The Amazon Resource Name (ARN) of the service-linked role that allows Amazon Macie to monitor and analyze data in Amazon Web Services resources for the account.</p> |
| `status` | String | <p>The current status of the Amazon Macie account. Possible values are: PAUSED, the account is enabled but all Macie activities are suspended (paused) for the account; and, ENABLED, the account is enabled and all Macie activities are enabled for the account.</p> |
| `created_at` | String | <p>The date and time, in UTC and extended ISO 8601 format, when the Amazon Macie account was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access macie_session outputs
macie_session_id = macie_session.id
macie_session_finding_publishing_frequency = macie_session.finding_publishing_frequency
macie_session_updated_at = macie_session.updated_at
macie_session_service_role = macie_session.service_role
macie_session_status = macie_session.status
macie_session_created_at = macie_session.created_at
```

---


### Findings

Findings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `findings` | Vec<String> | <p>An array of objects, one for each finding that matches the criteria specified in the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access findings outputs
findings_id = findings.id
findings_findings = findings.findings
```

---


### Sensitive_data_occurrences_availability

SensitiveDataOccurrencesAvailability resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `code` | String | <p>Specifies whether occurrences of sensitive data can be retrieved for the finding. Possible values are: AVAILABLE, the sensitive data can be retrieved; and, UNAVAILABLE, the sensitive data can't be retrieved. If this value is UNAVAILABLE, the reasons array indicates why the data can't be retrieved.</p> |
| `reasons` | Vec<String> | <p>Specifies why occurrences of sensitive data can't be retrieved for the finding. Possible values are:</p> <ul><li><p>ACCOUNT_NOT_IN_ORGANIZATION - The affected account isn't currently part of your organization. Or the account is part of your organization but Macie isn't currently enabled for the account. You're not allowed to access the affected S3 object by using Macie.</p></li> <li><p>INVALID_CLASSIFICATION_RESULT - There isn't a corresponding sensitive data discovery result for the finding. Or the corresponding sensitive data discovery result isn't available in the current Amazon Web Services Region, is malformed or corrupted, or uses an unsupported storage format. Macie can't verify the location of the sensitive data to retrieve.</p></li> <li><p>INVALID_RESULT_SIGNATURE - The corresponding sensitive data discovery result is stored in an S3 object that wasn't signed by Macie. Macie can't verify the integrity and authenticity of the sensitive data discovery result. Therefore, Macie can't verify the location of the sensitive data to retrieve.</p></li> <li><p>MEMBER_ROLE_TOO_PERMISSIVE - The trust or permissions policy for the IAM role in the affected member account doesn't meet Macie requirements for restricting access to the role. Or the role's trust policy doesn't specify the correct external ID for your organization. Macie can't assume the role to retrieve the sensitive data.</p></li> <li><p>MISSING_GET_MEMBER_PERMISSION - You're not allowed to retrieve information about the association between your account and the affected account. Macie can't determine whether you’re allowed to access the affected S3 object as the delegated Macie administrator for the affected account.</p></li> <li><p>OBJECT_EXCEEDS_SIZE_QUOTA - The storage size of the affected S3 object exceeds the size quota for retrieving occurrences of sensitive data from this type of file.</p></li> <li><p>OBJECT_UNAVAILABLE - The affected S3 object isn't available. The object was renamed, moved, deleted, or changed after Macie created the finding. Or the object is encrypted with an KMS key that isn’t available. For example, the key is disabled, is scheduled for deletion, or was deleted.</p></li> <li><p>RESULT_NOT_SIGNED - The corresponding sensitive data discovery result is stored in an S3 object that hasn't been signed. Macie can't verify the integrity and authenticity of the sensitive data discovery result. Therefore, Macie can't verify the location of the sensitive data to retrieve.</p></li> <li><p>ROLE_TOO_PERMISSIVE - Your account is configured to retrieve occurrences of sensitive data by using an IAM role whose trust or permissions policy doesn't meet Macie requirements for restricting access to the role. Macie can’t assume the role to retrieve the sensitive data.</p></li> <li><p>UNSUPPORTED_FINDING_TYPE - The specified finding isn't a sensitive data finding.</p></li> <li><p>UNSUPPORTED_OBJECT_TYPE - The affected S3 object uses a file or storage format that Macie doesn't support for retrieving occurrences of sensitive data.</p></li></ul> <p>This value is null if sensitive data can be retrieved for the finding.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sensitive_data_occurrences_availability outputs
sensitive_data_occurrences_availability_id = sensitive_data_occurrences_availability.id
sensitive_data_occurrences_availability_code = sensitive_data_occurrences_availability.code
sensitive_data_occurrences_availability_reasons = sensitive_data_occurrences_availability.reasons
```

---


### Member_session

MemberSession resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status` | String | ✅ | <p>Specifies the new status for the account. Valid values are: ENABLED, resume all Amazon Macie activities for the account; and, PAUSED, suspend all Macie activities for the account.</p> |
| `id` | String | ✅ | <p>The unique identifier for the Amazon Macie resource that the request applies to.</p> |



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


### Invitations

Invitations resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `message` | String |  | <p>Custom text to include in the email message that contains the invitation. The text can contain as many as 80 alphanumeric characters.</p> |
| `disable_email_notification` | bool |  | <p>Specifies whether to send the invitation as an email message. If this value is false, Amazon Macie sends the invitation (as an email message) to the email address that you specified for the recipient's account when you associated the account with your account. The default value is false.</p> |
| `account_ids` | Vec<String> | ✅ | <p>An array that lists Amazon Web Services account IDs, one for each account to send the invitation to.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create invitations
invitations = provider.macie2.Invitations {
    account_ids = "value"  # <p>An array that lists Amazon Web Services account IDs, one for each account to send the invitation to.</p>
}

```

---


### Sensitive_data_occurrences

SensitiveDataOccurrences resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The status of the request to retrieve occurrences of sensitive data reported by the finding. Possible values are:</p> <ul><li><p>ERROR - An error occurred when Amazon Macie attempted to locate, retrieve, or encrypt the sensitive data. The error value indicates the nature of the error that occurred.</p></li> <li><p>PROCESSING - Macie is processing the request.</p></li> <li><p>SUCCESS - Macie successfully located, retrieved, and encrypted the sensitive data.</p></li></ul> |
| `error` | String | <p>If an error occurred when Amazon Macie attempted to retrieve occurrences of sensitive data reported by the finding, a description of the error that occurred. This value is null if the status (status) of the request is PROCESSING or SUCCESS.</p> |
| `sensitive_data_occurrences` | HashMap<String, Vec<String>> | <p>A map that specifies 1-100 types of sensitive data reported by the finding and, for each type, 1-10 occurrences of sensitive data.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sensitive_data_occurrences outputs
sensitive_data_occurrences_id = sensitive_data_occurrences.id
sensitive_data_occurrences_status = sensitive_data_occurrences.status
sensitive_data_occurrences_error = sensitive_data_occurrences.error
sensitive_data_occurrences_sensitive_data_occurrences = sensitive_data_occurrences.sensitive_data_occurrences
```

---


### Master_account

MasterAccount resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `master` | String | <p>(Deprecated) The Amazon Web Services account ID for the administrator account. If the accounts are associated by a Macie membership invitation, this object also provides details about the invitation that was sent to establish the relationship between the accounts.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access master_account outputs
master_account_id = master_account.id
master_account_master = master_account.master
```

---


### Member

Member resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account` | String | ✅ | <p>The details of the account to associate with the administrator account.</p> |
| `tags` | HashMap<String, String> |  | <p>A map of key-value pairs that specifies the tags to associate with the account in Amazon Macie.</p> <p>An account can have a maximum of 50 tags. Each tag consists of a tag key and an associated tag value. The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `email` | String | <p>The email address for the account. This value is null if the account is associated with the administrator account through Organizations.</p> |
| `administrator_account_id` | String | <p>The Amazon Web Services account ID for the administrator account.</p> |
| `updated_at` | String | <p>The date and time, in UTC and extended ISO 8601 format, of the most recent change to the status of the relationship between the account and the administrator account.</p> |
| `invited_at` | String | <p>The date and time, in UTC and extended ISO 8601 format, when an Amazon Macie membership invitation was last sent to the account. This value is null if a Macie membership invitation hasn't been sent to the account.</p> |
| `tags` | HashMap<String, String> | <p>A map of key-value pairs that specifies which tags (keys and values) are associated with the account in Amazon Macie.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the account.</p> |
| `master_account_id` | String | <p>(Deprecated) The Amazon Web Services account ID for the administrator account. This property has been replaced by the administratorAccountId property and is retained only for backward compatibility.</p> |
| `relationship_status` | String | <p>The current status of the relationship between the account and the administrator account.</p> |
| `account_id` | String | <p>The Amazon Web Services account ID for the account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create member
member = provider.macie2.Member {
    account = "value"  # <p>The details of the account to associate with the administrator account.</p>
}

# Access member outputs
member_id = member.id
member_email = member.email
member_administrator_account_id = member.administrator_account_id
member_updated_at = member.updated_at
member_invited_at = member.invited_at
member_tags = member.tags
member_arn = member.arn
member_master_account_id = member.master_account_id
member_relationship_status = member.relationship_status
member_account_id = member.account_id
```

---


### Resource_profile

ResourceProfile resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the S3 bucket that the request applies to.</p> |
| `sensitivity_score_override` | i64 |  | <p>The new sensitivity score for the bucket. Valid values are: 100, assign the maximum score and apply the <i>Sensitive</i> label to the bucket; and, null (empty), assign a score that Amazon Macie calculates automatically after you submit the request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `profile_updated_at` | String | <p>The date and time, in UTC and extended ISO 8601 format, when Amazon Macie most recently recalculated sensitive data discovery statistics and details for the bucket. If the bucket's sensitivity score is calculated automatically, this includes the score.</p> |
| `sensitivity_score_overridden` | bool | <p>Specifies whether the bucket's current sensitivity score was set manually. If this value is true, the score was manually changed to 100. If this value is false, the score was calculated automatically by Amazon Macie.</p> |
| `statistics` | String | <p>The sensitive data discovery statistics for the bucket. The statistics capture the results of automated sensitive data discovery activities that Amazon Macie has performed for the bucket.</p> |
| `sensitivity_score` | i64 | <p>The current sensitivity score for the bucket, ranging from -1 (classification error) to 100 (sensitive). By default, this score is calculated automatically based on the amount of data that Amazon Macie has analyzed in the bucket and the amount of sensitive data that Macie has found in the bucket.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_profile outputs
resource_profile_id = resource_profile.id
resource_profile_profile_updated_at = resource_profile.profile_updated_at
resource_profile_sensitivity_score_overridden = resource_profile.sensitivity_score_overridden
resource_profile_statistics = resource_profile.statistics
resource_profile_sensitivity_score = resource_profile.sensitivity_score
```

---


### Classification_job

ClassificationJob resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `managed_data_identifier_selector` | String |  | <p>The selection type to apply when determining which managed data identifiers the job uses to analyze data. Valid values are:</p> <ul><li><p>ALL - Use all managed data identifiers. If you specify this value, don't specify any values for the managedDataIdentifierIds property.</p></li> <li><p>EXCLUDE - Use all managed data identifiers except the ones specified by the managedDataIdentifierIds property.</p></li> <li><p>INCLUDE - Use only the managed data identifiers specified by the managedDataIdentifierIds property.</p></li> <li><p>NONE - Don't use any managed data identifiers. If you specify this value, specify at least one value for the customDataIdentifierIds property and don't specify any values for the managedDataIdentifierIds property.</p></li> <li><p>RECOMMENDED (default) - Use the recommended set of managed data identifiers. If you specify this value, don't specify any values for the managedDataIdentifierIds property.</p></li></ul> <p>If you don't specify a value for this property, the job uses the recommended set of managed data identifiers.</p> <p>If the job is a recurring job and you specify ALL or EXCLUDE, each job run automatically uses new managed data identifiers that are released. If you don't specify a value for this property or you specify RECOMMENDED for a recurring job, each job run automatically uses all the managed data identifiers that are in the recommended set when the run starts.</p> <p>To learn about individual managed data identifiers or determine which ones are in the recommended set, see <a href="https://docs.aws.amazon.com/macie/latest/user/managed-data-identifiers.html">Using managed data identifiers</a> or <a href="https://docs.aws.amazon.com/macie/latest/user/discovery-jobs-mdis-recommended.html">Recommended managed data identifiers</a> in the <i>Amazon Macie User Guide</i>.</p> |
| `managed_data_identifier_ids` | Vec<String> |  | <p>An array of unique identifiers, one for each managed data identifier for the job to include (use) or exclude (not use) when it analyzes data. Inclusion or exclusion depends on the managed data identifier selection type that you specify for the job (managedDataIdentifierSelector).</p> <p>To retrieve a list of valid values for this property, use the ListManagedDataIdentifiers operation.</p> |
| `s3_job_definition` | String | ✅ | <p>The S3 buckets that contain the objects to analyze, and the scope of that analysis.</p> |
| `sampling_percentage` | i64 |  | <p>The sampling depth, as a percentage, for the job to apply when processing objects. This value determines the percentage of eligible objects that the job analyzes. If this value is less than 100, Amazon Macie selects the objects to analyze at random, up to the specified percentage, and analyzes all the data in those objects.</p> |
| `name` | String | ✅ | <p>A custom name for the job. The name can contain as many as 500 characters.</p> |
| `initial_run` | bool |  | <p>For a recurring job, specifies whether to analyze all existing, eligible objects immediately after the job is created (true). To analyze only those objects that are created or changed after you create the job and before the job's first scheduled run, set this value to false.</p> <p>If you configure the job to run only once, don't specify a value for this property.</p> |
| `schedule_frequency` | String |  | <p>The recurrence pattern for running the job. To run the job only once, don't specify a value for this property and set the value for the jobType property to ONE_TIME.</p> |
| `tags` | HashMap<String, String> |  | <p>A map of key-value pairs that specifies the tags to associate with the job.</p> <p>A job can have a maximum of 50 tags. Each tag consists of a tag key and an associated tag value. The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.</p> |
| `client_token` | String | ✅ | <p>A unique, case-sensitive token that you provide to ensure the idempotency of the request.</p> |
| `job_type` | String | ✅ | <p>The schedule for running the job. Valid values are:</p> <ul><li><p>ONE_TIME - Run the job only once. If you specify this value, don't specify a value for the scheduleFrequency property.</p></li> <li><p>SCHEDULED - Run the job on a daily, weekly, or monthly basis. If you specify this value, use the scheduleFrequency property to specify the recurrence pattern for the job.</p></li></ul> |
| `allow_list_ids` | Vec<String> |  | <p>An array of unique identifiers, one for each allow list for the job to use when it analyzes data.</p> |
| `custom_data_identifier_ids` | Vec<String> |  | <p>An array of unique identifiers, one for each custom data identifier for the job to use when it analyzes data. To use only managed data identifiers, don't specify a value for this property and specify a value other than NONE for the managedDataIdentifierSelector property.</p> |
| `description` | String |  | <p>A custom description of the job. The description can contain as many as 200 characters.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `s3_job_definition` | String | <p>The S3 buckets that contain the objects to analyze, and the scope of that analysis.</p> |
| `last_run_time` | String | <p>The date and time, in UTC and extended ISO 8601 format, when the job started. If the job is a recurring job, this value indicates when the most recent run started or, if the job hasn't run yet, when the job was created.</p> |
| `statistics` | String | <p>The number of times that the job has run and processing statistics for the job's current run.</p> |
| `last_run_error_status` | String | <p>Specifies whether any account- or bucket-level access errors occurred when the job ran. For a recurring job, this value indicates the error status of the job's most recent run.</p> |
| `initial_run` | bool | <p>For a recurring job, specifies whether you configured the job to analyze all existing, eligible objects immediately after the job was created (true). If you configured the job to analyze only those objects that were created or changed after the job was created and before the job's first scheduled run, this value is false. This value is also false for a one-time job.</p> |
| `job_status` | String | <p>The current status of the job. Possible values are:</p> <ul><li><p>CANCELLED - You cancelled the job or, if it's a one-time job, you paused the job and didn't resume it within 30 days.</p></li> <li><p>COMPLETE - For a one-time job, Amazon Macie finished processing the data specified for the job. This value doesn't apply to recurring jobs.</p></li> <li><p>IDLE - For a recurring job, the previous scheduled run is complete and the next scheduled run is pending. This value doesn't apply to one-time jobs.</p></li> <li><p>PAUSED - Macie started running the job but additional processing would exceed the monthly sensitive data discovery quota for your account or one or more member accounts that the job analyzes data for.</p></li> <li><p>RUNNING - For a one-time job, the job is in progress. For a recurring job, a scheduled run is in progress.</p></li> <li><p>USER_PAUSED - You paused the job. If you paused the job while it had a status of RUNNING and you don't resume it within 30 days of pausing it, the job or job run will expire and be cancelled, depending on the job's type. To check the expiration date, refer to the UserPausedDetails.jobExpiresAt property.</p></li></ul> |
| `name` | String | <p>The custom name of the job.</p> |
| `job_type` | String | <p>The schedule for running the job. Possible values are:</p> <ul><li><p>ONE_TIME - The job runs only once.</p></li> <li><p>SCHEDULED - The job runs on a daily, weekly, or monthly basis. The scheduleFrequency property indicates the recurrence pattern for the job.</p></li></ul> |
| `job_id` | String | <p>The unique identifier for the job.</p> |
| `sampling_percentage` | i64 | <p>The sampling depth, as a percentage, that determines the percentage of eligible objects that the job analyzes.</p> |
| `custom_data_identifier_ids` | Vec<String> | <p>An array of unique identifiers, one for each custom data identifier that the job is configured to use when it analyzes data. This value is null if the job is configured to use only managed data identifiers to analyze data.</p> |
| `job_arn` | String | <p>The Amazon Resource Name (ARN) of the job.</p> |
| `schedule_frequency` | String | <p>The recurrence pattern for running the job. This value is null if the job is configured to run only once.</p> |
| `tags` | HashMap<String, String> | <p>A map of key-value pairs that specifies which tags (keys and values) are associated with the job.</p> |
| `allow_list_ids` | Vec<String> | <p>An array of unique identifiers, one for each allow list that the job is configured to use when it analyzes data.</p> |
| `managed_data_identifier_ids` | Vec<String> | <p>An array of unique identifiers, one for each managed data identifier that the job is explicitly configured to include (use) or exclude (not use) when it analyzes data. Inclusion or exclusion depends on the managed data identifier selection type specified for the job (managedDataIdentifierSelector).</p><p>This value is null if the job's managed data identifier selection type is ALL, NONE, or RECOMMENDED.</p> |
| `user_paused_details` | String | <p>If the current status of the job is USER_PAUSED, specifies when the job was paused and when the job or job run will expire and be cancelled if it isn't resumed. This value is present only if the value for jobStatus is USER_PAUSED.</p> |
| `managed_data_identifier_selector` | String | <p>The selection type that determines which managed data identifiers the job uses when it analyzes data. Possible values are:</p> <ul><li><p>ALL - Use all managed data identifiers.</p></li> <li><p>EXCLUDE - Use all managed data identifiers except the ones specified by the managedDataIdentifierIds property.</p></li> <li><p>INCLUDE - Use only the managed data identifiers specified by the managedDataIdentifierIds property.</p></li> <li><p>NONE - Don't use any managed data identifiers. Use only custom data identifiers (customDataIdentifierIds).</p></li> <li><p>RECOMMENDED (default) - Use the recommended set of managed data identifiers.</p></li></ul> <p>If this value is null, the job uses the recommended set of managed data identifiers.</p> <p>If the job is a recurring job and this value is ALL or EXCLUDE, each job run automatically uses new managed data identifiers that are released. If this value is null or RECOMMENDED for a recurring job, each job run uses all the managed data identifiers that are in the recommended set when the run starts.</p> <p>To learn about individual managed data identifiers or determine which ones are in the recommended set, see <a href="https://docs.aws.amazon.com/macie/latest/user/managed-data-identifiers.html">Using managed data identifiers</a> or <a href="https://docs.aws.amazon.com/macie/latest/user/discovery-jobs-mdis-recommended.html">Recommended managed data identifiers</a> in the <i>Amazon Macie User Guide</i>.</p> |
| `created_at` | String | <p>The date and time, in UTC and extended ISO 8601 format, when the job was created.</p> |
| `description` | String | <p>The custom description of the job.</p> |
| `client_token` | String | <p>The token that was provided to ensure the idempotency of the request to create the job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create classification_job
classification_job = provider.macie2.Classification_job {
    s3_job_definition = "value"  # <p>The S3 buckets that contain the objects to analyze, and the scope of that analysis.</p>
    name = "value"  # <p>A custom name for the job. The name can contain as many as 500 characters.</p>
    client_token = "value"  # <p>A unique, case-sensitive token that you provide to ensure the idempotency of the request.</p>
    job_type = "value"  # <p>The schedule for running the job. Valid values are:</p> <ul><li><p>ONE_TIME - Run the job only once. If you specify this value, don't specify a value for the scheduleFrequency property.</p></li> <li><p>SCHEDULED - Run the job on a daily, weekly, or monthly basis. If you specify this value, use the scheduleFrequency property to specify the recurrence pattern for the job.</p></li></ul>
}

# Access classification_job outputs
classification_job_id = classification_job.id
classification_job_s3_job_definition = classification_job.s3_job_definition
classification_job_last_run_time = classification_job.last_run_time
classification_job_statistics = classification_job.statistics
classification_job_last_run_error_status = classification_job.last_run_error_status
classification_job_initial_run = classification_job.initial_run
classification_job_job_status = classification_job.job_status
classification_job_name = classification_job.name
classification_job_job_type = classification_job.job_type
classification_job_job_id = classification_job.job_id
classification_job_sampling_percentage = classification_job.sampling_percentage
classification_job_custom_data_identifier_ids = classification_job.custom_data_identifier_ids
classification_job_job_arn = classification_job.job_arn
classification_job_schedule_frequency = classification_job.schedule_frequency
classification_job_tags = classification_job.tags
classification_job_allow_list_ids = classification_job.allow_list_ids
classification_job_managed_data_identifier_ids = classification_job.managed_data_identifier_ids
classification_job_user_paused_details = classification_job.user_paused_details
classification_job_managed_data_identifier_selector = classification_job.managed_data_identifier_selector
classification_job_created_at = classification_job.created_at
classification_job_description = classification_job.description
classification_job_client_token = classification_job.client_token
```

---


### Automated_discovery_configuration

AutomatedDiscoveryConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auto_enable_organization_members` | String |  | <p>Specifies whether to automatically enable automated sensitive data discovery for accounts in the organization. Valid values are: ALL (default), enable it for all existing accounts and new member accounts; NEW, enable it only for new member accounts; and, NONE, don't enable it for any accounts.</p> <p>If you specify NEW or NONE, automated sensitive data discovery continues to be enabled for any existing accounts that it's currently enabled for. To enable or disable it for individual member accounts, specify NEW or NONE, and then enable or disable it for each account by using the BatchUpdateAutomatedDiscoveryAccounts operation.</p> |
| `status` | String | ✅ | <p>The new status of automated sensitive data discovery for the organization or account. Valid values are: ENABLED, start or resume all automated sensitive data discovery activities; and, DISABLED, stop performing all automated sensitive data discovery activities.</p> <p>If you specify DISABLED for an administrator account, you also disable automated sensitive data discovery for all member accounts in the organization.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auto_enable_organization_members` | String | <p>Specifies whether automated sensitive data discovery is enabled automatically for accounts in the organization. Possible values are: ALL, enable it for all existing accounts and new member accounts; NEW, enable it only for new member accounts; and, NONE, don't enable it for any accounts.</p> |
| `disabled_at` | String | <p>The date and time, in UTC and extended ISO 8601 format, when automated sensitive data discovery was most recently disabled. This value is null if automated sensitive data discovery is currently enabled.</p> |
| `classification_scope_id` | String | <p>The unique identifier for the classification scope that's used when performing automated sensitive data discovery. The classification scope specifies S3 buckets to exclude from analyses.</p> |
| `first_enabled_at` | String | <p>The date and time, in UTC and extended ISO 8601 format, when automated sensitive data discovery was initially enabled. This value is null if automated sensitive data discovery has never been enabled.</p> |
| `last_updated_at` | String | <p>The date and time, in UTC and extended ISO 8601 format, when the configuration settings or status of automated sensitive data discovery was most recently changed.</p> |
| `sensitivity_inspection_template_id` | String | <p>The unique identifier for the sensitivity inspection template that's used when performing automated sensitive data discovery. The template specifies which allow lists, custom data identifiers, and managed data identifiers to use when analyzing data.</p> |
| `status` | String | <p>The current status of automated sensitive data discovery for the organization or account. Possible values are: ENABLED, use the specified settings to perform automated sensitive data discovery activities; and, DISABLED, don't perform automated sensitive data discovery activities.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access automated_discovery_configuration outputs
automated_discovery_configuration_id = automated_discovery_configuration.id
automated_discovery_configuration_auto_enable_organization_members = automated_discovery_configuration.auto_enable_organization_members
automated_discovery_configuration_disabled_at = automated_discovery_configuration.disabled_at
automated_discovery_configuration_classification_scope_id = automated_discovery_configuration.classification_scope_id
automated_discovery_configuration_first_enabled_at = automated_discovery_configuration.first_enabled_at
automated_discovery_configuration_last_updated_at = automated_discovery_configuration.last_updated_at
automated_discovery_configuration_sensitivity_inspection_template_id = automated_discovery_configuration.sensitivity_inspection_template_id
automated_discovery_configuration_status = automated_discovery_configuration.status
```

---


### Organization_configuration

OrganizationConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auto_enable` | bool | ✅ | <p>Specifies whether to enable Amazon Macie automatically for accounts that are added to the organization in Organizations.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `max_account_limit_reached` | bool | <p>Specifies whether the maximum number of Amazon Macie member accounts are part of the organization.</p> |
| `auto_enable` | bool | <p>Specifies whether Amazon Macie is enabled automatically for accounts that are added to the organization.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organization_configuration outputs
organization_configuration_id = organization_configuration.id
organization_configuration_max_account_limit_reached = organization_configuration.max_account_limit_reached
organization_configuration_auto_enable = organization_configuration.auto_enable
```

---


### Findings_publication_configuration

FindingsPublicationConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `security_hub_configuration` | String |  | <p>The configuration settings that determine which findings to publish to Security Hub.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive token that you provide to ensure the idempotency of the request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `security_hub_configuration` | String | <p>The configuration settings that determine which findings are published to Security Hub.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create findings_publication_configuration
findings_publication_configuration = provider.macie2.Findings_publication_configuration {
}

# Access findings_publication_configuration outputs
findings_publication_configuration_id = findings_publication_configuration.id
findings_publication_configuration_security_hub_configuration = findings_publication_configuration.security_hub_configuration
```

---


### Findings_filter

FindingsFilter resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>A custom name for the filter. The name must contain at least 3 characters and can contain as many as 64 characters.</p> <p>We strongly recommend that you avoid including any sensitive data in the name of a filter. Other users of your account might be able to see this name, depending on the actions that they're allowed to perform in Amazon Macie.</p> |
| `position` | i64 |  | <p>The position of the filter in the list of saved filters on the Amazon Macie console. This value also determines the order in which the filter is applied to findings, relative to other filters that are also applied to the findings.</p> |
| `action` | String | ✅ | <p>The action to perform on findings that match the filter criteria (findingCriteria). Valid values are: ARCHIVE, suppress (automatically archive) the findings; and, NOOP, don't perform any action on the findings.</p> |
| `tags` | HashMap<String, String> |  | <p>A map of key-value pairs that specifies the tags to associate with the filter.</p> <p>A findings filter can have a maximum of 50 tags. Each tag consists of a tag key and an associated tag value. The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.</p> |
| `description` | String |  | <p>A custom description of the filter. The description can contain as many as 512 characters.</p> <p>We strongly recommend that you avoid including any sensitive data in the description of a filter. Other users of your account might be able to see this description, depending on the actions that they're allowed to perform in Amazon Macie.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive token that you provide to ensure the idempotency of the request.</p> |
| `finding_criteria` | String | ✅ | <p>The criteria to use to filter findings.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | <p>The unique identifier for the filter.</p> |
| `action` | String | <p>The action that's performed on findings that match the filter criteria (findingCriteria). Possible values are: ARCHIVE, suppress (automatically archive) the findings; and, NOOP, don't perform any action on the findings.</p> |
| `finding_criteria` | String | <p>The criteria that's used to filter findings.</p> |
| `description` | String | <p>The custom description of the filter.</p> |
| `tags` | HashMap<String, String> | <p>A map of key-value pairs that specifies which tags (keys and values) are associated with the filter.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the filter.</p> |
| `name` | String | <p>The custom name of the filter.</p> |
| `position` | i64 | <p>The position of the filter in the list of saved filters on the Amazon Macie console. This value also determines the order in which the filter is applied to findings, relative to other filters that are also applied to the findings.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create findings_filter
findings_filter = provider.macie2.Findings_filter {
    name = "value"  # <p>A custom name for the filter. The name must contain at least 3 characters and can contain as many as 64 characters.</p> <p>We strongly recommend that you avoid including any sensitive data in the name of a filter. Other users of your account might be able to see this name, depending on the actions that they're allowed to perform in Amazon Macie.</p>
    action = "value"  # <p>The action to perform on findings that match the filter criteria (findingCriteria). Valid values are: ARCHIVE, suppress (automatically archive) the findings; and, NOOP, don't perform any action on the findings.</p>
    finding_criteria = "value"  # <p>The criteria to use to filter findings.</p>
}

# Access findings_filter outputs
findings_filter_id = findings_filter.id
findings_filter_id = findings_filter.id
findings_filter_action = findings_filter.action
findings_filter_finding_criteria = findings_filter.finding_criteria
findings_filter_description = findings_filter.description
findings_filter_tags = findings_filter.tags
findings_filter_arn = findings_filter.arn
findings_filter_name = findings_filter.name
findings_filter_position = findings_filter.position
```

---


### Resource_profile_detections

ResourceProfileDetections resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `suppress_data_identifiers` | Vec<String> |  | <p>An array of objects, one for each custom data identifier or managed data identifier that detected a type of sensitive data to exclude from the bucket's score. To include all sensitive data types in the score, don't specify any values for this array.</p> |
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the S3 bucket that the request applies to.</p> |



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


### Usage_statistics

UsageStatistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `time_range` | String | <p>The inclusive time period that the usage data applies to. Possible values are: MONTH_TO_DATE, for the current calendar month to date; and, PAST_30_DAYS, for the preceding 30 days.</p> |
| `next_token` | String | <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null if there are no additional pages.</p> |
| `records` | Vec<String> | <p>An array of objects that contains the results of the query. Each object contains the data for an account that matches the filter criteria specified in the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access usage_statistics outputs
usage_statistics_id = usage_statistics.id
usage_statistics_time_range = usage_statistics.time_range
usage_statistics_next_token = usage_statistics.next_token
usage_statistics_records = usage_statistics.records
```

---


### Sensitivity_inspection_template

SensitivityInspectionTemplate resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `includes` | String |  | <p>The allow lists, custom data identifiers, and managed data identifiers to explicitly include (use) when performing automated sensitive data discovery.</p> |
| `id` | String | ✅ | <p>The unique identifier for the Amazon Macie resource that the request applies to.</p> |
| `excludes` | String |  | <p>The managed data identifiers to explicitly exclude (not use) when performing automated sensitive data discovery.</p> <p>To exclude an allow list or custom data identifier that's currently included by the template, update the values for the SensitivityInspectionTemplateIncludes.allowListIds and SensitivityInspectionTemplateIncludes.customDataIdentifierIds properties, respectively.</p> |
| `description` | String |  | <p>A custom description of the template. The description can contain as many as 200 characters.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the template: automated-sensitive-data-discovery.</p> |
| `excludes` | String | <p>The managed data identifiers that are explicitly excluded (not used) when performing automated sensitive data discovery.</p> |
| `includes` | String | <p>The allow lists, custom data identifiers, and managed data identifiers that are explicitly included (used) when performing automated sensitive data discovery.</p> |
| `description` | String | <p>The custom description of the template.</p> |
| `sensitivity_inspection_template_id` | String | <p>The unique identifier for the template.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sensitivity_inspection_template outputs
sensitivity_inspection_template_id = sensitivity_inspection_template.id
sensitivity_inspection_template_name = sensitivity_inspection_template.name
sensitivity_inspection_template_excludes = sensitivity_inspection_template.excludes
sensitivity_inspection_template_includes = sensitivity_inspection_template.includes
sensitivity_inspection_template_description = sensitivity_inspection_template.description
sensitivity_inspection_template_sensitivity_inspection_template_id = sensitivity_inspection_template.sensitivity_inspection_template_id
```

---


### Bucket_statistics

BucketStatistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bucket_count_by_object_encryption_requirement` | String | <p>The total number of buckets whose bucket policies do or don't require server-side encryption of objects when objects are added to the buckets.</p> |
| `classifiable_object_count` | i64 | <p>The total number of objects that Amazon Macie can analyze in the buckets. These objects use a supported storage class and have a file name extension for a supported file or storage format.</p> |
| `classifiable_size_in_bytes` | i64 | <p>The total storage size, in bytes, of all the objects that Amazon Macie can analyze in the buckets. These objects use a supported storage class and have a file name extension for a supported file or storage format.</p> <p>If versioning is enabled for any of the buckets, this value is based on the size of the latest version of each applicable object in the buckets. This value doesn't reflect the storage size of all versions of all applicable objects in the buckets.</p> |
| `bucket_count` | i64 | <p>The total number of buckets.</p> |
| `bucket_count_by_encryption_type` | String | <p>The total number of buckets whose settings do or don't specify default server-side encryption behavior for objects that are added to the buckets.</p> |
| `last_updated` | String | <p>The date and time, in UTC and extended ISO 8601 format, when Amazon Macie most recently retrieved bucket or object metadata from Amazon S3 for the buckets.</p> |
| `object_count` | i64 | <p>The total number of objects in the buckets.</p> |
| `bucket_count_by_shared_access_type` | String | <p>The total number of buckets that are or aren't shared with other Amazon Web Services accounts, Amazon CloudFront origin access identities (OAIs), or CloudFront origin access controls (OACs).</p> |
| `bucket_statistics_by_sensitivity` | String | <p>The aggregated sensitive data discovery statistics for the buckets. If automated sensitive data discovery is currently disabled for your account, the value for most statistics is 0.</p> |
| `size_in_bytes_compressed` | i64 | <p>The total storage size, in bytes, of the objects that are compressed (.gz, .gzip, .zip) files in the buckets.</p> <p>If versioning is enabled for any of the buckets, this value is based on the size of the latest version of each applicable object in the buckets. This value doesn't reflect the storage size of all versions of the applicable objects in the buckets.</p> |
| `unclassifiable_object_count` | String | <p>The total number of objects that Amazon Macie can't analyze in the buckets. These objects don't use a supported storage class or don't have a file name extension for a supported file or storage format.</p> |
| `unclassifiable_object_size_in_bytes` | String | <p>The total storage size, in bytes, of the objects that Amazon Macie can't analyze in the buckets. These objects don't use a supported storage class or don't have a file name extension for a supported file or storage format.</p> |
| `size_in_bytes` | i64 | <p>The total storage size, in bytes, of the buckets.</p> <p>If versioning is enabled for any of the buckets, this value is based on the size of the latest version of each object in the buckets. This value doesn't reflect the storage size of all versions of the objects in the buckets.</p> |
| `bucket_count_by_effective_permission` | String | <p>The total number of buckets that are publicly accessible due to a combination of permissions settings for each bucket.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access bucket_statistics outputs
bucket_statistics_id = bucket_statistics.id
bucket_statistics_bucket_count_by_object_encryption_requirement = bucket_statistics.bucket_count_by_object_encryption_requirement
bucket_statistics_classifiable_object_count = bucket_statistics.classifiable_object_count
bucket_statistics_classifiable_size_in_bytes = bucket_statistics.classifiable_size_in_bytes
bucket_statistics_bucket_count = bucket_statistics.bucket_count
bucket_statistics_bucket_count_by_encryption_type = bucket_statistics.bucket_count_by_encryption_type
bucket_statistics_last_updated = bucket_statistics.last_updated
bucket_statistics_object_count = bucket_statistics.object_count
bucket_statistics_bucket_count_by_shared_access_type = bucket_statistics.bucket_count_by_shared_access_type
bucket_statistics_bucket_statistics_by_sensitivity = bucket_statistics.bucket_statistics_by_sensitivity
bucket_statistics_size_in_bytes_compressed = bucket_statistics.size_in_bytes_compressed
bucket_statistics_unclassifiable_object_count = bucket_statistics.unclassifiable_object_count
bucket_statistics_unclassifiable_object_size_in_bytes = bucket_statistics.unclassifiable_object_size_in_bytes
bucket_statistics_size_in_bytes = bucket_statistics.size_in_bytes
bucket_statistics_bucket_count_by_effective_permission = bucket_statistics.bucket_count_by_effective_permission
```

---


### Buckets

Buckets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `buckets` | Vec<String> | <p>An array of objects, one for each bucket that matches the filter criteria specified in the request.</p> |
| `next_token` | String | <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null if there are no additional pages.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access buckets outputs
buckets_id = buckets.id
buckets_buckets = buckets.buckets
buckets_next_token = buckets.next_token
```

---


### Classification_scope

ClassificationScope resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `s3` | String |  | <p>The S3 buckets to add or remove from the exclusion list defined by the classification scope.</p> |
| `id` | String | ✅ | <p>The unique identifier for the Amazon Macie resource that the request applies to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the classification scope: automated-sensitive-data-discovery.</p> |
| `id` | String | <p>The unique identifier for the classification scope.</p> |
| `s3` | String | <p>The S3 buckets that are excluded from automated sensitive data discovery.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access classification_scope outputs
classification_scope_id = classification_scope.id
classification_scope_name = classification_scope.name
classification_scope_id = classification_scope.id
classification_scope_s3 = classification_scope.s3
```

---


### Classification_export_configuration

ClassificationExportConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration` | String | ✅ | <p>The location to store data classification results in, and the encryption settings to use when storing results in that location.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration` | String | <p>The location where data classification results are stored, and the encryption settings that are used when storing results in that location.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create classification_export_configuration
classification_export_configuration = provider.macie2.Classification_export_configuration {
    configuration = "value"  # <p>The location to store data classification results in, and the encryption settings to use when storing results in that location.</p>
}

# Access classification_export_configuration outputs
classification_export_configuration_id = classification_export_configuration.id
classification_export_configuration_configuration = classification_export_configuration.configuration
```

---


### Allow_list

AllowList resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String | ✅ | <p>A unique, case-sensitive token that you provide to ensure the idempotency of the request.</p> |
| `name` | String | ✅ | <p>A custom name for the allow list. The name can contain as many as 128 characters.</p> |
| `description` | String |  | <p>A custom description of the allow list. The description can contain as many as 512 characters.</p> |
| `tags` | HashMap<String, String> |  | <p>A map of key-value pairs that specifies the tags to associate with the allow list.</p> <p>An allow list can have a maximum of 50 tags. Each tag consists of a tag key and an associated tag value. The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.</p> |
| `criteria` | String | ✅ | <p>The criteria that specify the text or text pattern to ignore. The criteria can be the location and name of an S3 object that lists specific text to ignore (s3WordsList), or a regular expression (regex) that defines a text pattern to ignore.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `criteria` | String | <p>The criteria that specify the text or text pattern to ignore. The criteria can be the location and name of an S3 object that lists specific text to ignore (s3WordsList), or a regular expression (regex) that defines a text pattern to ignore.</p> |
| `id` | String | <p>The unique identifier for the allow list.</p> |
| `updated_at` | String | <p>The date and time, in UTC and extended ISO 8601 format, when the allow list's settings were most recently changed in Amazon Macie.</p> |
| `tags` | HashMap<String, String> | <p>A map of key-value pairs that specifies which tags (keys and values) are associated with the allow list.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the allow list.</p> |
| `created_at` | String | <p>The date and time, in UTC and extended ISO 8601 format, when the allow list was created in Amazon Macie.</p> |
| `description` | String | <p>The custom description of the allow list.</p> |
| `name` | String | <p>The custom name of the allow list.</p> |
| `status` | String | <p>The current status of the allow list, which indicates whether Amazon Macie can access and use the list's criteria.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create allow_list
allow_list = provider.macie2.Allow_list {
    client_token = "value"  # <p>A unique, case-sensitive token that you provide to ensure the idempotency of the request.</p>
    name = "value"  # <p>A custom name for the allow list. The name can contain as many as 128 characters.</p>
    criteria = "value"  # <p>The criteria that specify the text or text pattern to ignore. The criteria can be the location and name of an S3 object that lists specific text to ignore (s3WordsList), or a regular expression (regex) that defines a text pattern to ignore.</p>
}

# Access allow_list outputs
allow_list_id = allow_list.id
allow_list_criteria = allow_list.criteria
allow_list_id = allow_list.id
allow_list_updated_at = allow_list.updated_at
allow_list_tags = allow_list.tags
allow_list_arn = allow_list.arn
allow_list_created_at = allow_list.created_at
allow_list_description = allow_list.description
allow_list_name = allow_list.name
allow_list_status = allow_list.status
```

---


### Reveal_configuration

RevealConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `retrieval_configuration` | String |  | <p>The access method and settings to use when retrieving the sensitive data.</p> |
| `configuration` | String | ✅ | <p>The KMS key to use to encrypt the sensitive data, and the status of the configuration for the Amazon Macie account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `retrieval_configuration` | String | <p>The access method and settings that are used to retrieve the sensitive data.</p> |
| `configuration` | String | <p>The KMS key that's used to encrypt the sensitive data, and the status of the configuration for the Amazon Macie account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reveal_configuration outputs
reveal_configuration_id = reveal_configuration.id
reveal_configuration_retrieval_configuration = reveal_configuration.retrieval_configuration
reveal_configuration_configuration = reveal_configuration.configuration
```

---


### Administrator_account

AdministratorAccount resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `administrator` | String | <p>The Amazon Web Services account ID for the administrator account. If the accounts are associated by an Amazon Macie membership invitation, this object also provides details about the invitation that was sent to establish the relationship between the accounts.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access administrator_account outputs
administrator_account_id = administrator_account.id
administrator_account_administrator = administrator_account.administrator
```

---


### Finding_statistics

FindingStatistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `counts_by_group` | Vec<String> | <p>An array of objects, one for each group of findings that matches the filter criteria specified in the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access finding_statistics outputs
finding_statistics_id = finding_statistics.id
finding_statistics_counts_by_group = finding_statistics.counts_by_group
```

---


### Invitations_count

InvitationsCount resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `invitations_count` | i64 | <p>The total number of invitations that were received by the account, not including the currently accepted invitation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access invitations_count outputs
invitations_count_id = invitations_count.id
invitations_count_invitations_count = invitations_count.invitations_count
```

---


### Usage_totals

UsageTotals resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `time_range` | String | <p>The inclusive time period that the usage data applies to. Possible values are: MONTH_TO_DATE, for the current calendar month to date; and, PAST_30_DAYS, for the preceding 30 days.</p> |
| `usage_totals` | Vec<String> | <p>An array of objects that contains the results of the query. Each object contains the data for a specific usage metric.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access usage_totals outputs
usage_totals_id = usage_totals.id
usage_totals_time_range = usage_totals.time_range
usage_totals_usage_totals = usage_totals.usage_totals
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple sample_findings resources
sample_findings_0 = provider.macie2.Sample_findings {
}
sample_findings_1 = provider.macie2.Sample_findings {
}
sample_findings_2 = provider.macie2.Sample_findings {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    sample_findings = provider.macie2.Sample_findings {
    }
```

---

## Related Documentation

- [AWS Macie2 Documentation](https://docs.aws.amazon.com/macie2/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
