# Gameliftstreams Service



**Resources**: 2

---

## Overview

The gameliftstreams service provides access to 2 resource types:

- [Stream_session](#stream_session) [R]
- [Stream_session_connection](#stream_session_connection) [C]

---

## Resources


### Stream_session

StreamSession resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `export_files_metadata` | String | <p>Provides details about the stream session's exported files. </p> |
| `protocol` | String | <p>The data transfer protocol in use with the stream session.</p> |
| `status` | String | <p>The current status of the stream session. A stream session is ready for a client to connect when in <code>ACTIVE</code> status.</p> <ul> <li> <p> <code>ACTIVATING</code>: The stream session is starting and preparing to stream.</p> </li> <li> <p> <code>ACTIVE</code>: The stream session is ready and waiting for a client connection. A client has <code>ConnectionTimeoutSeconds</code> (specified in <code>StartStreamSession</code>) from when the session reaches <code>ACTIVE</code> state to establish a connection. If no client connects within this timeframe, the session automatically terminates.</p> </li> <li> <p> <code>CONNECTED</code>: The stream session has a connected client. A session will automatically terminate if there is no user input for 60 minutes, or if the maximum length of a session specified by <code>SessionLengthSeconds</code> in <code>StartStreamSession</code> is exceeded.</p> </li> <li> <p> <code>ERROR</code>: The stream session failed to activate. See <code>StatusReason</code> (returned by <code>GetStreamSession</code> and <code>StartStreamSession</code>) for more information.</p> </li> <li> <p> <code>PENDING_CLIENT_RECONNECTION</code>: A client has recently disconnected and the stream session is waiting for the client to reconnect. A client has <code>ConnectionTimeoutSeconds</code> (specified in <code>StartStreamSession</code>) from when the session reaches <code>PENDING_CLIENT_RECONNECTION</code> state to re-establish a connection. If no client connects within this timeframe, the session automatically terminates.</p> </li> <li> <p> <code>RECONNECTING</code>: A client has initiated a reconnect to a session that was in <code>PENDING_CLIENT_RECONNECTION</code> state.</p> </li> <li> <p> <code>TERMINATING</code>: The stream session is ending.</p> </li> <li> <p> <code>TERMINATED</code>: The stream session has ended.</p> </li> </ul> |
| `location` | String | <p>The location where Amazon GameLift Streams hosts and streams your application. For example, <code>us-east-1</code>. For a complete list of locations that Amazon GameLift Streams supports, refer to <a href="https://docs.aws.amazon.com/gameliftstreams/latest/developerguide/regions-quotas.html">Regions, quotas, and limitations</a> in the <i>Amazon GameLift Streams Developer Guide</i>. </p> |
| `signal_response` | String | <p>The WebRTC answer string that the stream server generates in response to the <code>SignalRequest</code>.</p> |
| `connection_timeout_seconds` | i64 | <p>The length of time that Amazon GameLift Streams should wait for a client to connect or reconnect to the stream session. This time span starts when the stream session reaches <code>ACTIVE</code> or <code>PENDING_CLIENT_RECONNECTION</code> state. If no client connects (or reconnects) before the timeout, Amazon GameLift Streams terminates the stream session.</p> |
| `application_arn` | String | <p>The application streaming in this session.</p> <p>This value is an <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference-arns.html">Amazon Resource Name (ARN)</a> that uniquely identifies the application resource. Example ARN: <code>arn:aws:gameliftstreams:us-west-2:111122223333:application/a-9ZY8X7Wv6</code>. </p> |
| `stream_group_id` | String | <p>The unique identifier for the Amazon GameLift Streams stream group that is hosting the stream session. Format example: <code>sg-1AB2C3De4</code>.</p> |
| `arn` | String | <p>The <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference-arns.html">Amazon Resource Name (ARN)</a> that's assigned to a stream session resource. When combined with the stream group resource ID, this value uniquely identifies the stream session across all Amazon Web Services Regions. Format is <code>arn:aws:gameliftstreams:[AWS Region]:[AWS account]:streamsession/[stream group resource ID]/[stream session resource ID]</code>.</p> |
| `description` | String | <p>A human-readable label for the stream session. You can update this value at any time.</p> |
| `status_reason` | String | <p>A short description of the reason the stream session is in <code>ERROR</code> status or <code>TERMINATED</code> status.</p> <p> <code>ERROR</code> status reasons:</p> <ul> <li> <p> <code>applicationLogS3DestinationError</code>: Could not write the application log to the Amazon S3 bucket that is configured for the streaming application. Make sure the bucket still exists.</p> </li> <li> <p> <code>internalError</code>: An internal service error occurred. Start a new stream session to continue streaming.</p> </li> <li> <p> <code>invalidSignalRequest</code>: The WebRTC signal request that was sent is not valid. When starting or reconnecting to a stream session, use <code>generateSignalRequest</code> in the Amazon GameLift Streams Web SDK to generate a new signal request.</p> </li> <li> <p> <code>placementTimeout</code>: Amazon GameLift Streams could not find available stream capacity to start a stream session. Increase the stream capacity in the stream group or wait until capacity becomes available.</p> </li> </ul> <p> <code>TERMINATED</code> status reasons:</p> <ul> <li> <p> <code>apiTerminated</code>: The stream session was terminated by an API call to <a href="https://docs.aws.amazon.com/gameliftstreams/latest/apireference/API_TerminateStreamSession.html">TerminateStreamSession</a>.</p> </li> <li> <p> <code>applicationExit</code>: The streaming application exited or crashed. The stream session was terminated because the application is no longer running.</p> </li> <li> <p> <code>connectionTimeout</code>: The stream session was terminated because the client failed to connect within the connection timeout period specified by <code>ConnectionTimeoutSeconds</code>.</p> </li> <li> <p> <code>idleTimeout</code>: The stream session was terminated because it exceeded the idle timeout period of 60 minutes with no user input activity.</p> </li> <li> <p> <code>maxSessionLengthTimeout</code>: The stream session was terminated because it exceeded the maximum session length timeout period specified by <code>SessionLengthSeconds</code>.</p> </li> <li> <p> <code>reconnectionTimeout</code>: The stream session was terminated because the client failed to reconnect within the reconnection timeout period specified by <code>ConnectionTimeoutSeconds</code> after losing connection.</p> </li> </ul> |
| `additional_environment_variables` | HashMap<String, String> | <p>A set of options that you can use to control the stream session runtime environment, expressed as a set of key-value pairs. You can use this to configure the application or stream session details. You can also provide custom environment variables that Amazon GameLift Streams passes to your game client.</p> <note> <p>If you want to debug your application with environment variables, we recommend that you do so in a local environment outside of Amazon GameLift Streams. For more information, refer to the Compatibility Guidance in the troubleshooting section of the Developer Guide.</p> </note> <p> <code>AdditionalEnvironmentVariables</code> and <code>AdditionalLaunchArgs</code> have similar purposes. <code>AdditionalEnvironmentVariables</code> passes data using environment variables; while <code>AdditionalLaunchArgs</code> passes data using command-line arguments.</p> |
| `session_length_seconds` | i64 | <p>The maximum duration of a session. Amazon GameLift Streams will automatically terminate a session after this amount of time has elapsed, regardless of any existing client connections.</p> |
| `last_updated_at` | String | <p>A timestamp that indicates when this resource was last updated. Timestamps are expressed using in ISO8601 format, such as: <code>2022-12-27T22:29:40+00:00</code> (UTC).</p> |
| `log_file_location_uri` | String | <p>Access location for log files that your content generates during a stream session. These log files are uploaded to cloud storage location at the end of a stream session. The Amazon GameLift Streams application resource defines which log files to upload.</p> |
| `web_sdk_protocol_url` | String | <p>The URL of an S3 bucket that stores Amazon GameLift Streams WebSDK files. The URL is used to establish connection with the client.</p> |
| `signal_request` | String | <p>The WebRTC ICE offer string that a client generates to initiate a connection to the stream session.</p> |
| `additional_launch_args` | Vec<String> | <p>A list of CLI arguments that are sent to the streaming server when a stream session launches. You can use this to configure the application or stream session details. You can also provide custom arguments that Amazon GameLift Streams passes to your game client.</p> <p> <code>AdditionalEnvironmentVariables</code> and <code>AdditionalLaunchArgs</code> have similar purposes. <code>AdditionalEnvironmentVariables</code> passes data using environment variables; while <code>AdditionalLaunchArgs</code> passes data using command-line arguments.</p> |
| `created_at` | String | <p>A timestamp that indicates when this resource was created. Timestamps are expressed using in ISO8601 format, such as: <code>2022-12-27T22:29:40+00:00</code> (UTC).</p> |
| `user_id` | String | <p> An opaque, unique identifier for an end-user, defined by the developer. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access stream_session outputs
stream_session_id = stream_session.id
stream_session_export_files_metadata = stream_session.export_files_metadata
stream_session_protocol = stream_session.protocol
stream_session_status = stream_session.status
stream_session_location = stream_session.location
stream_session_signal_response = stream_session.signal_response
stream_session_connection_timeout_seconds = stream_session.connection_timeout_seconds
stream_session_application_arn = stream_session.application_arn
stream_session_stream_group_id = stream_session.stream_group_id
stream_session_arn = stream_session.arn
stream_session_description = stream_session.description
stream_session_status_reason = stream_session.status_reason
stream_session_additional_environment_variables = stream_session.additional_environment_variables
stream_session_session_length_seconds = stream_session.session_length_seconds
stream_session_last_updated_at = stream_session.last_updated_at
stream_session_log_file_location_uri = stream_session.log_file_location_uri
stream_session_web_sdk_protocol_url = stream_session.web_sdk_protocol_url
stream_session_signal_request = stream_session.signal_request
stream_session_additional_launch_args = stream_session.additional_launch_args
stream_session_created_at = stream_session.created_at
stream_session_user_id = stream_session.user_id
```

---


### Stream_session_connection

StreamSessionConnection resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `signal_request` | String | ✅ | <p>A WebRTC ICE offer string to use when initializing a WebRTC connection. The offer is a very long JSON string. Provide the string as a text value in quotes. The offer must be newly generated, not the same offer provided to <code>StartStreamSession</code>. </p> |
| `stream_session_identifier` | String | ✅ | <p> <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference-arns.html">Amazon Resource Name (ARN)</a> or ID that uniquely identifies the stream session resource. Example ARN: <code>arn:aws:gameliftstreams:us-west-2:111122223333:streamsession/sg-1AB2C3De4/ABC123def4567</code>. Example ID: <code>ABC123def4567</code>. </p> <p> The stream session must be in <code>PENDING_CLIENT_RECONNECTION</code> or <code>ACTIVE</code> status. </p> |
| `client_token` | String |  | <p> A unique identifier that represents a client request. The request is idempotent, which ensures that an API request completes only once. When users send a request, Amazon GameLift Streams automatically populates this field. </p> |
| `identifier` | String | ✅ | <p> <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference-arns.html">Amazon Resource Name (ARN)</a> or ID that uniquely identifies the stream group resource. Example ARN: <code>arn:aws:gameliftstreams:us-west-2:111122223333:streamgroup/sg-1AB2C3De4</code>. Example ID: <code>sg-1AB2C3De4</code>. </p> <p> The stream group that you want to run this stream session with. The stream group must be in <code>ACTIVE</code> status. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create stream_session_connection
stream_session_connection = provider.gameliftstreams.Stream_session_connection {
    signal_request = "value"  # <p>A WebRTC ICE offer string to use when initializing a WebRTC connection. The offer is a very long JSON string. Provide the string as a text value in quotes. The offer must be newly generated, not the same offer provided to <code>StartStreamSession</code>. </p>
    stream_session_identifier = "value"  # <p> <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference-arns.html">Amazon Resource Name (ARN)</a> or ID that uniquely identifies the stream session resource. Example ARN: <code>arn:aws:gameliftstreams:us-west-2:111122223333:streamsession/sg-1AB2C3De4/ABC123def4567</code>. Example ID: <code>ABC123def4567</code>. </p> <p> The stream session must be in <code>PENDING_CLIENT_RECONNECTION</code> or <code>ACTIVE</code> status. </p>
    identifier = "value"  # <p> <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference-arns.html">Amazon Resource Name (ARN)</a> or ID that uniquely identifies the stream group resource. Example ARN: <code>arn:aws:gameliftstreams:us-west-2:111122223333:streamgroup/sg-1AB2C3De4</code>. Example ID: <code>sg-1AB2C3De4</code>. </p> <p> The stream group that you want to run this stream session with. The stream group must be in <code>ACTIVE</code> status. </p>
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

# Create multiple stream_session resources
stream_session_0 = provider.gameliftstreams.Stream_session {
}
stream_session_1 = provider.gameliftstreams.Stream_session {
}
stream_session_2 = provider.gameliftstreams.Stream_session {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    stream_session = provider.gameliftstreams.Stream_session {
    }
```

---

## Related Documentation

- [AWS Gameliftstreams Documentation](https://docs.aws.amazon.com/gameliftstreams/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
