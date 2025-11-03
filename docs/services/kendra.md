# Kendra Service



**Resources**: 12

---

## Overview

The kendra service provides access to 12 resource types:

- [Thesaurus](#thesaurus) [CRUD]
- [Principal_mapping](#principal_mapping) [CRD]
- [Index](#index) [CRUD]
- [Faq](#faq) [CRD]
- [Data_source](#data_source) [CRUD]
- [Query_suggestions_block_list](#query_suggestions_block_list) [CRUD]
- [Snapshots](#snapshots) [R]
- [Featured_results_set](#featured_results_set) [CRU]
- [Query_suggestions_config](#query_suggestions_config) [RU]
- [Query_suggestions](#query_suggestions) [R]
- [Access_control_configuration](#access_control_configuration) [CRUD]
- [Experience](#experience) [CRUD]

---

## Resources


### Thesaurus

Thesaurus resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role with permission 
         to access your S3 bucket that contains the thesaurus file. For more information, 
         see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM 
            access roles for Amazon Kendra</a>.</p> |
| `tags` | Vec<String> |  | <p>A list of key-value pairs that identify or categorize the thesaurus. You can 
         also use tags to help control access to the thesaurus. Tag keys and values can 
         consist of Unicode letters, digits, white space, and any of the following 
         symbols: _ . : / = + - @.</p> |
| `client_token` | String |  | <p>A token that you provide to identify the request to create a 
         thesaurus. Multiple calls to the <code>CreateThesaurus</code> API 
         with the same client token will create only one thesaurus.
      </p> |
| `source_s3_path` | String | ✅ | <p>The path to the thesaurus file in S3.</p> |
| `index_id` | String | ✅ | <p>The identifier of the index for the thesaurus.</p> |
| `name` | String | ✅ | <p>A name for the thesaurus.</p> |
| `description` | String |  | <p>A description for the thesaurus.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `source_s3_path` | String |  |
| `id` | String | <p>The identifier of the thesaurus.</p> |
| `index_id` | String | <p>The identifier of the index for the thesaurus.</p> |
| `error_message` | String | <p>When the <code>Status</code> field value is <code>FAILED</code>, the
         <code>ErrorMessage</code> field provides more information.
      </p> |
| `name` | String | <p>The thesaurus name.</p> |
| `status` | String | <p>The current status of the thesaurus. When the value is <code>ACTIVE</code>, 
         queries are able to use the thesaurus. If the <code>Status</code> field value 
         is <code>FAILED</code>, the <code>ErrorMessage</code> field provides
         more information.
      </p>
         <p>If the status is <code>ACTIVE_BUT_UPDATE_FAILED</code>, it means
         that Amazon Kendra could not ingest the new thesaurus file. The old
      thesaurus file is still active.
      </p> |
| `created_at` | String | <p>The Unix timestamp when the thesaurus was created.</p> |
| `description` | String | <p>The thesaurus description.</p> |
| `updated_at` | String | <p>The Unix timestamp when the thesaurus was last updated.</p> |
| `file_size_bytes` | i64 | <p>The size of the thesaurus file in bytes.</p> |
| `synonym_rule_count` | i64 | <p>The number of synonym rules in the thesaurus file.</p> |
| `role_arn` | String | <p>An IAM role that gives Amazon Kendra permissions 
         to access thesaurus file specified in <code>SourceS3Path</code>.
      </p> |
| `term_count` | i64 | <p>The number of unique terms in the thesaurus file. For example, the
        synonyms <code>a,b,c</code> and <code>a=>d</code>, the term
        count would be 4.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create thesaurus
thesaurus = provider.kendra.Thesaurus {
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role with permission 
         to access your S3 bucket that contains the thesaurus file. For more information, 
         see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM 
            access roles for Amazon Kendra</a>.</p>
    source_s3_path = "value"  # <p>The path to the thesaurus file in S3.</p>
    index_id = "value"  # <p>The identifier of the index for the thesaurus.</p>
    name = "value"  # <p>A name for the thesaurus.</p>
}

# Access thesaurus outputs
thesaurus_id = thesaurus.id
thesaurus_source_s3_path = thesaurus.source_s3_path
thesaurus_id = thesaurus.id
thesaurus_index_id = thesaurus.index_id
thesaurus_error_message = thesaurus.error_message
thesaurus_name = thesaurus.name
thesaurus_status = thesaurus.status
thesaurus_created_at = thesaurus.created_at
thesaurus_description = thesaurus.description
thesaurus_updated_at = thesaurus.updated_at
thesaurus_file_size_bytes = thesaurus.file_size_bytes
thesaurus_synonym_rule_count = thesaurus.synonym_rule_count
thesaurus_role_arn = thesaurus.role_arn
thesaurus_term_count = thesaurus.term_count
```

---


### Principal_mapping

PrincipalMapping resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `group_members` | String | ✅ | <p>The list that contains your users that belong the same group. This can include sub groups 
            that belong to a group.</p>
         <p>For example, the group "Company A" includes the user "CEO" and the sub groups
            "Research", "Engineering", and "Sales and Marketing".</p>
         <p>If you have more than 1000 users and/or sub groups for a single group, you need to
            provide the path to the S3 file that lists your users and sub groups for a group. Your
            sub groups can contain more than 1000 users, but the list of sub groups that belong to a
            group (and/or users) must be no more than 1000.</p> |
| `ordering_id` | i64 |  | <p>The timestamp identifier you specify to ensure Amazon Kendra doesn't override
            the latest <code>PUT</code> action with previous actions. The highest number ID, which
            is the ordering ID, is the latest action you want to process and apply on top of other
            actions with lower number IDs. This prevents previous actions with lower number IDs from
            possibly overriding the latest action.</p>
         <p>The ordering ID can be the Unix time of the last update you made to a group members
            list. You would then provide this list when calling <code>PutPrincipalMapping</code>.
            This ensures your <code>PUT</code> action for that updated group with the latest members
            list doesn't get overwritten by earlier <code>PUT</code> actions for the same group
            which are yet to be processed.</p>
         <p>The default ordering ID is the current Unix time in milliseconds that the action was
            received by Amazon Kendra.</p> |
| `role_arn` | String |  | <p>The Amazon Resource Name (ARN) of an IAM role that has access to the 
            S3 file that contains your list of users that belong to a group.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html#iam-roles-ds">IAM roles for 
            Amazon Kendra</a>.</p> |
| `data_source_id` | String |  | <p>The identifier of the data source you want to map users to their groups.</p>
         <p>This is useful if a group is tied to multiple data sources, but you only want the
            group to access documents of a certain data source. For example, the groups "Research",
            "Engineering", and "Sales and Marketing" are all tied to the company's documents stored
            in the data sources Confluence and Salesforce. However, "Sales and Marketing" team only
            needs access to customer-related documents stored in Salesforce.</p> |
| `group_id` | String | ✅ | <p>The identifier of the group you want to map its users to.</p> |
| `index_id` | String | ✅ | <p>The identifier of the index you want to map users to their groups.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `group_ordering_id_summaries` | Vec<String> | <p>Shows the following information on the processing of <code>PUT</code> and
                <code>DELETE</code> actions for mapping users to their groups:</p>
         <ul>
            <li>
               <p>Status—the status can be either <code>PROCESSING</code>,
                        <code>SUCCEEDED</code>, <code>DELETING</code>, <code>DELETED</code>, or
                        <code>FAILED</code>.</p>
            </li>
            <li>
               <p>Last updated—the last date-time an action was updated.</p>
            </li>
            <li>
               <p>Received—the last date-time an action was received or
                    submitted.</p>
            </li>
            <li>
               <p>Ordering ID—the latest action that should process and apply after
                    other actions.</p>
            </li>
            <li>
               <p>Failure reason—the reason an action could not be processed.</p>
            </li>
         </ul> |
| `index_id` | String | <p>Shows the identifier of the index to see information on the processing of
                <code>PUT</code> and <code>DELETE</code> actions for mapping users to their
            groups.</p> |
| `data_source_id` | String | <p>Shows the identifier of the data source to see information on the processing of
                <code>PUT</code> and <code>DELETE</code> actions for mapping users to their
            groups.</p> |
| `group_id` | String | <p>Shows the identifier of the group to see information on the processing of
                <code>PUT</code> and <code>DELETE</code> actions for mapping users to their
            groups.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create principal_mapping
principal_mapping = provider.kendra.Principal_mapping {
    group_members = "value"  # <p>The list that contains your users that belong the same group. This can include sub groups 
            that belong to a group.</p>
         <p>For example, the group "Company A" includes the user "CEO" and the sub groups
            "Research", "Engineering", and "Sales and Marketing".</p>
         <p>If you have more than 1000 users and/or sub groups for a single group, you need to
            provide the path to the S3 file that lists your users and sub groups for a group. Your
            sub groups can contain more than 1000 users, but the list of sub groups that belong to a
            group (and/or users) must be no more than 1000.</p>
    group_id = "value"  # <p>The identifier of the group you want to map its users to.</p>
    index_id = "value"  # <p>The identifier of the index you want to map users to their groups.</p>
}

# Access principal_mapping outputs
principal_mapping_id = principal_mapping.id
principal_mapping_group_ordering_id_summaries = principal_mapping.group_ordering_id_summaries
principal_mapping_index_id = principal_mapping.index_id
principal_mapping_data_source_id = principal_mapping.data_source_id
principal_mapping_group_id = principal_mapping.group_id
```

---


### Index

Index resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description for the index.</p> |
| `name` | String | ✅ | <p>A name for the index.</p> |
| `edition` | String |  | <p>The Amazon Kendra edition to use for the index. Choose <code>DEVELOPER_EDITION</code>
      for indexes intended for development, testing, or proof of concept. Use
        <code>ENTERPRISE_EDITION</code> for production. Use <code>GEN_AI_ENTERPRISE_EDITION</code>
      for creating generative AI applications. Once you set the edition for an index, it can't be
      changed. </p>
         <p>The <code>Edition</code> parameter is optional. If you don't supply a value, the default
      is <code>ENTERPRISE_EDITION</code>.</p>
         <p>For more information on quota limits for Gen AI Enterprise Edition, Enterprise Edition, and
      Developer Edition indices, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/quotas.html">Quotas</a>.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role with permission to access
      your Amazon CloudWatch logs and metrics. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM access roles
        for Amazon Kendra</a>.</p> |
| `server_side_encryption_configuration` | String |  | <p>The identifier of the KMS customer managed key (CMK) that's used to encrypt
      data indexed by Amazon Kendra. Amazon Kendra doesn't support asymmetric CMKs.</p> |
| `user_context_policy` | String |  | <p>The user context policy.</p>
         <important>
            <p>If you're using an Amazon Kendra Gen AI Enterprise Edition index, you can only use
               <code>ATTRIBUTE_FILTER</code> to filter search results by user context. If you're
            using an Amazon Kendra Gen AI Enterprise Edition index and you try to use
               <code>USER_TOKEN</code> to configure user context policy, Amazon Kendra returns a
               <code>ValidationException</code> error.</p>
         </important>
         <dl>
            <dt>ATTRIBUTE_FILTER</dt>
            <dd>
               <p>All indexed content is searchable and displayable for all users. If you want to
                  filter search results on user context, you can use the attribute filters of
                     <code>_user_id</code> and <code>_group_ids</code> or you can provide user and
                  group information in <code>UserContext</code>. </p>
            </dd>
            <dt>USER_TOKEN</dt>
            <dd>
               <p>Enables token-based user access control to filter search results on user
                  context. All documents with no access control and all documents accessible to the
                  user will be searchable and displayable. </p>
            </dd>
         </dl> |
| `client_token` | String |  | <p>A token that you provide to identify the request to create an index. Multiple calls to the
        <code>CreateIndex</code> API with the same client token will create only one index.</p> |
| `user_token_configurations` | Vec<String> |  | <p>The user token configuration.</p>
         <important>
            <p>If you're using an Amazon Kendra Gen AI Enterprise Edition index and you try to use
               <code>UserTokenConfigurations</code> to configure user context policy, Amazon Kendra returns
            a <code>ValidationException</code> error.</p>
         </important> |
| `tags` | Vec<String> |  | <p>A list of key-value pairs that identify or categorize the index. You can also use tags to
      help control access to the index. Tag keys and values can consist of Unicode letters, digits,
      white space, and any of the following symbols: _ . : / = + - @.</p> |
| `user_group_resolution_configuration` | String |  | <p>Gets users and groups from IAM Identity Center identity source. To configure this,
         see <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_UserGroupResolutionConfiguration.html">UserGroupResolutionConfiguration</a>. This is useful for user context filtering,
         where search results are filtered based on the user or their group access to
         documents.</p>
         <important>
            <p>If you're using an Amazon Kendra Gen AI Enterprise Edition index,
               <code>UserGroupResolutionConfiguration</code> isn't supported.</p>
         </important> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The current status of the index. When the value is <code>ACTIVE</code>, the index is ready
      for use. If the <code>Status</code> field value is <code>FAILED</code>, the
        <code>ErrorMessage</code> field contains a message that explains why.</p> |
| `created_at` | String | <p>The Unix timestamp when the index was created.</p> |
| `capacity_units` | String | <p>For Enterprise Edition indexes, you can choose to use additional capacity to meet the
      needs of your application. This contains the capacity units used for the index. A query or
      document storage capacity of zero indicates that the index is using the default capacity. For
      more information on the default capacity for an index and adjusting this, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/adjusting-capacity.html">Adjusting
        capacity</a>.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role that gives Amazon Kendra permission to write to your Amazon CloudWatch logs.</p> |
| `user_context_policy` | String | <p>The user context policy for the Amazon Kendra index.</p> |
| `description` | String | <p>The description for the index.</p> |
| `user_token_configurations` | Vec<String> | <p>The user token configuration for the Amazon Kendra index.</p> |
| `name` | String | <p>The name of the index.</p> |
| `id` | String | <p>The identifier of the index.</p> |
| `error_message` | String | <p>When the <code>Status</code> field value is <code>FAILED</code>, the
        <code>ErrorMessage</code> field contains a message that explains why.</p> |
| `document_metadata_configurations` | Vec<String> | <p>Configuration information for document metadata or fields. Document metadata are fields or
      attributes associated with your documents. For example, the company department name associated
      with each document.</p> |
| `server_side_encryption_configuration` | String | <p>The identifier of the KMS customer master key (CMK) that is used to encrypt
      your data. Amazon Kendra doesn't support asymmetric CMKs.</p> |
| `user_group_resolution_configuration` | String | <p>Whether you have enabled IAM Identity Center identity source for your users and
         groups. This is useful for user context filtering, where search results are filtered based
         on the user or their group access to documents.</p> |
| `updated_at` | String | <p>The Unix timestamp when the index was last updated.</p> |
| `edition` | String | <p>The Amazon Kendra edition used for the index. You decide the edition when you create
      the index.</p> |
| `index_statistics` | String | <p>Provides information about the number of FAQ questions and answers and the number of text
      documents indexed.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create index
index = provider.kendra.Index {
    name = "value"  # <p>A name for the index.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role with permission to access
      your Amazon CloudWatch logs and metrics. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM access roles
        for Amazon Kendra</a>.</p>
}

# Access index outputs
index_id = index.id
index_status = index.status
index_created_at = index.created_at
index_capacity_units = index.capacity_units
index_role_arn = index.role_arn
index_user_context_policy = index.user_context_policy
index_description = index.description
index_user_token_configurations = index.user_token_configurations
index_name = index.name
index_id = index.id
index_error_message = index.error_message
index_document_metadata_configurations = index.document_metadata_configurations
index_server_side_encryption_configuration = index.server_side_encryption_configuration
index_user_group_resolution_configuration = index.user_group_resolution_configuration
index_updated_at = index.updated_at
index_edition = index.edition
index_index_statistics = index.index_statistics
```

---


### Faq

Faq resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A token that you provide to identify the request to create a FAQ. Multiple calls to
            the <code>CreateFaqRequest</code> API with the same client token will create only one
            FAQ. </p> |
| `language_code` | String |  | <p>The code for a language. This allows you to support a language 
            for the FAQ document. English is supported by default. 
            For more information on supported languages, including their codes, 
            see <a href="https://docs.aws.amazon.com/kendra/latest/dg/in-adding-languages.html">Adding 
                documents in languages other than English</a>.</p> |
| `s3_path` | String | ✅ | <p>The path to the FAQ file in S3.</p> |
| `name` | String | ✅ | <p>A name for the FAQ.</p> |
| `index_id` | String | ✅ | <p>The identifier of the index for the FAQ.</p> |
| `description` | String |  | <p>A description for the FAQ.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role with permission to access 
            the S3 bucket that contains the FAQ file. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM access roles for 
                Amazon Kendra</a>.</p> |
| `file_format` | String |  | <p>The format of the FAQ input file. You can choose between a basic CSV format, a CSV
            format that includes customs attributes in a header, and a JSON format that includes
            custom attributes.</p>
         <p>The default format is CSV.</p>
         <p>The format must match the format of the file stored in the S3 bucket identified in 
            the <code>S3Path</code> parameter.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/in-creating-faq.html">Adding questions and
            answers</a>.</p> |
| `tags` | Vec<String> |  | <p>A list of key-value pairs that identify the FAQ. You can use the tags to identify and
            organize your resources and to control access to resources.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_at` | String | <p>The Unix timestamp when the FAQ was created.</p> |
| `language_code` | String | <p>The code for a language. This shows a supported language 
            for the FAQ document. English is supported by default.  
            For more information on supported languages, including their codes, 
            see <a href="https://docs.aws.amazon.com/kendra/latest/dg/in-adding-languages.html">Adding 
                documents in languages other than English</a>.</p> |
| `description` | String | <p>The description of the FAQ that you provided when it was created.</p> |
| `id` | String | <p>The identifier of the FAQ.</p> |
| `file_format` | String | <p>The file format used for the FAQ file.</p> |
| `error_message` | String | <p>If the <code>Status</code> field is <code>FAILED</code>, the <code>ErrorMessage</code>
            field contains the reason why the FAQ failed.</p> |
| `index_id` | String | <p>The identifier of the index for the FAQ.</p> |
| `s3_path` | String |  |
| `status` | String | <p>The status of the FAQ. It is ready to use when the status is
            <code>ACTIVE</code>.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role that provides access 
            to the S3 bucket containing the FAQ file.</p> |
| `updated_at` | String | <p>The Unix timestamp when the FAQ was last updated.</p> |
| `name` | String | <p>The name that you gave the FAQ when it was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create faq
faq = provider.kendra.Faq {
    s3_path = "value"  # <p>The path to the FAQ file in S3.</p>
    name = "value"  # <p>A name for the FAQ.</p>
    index_id = "value"  # <p>The identifier of the index for the FAQ.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role with permission to access 
            the S3 bucket that contains the FAQ file. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM access roles for 
                Amazon Kendra</a>.</p>
}

# Access faq outputs
faq_id = faq.id
faq_created_at = faq.created_at
faq_language_code = faq.language_code
faq_description = faq.description
faq_id = faq.id
faq_file_format = faq.file_format
faq_error_message = faq.error_message
faq_index_id = faq.index_id
faq_s3_path = faq.s3_path
faq_status = faq.status
faq_role_arn = faq.role_arn
faq_updated_at = faq.updated_at
faq_name = faq.name
```

---


### Data_source

DataSource resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A token that you provide to identify the request to create a data source connector.
      Multiple calls to the <code>CreateDataSource</code> API with the same client token will create
      only one data source connector.</p> |
| `tags` | Vec<String> |  | <p>A list of key-value pairs that identify or categorize the data source connector. You 
      can also use tags to help control access to the data source connector. Tag keys and values 
      can consist of Unicode letters, digits, white space, and any of the following symbols: 
      _ . : / = + - @.</p> |
| `configuration` | String |  | <p>Configuration information to connect to your data source repository.</p>
         <p>You can't specify the <code>Configuration</code> parameter when the <code>Type</code>
      parameter is set to <code>CUSTOM</code>. If you do, you receive a
        <code>ValidationException</code> exception.</p>
         <p>The <code>Configuration</code> parameter is required for all other data sources.</p> |
| `name` | String | ✅ | <p>A name for the data source connector.</p> |
| `language_code` | String |  | <p>The code for a language. This allows you to support a language for all 
            documents when creating the data source connector. English is supported 
            by default. For more information on supported languages, including their codes, 
            see <a href="https://docs.aws.amazon.com/kendra/latest/dg/in-adding-languages.html">Adding 
                documents in languages other than English</a>.</p> |
| `vpc_configuration` | String |  | <p>Configuration information for an Amazon Virtual Private Cloud to connect to your data source.
      For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/vpc-configuration.html">Configuring a VPC</a>.</p> |
| `custom_document_enrichment_configuration` | String |  | <p>Configuration information for altering document metadata and content during the
            document ingestion process.</p>
         <p>For more information on how to create, modify and delete document metadata, or make
            other content alterations when you ingest documents into Amazon Kendra, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/custom-document-enrichment.html">Customizing document metadata during the ingestion process</a>.</p> |
| `index_id` | String | ✅ | <p>The identifier of the index you want to use with the data source connector.</p> |
| `type` | String | ✅ | <p>The type of data source repository. For example, <code>SHAREPOINT</code>.</p> |
| `description` | String |  | <p>A description for the data source connector.</p> |
| `role_arn` | String |  | <p>The Amazon Resource Name (ARN) of an IAM role with permission to access 
      the data source and required resources. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM access roles for Amazon Kendra.</a>.</p>
         <p>You can't specify the <code>RoleArn</code> parameter when the <code>Type</code> parameter
      is set to <code>CUSTOM</code>. If you do, you receive a <code>ValidationException</code>
      exception.</p>
         <p>The <code>RoleArn</code> parameter is required for all other data sources.</p> |
| `schedule` | String |  | <p>Sets the frequency for Amazon Kendra to check the documents in your data source
      repository and update the index. If you don't set a schedule Amazon Kendra will not
      periodically update the index. You can call the <code>StartDataSourceSyncJob</code> API to
      update the index.</p>
         <p>Specify a <code>cron-</code> format schedule string or an empty string to indicate that 
      the index is updated on demand.</p>
         <p>You can't specify the <code>Schedule</code> parameter when the <code>Type</code> parameter
      is set to <code>CUSTOM</code>. If you do, you receive a <code>ValidationException</code>
      exception.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `schedule` | String | <p>The schedule for Amazon Kendra to update the index.</p> |
| `type` | String | <p>The type of the data source. For example, <code>SHAREPOINT</code>.</p> |
| `vpc_configuration` | String | <p>Configuration information for an Amazon Virtual Private Cloud to connect to your data source.
      For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/vpc-configuration.html">Configuring a VPC</a>.</p> |
| `id` | String | <p>The identifier of the data source connector.</p> |
| `description` | String | <p>The description for the data source connector.</p> |
| `status` | String | <p>The current status of the data source connector. When the status is <code>ACTIVE</code>
      the data source is ready to use. When the status is <code>FAILED</code>, the
        <code>ErrorMessage</code> field contains the reason that the data source failed.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role with permission to 
      access the data source and required resources.</p> |
| `configuration` | String | <p>Configuration details for the data source connector. This shows how the data source is
      configured. The configuration options for a data source depend on the data source
      provider.</p> |
| `created_at` | String | <p>The Unix timestamp when the data source connector was created.</p> |
| `name` | String | <p>The name for the data source connector.</p> |
| `error_message` | String | <p>When the <code>Status</code> field value is <code>FAILED</code>, the
        <code>ErrorMessage</code> field contains a description of the error that caused the data
      source to fail.</p> |
| `custom_document_enrichment_configuration` | String | <p>Configuration information for altering document metadata and content during the
            document ingestion process when you describe a data source.</p>
         <p>For more information on how to create, modify and delete document metadata, or make
            other content alterations when you ingest documents into Amazon Kendra, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/custom-document-enrichment.html">Customizing document metadata during the ingestion process</a>.</p> |
| `language_code` | String | <p>The code for a language. This shows a supported language for all 
            documents in the data source. English is supported by 
            default. For more information on supported languages, including their codes, 
            see <a href="https://docs.aws.amazon.com/kendra/latest/dg/in-adding-languages.html">Adding 
                documents in languages other than English</a>.</p> |
| `index_id` | String | <p>The identifier of the index used with the data source connector.</p> |
| `updated_at` | String | <p>The Unix timestamp when the data source connector was last updated.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_source
data_source = provider.kendra.Data_source {
    name = "value"  # <p>A name for the data source connector.</p>
    index_id = "value"  # <p>The identifier of the index you want to use with the data source connector.</p>
    type = "value"  # <p>The type of data source repository. For example, <code>SHAREPOINT</code>.</p>
}

# Access data_source outputs
data_source_id = data_source.id
data_source_schedule = data_source.schedule
data_source_type = data_source.type
data_source_vpc_configuration = data_source.vpc_configuration
data_source_id = data_source.id
data_source_description = data_source.description
data_source_status = data_source.status
data_source_role_arn = data_source.role_arn
data_source_configuration = data_source.configuration
data_source_created_at = data_source.created_at
data_source_name = data_source.name
data_source_error_message = data_source.error_message
data_source_custom_document_enrichment_configuration = data_source.custom_document_enrichment_configuration
data_source_language_code = data_source.language_code
data_source_index_id = data_source.index_id
data_source_updated_at = data_source.updated_at
```

---


### Query_suggestions_block_list

QuerySuggestionsBlockList resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `index_id` | String | ✅ | <p>The identifier of the index you want to create a query suggestions block list for.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role with permission to 
            access your S3 bucket that contains the block list text file. For more information, 
            see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM access roles for 
                Amazon Kendra</a>.</p> |
| `source_s3_path` | String | ✅ | <p>The S3 path to your block list text file in your S3 bucket.</p>
         <p>Each block word or phrase should be on a separate line in a text file.</p>
         <p>For information on the current quota limits for block lists, see 
                <a href="https://docs.aws.amazon.com/kendra/latest/dg/quotas.html">Quotas 
                    for Amazon Kendra</a>.</p> |
| `description` | String |  | <p>A description for the block list.</p>
         <p>For example, the description "List of all offensive words that can 
            appear in user queries and need to be blocked from suggestions."</p> |
| `client_token` | String |  | <p>A token that you provide to identify the request to create a 
            query suggestions block list.</p> |
| `tags` | Vec<String> |  | <p>A list of key-value pairs that identify or categorize the block list. 
            Tag keys and values can consist of Unicode letters, digits, white space, 
            and any of the following symbols: _ . : / = + - @.</p> |
| `name` | String | ✅ | <p>A name for the block list.</p>
         <p>For example, the name 'offensive-words', which includes all 
            offensive words that could appear in user queries and need to be 
            blocked from suggestions.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `file_size_bytes` | i64 | <p>The current size of the block list text file in S3.</p> |
| `role_arn` | String | <p>The IAM (Identity and Access Management) role used by 
            Amazon Kendra to access the block list text file in S3.</p>
         <p>The role needs S3 read permissions to your file in S3 and needs to 
            give STS (Security Token Service) assume role permissions to 
            Amazon Kendra.</p> |
| `source_s3_path` | String | <p>Shows the current S3 path to your block list text file in your S3 bucket.</p>
         <p>Each block word or phrase should be on a separate line in a text file.</p>
         <p>For information on the current quota limits for block lists, see 
            <a href="https://docs.aws.amazon.com/kendra/latest/dg/quotas.html">Quotas 
                for Amazon Kendra</a>.</p> |
| `created_at` | String | <p>The Unix timestamp when a block list for query suggestions was created.</p> |
| `description` | String | <p>The description for the block list.</p> |
| `index_id` | String | <p>The identifier of the index for the block list.</p> |
| `name` | String | <p>The name of the block list.</p> |
| `error_message` | String | <p>The error message containing details if there are issues processing 
            the block list.</p> |
| `status` | String | <p>The current status of the block list. When the value is 
            <code>ACTIVE</code>, the block list is ready for use.</p> |
| `updated_at` | String | <p>The Unix timestamp when a block list for query suggestions was last updated.</p> |
| `item_count` | i64 | <p>The current number of valid, non-empty words or phrases in 
            the block list text file.</p> |
| `id` | String | <p>The identifier of the block list.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create query_suggestions_block_list
query_suggestions_block_list = provider.kendra.Query_suggestions_block_list {
    index_id = "value"  # <p>The identifier of the index you want to create a query suggestions block list for.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role with permission to 
            access your S3 bucket that contains the block list text file. For more information, 
            see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM access roles for 
                Amazon Kendra</a>.</p>
    source_s3_path = "value"  # <p>The S3 path to your block list text file in your S3 bucket.</p>
         <p>Each block word or phrase should be on a separate line in a text file.</p>
         <p>For information on the current quota limits for block lists, see 
                <a href="https://docs.aws.amazon.com/kendra/latest/dg/quotas.html">Quotas 
                    for Amazon Kendra</a>.</p>
    name = "value"  # <p>A name for the block list.</p>
         <p>For example, the name 'offensive-words', which includes all 
            offensive words that could appear in user queries and need to be 
            blocked from suggestions.</p>
}

# Access query_suggestions_block_list outputs
query_suggestions_block_list_id = query_suggestions_block_list.id
query_suggestions_block_list_file_size_bytes = query_suggestions_block_list.file_size_bytes
query_suggestions_block_list_role_arn = query_suggestions_block_list.role_arn
query_suggestions_block_list_source_s3_path = query_suggestions_block_list.source_s3_path
query_suggestions_block_list_created_at = query_suggestions_block_list.created_at
query_suggestions_block_list_description = query_suggestions_block_list.description
query_suggestions_block_list_index_id = query_suggestions_block_list.index_id
query_suggestions_block_list_name = query_suggestions_block_list.name
query_suggestions_block_list_error_message = query_suggestions_block_list.error_message
query_suggestions_block_list_status = query_suggestions_block_list.status
query_suggestions_block_list_updated_at = query_suggestions_block_list.updated_at
query_suggestions_block_list_item_count = query_suggestions_block_list.item_count
query_suggestions_block_list_id = query_suggestions_block_list.id
```

---


### Snapshots

Snapshots resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `snapshots_data` | Vec<Vec<String>> | <p>The search metrics data. The data returned depends on the metric type you
            requested.</p> |
| `next_token` | String | <p>If the response is truncated, Amazon Kendra returns this token, which you can use
            in a later request to retrieve the next set of search metrics data.</p> |
| `snap_shot_time_filter` | String | <p>The Unix timestamp for the beginning and end of the time window for the 
            search metrics data.</p> |
| `snapshots_data_header` | Vec<String> | <p>The column headers for the search metrics data.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access snapshots outputs
snapshots_id = snapshots.id
snapshots_snapshots_data = snapshots.snapshots_data
snapshots_next_token = snapshots.next_token
snapshots_snap_shot_time_filter = snapshots.snap_shot_time_filter
snapshots_snapshots_data_header = snapshots.snapshots_data_header
```

---


### Featured_results_set

FeaturedResultsSet resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `index_id` | String | ✅ | <p>The identifier of the index that you want to use for featuring results.</p> |
| `featured_documents` | Vec<String> |  | <p>A list of document IDs for the documents you want to feature at the 
            top of the search results page. For more information on the list of 
            documents, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_FeaturedResultsSet.html">FeaturedResultsSet</a>.</p> |
| `tags` | Vec<String> |  | <p>A list of key-value pairs that identify or categorize the featured results set. You
            can also use tags to help control access to the featured results set. Tag keys and
            values can consist of Unicode letters, digits, white space, and any of the following
            symbols:_ . : / = + - @.</p> |
| `description` | String |  | <p>A description for the set of featured results.</p> |
| `query_texts` | Vec<String> |  | <p>A list of queries for featuring results. For more information on the 
            list of queries, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_FeaturedResultsSet.html">FeaturedResultsSet</a>.</p> |
| `featured_results_set_name` | String | ✅ | <p>A name for the set of featured results.</p> |
| `status` | String |  | <p>The current status of the set of featured results. When the value is 
            <code>ACTIVE</code>, featured results are ready for use. You can still 
            configure your settings before setting the status to <code>ACTIVE</code>. 
            You can set the status to <code>ACTIVE</code> or <code>INACTIVE</code> 
            using the <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_UpdateFeaturedResultsSet.html">UpdateFeaturedResultsSet</a> API. The queries you specify for 
            featured results must be unique per featured results set for each index, 
            whether the status is <code>ACTIVE</code> or <code>INACTIVE</code>.</p> |
| `client_token` | String |  | <p>A token that you provide to identify the request to create a set of 
            featured results. Multiple calls to the <code>CreateFeaturedResultsSet</code> 
            API with the same client token will create only one featured results set.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `query_texts` | Vec<String> | <p>The list of queries for featuring results. For more information on the 
            list of queries, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_FeaturedResultsSet.html">FeaturedResultsSet</a>.</p> |
| `featured_results_set_id` | String | <p>The identifier of the set of featured results.</p> |
| `featured_results_set_name` | String | <p>The name for the set of featured results.</p> |
| `status` | String | <p>The current status of the set of featured results. When the value is 
            <code>ACTIVE</code>, featured results are ready for use. You can still 
            configure your settings before setting the status to <code>ACTIVE</code>. 
            You can set the status to <code>ACTIVE</code> or <code>INACTIVE</code> 
            using the <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_UpdateFeaturedResultsSet.html">UpdateFeaturedResultsSet</a> API. The queries you specify for 
            featured results must be unique per featured results set for each index, 
            whether the status is <code>ACTIVE</code> or <code>INACTIVE</code>.</p> |
| `featured_documents_with_metadata` | Vec<String> | <p>The list of document IDs for the documents you want to feature with their 
            metadata information. For more information on the list of featured documents, 
            see <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_FeaturedResultsSet.html">FeaturedResultsSet</a>.</p> |
| `creation_timestamp` | i64 | <p>The Unix timestamp when the set of the featured results was created.</p> |
| `featured_documents_missing` | Vec<String> | <p>The list of document IDs that don't exist but you have specified as featured 
            documents. Amazon Kendra cannot feature these documents if they don't exist 
            in the index. You can check the status of a document and its ID or check for 
            documents with status errors using the <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_BatchGetDocumentStatus.html">BatchGetDocumentStatus</a> API.</p> |
| `last_updated_timestamp` | i64 | <p>The timestamp when the set of featured results was last updated.</p> |
| `description` | String | <p>The description for the set of featured results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create featured_results_set
featured_results_set = provider.kendra.Featured_results_set {
    index_id = "value"  # <p>The identifier of the index that you want to use for featuring results.</p>
    featured_results_set_name = "value"  # <p>A name for the set of featured results.</p>
}

# Access featured_results_set outputs
featured_results_set_id = featured_results_set.id
featured_results_set_query_texts = featured_results_set.query_texts
featured_results_set_featured_results_set_id = featured_results_set.featured_results_set_id
featured_results_set_featured_results_set_name = featured_results_set.featured_results_set_name
featured_results_set_status = featured_results_set.status
featured_results_set_featured_documents_with_metadata = featured_results_set.featured_documents_with_metadata
featured_results_set_creation_timestamp = featured_results_set.creation_timestamp
featured_results_set_featured_documents_missing = featured_results_set.featured_documents_missing
featured_results_set_last_updated_timestamp = featured_results_set.last_updated_timestamp
featured_results_set_description = featured_results_set.description
```

---


### Query_suggestions_config

QuerySuggestionsConfig resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `index_id` | String | ✅ | <p> The identifier of the index with query suggestions you want to update.</p> |
| `mode` | String |  | <p>Set the mode to <code>ENABLED</code> or <code>LEARN_ONLY</code>.</p>
         <p>By default, Amazon Kendra enables query suggestions. 
            <code>LEARN_ONLY</code> mode allows you to turn off query suggestions. 
            You can to update this at any time.</p>
         <p>In <code>LEARN_ONLY</code> mode, Amazon Kendra continues to learn from new 
            queries to keep suggestions up to date for when you are ready to 
            switch to ENABLED mode again.</p> |
| `include_queries_without_user_information` | bool |  | <p>
            <code>TRUE</code> to include queries without user information (i.e. all queries, 
            irrespective of the user), otherwise <code>FALSE</code> to only include queries 
            with user information.</p>
         <p>If you pass user information to Amazon Kendra along with the queries, you can set this 
            flag to <code>FALSE</code> and instruct Amazon Kendra to only consider queries with user 
            information.</p>
         <p>If you set to <code>FALSE</code>, Amazon Kendra only considers queries searched at least 
            <code>MinimumQueryCount</code> times across <code>MinimumNumberOfQueryingUsers</code> 
            unique users for suggestions.</p>
         <p>If you set to <code>TRUE</code>, Amazon Kendra ignores all user information and learns 
            from all queries.</p> |
| `query_log_look_back_window_in_days` | i64 |  | <p>How recent your queries are in your query log time window.</p>
         <p>The time window is the number of days from current day to past days.</p>
         <p>By default, Amazon Kendra sets this to 180.</p> |
| `minimum_query_count` | i64 |  | <p>The the minimum number of times a query must be searched in order to be 
            eligible to suggest to your users.</p>
         <p>Decreasing this number increases the number of suggestions. However, this 
            affects the quality of suggestions as it sets a low bar for a query to be 
            considered popular to suggest to users.</p>
         <p>How you tune this setting depends on your specific needs.</p> |
| `attribute_suggestions_config` | String |  | <p>Configuration information for the document fields/attributes that you want to base 
            query suggestions on.</p> |
| `minimum_number_of_querying_users` | i64 |  | <p>The minimum number of unique users who must search a query in order for the query 
            to be eligible to suggest to your users.</p>
         <p>Increasing this number might decrease the number of suggestions. However, this 
            ensures a query is searched by many users and is truly popular to suggest to users.</p>
         <p>How you tune this setting depends on your specific needs.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_suggestions_build_time` | String | <p>The Unix timestamp when query suggestions for an index was last updated.</p>
         <p>Amazon Kendra automatically updates suggestions every 24 hours, after you 
            change a setting or after you apply a <a href="https://docs.aws.amazon.com/kendra/latest/dg/query-suggestions.html#query-suggestions-blocklist">block list</a>.</p> |
| `query_log_look_back_window_in_days` | i64 | <p>How recent your queries are in your query log time 
            window (in days).</p> |
| `mode` | String | <p>Whether query suggestions are currently in 
            <code>ENABLED</code> mode or <code>LEARN_ONLY</code> mode.</p>
         <p>By default, Amazon Kendra enables query suggestions.<code>LEARN_ONLY</code> 
            turns off query suggestions for your users. You can change the mode using 
            the <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_UpdateQuerySuggestionsConfig.html">UpdateQuerySuggestionsConfig</a> 
            API.</p> |
| `include_queries_without_user_information` | bool | <p>
            <code>TRUE</code> to use all queries, otherwise use only queries that include 
            user information to generate the query suggestions.</p> |
| `last_clear_time` | String | <p>The Unix timestamp when query suggestions for an index was last cleared.</p>
         <p>After you clear suggestions, Amazon Kendra learns new suggestions based 
            on new queries added to the query log from the time you cleared suggestions. 
            Amazon Kendra only considers re-occurences of a query from the time you cleared 
            suggestions. </p> |
| `attribute_suggestions_config` | String | <p>Configuration information for the document fields/attributes that you want to base query 
            suggestions on.</p> |
| `status` | String | <p>Whether the status of query suggestions settings is currently 
            <code>ACTIVE</code> or <code>UPDATING</code>.</p>
         <p>Active means the current settings apply and Updating means your 
            changed settings are in the process of applying.</p> |
| `total_suggestions_count` | i64 | <p>The current total count of query suggestions for an index.</p>
         <p>This count can change when you update your query suggestions settings, 
            if you filter out certain queries from suggestions using a block list, 
            and as the query log accumulates more queries for Amazon Kendra to learn from.</p>
         <p>If the count is much lower than you expected, it could be because Amazon Kendra 
            needs more queries in the query history to learn from or your current query suggestions 
            settings are too strict.</p> |
| `minimum_query_count` | i64 | <p>The minimum number of times a query must be searched in order for 
            the query to be eligible to suggest to your users.</p> |
| `minimum_number_of_querying_users` | i64 | <p>The minimum number of unique users who must search a query in 
            order for the query to be eligible to suggest to your users.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access query_suggestions_config outputs
query_suggestions_config_id = query_suggestions_config.id
query_suggestions_config_last_suggestions_build_time = query_suggestions_config.last_suggestions_build_time
query_suggestions_config_query_log_look_back_window_in_days = query_suggestions_config.query_log_look_back_window_in_days
query_suggestions_config_mode = query_suggestions_config.mode
query_suggestions_config_include_queries_without_user_information = query_suggestions_config.include_queries_without_user_information
query_suggestions_config_last_clear_time = query_suggestions_config.last_clear_time
query_suggestions_config_attribute_suggestions_config = query_suggestions_config.attribute_suggestions_config
query_suggestions_config_status = query_suggestions_config.status
query_suggestions_config_total_suggestions_count = query_suggestions_config.total_suggestions_count
query_suggestions_config_minimum_query_count = query_suggestions_config.minimum_query_count
query_suggestions_config_minimum_number_of_querying_users = query_suggestions_config.minimum_number_of_querying_users
```

---


### Query_suggestions

QuerySuggestions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `query_suggestions_id` | String | <p>The identifier for a list of query suggestions for an index.</p> |
| `suggestions` | Vec<String> | <p>A list of query suggestions for an index.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access query_suggestions outputs
query_suggestions_id = query_suggestions.id
query_suggestions_query_suggestions_id = query_suggestions.query_suggestions_id
query_suggestions_suggestions = query_suggestions.suggestions
```

---


### Access_control_configuration

AccessControlConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description for the access control configuration.</p> |
| `access_control_list` | Vec<String> |  | <p>Information on principals (users and/or groups) and which documents they should have
            access to. This is useful for user context filtering, where search results are filtered
            based on the user or their group access to documents.</p> |
| `hierarchical_access_control_list` | Vec<String> |  | <p>The list of <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_Principal.html">principal</a> lists that define the hierarchy for which documents users should
            have access to.</p> |
| `name` | String | ✅ | <p>A name for the access control configuration.</p> |
| `index_id` | String | ✅ | <p>The identifier of the index to create an access control configuration for your
            documents.</p> |
| `client_token` | String |  | <p>A token that you provide to identify the request to create an access control
            configuration. Multiple calls to the <code>CreateAccessControlConfiguration</code> API
            with the same client token will create only one access control configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name for the access control configuration.</p> |
| `description` | String | <p>The description for the access control configuration.</p> |
| `error_message` | String | <p>The error message containing details if there are issues processing the access control
            configuration.</p> |
| `hierarchical_access_control_list` | Vec<String> | <p>The list of <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_Principal.html">principal</a> lists that define the hierarchy for which documents users should
            have access to.</p> |
| `access_control_list` | Vec<String> | <p>Information on principals (users and/or groups) and which documents they should have
            access to. This is useful for user context filtering, where search results are filtered
            based on the user or their group access to documents.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create access_control_configuration
access_control_configuration = provider.kendra.Access_control_configuration {
    name = "value"  # <p>A name for the access control configuration.</p>
    index_id = "value"  # <p>The identifier of the index to create an access control configuration for your
            documents.</p>
}

# Access access_control_configuration outputs
access_control_configuration_id = access_control_configuration.id
access_control_configuration_name = access_control_configuration.name
access_control_configuration_description = access_control_configuration.description
access_control_configuration_error_message = access_control_configuration.error_message
access_control_configuration_hierarchical_access_control_list = access_control_configuration.hierarchical_access_control_list
access_control_configuration_access_control_list = access_control_configuration.access_control_list
```

---


### Experience

Experience resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>A name for your Amazon Kendra experience.</p> |
| `configuration` | String |  | <p>Configuration information for your Amazon Kendra experience. This includes
            <code>ContentSourceConfiguration</code>, which specifies the data source IDs 
            and/or FAQ IDs, and <code>UserIdentityConfiguration</code>, which specifies the 
            user or group information to grant access to your Amazon Kendra experience.</p> |
| `index_id` | String | ✅ | <p>The identifier of the index for your Amazon Kendra experience.</p> |
| `client_token` | String |  | <p>A token that you provide to identify the request to create your Amazon Kendra experience.
            Multiple calls to the <code>CreateExperience</code> API with the same client 
            token creates only one Amazon Kendra experience.</p> |
| `description` | String |  | <p>A description for your Amazon Kendra experience.</p> |
| `role_arn` | String |  | <p>The Amazon Resource Name (ARN) of an IAM role with permission to access 
            <code>Query</code> API, <code>GetQuerySuggestions</code> API, and other required APIs. 
            The role also must include permission to access IAM Identity Center that stores your 
            user and group information. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM access roles for Amazon Kendra</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_at` | String | <p>The Unix timestamp when your Amazon Kendra experience was created.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role with permission to access 
            the <code>Query</code> API, <code>QuerySuggestions</code> API, 
            <code>SubmitFeedback</code> API, and IAM Identity Center that stores 
            your users and groups information.</p> |
| `id` | String | <p>Shows the identifier of your Amazon Kendra experience.</p> |
| `error_message` | String | <p>The reason your Amazon Kendra experience could not properly process.</p> |
| `configuration` | String | <p>Shows the configuration information for your Amazon Kendra experience. This includes
                <code>ContentSourceConfiguration</code>, which specifies the data source IDs 
            and/or FAQ IDs, and <code>UserIdentityConfiguration</code>, which specifies the 
            user or group information to grant access to your Amazon Kendra experience.</p> |
| `name` | String | <p>Shows the name of your Amazon Kendra experience.</p> |
| `index_id` | String | <p>Shows the identifier of the index for your Amazon Kendra experience.</p> |
| `endpoints` | Vec<String> | <p>Shows the endpoint URLs for your Amazon Kendra experiences. The URLs are unique and fully
            hosted by Amazon Web Services.</p> |
| `updated_at` | String | <p>The Unix timestamp when your Amazon Kendra experience was last updated.</p> |
| `status` | String | <p>The current processing status of your Amazon Kendra experience. When the status 
            is <code>ACTIVE</code>, your Amazon Kendra experience is ready to use. When the 
            status is <code>FAILED</code>, the <code>ErrorMessage</code> field contains 
            the reason that this failed.</p> |
| `description` | String | <p>Shows the description for your Amazon Kendra experience.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create experience
experience = provider.kendra.Experience {
    name = "value"  # <p>A name for your Amazon Kendra experience.</p>
    index_id = "value"  # <p>The identifier of the index for your Amazon Kendra experience.</p>
}

# Access experience outputs
experience_id = experience.id
experience_created_at = experience.created_at
experience_role_arn = experience.role_arn
experience_id = experience.id
experience_error_message = experience.error_message
experience_configuration = experience.configuration
experience_name = experience.name
experience_index_id = experience.index_id
experience_endpoints = experience.endpoints
experience_updated_at = experience.updated_at
experience_status = experience.status
experience_description = experience.description
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple thesaurus resources
thesaurus_0 = provider.kendra.Thesaurus {
    role_arn = "value-0"
    source_s3_path = "value-0"
    index_id = "value-0"
    name = "value-0"
}
thesaurus_1 = provider.kendra.Thesaurus {
    role_arn = "value-1"
    source_s3_path = "value-1"
    index_id = "value-1"
    name = "value-1"
}
thesaurus_2 = provider.kendra.Thesaurus {
    role_arn = "value-2"
    source_s3_path = "value-2"
    index_id = "value-2"
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    thesaurus = provider.kendra.Thesaurus {
        role_arn = "production-value"
        source_s3_path = "production-value"
        index_id = "production-value"
        name = "production-value"
    }
```

---

## Related Documentation

- [AWS Kendra Documentation](https://docs.aws.amazon.com/kendra/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
