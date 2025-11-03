# Chime_sdk_meetings Service



**Resources**: 4

---

## Overview

The chime_sdk_meetings service provides access to 4 resource types:

- [Meeting](#meeting) [CRD]
- [Meeting_with_attendees](#meeting_with_attendees) [C]
- [Attendee_capabilities](#attendee_capabilities) [U]
- [Attendee](#attendee) [CRD]

---

## Resources


### Meeting

Meeting resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `media_region` | String | ✅ | <p>The Region in which to create the meeting.</p>
         <p>
           Available values: 
           <code>af-south-1</code>, 
           <code>ap-northeast-1</code>, 
           <code>ap-northeast-2</code>, 
           <code>ap-south-1</code>, 
           <code>ap-southeast-1</code>, 
           <code>ap-southeast-2</code>,           
           <code>ca-central-1</code>, 
           <code>eu-central-1</code>, 
           <code>eu-north-1</code>, 
           <code>eu-south-1</code>, 
           <code>eu-west-1</code>, 
           <code>eu-west-2</code>, 
           <code>eu-west-3</code>,            
           <code>sa-east-1</code>, 
           <code>us-east-1</code>, 
           <code>us-east-2</code>, 
           <code>us-west-1</code>, 
           <code>us-west-2</code>.
       </p>
         <p>Available values in Amazon Web Services GovCloud (US) Regions: <code>us-gov-east-1</code>, <code>us-gov-west-1</code>.</p> |
| `notifications_configuration` | String |  | <p>The configuration for resource targets to receive notifications when meeting and attendee events occur.</p> |
| `external_meeting_id` | String | ✅ | <p>The external meeting ID.</p>
         <p>Pattern: <code>[-_&@+=,(){}\[\]\/«».:|'"#a-zA-Z0-9À-ÿ\s]*</code>
         </p>
         <p>Values that begin with <code>aws:</code> are reserved. You can't configure a value that uses this prefix. 
           Case insensitive.</p> |
| `meeting_features` | String |  | <p>Lists the audio and video features enabled for a meeting, such as echo reduction.</p> |
| `tenant_ids` | Vec<String> |  | <p>A consistent and opaque identifier, created and maintained by the builder to represent a segment of their users.</p> |
| `media_placement_network_type` | String |  | <p>The type of network for the media placement. Either IPv4 only or dual-stack (IPv4 and IPv6).</p> |
| `primary_meeting_id` | String |  | <p>When specified, replicates the media from the primary meeting to the new meeting.</p> |
| `tags` | Vec<String> |  | <p>Applies one or more tags to an Amazon Chime SDK meeting. Note the following:</p>
         <ul>
            <li>
               <p>Not all resources have tags. For a list of services with resources that support tagging using this operation, see 
        <a href="https://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/supported-services.html">Services that support the Resource Groups Tagging API</a>. If the resource 
        doesn't yet support this operation, the resource's service might support tagging using its own API operations. For more information, refer to the documentation for that service.</p>
            </li>
            <li>
               <p>Each resource can have up to 50 tags. For other limits, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html#tag-conventions">Tag Naming and Usage Conventions</a> in the 
       <i>AWS General Reference</i>.</p>
            </li>
            <li>
               <p>You can only tag resources that are located in the specified Amazon Web Services Region for the Amazon Web Services account.</p>
            </li>
            <li>
               <p>To add tags to a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for adding tags. For more information, see the 
        documentation for each service.</p>
            </li>
         </ul>
         <important>
            <p>Do not store personally identifiable information (PII) or other confidential or sensitive information in tags. We use tags to provide you with billing and administration services. Tags are not intended to be 
            used for private or sensitive data.</p>
         </important>
         <p>
            <b>Minimum permissions</b>
         </p>
         <p>In addition to the <code>tag:TagResources</code> permission required by this operation, you must also have the tagging permission defined by the service that created the resource. For example, 
                   to tag a <code>ChimeSDKMeetings</code> instance using the <code>TagResources</code> operation, you must have both of the following permissions:</p>
         <p>
            <code>tag:TagResources</code>
         </p>
         <p>
            <code>ChimeSDKMeetings:CreateTags</code>
         </p>
         <note>
            <p>Some services might have specific requirements for tagging some resources. For example, to tag an Amazon S3 bucket, you must also have the <code>s3:GetBucketTagging</code> permission. 
                If the expected minimum permissions don't work, check the documentation for that service's tagging APIs for more information.</p>
         </note> |
| `meeting_host_id` | String |  | <p>Reserved.</p> |
| `client_request_token` | String | ✅ | <p>The unique identifier for the client request. Use a different token for different meetings.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `meeting` | String | <p>The Amazon Chime SDK meeting information.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create meeting
meeting = provider.chime_sdk_meetings.Meeting {
    media_region = "value"  # <p>The Region in which to create the meeting.</p>
         <p>
           Available values: 
           <code>af-south-1</code>, 
           <code>ap-northeast-1</code>, 
           <code>ap-northeast-2</code>, 
           <code>ap-south-1</code>, 
           <code>ap-southeast-1</code>, 
           <code>ap-southeast-2</code>,           
           <code>ca-central-1</code>, 
           <code>eu-central-1</code>, 
           <code>eu-north-1</code>, 
           <code>eu-south-1</code>, 
           <code>eu-west-1</code>, 
           <code>eu-west-2</code>, 
           <code>eu-west-3</code>,            
           <code>sa-east-1</code>, 
           <code>us-east-1</code>, 
           <code>us-east-2</code>, 
           <code>us-west-1</code>, 
           <code>us-west-2</code>.
       </p>
         <p>Available values in Amazon Web Services GovCloud (US) Regions: <code>us-gov-east-1</code>, <code>us-gov-west-1</code>.</p>
    external_meeting_id = "value"  # <p>The external meeting ID.</p>
         <p>Pattern: <code>[-_&@+=,(){}\[\]\/«».:|'"#a-zA-Z0-9À-ÿ\s]*</code>
         </p>
         <p>Values that begin with <code>aws:</code> are reserved. You can't configure a value that uses this prefix. 
           Case insensitive.</p>
    client_request_token = "value"  # <p>The unique identifier for the client request. Use a different token for different meetings.</p>
}

# Access meeting outputs
meeting_id = meeting.id
meeting_meeting = meeting.meeting
```

---


### Meeting_with_attendees

MeetingWithAttendees resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_token` | String | ✅ | <p>The unique identifier for the client request. Use a different token for different meetings.</p> |
| `meeting_features` | String |  | <p>Lists the audio and video features enabled for a meeting, such as echo reduction.</p> |
| `tags` | Vec<String> |  | <p>The tags in the request.</p> |
| `notifications_configuration` | String |  | <p>The configuration for resource targets to receive notifications when meeting and attendee events occur.</p> |
| `attendees` | Vec<String> | ✅ | <p>The attendee information, including attendees' IDs and join tokens.</p> |
| `external_meeting_id` | String | ✅ | <p>The external meeting ID.</p>
         <p>Pattern: <code>[-_&@+=,(){}\[\]\/«».:|'"#a-zA-Z0-9À-ÿ\s]*</code>
         </p>
         <p>Values that begin with <code>aws:</code> are reserved. You can't configure a value that uses this prefix. 
           Case insensitive.</p> |
| `primary_meeting_id` | String |  | <p>When specified, replicates the media from the primary meeting to the new meeting.</p> |
| `tenant_ids` | Vec<String> |  | <p>A consistent and opaque identifier, created and maintained by the builder to represent a segment of their users.</p> |
| `media_placement_network_type` | String |  | <p>The type of network for the media placement. Either IPv4 only or dual-stack (IPv4 and IPv6).</p> |
| `media_region` | String | ✅ | <p>The Region in which to create the meeting.</p>
         <p>
           Available values: 
           <code>af-south-1</code>, 
           <code>ap-northeast-1</code>, 
           <code>ap-northeast-2</code>, 
           <code>ap-south-1</code>, 
           <code>ap-southeast-1</code>, 
           <code>ap-southeast-2</code>,           
           <code>ca-central-1</code>, 
           <code>eu-central-1</code>, 
           <code>eu-north-1</code>, 
           <code>eu-south-1</code>, 
           <code>eu-west-1</code>, 
           <code>eu-west-2</code>, 
           <code>eu-west-3</code>,            
           <code>sa-east-1</code>, 
           <code>us-east-1</code>, 
           <code>us-east-2</code>, 
           <code>us-west-1</code>, 
           <code>us-west-2</code>.
       </p>
         <p>Available values in Amazon Web Services GovCloud (US) Regions: <code>us-gov-east-1</code>, <code>us-gov-west-1</code>.</p> |
| `meeting_host_id` | String |  | <p>Reserved.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create meeting_with_attendees
meeting_with_attendees = provider.chime_sdk_meetings.Meeting_with_attendees {
    client_request_token = "value"  # <p>The unique identifier for the client request. Use a different token for different meetings.</p>
    attendees = "value"  # <p>The attendee information, including attendees' IDs and join tokens.</p>
    external_meeting_id = "value"  # <p>The external meeting ID.</p>
         <p>Pattern: <code>[-_&@+=,(){}\[\]\/«».:|'"#a-zA-Z0-9À-ÿ\s]*</code>
         </p>
         <p>Values that begin with <code>aws:</code> are reserved. You can't configure a value that uses this prefix. 
           Case insensitive.</p>
    media_region = "value"  # <p>The Region in which to create the meeting.</p>
         <p>
           Available values: 
           <code>af-south-1</code>, 
           <code>ap-northeast-1</code>, 
           <code>ap-northeast-2</code>, 
           <code>ap-south-1</code>, 
           <code>ap-southeast-1</code>, 
           <code>ap-southeast-2</code>,           
           <code>ca-central-1</code>, 
           <code>eu-central-1</code>, 
           <code>eu-north-1</code>, 
           <code>eu-south-1</code>, 
           <code>eu-west-1</code>, 
           <code>eu-west-2</code>, 
           <code>eu-west-3</code>,            
           <code>sa-east-1</code>, 
           <code>us-east-1</code>, 
           <code>us-east-2</code>, 
           <code>us-west-1</code>, 
           <code>us-west-2</code>.
       </p>
         <p>Available values in Amazon Web Services GovCloud (US) Regions: <code>us-gov-east-1</code>, <code>us-gov-west-1</code>.</p>
}

```

---


### Attendee_capabilities

AttendeeCapabilities resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `meeting_id` | String | ✅ | <p>The ID of the meeting associated with the update request.</p> |
| `attendee_id` | String | ✅ | <p>The ID of the attendee associated with the update request.</p> |
| `capabilities` | String | ✅ | <p>The capabilities that you want to update.</p> |



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


### Attendee

Attendee resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `capabilities` | String |  | <p>The capabilities (<code>audio</code>, <code>video</code>, or <code>content</code>) that you want to grant an attendee. If you don't specify capabilities, all users have send and receive capabilities on 
            all media channels by default.</p>
         <note>
            <p>You use the capabilities with a set of values that control what the capabilities can do, such as <code>SendReceive</code> data. For more information about those values, see 
            .</p>
         </note>
         <p>When using capabilities, be aware of these corner cases:</p>
         <ul>
            <li>
               <p>If you specify <code>MeetingFeatures:Video:MaxResolution:None</code> when you create a meeting, all API requests 
                        that include <code>SendReceive</code>, <code>Send</code>, or <code>Receive</code> for <code>AttendeeCapabilities:Video</code> will be rejected with <code>ValidationError 400</code>.</p>
            </li>
            <li>
               <p>If you specify <code>MeetingFeatures:Content:MaxResolution:None</code> when you create a meeting, all API requests that include <code>SendReceive</code>, <code>Send</code>, or 
                        <code>Receive</code> for <code>AttendeeCapabilities:Content</code> will be rejected with <code>ValidationError 400</code>.</p>
            </li>
            <li>
               <p>You can't set <code>content</code> capabilities to <code>SendReceive</code> or <code>Receive</code> unless you also set <code>video</code> capabilities to <code>SendReceive</code> 
                    or <code>Receive</code>. If you don't set the <code>video</code> capability to receive, the response will contain an HTTP 400 Bad Request status code. However, you can set your <code>video</code> capability 
                    to receive and you set your <code>content</code> capability to not receive.</p>
            </li>
            <li>
               <p>If meeting features is defined as <code>Video:MaxResolution:None</code> but
                        <code>Content:MaxResolution</code> is defined as something other than
                        <code>None</code> and attendee capabilities are not defined in the API
                        request, then the default attendee video capability is set to
                        <code>Receive</code> and attendee content capability is set to
                        <code>SendReceive</code>. This is because content <code>SendReceive</code>
                        requires video to be at least <code>Receive</code>.</p>
            </li>
            <li>
               <p>When you change an <code>audio</code> capability from <code>None</code> or <code>Receive</code> to <code>Send</code> or <code>SendReceive</code> , 
                    and if the attendee left their microphone unmuted, audio will flow from the attendee to the other meeting participants.</p>
            </li>
            <li>
               <p>When you change a <code>video</code> or <code>content</code> capability from <code>None</code> or <code>Receive</code> to <code>Send</code> or <code>SendReceive</code> , 
                   and if the attendee turned on their video or content streams, remote attendees can receive those streams, but only after media renegotiation between the client and the Amazon Chime back-end server.</p>
            </li>
         </ul> |
| `meeting_id` | String | ✅ | <p>The unique ID of the meeting.</p> |
| `external_user_id` | String | ✅ | <p>The Amazon Chime SDK external user ID. An idempotency token. Links the attendee to an identity managed by a builder application.</p>
         <p>Pattern: <code>[-_&@+=,(){}\[\]\/«».:|'"#a-zA-Z0-9À-ÿ\s]*</code>
         </p>
         <p>Values that begin with <code>aws:</code> are reserved. You can't configure a value that
            uses this prefix.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attendee` | String | <p>The Amazon Chime SDK attendee information.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create attendee
attendee = provider.chime_sdk_meetings.Attendee {
    meeting_id = "value"  # <p>The unique ID of the meeting.</p>
    external_user_id = "value"  # <p>The Amazon Chime SDK external user ID. An idempotency token. Links the attendee to an identity managed by a builder application.</p>
         <p>Pattern: <code>[-_&@+=,(){}\[\]\/«».:|'"#a-zA-Z0-9À-ÿ\s]*</code>
         </p>
         <p>Values that begin with <code>aws:</code> are reserved. You can't configure a value that
            uses this prefix.</p>
}

# Access attendee outputs
attendee_id = attendee.id
attendee_attendee = attendee.attendee
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple meeting resources
meeting_0 = provider.chime_sdk_meetings.Meeting {
    media_region = "value-0"
    external_meeting_id = "value-0"
    client_request_token = "value-0"
}
meeting_1 = provider.chime_sdk_meetings.Meeting {
    media_region = "value-1"
    external_meeting_id = "value-1"
    client_request_token = "value-1"
}
meeting_2 = provider.chime_sdk_meetings.Meeting {
    media_region = "value-2"
    external_meeting_id = "value-2"
    client_request_token = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    meeting = provider.chime_sdk_meetings.Meeting {
        media_region = "production-value"
        external_meeting_id = "production-value"
        client_request_token = "production-value"
    }
```

---

## Related Documentation

- [AWS Chime_sdk_meetings Documentation](https://docs.aws.amazon.com/chime_sdk_meetings/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
