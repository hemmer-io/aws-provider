# Connectparticipant Service



**Resources**: 5

---

## Overview

The connectparticipant service provides access to 5 resource types:

- [Authentication_url](#authentication_url) [R]
- [Transcript](#transcript) [R]
- [View](#view) [R]
- [Participant_connection](#participant_connection) [C]
- [Attachment](#attachment) [R]

---

## Resources


### Authentication_url

AuthenticationUrl resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `authentication_url` | String | <p>The URL where the customer will sign in to the identity provider. This URL contains
            the authorize endpoint for the Cognito UserPool used in the authentication.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access authentication_url outputs
authentication_url_id = authentication_url.id
authentication_url_authentication_url = authentication_url.authentication_url
```

---


### Transcript

Transcript resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The pagination token. Use the value returned previously in the next subsequent request
            to retrieve the next set of results.</p> |
| `initial_contact_id` | String | <p>The initial contact ID for the contact. </p> |
| `transcript` | Vec<String> | <p>The list of messages in the session.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transcript outputs
transcript_id = transcript.id
transcript_next_token = transcript.next_token
transcript_initial_contact_id = transcript.initial_contact_id
transcript_transcript = transcript.transcript
```

---


### View

View resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `view` | String | <p>A view resource object. Contains metadata and content necessary to render the
            view.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access view outputs
view_id = view.id
view_view = view.view
```

---


### Participant_connection

ParticipantConnection resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | Vec<String> |  | <p>Type of connection information required. If you need
                <code>CONNECTION_CREDENTIALS</code> along with marking participant as connected,
            pass <code>CONNECTION_CREDENTIALS</code> in <code>Type</code>.</p> |
| `connect_participant` | bool |  | <p>Amazon Connect Participant is used to mark the participant as connected for customer
            participant in message streaming, as well as for agent or manager participant in
            non-streaming chats.</p> |
| `participant_token` | String | ✅ | <p>This is a header parameter.</p>
         <p>The ParticipantToken as obtained from <a href="https://docs.aws.amazon.com/connect/latest/APIReference/API_StartChatContact.html">StartChatContact</a>
            API response.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create participant_connection
participant_connection = provider.connectparticipant.Participant_connection {
    participant_token = "value"  # <p>This is a header parameter.</p>
         <p>The ParticipantToken as obtained from <a href="https://docs.aws.amazon.com/connect/latest/APIReference/API_StartChatContact.html">StartChatContact</a>
            API response.</p>
}

```

---


### Attachment

Attachment resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `url_expiry` | String | <p>The expiration time of the URL in ISO timestamp. It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p> |
| `url` | String | <p>This is the pre-signed URL that can be used for uploading the file to Amazon S3 when used in response 
to <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_StartAttachmentUpload.html">StartAttachmentUpload</a>.</p> |
| `attachment_size_in_bytes` | i64 | <p>The size of the attachment in bytes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access attachment outputs
attachment_id = attachment.id
attachment_url_expiry = attachment.url_expiry
attachment_url = attachment.url
attachment_attachment_size_in_bytes = attachment.attachment_size_in_bytes
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple authentication_url resources
authentication_url_0 = provider.connectparticipant.Authentication_url {
}
authentication_url_1 = provider.connectparticipant.Authentication_url {
}
authentication_url_2 = provider.connectparticipant.Authentication_url {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    authentication_url = provider.connectparticipant.Authentication_url {
    }
```

---

## Related Documentation

- [AWS Connectparticipant Documentation](https://docs.aws.amazon.com/connectparticipant/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
