# Ecr Service



**Resources**: 22

---

## Overview

The ecr service provides access to 22 resource types:

- [Pull_through_cache_rule](#pull_through_cache_rule) [CUD]
- [Lifecycle_policy](#lifecycle_policy) [CRD]
- [Repository_creation_template](#repository_creation_template) [CUD]
- [Images](#images) [R]
- [Download_url_for_layer](#download_url_for_layer) [R]
- [Lifecycle_policy_preview](#lifecycle_policy_preview) [R]
- [Registry_scanning_configuration](#registry_scanning_configuration) [CR]
- [Replication_configuration](#replication_configuration) [C]
- [Account_setting](#account_setting) [CR]
- [Registry](#registry) [R]
- [Image](#image) [C]
- [Authorization_token](#authorization_token) [R]
- [Image_scanning_configuration](#image_scanning_configuration) [C]
- [Image_tag_mutability](#image_tag_mutability) [C]
- [Pull_through_cache_rules](#pull_through_cache_rules) [R]
- [Repositories](#repositories) [R]
- [Image_replication_status](#image_replication_status) [R]
- [Registry_policy](#registry_policy) [CRD]
- [Repository_policy](#repository_policy) [RD]
- [Repository](#repository) [CD]
- [Image_scan_findings](#image_scan_findings) [R]
- [Repository_creation_templates](#repository_creation_templates) [R]

---

## Resources


### Pull_through_cache_rule

PullThroughCacheRule resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `credential_arn` | String |  | <p>The Amazon Resource Name (ARN) of the Amazon Web Services Secrets Manager secret that identifies the credentials to authenticate
            to the upstream registry.</p> |
| `ecr_repository_prefix` | String | ✅ | <p>The repository name prefix to use when caching images from the source registry.</p>
         <important>
            <p>There is always an assumed <code>/</code> applied to the end of the prefix. If you
                specify <code>ecr-public</code> as the prefix, Amazon ECR treats that as
                    <code>ecr-public/</code>.</p>
         </important> |
| `upstream_registry_url` | String | ✅ | <p>The registry URL of the upstream public registry to use as the source for the pull
            through cache rule. The following is the syntax to use for each supported upstream
            registry.</p>
         <ul>
            <li>
               <p>Amazon ECR (<code>ecr</code>) –
                    <code><accountId>.dkr.ecr.<region>.amazonaws.com</code>
               </p>
            </li>
            <li>
               <p>Amazon ECR Public (<code>ecr-public</code>) – <code>public.ecr.aws</code>
               </p>
            </li>
            <li>
               <p>Docker Hub (<code>docker-hub</code>) –
                    <code>registry-1.docker.io</code>
               </p>
            </li>
            <li>
               <p>GitHub Container Registry (<code>github-container-registry</code>) –
                        <code>ghcr.io</code>
               </p>
            </li>
            <li>
               <p>GitLab Container Registry (<code>gitlab-container-registry</code>) –
                        <code>registry.gitlab.com</code>
               </p>
            </li>
            <li>
               <p>Kubernetes (<code>k8s</code>) – <code>registry.k8s.io</code>
               </p>
            </li>
            <li>
               <p>Microsoft Azure Container Registry (<code>azure-container-registry</code>) –
                        <code><custom>.azurecr.io</code>
               </p>
            </li>
            <li>
               <p>Quay (<code>quay</code>) – <code>quay.io</code>
               </p>
            </li>
         </ul> |
| `custom_role_arn` | String |  | <p>Amazon Resource Name (ARN) of the IAM role to be assumed by Amazon ECR to authenticate to
            the ECR upstream registry. This role must be in the same account as the registry that
            you are configuring.</p> |
| `registry_id` | String |  | <p>The Amazon Web Services account ID associated with the registry to create the pull through cache
            rule for. If you do not specify a registry, the default registry is assumed.</p> |
| `upstream_repository_prefix` | String |  | <p>The repository name prefix of the upstream registry to match with the upstream
            repository name. When this field isn't specified, Amazon ECR will use the
            <code>ROOT</code>.</p> |
| `upstream_registry` | String |  | <p>The name of the upstream registry.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create pull_through_cache_rule
pull_through_cache_rule = provider.ecr.Pull_through_cache_rule {
    ecr_repository_prefix = "value"  # <p>The repository name prefix to use when caching images from the source registry.</p>
         <important>
            <p>There is always an assumed <code>/</code> applied to the end of the prefix. If you
                specify <code>ecr-public</code> as the prefix, Amazon ECR treats that as
                    <code>ecr-public/</code>.</p>
         </important>
    upstream_registry_url = "value"  # <p>The registry URL of the upstream public registry to use as the source for the pull
            through cache rule. The following is the syntax to use for each supported upstream
            registry.</p>
         <ul>
            <li>
               <p>Amazon ECR (<code>ecr</code>) –
                    <code><accountId>.dkr.ecr.<region>.amazonaws.com</code>
               </p>
            </li>
            <li>
               <p>Amazon ECR Public (<code>ecr-public</code>) – <code>public.ecr.aws</code>
               </p>
            </li>
            <li>
               <p>Docker Hub (<code>docker-hub</code>) –
                    <code>registry-1.docker.io</code>
               </p>
            </li>
            <li>
               <p>GitHub Container Registry (<code>github-container-registry</code>) –
                        <code>ghcr.io</code>
               </p>
            </li>
            <li>
               <p>GitLab Container Registry (<code>gitlab-container-registry</code>) –
                        <code>registry.gitlab.com</code>
               </p>
            </li>
            <li>
               <p>Kubernetes (<code>k8s</code>) – <code>registry.k8s.io</code>
               </p>
            </li>
            <li>
               <p>Microsoft Azure Container Registry (<code>azure-container-registry</code>) –
                        <code><custom>.azurecr.io</code>
               </p>
            </li>
            <li>
               <p>Quay (<code>quay</code>) – <code>quay.io</code>
               </p>
            </li>
         </ul>
}

```

---


### Lifecycle_policy

LifecyclePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `registry_id` | String |  | <p>The Amazon Web Services account ID associated with the registry that contains the repository. If you
            do  not specify a registry, the default registry is assumed.</p> |
| `repository_name` | String | ✅ | <p>The name of the repository to receive the policy.</p> |
| `lifecycle_policy_text` | String | ✅ | <p>The JSON repository policy text to apply to the repository.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `registry_id` | String | <p>The registry ID associated with the request.</p> |
| `repository_name` | String | <p>The repository name associated with the request.</p> |
| `last_evaluated_at` | String | <p>The time stamp of the last time that the lifecycle policy was run.</p> |
| `lifecycle_policy_text` | String | <p>The JSON lifecycle policy text.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create lifecycle_policy
lifecycle_policy = provider.ecr.Lifecycle_policy {
    repository_name = "value"  # <p>The name of the repository to receive the policy.</p>
    lifecycle_policy_text = "value"  # <p>The JSON repository policy text to apply to the repository.</p>
}

# Access lifecycle_policy outputs
lifecycle_policy_id = lifecycle_policy.id
lifecycle_policy_registry_id = lifecycle_policy.registry_id
lifecycle_policy_repository_name = lifecycle_policy.repository_name
lifecycle_policy_last_evaluated_at = lifecycle_policy.last_evaluated_at
lifecycle_policy_lifecycle_policy_text = lifecycle_policy.lifecycle_policy_text
```

---


### Repository_creation_template

RepositoryCreationTemplate resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `prefix` | String | ✅ | <p>The repository namespace prefix to associate with the template. All repositories
            created using this namespace prefix will have the settings defined in this template
            applied. For example, a prefix of <code>prod</code> would apply to all repositories
            beginning with <code>prod/</code>. Similarly, a prefix of <code>prod/team</code> would
            apply to all repositories beginning with <code>prod/team/</code>.</p>
         <p>To apply a template to all repositories in your registry that don't have an associated
            creation template, you can use <code>ROOT</code> as the prefix.</p>
         <important>
            <p>There is always an assumed <code>/</code> applied to the end of the prefix. If you
                specify <code>ecr-public</code> as the prefix, Amazon ECR treats that as
                    <code>ecr-public/</code>. When using a pull through cache rule, the repository
                prefix you specify during rule creation is what you should specify as your
                repository creation template prefix as well.</p>
         </important> |
| `image_tag_mutability_exclusion_filters` | Vec<String> |  | <p>Creates a repository creation template with a list of filters that define which image tags can override the default image tag mutability setting.</p> |
| `repository_policy` | String |  | <p>The repository policy to apply to repositories created using the template. A
            repository policy is a permissions policy associated with a repository to control access
            permissions. </p> |
| `custom_role_arn` | String |  | <p>The ARN of the role to be assumed by Amazon ECR. This role must be in the same account as
            the registry that you are configuring. Amazon ECR will assume your supplied role when the
            customRoleArn is specified. When this field isn't specified, Amazon ECR will use the
            service-linked role for the repository creation template.</p> |
| `image_tag_mutability` | String |  | <p>The tag mutability setting for the repository. If this parameter is omitted, the
            default setting of <code>MUTABLE</code> will be used which will allow image tags to be
            overwritten. If <code>IMMUTABLE</code> is specified, all image tags within the
            repository will be immutable which will prevent them from being overwritten.</p> |
| `lifecycle_policy` | String |  | <p>The lifecycle policy to use for repositories created using the template.</p> |
| `description` | String |  | <p>A description for the repository creation template.</p> |
| `encryption_configuration` | String |  | <p>The encryption configuration to use for repositories created using the
            template.</p> |
| `applied_for` | Vec<String> | ✅ | <p>A list of enumerable strings representing the Amazon ECR repository creation scenarios that
            this template will apply towards. The two supported scenarios are
                <code>PULL_THROUGH_CACHE</code> and <code>REPLICATION</code>
         </p> |
| `resource_tags` | Vec<String> |  | <p>The metadata to apply to the repository to help you categorize and organize. Each tag
            consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have
            a maximum length of 256 characters.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create repository_creation_template
repository_creation_template = provider.ecr.Repository_creation_template {
    prefix = "value"  # <p>The repository namespace prefix to associate with the template. All repositories
            created using this namespace prefix will have the settings defined in this template
            applied. For example, a prefix of <code>prod</code> would apply to all repositories
            beginning with <code>prod/</code>. Similarly, a prefix of <code>prod/team</code> would
            apply to all repositories beginning with <code>prod/team/</code>.</p>
         <p>To apply a template to all repositories in your registry that don't have an associated
            creation template, you can use <code>ROOT</code> as the prefix.</p>
         <important>
            <p>There is always an assumed <code>/</code> applied to the end of the prefix. If you
                specify <code>ecr-public</code> as the prefix, Amazon ECR treats that as
                    <code>ecr-public/</code>. When using a pull through cache rule, the repository
                prefix you specify during rule creation is what you should specify as your
                repository creation template prefix as well.</p>
         </important>
    applied_for = "value"  # <p>A list of enumerable strings representing the Amazon ECR repository creation scenarios that
            this template will apply towards. The two supported scenarios are
                <code>PULL_THROUGH_CACHE</code> and <code>REPLICATION</code>
         </p>
}

```

---


### Images

Images resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `image_details` | Vec<String> | <p>A list of <a>ImageDetail</a> objects that contain data about the
            image.</p> |
| `next_token` | String | <p>The <code>nextToken</code> value to include in a future <code>DescribeImages</code>
            request. When the results of a <code>DescribeImages</code> request exceed
                <code>maxResults</code>, this value can be used to retrieve the next page of
            results. This value is <code>null</code> when there are no more results to
            return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access images outputs
images_id = images.id
images_image_details = images.image_details
images_next_token = images.next_token
```

---


### Download_url_for_layer

DownloadUrlForLayer resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `download_url` | String | <p>The pre-signed Amazon S3 download URL for the requested layer.</p> |
| `layer_digest` | String | <p>The digest of the image layer to download.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access download_url_for_layer outputs
download_url_for_layer_id = download_url_for_layer.id
download_url_for_layer_download_url = download_url_for_layer.download_url
download_url_for_layer_layer_digest = download_url_for_layer.layer_digest
```

---


### Lifecycle_policy_preview

LifecyclePolicyPreview resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `registry_id` | String | <p>The registry ID associated with the request.</p> |
| `summary` | String | <p>The list of images that is returned as a result of the action.</p> |
| `lifecycle_policy_text` | String | <p>The JSON lifecycle policy text.</p> |
| `next_token` | String | <p>The <code>nextToken</code> value to include in a future
                <code>GetLifecyclePolicyPreview</code> request. When the results of a
                <code>GetLifecyclePolicyPreview</code> request exceed <code>maxResults</code>, this
            value can be used to retrieve the next page of results. This value is <code>null</code>
            when there are no more results to return.</p> |
| `repository_name` | String | <p>The repository name associated with the request.</p> |
| `status` | String | <p>The status of the lifecycle policy preview request.</p> |
| `preview_results` | Vec<String> | <p>The results of the lifecycle policy preview request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access lifecycle_policy_preview outputs
lifecycle_policy_preview_id = lifecycle_policy_preview.id
lifecycle_policy_preview_registry_id = lifecycle_policy_preview.registry_id
lifecycle_policy_preview_summary = lifecycle_policy_preview.summary
lifecycle_policy_preview_lifecycle_policy_text = lifecycle_policy_preview.lifecycle_policy_text
lifecycle_policy_preview_next_token = lifecycle_policy_preview.next_token
lifecycle_policy_preview_repository_name = lifecycle_policy_preview.repository_name
lifecycle_policy_preview_status = lifecycle_policy_preview.status
lifecycle_policy_preview_preview_results = lifecycle_policy_preview.preview_results
```

---


### Registry_scanning_configuration

RegistryScanningConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rules` | Vec<String> |  | <p>The scanning rules to use for the registry. A scanning rule is used to determine which
            repository filters are used and at what frequency scanning will occur.</p> |
| `scan_type` | String |  | <p>The scanning type to set for the registry.</p>
         <p>When a registry scanning configuration is not defined, by default the
                <code>BASIC</code> scan type is used. When basic scanning is used, you may specify
            filters to determine which individual repositories, or all repositories, are scanned
            when new images are pushed to those repositories. Alternatively, you can do manual scans
            of images with basic scanning.</p>
         <p>When the <code>ENHANCED</code> scan type is set, Amazon Inspector provides automated
            vulnerability scanning. You may choose between continuous scanning or scan on push and
            you may specify filters to determine which individual repositories, or all repositories,
            are scanned.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scanning_configuration` | String | <p>The scanning configuration for the registry.</p> |
| `registry_id` | String | <p>The registry ID associated with the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create registry_scanning_configuration
registry_scanning_configuration = provider.ecr.Registry_scanning_configuration {
}

# Access registry_scanning_configuration outputs
registry_scanning_configuration_id = registry_scanning_configuration.id
registry_scanning_configuration_scanning_configuration = registry_scanning_configuration.scanning_configuration
registry_scanning_configuration_registry_id = registry_scanning_configuration.registry_id
```

---


### Replication_configuration

ReplicationConfiguration resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `replication_configuration` | String | ✅ | <p>An object representing the replication configuration for a registry.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create replication_configuration
replication_configuration = provider.ecr.Replication_configuration {
    replication_configuration = "value"  # <p>An object representing the replication configuration for a registry.</p>
}

```

---


### Account_setting

AccountSetting resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `value` | String | ✅ | <p>Setting value that is specified. The following are valid values for the basic scan
            type being used: <code>AWS_NATIVE</code> or <code>CLAIR</code>. The following are valid
            values for the registry policy scope being used: <code>V1</code> or
            <code>V2</code>.</p> |
| `name` | String | ✅ | <p>The name of the account setting, such as <code>BASIC_SCAN_TYPE_VERSION</code> or
                <code>REGISTRY_POLICY_SCOPE</code>. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `value` | String | <p>The setting value for the setting name. The following are valid values for the basic
            scan type being used: <code>AWS_NATIVE</code> or <code>CLAIR</code>. The following are
            valid values for the registry policy scope being used: <code>V1</code> or
                <code>V2</code>.</p> |
| `name` | String | <p>Retrieves the name of the account setting.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account_setting
account_setting = provider.ecr.Account_setting {
    value = "value"  # <p>Setting value that is specified. The following are valid values for the basic scan
            type being used: <code>AWS_NATIVE</code> or <code>CLAIR</code>. The following are valid
            values for the registry policy scope being used: <code>V1</code> or
            <code>V2</code>.</p>
    name = "value"  # <p>The name of the account setting, such as <code>BASIC_SCAN_TYPE_VERSION</code> or
                <code>REGISTRY_POLICY_SCOPE</code>. </p>
}

# Access account_setting outputs
account_setting_id = account_setting.id
account_setting_value = account_setting.value
account_setting_name = account_setting.name
```

---


### Registry

Registry resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replication_configuration` | String | <p>The replication configuration for the registry.</p> |
| `registry_id` | String | <p>The registry ID associated with the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access registry outputs
registry_id = registry.id
registry_replication_configuration = registry.replication_configuration
registry_registry_id = registry.registry_id
```

---


### Image

Image resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `repository_name` | String | ✅ | <p>The name of the repository in which to put the image.</p> |
| `image_manifest` | String | ✅ | <p>The image manifest corresponding to the image to be uploaded.</p> |
| `registry_id` | String |  | <p>The Amazon Web Services account ID associated with the registry that contains the repository in
            which to put the image. If you do not specify a registry, the default registry is assumed.</p> |
| `image_manifest_media_type` | String |  | <p>The media type of the image manifest. If you push an image manifest that does not
            contain the <code>mediaType</code> field, you must specify the
                <code>imageManifestMediaType</code> in the request.</p> |
| `image_tag` | String |  | <p>The tag to associate with the image. This parameter is required for images that use
            the Docker Image Manifest V2 Schema 2 or Open Container Initiative (OCI) formats.</p> |
| `image_digest` | String |  | <p>The image digest of the image manifest corresponding to the image.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create image
image = provider.ecr.Image {
    repository_name = "value"  # <p>The name of the repository in which to put the image.</p>
    image_manifest = "value"  # <p>The image manifest corresponding to the image to be uploaded.</p>
}

```

---


### Authorization_token

AuthorizationToken resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `authorization_data` | Vec<String> | <p>A list of authorization token data objects that correspond to the
                <code>registryIds</code> values in the request.</p>
         <note>
            <p>The size of the authorization token returned by Amazon ECR is not fixed. We recommend
                that you don't make assumptions about the maximum size.</p>
         </note> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access authorization_token outputs
authorization_token_id = authorization_token.id
authorization_token_authorization_data = authorization_token.authorization_data
```

---


### Image_scanning_configuration

ImageScanningConfiguration resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `image_scanning_configuration` | String | ✅ | <p>The image scanning configuration for the repository. This setting determines whether
            images are scanned for known vulnerabilities after being pushed to the
            repository.</p> |
| `registry_id` | String |  | <p>The Amazon Web Services account ID associated with the registry that contains the repository in
            which to update the image scanning configuration setting.
            If you do not specify a registry, the default registry is assumed.</p> |
| `repository_name` | String | ✅ | <p>The name of the repository in which to update the image scanning configuration
            setting.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create image_scanning_configuration
image_scanning_configuration = provider.ecr.Image_scanning_configuration {
    image_scanning_configuration = "value"  # <p>The image scanning configuration for the repository. This setting determines whether
            images are scanned for known vulnerabilities after being pushed to the
            repository.</p>
    repository_name = "value"  # <p>The name of the repository in which to update the image scanning configuration
            setting.</p>
}

```

---


### Image_tag_mutability

ImageTagMutability resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `registry_id` | String |  | <p>The Amazon Web Services account ID associated with the registry that contains the repository in
            which to update the image tag mutability settings. If you do not specify a registry, the default registry is assumed.</p> |
| `image_tag_mutability_exclusion_filters` | Vec<String> |  | <p>Creates or updates a repository with filters that define which image tags can override the default image tag mutability setting.</p> |
| `image_tag_mutability` | String | ✅ | <p>The tag mutability setting for the repository. If <code>MUTABLE</code> is specified,
            image tags can be overwritten. If <code>IMMUTABLE</code> is specified, all image tags
            within the repository will be immutable which will prevent them from being
            overwritten.</p> |
| `repository_name` | String | ✅ | <p>The name of the repository in which to update the image tag mutability
            settings.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create image_tag_mutability
image_tag_mutability = provider.ecr.Image_tag_mutability {
    image_tag_mutability = "value"  # <p>The tag mutability setting for the repository. If <code>MUTABLE</code> is specified,
            image tags can be overwritten. If <code>IMMUTABLE</code> is specified, all image tags
            within the repository will be immutable which will prevent them from being
            overwritten.</p>
    repository_name = "value"  # <p>The name of the repository in which to update the image tag mutability
            settings.</p>
}

```

---


### Pull_through_cache_rules

PullThroughCacheRules resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The <code>nextToken</code> value to include in a future
                <code>DescribePullThroughCacheRulesRequest</code> request. When the results of a
                <code>DescribePullThroughCacheRulesRequest</code> request exceed
                <code>maxResults</code>, this value can be used to retrieve the next page of
            results. This value is null when there are no more results to return.</p> |
| `pull_through_cache_rules` | Vec<String> | <p>The details of the pull through cache rules.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access pull_through_cache_rules outputs
pull_through_cache_rules_id = pull_through_cache_rules.id
pull_through_cache_rules_next_token = pull_through_cache_rules.next_token
pull_through_cache_rules_pull_through_cache_rules = pull_through_cache_rules.pull_through_cache_rules
```

---


### Repositories

Repositories resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `repositories` | Vec<String> | <p>A list of repository objects corresponding to valid repositories.</p> |
| `next_token` | String | <p>The <code>nextToken</code> value to include in a future
                <code>DescribeRepositories</code> request. When the results of a
                <code>DescribeRepositories</code> request exceed <code>maxResults</code>, this value
            can be used to retrieve the next page of results. This value is <code>null</code> when
            there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access repositories outputs
repositories_id = repositories.id
repositories_repositories = repositories.repositories
repositories_next_token = repositories.next_token
```

---


### Image_replication_status

ImageReplicationStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `repository_name` | String | <p>The repository name associated with the request.</p> |
| `image_id` | String |  |
| `replication_statuses` | Vec<String> | <p>The replication status details for the images in the specified repository.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access image_replication_status outputs
image_replication_status_id = image_replication_status.id
image_replication_status_repository_name = image_replication_status.repository_name
image_replication_status_image_id = image_replication_status.image_id
image_replication_status_replication_statuses = image_replication_status.replication_statuses
```

---


### Registry_policy

RegistryPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_text` | String | ✅ | <p>The JSON policy text to apply to your registry. The policy text follows the same
            format as IAM policy text. For more information, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/registry-permissions.html">Registry
                permissions</a> in the <i>Amazon Elastic Container Registry User Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `registry_id` | String | <p>The registry ID associated with the request.</p> |
| `policy_text` | String | <p>The JSON text of the permissions policy for a registry.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create registry_policy
registry_policy = provider.ecr.Registry_policy {
    policy_text = "value"  # <p>The JSON policy text to apply to your registry. The policy text follows the same
            format as IAM policy text. For more information, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/registry-permissions.html">Registry
                permissions</a> in the <i>Amazon Elastic Container Registry User Guide</i>.</p>
}

# Access registry_policy outputs
registry_policy_id = registry_policy.id
registry_policy_registry_id = registry_policy.registry_id
registry_policy_policy_text = registry_policy.policy_text
```

---


### Repository_policy

RepositoryPolicy resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_text` | String | <p>The JSON repository policy text associated with the repository.</p> |
| `registry_id` | String | <p>The registry ID associated with the request.</p> |
| `repository_name` | String | <p>The repository name associated with the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access repository_policy outputs
repository_policy_id = repository_policy.id
repository_policy_policy_text = repository_policy.policy_text
repository_policy_registry_id = repository_policy.registry_id
repository_policy_repository_name = repository_policy.repository_name
```

---


### Repository

Repository resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `image_scanning_configuration` | String |  | <p>The image scanning configuration for the repository. This determines whether images
            are scanned for known vulnerabilities after being pushed to the repository.</p> |
| `repository_name` | String | ✅ | <p>The name to use for the repository. The repository name may be specified on its own
            (such as <code>nginx-web-app</code>) or it can be prepended with a namespace to group
            the repository into a category (such as <code>project-a/nginx-web-app</code>).</p>
         <p>The repository name must start with a letter and can only contain lowercase letters,
            numbers, hyphens, underscores, and forward slashes.</p> |
| `image_tag_mutability` | String |  | <p>The tag mutability setting for the repository. If this parameter is omitted, the
            default setting of <code>MUTABLE</code> will be used which will allow image tags to be
            overwritten. If <code>IMMUTABLE</code> is specified, all image tags within the
            repository will be immutable which will prevent them from being overwritten.</p> |
| `encryption_configuration` | String |  | <p>The encryption configuration for the repository. This determines how the contents of
            your repository are encrypted at rest.</p> |
| `registry_id` | String |  | <p>The Amazon Web Services account ID associated with the registry to create the repository.
            If you do not specify a registry, the default registry is assumed.</p> |
| `tags` | Vec<String> |  | <p>The metadata that you apply to the repository to help you categorize and organize
            them. Each tag consists of a key and an optional value, both of which you define.
            Tag keys can have a maximum character length of 128 characters, and tag values can have
            a maximum length of 256 characters.</p> |
| `image_tag_mutability_exclusion_filters` | Vec<String> |  | <p>Creates a repository with a list of filters that define which image tags can override the default image tag mutability setting.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create repository
repository = provider.ecr.Repository {
    repository_name = "value"  # <p>The name to use for the repository. The repository name may be specified on its own
            (such as <code>nginx-web-app</code>) or it can be prepended with a namespace to group
            the repository into a category (such as <code>project-a/nginx-web-app</code>).</p>
         <p>The repository name must start with a letter and can only contain lowercase letters,
            numbers, hyphens, underscores, and forward slashes.</p>
}

```

---


### Image_scan_findings

ImageScanFindings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `image_scan_status` | String | <p>The current state of the scan.</p> |
| `next_token` | String | <p>The <code>nextToken</code> value to include in a future
                <code>DescribeImageScanFindings</code> request. When the results of a
                <code>DescribeImageScanFindings</code> request exceed <code>maxResults</code>, this
            value can be used to retrieve the next page of results. This value is null when there
            are no more results to return.</p> |
| `image_id` | String |  |
| `image_scan_findings` | String | <p>The information contained in the image scan findings.</p> |
| `registry_id` | String | <p>The registry ID associated with the request.</p> |
| `repository_name` | String | <p>The repository name associated with the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access image_scan_findings outputs
image_scan_findings_id = image_scan_findings.id
image_scan_findings_image_scan_status = image_scan_findings.image_scan_status
image_scan_findings_next_token = image_scan_findings.next_token
image_scan_findings_image_id = image_scan_findings.image_id
image_scan_findings_image_scan_findings = image_scan_findings.image_scan_findings
image_scan_findings_registry_id = image_scan_findings.registry_id
image_scan_findings_repository_name = image_scan_findings.repository_name
```

---


### Repository_creation_templates

RepositoryCreationTemplates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `repository_creation_templates` | Vec<String> | <p>The details of the repository creation templates.</p> |
| `registry_id` | String | <p>The registry ID associated with the request.</p> |
| `next_token` | String | <p>The <code>nextToken</code> value to include in a future
                <code>DescribeRepositoryCreationTemplates</code> request. When the results of a
                <code>DescribeRepositoryCreationTemplates</code> request exceed
                <code>maxResults</code>, this value can be used to retrieve the next page of
            results. This value is <code>null</code> when there are no more results to
            return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access repository_creation_templates outputs
repository_creation_templates_id = repository_creation_templates.id
repository_creation_templates_repository_creation_templates = repository_creation_templates.repository_creation_templates
repository_creation_templates_registry_id = repository_creation_templates.registry_id
repository_creation_templates_next_token = repository_creation_templates.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple pull_through_cache_rule resources
pull_through_cache_rule_0 = provider.ecr.Pull_through_cache_rule {
    ecr_repository_prefix = "value-0"
    upstream_registry_url = "value-0"
}
pull_through_cache_rule_1 = provider.ecr.Pull_through_cache_rule {
    ecr_repository_prefix = "value-1"
    upstream_registry_url = "value-1"
}
pull_through_cache_rule_2 = provider.ecr.Pull_through_cache_rule {
    ecr_repository_prefix = "value-2"
    upstream_registry_url = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    pull_through_cache_rule = provider.ecr.Pull_through_cache_rule {
        ecr_repository_prefix = "production-value"
        upstream_registry_url = "production-value"
    }
```

---

## Related Documentation

- [AWS Ecr Documentation](https://docs.aws.amazon.com/ecr/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
