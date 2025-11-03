# Ecr_public Service



**Resources**: 10

---

## Overview

The ecr_public service provides access to 10 resource types:

- [Image](#image) [C]
- [Repository_catalog_data](#repository_catalog_data) [CR]
- [Images](#images) [R]
- [Repository](#repository) [CD]
- [Image_tags](#image_tags) [R]
- [Repository_policy](#repository_policy) [RD]
- [Registries](#registries) [R]
- [Registry_catalog_data](#registry_catalog_data) [CR]
- [Authorization_token](#authorization_token) [R]
- [Repositories](#repositories) [R]

---

## Resources


### Image

Image resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `registry_id` | String |  | <p>The Amazon Web Services account ID, or registry alias, that's associated with the public registry that
         contains the repository where the image is put. If you do not specify a registry, the default public registry is assumed.</p> |
| `image_digest` | String |  | <p>The image digest of the image manifest that corresponds to the image.</p> |
| `image_manifest_media_type` | String |  | <p>The media type of the image manifest. If you push an image manifest that doesn't contain
         the <code>mediaType</code> field, you must specify the <code>imageManifestMediaType</code>
         in the request.</p> |
| `image_tag` | String |  | <p>The tag to associate with the image. This parameter is required for images that use the
         Docker Image Manifest V2 Schema 2 or Open Container Initiative (OCI) formats.</p> |
| `repository_name` | String | ✅ | <p>The name of the repository where the image is put.</p> |
| `image_manifest` | String | ✅ | <p>The image manifest that corresponds to the image to be uploaded.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create image
image = provider.ecr_public.Image {
    repository_name = "value"  # <p>The name of the repository where the image is put.</p>
    image_manifest = "value"  # <p>The image manifest that corresponds to the image to be uploaded.</p>
}

```

---


### Repository_catalog_data

RepositoryCatalogData resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `registry_id` | String |  | <p>The Amazon Web Services account ID that's associated with the public registry the repository is in.
         If you do not specify a registry, the default public registry is assumed.</p> |
| `catalog_data` | String | ✅ | <p>An object containing the catalog data for a repository. This data is publicly visible in
         the Amazon ECR Public Gallery.</p> |
| `repository_name` | String | ✅ | <p>The name of the repository to create or update the catalog data for.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `catalog_data` | String | <p>The catalog metadata for the repository.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create repository_catalog_data
repository_catalog_data = provider.ecr_public.Repository_catalog_data {
    catalog_data = "value"  # <p>An object containing the catalog data for a repository. This data is publicly visible in
         the Amazon ECR Public Gallery.</p>
    repository_name = "value"  # <p>The name of the repository to create or update the catalog data for.</p>
}

# Access repository_catalog_data outputs
repository_catalog_data_id = repository_catalog_data.id
repository_catalog_data_catalog_data = repository_catalog_data.catalog_data
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
            <code>maxResults</code>, you can use this value to retrieve the next page of results. If
         there are no more results to return, this value is <code>null</code>.</p> |


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


### Repository

Repository resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `catalog_data` | String |  | <p>The details about the repository that are publicly visible in the
         Amazon ECR Public Gallery.</p> |
| `tags` | Vec<String> |  | <p>The metadata that you apply to each repository to help categorize and organize your
         repositories. Each tag consists of a key and an optional value. You define both of them.
         Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p> |
| `repository_name` | String | ✅ | <p>The name to use for the repository. This appears publicly in the Amazon ECR Public Gallery.
         The repository name can be specified on its own (for example <code>nginx-web-app</code>) or
         prepended with a namespace to group the repository into a category (for example
            <code>project-a/nginx-web-app</code>).</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create repository
repository = provider.ecr_public.Repository {
    repository_name = "value"  # <p>The name to use for the repository. This appears publicly in the Amazon ECR Public Gallery.
         The repository name can be specified on its own (for example <code>nginx-web-app</code>) or
         prepended with a namespace to group the repository into a category (for example
            <code>project-a/nginx-web-app</code>).</p>
}

```

---


### Image_tags

ImageTags resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `image_tag_details` | Vec<String> | <p>The image tag details for the images in the requested repository.</p> |
| `next_token` | String | <p>The <code>nextToken</code> value to include in a future <code>DescribeImageTags</code>
         request. When the results of a <code>DescribeImageTags</code> request exceed
            <code>maxResults</code>, you can use this value to retrieve the next page of results. If
         there are no more results to return, this value is <code>null</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access image_tags outputs
image_tags_id = image_tags.id
image_tags_image_tag_details = image_tags.image_tag_details
image_tags_next_token = image_tags.next_token
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
| `repository_name` | String | <p>The repository name that's associated with the request.</p> |
| `registry_id` | String | <p>The registry ID that's associated with the request.</p> |
| `policy_text` | String | <p>The repository policy text that's associated with the repository. The policy text will
         be in JSON format.</p> |


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
repository_policy_repository_name = repository_policy.repository_name
repository_policy_registry_id = repository_policy.registry_id
repository_policy_policy_text = repository_policy.policy_text
```

---


### Registries

Registries resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `registries` | Vec<String> | <p>An object that contains the details for a public registry.</p> |
| `next_token` | String | <p>The <code>nextToken</code> value to include in a future
            <code>DescribeRepositories</code> request. If the results of a
            <code>DescribeRepositories</code> request exceed <code>maxResults</code>, you can use
         this value to retrieve the next page of results. If there are no more results, this value
         is <code>null</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access registries outputs
registries_id = registries.id
registries_registries = registries.registries
registries_next_token = registries.next_token
```

---


### Registry_catalog_data

RegistryCatalogData resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | <p>The display name for a public registry. The display name is shown as the repository
         author in the Amazon ECR Public Gallery.</p>
         <note>
            <p>The registry display name is only publicly visible in the Amazon ECR Public Gallery for
            verified accounts.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `registry_catalog_data` | String | <p>The catalog metadata for the public registry.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create registry_catalog_data
registry_catalog_data = provider.ecr_public.Registry_catalog_data {
}

# Access registry_catalog_data outputs
registry_catalog_data_id = registry_catalog_data.id
registry_catalog_data_registry_catalog_data = registry_catalog_data.registry_catalog_data
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
| `authorization_data` | String | <p>An authorization token data object that corresponds to a public registry.</p> |


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


### Repositories

Repositories resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The <code>nextToken</code> value to include in a future
            <code>DescribeRepositories</code> request. When the results of a
            <code>DescribeRepositories</code> request exceed <code>maxResults</code>, this value can
         be used to retrieve the next page of results. If there are no more results to return, this
         value is <code>null</code>.</p> |
| `repositories` | Vec<String> | <p>A list of repository objects corresponding to valid repositories.</p> |


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
repositories_next_token = repositories.next_token
repositories_repositories = repositories.repositories
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple image resources
image_0 = provider.ecr_public.Image {
    repository_name = "value-0"
    image_manifest = "value-0"
}
image_1 = provider.ecr_public.Image {
    repository_name = "value-1"
    image_manifest = "value-1"
}
image_2 = provider.ecr_public.Image {
    repository_name = "value-2"
    image_manifest = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    image = provider.ecr_public.Image {
        repository_name = "production-value"
        image_manifest = "production-value"
    }
```

---

## Related Documentation

- [AWS Ecr_public Documentation](https://docs.aws.amazon.com/ecr_public/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
