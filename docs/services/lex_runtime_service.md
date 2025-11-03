# Lex_runtime_service Service



**Resources**: 1

---

## Overview

The lex_runtime_service service provides access to 1 resource type:

- [Session](#session) [CRD]

---

## Resources


### Session

Session resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `accept` | String |  | <p>The message that Amazon Lex returns in the response can be either text or
      speech based depending on the value of this field.</p>
         <ul>
            <li>
               <p>If the value is <code>text/plain; charset=utf-8</code>, Amazon Lex
          returns text in the response.</p>
            </li>
            <li>
               <p>If the value begins with <code>audio/</code>, Amazon Lex returns speech
          in the response. Amazon Lex uses Amazon Polly to generate the speech in the
          configuration that you specify. For example, if you specify
            <code>audio/mpeg</code> as the value, Amazon Lex returns speech in the
          MPEG format.</p>
            </li>
            <li>
               <p>If the value is <code>audio/pcm</code>, the speech is returned as
            <code>audio/pcm</code> in 16-bit, little endian format.</p>
            </li>
            <li>
               <p>The following are the accepted values:</p>
               <ul>
                  <li>
                     <p>
                        <code>audio/mpeg</code>
                     </p>
                  </li>
                  <li>
                     <p>
                        <code>audio/ogg</code>
                     </p>
                  </li>
                  <li>
                     <p>
                        <code>audio/pcm</code>
                     </p>
                  </li>
                  <li>
                     <p>
                        <code>audio/*</code> (defaults to mpeg)</p>
                  </li>
                  <li>
                     <p>
                        <code>text/plain; charset=utf-8</code>
                     </p>
                  </li>
               </ul>
            </li>
         </ul> |
| `bot_name` | String | ✅ | <p>The name of the bot that contains the session data.</p> |
| `dialog_action` | String |  | <p>Sets the next action that the bot should take to fulfill the
      conversation.</p> |
| `recent_intent_summary_view` | Vec<String> |  | <p>A summary of the recent intents for the bot. You can use the intent
      summary view to set a checkpoint label on an intent and modify attributes
      of intents. You can also use it to remove or add intent summary objects to
      the list.</p>
         <p>An intent that you modify or add to the list must make sense for the
      bot. For example, the intent name must be valid for the bot. You must
      provide valid values for:</p>
         <ul>
            <li>
               <p>
                  <code>intentName</code>
               </p>
            </li>
            <li>
               <p>slot names</p>
            </li>
            <li>
               <p>
                  <code>slotToElict</code>
               </p>
            </li>
         </ul>
         <p>If you send the <code>recentIntentSummaryView</code> parameter in a
        <code>PutSession</code> request, the contents of the new summary view
      replaces the old summary view. For example, if a <code>GetSession</code>
      request returns three intents in the summary view and you call
        <code>PutSession</code> with one intent in the summary view, the next
      call to <code>GetSession</code> will only return one intent.</p> |
| `bot_alias` | String | ✅ | <p>The alias in use for the bot that contains the session data.</p> |
| `active_contexts` | Vec<String> |  | <p>A list of contexts active for the request. A context can be activated
      when a previous intent is fulfilled, or by including the context in the
      request,</p>
         <p>If you don't specify a list of contexts, Amazon Lex will use the current
      list of contexts for the session. If you specify an empty list, all
      contexts for the session are cleared.</p> |
| `user_id` | String | ✅ | <p>The ID of the client application user. Amazon Lex uses this to identify a
      user's conversation with your bot. </p> |
| `session_attributes` | String |  | <p>Map of key/value pairs representing the session-specific context
      information. It contains application information passed between Amazon Lex and
      a client application.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `session_id` | String | <p>A unique identifier for the session.</p> |
| `session_attributes` | String | <p>Map of key/value pairs representing the session-specific context
      information. It contains application information passed between Amazon Lex and
      a client application.</p> |
| `dialog_action` | String | <p>Describes the current state of the bot.</p> |
| `active_contexts` | Vec<String> | <p>A list of active contexts for the session. A context can be set when
      an intent is fulfilled or by calling the <code>PostContent</code>,
        <code>PostText</code>, or <code>PutSession</code> operation.</p>
         <p>You can use a context to control the intents that can follow up an
      intent, or to modify the operation of your application.</p> |
| `recent_intent_summary_view` | Vec<String> | <p>An array of information about the intents used in the session. The
      array can contain a maximum of three summaries. If more than three intents
      are used in the session, the <code>recentIntentSummaryView</code>
      operation contains information about the last three intents used.</p>
         <p>If you set the <code>checkpointLabelFilter</code> parameter in the
      request, the array contains only the intents with the specified
      label.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create session
session = provider.lex_runtime_service.Session {
    bot_name = "value"  # <p>The name of the bot that contains the session data.</p>
    bot_alias = "value"  # <p>The alias in use for the bot that contains the session data.</p>
    user_id = "value"  # <p>The ID of the client application user. Amazon Lex uses this to identify a
      user's conversation with your bot. </p>
}

# Access session outputs
session_id = session.id
session_session_id = session.session_id
session_session_attributes = session.session_attributes
session_dialog_action = session.dialog_action
session_active_contexts = session.active_contexts
session_recent_intent_summary_view = session.recent_intent_summary_view
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
session_0 = provider.lex_runtime_service.Session {
    bot_name = "value-0"
    bot_alias = "value-0"
    user_id = "value-0"
}
session_1 = provider.lex_runtime_service.Session {
    bot_name = "value-1"
    bot_alias = "value-1"
    user_id = "value-1"
}
session_2 = provider.lex_runtime_service.Session {
    bot_name = "value-2"
    bot_alias = "value-2"
    user_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    session = provider.lex_runtime_service.Session {
        bot_name = "production-value"
        bot_alias = "production-value"
        user_id = "production-value"
    }
```

---

## Related Documentation

- [AWS Lex_runtime_service Documentation](https://docs.aws.amazon.com/lex_runtime_service/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
