# Appfabric Service



**Resources**: 4

---

## Overview

The appfabric service provides access to 4 resource types:

- [Ingestion_destination](#ingestion_destination) [CRUD]
- [App_bundle](#app_bundle) [CRD]
- [App_authorization](#app_authorization) [CRUD]
- [Ingestion](#ingestion) [CRD]

---

## Resources


### Ingestion_destination

IngestionDestination resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ingestion_identifier` | String | ✅ | <p>The Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the ingestion to
         use for the request.</p> |
| `client_token` | String |  | <p>Specifies a unique, case-sensitive identifier that you provide to ensure the idempotency
         of the request. This lets you safely retry the request without accidentally performing the
         same operation a second time. Passing the same value to a later call to an operation
         requires that you also pass the same value for all other parameters. We recommend that you
         use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of
            value</a>.</p>
         <p>If you don't provide this value, then Amazon Web Services generates a random one for
         you.</p>
         <p>If you retry the operation with the same <code>ClientToken</code>, but with different
         parameters, the retry fails with an <code>IdempotentParameterMismatch</code> error.</p> |
| `tags` | Vec<String> |  | <p>A map of the key-value pairs of the tag or tags to assign to the resource.</p> |
| `app_bundle_identifier` | String | ✅ | <p>The Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the app bundle
         to use for the request.</p> |
| `processing_configuration` | String | ✅ | <p>Contains information about how ingested data is processed.</p> |
| `destination_configuration` | String | ✅ | <p>Contains information about the destination of ingested data.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ingestion_destination` | String | <p>Contains information about an ingestion destination.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ingestion_destination
ingestion_destination = provider.appfabric.Ingestion_destination {
    ingestion_identifier = "value"  # <p>The Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the ingestion to
         use for the request.</p>
    app_bundle_identifier = "value"  # <p>The Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the app bundle
         to use for the request.</p>
    processing_configuration = "value"  # <p>Contains information about how ingested data is processed.</p>
    destination_configuration = "value"  # <p>Contains information about the destination of ingested data.</p>
}

# Access ingestion_destination outputs
ingestion_destination_id = ingestion_destination.id
ingestion_destination_ingestion_destination = ingestion_destination.ingestion_destination
```

---


### App_bundle

AppBundle resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A map of the key-value pairs of the tag or tags to assign to the resource.</p> |
| `customer_managed_key_identifier` | String |  | <p>The Amazon Resource Name (ARN) of the Key Management Service (KMS) key to
         use to encrypt the application data. If this is not specified, an Amazon Web Services owned key is used for encryption.</p> |
| `client_token` | String |  | <p>Specifies a unique, case-sensitive identifier that you provide to ensure the idempotency
         of the request. This lets you safely retry the request without accidentally performing the
         same operation a second time. Passing the same value to a later call to an operation
         requires that you also pass the same value for all other parameters. We recommend that you
         use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of
            value</a>.</p>
         <p>If you don't provide this value, then Amazon Web Services generates a random one for
         you.</p>
         <p>If you retry the operation with the same <code>ClientToken</code>, but with different
         parameters, the retry fails with an <code>IdempotentParameterMismatch</code> error.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_bundle` | String | <p>Contains information about an app bundle.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app_bundle
app_bundle = provider.appfabric.App_bundle {
}

# Access app_bundle outputs
app_bundle_id = app_bundle.id
app_bundle_app_bundle = app_bundle.app_bundle
```

---


### App_authorization

AppAuthorization resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `credential` | String | ✅ | <p>Contains credentials for the application, such as an API key or OAuth2 client ID and
         secret.</p>
         <p>Specify credentials that match the authorization type for your request. For example, if
         the authorization type for your request is OAuth2 (<code>oauth2</code>), then you should
         provide only the OAuth2 credentials.</p> |
| `app` | String | ✅ | <p>The name of the application.</p>
         <p>Valid values are:</p>
         <ul>
            <li>
               <p>
                  <code>SLACK</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ASANA</code>
               </p>
            </li>
            <li>
               <p>
                  <code>JIRA</code>
               </p>
            </li>
            <li>
               <p>
                  <code>M365</code>
               </p>
            </li>
            <li>
               <p>
                  <code>M365AUDITLOGS</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ZOOM</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ZENDESK</code>
               </p>
            </li>
            <li>
               <p>
                  <code>OKTA</code>
               </p>
            </li>
            <li>
               <p>
                  <code>GOOGLE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DROPBOX</code>
               </p>
            </li>
            <li>
               <p>
                  <code>SMARTSHEET</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CISCO</code>
               </p>
            </li>
         </ul> |
| `tenant` | String | ✅ | <p>Contains information about an application tenant, such as the application display name
         and identifier.</p> |
| `auth_type` | String | ✅ | <p>The authorization type for the app authorization.</p> |
| `client_token` | String |  | <p>Specifies a unique, case-sensitive identifier that you provide to ensure the idempotency
         of the request. This lets you safely retry the request without accidentally performing the
         same operation a second time. Passing the same value to a later call to an operation
         requires that you also pass the same value for all other parameters. We recommend that you
         use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of
            value</a>.</p>
         <p>If you don't provide this value, then Amazon Web Services generates a random one for
         you.</p>
         <p>If you retry the operation with the same <code>ClientToken</code>, but with different
         parameters, the retry fails with an <code>IdempotentParameterMismatch</code> error.</p> |
| `tags` | Vec<String> |  | <p>A map of the key-value pairs of the tag or tags to assign to the resource.</p> |
| `app_bundle_identifier` | String | ✅ | <p>The Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the app bundle
         to use for the request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_authorization` | String | <p>Contains information about an app authorization.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app_authorization
app_authorization = provider.appfabric.App_authorization {
    credential = "value"  # <p>Contains credentials for the application, such as an API key or OAuth2 client ID and
         secret.</p>
         <p>Specify credentials that match the authorization type for your request. For example, if
         the authorization type for your request is OAuth2 (<code>oauth2</code>), then you should
         provide only the OAuth2 credentials.</p>
    app = "value"  # <p>The name of the application.</p>
         <p>Valid values are:</p>
         <ul>
            <li>
               <p>
                  <code>SLACK</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ASANA</code>
               </p>
            </li>
            <li>
               <p>
                  <code>JIRA</code>
               </p>
            </li>
            <li>
               <p>
                  <code>M365</code>
               </p>
            </li>
            <li>
               <p>
                  <code>M365AUDITLOGS</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ZOOM</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ZENDESK</code>
               </p>
            </li>
            <li>
               <p>
                  <code>OKTA</code>
               </p>
            </li>
            <li>
               <p>
                  <code>GOOGLE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DROPBOX</code>
               </p>
            </li>
            <li>
               <p>
                  <code>SMARTSHEET</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CISCO</code>
               </p>
            </li>
         </ul>
    tenant = "value"  # <p>Contains information about an application tenant, such as the application display name
         and identifier.</p>
    auth_type = "value"  # <p>The authorization type for the app authorization.</p>
    app_bundle_identifier = "value"  # <p>The Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the app bundle
         to use for the request.</p>
}

# Access app_authorization outputs
app_authorization_id = app_authorization.id
app_authorization_app_authorization = app_authorization.app_authorization
```

---


### Ingestion

Ingestion resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_bundle_identifier` | String | ✅ | <p>The Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the app bundle
         to use for the request.</p> |
| `ingestion_type` | String | ✅ | <p>The ingestion type.</p> |
| `tags` | Vec<String> |  | <p>A map of the key-value pairs of the tag or tags to assign to the resource.</p> |
| `app` | String | ✅ | <p>The name of the application.</p>
         <p>Valid values are:</p>
         <ul>
            <li>
               <p>
                  <code>SLACK</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ASANA</code>
               </p>
            </li>
            <li>
               <p>
                  <code>JIRA</code>
               </p>
            </li>
            <li>
               <p>
                  <code>M365</code>
               </p>
            </li>
            <li>
               <p>
                  <code>M365AUDITLOGS</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ZOOM</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ZENDESK</code>
               </p>
            </li>
            <li>
               <p>
                  <code>OKTA</code>
               </p>
            </li>
            <li>
               <p>
                  <code>GOOGLE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DROPBOX</code>
               </p>
            </li>
            <li>
               <p>
                  <code>SMARTSHEET</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CISCO</code>
               </p>
            </li>
         </ul> |
| `tenant_id` | String | ✅ | <p>The ID of the application tenant.</p> |
| `client_token` | String |  | <p>Specifies a unique, case-sensitive identifier that you provide to ensure the idempotency
         of the request. This lets you safely retry the request without accidentally performing the
         same operation a second time. Passing the same value to a later call to an operation
         requires that you also pass the same value for all other parameters. We recommend that you
         use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of
            value</a>.</p>
         <p>If you don't provide this value, then Amazon Web Services generates a random one for
         you.</p>
         <p>If you retry the operation with the same <code>ClientToken</code>, but with different
         parameters, the retry fails with an <code>IdempotentParameterMismatch</code> error.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ingestion` | String | <p>Contains information about an ingestion.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ingestion
ingestion = provider.appfabric.Ingestion {
    app_bundle_identifier = "value"  # <p>The Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the app bundle
         to use for the request.</p>
    ingestion_type = "value"  # <p>The ingestion type.</p>
    app = "value"  # <p>The name of the application.</p>
         <p>Valid values are:</p>
         <ul>
            <li>
               <p>
                  <code>SLACK</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ASANA</code>
               </p>
            </li>
            <li>
               <p>
                  <code>JIRA</code>
               </p>
            </li>
            <li>
               <p>
                  <code>M365</code>
               </p>
            </li>
            <li>
               <p>
                  <code>M365AUDITLOGS</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ZOOM</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ZENDESK</code>
               </p>
            </li>
            <li>
               <p>
                  <code>OKTA</code>
               </p>
            </li>
            <li>
               <p>
                  <code>GOOGLE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DROPBOX</code>
               </p>
            </li>
            <li>
               <p>
                  <code>SMARTSHEET</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CISCO</code>
               </p>
            </li>
         </ul>
    tenant_id = "value"  # <p>The ID of the application tenant.</p>
}

# Access ingestion outputs
ingestion_id = ingestion.id
ingestion_ingestion = ingestion.ingestion
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple ingestion_destination resources
ingestion_destination_0 = provider.appfabric.Ingestion_destination {
    ingestion_identifier = "value-0"
    app_bundle_identifier = "value-0"
    processing_configuration = "value-0"
    destination_configuration = "value-0"
}
ingestion_destination_1 = provider.appfabric.Ingestion_destination {
    ingestion_identifier = "value-1"
    app_bundle_identifier = "value-1"
    processing_configuration = "value-1"
    destination_configuration = "value-1"
}
ingestion_destination_2 = provider.appfabric.Ingestion_destination {
    ingestion_identifier = "value-2"
    app_bundle_identifier = "value-2"
    processing_configuration = "value-2"
    destination_configuration = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    ingestion_destination = provider.appfabric.Ingestion_destination {
        ingestion_identifier = "production-value"
        app_bundle_identifier = "production-value"
        processing_configuration = "production-value"
        destination_configuration = "production-value"
    }
```

---

## Related Documentation

- [AWS Appfabric Documentation](https://docs.aws.amazon.com/appfabric/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
