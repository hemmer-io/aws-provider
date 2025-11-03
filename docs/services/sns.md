# Sns Service



**Resources**: 12

---

## Overview

The sns service provides access to 12 resource types:

- [Sms_sandbox_phone_number](#sms_sandbox_phone_number) [CD]
- [Sms_attributes](#sms_attributes) [R]
- [Endpoint_attributes](#endpoint_attributes) [R]
- [Topic](#topic) [CD]
- [Topic_attributes](#topic_attributes) [R]
- [Data_protection_policy](#data_protection_policy) [CR]
- [Subscription_attributes](#subscription_attributes) [R]
- [Platform_endpoint](#platform_endpoint) [C]
- [Platform_application_attributes](#platform_application_attributes) [R]
- [Endpoint](#endpoint) [D]
- [Sms_sandbox_account_status](#sms_sandbox_account_status) [R]
- [Platform_application](#platform_application) [CD]

---

## Resources


### Sms_sandbox_phone_number

SMSSandboxPhoneNumber resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `phone_number` | String | ✅ | <p>The destination phone number to verify. On verification, Amazon SNS adds this phone number
            to the list of verified phone numbers that you can send SMS messages to.</p> |
| `language_code` | String |  | <p>The language to use for sending the OTP. The default value is
            <code>en-US</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sms_sandbox_phone_number
sms_sandbox_phone_number = provider.sns.Sms_sandbox_phone_number {
    phone_number = "value"  # <p>The destination phone number to verify. On verification, Amazon SNS adds this phone number
            to the list of verified phone numbers that you can send SMS messages to.</p>
}

```

---


### Sms_attributes

SMSAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attributes` | String | <p>The SMS attribute names and their values.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sms_attributes outputs
sms_attributes_id = sms_attributes.id
sms_attributes_attributes = sms_attributes.attributes
```

---


### Endpoint_attributes

EndpointAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attributes` | String | <p>Attributes include the following:</p>
         <ul>
            <li>
               <p>
                  <code>CustomUserData</code> – arbitrary user data to associate with the
                    endpoint. Amazon SNS does not use this data. The data must be in UTF-8 format and
                    less than 2KB.</p>
            </li>
            <li>
               <p>
                  <code>Enabled</code> – flag that enables/disables delivery to the
                    endpoint. Amazon SNS will set this to false when a notification service indicates to
                    Amazon SNS that the endpoint is invalid. Users can set it back to true, typically
                    after updating Token.</p>
            </li>
            <li>
               <p>
                  <code>Token</code> – device token, also referred to as a registration id,
                    for an app and mobile device. This is returned from the notification service
                    when an app and mobile device are registered with the notification
                    service.</p>
               <note>
                  <p>The device token for the iOS platform is returned in lowercase.</p>
               </note>
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

# Access endpoint_attributes outputs
endpoint_attributes_id = endpoint_attributes.id
endpoint_attributes_attributes = endpoint_attributes.attributes
```

---


### Topic

Topic resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_protection_policy` | String |  | <p>The body of the policy document you want to use for this topic.</p>
         <p>You can only add one policy per topic.</p>
         <p>The policy must be in JSON string format.</p>
         <p>Length Constraints: Maximum length of 30,720.</p> |
| `tags` | Vec<String> |  | <p>The list of tags to add to a new topic.</p>
         <note>
            <p>To be able to tag a topic on creation, you must have the
                    <code>sns:CreateTopic</code> and <code>sns:TagResource</code>
                permissions.</p>
         </note> |
| `attributes` | HashMap<String, String> |  | <p>A map of attributes with their corresponding values.</p>
         <p>The following lists names, descriptions, and values of the special request parameters
            that the <code>CreateTopic</code> action uses:</p>
         <ul>
            <li>
               <p>
                  <code>DeliveryPolicy</code> – The policy that defines how Amazon SNS retries
                    failed deliveries to HTTP/S endpoints.</p>
            </li>
            <li>
               <p>
                  <code>DisplayName</code> – The display name to use for a topic with SMS
                    subscriptions.</p>
            </li>
            <li>
               <p>
                  <code>Policy</code> – The policy that defines who can access your
                    topic. By default, only the topic owner can publish or subscribe to the
                    topic.</p>
            </li>
            <li>
               <p>
                  <code>TracingConfig</code> – Tracing mode of an Amazon SNS topic. By default
                        <code>TracingConfig</code> is set to <code>PassThrough</code>, and the topic
                    passes through the tracing header it receives from an Amazon SNS publisher to its
                    subscriptions. If set to <code>Active</code>, Amazon SNS will vend X-Ray segment data
                    to topic owner account if the sampled flag in the tracing header is true. This
                    is only supported on standard topics.</p>
            </li>
            <li>
               <p>HTTP</p>
               <ul>
                  <li>
                     <p>
                        <code>HTTPSuccessFeedbackRoleArn</code> – Indicates successful
                            message delivery status for an Amazon SNS topic that is subscribed to an HTTP
                            endpoint. </p>
                  </li>
                  <li>
                     <p>
                        <code>HTTPSuccessFeedbackSampleRate</code> – Indicates
                            percentage of successful messages to sample for an Amazon SNS topic that is
                            subscribed to an HTTP endpoint.</p>
                  </li>
                  <li>
                     <p>
                        <code>HTTPFailureFeedbackRoleArn</code> – Indicates failed
                            message delivery status for an Amazon SNS topic that is subscribed to an HTTP
                            endpoint.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>Amazon Data Firehose</p>
               <ul>
                  <li>
                     <p>
                        <code>FirehoseSuccessFeedbackRoleArn</code> – Indicates
                            successful message delivery status for an Amazon SNS topic that is subscribed
                            to an Amazon Data Firehose endpoint.</p>
                  </li>
                  <li>
                     <p>
                        <code>FirehoseSuccessFeedbackSampleRate</code> – Indicates
                            percentage of successful messages to sample for an Amazon SNS topic that is
                            subscribed to an Amazon Data Firehose endpoint.</p>
                  </li>
                  <li>
                     <p>
                        <code>FirehoseFailureFeedbackRoleArn</code> – Indicates failed
                            message delivery status for an Amazon SNS topic that is subscribed to an
                            Amazon Data Firehose endpoint. </p>
                  </li>
               </ul>
            </li>
            <li>
               <p>Lambda</p>
               <ul>
                  <li>
                     <p>
                        <code>LambdaSuccessFeedbackRoleArn</code> – Indicates
                            successful message delivery status for an Amazon SNS topic that is subscribed
                            to an Lambda endpoint.</p>
                  </li>
                  <li>
                     <p>
                        <code>LambdaSuccessFeedbackSampleRate</code> – Indicates
                            percentage of successful messages to sample for an Amazon SNS topic that is
                            subscribed to an Lambda endpoint.</p>
                  </li>
                  <li>
                     <p>
                        <code>LambdaFailureFeedbackRoleArn</code> – Indicates failed
                            message delivery status for an Amazon SNS topic that is subscribed to an
                            Lambda endpoint. </p>
                  </li>
               </ul>
            </li>
            <li>
               <p>Platform application endpoint</p>
               <ul>
                  <li>
                     <p>
                        <code>ApplicationSuccessFeedbackRoleArn</code> – Indicates
                            successful message delivery status for an Amazon SNS topic that is subscribed
                            to a platform application endpoint.</p>
                  </li>
                  <li>
                     <p>
                        <code>ApplicationSuccessFeedbackSampleRate</code> – Indicates
                            percentage of successful messages to sample for an Amazon SNS topic that is
                            subscribed to an platform application endpoint.</p>
                  </li>
                  <li>
                     <p>
                        <code>ApplicationFailureFeedbackRoleArn</code> – Indicates
                            failed message delivery status for an Amazon SNS topic that is subscribed to
                            an platform application endpoint.</p>
                  </li>
               </ul>
               <note>
                  <p>In addition to being able to configure topic attributes for message
                        delivery status of notification messages sent to Amazon SNS application
                        endpoints, you can also configure application attributes for the delivery
                        status of push notification messages sent to push notification
                        services.</p>
                  <p>For example, For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-msg-status.html">Using Amazon SNS Application
                            Attributes for Message Delivery Status</a>. </p>
               </note>
            </li>
            <li>
               <p>Amazon SQS</p>
               <ul>
                  <li>
                     <p>
                        <code>SQSSuccessFeedbackRoleArn</code> – Indicates successful
                            message delivery status for an Amazon SNS topic that is subscribed to an
                            Amazon SQS endpoint. </p>
                  </li>
                  <li>
                     <p>
                        <code>SQSSuccessFeedbackSampleRate</code> – Indicates
                            percentage of successful messages to sample for an Amazon SNS topic that is
                            subscribed to an Amazon SQS endpoint. </p>
                  </li>
                  <li>
                     <p>
                        <code>SQSFailureFeedbackRoleArn</code> – Indicates failed
                            message delivery status for an Amazon SNS topic that is subscribed to an
                            Amazon SQS endpoint. </p>
                  </li>
               </ul>
            </li>
         </ul>
         <note>
            <p>The <ENDPOINT>SuccessFeedbackRoleArn and <ENDPOINT>FailureFeedbackRoleArn
                attributes are used to give Amazon SNS write access to use CloudWatch Logs on your
                behalf. The <ENDPOINT>SuccessFeedbackSampleRate attribute is for specifying the
                sample rate percentage (0-100) of successfully delivered messages. After you
                configure the <ENDPOINT>FailureFeedbackRoleArn attribute, then all failed message
                deliveries generate CloudWatch Logs. </p>
         </note>
         <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html">server-side
            encryption</a>:</p>
         <ul>
            <li>
               <p>
                  <code>KmsMasterKeyId</code> – The ID of an Amazon Web Services managed customer master
                    key (CMK) for Amazon SNS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms">Key
                        Terms</a>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>Key Management Service API Reference</i>. </p>
            </li>
         </ul>
         <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">FIFO topics</a>:</p>
         <ul>
            <li>
               <p>
                  <code>ArchivePolicy</code> – The policy that sets the retention period
                    for messages stored in the message archive of an Amazon SNS FIFO
                    topic.</p>
            </li>
            <li>
               <p>
                  <code>ContentBasedDeduplication</code> – Enables content-based
                    deduplication for FIFO topics.</p>
               <ul>
                  <li>
                     <p>By default, <code>ContentBasedDeduplication</code> is set to
                                <code>false</code>. If you create a FIFO topic and this attribute is
                                <code>false</code>, you must specify a value for the
                                <code>MessageDeduplicationId</code> parameter for the <a href="https://docs.aws.amazon.com/sns/latest/api/API_Publish.html">Publish</a> action. </p>
                  </li>
                  <li>
                     <p>When you set <code>ContentBasedDeduplication</code> to
                                <code>true</code>, Amazon SNS uses a SHA-256 hash to
                            generate the <code>MessageDeduplicationId</code> using the body of the
                            message (but not the attributes of the message).</p>
                     <p>(Optional) To override the generated value, you can specify a value
                            for the <code>MessageDeduplicationId</code> parameter for the
                                <code>Publish</code> action.</p>
                  </li>
               </ul>
            </li>
         </ul>
         <ul>
            <li>
               <p>
                  <code>FifoThroughputScope</code> – Enables higher throughput for your FIFO topic by adjusting the scope of deduplication. This attribute has two possible values:</p>
               <ul>
                  <li>
                     <p>
                        <code>Topic</code> – The scope of message deduplication is across the entire topic. This is the default value and maintains existing behavior, with a maximum throughput of 3000 messages per second or 20MB per second, whichever comes first.</p>
                  </li>
                  <li>
                     <p>
                        <code>MessageGroup</code> – The scope of deduplication is within each individual message group, which enables higher throughput per topic subject to regional quotas. For more information on quotas or to request an increase, see <a href="https://docs.aws.amazon.com/general/latest/gr/sns.html">Amazon SNS service quotas</a> in the Amazon Web Services General Reference.</p>
                  </li>
               </ul>
            </li>
         </ul> |
| `name` | String | ✅ | <p>The name of the topic you want to create.</p>
         <p>Constraints: Topic names must be made up of only uppercase and lowercase ASCII
            letters, numbers, underscores, and hyphens, and must be between 1 and 256 characters
            long.</p>
         <p>For a FIFO (first-in-first-out) topic, the name must end with the <code>.fifo</code>
            suffix. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create topic
topic = provider.sns.Topic {
    name = "value"  # <p>The name of the topic you want to create.</p>
         <p>Constraints: Topic names must be made up of only uppercase and lowercase ASCII
            letters, numbers, underscores, and hyphens, and must be between 1 and 256 characters
            long.</p>
         <p>For a FIFO (first-in-first-out) topic, the name must end with the <code>.fifo</code>
            suffix. </p>
}

```

---


### Topic_attributes

TopicAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attributes` | HashMap<String, String> | <p>A map of the topic's attributes. Attributes in this map include the following:</p>
         <ul>
            <li>
               <p>
                  <code>DeliveryPolicy</code> – The JSON serialization of the topic's
                    delivery policy.</p>
            </li>
            <li>
               <p>
                  <code>DisplayName</code> – The human-readable name used in the
                        <code>From</code> field for notifications to <code>email</code> and
                        <code>email-json</code> endpoints.</p>
            </li>
            <li>
               <p>
                  <code>EffectiveDeliveryPolicy</code> – The JSON serialization of the
                    effective delivery policy, taking system defaults into account.</p>
            </li>
            <li>
               <p>
                  <code>Owner</code> – The Amazon Web Services account ID of the topic's owner.</p>
            </li>
            <li>
               <p>
                  <code>Policy</code> – The JSON serialization of the topic's access
                    control policy.</p>
            </li>
            <li>
               <p>
                  <code>SignatureVersion</code> – The signature version corresponds to
                    the hashing algorithm used while creating the signature of the notifications,
                    subscription confirmations, or unsubscribe confirmation messages sent by
                    Amazon SNS.</p>
               <ul>
                  <li>
                     <p>By default, <code>SignatureVersion</code> is set to <b>1</b>. The signature is a Base64-encoded
                                <b>SHA1withRSA</b> signature.</p>
                  </li>
                  <li>
                     <p>When you set <code>SignatureVersion</code> to <b>2</b>. Amazon SNS uses a Base64-encoded <b>SHA256withRSA</b> signature. </p>
                     <note>
                        <p>If the API response does not include the
                                    <code>SignatureVersion</code> attribute, it means that the
                                    <code>SignatureVersion</code> for the topic has value <b>1</b>.</p>
                     </note>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <code>SubscriptionsConfirmed</code> – The number of confirmed
                    subscriptions for the topic.</p>
            </li>
            <li>
               <p>
                  <code>SubscriptionsDeleted</code> – The number of deleted subscriptions
                    for the topic.</p>
            </li>
            <li>
               <p>
                  <code>SubscriptionsPending</code> – The number of subscriptions pending
                    confirmation for the topic.</p>
            </li>
            <li>
               <p>
                  <code>TopicArn</code> – The topic's ARN.</p>
            </li>
            <li>
               <p>
                  <code>TracingConfig</code> – Tracing mode of an Amazon SNS topic. By default
                        <code>TracingConfig</code> is set to <code>PassThrough</code>, and the topic
                    passes through the tracing header it receives from an Amazon SNS publisher to its
                    subscriptions. If set to <code>Active</code>, Amazon SNS will vend X-Ray segment data
                    to topic owner account if the sampled flag in the tracing header is true. This
                    is only supported on standard topics.</p>
            </li>
         </ul>
         <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html">server-side-encryption</a>:</p>
         <ul>
            <li>
               <p>
                  <code>KmsMasterKeyId</code> - The ID of an Amazon Web Services managed customer master key
                    (CMK) for Amazon SNS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms">Key
                        Terms</a>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>Key Management Service API Reference</i>.</p>
            </li>
         </ul>
         <p>The following attributes apply only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-fifo-topics.html">FIFO topics</a>:</p>
         <ul>
            <li>
               <p>
                  <code>ArchivePolicy</code> – The policy that sets the retention period
                    for messages stored in the message archive of an Amazon SNS FIFO
                    topic.</p>
            </li>
            <li>
               <p>
                  <code>BeginningArchiveTime</code> – The earliest starting point at
                    which a message in the topic’s archive can be replayed from. This point in time
                    is based on the configured message retention period set by the topic’s message
                    archiving policy.</p>
            </li>
            <li>
               <p>
                  <code>ContentBasedDeduplication</code> – Enables content-based
                    deduplication for FIFO topics.</p>
               <ul>
                  <li>
                     <p>By default, <code>ContentBasedDeduplication</code> is set to
                                <code>false</code>. If you create a FIFO topic and this attribute is
                                <code>false</code>, you must specify a value for the
                                <code>MessageDeduplicationId</code> parameter for the <a href="https://docs.aws.amazon.com/sns/latest/api/API_Publish.html">Publish</a> action. </p>
                  </li>
                  <li>
                     <p>When you set <code>ContentBasedDeduplication</code> to
                                <code>true</code>, Amazon SNS uses a SHA-256 hash to
                            generate the <code>MessageDeduplicationId</code> using the body of the
                            message (but not the attributes of the message).</p>
                     <p>(Optional) To override the generated value, you can specify a value
                            for the <code>MessageDeduplicationId</code> parameter for the
                                <code>Publish</code> action.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <code>FifoTopic</code> – When this is set to <code>true</code>, a FIFO
                topic is created.</p>
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

# Access topic_attributes outputs
topic_attributes_id = topic_attributes.id
topic_attributes_attributes = topic_attributes.attributes
```

---


### Data_protection_policy

DataProtectionPolicy resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String | ✅ | <p>The ARN of the topic whose <code>DataProtectionPolicy</code> you want to add or
            update.</p>
         <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names
                (ARNs)</a> in the Amazon Web Services General Reference.</p> |
| `data_protection_policy` | String | ✅ | <p>The JSON serialization of the topic's <code>DataProtectionPolicy</code>.</p>
         <p>The <code>DataProtectionPolicy</code> must be in JSON string format.</p>
         <p>Length Constraints: Maximum length of 30,720.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_protection_policy` | String | <p>Retrieves the <code>DataProtectionPolicy</code> in JSON string format.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_protection_policy
data_protection_policy = provider.sns.Data_protection_policy {
    resource_arn = "value"  # <p>The ARN of the topic whose <code>DataProtectionPolicy</code> you want to add or
            update.</p>
         <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names
                (ARNs)</a> in the Amazon Web Services General Reference.</p>
    data_protection_policy = "value"  # <p>The JSON serialization of the topic's <code>DataProtectionPolicy</code>.</p>
         <p>The <code>DataProtectionPolicy</code> must be in JSON string format.</p>
         <p>Length Constraints: Maximum length of 30,720.</p>
}

# Access data_protection_policy outputs
data_protection_policy_id = data_protection_policy.id
data_protection_policy_data_protection_policy = data_protection_policy.data_protection_policy
```

---


### Subscription_attributes

SubscriptionAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attributes` | HashMap<String, String> | <p>A map of the subscription's attributes. Attributes in this map include the
            following:</p>
         <ul>
            <li>
               <p>
                  <code>ConfirmationWasAuthenticated</code> – <code>true</code> if the
                    subscription confirmation request was authenticated.</p>
            </li>
            <li>
               <p>
                  <code>DeliveryPolicy</code> – The JSON serialization of the
                    subscription's delivery policy.</p>
            </li>
            <li>
               <p>
                  <code>EffectiveDeliveryPolicy</code> – The JSON serialization of the
                    effective delivery policy that takes into account the topic delivery policy and
                    account system defaults.</p>
            </li>
            <li>
               <p>
                  <code>FilterPolicy</code> – The filter policy JSON that is assigned to
                    the subscription. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-message-filtering.html">Amazon SNS Message
                        Filtering</a> in the <i>Amazon SNS Developer Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>FilterPolicyScope</code> – This attribute lets you choose the
                    filtering scope by using one of the following string value types:</p>
               <ul>
                  <li>
                     <p>
                        <code>MessageAttributes</code> (default) – The filter is
                            applied on the message attributes.</p>
                  </li>
                  <li>
                     <p>
                        <code>MessageBody</code> – The filter is applied on the message
                            body.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <code>Owner</code> – The Amazon Web Services account ID of the subscription's
                    owner.</p>
            </li>
            <li>
               <p>
                  <code>PendingConfirmation</code> – <code>true</code> if the subscription
                    hasn't been confirmed. To confirm a pending subscription, call the
                        <code>ConfirmSubscription</code> action with a confirmation token.</p>
            </li>
            <li>
               <p>
                  <code>RawMessageDelivery</code> – <code>true</code> if raw message
                    delivery is enabled for the subscription. Raw messages are free of JSON
                    formatting and can be sent to HTTP/S and Amazon SQS endpoints.</p>
            </li>
            <li>
               <p>
                  <code>RedrivePolicy</code> – When specified, sends undeliverable messages to the specified Amazon SQS dead-letter queue. 
    Messages that can't be delivered due to client errors (for example, when the subscribed endpoint is unreachable)
    or server errors (for example, when the service that powers the subscribed endpoint becomes unavailable) are held 
    in the dead-letter queue for further analysis or reprocessing.</p>
            </li>
            <li>
               <p>
                  <code>SubscriptionArn</code> – The subscription's ARN.</p>
            </li>
            <li>
               <p>
                  <code>TopicArn</code> – The topic ARN that the subscription is associated
                    with.</p>
            </li>
         </ul>
         <p>The following attribute applies only to Amazon Data Firehose delivery stream subscriptions:</p>
         <ul>
            <li>
               <p>
                  <code>SubscriptionRoleArn</code> – The ARN of the IAM role that has the following:</p>
               <ul>
                  <li>
                     <p>Permission to write to the Firehose delivery stream</p>
                  </li>
                  <li>
                     <p>Amazon SNS listed as a trusted entity</p>
                  </li>
               </ul>
               <p>Specifying a valid ARN for this attribute is required for Firehose delivery stream subscriptions. 
                For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-firehose-as-subscriber.html">Fanout 
                    to Firehose delivery streams</a> in the <i>Amazon SNS Developer Guide</i>.</p>
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

# Access subscription_attributes outputs
subscription_attributes_id = subscription_attributes.id
subscription_attributes_attributes = subscription_attributes.attributes
```

---


### Platform_endpoint

PlatformEndpoint resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attributes` | String |  | <p>For a list of attributes, see <a href="https://docs.aws.amazon.com/sns/latest/api/API_SetEndpointAttributes.html">
               <code>SetEndpointAttributes</code>
            </a>.</p> |
| `custom_user_data` | String |  | <p>Arbitrary user data to associate with the endpoint. Amazon SNS does not use this data. The
            data must be in UTF-8 format and less than 2KB.</p> |
| `token` | String | ✅ | <p>Unique identifier created by the notification service for an app on a device. The
            specific name for Token will vary, depending on which notification service is being
            used. For example, when using APNS as the notification service, you need the device
            token. Alternatively, when using GCM (Firebase Cloud Messaging) or ADM, the device token
            equivalent is called the registration ID.</p> |
| `platform_application_arn` | String | ✅ | <p>
            <code>PlatformApplicationArn</code> returned from CreatePlatformApplication is used to
            create a an endpoint.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create platform_endpoint
platform_endpoint = provider.sns.Platform_endpoint {
    token = "value"  # <p>Unique identifier created by the notification service for an app on a device. The
            specific name for Token will vary, depending on which notification service is being
            used. For example, when using APNS as the notification service, you need the device
            token. Alternatively, when using GCM (Firebase Cloud Messaging) or ADM, the device token
            equivalent is called the registration ID.</p>
    platform_application_arn = "value"  # <p>
            <code>PlatformApplicationArn</code> returned from CreatePlatformApplication is used to
            create a an endpoint.</p>
}

```

---


### Platform_application_attributes

PlatformApplicationAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attributes` | String | <p>Attributes include the following:</p>
         <ul>
            <li>
               <p>
                  <code>AppleCertificateExpiryDate</code> – The expiry date of the SSL
                    certificate used to configure certificate-based authentication.</p>
            </li>
            <li>
               <p>
                  <code>ApplePlatformTeamID</code> – The Apple developer account ID used to
                    configure token-based authentication.</p>
            </li>
            <li>
               <p>
                  <code>ApplePlatformBundleID</code> – The app identifier used to configure
                    token-based authentication.</p>
            </li>
            <li>
               <p>
                  <code>AuthenticationMethod</code> – Returns the credential type used when
                    sending push notifications from application to APNS/APNS_Sandbox, or application
                    to GCM.</p>
               <ul>
                  <li>
                     <p>APNS – Returns the token or certificate.</p>
                  </li>
                  <li>
                     <p>GCM – Returns the token or key.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <code>EventEndpointCreated</code> – Topic ARN to which EndpointCreated
                    event notifications should be sent.</p>
            </li>
            <li>
               <p>
                  <code>EventEndpointDeleted</code> – Topic ARN to which EndpointDeleted
                    event notifications should be sent.</p>
            </li>
            <li>
               <p>
                  <code>EventEndpointUpdated</code> – Topic ARN to which EndpointUpdate
                    event notifications should be sent.</p>
            </li>
            <li>
               <p>
                  <code>EventDeliveryFailure</code> – Topic ARN to which DeliveryFailure
                    event notifications should be sent upon Direct Publish delivery failure
                    (permanent) to one of the application's endpoints.</p>
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

# Access platform_application_attributes outputs
platform_application_attributes_id = platform_application_attributes.id
platform_application_attributes_attributes = platform_application_attributes.attributes
```

---


### Endpoint

Endpoint resource

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


### Sms_sandbox_account_status

SMSSandboxAccountStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `is_in_sandbox` | bool | <p>Indicates whether the calling Amazon Web Services account is in the SMS sandbox.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sms_sandbox_account_status outputs
sms_sandbox_account_status_id = sms_sandbox_account_status.id
sms_sandbox_account_status_is_in_sandbox = sms_sandbox_account_status.is_in_sandbox
```

---


### Platform_application

PlatformApplication resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>Application names must be made up of only uppercase and lowercase ASCII letters,
            numbers, underscores, hyphens, and periods, and must be between 1 and 256 characters
            long.</p> |
| `platform` | String | ✅ | <p>The following platforms are supported: ADM (Amazon Device Messaging), APNS (Apple Push
            Notification Service), APNS_SANDBOX, and GCM (Firebase Cloud Messaging).</p> |
| `attributes` | String | ✅ | <p>For a list of attributes, see <a href="https://docs.aws.amazon.com/sns/latest/api/API_SetPlatformApplicationAttributes.html">
               <code>SetPlatformApplicationAttributes</code>
            </a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create platform_application
platform_application = provider.sns.Platform_application {
    name = "value"  # <p>Application names must be made up of only uppercase and lowercase ASCII letters,
            numbers, underscores, hyphens, and periods, and must be between 1 and 256 characters
            long.</p>
    platform = "value"  # <p>The following platforms are supported: ADM (Amazon Device Messaging), APNS (Apple Push
            Notification Service), APNS_SANDBOX, and GCM (Firebase Cloud Messaging).</p>
    attributes = "value"  # <p>For a list of attributes, see <a href="https://docs.aws.amazon.com/sns/latest/api/API_SetPlatformApplicationAttributes.html">
               <code>SetPlatformApplicationAttributes</code>
            </a>.</p>
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

# Create multiple sms_sandbox_phone_number resources
sms_sandbox_phone_number_0 = provider.sns.Sms_sandbox_phone_number {
    phone_number = "value-0"
}
sms_sandbox_phone_number_1 = provider.sns.Sms_sandbox_phone_number {
    phone_number = "value-1"
}
sms_sandbox_phone_number_2 = provider.sns.Sms_sandbox_phone_number {
    phone_number = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    sms_sandbox_phone_number = provider.sns.Sms_sandbox_phone_number {
        phone_number = "production-value"
    }
```

---

## Related Documentation

- [AWS Sns Documentation](https://docs.aws.amazon.com/sns/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
