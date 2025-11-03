# Elastic_transcoder Service



**Resources**: 5

---

## Overview

The elastic_transcoder service provides access to 5 resource types:

- [Pipeline](#pipeline) [CUD]
- [Pipeline_notifications](#pipeline_notifications) [U]
- [Preset](#preset) [CD]
- [Job](#job) [C]
- [Pipeline_status](#pipeline_status) [U]

---

## Resources


### Pipeline

Pipeline resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_kms_key_arn` | String |  | <p>The AWS Key Management Service (AWS KMS) key that you want to use with this pipeline.</p>
        <p>If you use either <code>s3</code> or <code>s3-aws-kms</code> as your 
            <code>Encryption:Mode</code>, you don't need to provide a key with
            your job because a default key, known as an AWS-KMS key, is created for you automatically.
            You need to provide an AWS-KMS key only if you want to use a non-default AWS-KMS key, or if you are
            using an <code>Encryption:Mode</code> of <code>aes-cbc-pkcs7</code>, <code>aes-ctr</code>,
            or <code>aes-gcm</code>.</p> |
| `input_bucket` | String | ✅ | <p>The Amazon S3 bucket in which you saved the media files that you want to transcode.</p> |
| `notifications` | String |  | <p>The Amazon Simple Notification Service (Amazon SNS) topic that you want to notify to report job status.</p>
        <important>
            <p>To receive notifications, you must also subscribe to the new topic in the Amazon SNS console.</p>
         </important>
        <ul>
            <li>
               <p>
                  <b>Progressing</b>: The topic ARN for the Amazon Simple Notification Service (Amazon SNS) topic that you want to
                notify when Elastic Transcoder has started to process a job in this pipeline. This is the ARN that
                Amazon SNS returned when you created the topic. For more information, see Create a
                Topic in the Amazon Simple Notification Service Developer Guide.</p>
            </li>
            <li>
               <p>
                  <b>Complete</b>: The topic ARN for the Amazon SNS topic that you want to notify when
                Elastic Transcoder has finished processing a job in this pipeline. This is the ARN that Amazon SNS
                returned when you created the topic.</p>
            </li>
            <li>
               <p>
                  <b>Warning</b>: The topic ARN for the Amazon SNS topic that you want to notify when Elastic Transcoder
                encounters a warning condition while processing a job in this pipeline. This is the
                ARN that Amazon SNS returned when you created the topic.</p>
            </li>
            <li>
               <p>
                  <b>Error</b>: The topic ARN for the Amazon SNS topic that you want to notify when Elastic Transcoder
                encounters an error condition while processing a job in this pipeline. This is the
                ARN that Amazon SNS returned when you created the topic.</p>
            </li>
         </ul> |
| `thumbnail_config` | String |  | <p>The <code>ThumbnailConfig</code> object specifies several values, including the Amazon S3
            bucket in which you want Elastic Transcoder to save thumbnail files, which users you want to have
            access to the files, the type of access you want users to have, and the storage class
            that you want to assign to the files.</p>
        <p>If you specify values for <code>ContentConfig</code>, you must also specify values for
                <code>ThumbnailConfig</code> even if you don't want to create thumbnails.</p>
        <p>If you specify values for <code>ContentConfig</code> and <code>ThumbnailConfig</code>,
            omit the <code>OutputBucket</code> object.</p>
        <ul>
            <li>
               <p>
                  <b>Bucket</b>: The Amazon S3 bucket in which you want Elastic Transcoder to save
                thumbnail files.</p>
            </li>
            <li>
               <p>
                  <b>Permissions</b> (Optional): The <code>Permissions</code> object specifies which
                users and/or predefined Amazon S3 groups you want to have access to thumbnail files,
                and the type of access you want them to have. You can grant permissions to a maximum
                of 30 users and/or predefined Amazon S3 groups.</p>
            </li>
            <li>
               <p>
                  <b>GranteeType</b>: Specify the type of value that appears in the Grantee object: </p>
               <ul>
                  <li>
                     <p>
                        <b>Canonical</b>: The value in the <code>Grantee</code> object is either the
                        canonical user ID for an AWS account or an origin access identity for an
                        Amazon CloudFront distribution.</p> 
                     <important>
                        <p>A canonical user ID is not the
                            same as an AWS account number.</p>
                     </important>
                  </li>
                  <li>
                     <p>
                        <b>Email</b>: The value in the <code>Grantee</code> object is the registered
                        email address of an AWS account. </p>
                  </li>
                  <li>
                     <p>
                        <b>Group</b>: The value in the <code>Grantee</code> object is one of the
                        following predefined Amazon S3 groups: <code>AllUsers</code>,
                            <code>AuthenticatedUsers</code>, or <code>LogDelivery</code>.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>Grantee</b>: The AWS user or group that you want to have access to thumbnail
                files. To identify the user or group, you can specify the canonical user ID for an
                AWS account, an origin access identity for a CloudFront distribution, the registered
                email address of an AWS account, or a predefined Amazon S3 group. </p>
            </li>
            <li>
               <p>
                  <b>Access</b>: The permission that you want to give to the AWS user that you
                specified in <code>Grantee</code>. Permissions are granted on the thumbnail files
                that Elastic Transcoder adds to the bucket. Valid values include: </p>
               <ul>
                  <li>
                     <p>
                        <code>READ</code>: The grantee can read the thumbnails and metadata for
                        objects that Elastic Transcoder adds to the Amazon S3 bucket.</p>
                  </li>
                  <li>
                     <p>
                        <code>READ_ACP</code>: The grantee can read the object ACL for thumbnails
                        that Elastic Transcoder adds to the Amazon S3 bucket.</p> 
                  </li>
                  <li>
                     <p>
                        <code>WRITE_ACP</code>: The grantee can write the ACL for the thumbnails
                        that Elastic Transcoder adds to the Amazon S3 bucket.</p>
                  </li>
                  <li>
                     <p>
                        <code>FULL_CONTROL</code>: The grantee has <code>READ</code>,
                            <code>READ_ACP</code>, and <code>WRITE_ACP</code> permissions for the
                        thumbnails that Elastic Transcoder adds to the Amazon S3 bucket.</p> 
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>StorageClass</b>: The Amazon S3 storage class, <code>Standard</code> or
                    <code>ReducedRedundancy</code>, that you want Elastic Transcoder to assign to
                the thumbnails that it stores in your Amazon S3 bucket.</p>
            </li>
         </ul> |
| `content_config` | String |  | <p>The optional <code>ContentConfig</code> object specifies information about the Amazon S3
            bucket in which you want Elastic Transcoder to save transcoded files and playlists:
            which bucket to use, which users you want to have access to the files, the type of
            access you want users to have, and the storage class that you want to assign to the
            files.</p>
        <p>If you specify values for <code>ContentConfig</code>, you must also specify values for
                <code>ThumbnailConfig</code>.</p>
        <p>If you specify values for <code>ContentConfig</code> and <code>ThumbnailConfig</code>,
            omit the <code>OutputBucket</code> object.</p>
        <ul>
            <li>
               <p>
                  <b>Bucket</b>: The Amazon S3 bucket in which you want Elastic Transcoder to save
                transcoded files and playlists.</p>
            </li>
            <li>
               <p>
                  <b>Permissions</b> (Optional): The Permissions object specifies which users you want
                to have access to transcoded files and the type of access you want them to have. You
                can grant permissions to a maximum of 30 users and/or predefined Amazon S3
                groups.</p>
            </li>
            <li>
               <p>
                  <b>Grantee Type</b>: Specify the type of value that appears in the
                    <code>Grantee</code> object: </p>
					          <ul>
                  <li>
                     <p>
                        <b>Canonical</b>: The value in the <code>Grantee</code> object is either the
                        canonical user ID for an AWS account or an origin access identity for an
                        Amazon CloudFront distribution. For more information about canonical user
                        IDs, see Access Control List (ACL) Overview in the Amazon Simple Storage
                        Service Developer Guide. For more information about using CloudFront origin
                        access identities to require that users use CloudFront URLs instead of
                        Amazon S3 URLs, see Using an Origin Access Identity to Restrict Access to
                        Your Amazon S3 Content.</p> 
						               <important>
                        <p>A canonical user ID is not the same as an
                            AWS account number.</p>
                     </important>
                    </li>
                  <li>
                     <p>
                        <b>Email</b>: The value in the <code>Grantee</code> object is the registered
                        email address of an AWS account.</p>
                  </li>
                  <li>
                     <p>
                        <b>Group</b>: The value in the <code>Grantee</code> object is one of the
                        following predefined Amazon S3 groups: <code>AllUsers</code>,
                            <code>AuthenticatedUsers</code>, or <code>LogDelivery</code>.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>Grantee</b>: The AWS user or group that you want to have access to transcoded
                files and playlists. To identify the user or group, you can specify the canonical
                user ID for an AWS account, an origin access identity for a CloudFront distribution,
                the registered email address of an AWS account, or a predefined Amazon S3 group </p>
            </li>
            <li>
               <p>
                  <b>Access</b>: The permission that you want to give to the AWS user that you
                specified in <code>Grantee</code>. Permissions are granted on the files that Elastic
                Transcoder adds to the bucket, including playlists and video files. Valid values
                include: </p>
               <ul>
                  <li>
                     <p>
                        <code>READ</code>: The grantee can read the objects and metadata for objects
                        that Elastic Transcoder adds to the Amazon S3 bucket.</p>
                  </li>
                  <li>
                     <p>
                        <code>READ_ACP</code>: The grantee can read the object ACL for objects that
                        Elastic Transcoder adds to the Amazon S3 bucket.</p>
                  </li>
                  <li>
                     <p>
                        <code>WRITE_ACP</code>: The grantee can write the ACL for the objects that
                        Elastic Transcoder adds to the Amazon S3 bucket.</p>
                  </li>
                  <li>
                     <p>
                        <code>FULL_CONTROL</code>: The grantee has <code>READ</code>,
                            <code>READ_ACP</code>, and <code>WRITE_ACP</code> permissions for the
                        objects that Elastic Transcoder adds to the Amazon S3 bucket.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>StorageClass</b>: The Amazon S3 storage class, <code>Standard</code> or
                    <code>ReducedRedundancy</code>, that you want Elastic Transcoder to assign to
                the video files and playlists that it stores in your Amazon S3 bucket.</p>
            </li>
         </ul> |
| `output_bucket` | String |  | <p>The Amazon S3 bucket in which you want Elastic Transcoder to save the transcoded files. (Use 
		  this, or use ContentConfig:Bucket plus ThumbnailConfig:Bucket.)</p>
        <p>Specify this value when all of the following are true:</p> 
		       <ul>
            <li>
               <p>You want to save transcoded files, thumbnails (if any), and playlists (if any)
                    together in one bucket.</p>
            </li>
            <li>
               <p>You do not want to specify the users or groups who have access to the transcoded
                    files, thumbnails, and playlists.</p>
            </li>
            <li>
               <p>You do not want to specify the permissions that Elastic Transcoder grants to the   
				
                    files. </p>
				           <important>
                  <p>When Elastic Transcoder saves files in
                            <code>OutputBucket</code>, it grants full control over the files only to
                        the AWS account that owns the role that is specified by
                        <code>Role</code>.</p>
               </important>
            </li>
            <li>
               <p>You want to associate the transcoded files and thumbnails with the Amazon S3
                    Standard storage class.</p>
            </li>
         </ul>

        <p>If you want to save transcoded files and playlists in one bucket and thumbnails in
            another bucket, specify which users can access the transcoded files or the permissions
            the users have, or change the Amazon S3 storage class, omit <code>OutputBucket</code>
            and specify values for <code>ContentConfig</code> and <code>ThumbnailConfig</code>
            instead.</p> |
| `name` | String | ✅ | <p>The name of the pipeline. We recommend that the name be unique within the AWS account, 
            but uniqueness is not enforced.</p>
        <p>Constraints: Maximum 40 characters.</p> |
| `role` | String | ✅ | <p>The IAM Amazon Resource Name (ARN) for the role that you want Elastic Transcoder to use to create the pipeline.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create pipeline
pipeline = provider.elastic_transcoder.Pipeline {
    input_bucket = "value"  # <p>The Amazon S3 bucket in which you saved the media files that you want to transcode.</p>
    name = "value"  # <p>The name of the pipeline. We recommend that the name be unique within the AWS account, 
            but uniqueness is not enforced.</p>
        <p>Constraints: Maximum 40 characters.</p>
    role = "value"  # <p>The IAM Amazon Resource Name (ARN) for the role that you want Elastic Transcoder to use to create the pipeline.</p>
}

```

---


### Pipeline_notifications

PipelineNotifications resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ | <p>The identifier of the pipeline for which you want to change notification settings.</p> |
| `notifications` | String | ✅ | <p>The topic ARN for the Amazon Simple Notification Service (Amazon SNS) topic that you want to notify to report job status.</p>
        <important>
            <p>To receive notifications, you must also subscribe to the new topic in the Amazon SNS console.</p>
         </important>
        <ul>
            <li>
               <p>
                  <b>Progressing</b>: The topic ARN for the Amazon Simple Notification Service (Amazon SNS) topic that you want to
                notify when Elastic Transcoder has started to process jobs that are added to this pipeline. This
                is the ARN that Amazon SNS returned when you created the topic.</p>
            </li>
            <li>
               <p>
                  <b>Complete</b>: The topic ARN for the Amazon SNS topic that you want to notify when
                Elastic Transcoder has finished processing a job. This is the ARN that Amazon SNS returned when
                you created the topic.</p>
            </li>
            <li>
               <p>
                  <b>Warning</b>: The topic ARN for the Amazon SNS topic that you want to notify when Elastic Transcoder
                encounters a warning condition. This is the ARN that Amazon SNS returned when you
                created the topic.</p>
            </li>
            <li>
               <p>
                  <b>Error</b>: The topic ARN for the Amazon SNS topic that you want to notify when Elastic Transcoder
                encounters an error condition. This is the ARN that Amazon SNS returned when you
                created the topic.</p>
            </li>
         </ul> |



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


### Preset

Preset resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `container` | String | ✅ | <p>The container type for the output file. Valid values include <code>flac</code>,
            <code>flv</code>, <code>fmp4</code>, 
            <code>gif</code>, <code>mp3</code>, 
            <code>mp4</code>, <code>mpg</code>, <code>mxf</code>, <code>oga</code>, 
            <code>ogg</code>, <code>ts</code>, and <code>webm</code>.</p> |
| `video` | String |  | <p>A section of the request body that specifies the video parameters.</p> |
| `audio` | String |  | <p>A section of the request body that specifies the audio parameters.</p> |
| `description` | String |  | <p>A description of the preset.</p> |
| `thumbnails` | String |  | <p>A section of the request body that specifies the thumbnail parameters, if any.</p> |
| `name` | String | ✅ | <p>The name of the preset. We recommend that the name be unique within the AWS account, but uniqueness is not enforced.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create preset
preset = provider.elastic_transcoder.Preset {
    container = "value"  # <p>The container type for the output file. Valid values include <code>flac</code>,
            <code>flv</code>, <code>fmp4</code>, 
            <code>gif</code>, <code>mp3</code>, 
            <code>mp4</code>, <code>mpg</code>, <code>mxf</code>, <code>oga</code>, 
            <code>ogg</code>, <code>ts</code>, and <code>webm</code>.</p>
    name = "value"  # <p>The name of the preset. We recommend that the name be unique within the AWS account, but uniqueness is not enforced.</p>
}

```

---


### Job

Job resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `playlists` | Vec<String> |  | <p>If you specify a preset in <code>PresetId</code> for which the value of
                <code>Container</code> is fmp4 (Fragmented MP4) or ts (MPEG-TS), Playlists contains 
                information about the master playlists that you want Elastic Transcoder to create.</p>
        <p>The maximum number of master playlists in a job is 30.</p> |
| `output` | String |  | <p> A section of the request body that provides information about the transcoded (target)
            file. We strongly recommend that you use the <code>Outputs</code> syntax instead of the
            <code>Output</code> syntax. </p> |
| `output_key_prefix` | String |  | <p>The value, if any, that you want Elastic Transcoder to prepend to the names of all files that this 
            job creates, including output files, thumbnails, and playlists.</p> |
| `pipeline_id` | String | ✅ | <p>The <code>Id</code> of the pipeline that you want Elastic Transcoder to use for
            transcoding. The pipeline determines several settings, including the Amazon S3 bucket
            from which Elastic Transcoder gets the files to transcode and the bucket into which
            Elastic Transcoder puts the transcoded files.</p> |
| `outputs` | Vec<String> |  | <p> A section of the request body that provides information about the transcoded (target)
            files. We recommend that you use the <code>Outputs</code> syntax instead of the
                <code>Output</code> syntax. </p> |
| `input` | String |  | <p>A section of the request body that provides information about the file that is being
             transcoded.</p> |
| `user_metadata` | HashMap<String, String> |  | <p>User-defined metadata that you want to associate with an Elastic Transcoder job. You specify metadata in 
            <code>key/value</code> pairs, and you can add up to 10 <code>key/value</code> pairs per job. 
            Elastic Transcoder does not guarantee that <code>key/value</code> pairs are returned in the same 
            order in which you specify them.</p> |
| `inputs` | Vec<String> |  | <p>A section of the request body that provides information about the files that are being 
            transcoded.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create job
job = provider.elastic_transcoder.Job {
    pipeline_id = "value"  # <p>The <code>Id</code> of the pipeline that you want Elastic Transcoder to use for
            transcoding. The pipeline determines several settings, including the Amazon S3 bucket
            from which Elastic Transcoder gets the files to transcode and the bucket into which
            Elastic Transcoder puts the transcoded files.</p>
}

```

---


### Pipeline_status

PipelineStatus resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ | <p>The identifier of the pipeline to update.</p> |
| `status` | String | ✅ | <p>The desired status of the pipeline:</p>
        <ul>
            <li>
               <p>
                  <code>Active</code>: The pipeline is processing jobs.</p>
            </li>
            <li>
               <p>
                  <code>Paused</code>: The pipeline is not currently processing jobs.</p>
            </li>
         </ul> |



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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple pipeline resources
pipeline_0 = provider.elastic_transcoder.Pipeline {
    input_bucket = "value-0"
    name = "value-0"
    role = "value-0"
}
pipeline_1 = provider.elastic_transcoder.Pipeline {
    input_bucket = "value-1"
    name = "value-1"
    role = "value-1"
}
pipeline_2 = provider.elastic_transcoder.Pipeline {
    input_bucket = "value-2"
    name = "value-2"
    role = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    pipeline = provider.elastic_transcoder.Pipeline {
        input_bucket = "production-value"
        name = "production-value"
        role = "production-value"
    }
```

---

## Related Documentation

- [AWS Elastic_transcoder Documentation](https://docs.aws.amazon.com/elastic_transcoder/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
