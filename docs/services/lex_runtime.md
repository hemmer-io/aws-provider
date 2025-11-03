# Lex_runtime Service



**Resources**: 1

---

## Overview

The lex_runtime service provides access to 1 resource type:

- [Session](#session) [CRD]

---

## Resources


### Session

Session resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bot_id` | String | ✅ | <p>The identifier of the bot that receives the session data.</p> |
| `locale_id` | String | ✅ | <p>The locale where the session is in use.</p> |
| `messages` | Vec<String> |  | <p>A list of messages to send to the user. Messages are sent in the
         order that they are defined in the list.</p> |
| `response_content_type` | String |  | <p>The message that Amazon Lex V2 returns in the response can be either text or
         speech depending on the value of this parameter. </p>
         <ul>
            <li>
               <p>If the value is <code>text/plain; charset=utf-8</code>, Amazon Lex V2
               returns text in the response.</p>
            </li>
         </ul> |
| `bot_alias_id` | String | ✅ | <p>The alias identifier of the bot that receives the session
         data.</p> |
| `session_id` | String | ✅ | <p>The identifier of the session that receives the session data.</p> |
| `session_state` | String | ✅ | <p>Sets the state of the session with the user. You can use this to set
         the current intent, attributes, context, and dialog action. Use the
         dialog action to determine the next step that Amazon Lex V2 should use in the
         conversation with the user.</p> |
| `request_attributes` | String |  | <p>Request-specific information passed between Amazon Lex V2 and the client
         application.</p>
         <p>The namespace <code>x-amz-lex:</code> is reserved for special
         attributes. Don't create any request attributes with the prefix
            <code>x-amz-lex:</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `session_id` | String | <p>The identifier of the returned session.</p> |
| `messages` | Vec<String> | <p>A list of messages that were last sent to the user. The messages are
         ordered based on the order that your returned the messages from your
         Lambda function or the order that messages are defined in the bot.
      </p> |
| `interpretations` | Vec<String> | <p>A list of intents that Amazon Lex V2 determined might satisfy the user's
         utterance. </p>
         <p>Each interpretation includes the intent, a score that indicates how
         confident Amazon Lex V2 is that the interpretation is the correct one, and an
         optional sentiment response that indicates the sentiment expressed in
         the utterance.</p> |
| `session_state` | String | <p>Represents the current state of the dialog between the user and the
         bot.</p>
         <p>You can use this to determine the progress of the conversation and
         what the next action might be.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create session
session = provider.lex_runtime.Session {
    bot_id = "value"  # <p>The identifier of the bot that receives the session data.</p>
    locale_id = "value"  # <p>The locale where the session is in use.</p>
    bot_alias_id = "value"  # <p>The alias identifier of the bot that receives the session
         data.</p>
    session_id = "value"  # <p>The identifier of the session that receives the session data.</p>
    session_state = "value"  # <p>Sets the state of the session with the user. You can use this to set
         the current intent, attributes, context, and dialog action. Use the
         dialog action to determine the next step that Amazon Lex V2 should use in the
         conversation with the user.</p>
}

# Access session outputs
session_id = session.id
session_session_id = session.session_id
session_messages = session.messages
session_interpretations = session.interpretations
session_session_state = session.session_state
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple session resources
session_0 = provider.lex_runtime.Session {
    bot_id = "value-0"
    locale_id = "value-0"
    bot_alias_id = "value-0"
    session_id = "value-0"
    session_state = "value-0"
}
session_1 = provider.lex_runtime.Session {
    bot_id = "value-1"
    locale_id = "value-1"
    bot_alias_id = "value-1"
    session_id = "value-1"
    session_state = "value-1"
}
session_2 = provider.lex_runtime.Session {
    bot_id = "value-2"
    locale_id = "value-2"
    bot_alias_id = "value-2"
    session_id = "value-2"
    session_state = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    session = provider.lex_runtime.Session {
        bot_id = "production-value"
        locale_id = "production-value"
        bot_alias_id = "production-value"
        session_id = "production-value"
        session_state = "production-value"
    }
```

---

## Related Documentation

- [AWS Lex_runtime Documentation](https://docs.aws.amazon.com/lex_runtime/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
