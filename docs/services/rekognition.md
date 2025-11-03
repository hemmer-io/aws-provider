# Rekognition Service



**Resources**: 23

---

## Overview

The rekognition service provides access to 23 resource types:

- [Dataset_entries](#dataset_entries) [U]
- [Collection](#collection) [CRD]
- [Project](#project) [CD]
- [Celebrity_recognition](#celebrity_recognition) [R]
- [Stream_processor](#stream_processor) [CRUD]
- [Content_moderation](#content_moderation) [R]
- [Media_analysis_job](#media_analysis_job) [R]
- [Person_tracking](#person_tracking) [R]
- [Face_liveness_session_results](#face_liveness_session_results) [R]
- [Face_search](#face_search) [R]
- [User](#user) [CD]
- [Project_versions](#project_versions) [R]
- [Project_policy](#project_policy) [CD]
- [Dataset](#dataset) [CRD]
- [Projects](#projects) [R]
- [Celebrity_info](#celebrity_info) [R]
- [Face_liveness_session](#face_liveness_session) [C]
- [Face_detection](#face_detection) [R]
- [Label_detection](#label_detection) [R]
- [Project_version](#project_version) [CD]
- [Segment_detection](#segment_detection) [R]
- [Faces](#faces) [D]
- [Text_detection](#text_detection) [R]

---

## Resources


### Dataset_entries

DatasetEntries resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `changes` | String | ✅ | <p>
   The changes that you want to make to the dataset. 
</p> |
| `dataset_arn` | String | ✅ | <p>
The Amazon Resource Name (ARN) of the dataset that you want to update.
</p> |



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


### Collection

Collection resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p> A set of tags (key-value pairs) that you want to attach to the collection. </p> |
| `collection_id` | String | ✅ | <p>ID for the collection that you are creating.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_count` | i64 | <p>The number of UserIDs assigned to the specified colleciton.</p> |
| `collection_arn` | String | <p>The Amazon Resource Name (ARN) of the collection.</p> |
| `face_count` | i64 | <p>The number of faces that are indexed into the collection. To index faces into a
         collection, use <a>IndexFaces</a>.</p> |
| `creation_timestamp` | String | <p>The number of milliseconds since the Unix epoch time until the creation of the collection.
         The Unix epoch time is 00:00:00 Coordinated Universal Time (UTC), Thursday, 1 January 1970.</p> |
| `face_model_version` | String | <p>The version of the face model that's used by the collection for face detection.</p>
         <p>For more information, see Model versioning in the 
     Amazon Rekognition Developer Guide.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create collection
collection = provider.rekognition.Collection {
    collection_id = "value"  # <p>ID for the collection that you are creating.</p>
}

# Access collection outputs
collection_id = collection.id
collection_user_count = collection.user_count
collection_collection_arn = collection.collection_arn
collection_face_count = collection.face_count
collection_creation_timestamp = collection.creation_timestamp
collection_face_model_version = collection.face_model_version
```

---


### Project

Project resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>A set of tags (key-value pairs) that you want to attach to the project.</p> |
| `project_name` | String | ✅ | <p>The name of the project to create.</p> |
| `auto_update` | String |  | <p>Specifies whether automatic retraining should be attempted for the versions of the
         project. Automatic retraining is done as a best effort. Required argument for Content
         Moderation. Applicable only to adapters.</p> |
| `feature` | String |  | <p>Specifies feature that is being customized. If no value is provided CUSTOM_LABELS is used as a default.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create project
project = provider.rekognition.Project {
    project_name = "value"  # <p>The name of the project to create.</p>
}

```

---


### Celebrity_recognition

CelebrityRecognition resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `celebrities` | Vec<String> | <p>Array of celebrities recognized in the video.</p> |
| `job_id` | String | <p>Job identifier for the celebrity recognition operation for which you want to obtain
      results. The job identifer is returned by an initial call to StartCelebrityRecognition.</p> |
| `video` | String |  |
| `video_metadata` | String | <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in
      every page of paginated responses from a Amazon Rekognition Video operation.</p> |
| `job_tag` | String | <p>A job identifier specified in the call to StartCelebrityRecognition and returned in the
      job completion notification sent to your Amazon Simple Notification Service topic.</p> |
| `status_message` | String | <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p> |
| `next_token` | String | <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request
      to retrieve the next set of celebrities.</p> |
| `job_status` | String | <p>The current status of the celebrity recognition job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access celebrity_recognition outputs
celebrity_recognition_id = celebrity_recognition.id
celebrity_recognition_celebrities = celebrity_recognition.celebrities
celebrity_recognition_job_id = celebrity_recognition.job_id
celebrity_recognition_video = celebrity_recognition.video
celebrity_recognition_video_metadata = celebrity_recognition.video_metadata
celebrity_recognition_job_tag = celebrity_recognition.job_tag
celebrity_recognition_status_message = celebrity_recognition.status_message
celebrity_recognition_next_token = celebrity_recognition.next_token
celebrity_recognition_job_status = celebrity_recognition.job_status
```

---


### Stream_processor

StreamProcessor resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `output` | String | ✅ | <p>Kinesis data stream stream or Amazon S3 bucket location to which Amazon Rekognition Video puts the analysis results. If you are using the AWS CLI, the parameter name is <code>StreamProcessorOutput</code>. 
            This must be a <a>S3Destination</a> of an Amazon S3 bucket that you own for a label detection stream processor or a Kinesis data stream ARN for a face search stream processor.</p> |
| `settings` | String | ✅ | <p>Input parameters used in a streaming video analyzed by a stream processor. You can use <code>FaceSearch</code> to recognize faces in a streaming video, or you can use <code>ConnectedHome</code> to detect labels.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Number (ARN) of the IAM role that allows access to the stream processor. 
            The IAM role provides Rekognition read permissions for a Kinesis stream. 
            It also provides write permissions to an Amazon S3 bucket and Amazon Simple Notification Service topic for a label detection stream processor. This is required for both face search and label detection stream processors.</p> |
| `tags` | HashMap<String, String> |  | <p> A set of tags (key-value pairs) that you want to attach to the stream processor. </p> |
| `notification_channel` | String |  |  |
| `kms_key_id` | String |  | <p>
            The identifier for your AWS Key Management Service key (AWS KMS key). This is an optional parameter for label detection stream processors and should not be used to create a face search stream processor.
            You can supply the Amazon Resource Name (ARN) of your KMS key, the ID of your KMS key, an alias for your KMS key, or an alias ARN. 
            The key is used to encrypt results and data published to your Amazon S3 bucket, which includes  image frames and hero images. Your source images are unaffected. 
        </p>
         <p>
            </p> |
| `input` | String | ✅ | <p>Kinesis video stream stream that provides the source streaming video. If you are using the AWS CLI, the parameter name is <code>StreamProcessorInput</code>. This is required for both face search and label detection stream processors.</p> |
| `data_sharing_preference` | String |  | <p>
            Shows whether you are sharing data with Rekognition to improve model performance. You can choose this option at the account level or on a per-stream basis.
            Note that if you opt out at the account level this setting is ignored on individual streams.
        </p> |
| `regions_of_interest` | Vec<String> |  | <p>
            Specifies locations in the frames where Amazon Rekognition checks for objects or people. You can specify up to 10 regions of interest, and each region has either a polygon or a bounding box. This is an optional parameter for label detection stream processors and should not be used to create a face search stream processor.
        </p> |
| `name` | String | ✅ | <p>An identifier you assign to the stream processor. You can use <code>Name</code> to
            manage the stream processor. For example, you can get the current status of the stream processor by calling <a>DescribeStreamProcessor</a>.
            <code>Name</code> is idempotent. This is required for both face search and label detection stream processors.
       </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `regions_of_interest` | Vec<String> | <p>
            Specifies locations in the frames where Amazon Rekognition checks for objects or people. This is an optional parameter for label detection stream processors.
        </p> |
| `status` | String | <p>Current status of the stream processor.</p> |
| `status_message` | String | <p>Detailed status message about the stream processor.</p> |
| `data_sharing_preference` | String | <p>
            Shows whether you are sharing data with Rekognition to improve model performance. You can choose this option at the account level or on a per-stream basis.
            Note that if you opt out at the account level this setting is ignored on individual streams.
        </p> |
| `last_update_timestamp` | String | <p>The time, in Unix format, the stream processor was last updated. For example, when the stream
        processor moves from a running state to a failed state, or when the user starts or stops the stream processor.</p> |
| `output` | String | <p>Kinesis data stream to which Amazon Rekognition Video puts the analysis results.</p> |
| `notification_channel` | String |  |
| `stream_processor_arn` | String | <p>ARN of the stream processor.</p> |
| `creation_timestamp` | String | <p>Date and time the stream processor was created</p> |
| `kms_key_id` | String | <p>
            The identifier for your AWS Key Management Service key (AWS KMS key). This is an optional parameter for label detection stream processors.
        </p> |
| `role_arn` | String | <p>ARN of the IAM role that allows access to the stream processor.</p> |
| `input` | String | <p>Kinesis video stream that provides the source streaming video.</p> |
| `name` | String | <p>Name of the stream processor. </p> |
| `settings` | String | <p>Input parameters used in a streaming video analyzed by a stream processor. You can use <code>FaceSearch</code> to recognize faces
            in a streaming video, or you can use <code>ConnectedHome</code> to detect labels.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create stream_processor
stream_processor = provider.rekognition.Stream_processor {
    output = "value"  # <p>Kinesis data stream stream or Amazon S3 bucket location to which Amazon Rekognition Video puts the analysis results. If you are using the AWS CLI, the parameter name is <code>StreamProcessorOutput</code>. 
            This must be a <a>S3Destination</a> of an Amazon S3 bucket that you own for a label detection stream processor or a Kinesis data stream ARN for a face search stream processor.</p>
    settings = "value"  # <p>Input parameters used in a streaming video analyzed by a stream processor. You can use <code>FaceSearch</code> to recognize faces in a streaming video, or you can use <code>ConnectedHome</code> to detect labels.</p>
    role_arn = "value"  # <p>The Amazon Resource Number (ARN) of the IAM role that allows access to the stream processor. 
            The IAM role provides Rekognition read permissions for a Kinesis stream. 
            It also provides write permissions to an Amazon S3 bucket and Amazon Simple Notification Service topic for a label detection stream processor. This is required for both face search and label detection stream processors.</p>
    input = "value"  # <p>Kinesis video stream stream that provides the source streaming video. If you are using the AWS CLI, the parameter name is <code>StreamProcessorInput</code>. This is required for both face search and label detection stream processors.</p>
    name = "value"  # <p>An identifier you assign to the stream processor. You can use <code>Name</code> to
            manage the stream processor. For example, you can get the current status of the stream processor by calling <a>DescribeStreamProcessor</a>.
            <code>Name</code> is idempotent. This is required for both face search and label detection stream processors.
       </p>
}

# Access stream_processor outputs
stream_processor_id = stream_processor.id
stream_processor_regions_of_interest = stream_processor.regions_of_interest
stream_processor_status = stream_processor.status
stream_processor_status_message = stream_processor.status_message
stream_processor_data_sharing_preference = stream_processor.data_sharing_preference
stream_processor_last_update_timestamp = stream_processor.last_update_timestamp
stream_processor_output = stream_processor.output
stream_processor_notification_channel = stream_processor.notification_channel
stream_processor_stream_processor_arn = stream_processor.stream_processor_arn
stream_processor_creation_timestamp = stream_processor.creation_timestamp
stream_processor_kms_key_id = stream_processor.kms_key_id
stream_processor_role_arn = stream_processor.role_arn
stream_processor_input = stream_processor.input
stream_processor_name = stream_processor.name
stream_processor_settings = stream_processor.settings
```

---


### Content_moderation

ContentModeration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `video_metadata` | String | <p>Information about a video that Amazon Rekognition analyzed. <code>Videometadata</code>
     is returned in every page of paginated responses from <code>GetContentModeration</code>. </p> |
| `job_tag` | String | <p>A job identifier specified in the call to StartContentModeration and returned in the job
      completion notification sent to your Amazon Simple Notification Service topic.</p> |
| `moderation_model_version` | String | <p>Version number of the moderation detection model that was used to detect inappropriate, unwanted, or offensive content.</p> |
| `moderation_labels` | Vec<String> | <p>The detected inappropriate, unwanted, or offensive content moderation labels and the time(s) they were detected.</p> |
| `video` | String |  |
| `status_message` | String | <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p> |
| `job_id` | String | <p>Job identifier for the content moderation operation for which you want to obtain results.
      The job identifer is returned by an initial call to StartContentModeration.</p> |
| `job_status` | String | <p>The current status of the content moderation analysis job.</p> |
| `get_request_metadata` | String | <p>Information about the paramters used when getting a response. Includes information on
      aggregation and sorting methods.</p> |
| `next_token` | String | <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent
     request to retrieve the next set of content moderation labels. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access content_moderation outputs
content_moderation_id = content_moderation.id
content_moderation_video_metadata = content_moderation.video_metadata
content_moderation_job_tag = content_moderation.job_tag
content_moderation_moderation_model_version = content_moderation.moderation_model_version
content_moderation_moderation_labels = content_moderation.moderation_labels
content_moderation_video = content_moderation.video
content_moderation_status_message = content_moderation.status_message
content_moderation_job_id = content_moderation.job_id
content_moderation_job_status = content_moderation.job_status
content_moderation_get_request_metadata = content_moderation.get_request_metadata
content_moderation_next_token = content_moderation.next_token
```

---


### Media_analysis_job

MediaAnalysisJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_id` | String | <p>The identifier for the media analysis job.</p> |
| `operations_config` | String | <p>Operation configurations that were provided during job creation.</p> |
| `kms_key_id` | String | <p>KMS Key that was provided in the creation request.</p> |
| `failure_details` | String | <p>Details about the error that resulted in failure of the job.</p> |
| `creation_timestamp` | String | <p>The Unix date and time when the job was started.</p> |
| `completion_timestamp` | String | <p>The Unix date and time when the job finished.</p> |
| `job_name` | String | <p>The name of the media analysis job.</p> |
| `output_config` | String | <p>Output configuration that was provided in the creation request.</p> |
| `status` | String | <p>The current status of the media analysis job.</p> |
| `results` | String | <p>Output manifest that contains prediction results.</p> |
| `manifest_summary` | String | <p>The summary manifest provides statistics on input manifest and errors identified in the input manifest.</p> |
| `input` | String | <p>Reference to the input manifest that was provided in the job creation request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access media_analysis_job outputs
media_analysis_job_id = media_analysis_job.id
media_analysis_job_job_id = media_analysis_job.job_id
media_analysis_job_operations_config = media_analysis_job.operations_config
media_analysis_job_kms_key_id = media_analysis_job.kms_key_id
media_analysis_job_failure_details = media_analysis_job.failure_details
media_analysis_job_creation_timestamp = media_analysis_job.creation_timestamp
media_analysis_job_completion_timestamp = media_analysis_job.completion_timestamp
media_analysis_job_job_name = media_analysis_job.job_name
media_analysis_job_output_config = media_analysis_job.output_config
media_analysis_job_status = media_analysis_job.status
media_analysis_job_results = media_analysis_job.results
media_analysis_job_manifest_summary = media_analysis_job.manifest_summary
media_analysis_job_input = media_analysis_job.input
```

---


### Person_tracking

PersonTracking resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_id` | String | <p>Job identifier for the person tracking operation for which you want to obtain results. The
      job identifer is returned by an initial call to StartPersonTracking.</p> |
| `job_tag` | String | <p>A job identifier specified in the call to StartCelebrityRecognition and returned in the
      job completion notification sent to your Amazon Simple Notification Service topic.</p> |
| `persons` | Vec<String> | <p>An array of the persons detected in the video and the time(s) their path was tracked throughout the video.
        An array element will exist for each time a person's path is tracked. </p> |
| `video_metadata` | String | <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in
       every page of paginated responses from a Amazon Rekognition Video operation.</p> |
| `job_status` | String | <p>The current status of the person tracking job.</p> |
| `next_token` | String | <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of persons. </p> |
| `video` | String |  |
| `status_message` | String | <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access person_tracking outputs
person_tracking_id = person_tracking.id
person_tracking_job_id = person_tracking.job_id
person_tracking_job_tag = person_tracking.job_tag
person_tracking_persons = person_tracking.persons
person_tracking_video_metadata = person_tracking.video_metadata
person_tracking_job_status = person_tracking.job_status
person_tracking_next_token = person_tracking.next_token
person_tracking_video = person_tracking.video
person_tracking_status_message = person_tracking.status_message
```

---


### Face_liveness_session_results

FaceLivenessSessionResults resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `session_id` | String | <p>The sessionId for which this request was called.</p> |
| `confidence` | String | <p>Probabalistic confidence score for if the person in the given video was live, represented
      as a float value between 0 to 100.</p> |
| `audit_images` | Vec<String> | <p>A set of images from the Face Liveness video that can be used for audit purposes. It
      includes a bounding box of the face and the Base64-encoded bytes that return an image. If the
      CreateFaceLivenessSession request included an OutputConfig argument, the image will be
      uploaded to an S3Object specified in the output configuration. If no Amazon S3 bucket is defined,
      raw bytes are sent instead.</p> |
| `challenge` | String | <p>Contains information regarding the challenge type used for the Face Liveness check.</p> |
| `reference_image` | String | <p>A high-quality image from the Face Liveness video that can be used for face comparison or
      search. It includes a bounding box of the face and the Base64-encoded bytes that return an
      image. If the CreateFaceLivenessSession request included an OutputConfig argument, the image
      will be uploaded to an S3Object specified in the output configuration. In case the reference
      image is not returned, it's recommended to retry the Liveness check.</p> |
| `status` | String | <p>Represents a status corresponding to the state of the session. Possible statuses are:
      CREATED, IN_PROGRESS, SUCCEEDED, FAILED, EXPIRED.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access face_liveness_session_results outputs
face_liveness_session_results_id = face_liveness_session_results.id
face_liveness_session_results_session_id = face_liveness_session_results.session_id
face_liveness_session_results_confidence = face_liveness_session_results.confidence
face_liveness_session_results_audit_images = face_liveness_session_results.audit_images
face_liveness_session_results_challenge = face_liveness_session_results.challenge
face_liveness_session_results_reference_image = face_liveness_session_results.reference_image
face_liveness_session_results_status = face_liveness_session_results.status
```

---


### Face_search

FaceSearch resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `video_metadata` | String | <p>Information about a video that Amazon Rekognition analyzed. <code>Videometadata</code> is returned in every page of paginated responses
      from a Amazon Rekognition Video operation. </p> |
| `status_message` | String | <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p> |
| `persons` | Vec<String> | <p>An array of persons,  <a>PersonMatch</a>,
      in the video whose face(s) match the face(s) in an Amazon Rekognition collection. It also includes time information
       for when persons are matched in the video.
      You specify the input collection in an initial call to <code>StartFaceSearch</code>.
      Each  <code>Persons</code> element includes a time the person was matched,
      face match details (<code>FaceMatches</code>) for matching faces in the collection,
       and person information (<code>Person</code>) for the matched person. </p> |
| `video` | String |  |
| `job_status` | String | <p>The current status of the face search job.</p> |
| `job_tag` | String | <p>A job identifier specified in the call to StartFaceSearch and returned in the job
      completion notification sent to your Amazon Simple Notification Service topic.</p> |
| `job_id` | String | <p>Job identifier for the face search operation for which you want to obtain results. The job
      identifer is returned by an initial call to StartFaceSearch.</p> |
| `next_token` | String | <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of search results. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access face_search outputs
face_search_id = face_search.id
face_search_video_metadata = face_search.video_metadata
face_search_status_message = face_search.status_message
face_search_persons = face_search.persons
face_search_video = face_search.video
face_search_job_status = face_search.job_status
face_search_job_tag = face_search.job_tag
face_search_job_id = face_search.job_id
face_search_next_token = face_search.next_token
```

---


### User

User resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_id` | String | ✅ | <p>ID for the UserID to be created. This ID needs to be unique within the collection.</p> |
| `client_request_token` | String |  | <p>Idempotent token used to identify the request to <code>CreateUser</code>. If you use the
      same token with multiple <code>CreateUser</code> requests, the same response is returned. Use
      ClientRequestToken to prevent the same request from being processed more than once.</p> |
| `collection_id` | String | ✅ | <p>The ID of an existing collection to which the new UserID needs to be created.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user
user = provider.rekognition.User {
    user_id = "value"  # <p>ID for the UserID to be created. This ID needs to be unique within the collection.</p>
    collection_id = "value"  # <p>The ID of an existing collection to which the new UserID needs to be created.</p>
}

```

---


### Project_versions

ProjectVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If the previous response was incomplete (because there is more
         results to retrieve), Amazon Rekognition returns a pagination token in the response. 
         You can use this pagination token to retrieve the next set of results. </p> |
| `project_version_descriptions` | Vec<String> | <p>A list of project version descriptions. The list is sorted by the creation date and
         time of the project versions, latest to earliest.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access project_versions outputs
project_versions_id = project_versions.id
project_versions_next_token = project_versions.next_token
project_versions_project_version_descriptions = project_versions.project_version_descriptions
```

---


### Project_policy

ProjectPolicy resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_revision_id` | String |  | <p>The revision ID for the Project Policy. Each time you modify a policy, Amazon Rekognition Custom Labels
         generates and assigns a new <code>PolicyRevisionId</code> and then deletes the previous version of the
         policy.</p> |
| `policy_document` | String | ✅ | <p>A resource policy to add to the model. The policy is a JSON structure that contains
         one or more statements that define the policy. 
         The policy must follow the IAM syntax. For
         more information about the contents of a JSON policy document, see 
         <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON policy reference</a>. </p> |
| `project_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the project that the project policy is attached to.</p> |
| `policy_name` | String | ✅ | <p>A name for the policy.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create project_policy
project_policy = provider.rekognition.Project_policy {
    policy_document = "value"  # <p>A resource policy to add to the model. The policy is a JSON structure that contains
         one or more statements that define the policy. 
         The policy must follow the IAM syntax. For
         more information about the contents of a JSON policy document, see 
         <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON policy reference</a>. </p>
    project_arn = "value"  # <p>The Amazon Resource Name (ARN) of the project that the project policy is attached to.</p>
    policy_name = "value"  # <p>A name for the policy.</p>
}

```

---


### Dataset

Dataset resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>A set of tags (key-value pairs) that you want to attach to the dataset.</p> |
| `dataset_source` | String |  | <p>
The source files for the dataset. You can specify the ARN of an existing dataset or specify the Amazon S3 bucket location
of an Amazon Sagemaker format manifest file. If you don't specify <code>datasetSource</code>, an empty dataset is created.
  To add labeled images to the dataset,  You can use the console or call <a>UpdateDatasetEntries</a>.
  
</p> |
| `dataset_type` | String | ✅ | <p>
The type of the dataset. Specify <code>TRAIN</code> to create a training dataset. Specify <code>TEST</code> 
   to create a test dataset.
</p> |
| `project_arn` | String | ✅ | <p>
The ARN of the Amazon Rekognition Custom Labels project to which you want to asssign the dataset.
</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dataset_description` | String | <p>
The description for the dataset.
</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dataset
dataset = provider.rekognition.Dataset {
    dataset_type = "value"  # <p>
The type of the dataset. Specify <code>TRAIN</code> to create a training dataset. Specify <code>TEST</code> 
   to create a test dataset.
</p>
    project_arn = "value"  # <p>
The ARN of the Amazon Rekognition Custom Labels project to which you want to asssign the dataset.
</p>
}

# Access dataset outputs
dataset_id = dataset.id
dataset_dataset_description = dataset.dataset_description
```

---


### Projects

Projects resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If the previous response was incomplete (because there is more
         results to retrieve), Amazon Rekognition returns a pagination token in the response. 
         You can use this pagination token to retrieve the next set of results. </p> |
| `project_descriptions` | Vec<String> | <p>A list of project descriptions. The list is sorted by the date and time the projects are created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access projects outputs
projects_id = projects.id
projects_next_token = projects.next_token
projects_project_descriptions = projects.project_descriptions
```

---


### Celebrity_info

CelebrityInfo resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `urls` | Vec<String> | <p>An array of URLs pointing to additional celebrity information. </p> |
| `name` | String | <p>The name of the celebrity.</p> |
| `known_gender` | String | <p>Retrieves the known gender for the celebrity.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access celebrity_info outputs
celebrity_info_id = celebrity_info.id
celebrity_info_urls = celebrity_info.urls
celebrity_info_name = celebrity_info.name
celebrity_info_known_gender = celebrity_info.known_gender
```

---


### Face_liveness_session

FaceLivenessSession resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kms_key_id` | String |  | <p> The identifier for your AWS Key Management Service key (AWS KMS key). Used to encrypt
      audit images and reference images.</p> |
| `client_request_token` | String |  | <p>Idempotent token is used to recognize the Face Liveness request. If the same token is used
      with multiple <code>CreateFaceLivenessSession</code> requests, the same session is returned.
      This token is employed to avoid unintentionally creating the same session multiple
      times.</p> |
| `settings` | String |  | <p>A session settings object. It contains settings for the operation to be performed. For
      Face Liveness, it accepts <code>OutputConfig</code> and <code>AuditImagesLimit</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create face_liveness_session
face_liveness_session = provider.rekognition.Face_liveness_session {
}

```

---


### Face_detection

FaceDetection resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status_message` | String | <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p> |
| `video` | String |  |
| `next_token` | String | <p>If the response is truncated, Amazon Rekognition returns this token that you can use in the subsequent request to retrieve the next set of faces. </p> |
| `job_id` | String | <p>Job identifier for the face detection operation for which you want to obtain results. The
      job identifer is returned by an initial call to StartFaceDetection.</p> |
| `job_tag` | String | <p>A job identifier specified in the call to StartFaceDetection and returned in the job
      completion notification sent to your Amazon Simple Notification Service topic.</p> |
| `faces` | Vec<String> | <p>An array of faces detected in the video. Each element contains a detected face's details and the time,
       in milliseconds from the start of the video, the face was detected. </p> |
| `job_status` | String | <p>The current status of the face detection job.</p> |
| `video_metadata` | String | <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in
       every page of paginated responses from a Amazon Rekognition video operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access face_detection outputs
face_detection_id = face_detection.id
face_detection_status_message = face_detection.status_message
face_detection_video = face_detection.video
face_detection_next_token = face_detection.next_token
face_detection_job_id = face_detection.job_id
face_detection_job_tag = face_detection.job_tag
face_detection_faces = face_detection.faces
face_detection_job_status = face_detection.job_status
face_detection_video_metadata = face_detection.video_metadata
```

---


### Label_detection

LabelDetection resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_id` | String | <p>Job identifier for the label detection operation for which you want to obtain results. The
      job identifer is returned by an initial call to StartLabelDetection.</p> |
| `label_model_version` | String | <p>Version number of the label detection model that was used to detect labels.</p> |
| `labels` | Vec<String> | <p>An array of labels detected in the video. Each element contains the detected label and the time,
        in milliseconds from the start of the video, that the label was detected. </p> |
| `status_message` | String | <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p> |
| `job_status` | String | <p>The current status of the label detection job.</p> |
| `video` | String |  |
| `get_request_metadata` | String | <p>Information about the paramters used when getting a response. Includes information on
      aggregation and sorting methods.</p> |
| `video_metadata` | String | <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in
       every page of paginated responses from a Amazon Rekognition video operation.</p> |
| `next_token` | String | <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request
        to retrieve the next set of labels.</p> |
| `job_tag` | String | <p>A job identifier specified in the call to StartLabelDetection and returned in the job
      completion notification sent to your Amazon Simple Notification Service topic.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access label_detection outputs
label_detection_id = label_detection.id
label_detection_job_id = label_detection.job_id
label_detection_label_model_version = label_detection.label_model_version
label_detection_labels = label_detection.labels
label_detection_status_message = label_detection.status_message
label_detection_job_status = label_detection.job_status
label_detection_video = label_detection.video
label_detection_get_request_metadata = label_detection.get_request_metadata
label_detection_video_metadata = label_detection.video_metadata
label_detection_next_token = label_detection.next_token
label_detection_job_tag = label_detection.job_tag
```

---


### Project_version

ProjectVersion resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `training_data` | String |  | <p>Specifies an external manifest that the services uses to train the project version.
         If you specify <code>TrainingData</code> you must also specify <code>TestingData</code>.
         The project must not have any associated datasets. </p> |
| `project_arn` | String | ✅ | <p>The ARN of the Amazon Rekognition project that will manage the project version you want to
         train.</p> |
| `version_description` | String |  | <p>A description applied to the project version being created.</p> |
| `kms_key_id` | String |  | <p>The identifier for your AWS Key Management Service key (AWS KMS key). You can supply
         the Amazon Resource Name (ARN) of your KMS key, the ID of your KMS key, an alias for
         your KMS key, or an alias ARN. The key is used to encrypt training images, test images, and manifest files copied
         into the service for the project version. Your source images are unaffected. The
         key is also used to encrypt training results and manifest files written to the output Amazon S3
         bucket (<code>OutputConfig</code>).</p>
         <p>If you choose to use your own KMS key, you need the following permissions on the KMS key.</p>
         <ul>
            <li>
               <p>kms:CreateGrant</p>
            </li>
            <li>
               <p>kms:DescribeKey</p>
            </li>
            <li>
               <p>kms:GenerateDataKey</p>
            </li>
            <li>
               <p>kms:Decrypt</p>
            </li>
         </ul>
         <p>If you don't specify a value for <code>KmsKeyId</code>, images copied into the service are encrypted
         using a key that AWS owns and manages.</p> |
| `testing_data` | String |  | <p>Specifies an external manifest that the service uses to test the project version. If
         you specify <code>TestingData</code> you must also specify <code>TrainingData</code>. The
         project must not have any associated datasets.</p> |
| `output_config` | String | ✅ | <p>The Amazon S3 bucket location to store the results of training. The bucket can be any S3
         bucket in your AWS account. You need <code>s3:PutObject</code> permission on the bucket.
      </p> |
| `tags` | HashMap<String, String> |  | <p> A set of tags (key-value pairs) that you want to attach to the project version. </p> |
| `version_name` | String | ✅ | <p>A name for the version of the project version. This value must be unique.</p> |
| `feature_config` | String |  | <p>Feature-specific configuration of the training job. If the job configuration does not match the feature type associated with the project, an InvalidParameterException is returned.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create project_version
project_version = provider.rekognition.Project_version {
    project_arn = "value"  # <p>The ARN of the Amazon Rekognition project that will manage the project version you want to
         train.</p>
    output_config = "value"  # <p>The Amazon S3 bucket location to store the results of training. The bucket can be any S3
         bucket in your AWS account. You need <code>s3:PutObject</code> permission on the bucket.
      </p>
    version_name = "value"  # <p>A name for the version of the project version. This value must be unique.</p>
}

```

---


### Segment_detection

SegmentDetection resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `video_metadata` | Vec<String> | <p>Currently, Amazon Rekognition Video returns a single   object in the
      <code>VideoMetadata</code> array. The object
      contains information about the video stream in the input file that Amazon Rekognition Video chose to analyze.  
      The <code>VideoMetadata</code> object includes the video codec, video format and other information.
      Video metadata is returned in each page of information returned by <code>GetSegmentDetection</code>.</p> |
| `job_id` | String | <p>Job identifier for the segment detection operation for which you want to obtain results.
      The job identifer is returned by an initial call to StartSegmentDetection.</p> |
| `video` | String |  |
| `job_tag` | String | <p>A job identifier specified in the call to StartSegmentDetection and returned in the job
      completion notification sent to your Amazon Simple Notification Service topic.</p> |
| `job_status` | String | <p>Current status of the segment detection job.</p> |
| `audio_metadata` | Vec<String> | <p>An array of 
       objects. There can be multiple audio streams. 
      Each <code>AudioMetadata</code> object contains metadata for a single audio stream.
      Audio information in an <code>AudioMetadata</code> objects includes 
      the audio codec, the number of audio channels, the duration of the audio stream,
      and the sample rate. Audio metadata is returned in each page of information returned
      by <code>GetSegmentDetection</code>.</p> |
| `next_token` | String | <p>If the previous response was incomplete (because there are more labels to retrieve), Amazon Rekognition Video returns 
      a pagination token in the response. You can use this pagination token to retrieve the next set of text.</p> |
| `selected_segment_types` | Vec<String> | <p>An array containing the segment types requested in the call to <code>StartSegmentDetection</code>.
    </p> |
| `segments` | Vec<String> | <p>An array of segments detected in a video.  The array is sorted by the segment types (TECHNICAL_CUE or SHOT) 
      specified in the <code>SegmentTypes</code> input parameter of <code>StartSegmentDetection</code>. Within
      each segment type the array is sorted by timestamp values.</p> |
| `status_message` | String | <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access segment_detection outputs
segment_detection_id = segment_detection.id
segment_detection_video_metadata = segment_detection.video_metadata
segment_detection_job_id = segment_detection.job_id
segment_detection_video = segment_detection.video
segment_detection_job_tag = segment_detection.job_tag
segment_detection_job_status = segment_detection.job_status
segment_detection_audio_metadata = segment_detection.audio_metadata
segment_detection_next_token = segment_detection.next_token
segment_detection_selected_segment_types = segment_detection.selected_segment_types
segment_detection_segments = segment_detection.segments
segment_detection_status_message = segment_detection.status_message
```

---


### Faces

Faces resource

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


### Text_detection

TextDetection resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `video` | String |  |
| `text_detections` | Vec<String> | <p>An array of text detected in the video. Each element contains the detected text, the time in milliseconds
      from the start of the video that the text was detected, and where it was detected on the screen.</p> |
| `text_model_version` | String | <p>Version number of the text detection model that was used to detect text.</p> |
| `job_tag` | String | <p>A job identifier specified in the call to StartTextDetection and returned in the job
      completion notification sent to your Amazon Simple Notification Service topic.</p> |
| `next_token` | String | <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent 
        request to retrieve the next set of text.</p> |
| `job_status` | String | <p>Current status of the text detection job.</p> |
| `status_message` | String | <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p> |
| `video_metadata` | String |  |
| `job_id` | String | <p>Job identifier for the text detection operation for which you want to obtain results. The
      job identifer is returned by an initial call to StartTextDetection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access text_detection outputs
text_detection_id = text_detection.id
text_detection_video = text_detection.video
text_detection_text_detections = text_detection.text_detections
text_detection_text_model_version = text_detection.text_model_version
text_detection_job_tag = text_detection.job_tag
text_detection_next_token = text_detection.next_token
text_detection_job_status = text_detection.job_status
text_detection_status_message = text_detection.status_message
text_detection_video_metadata = text_detection.video_metadata
text_detection_job_id = text_detection.job_id
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple dataset_entries resources
dataset_entries_0 = provider.rekognition.Dataset_entries {
    changes = "value-0"
    dataset_arn = "value-0"
}
dataset_entries_1 = provider.rekognition.Dataset_entries {
    changes = "value-1"
    dataset_arn = "value-1"
}
dataset_entries_2 = provider.rekognition.Dataset_entries {
    changes = "value-2"
    dataset_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    dataset_entries = provider.rekognition.Dataset_entries {
        changes = "production-value"
        dataset_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Rekognition Documentation](https://docs.aws.amazon.com/rekognition/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
