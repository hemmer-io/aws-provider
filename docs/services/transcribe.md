# Transcribe Service



**Resources**: 9

---

## Overview

The transcribe service provides access to 9 resource types:

- [Call_analytics_job](#call_analytics_job) [RD]
- [Language_model](#language_model) [CRD]
- [Call_analytics_category](#call_analytics_category) [CRUD]
- [Vocabulary_filter](#vocabulary_filter) [CRUD]
- [Medical_vocabulary](#medical_vocabulary) [CRUD]
- [Medical_transcription_job](#medical_transcription_job) [RD]
- [Transcription_job](#transcription_job) [RD]
- [Medical_scribe_job](#medical_scribe_job) [RD]
- [Vocabulary](#vocabulary) [CRUD]

---

## Resources


### Call_analytics_job

CallAnalyticsJob resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `call_analytics_job` | String | <p>Provides detailed information about the specified Call Analytics job, including job
            status and, if applicable, failure reason.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access call_analytics_job outputs
call_analytics_job_id = call_analytics_job.id
call_analytics_job_call_analytics_job = call_analytics_job.call_analytics_job
```

---


### Language_model

LanguageModel resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `language_code` | String | ✅ | <p>The language code that represents the language of your model. Each custom language
            model must contain terms in only one language, and the language you select for your
            custom language model must match the language of your training and tuning data.</p>
         <p>For a list of supported languages and their associated language codes, refer to the
                <a href="https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html">Supported languages</a> table. Note that US English (<code>en-US</code>) is the 
            only language supported with Amazon Transcribe Medical.</p>
         <p>A custom language model can only be used to transcribe files in the same language as
            the model. For example, if you create a custom language model using US English
                (<code>en-US</code>), you can only apply this model to files that contain English
            audio.</p> |
| `model_name` | String | ✅ | <p>A unique name, chosen by you, for your custom language model.</p>
         <p>This name is case sensitive, cannot contain spaces, and must be unique within an
                Amazon Web Services account. If you try to create a new custom language model with
            the same name as an existing custom language model, you get a
                <code>ConflictException</code> error.</p> |
| `base_model_name` | String | ✅ | <p>The Amazon Transcribe standard language model, or base model, used to create your
            custom language model. Amazon Transcribe offers two options for base models: Wideband
            and Narrowband.</p>
         <p>If the audio you want to transcribe has a sample rate of 16,000 Hz or greater, choose
                <code>WideBand</code>. To transcribe audio with a sample rate less than 16,000 Hz,
            choose <code>NarrowBand</code>.</p> |
| `input_data_config` | String | ✅ | <p>Contains the Amazon S3 location of the training data you want to use to create
            a new custom language model, and permissions to access this location.</p>
         <p>When using <code>InputDataConfig</code>, you must include these sub-parameters:
                <code>S3Uri</code>, which is the Amazon S3 location of your training data,
            and <code>DataAccessRoleArn</code>, which is the Amazon Resource Name (ARN) of the role
            that has permission to access your specified Amazon S3 location. You can
            optionally include <code>TuningDataS3Uri</code>, which is the Amazon S3 location
            of your tuning data. If you specify different Amazon S3 locations for training
            and tuning data, the ARN you use must have permissions to access both locations.</p> |
| `tags` | Vec<String> |  | <p>Adds one or more custom tags, each in the form of a key:value pair, to a new custom
            language model at the time you create this new model.</p>
         <p>To learn more about using tags with Amazon Transcribe, refer to <a href="https://docs.aws.amazon.com/transcribe/latest/dg/tagging.html">Tagging
                resources</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `language_model` | String | <p>Provides information about the specified custom language model.</p>
         <p>This parameter also shows if the base language model you used to create your custom
            language model has been updated. If Amazon Transcribe has updated the base model, you
            can create a new custom language model using the updated base model.</p>
         <p>If you tried to create a new custom language model and the request wasn't successful,
            you can use this <code>DescribeLanguageModel</code> to help identify the reason for this
            failure.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create language_model
language_model = provider.transcribe.Language_model {
    language_code = "value"  # <p>The language code that represents the language of your model. Each custom language
            model must contain terms in only one language, and the language you select for your
            custom language model must match the language of your training and tuning data.</p>
         <p>For a list of supported languages and their associated language codes, refer to the
                <a href="https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html">Supported languages</a> table. Note that US English (<code>en-US</code>) is the 
            only language supported with Amazon Transcribe Medical.</p>
         <p>A custom language model can only be used to transcribe files in the same language as
            the model. For example, if you create a custom language model using US English
                (<code>en-US</code>), you can only apply this model to files that contain English
            audio.</p>
    model_name = "value"  # <p>A unique name, chosen by you, for your custom language model.</p>
         <p>This name is case sensitive, cannot contain spaces, and must be unique within an
                Amazon Web Services account. If you try to create a new custom language model with
            the same name as an existing custom language model, you get a
                <code>ConflictException</code> error.</p>
    base_model_name = "value"  # <p>The Amazon Transcribe standard language model, or base model, used to create your
            custom language model. Amazon Transcribe offers two options for base models: Wideband
            and Narrowband.</p>
         <p>If the audio you want to transcribe has a sample rate of 16,000 Hz or greater, choose
                <code>WideBand</code>. To transcribe audio with a sample rate less than 16,000 Hz,
            choose <code>NarrowBand</code>.</p>
    input_data_config = "value"  # <p>Contains the Amazon S3 location of the training data you want to use to create
            a new custom language model, and permissions to access this location.</p>
         <p>When using <code>InputDataConfig</code>, you must include these sub-parameters:
                <code>S3Uri</code>, which is the Amazon S3 location of your training data,
            and <code>DataAccessRoleArn</code>, which is the Amazon Resource Name (ARN) of the role
            that has permission to access your specified Amazon S3 location. You can
            optionally include <code>TuningDataS3Uri</code>, which is the Amazon S3 location
            of your tuning data. If you specify different Amazon S3 locations for training
            and tuning data, the ARN you use must have permissions to access both locations.</p>
}

# Access language_model outputs
language_model_id = language_model.id
language_model_language_model = language_model.language_model
```

---


### Call_analytics_category

CallAnalyticsCategory resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `category_name` | String | ✅ | <p>A unique name, chosen by you, for your Call Analytics category. It's helpful to use a
            detailed naming system that will make sense to you in the future. For example, it's
            better to use <code>sentiment-positive-last30seconds</code> for a category over a
            generic name like <code>test-category</code>.</p>
         <p>Category names are case sensitive.</p> |
| `tags` | Vec<String> |  | <p>Adds one or more custom tags, each in the form of a key:value pair, to a new
            call analytics category at the time you start this new job.</p>
         <p>To learn more about using tags with Amazon Transcribe, refer to <a href="https://docs.aws.amazon.com/transcribe/latest/dg/tagging.html">Tagging
            resources</a>.</p> |
| `rules` | Vec<String> | ✅ | <p>Rules define a Call Analytics category. When creating a new category, you must create 
            between 1 and 20 rules for that category. For each rule, you specify a filter you want 
            applied to the attributes of a call. For example, you can choose a sentiment filter that 
            detects if a customer's sentiment was positive during the last 30 seconds of the call.</p> |
| `input_type` | String |  | <p>Choose whether you want to create a real-time or a post-call category for your Call 
            Analytics transcription.</p>
         <p>Specifying <code>POST_CALL</code> assigns your category to post-call transcriptions; 
            categories with this input type cannot be applied to streaming (real-time) 
            transcriptions.</p>
         <p>Specifying <code>REAL_TIME</code> assigns your category to streaming transcriptions; 
            categories with this input type cannot be applied to post-call transcriptions.</p>
         <p>If you do not include <code>InputType</code>, your category is created as a post-call 
            category by default.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `category_properties` | String | <p>Provides you with the properties of the Call Analytics category you specified in your
                <code>GetCallAnalyticsCategory</code> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create call_analytics_category
call_analytics_category = provider.transcribe.Call_analytics_category {
    category_name = "value"  # <p>A unique name, chosen by you, for your Call Analytics category. It's helpful to use a
            detailed naming system that will make sense to you in the future. For example, it's
            better to use <code>sentiment-positive-last30seconds</code> for a category over a
            generic name like <code>test-category</code>.</p>
         <p>Category names are case sensitive.</p>
    rules = "value"  # <p>Rules define a Call Analytics category. When creating a new category, you must create 
            between 1 and 20 rules for that category. For each rule, you specify a filter you want 
            applied to the attributes of a call. For example, you can choose a sentiment filter that 
            detects if a customer's sentiment was positive during the last 30 seconds of the call.</p>
}

# Access call_analytics_category outputs
call_analytics_category_id = call_analytics_category.id
call_analytics_category_category_properties = call_analytics_category.category_properties
```

---


### Vocabulary_filter

VocabularyFilter resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `language_code` | String | ✅ | <p>The language code that represents the language of the entries in your vocabulary
            filter. Each custom vocabulary filter must contain terms in only one language.</p>
         <p>A custom vocabulary filter can only be used to transcribe files in the same language
            as the filter. For example, if you create a custom vocabulary filter using US English
                (<code>en-US</code>), you can only apply this filter to files that contain English
            audio.</p>
         <p>For a list of supported languages and their associated language codes, refer to the
                <a href="https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html">Supported languages</a> table.</p> |
| `tags` | Vec<String> |  | <p>Adds one or more custom tags, each in the form of a key:value pair, to a new custom
            vocabulary filter at the time you create this new vocabulary filter.</p>
         <p>To learn more about using tags with Amazon Transcribe, refer to <a href="https://docs.aws.amazon.com/transcribe/latest/dg/tagging.html">Tagging
                resources</a>.</p> |
| `vocabulary_filter_name` | String | ✅ | <p>A unique name, chosen by you, for your new custom vocabulary filter.</p>
         <p>This name is case sensitive, cannot contain spaces, and must be unique within an
                Amazon Web Services account. If you try to create a new custom vocabulary filter with
            the same name as an existing custom vocabulary filter, you get a
                <code>ConflictException</code> error.</p> |
| `data_access_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of an IAM role that has permissions to
            access the Amazon S3 bucket that contains your input files (in this case, your custom
            vocabulary filter). If the role that you specify doesn’t have the appropriate permissions to access
            the specified Amazon S3 location, your request fails.</p>
         <p>IAM role ARNs have the format
            <code>arn:partition:iam::account:role/role-name-with-path</code>. For example:
            <code>arn:aws:iam::111122223333:role/Admin</code>.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM
            ARNs</a>.</p> |
| `words` | Vec<String> |  | <p>Use this parameter if you want to create your custom vocabulary filter by including
            all desired terms, as comma-separated values, within your request. The other option for
            creating your vocabulary filter is to save your entries in a text file and upload them
            to an Amazon S3 bucket, then specify the location of your file using the
                <code>VocabularyFilterFileUri</code> parameter.</p>
         <p>Note that if you include <code>Words</code> in your request, you cannot use
                <code>VocabularyFilterFileUri</code>; you must choose one or the other.</p>
         <p>Each language has a character set that contains all allowed characters for that
            specific language. If you use unsupported characters, your custom vocabulary filter
            request fails. Refer to <a href="https://docs.aws.amazon.com/transcribe/latest/dg/charsets.html">Character Sets for Custom
                Vocabularies</a> to get the character set for your language.</p> |
| `vocabulary_filter_file_uri` | String |  | <p>The Amazon S3 location of the text file that contains your custom vocabulary
            filter terms. The URI must be located in the same Amazon Web Services Region as the
            resource you're calling.</p>
         <p>Here's an example URI path:
                <code>s3://DOC-EXAMPLE-BUCKET/my-vocab-filter-file.txt</code>
         </p>
         <p>Note that if you include <code>VocabularyFilterFileUri</code> in your request, you
            cannot use <code>Words</code>; you must choose one or the other.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_modified_time` | String | <p>The date and time the specified custom vocabulary filter was last modified.</p>
         <p>Timestamps are in the format <code>YYYY-MM-DD'T'HH:MM:SS.SSSSSS-UTC</code>. For
            example, <code>2022-05-04T12:32:58.761000-07:00</code> represents 12:32 PM UTC-7 on May
            4, 2022.</p> |
| `download_uri` | String | <p>The Amazon S3 location where the custom vocabulary filter is stored; use this
            URI to view or download the custom vocabulary filter.</p> |
| `language_code` | String | <p>The language code you selected for your custom vocabulary filter.</p> |
| `vocabulary_filter_name` | String | <p>The name of the custom vocabulary filter you requested information about.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vocabulary_filter
vocabulary_filter = provider.transcribe.Vocabulary_filter {
    language_code = "value"  # <p>The language code that represents the language of the entries in your vocabulary
            filter. Each custom vocabulary filter must contain terms in only one language.</p>
         <p>A custom vocabulary filter can only be used to transcribe files in the same language
            as the filter. For example, if you create a custom vocabulary filter using US English
                (<code>en-US</code>), you can only apply this filter to files that contain English
            audio.</p>
         <p>For a list of supported languages and their associated language codes, refer to the
                <a href="https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html">Supported languages</a> table.</p>
    vocabulary_filter_name = "value"  # <p>A unique name, chosen by you, for your new custom vocabulary filter.</p>
         <p>This name is case sensitive, cannot contain spaces, and must be unique within an
                Amazon Web Services account. If you try to create a new custom vocabulary filter with
            the same name as an existing custom vocabulary filter, you get a
                <code>ConflictException</code> error.</p>
}

# Access vocabulary_filter outputs
vocabulary_filter_id = vocabulary_filter.id
vocabulary_filter_last_modified_time = vocabulary_filter.last_modified_time
vocabulary_filter_download_uri = vocabulary_filter.download_uri
vocabulary_filter_language_code = vocabulary_filter.language_code
vocabulary_filter_vocabulary_filter_name = vocabulary_filter.vocabulary_filter_name
```

---


### Medical_vocabulary

MedicalVocabulary resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vocabulary_file_uri` | String | ✅ | <p>The Amazon S3 location (URI) of the text file that contains your custom
            medical vocabulary. The URI must be in the same Amazon Web Services Region as the
            resource you're calling.</p>
         <p>Here's an example URI path:
            <code>s3://DOC-EXAMPLE-BUCKET/my-vocab-file.txt</code>
         </p> |
| `tags` | Vec<String> |  | <p>Adds one or more custom tags, each in the form of a key:value pair, to a new custom
            medical vocabulary at the time you create this new custom vocabulary.</p>
         <p>To learn more about using tags with Amazon Transcribe, refer to <a href="https://docs.aws.amazon.com/transcribe/latest/dg/tagging.html">Tagging
                resources</a>.</p> |
| `vocabulary_name` | String | ✅ | <p>A unique name, chosen by you, for your new custom medical vocabulary.</p>
         <p>This name is case sensitive, cannot contain spaces, and must be unique within an
                Amazon Web Services account. If you try to create a new custom medical vocabulary
            with the same name as an existing custom medical vocabulary, you get a
                <code>ConflictException</code> error.</p> |
| `language_code` | String | ✅ | <p>The language code that represents the language of the entries in your custom
            vocabulary. US English (<code>en-US</code>) is the only language supported with Amazon Transcribe Medical.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `failure_reason` | String | <p>If <code>VocabularyState</code> is <code>FAILED</code>, <code>FailureReason</code>
            contains information about why the custom medical vocabulary request failed. See also:
                <a href="https://docs.aws.amazon.com/transcribe/latest/APIReference/CommonErrors.html">Common Errors</a>.</p> |
| `vocabulary_name` | String | <p>The name of the custom medical vocabulary you requested information about.</p> |
| `vocabulary_state` | String | <p>The processing state of your custom medical vocabulary. If the state is
                <code>READY</code>, you can use the custom vocabulary in a
                <code>StartMedicalTranscriptionJob</code> request.</p> |
| `language_code` | String | <p>The language code you selected for your custom medical vocabulary. US English
                (<code>en-US</code>) is the only language supported with Amazon Transcribe
            Medical.</p> |
| `last_modified_time` | String | <p>The date and time the specified custom medical vocabulary was last modified.</p>
         <p>Timestamps are in the format <code>YYYY-MM-DD'T'HH:MM:SS.SSSSSS-UTC</code>. For
            example, <code>2022-05-04T12:32:58.761000-07:00</code> represents 12:32 PM UTC-7 on May
            4, 2022.</p> |
| `download_uri` | String | <p>The Amazon S3 location where the specified custom medical vocabulary is stored; use this URI
            to view or download the custom vocabulary.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create medical_vocabulary
medical_vocabulary = provider.transcribe.Medical_vocabulary {
    vocabulary_file_uri = "value"  # <p>The Amazon S3 location (URI) of the text file that contains your custom
            medical vocabulary. The URI must be in the same Amazon Web Services Region as the
            resource you're calling.</p>
         <p>Here's an example URI path:
            <code>s3://DOC-EXAMPLE-BUCKET/my-vocab-file.txt</code>
         </p>
    vocabulary_name = "value"  # <p>A unique name, chosen by you, for your new custom medical vocabulary.</p>
         <p>This name is case sensitive, cannot contain spaces, and must be unique within an
                Amazon Web Services account. If you try to create a new custom medical vocabulary
            with the same name as an existing custom medical vocabulary, you get a
                <code>ConflictException</code> error.</p>
    language_code = "value"  # <p>The language code that represents the language of the entries in your custom
            vocabulary. US English (<code>en-US</code>) is the only language supported with Amazon Transcribe Medical.</p>
}

# Access medical_vocabulary outputs
medical_vocabulary_id = medical_vocabulary.id
medical_vocabulary_failure_reason = medical_vocabulary.failure_reason
medical_vocabulary_vocabulary_name = medical_vocabulary.vocabulary_name
medical_vocabulary_vocabulary_state = medical_vocabulary.vocabulary_state
medical_vocabulary_language_code = medical_vocabulary.language_code
medical_vocabulary_last_modified_time = medical_vocabulary.last_modified_time
medical_vocabulary_download_uri = medical_vocabulary.download_uri
```

---


### Medical_transcription_job

MedicalTranscriptionJob resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `medical_transcription_job` | String | <p>Provides detailed information about the specified medical transcription job, including
            job status and, if applicable, failure reason.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access medical_transcription_job outputs
medical_transcription_job_id = medical_transcription_job.id
medical_transcription_job_medical_transcription_job = medical_transcription_job.medical_transcription_job
```

---


### Transcription_job

TranscriptionJob resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transcription_job` | String | <p>Provides detailed information about the specified transcription job, including job
            status and, if applicable, failure reason.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transcription_job outputs
transcription_job_id = transcription_job.id
transcription_job_transcription_job = transcription_job.transcription_job
```

---


### Medical_scribe_job

MedicalScribeJob resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `medical_scribe_job` | String | <p>Provides detailed information about the specified Medical Scribe job, including 
            job status and, if applicable, failure reason</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access medical_scribe_job outputs
medical_scribe_job_id = medical_scribe_job.id
medical_scribe_job_medical_scribe_job = medical_scribe_job.medical_scribe_job
```

---


### Vocabulary

Vocabulary resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>Adds one or more custom tags, each in the form of a key:value pair, to a new custom
            vocabulary at the time you create this new custom vocabulary.</p>
         <p>To learn more about using tags with Amazon Transcribe, refer to <a href="https://docs.aws.amazon.com/transcribe/latest/dg/tagging.html">Tagging
                resources</a>.</p> |
| `vocabulary_name` | String | ✅ | <p>A unique name, chosen by you, for your new custom vocabulary.</p>
         <p>This name is case sensitive, cannot contain spaces, and must be unique within an
                Amazon Web Services account. If you try to create a new custom vocabulary with the
            same name as an existing custom vocabulary, you get a <code>ConflictException</code>
            error.</p> |
| `phrases` | Vec<String> |  | <p>Use this parameter if you want to create your custom vocabulary by including all
            desired terms, as comma-separated values, within your request. The other option for
            creating your custom vocabulary is to save your entries in a text file and upload them
            to an Amazon S3 bucket, then specify the location of your file using the
                <code>VocabularyFileUri</code> parameter.</p>
         <p>Note that if you include <code>Phrases</code> in your request, you cannot use
                <code>VocabularyFileUri</code>; you must choose one or the other.</p>
         <p>Each language has a character set that contains all allowed characters for that
            specific language. If you use unsupported characters, your custom vocabulary filter
            request fails. Refer to <a href="https://docs.aws.amazon.com/transcribe/latest/dg/charsets.html">Character Sets for Custom
                Vocabularies</a> to get the character set for your language.</p> |
| `language_code` | String | ✅ | <p>The language code that represents the language of the entries in your custom
            vocabulary. Each custom vocabulary must contain terms in only one language.</p>
         <p>A custom vocabulary can only be used to transcribe files in the same language as the
            custom vocabulary. For example, if you create a custom vocabulary using US English
                (<code>en-US</code>), you can only apply this custom vocabulary to files that
            contain English audio.</p>
         <p>For a list of supported languages and their associated language codes, refer to the
                <a href="https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html">Supported languages</a> table.</p> |
| `vocabulary_file_uri` | String |  | <p>The Amazon S3 location of the text file that contains your custom vocabulary.
            The URI must be located in the same Amazon Web Services Region as the resource you're
            calling.</p>
         <p>Here's an example URI path:
            <code>s3://DOC-EXAMPLE-BUCKET/my-vocab-file.txt</code>
         </p>
         <p>Note that if you include <code>VocabularyFileUri</code> in your request, you cannot
            use the <code>Phrases</code> flag; you must choose one or the other.</p> |
| `data_access_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of an IAM role that has permissions to
            access the Amazon S3 bucket that contains your input files (in this case, your custom
            vocabulary). If the role that you specify doesn’t have the appropriate permissions to access
            the specified Amazon S3 location, your request fails.</p>
         <p>IAM role ARNs have the format
            <code>arn:partition:iam::account:role/role-name-with-path</code>. For example:
            <code>arn:aws:iam::111122223333:role/Admin</code>.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM
            ARNs</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `language_code` | String | <p>The language code you selected for your custom vocabulary.</p> |
| `vocabulary_state` | String | <p>The processing state of your custom vocabulary. If the state is <code>READY</code>,
            you can use the custom vocabulary in a <code>StartTranscriptionJob</code>
            request.</p> |
| `vocabulary_name` | String | <p>The name of the custom vocabulary you requested information about.</p> |
| `last_modified_time` | String | <p>The date and time the specified custom vocabulary was last modified.</p>
         <p>Timestamps are in the format <code>YYYY-MM-DD'T'HH:MM:SS.SSSSSS-UTC</code>. For
            example, <code>2022-05-04T12:32:58.761000-07:00</code> represents 12:32 PM UTC-7 on May
            4, 2022.</p> |
| `download_uri` | String | <p>The Amazon S3 location where the custom vocabulary is stored; use this URI to view or
            download the custom vocabulary.</p> |
| `failure_reason` | String | <p>If <code>VocabularyState</code> is <code>FAILED</code>, <code>FailureReason</code>
            contains information about why the custom vocabulary request failed. See also: <a href="https://docs.aws.amazon.com/transcribe/latest/APIReference/CommonErrors.html">Common
                Errors</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vocabulary
vocabulary = provider.transcribe.Vocabulary {
    vocabulary_name = "value"  # <p>A unique name, chosen by you, for your new custom vocabulary.</p>
         <p>This name is case sensitive, cannot contain spaces, and must be unique within an
                Amazon Web Services account. If you try to create a new custom vocabulary with the
            same name as an existing custom vocabulary, you get a <code>ConflictException</code>
            error.</p>
    language_code = "value"  # <p>The language code that represents the language of the entries in your custom
            vocabulary. Each custom vocabulary must contain terms in only one language.</p>
         <p>A custom vocabulary can only be used to transcribe files in the same language as the
            custom vocabulary. For example, if you create a custom vocabulary using US English
                (<code>en-US</code>), you can only apply this custom vocabulary to files that
            contain English audio.</p>
         <p>For a list of supported languages and their associated language codes, refer to the
                <a href="https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html">Supported languages</a> table.</p>
}

# Access vocabulary outputs
vocabulary_id = vocabulary.id
vocabulary_language_code = vocabulary.language_code
vocabulary_vocabulary_state = vocabulary.vocabulary_state
vocabulary_vocabulary_name = vocabulary.vocabulary_name
vocabulary_last_modified_time = vocabulary.last_modified_time
vocabulary_download_uri = vocabulary.download_uri
vocabulary_failure_reason = vocabulary.failure_reason
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple call_analytics_job resources
call_analytics_job_0 = provider.transcribe.Call_analytics_job {
}
call_analytics_job_1 = provider.transcribe.Call_analytics_job {
}
call_analytics_job_2 = provider.transcribe.Call_analytics_job {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    call_analytics_job = provider.transcribe.Call_analytics_job {
    }
```

---

## Related Documentation

- [AWS Transcribe Documentation](https://docs.aws.amazon.com/transcribe/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
