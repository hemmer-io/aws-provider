# Codeartifact Service



**Resources**: 16

---

## Overview

The codeartifact service provides access to 16 resource types:

- [Repository_permissions_policy](#repository_permissions_policy) [CRD]
- [Package_version](#package_version) [R]
- [Domain](#domain) [CRD]
- [Repository](#repository) [CRUD]
- [Authorization_token](#authorization_token) [R]
- [Package_version_readme](#package_version_readme) [R]
- [Package_version_asset](#package_version_asset) [R]
- [Associated_package_group](#associated_package_group) [R]
- [Package_origin_configuration](#package_origin_configuration) [C]
- [Package_versions](#package_versions) [D]
- [Package_versions_status](#package_versions_status) [U]
- [Package_group](#package_group) [CRUD]
- [Domain_permissions_policy](#domain_permissions_policy) [CRD]
- [Package_group_origin_configuration](#package_group_origin_configuration) [U]
- [Package](#package) [RD]
- [Repository_endpoint](#repository_endpoint) [R]

---

## Resources


### Repository_permissions_policy

RepositoryPermissionsPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain` | String | ✅ | <p>
        The name of the domain containing the repository to set the resource policy on.
      </p> |
| `domain_owner` | String |  | <p>
        The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include 
        dashes or spaces.
      </p> |
| `repository` | String | ✅ | <p> The name of the repository to set the resource policy on. </p> |
| `policy_document` | String | ✅ | <p> A valid displayable JSON Aspen policy string to be set as the access control resource
      policy on the provided repository. </p> |
| `policy_revision` | String |  | <p>
        Sets the revision of the resource policy that specifies permissions to access the repository. 
        This revision is used for optimistic locking, which prevents others from overwriting your 
        changes to the repository's resource policy.
      </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>
        The returned resource policy.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create repository_permissions_policy
repository_permissions_policy = provider.codeartifact.Repository_permissions_policy {
    domain = "value"  # <p>
        The name of the domain containing the repository to set the resource policy on.
      </p>
    repository = "value"  # <p> The name of the repository to set the resource policy on. </p>
    policy_document = "value"  # <p> A valid displayable JSON Aspen policy string to be set as the access control resource
      policy on the provided repository. </p>
}

# Access repository_permissions_policy outputs
repository_permissions_policy_id = repository_permissions_policy.id
repository_permissions_policy_policy = repository_permissions_policy.policy
```

---


### Package_version

PackageVersion resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `package_version` | String | <p>
      A <a href="https://docs.aws.amazon.com/codeartifact/latest/APIReference/API_PackageVersionDescription.html">PackageVersionDescription</a> 
      object that contains information about the requested package version.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access package_version outputs
package_version_id = package_version.id
package_version_package_version = package_version.package_version
```

---


### Domain

Domain resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>One or more tag key-value pairs for the domain.</p> |
| `domain` | String | ✅ | <p> The name of the domain to create. All domain names in an Amazon Web Services Region that are in the
      same Amazon Web Services account must be unique. The domain name is used as the prefix in DNS hostnames. Do
      not use sensitive information in a domain name because it is publicly discoverable. </p> |
| `encryption_key` | String |  | <p> The encryption key for the domain. This is used to encrypt content stored in a domain.
      An encryption key can be a key ID, a key Amazon Resource Name (ARN), a key alias, or a key
      alias ARN. To specify an <code>encryptionKey</code>, your IAM role must have
        <code>kms:DescribeKey</code> and <code>kms:CreateGrant</code> permissions on the encryption
      key that is used. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestSyntax">DescribeKey</a> in the <i>Key Management Service API Reference</i>
      and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">Key Management Service API Permissions
        Reference</a> in the <i>Key Management Service Developer Guide</i>. </p>
         <important>
            <p> CodeArtifact supports only symmetric CMKs. Do not associate an asymmetric CMK with your
        domain. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using symmetric and asymmetric
          keys</a> in the <i>Key Management Service Developer Guide</i>. </p>
         </important> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create domain
domain = provider.codeartifact.Domain {
    domain = "value"  # <p> The name of the domain to create. All domain names in an Amazon Web Services Region that are in the
      same Amazon Web Services account must be unique. The domain name is used as the prefix in DNS hostnames. Do
      not use sensitive information in a domain name because it is publicly discoverable. </p>
}

# Access domain outputs
domain_id = domain.id
domain_domain = domain.domain
```

---


### Repository

Repository resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>
      A description of the created repository.
    </p> |
| `tags` | Vec<String> |  | <p>One or more tag key-value pairs for the repository.</p> |
| `repository` | String | ✅ | <p>The name of the repository to create. </p> |
| `upstreams` | Vec<String> |  | <p> A list of upstream repositories to associate with the repository. The order of the upstream repositories 
        in the list determines their priority order when CodeArtifact looks for a requested package version. For more 
        information, see <a href="https://docs.aws.amazon.com/codeartifact/latest/ug/repos-upstream.html">Working with upstream repositories</a>. </p> |
| `domain_owner` | String |  | <p>
        The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include 
        dashes or spaces.
      </p> |
| `domain` | String | ✅ | <p>
        The name of the domain that contains the created repository.
      </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `repository` | String | <p>
         A <code>RepositoryDescription</code> object that contains the requested repository information.
       </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create repository
repository = provider.codeartifact.Repository {
    repository = "value"  # <p>The name of the repository to create. </p>
    domain = "value"  # <p>
        The name of the domain that contains the created repository.
      </p>
}

# Access repository outputs
repository_id = repository.id
repository_repository = repository.repository
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
| `authorization_token` | String | <p>
        The returned authentication token.
       </p> |
| `expiration` | String | <p>
      A timestamp that specifies the date and time the authorization token expires.
    </p> |


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
authorization_token_authorization_token = authorization_token.authorization_token
authorization_token_expiration = authorization_token.expiration
```

---


### Package_version_readme

PackageVersionReadme resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `namespace` | String | <p>The namespace of the package version with the requested readme file. The package component that specifies its 
      namespace depends on its type. For example:</p>
         <ul>
            <li>
               <p>
          The namespace of a Maven package version is its <code>groupId</code>.
        </p>
            </li>
            <li>
               <p>
          The namespace of an npm or Swift package version is its <code>scope</code>.
        </p>
            </li>
            <li>
               <p>The namespace of a generic package is its <code>namespace</code>.</p>
            </li>
            <li>
               <p>
          Python, NuGet, Ruby, and Cargo package versions do not contain a corresponding component, package versions 
          of those formats do not have a namespace.
        </p>
            </li>
         </ul> |
| `version_revision` | String | <p>
      The current revision associated with the package version.
    </p> |
| `format` | String | <p>
      The format of the package with the requested readme file.
    </p> |
| `package` | String | <p>
      The name of the package that contains the returned readme file.
    </p> |
| `version` | String | <p>
      The version of the package with the requested readme file.
    </p> |
| `readme` | String | <p>
      The text of the returned readme file.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access package_version_readme outputs
package_version_readme_id = package_version_readme.id
package_version_readme_namespace = package_version_readme.namespace
package_version_readme_version_revision = package_version_readme.version_revision
package_version_readme_format = package_version_readme.format
package_version_readme_package = package_version_readme.package
package_version_readme_version = package_version_readme.version
package_version_readme_readme = package_version_readme.readme
```

---


### Package_version_asset

PackageVersionAsset resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `package_version_revision` | String | <p>
      The name of the package version revision that contains the downloaded asset.
    </p> |
| `asset_name` | String | <p>
      The name of the asset that is downloaded.
    </p> |
| `package_version` | String | <p>
      A string that contains the package version (for example, <code>3.5.2</code>).
    </p> |
| `asset` | String | <p> The binary file, or asset, that is downloaded.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access package_version_asset outputs
package_version_asset_id = package_version_asset.id
package_version_asset_package_version_revision = package_version_asset.package_version_revision
package_version_asset_asset_name = package_version_asset.asset_name
package_version_asset_package_version = package_version_asset.package_version
package_version_asset_asset = package_version_asset.asset
```

---


### Associated_package_group

AssociatedPackageGroup resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `package_group` | String | <p>The package group that is associated with the requested package.</p> |
| `association_type` | String | <p>Describes the strength of the association between the package and package group. A strong match is also known as an 
    exact match, and a weak match is known as a relative match.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access associated_package_group outputs
associated_package_group_id = associated_package_group.id
associated_package_group_package_group = associated_package_group.package_group
associated_package_group_association_type = associated_package_group.association_type
```

---


### Package_origin_configuration

PackageOriginConfiguration resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `repository` | String | ✅ | <p>The name of the repository that contains the package.</p> |
| `namespace` | String |  | <p>The namespace of the package to be updated. The package component that specifies its 
      namespace depends on its type. For example:</p>
         <ul>
            <li>
               <p>
          The namespace of a Maven package version is its <code>groupId</code>.
        </p>
            </li>
            <li>
               <p>
          The namespace of an npm or Swift package version is its <code>scope</code>.
        </p>
            </li>
            <li>
               <p>The namespace of a generic package is its <code>namespace</code>.</p>
            </li>
            <li>
               <p>
          Python, NuGet, Ruby, and Cargo package versions do not contain a corresponding component, package versions 
          of those formats do not have a namespace.
        </p>
            </li>
         </ul> |
| `package` | String | ✅ | <p>The name of the package to be updated.</p> |
| `restrictions` | String | ✅ | <p>A <a href="https://docs.aws.amazon.com/codeartifact/latest/APIReference/API_PackageOriginRestrictions.html">PackageOriginRestrictions</a> 
      object that contains information about the <code>upstream</code> and <code>publish</code> package origin restrictions. 
      The <code>upstream</code> restriction determines if new package versions can be ingested or retained from external connections or upstream repositories. 
    The <code>publish</code> restriction determines if new package versions can be published directly to the repository.</p>
         <p>You must include both the desired <code>upstream</code> and <code>publish</code> restrictions.</p> |
| `format` | String | ✅ | <p>A format that specifies the type of the package to be updated.</p> |
| `domain` | String | ✅ | <p>The name of the domain that contains the repository that contains the package.</p> |
| `domain_owner` | String |  | <p>
        The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include 
        dashes or spaces.
      </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create package_origin_configuration
package_origin_configuration = provider.codeartifact.Package_origin_configuration {
    repository = "value"  # <p>The name of the repository that contains the package.</p>
    package = "value"  # <p>The name of the package to be updated.</p>
    restrictions = "value"  # <p>A <a href="https://docs.aws.amazon.com/codeartifact/latest/APIReference/API_PackageOriginRestrictions.html">PackageOriginRestrictions</a> 
      object that contains information about the <code>upstream</code> and <code>publish</code> package origin restrictions. 
      The <code>upstream</code> restriction determines if new package versions can be ingested or retained from external connections or upstream repositories. 
    The <code>publish</code> restriction determines if new package versions can be published directly to the repository.</p>
         <p>You must include both the desired <code>upstream</code> and <code>publish</code> restrictions.</p>
    format = "value"  # <p>A format that specifies the type of the package to be updated.</p>
    domain = "value"  # <p>The name of the domain that contains the repository that contains the package.</p>
}

```

---


### Package_versions

PackageVersions resource

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


### Package_versions_status

PackageVersionsStatus resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `namespace` | String |  | <p>The namespace of the package version to be updated. The package component that specifies its 
      namespace depends on its type. For example:</p>
         <ul>
            <li>
               <p>
          The namespace of a Maven package version is its <code>groupId</code>.
        </p>
            </li>
            <li>
               <p>
          The namespace of an npm or Swift package version is its <code>scope</code>.
        </p>
            </li>
            <li>
               <p>The namespace of a generic package is its <code>namespace</code>.</p>
            </li>
            <li>
               <p>
          Python, NuGet, Ruby, and Cargo package versions do not contain a corresponding component, package versions 
          of those formats do not have a namespace.
        </p>
            </li>
         </ul> |
| `version_revisions` | HashMap<String, String> |  | <p> A map of package versions and package version revisions. The map <code>key</code> is the
      package version (for example, <code>3.5.2</code>), and the map <code>value</code> is the
      package version revision. </p> |
| `package` | String | ✅ | <p>
      The name of the package with the version statuses to update.
    </p> |
| `target_status` | String | ✅ | <p>
      The status you want to change the package version status to.
    </p> |
| `repository` | String | ✅ | <p>
      The repository that contains the package versions with the status you want to update. 
    </p> |
| `domain` | String | ✅ | <p>
      The name of the domain that contains the repository that contains the package versions with a status to be updated.
    </p> |
| `versions` | Vec<String> | ✅ | <p>
      An array of strings that specify the versions of the package with the statuses to update.
    </p> |
| `expected_status` | String |  | <p> The package version’s expected status before it is updated. If
        <code>expectedStatus</code> is provided, the package version's status is updated only if its
      status at the time <code>UpdatePackageVersionsStatus</code> is called matches
        <code>expectedStatus</code>. </p> |
| `domain_owner` | String |  | <p>
        The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include 
        dashes or spaces.
      </p> |
| `format` | String | ✅ | <p>
      A format that specifies the type of the package with the statuses to update.
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


### Package_group

PackageGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>
      A description of the package group.
    </p> |
| `contact_info` | String |  | <p>
      The contact information for the created package group.
    </p> |
| `domain` | String | ✅ | <p>
      The name of the domain in which you want to create a package group.
    </p> |
| `tags` | Vec<String> |  | <p>One or more tag key-value pairs for the package group.</p> |
| `domain_owner` | String |  | <p>
        The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include 
        dashes or spaces.
      </p> |
| `package_group` | String | ✅ | <p>The pattern of the package group to create. The pattern is also the identifier of the package group. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `package_group` | String | <p>A <a href="https://docs.aws.amazon.com/codeartifact/latest/APIReference/API_PackageGroupDescription.html">PackageGroupDescription</a> object 
    that contains information about the requested package group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create package_group
package_group = provider.codeartifact.Package_group {
    domain = "value"  # <p>
      The name of the domain in which you want to create a package group.
    </p>
    package_group = "value"  # <p>The pattern of the package group to create. The pattern is also the identifier of the package group. </p>
}

# Access package_group outputs
package_group_id = package_group.id
package_group_package_group = package_group.package_group
```

---


### Domain_permissions_policy

DomainPermissionsPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain` | String | ✅ | <p>
        The name of the domain on which to set the resource policy.
      </p> |
| `policy_revision` | String |  | <p>
        The current revision of the resource policy to be set. This revision is used for optimistic locking, which
        prevents others from overwriting your changes to the domain's resource policy.
      </p> |
| `policy_document` | String | ✅ | <p> A valid displayable JSON Aspen policy string to be set as the access control resource
      policy on the provided domain. </p> |
| `domain_owner` | String |  | <p>
        The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include 
        dashes or spaces.
      </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>
        The returned resource policy.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create domain_permissions_policy
domain_permissions_policy = provider.codeartifact.Domain_permissions_policy {
    domain = "value"  # <p>
        The name of the domain on which to set the resource policy.
      </p>
    policy_document = "value"  # <p> A valid displayable JSON Aspen policy string to be set as the access control resource
      policy on the provided domain. </p>
}

# Access domain_permissions_policy outputs
domain_permissions_policy_id = domain_permissions_policy.id
domain_permissions_policy_policy = domain_permissions_policy.policy
```

---


### Package_group_origin_configuration

PackageGroupOriginConfiguration resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain` | String | ✅ | <p>
      The name of the domain which contains the package group for which to update the origin configuration.
    </p> |
| `package_group` | String | ✅ | <p>
      The pattern of the package group for which to update the origin configuration.
    </p> |
| `domain_owner` | String |  | <p>
        The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include 
        dashes or spaces.
      </p> |
| `restrictions` | HashMap<String, String> |  | <p>
        The origin configuration settings that determine how package versions can enter repositories.
      </p> |
| `add_allowed_repositories` | Vec<String> |  | <p>The repository name and restrictions to add to the allowed repository list of the specified package group.</p> |
| `remove_allowed_repositories` | Vec<String> |  | <p>The repository name and restrictions to remove from the allowed repository list of the specified package group.</p> |



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


### Package

Package resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `package` | String | <p>A <a href="https://docs.aws.amazon.com/codeartifact/latest/APIReference/API_PackageDescription.html">PackageDescription</a> 
      object that contains information about the requested package.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access package outputs
package_id = package.id
package_package = package.package
```

---


### Repository_endpoint

RepositoryEndpoint resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `repository_endpoint` | String | <p>
         A string that specifies the URL of the returned endpoint.
     </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access repository_endpoint outputs
repository_endpoint_id = repository_endpoint.id
repository_endpoint_repository_endpoint = repository_endpoint.repository_endpoint
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple repository_permissions_policy resources
repository_permissions_policy_0 = provider.codeartifact.Repository_permissions_policy {
    domain = "value-0"
    repository = "value-0"
    policy_document = "value-0"
}
repository_permissions_policy_1 = provider.codeartifact.Repository_permissions_policy {
    domain = "value-1"
    repository = "value-1"
    policy_document = "value-1"
}
repository_permissions_policy_2 = provider.codeartifact.Repository_permissions_policy {
    domain = "value-2"
    repository = "value-2"
    policy_document = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    repository_permissions_policy = provider.codeartifact.Repository_permissions_policy {
        domain = "production-value"
        repository = "production-value"
        policy_document = "production-value"
    }
```

---

## Related Documentation

- [AWS Codeartifact Documentation](https://docs.aws.amazon.com/codeartifact/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
