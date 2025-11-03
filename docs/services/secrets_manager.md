# Secrets_manager Service



**Resources**: 5

---

## Overview

The secrets_manager service provides access to 5 resource types:

- [Random_password](#random_password) [R]
- [Secret_value](#secret_value) [CR]
- [Secret](#secret) [CRUD]
- [Resource_policy](#resource_policy) [CRD]
- [Secret_version_stage](#secret_version_stage) [U]

---

## Resources


### Random_password

RandomPassword resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `random_password` | String | <p>A string with the password.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access random_password outputs
random_password_id = random_password.id
random_password_random_password = random_password.random_password
```

---


### Secret_value

SecretValue resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `secret_id` | String | ✅ | <p>The ARN or name of the secret to add a new version to.</p>
         <p>For an ARN, we recommend that you specify a complete ARN rather 
      than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
         <p>If the secret doesn't already exist, use <code>CreateSecret</code> instead.</p> |
| `client_request_token` | String |  | <p>A unique identifier for the new version of the secret. </p>
         <note>
            <p>If you use the Amazon Web Services CLI or one of the Amazon Web Services SDKs to call this operation, then you can leave this parameter empty. The CLI or SDK generates a random UUID for you and includes it as the value for this parameter in the request. </p>
         </note>
         <p>If you generate a raw HTTP request to the Secrets Manager service endpoint, then you must generate a <code>ClientRequestToken</code> and include it in the request.</p>
         <p>This value helps ensure idempotency. Secrets Manager uses this value to prevent the accidental creation of duplicate versions if there are failures and retries during a rotation. We recommend that you generate a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID-type</a> value to ensure uniqueness of your versions within the specified secret. </p>
         <ul>
            <li>
               <p>If the <code>ClientRequestToken</code> value isn't already associated with a version
          of the secret then a new version of the secret is created. </p>
            </li>
            <li>
               <p>If a version with this value already exists and that version's
          <code>SecretString</code> or <code>SecretBinary</code> values are the same as those in
          the request then the request is ignored. The operation is idempotent. </p>
            </li>
            <li>
               <p>If a version with this value already exists and the version of the
          <code>SecretString</code> and <code>SecretBinary</code> values are different from those
          in the request, then the request fails because you can't modify a secret 
          version. You can only create new versions to store new secret values.</p>
            </li>
         </ul>
         <p>This value becomes the <code>VersionId</code> of the new version.</p> |
| `version_stages` | Vec<String> |  | <p>A list of staging labels to attach to this version of the
      secret. Secrets Manager uses staging labels to track versions of a secret through the rotation process.</p>
         <p>If you specify a staging
      label that's already associated with a different version of the same secret, then Secrets Manager  
      removes the label from the other version and attaches it to this version. 
      If you specify 
      <code>AWSCURRENT</code>, and it is already attached to another version, then Secrets Manager also  
      moves the staging label <code>AWSPREVIOUS</code> to the version that <code>AWSCURRENT</code> was removed from.</p>
         <p>If you don't include <code>VersionStages</code>, then Secrets Manager automatically
      moves the staging label <code>AWSCURRENT</code> to this version.</p> |
| `secret_binary` | String |  | <p>The binary data to encrypt and store in the new version of
      the secret. To use this parameter in the command-line tools, we recommend that you store your
      binary data in a file and then pass the
      contents of the file as a parameter. </p>
         <p>You must include <code>SecretBinary</code> or <code>SecretString</code>, but not both.</p>
         <p>You can't access this value from the Secrets Manager console.</p>
         <p>Sensitive: This field contains sensitive information, so the service does not include it in CloudTrail log entries. If you create your own log entries, you must also avoid logging the information in this field.</p> |
| `secret_string` | String |  | <p>The text to encrypt and store in the new version of the secret. </p>
         <p>You must include <code>SecretBinary</code> or <code>SecretString</code>, but not both.</p>
         <p>We recommend you create the secret string as JSON key/value pairs, as shown in the example.</p>
         <p>Sensitive: This field contains sensitive information, so the service does not include it in CloudTrail log entries. If you create your own log entries, you must also avoid logging the information in this field.</p> |
| `rotation_token` | String |  | <p>A unique identifier that indicates the source of the request. For cross-account rotation (when you rotate a secret in one account by using a Lambda rotation function in another account) and the Lambda rotation function assumes an IAM role to call Secrets Manager, Secrets Manager validates the identity with the rotation token. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotating-secrets.html">How rotation works</a>.</p>
         <p>Sensitive: This field contains sensitive information, so the service does not include it in CloudTrail log entries. If you create your own log entries, you must also avoid logging the information in this field.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The friendly name of the secret.</p> |
| `created_date` | String | <p>The date and time that this version of the secret was created. If you don't specify 
      which version in <code>VersionId</code> or <code>VersionStage</code>, then Secrets Manager uses the 
      <code>AWSCURRENT</code> version.</p> |
| `version_stages` | Vec<String> | <p>A list of all of the staging labels currently attached to this version of the
      secret.</p> |
| `secret_binary` | String | <p>The decrypted secret value, if the secret value was originally provided as
      binary data in the form of a byte array. When you retrieve a <code>SecretBinary</code> using the HTTP API, the Python SDK, or the Amazon Web Services CLI, the value is Base64-encoded. Otherwise, it is not encoded.</p>
         <p>If the secret was created by using the Secrets Manager console, or if the secret value was 
      originally provided as a string, then this field is omitted. The secret value appears in 
      <code>SecretString</code> instead.</p>
         <p>Sensitive: This field contains sensitive information, so the service does not include it in CloudTrail log entries. If you create your own log entries, you must also avoid logging the information in this field.</p> |
| `secret_string` | String | <p>The decrypted secret value, if the secret value was originally provided as a string or 
      through the Secrets Manager console.</p>
         <p>If this secret was created by using the console, then Secrets Manager stores the information as a
      JSON structure of key/value pairs. </p>
         <p>Sensitive: This field contains sensitive information, so the service does not include it in CloudTrail log entries. If you create your own log entries, you must also avoid logging the information in this field.</p> |
| `version_id` | String | <p>The unique identifier of this version of the secret.</p> |
| `arn` | String | <p>The ARN of the secret.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create secret_value
secret_value = provider.secrets_manager.Secret_value {
    secret_id = "value"  # <p>The ARN or name of the secret to add a new version to.</p>
         <p>For an ARN, we recommend that you specify a complete ARN rather 
      than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
         <p>If the secret doesn't already exist, use <code>CreateSecret</code> instead.</p>
}

# Access secret_value outputs
secret_value_id = secret_value.id
secret_value_name = secret_value.name
secret_value_created_date = secret_value.created_date
secret_value_version_stages = secret_value.version_stages
secret_value_secret_binary = secret_value.secret_binary
secret_value_secret_string = secret_value.secret_string
secret_value_version_id = secret_value.version_id
secret_value_arn = secret_value.arn
```

---


### Secret

Secret resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `add_replica_regions` | Vec<String> |  | <p>A list of Regions and KMS keys to replicate secrets.</p> |
| `force_overwrite_replica_secret` | bool |  | <p>Specifies whether to overwrite a secret with the same name in the
      destination Region. By default, secrets aren't overwritten.</p> |
| `name` | String | ✅ | <p>The name of the new secret.</p>
         <p>The secret name can contain ASCII letters, numbers, and the following characters:
      /_+=.@-</p>
         <p>Do not end your secret name with a hyphen followed by six characters. If you do so, you
        risk confusion and unexpected results when searching for a secret by partial ARN. Secrets Manager
        automatically adds a hyphen and six random characters after the secret name at the end of the ARN.</p> |
| `client_request_token` | String |  | <p>If you include <code>SecretString</code> or <code>SecretBinary</code>, then 
      Secrets Manager creates an initial version for the secret, and this parameter specifies the unique
      identifier for the new version. </p>
         <note>
            <p>If you use the Amazon Web Services CLI or one of the Amazon Web Services SDKs to call this operation, then you can leave this parameter empty. The CLI or SDK generates a random UUID for you and includes it as the value for this parameter in the request. </p>
         </note>
         <p>If you generate a raw HTTP request to the Secrets Manager service endpoint, then you must generate a <code>ClientRequestToken</code> and include it in the request.</p>
         <p>This value helps ensure idempotency. Secrets Manager uses this value to prevent the accidental creation of duplicate versions if there are failures and retries during a rotation. We recommend that you generate a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID-type</a> value to ensure uniqueness of your versions within the specified secret. </p>
         <ul>
            <li>
               <p>If the <code>ClientRequestToken</code> value isn't already associated with a version
          of the secret then a new version of the secret is created. </p>
            </li>
            <li>
               <p>If a version with this value already exists and the version <code>SecretString</code>
          and <code>SecretBinary</code> values are the same as those in the request, then the
          request is ignored.</p>
            </li>
            <li>
               <p>If a version with this value already exists and that version's
          <code>SecretString</code> and <code>SecretBinary</code> values are different from those
          in the request, then the request fails because you cannot modify an existing version.
          Instead, use <a>PutSecretValue</a> to create a new version.</p>
            </li>
         </ul>
         <p>This value becomes the <code>VersionId</code> of the new version.</p> |
| `kms_key_id` | String |  | <p>The ARN, key ID, or alias of the KMS key that Secrets Manager uses to
      encrypt the secret value in the secret. An alias is always prefixed by <code>alias/</code>, 
      for example <code>alias/aws/secretsmanager</code>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/alias-about.html">About aliases</a>.</p>
         <p>To use a KMS key in a different account, use the key ARN or the alias ARN.</p>
         <p>If you don't specify this value, then Secrets Manager uses the key <code>aws/secretsmanager</code>. 
      If that key doesn't yet exist, then Secrets Manager creates it for you automatically the first time it 
      encrypts the secret value.</p>
         <p>If the secret is in a different Amazon Web Services account from the credentials calling the API, then 
      you can't use <code>aws/secretsmanager</code> to encrypt the secret, and you must create 
      and use a customer managed KMS key. </p> |
| `secret_binary` | String |  | <p>The binary data to encrypt and store in the new version of
      the secret. We recommend that you store your binary data in a file and then pass the
      contents of the file as a parameter.</p>
         <p>Either <code>SecretString</code> or <code>SecretBinary</code> must have a value, but not
      both.</p>
         <p>This parameter is not available in the Secrets Manager console.</p>
         <p>Sensitive: This field contains sensitive information, so the service does not include it in CloudTrail log entries. If you create your own log entries, you must also avoid logging the information in this field.</p> |
| `secret_string` | String |  | <p>The text data to encrypt and store in this new version of
      the secret. We recommend you use a JSON structure of key/value pairs for your secret value.</p>
         <p>Either <code>SecretString</code> or <code>SecretBinary</code> must have a value, but not
      both.</p>
         <p>If you create a secret by using the Secrets Manager console then Secrets Manager puts the protected
      secret text in only the <code>SecretString</code> parameter. The Secrets Manager console stores the
      information as a JSON structure of key/value pairs that a Lambda rotation function can parse.</p>
         <p>Sensitive: This field contains sensitive information, so the service does not include it in CloudTrail log entries. If you create your own log entries, you must also avoid logging the information in this field.</p> |
| `description` | String |  | <p>The description of the secret.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to attach to the secret. Each tag
      is a key and value pair of strings in a JSON text string, for example:</p>
         <p>
            <code>[{"Key":"CostCenter","Value":"12345"},{"Key":"environment","Value":"production"}]</code>
         </p>
         <p>Secrets Manager tag key names are case sensitive. A tag with the key "ABC" is a different tag
      from one with key "abc".</p>
         <p>If you check tags in permissions policies as part of your
      security strategy, then adding or removing a tag can change permissions. If the
      completion of this operation would result in you losing your permissions for
      this secret, then Secrets Manager blocks the operation and returns an <code>Access Denied</code>
      error. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_examples.html#tag-secrets-abac">Control 
        access to secrets using tags</a> and <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_examples.html#auth-and-access_tags2">Limit access to identities with tags that match secrets' tags</a>.</p>
         <p>For information about how to format a
      JSON parameter for the various command line tool environments, see <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#cli-using-param-json">Using JSON for
        Parameters</a>. If your command-line tool or SDK requires quotation marks around the parameter, you should
      use single quotes to avoid confusion with the double quotes required in the JSON text.</p>
         <p>For tag quotas and naming restrictions, see <a href="https://docs.aws.amazon.com/general/latest/gr/arg.html#taged-reference-quotas">Service quotas for Tagging</a> in the <i>Amazon Web Services General 
      Reference guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | <p>The ARN of the secret.</p> |
| `rotation_enabled` | bool | <p>Specifies whether automatic rotation is turned on for this secret.  If the secret has never been configured for rotation, Secrets Manager returns null.</p>
         <p>To turn on rotation, use <a>RotateSecret</a>. To turn off
      rotation, use <a>CancelRotateSecret</a>.</p> |
| `last_accessed_date` | String | <p>The date that the secret was last accessed in the Region. This field is omitted if the secret has never been retrieved in the Region.</p> |
| `created_date` | String | <p>The date the secret was created.</p> |
| `deleted_date` | String | <p>The date the secret is scheduled for deletion. If it is not scheduled for deletion, this 
      field is omitted. When you delete a secret, Secrets Manager requires a 
      recovery window of at least 7 days before deleting the secret. Some time after the deleted date, 
      Secrets Manager deletes the secret, including all of its versions.</p>
         <p>If a secret is scheduled for deletion, then its details, including the encrypted secret
      value, is not accessible. To cancel a scheduled deletion and restore access to the secret, use <a>RestoreSecret</a>.</p> |
| `replication_status` | Vec<String> | <p>A list of the replicas of this secret and their status: </p>
         <ul>
            <li>
               <p>
                  <code>Failed</code>, which indicates that the replica was not created.</p>
            </li>
            <li>
               <p>
                  <code>InProgress</code>, which indicates that Secrets Manager is in the process of creating the replica.</p>
            </li>
            <li>
               <p>
                  <code>InSync</code>, which indicates that the replica was created.</p>
            </li>
         </ul> |
| `owning_service` | String | <p>The ID of the service that created this secret. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/service-linked-secrets.html">Secrets managed by other Amazon Web Services services</a>.</p> |
| `primary_region` | String | <p>The Region the secret is in. If a secret is replicated to other Regions, the replicas are listed in <code>ReplicationStatus</code>. </p> |
| `kms_key_id` | String | <p>The key ID or alias ARN of the KMS key that Secrets Manager uses to encrypt the secret value. 
      If the secret is encrypted with the Amazon Web Services managed key <code>aws/secretsmanager</code>, 
      this field is omitted. Secrets created using the console use an KMS key ID.</p> |
| `rotation_rules` | String | <p>The rotation schedule and Lambda function for this secret. If the secret previously had rotation turned on, but 
      it is now turned off, this field shows the previous rotation schedule and rotation function. If the secret never had 
    rotation turned on, this field is omitted.</p> |
| `last_changed_date` | String | <p>The last date and time that this secret was modified in any way.</p> |
| `next_rotation_date` | String | <p>The next rotation is scheduled to occur on or before this date. If the secret isn't configured for rotation or rotation has been disabled, Secrets Manager returns null. If rotation fails, Secrets Manager retries the entire rotation process multiple times. If rotation is unsuccessful, this date may be in the past.</p>
         <p>This date represents the latest date that rotation will occur, but it is not an approximate rotation date. In some cases, for example if you turn off automatic rotation and then turn it back on, the next rotation may occur much sooner than this date.</p> |
| `last_rotated_date` | String | <p>The last date and time that Secrets Manager rotated the secret. 
      If the secret isn't configured for rotation or rotation has been disabled, Secrets Manager returns null.</p> |
| `tags` | Vec<String> | <p>The list of tags attached to the secret. To add tags to a
      secret, use <a>TagResource</a>. To remove tags, use <a>UntagResource</a>.</p> |
| `description` | String | <p>The description of the secret.</p> |
| `version_ids_to_stages` | HashMap<String, Vec<String>> | <p>A list of the versions of the secret that have staging labels attached.
      Versions that don't have staging labels are considered deprecated and Secrets Manager 
      can delete them.</p>
         <p>Secrets Manager uses staging labels to indicate the status of a secret version during rotation. The three 
    staging labels for rotation are: </p>
         <ul>
            <li>
               <p>
                  <code>AWSCURRENT</code>, which indicates the current version of the secret.</p>
            </li>
            <li>
               <p>
                  <code>AWSPENDING</code>, which indicates the version of the secret that contains new 
        secret information that will become the next current version when rotation finishes.</p>
               <p>During  
          rotation, Secrets Manager creates an <code>AWSPENDING</code> version ID before creating the new secret version. 
        To check if a secret version exists, call <a>GetSecretValue</a>.</p>
            </li>
            <li>
               <p>
                  <code>AWSPREVIOUS</code>, which indicates the previous current version of the secret. 
      You can use this as the <i>last known good</i> version.</p>
            </li>
         </ul>
         <p>For more information about rotation and staging labels, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_how.html">How rotation works</a>.</p> |
| `rotation_lambda_arn` | String | <p>The ARN of the Lambda function that Secrets Manager invokes to rotate the
      secret. </p> |
| `name` | String | <p>The name of the secret.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create secret
secret = provider.secrets_manager.Secret {
    name = "value"  # <p>The name of the new secret.</p>
         <p>The secret name can contain ASCII letters, numbers, and the following characters:
      /_+=.@-</p>
         <p>Do not end your secret name with a hyphen followed by six characters. If you do so, you
        risk confusion and unexpected results when searching for a secret by partial ARN. Secrets Manager
        automatically adds a hyphen and six random characters after the secret name at the end of the ARN.</p>
}

# Access secret outputs
secret_id = secret.id
secret_arn = secret.arn
secret_rotation_enabled = secret.rotation_enabled
secret_last_accessed_date = secret.last_accessed_date
secret_created_date = secret.created_date
secret_deleted_date = secret.deleted_date
secret_replication_status = secret.replication_status
secret_owning_service = secret.owning_service
secret_primary_region = secret.primary_region
secret_kms_key_id = secret.kms_key_id
secret_rotation_rules = secret.rotation_rules
secret_last_changed_date = secret.last_changed_date
secret_next_rotation_date = secret.next_rotation_date
secret_last_rotated_date = secret.last_rotated_date
secret_tags = secret.tags
secret_description = secret.description
secret_version_ids_to_stages = secret.version_ids_to_stages
secret_rotation_lambda_arn = secret.rotation_lambda_arn
secret_name = secret.name
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `secret_id` | String | ✅ | <p>The ARN or name of the secret to attach the resource-based policy.</p>
         <p>For an ARN, we recommend that you specify a complete ARN rather 
      than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p> |
| `resource_policy` | String | ✅ | <p>A JSON-formatted string for an Amazon Web Services
      resource-based policy. For example policies, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_examples.html">Permissions 
        policy examples</a>.</p> |
| `block_public_policy` | bool |  | <p>Specifies whether to block resource-based policies that allow broad access to the secret, for example those that use a wildcard for the principal. By default, public policies aren't blocked.</p>
         <important>
            <p>Resource policy validation and the BlockPublicPolicy parameter help protect your resources by preventing public access from being granted through the resource policies that are directly attached to your secrets. In addition to using these features, carefully inspect the following policies to confirm that they do not grant public access:</p>
            <ul>
               <li>
                  <p>Identity-based policies attached to associated Amazon Web Services principals (for example, IAM roles)</p>
               </li>
               <li>
                  <p>Resource-based policies attached to associated Amazon Web Services resources (for example, Key Management Service (KMS) keys)</p>
               </li>
            </ul>
            <p>To review permissions to your secrets, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/determine-acccess_examine-iam-policies.html">Determine who has permissions to your secrets</a>.</p>
         </important> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the secret that the resource-based policy was retrieved for.</p> |
| `resource_policy` | String | <p>A JSON-formatted string that contains the permissions policy 
      attached to the secret. For more information about permissions policies, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access.html">Authentication and access control for
        Secrets Manager</a>.</p> |
| `arn` | String | <p>The ARN of the secret that the resource-based policy was retrieved for.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.secrets_manager.Resource_policy {
    secret_id = "value"  # <p>The ARN or name of the secret to attach the resource-based policy.</p>
         <p>For an ARN, we recommend that you specify a complete ARN rather 
      than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
    resource_policy = "value"  # <p>A JSON-formatted string for an Amazon Web Services
      resource-based policy. For example policies, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_examples.html">Permissions 
        policy examples</a>.</p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_name = resource_policy.name
resource_policy_resource_policy = resource_policy.resource_policy
resource_policy_arn = resource_policy.arn
```

---


### Secret_version_stage

SecretVersionStage resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version_stage` | String | ✅ | <p>The staging label to add to this version.</p> |
| `move_to_version_id` | String |  | <p>The ID of the version to add the staging label to. To
      remove a label from a version, then do not specify this parameter.</p>
         <p>If the staging label is already attached to a different version of the secret, then you
      must also specify the <code>RemoveFromVersionId</code> parameter. </p> |
| `secret_id` | String | ✅ | <p>The ARN or the name of the secret with the version and staging labelsto modify.</p>
         <p>For an ARN, we recommend that you specify a complete ARN rather 
      than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p> |
| `remove_from_version_id` | String |  | <p>The ID of the version that the staging label is to be removed
      from. If the staging label you are trying to attach to one version is already attached to a
      different version, then you must include this parameter and specify the version that the label
      is to be removed from. If the label is attached and you either do not specify this parameter,
      or the version ID does not match, then the operation fails.</p> |



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

# Create multiple random_password resources
random_password_0 = provider.secrets_manager.Random_password {
}
random_password_1 = provider.secrets_manager.Random_password {
}
random_password_2 = provider.secrets_manager.Random_password {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    random_password = provider.secrets_manager.Random_password {
    }
```

---

## Related Documentation

- [AWS Secrets_manager Documentation](https://docs.aws.amazon.com/secrets_manager/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
