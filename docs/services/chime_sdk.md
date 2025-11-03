# Chime_sdk Service



**Resources**: 25

---

## Overview

The chime_sdk service provides access to 25 resource types:

- [Voice_connector_origination](#voice_connector_origination) [CRD]
- [Speaker_search_task](#speaker_search_task) [R]
- [Voice_connector_emergency_calling_configuration](#voice_connector_emergency_calling_configuration) [CRD]
- [Voice_connector_termination_health](#voice_connector_termination_health) [R]
- [Voice_connector](#voice_connector) [CRUD]
- [Phone_number_order](#phone_number_order) [CR]
- [Voice_connector_logging_configuration](#voice_connector_logging_configuration) [CR]
- [Sip_media_application_call](#sip_media_application_call) [CU]
- [Voice_connector_termination_credentials](#voice_connector_termination_credentials) [CD]
- [Phone_number_settings](#phone_number_settings) [RU]
- [Voice_connector_proxy](#voice_connector_proxy) [CRD]
- [Voice_connector_group](#voice_connector_group) [CRUD]
- [Sip_media_application](#sip_media_application) [CRUD]
- [Sip_rule](#sip_rule) [CRUD]
- [Voice_connector_streaming_configuration](#voice_connector_streaming_configuration) [CRD]
- [Voice_profile](#voice_profile) [CRUD]
- [Proxy_session](#proxy_session) [CRUD]
- [Voice_profile_domain](#voice_profile_domain) [CRUD]
- [Voice_connector_external_systems_configuration](#voice_connector_external_systems_configuration) [CRD]
- [Voice_connector_termination](#voice_connector_termination) [CRD]
- [Sip_media_application_logging_configuration](#sip_media_application_logging_configuration) [CR]
- [Phone_number](#phone_number) [RUD]
- [Global_settings](#global_settings) [RU]
- [Voice_tone_analysis_task](#voice_tone_analysis_task) [R]
- [Sip_media_application_alexa_skill_configuration](#sip_media_application_alexa_skill_configuration) [CR]

---

## Resources


### Voice_connector_origination

VoiceConnectorOrigination resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `voice_connector_id` | String | ✅ | <p>The Voice Connector ID.</p> |
| `origination` | String | ✅ | <p>The origination settings being updated.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `origination` | String | <p>The origination setting details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create voice_connector_origination
voice_connector_origination = provider.chime_sdk.Voice_connector_origination {
    voice_connector_id = "value"  # <p>The Voice Connector ID.</p>
    origination = "value"  # <p>The origination settings being updated.</p>
}

# Access voice_connector_origination outputs
voice_connector_origination_id = voice_connector_origination.id
voice_connector_origination_origination = voice_connector_origination.origination
```

---


### Speaker_search_task

SpeakerSearchTask resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `speaker_search_task` | String | <p>The details of the speaker search task.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access speaker_search_task outputs
speaker_search_task_id = speaker_search_task.id
speaker_search_task_speaker_search_task = speaker_search_task.speaker_search_task
```

---


### Voice_connector_emergency_calling_configuration

VoiceConnectorEmergencyCallingConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `emergency_calling_configuration` | String | ✅ | <p>The configuration being updated.</p> |
| `voice_connector_id` | String | ✅ | <p>The Voice Connector ID.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `emergency_calling_configuration` | String | <p>The details of the emergency calling configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create voice_connector_emergency_calling_configuration
voice_connector_emergency_calling_configuration = provider.chime_sdk.Voice_connector_emergency_calling_configuration {
    emergency_calling_configuration = "value"  # <p>The configuration being updated.</p>
    voice_connector_id = "value"  # <p>The Voice Connector ID.</p>
}

# Access voice_connector_emergency_calling_configuration outputs
voice_connector_emergency_calling_configuration_id = voice_connector_emergency_calling_configuration.id
voice_connector_emergency_calling_configuration_emergency_calling_configuration = voice_connector_emergency_calling_configuration.emergency_calling_configuration
```

---


### Voice_connector_termination_health

VoiceConnectorTerminationHealth resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `termination_health` | String | <p>The termination health details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access voice_connector_termination_health outputs
voice_connector_termination_health_id = voice_connector_termination_health.id
voice_connector_termination_health_termination_health = voice_connector_termination_health.termination_health
```

---


### Voice_connector

VoiceConnector resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the Voice Connector.</p> |
| `integration_type` | String |  | <p>The connectors for use with Amazon Connect.</p>
         <p>The following options are available:</p>
         <ul>
            <li>
               <p>
                  <code>CONNECT_CALL_TRANSFER_CONNECTOR</code> - Enables enterprises to integrate
               Amazon Connect with other voice systems to directly transfer voice calls and
               metadata without using the public telephone network. They can use Amazon Connect
               telephony and Interactive Voice Response (IVR) with their existing voice systems to
               modernize the IVR experience of their existing contact center and their enterprise
               and branch voice systems. Additionally, enterprises migrating their contact center to
               Amazon Connect can start with Connect telephony and IVR for immediate
               modernization ahead of agent migration.</p>
            </li>
            <li>
               <p>
                  <code>CONNECT_ANALYTICS_CONNECTOR</code> - Enables enterprises to integrate
               Amazon Connect with other voice systems for real-time and post-call analytics.
               They can use Amazon Connect Contact Lens with their existing voice systems to
               provides call recordings, conversational analytics (including contact transcript,
               sensitive data redaction, content categorization, theme detection, sentiment
               analysis, real-time alerts, and post-contact summary), and agent performance
               evaluations (including evaluation forms, automated evaluation, supervisor review)
               with a rich user experience to display, search and filter customer interactions, and
               programmatic access to data streams and the data lake. Additionally, enterprises
               migrating their contact center to Amazon Connect can start with Contact Lens
               analytics and performance insights ahead of agent migration.</p>
            </li>
         </ul> |
| `aws_region` | String |  | <p>The AWS Region in which the Amazon Chime SDK Voice Connector is created. Default value: 
         <code>us-east-1</code> .</p> |
| `network_type` | String |  | <p>The type of network for the Voice Connector. Either IPv4 only or dual-stack (IPv4 and IPv6).</p> |
| `require_encryption` | bool | ✅ | <p>Enables or disables encryption for the Voice Connector.</p> |
| `tags` | Vec<String> |  | <p>The tags assigned to the Voice Connector.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `voice_connector` | String | <p>The Voice Connector details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create voice_connector
voice_connector = provider.chime_sdk.Voice_connector {
    name = "value"  # <p>The name of the Voice Connector.</p>
    require_encryption = "value"  # <p>Enables or disables encryption for the Voice Connector.</p>
}

# Access voice_connector outputs
voice_connector_id = voice_connector.id
voice_connector_voice_connector = voice_connector.voice_connector
```

---


### Phone_number_order

PhoneNumberOrder resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | <p>Specifies the name assigned to one or more phone numbers.</p> |
| `product_type` | String | ✅ | <p>The phone number product type.</p> |
| `e164_phone_numbers` | Vec<String> | ✅ | <p>List of phone numbers, in E.164 format.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `phone_number_order` | String | <p>The phone number order details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create phone_number_order
phone_number_order = provider.chime_sdk.Phone_number_order {
    product_type = "value"  # <p>The phone number product type.</p>
    e164_phone_numbers = "value"  # <p>List of phone numbers, in E.164 format.</p>
}

# Access phone_number_order outputs
phone_number_order_id = phone_number_order.id
phone_number_order_phone_number_order = phone_number_order.phone_number_order
```

---


### Voice_connector_logging_configuration

VoiceConnectorLoggingConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `voice_connector_id` | String | ✅ | <p>The Voice Connector ID.</p> |
| `logging_configuration` | String | ✅ | <p>The logging configuration being updated.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `logging_configuration` | String | <p>The logging configuration details .</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create voice_connector_logging_configuration
voice_connector_logging_configuration = provider.chime_sdk.Voice_connector_logging_configuration {
    voice_connector_id = "value"  # <p>The Voice Connector ID.</p>
    logging_configuration = "value"  # <p>The logging configuration being updated.</p>
}

# Access voice_connector_logging_configuration outputs
voice_connector_logging_configuration_id = voice_connector_logging_configuration.id
voice_connector_logging_configuration_logging_configuration = voice_connector_logging_configuration.logging_configuration
```

---


### Sip_media_application_call

SipMediaApplicationCall resource

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `from_phone_number` | String | ✅ | <p>The phone number that a user calls from. This is a phone number in your 
         Amazon Chime SDK phone number inventory.</p> |
| `arguments_map` | HashMap<String, String> |  | <p>Context passed to a CreateSipMediaApplication API call. For example, you could pass
            key-value pairs such as: <code>"FirstName": "John", "LastName": "Doe"</code>
         </p> |
| `to_phone_number` | String | ✅ | <p>The phone number that the service should call.</p> |
| `sip_headers` | HashMap<String, String> |  | <p>The SIP headers added to an outbound call leg.</p> |
| `sip_media_application_id` | String | ✅ | <p>The ID of the SIP media application.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sip_media_application_call
sip_media_application_call = provider.chime_sdk.Sip_media_application_call {
    from_phone_number = "value"  # <p>The phone number that a user calls from. This is a phone number in your 
         Amazon Chime SDK phone number inventory.</p>
    to_phone_number = "value"  # <p>The phone number that the service should call.</p>
    sip_media_application_id = "value"  # <p>The ID of the SIP media application.</p>
}

```

---


### Voice_connector_termination_credentials

VoiceConnectorTerminationCredentials resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `voice_connector_id` | String | ✅ | <p>The Voice Connector ID.</p> |
| `credentials` | Vec<String> |  | <p>The termination credentials being updated.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create voice_connector_termination_credentials
voice_connector_termination_credentials = provider.chime_sdk.Voice_connector_termination_credentials {
    voice_connector_id = "value"  # <p>The Voice Connector ID.</p>
}

```

---


### Phone_number_settings

PhoneNumberSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `calling_name` | String | ✅ | <p>The default outbound calling name for the account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `calling_name` | String | <p>The default outbound calling name for the account.</p> |
| `calling_name_updated_timestamp` | String | <p>The updated outbound calling name timestamp, in ISO 8601 format.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access phone_number_settings outputs
phone_number_settings_id = phone_number_settings.id
phone_number_settings_calling_name = phone_number_settings.calling_name
phone_number_settings_calling_name_updated_timestamp = phone_number_settings.calling_name_updated_timestamp
```

---


### Voice_connector_proxy

VoiceConnectorProxy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fall_back_phone_number` | String |  | <p>The phone number to route calls to after a proxy session expires.</p> |
| `phone_number_pool_countries` | Vec<String> | ✅ | <p>The countries for proxy phone numbers to be selected from.</p> |
| `voice_connector_id` | String | ✅ | <p>The Voice Connector ID.</p> |
| `default_session_expiry_minutes` | i64 | ✅ | <p>The default number of minutes allowed for proxy session.</p> |
| `disabled` | bool |  | <p>When true, stops proxy sessions from being created on the specified Amazon Chime SDK Voice Connector.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `proxy` | String | <p>The proxy configuration details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create voice_connector_proxy
voice_connector_proxy = provider.chime_sdk.Voice_connector_proxy {
    phone_number_pool_countries = "value"  # <p>The countries for proxy phone numbers to be selected from.</p>
    voice_connector_id = "value"  # <p>The Voice Connector ID.</p>
    default_session_expiry_minutes = "value"  # <p>The default number of minutes allowed for proxy session.</p>
}

# Access voice_connector_proxy outputs
voice_connector_proxy_id = voice_connector_proxy.id
voice_connector_proxy_proxy = voice_connector_proxy.proxy
```

---


### Voice_connector_group

VoiceConnectorGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the Voice Connector group.</p> |
| `voice_connector_items` | Vec<String> |  | <p>Lists the Voice Connectors that inbound calls are routed to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `voice_connector_group` | String | <p>The details of the Voice Connector group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create voice_connector_group
voice_connector_group = provider.chime_sdk.Voice_connector_group {
    name = "value"  # <p>The name of the Voice Connector group.</p>
}

# Access voice_connector_group outputs
voice_connector_group_id = voice_connector_group.id
voice_connector_group_voice_connector_group = voice_connector_group.voice_connector_group
```

---


### Sip_media_application

SipMediaApplication resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_region` | String | ✅ | <p>The AWS Region assigned to the SIP media application.</p> |
| `name` | String | ✅ | <p>The SIP media application's name.</p> |
| `endpoints` | Vec<String> | ✅ | <p>List of endpoints (Lambda ARNs) specified for the SIP media application.</p> |
| `tags` | Vec<String> |  | <p>The tags assigned to the SIP media application.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sip_media_application` | String | <p>The details of the SIP media application.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sip_media_application
sip_media_application = provider.chime_sdk.Sip_media_application {
    aws_region = "value"  # <p>The AWS Region assigned to the SIP media application.</p>
    name = "value"  # <p>The SIP media application's name.</p>
    endpoints = "value"  # <p>List of endpoints (Lambda ARNs) specified for the SIP media application.</p>
}

# Access sip_media_application outputs
sip_media_application_id = sip_media_application.id
sip_media_application_sip_media_application = sip_media_application.sip_media_application
```

---


### Sip_rule

SipRule resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the SIP rule.</p> |
| `trigger_value` | String | ✅ | <p>If <code>TriggerType</code> is <code>RequestUriHostname</code>, the 
         value can be the outbound host name of a Voice Connector. If 
         <code>TriggerType</code> is <code>ToPhoneNumber</code>, the value can 
         be a customer-owned phone number in the E164 format. The 
         <code>SipMediaApplication</code> specified in the <code>SipRule</code> is triggered 
         if the request URI in an incoming SIP request matches the 
         <code>RequestUriHostname</code>, or if the <code>To</code> header in the 
         incoming SIP request matches the <code>ToPhoneNumber</code> value.</p> |
| `target_applications` | Vec<String> |  | <p>List of SIP media applications, with priority and AWS Region. Only one SIP 
         application per AWS Region can be used.</p> |
| `disabled` | bool |  | <p>Disables or enables a SIP rule. You must disable SIP rules 
         before you can delete them.</p> |
| `trigger_type` | String | ✅ | <p>The type of trigger assigned to the SIP rule in <code>TriggerValue</code>, 
         currently <code>RequestUriHostname</code> or <code>ToPhoneNumber</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sip_rule` | String | <p>The SIP rule details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sip_rule
sip_rule = provider.chime_sdk.Sip_rule {
    name = "value"  # <p>The name of the SIP rule.</p>
    trigger_value = "value"  # <p>If <code>TriggerType</code> is <code>RequestUriHostname</code>, the 
         value can be the outbound host name of a Voice Connector. If 
         <code>TriggerType</code> is <code>ToPhoneNumber</code>, the value can 
         be a customer-owned phone number in the E164 format. The 
         <code>SipMediaApplication</code> specified in the <code>SipRule</code> is triggered 
         if the request URI in an incoming SIP request matches the 
         <code>RequestUriHostname</code>, or if the <code>To</code> header in the 
         incoming SIP request matches the <code>ToPhoneNumber</code> value.</p>
    trigger_type = "value"  # <p>The type of trigger assigned to the SIP rule in <code>TriggerValue</code>, 
         currently <code>RequestUriHostname</code> or <code>ToPhoneNumber</code>.</p>
}

# Access sip_rule outputs
sip_rule_id = sip_rule.id
sip_rule_sip_rule = sip_rule.sip_rule
```

---


### Voice_connector_streaming_configuration

VoiceConnectorStreamingConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `voice_connector_id` | String | ✅ | <p>The Voice Connector ID.</p> |
| `streaming_configuration` | String | ✅ | <p>The streaming settings being updated.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `streaming_configuration` | String | <p>The details of the streaming configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create voice_connector_streaming_configuration
voice_connector_streaming_configuration = provider.chime_sdk.Voice_connector_streaming_configuration {
    voice_connector_id = "value"  # <p>The Voice Connector ID.</p>
    streaming_configuration = "value"  # <p>The streaming settings being updated.</p>
}

# Access voice_connector_streaming_configuration outputs
voice_connector_streaming_configuration_id = voice_connector_streaming_configuration.id
voice_connector_streaming_configuration_streaming_configuration = voice_connector_streaming_configuration.streaming_configuration
```

---


### Voice_profile

VoiceProfile resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `speaker_search_task_id` | String | ✅ | <p>The ID of the speaker search task.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `voice_profile` | String | <p>The voice profile details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create voice_profile
voice_profile = provider.chime_sdk.Voice_profile {
    speaker_search_task_id = "value"  # <p>The ID of the speaker search task.</p>
}

# Access voice_profile outputs
voice_profile_id = voice_profile.id
voice_profile_voice_profile = voice_profile.voice_profile
```

---


### Proxy_session

ProxySession resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expiry_minutes` | i64 |  | <p>The number of minutes allowed for the proxy session.</p> |
| `voice_connector_id` | String | ✅ | <p>The Voice Connector ID.</p> |
| `geo_match_level` | String |  | <p>The preference for matching the country or area code of the proxy phone number with that of the first participant.</p> |
| `geo_match_params` | String |  | <p>The country and area code for the proxy phone number.</p> |
| `name` | String |  | <p>The name of the proxy session.</p> |
| `participant_phone_numbers` | Vec<String> | ✅ | <p>The participant phone numbers.</p> |
| `capabilities` | Vec<String> | ✅ | <p>The proxy session's capabilities.</p> |
| `number_selection_behavior` | String |  | <p>The preference for proxy phone number reuse, or stickiness, between the same 
            participants across sessions.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `proxy_session` | String | <p>The proxy session details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create proxy_session
proxy_session = provider.chime_sdk.Proxy_session {
    voice_connector_id = "value"  # <p>The Voice Connector ID.</p>
    participant_phone_numbers = "value"  # <p>The participant phone numbers.</p>
    capabilities = "value"  # <p>The proxy session's capabilities.</p>
}

# Access proxy_session outputs
proxy_session_id = proxy_session.id
proxy_session_proxy_session = proxy_session.proxy_session
```

---


### Voice_profile_domain

VoiceProfileDomain resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The tags assigned to the domain.</p> |
| `name` | String | ✅ | <p>The name of the voice profile domain.</p> |
| `server_side_encryption_configuration` | String | ✅ | <p>The server-side encryption configuration for the request.</p> |
| `description` | String |  | <p>A description of the voice profile domain.</p> |
| `client_request_token` | String |  | <p>The unique identifier for the client request. Use a different token for different domain creation requests.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `voice_profile_domain` | String | <p>The details of the voice profile domain.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create voice_profile_domain
voice_profile_domain = provider.chime_sdk.Voice_profile_domain {
    name = "value"  # <p>The name of the voice profile domain.</p>
    server_side_encryption_configuration = "value"  # <p>The server-side encryption configuration for the request.</p>
}

# Access voice_profile_domain outputs
voice_profile_domain_id = voice_profile_domain.id
voice_profile_domain_voice_profile_domain = voice_profile_domain.voice_profile_domain
```

---


### Voice_connector_external_systems_configuration

VoiceConnectorExternalSystemsConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `session_border_controller_types` | Vec<String> |  | <p>The session border controllers to use.</p> |
| `voice_connector_id` | String | ✅ | <p>The ID of the Voice Connector for which to add the external system
         configuration.</p> |
| `contact_center_system_types` | Vec<String> |  | <p>The contact center system to use.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `external_systems_configuration` | String | <p>An object that contains information about an external systems configuration for a Voice Connector.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create voice_connector_external_systems_configuration
voice_connector_external_systems_configuration = provider.chime_sdk.Voice_connector_external_systems_configuration {
    voice_connector_id = "value"  # <p>The ID of the Voice Connector for which to add the external system
         configuration.</p>
}

# Access voice_connector_external_systems_configuration outputs
voice_connector_external_systems_configuration_id = voice_connector_external_systems_configuration.id
voice_connector_external_systems_configuration_external_systems_configuration = voice_connector_external_systems_configuration.external_systems_configuration
```

---


### Voice_connector_termination

VoiceConnectorTermination resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `voice_connector_id` | String | ✅ | <p>The Voice Connector ID.</p> |
| `termination` | String | ✅ | <p>The termination settings to be updated.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `termination` | String | <p>The termination setting details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create voice_connector_termination
voice_connector_termination = provider.chime_sdk.Voice_connector_termination {
    voice_connector_id = "value"  # <p>The Voice Connector ID.</p>
    termination = "value"  # <p>The termination settings to be updated.</p>
}

# Access voice_connector_termination outputs
voice_connector_termination_id = voice_connector_termination.id
voice_connector_termination_termination = voice_connector_termination.termination
```

---


### Sip_media_application_logging_configuration

SipMediaApplicationLoggingConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sip_media_application_id` | String | ✅ | <p>The SIP media application ID.</p> |
| `sip_media_application_logging_configuration` | String |  | <p>The logging configuration for the specified SIP media application.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sip_media_application_logging_configuration` | String | <p>The actual logging configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sip_media_application_logging_configuration
sip_media_application_logging_configuration = provider.chime_sdk.Sip_media_application_logging_configuration {
    sip_media_application_id = "value"  # <p>The SIP media application ID.</p>
}

# Access sip_media_application_logging_configuration outputs
sip_media_application_logging_configuration_id = sip_media_application_logging_configuration.id
sip_media_application_logging_configuration_sip_media_application_logging_configuration = sip_media_application_logging_configuration.sip_media_application_logging_configuration
```

---


### Phone_number

PhoneNumber resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `phone_number_id` | String | ✅ | <p>The phone number ID.</p> |
| `product_type` | String |  | <p>The product type.</p> |
| `calling_name` | String |  | <p>The outbound calling name associated with the phone number.</p> |
| `name` | String |  | <p>Specifies the updated name assigned to one or more phone numbers.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `phone_number` | String | <p>The phone number details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access phone_number outputs
phone_number_id = phone_number.id
phone_number_phone_number = phone_number.phone_number
```

---


### Global_settings

GlobalSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `voice_connector` | String |  | <p>The Voice Connector settings.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `voice_connector` | String | <p>The Voice Connector settings.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access global_settings outputs
global_settings_id = global_settings.id
global_settings_voice_connector = global_settings.voice_connector
```

---


### Voice_tone_analysis_task

VoiceToneAnalysisTask resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `voice_tone_analysis_task` | String | <p>The details of the voice tone analysis task.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access voice_tone_analysis_task outputs
voice_tone_analysis_task_id = voice_tone_analysis_task.id
voice_tone_analysis_task_voice_tone_analysis_task = voice_tone_analysis_task.voice_tone_analysis_task
```

---


### Sip_media_application_alexa_skill_configuration

SipMediaApplicationAlexaSkillConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sip_media_application_alexa_skill_configuration` | String |  | <p>The Alexa Skill configuration.</p> |
| `sip_media_application_id` | String | ✅ | <p>The SIP media application ID.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sip_media_application_alexa_skill_configuration` | String | <p>Returns the Alexa Skill configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sip_media_application_alexa_skill_configuration
sip_media_application_alexa_skill_configuration = provider.chime_sdk.Sip_media_application_alexa_skill_configuration {
    sip_media_application_id = "value"  # <p>The SIP media application ID.</p>
}

# Access sip_media_application_alexa_skill_configuration outputs
sip_media_application_alexa_skill_configuration_id = sip_media_application_alexa_skill_configuration.id
sip_media_application_alexa_skill_configuration_sip_media_application_alexa_skill_configuration = sip_media_application_alexa_skill_configuration.sip_media_application_alexa_skill_configuration
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple voice_connector_origination resources
voice_connector_origination_0 = provider.chime_sdk.Voice_connector_origination {
    voice_connector_id = "value-0"
    origination = "value-0"
}
voice_connector_origination_1 = provider.chime_sdk.Voice_connector_origination {
    voice_connector_id = "value-1"
    origination = "value-1"
}
voice_connector_origination_2 = provider.chime_sdk.Voice_connector_origination {
    voice_connector_id = "value-2"
    origination = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    voice_connector_origination = provider.chime_sdk.Voice_connector_origination {
        voice_connector_id = "production-value"
        origination = "production-value"
    }
```

---

## Related Documentation

- [AWS Chime_sdk Documentation](https://docs.aws.amazon.com/chime_sdk/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
