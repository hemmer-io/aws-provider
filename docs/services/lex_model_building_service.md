# Lex_model_building_service Service



**Resources**: 25

---

## Overview

The lex_model_building_service service provides access to 25 resource types:

- [Slot_types](#slot_types) [R]
- [Bot_alias](#bot_alias) [CRD]
- [Builtin_intents](#builtin_intents) [R]
- [Import](#import) [R]
- [Bot_channel_association](#bot_channel_association) [RD]
- [Intent](#intent) [CRD]
- [Bot](#bot) [CRD]
- [Builtin_slot_types](#builtin_slot_types) [R]
- [Intent_versions](#intent_versions) [R]
- [Bot_versions](#bot_versions) [R]
- [Migration](#migration) [R]
- [Slot_type_version](#slot_type_version) [CD]
- [Utterances](#utterances) [D]
- [Bot_aliases](#bot_aliases) [R]
- [Migrations](#migrations) [R]
- [Slot_type_versions](#slot_type_versions) [R]
- [Utterances_view](#utterances_view) [R]
- [Intent_version](#intent_version) [CD]
- [Slot_type](#slot_type) [CRD]
- [Builtin_intent](#builtin_intent) [R]
- [Bot_channel_associations](#bot_channel_associations) [R]
- [Bot_version](#bot_version) [CD]
- [Bots](#bots) [R]
- [Export](#export) [R]
- [Intents](#intents) [R]

---

## Resources


### Slot_types

SlotTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If the response is truncated, it includes a pagination token that
      you can specify in your next request to fetch the next page of slot
      types.</p> |
| `slot_types` | Vec<String> | <p>An array of objects, one for each slot type, that provides
      information such as the name of the slot type, the version, and a
      description.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access slot_types outputs
slot_types_id = slot_types.id
slot_types_next_token = slot_types.next_token
slot_types_slot_types = slot_types.slot_types
```

---


### Bot_alias

BotAlias resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bot_version` | String | ✅ | <p>The version of the bot.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to add to the bot alias. You can only add tags when you
      create an alias, you can't use the <code>PutBotAlias</code> operation to
      update the tags on a bot alias. To update tags, use the
        <code>TagResource</code> operation.</p> |
| `description` | String |  | <p>A description of the alias.</p> |
| `checksum` | String |  | <p>Identifies a specific revision of the <code>$LATEST</code>
      version.</p>
         <p>When you create a new bot alias, leave the <code>checksum</code>
      field blank. If you specify a checksum you get a
        <code>BadRequestException</code> exception.</p>
         <p>When you want to update a bot alias, set the <code>checksum</code>
      field to the checksum of the most recent revision of the
        <code>$LATEST</code> version. If you don't specify the <code>
        checksum</code> field, or if the checksum does not match the
        <code>$LATEST</code> version, you get a
        <code>PreconditionFailedException</code> exception.</p> |
| `conversation_logs` | String |  | <p>Settings for conversation logs for the alias.</p> |
| `name` | String | ✅ | <p>The name of the alias. The name is <i>not</i> case
      sensitive.</p> |
| `bot_name` | String | ✅ | <p>The name of the bot.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `checksum` | String | <p>Checksum of the bot alias.</p> |
| `created_date` | String | <p>The date that the bot alias was created.</p> |
| `description` | String | <p>A description of the bot alias.</p> |
| `conversation_logs` | String | <p>The settings that determine how Amazon Lex uses conversation logs for the
      alias.</p> |
| `name` | String | <p>The name of the bot alias.</p> |
| `bot_version` | String | <p>The version of the bot that the alias points to.</p> |
| `bot_name` | String | <p>The name of the bot that the alias points to.</p> |
| `last_updated_date` | String | <p>The date that the bot alias was updated. When you create a
      resource, the creation date and the last updated date are the
      same.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bot_alias
bot_alias = provider.lex_model_building_service.Bot_alias {
    bot_version = "value"  # <p>The version of the bot.</p>
    name = "value"  # <p>The name of the alias. The name is <i>not</i> case
      sensitive.</p>
    bot_name = "value"  # <p>The name of the bot.</p>
}

# Access bot_alias outputs
bot_alias_id = bot_alias.id
bot_alias_checksum = bot_alias.checksum
bot_alias_created_date = bot_alias.created_date
bot_alias_description = bot_alias.description
bot_alias_conversation_logs = bot_alias.conversation_logs
bot_alias_name = bot_alias.name
bot_alias_bot_version = bot_alias.bot_version
bot_alias_bot_name = bot_alias.bot_name
bot_alias_last_updated_date = bot_alias.last_updated_date
```

---


### Builtin_intents

BuiltinIntents resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `intents` | Vec<String> | <p>An array of <code>builtinIntentMetadata</code> objects, one for
      each intent in the response.</p> |
| `next_token` | String | <p>A pagination token that fetches the next page of intents. If the
      response to this API call is truncated, Amazon Lex returns a pagination token
      in the response. To fetch the next page of intents, specify the pagination
      token in the next request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access builtin_intents outputs
builtin_intents_id = builtin_intents.id
builtin_intents_intents = builtin_intents.intents
builtin_intents_next_token = builtin_intents.next_token
```

---


### Import

Import resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_type` | String | <p>The type of resource imported.</p> |
| `name` | String | <p>The name given to the import job.</p> |
| `import_status` | String | <p>The status of the import job. If the status is <code>FAILED</code>,
      you can get the reason for the failure from the <code>failureReason</code>
      field.</p> |
| `failure_reason` | String | <p>A string that describes why an import job failed to
      complete.</p> |
| `created_date` | String | <p>A timestamp for the date and time that the import job was
      created.</p> |
| `merge_strategy` | String | <p>The action taken when there was a conflict between an existing
      resource and a resource in the import file.</p> |
| `import_id` | String | <p>The identifier for the specific import job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access import outputs
import_id = import.id
import_resource_type = import.resource_type
import_name = import.name
import_import_status = import.import_status
import_failure_reason = import.failure_reason
import_created_date = import.created_date
import_merge_strategy = import.merge_strategy
import_import_id = import.import_id
```

---


### Bot_channel_association

BotChannelAssociation resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `failure_reason` | String | <p>If <code>status</code> is <code>FAILED</code>, Amazon Lex provides the
      reason that it failed to create the association.</p> |
| `description` | String | <p>A description of the association between the bot and the
      channel.</p> |
| `type` | String | <p>The type of the messaging platform.</p> |
| `name` | String | <p>The name of the association between the bot and the
      channel.</p> |
| `bot_name` | String | <p>The name of the Amazon Lex bot.</p> |
| `bot_alias` | String | <p>An alias pointing to the specific version of the Amazon Lex bot to which
      this association is being made.</p> |
| `created_date` | String | <p>The date that the association between the bot and the channel was
      created.</p> |
| `bot_configuration` | HashMap<String, String> | <p>Provides information that the messaging platform needs to
      communicate with the Amazon Lex bot.</p> |
| `status` | String | <p>The status of the bot channel. </p>
         <ul>
            <li>
               <p>
                  <code>CREATED</code> - The channel has been created and is
          ready for use.</p>
            </li>
            <li>
               <p>
                  <code>IN_PROGRESS</code> - Channel creation is in
          progress.</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code> - There was an error creating the channel.
          For information about the reason for the failure, see the
            <code>failureReason</code> field.</p>
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

# Access bot_channel_association outputs
bot_channel_association_id = bot_channel_association.id
bot_channel_association_failure_reason = bot_channel_association.failure_reason
bot_channel_association_description = bot_channel_association.description
bot_channel_association_type = bot_channel_association.type
bot_channel_association_name = bot_channel_association.name
bot_channel_association_bot_name = bot_channel_association.bot_name
bot_channel_association_bot_alias = bot_channel_association.bot_alias
bot_channel_association_created_date = bot_channel_association.created_date
bot_channel_association_bot_configuration = bot_channel_association.bot_configuration
bot_channel_association_status = bot_channel_association.status
```

---


### Intent

Intent resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rejection_statement` | String |  | <p>When the user answers "no" to the question defined in
        <code>confirmationPrompt</code>, Amazon Lex responds with this statement to
      acknowledge that the intent was canceled. </p>
         <note>
            <p>You must provide both the <code>rejectionStatement</code> and the
          <code>confirmationPrompt</code>, or neither.</p>
         </note> |
| `confirmation_prompt` | String |  | <p>Prompts the user to confirm the intent. This question should have a
      yes or no answer.</p>
         <p>Amazon Lex uses this prompt to ensure that the user acknowledges that
      the intent is ready for fulfillment. For example, with the
        <code>OrderPizza</code> intent, you might want to confirm that the order
      is correct before placing it. For other intents, such as intents that
      simply respond to user questions, you might not need to ask the user for
      confirmation before providing the information. </p>
         <note>
            <p>You you must provide both the <code>rejectionStatement</code> and
        the <code>confirmationPrompt</code>, or neither.</p>
         </note> |
| `name` | String | ✅ | <p>The name of the intent. The name is <i>not</i> case
      sensitive. </p>
         <p>The name can't match a built-in intent name, or a built-in intent
      name with "AMAZON." removed. For example, because there is a built-in
      intent called <code>AMAZON.HelpIntent</code>, you can't create a custom
      intent called <code>HelpIntent</code>.</p>
         <p>For a list of built-in intents, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/standard-intents">Standard Built-in Intents</a> in the <i>Alexa Skills
        Kit</i>.</p> |
| `parent_intent_signature` | String |  | <p>A unique identifier for the built-in intent to base this intent on.
      To find the signature for an intent, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/standard-intents">Standard Built-in Intents</a> in the <i>Alexa Skills
        Kit</i>.</p> |
| `input_contexts` | Vec<String> |  | <p>An array of <code>InputContext</code> objects that lists the contexts
      that must be active for Amazon Lex to choose the intent in a conversation with
      the user.</p> |
| `checksum` | String |  | <p>Identifies a specific revision of the <code>$LATEST</code>
      version.</p>
         <p>When you create a new intent, leave the <code>checksum</code> field
      blank. If you specify a checksum you get a
        <code>BadRequestException</code> exception.</p>
         <p>When you want to update a intent, set the <code>checksum</code>
      field to the checksum of the most recent revision of the
        <code>$LATEST</code> version. If you don't specify the <code>
        checksum</code> field, or if the checksum does not match the
        <code>$LATEST</code> version, you get a
        <code>PreconditionFailedException</code> exception.</p> |
| `sample_utterances` | Vec<String> |  | <p>An array of utterances (strings) that a user might say to signal
      the intent. For example, "I want {PizzaSize} pizza", "Order {Quantity}
      {PizzaSize} pizzas". </p>
         <p>In each utterance, a slot name is enclosed in curly braces.
    </p> |
| `follow_up_prompt` | String |  | <p>Amazon Lex uses this prompt to solicit additional activity after
      fulfilling an intent. For example, after the <code>OrderPizza</code>
      intent is fulfilled, you might prompt the user to order a drink.</p>
         <p>The action that Amazon Lex takes depends on the user's response, as
      follows:</p>
         <ul>
            <li>
               <p>If the user says "Yes" it responds with the clarification
          prompt that is configured for the bot.</p>
            </li>
            <li>
               <p>if the user says "Yes" and continues with an utterance that
          triggers an intent it starts a conversation for the intent.</p>
            </li>
            <li>
               <p>If the user says "No" it responds with the rejection statement
          configured for the the follow-up prompt.</p>
            </li>
            <li>
               <p>If it doesn't recognize the utterance it repeats the follow-up
          prompt again.</p>
            </li>
         </ul>
         <p>The <code>followUpPrompt</code> field and the
        <code>conclusionStatement</code> field are mutually exclusive. You can
      specify only one. </p> |
| `dialog_code_hook` | String |  | <p> Specifies a Lambda function to invoke for each user input. You can
      invoke this Lambda function to personalize user interaction. </p>
         <p>For example, suppose your bot determines that the user is John.
      Your Lambda function might retrieve John's information from a backend
      database and prepopulate some of the values. For example, if you find that
      John is gluten intolerant, you might set the corresponding intent slot,
        <code>GlutenIntolerant</code>, to true. You might find John's phone
      number and set the corresponding session attribute. </p> |
| `fulfillment_activity` | String |  | <p>Required. Describes how the intent is fulfilled. For example, after
      a user provides all of the information for a pizza order,
        <code>fulfillmentActivity</code> defines how the bot places an order
      with a local pizza store. </p>
         <p> You might configure Amazon Lex to return all of the intent information
      to the client application, or direct it to invoke a Lambda function that
      can process the intent (for example, place an order with a pizzeria).
    </p> |
| `conclusion_statement` | String |  | <p> The statement that you want Amazon Lex to convey to the user after the
      intent is successfully fulfilled by the Lambda function. </p>
         <p>This element is relevant only if you provide a Lambda function in
      the <code>fulfillmentActivity</code>. If you return the intent to the
      client application, you can't specify this element.</p>
         <note>
            <p>The <code>followUpPrompt</code> and
          <code>conclusionStatement</code> are mutually exclusive. You can
        specify only one.</p>
         </note> |
| `output_contexts` | Vec<String> |  | <p>An array of <code>OutputContext</code> objects that lists the contexts
      that the intent activates when the intent is fulfilled.</p> |
| `description` | String |  | <p>A description of the intent.</p> |
| `create_version` | bool |  | <p>When set to <code>true</code> a new numbered version of the intent
      is created. This is the same as calling the
        <code>CreateIntentVersion</code> operation. If you do not specify
        <code>createVersion</code>, the default is <code>false</code>.</p> |
| `kendra_configuration` | String |  | <p>Configuration information required to use the
        <code>AMAZON.KendraSearchIntent</code> intent to connect to an Amazon
      Kendra index. For more information, see <a href="http://docs.aws.amazon.com/lex/latest/dg/built-in-intent-kendra-search.html">
        AMAZON.KendraSearchIntent</a>.</p> |
| `slots` | Vec<String> |  | <p>An array of intent slots. At runtime, Amazon Lex elicits required slot
      values from the user using prompts defined in the slots. For more
      information, see <a>how-it-works</a>. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_updated_date` | String | <p>The date that the intent was updated. When you create a resource,
      the creation date and the last updated date are the same. </p> |
| `confirmation_prompt` | String | <p>If defined in the bot, Amazon Lex uses prompt to confirm the intent
      before fulfilling the user's request. For more information, see <a>PutIntent</a>. </p> |
| `rejection_statement` | String | <p>If the user answers "no" to the question defined in
        <code>confirmationPrompt</code>, Amazon Lex responds with this statement to
      acknowledge that the intent was canceled. </p> |
| `slots` | Vec<String> | <p>An array of intent slots configured for the intent.</p> |
| `sample_utterances` | Vec<String> | <p>An array of sample utterances configured for the intent.</p> |
| `follow_up_prompt` | String | <p>If defined in the bot, Amazon Lex uses this prompt to solicit additional
      user activity after the intent is fulfilled. For more information, see
        <a>PutIntent</a>.</p> |
| `checksum` | String | <p>Checksum of the intent.</p> |
| `description` | String | <p>A description of the intent.</p> |
| `conclusion_statement` | String | <p>After the Lambda function specified in the
        <code>fulfillmentActivity</code> element fulfills the intent, Amazon Lex
      conveys this statement to the user.</p> |
| `kendra_configuration` | String | <p>Configuration information, if any, to connect to an Amazon Kendra
      index with the <code>AMAZON.KendraSearchIntent</code> intent.</p> |
| `output_contexts` | Vec<String> | <p>An array of <code>OutputContext</code> objects that lists the contexts
      that the intent activates when the intent is fulfilled.</p> |
| `fulfillment_activity` | String | <p>Describes how the intent is fulfilled. For more information, see
        <a>PutIntent</a>. </p> |
| `dialog_code_hook` | String | <p>If defined in the bot, Amazon Amazon Lex invokes this Lambda function
      for each user input. For more information, see <a>PutIntent</a>. </p> |
| `parent_intent_signature` | String | <p>A unique identifier for a built-in intent.</p> |
| `created_date` | String | <p>The date that the intent was created.</p> |
| `version` | String | <p>The version of the intent.</p> |
| `name` | String | <p>The name of the intent.</p> |
| `input_contexts` | Vec<String> | <p>An array of <code>InputContext</code> objects that lists the contexts
      that must be active for Amazon Lex to choose the intent in a conversation with
      the user.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create intent
intent = provider.lex_model_building_service.Intent {
    name = "value"  # <p>The name of the intent. The name is <i>not</i> case
      sensitive. </p>
         <p>The name can't match a built-in intent name, or a built-in intent
      name with "AMAZON." removed. For example, because there is a built-in
      intent called <code>AMAZON.HelpIntent</code>, you can't create a custom
      intent called <code>HelpIntent</code>.</p>
         <p>For a list of built-in intents, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/standard-intents">Standard Built-in Intents</a> in the <i>Alexa Skills
        Kit</i>.</p>
}

# Access intent outputs
intent_id = intent.id
intent_last_updated_date = intent.last_updated_date
intent_confirmation_prompt = intent.confirmation_prompt
intent_rejection_statement = intent.rejection_statement
intent_slots = intent.slots
intent_sample_utterances = intent.sample_utterances
intent_follow_up_prompt = intent.follow_up_prompt
intent_checksum = intent.checksum
intent_description = intent.description
intent_conclusion_statement = intent.conclusion_statement
intent_kendra_configuration = intent.kendra_configuration
intent_output_contexts = intent.output_contexts
intent_fulfillment_activity = intent.fulfillment_activity
intent_dialog_code_hook = intent.dialog_code_hook
intent_parent_intent_signature = intent.parent_intent_signature
intent_created_date = intent.created_date
intent_version = intent.version
intent_name = intent.name
intent_input_contexts = intent.input_contexts
```

---


### Bot

Bot resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the bot. The name is <i>not</i> case
      sensitive. </p> |
| `clarification_prompt` | String |  | <p>When Amazon Lex doesn't understand the user's intent, it uses this
      message to get clarification. To specify how many times Amazon Lex should
      repeat the clarification prompt, use the <code>maxAttempts</code> field.
      If Amazon Lex still doesn't understand, it sends the message in the
        <code>abortStatement</code> field. </p>
         <p>When you create a clarification prompt, make sure that it suggests
      the correct response from the user. for example, for a bot that orders
      pizza and drinks, you might create this clarification prompt: "What would
      you like to do? You can say 'Order a pizza' or 'Order a drink.'"</p>
         <p>If you have defined a fallback intent, it will be invoked if the
      clarification prompt is repeated the number of times defined in the
        <code>maxAttempts</code> field. For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/built-in-intent-fallback.html">
        AMAZON.FallbackIntent</a>.</p>
         <p>If you don't define a clarification prompt, at runtime Amazon Lex will
      return a 400 Bad Request exception in three cases: </p>
         <ul>
            <li>
               <p>Follow-up prompt - When the user responds to a follow-up prompt
          but does not provide an intent. For example, in response to a
          follow-up prompt that says "Would you like anything else today?" the
          user says "Yes." Amazon Lex will return a 400 Bad Request exception because
          it does not have a clarification prompt to send to the user to get an
          intent.</p>
            </li>
            <li>
               <p>Lambda function - When using a Lambda function, you return an
            <code>ElicitIntent</code> dialog type. Since Amazon Lex does not have a
          clarification prompt to get an intent from the user, it returns a 400
          Bad Request exception.</p>
            </li>
            <li>
               <p>PutSession operation - When using the <code>PutSession</code>
          operation, you send an <code>ElicitIntent</code> dialog type. Since
          Amazon Lex does not have a clarification prompt to get an intent from the
          user, it returns a 400 Bad Request exception.</p>
            </li>
         </ul> |
| `intents` | Vec<String> |  | <p>An array of <code>Intent</code> objects. Each intent represents a
      command that a user can express. For example, a pizza ordering bot might
      support an OrderPizza intent. For more information, see <a>how-it-works</a>.</p> |
| `child_directed` | bool | ✅ | <p>For each Amazon Lex bot created with the Amazon Lex Model Building Service,
      you must specify whether your use of Amazon Lex is related to a website,
      program, or other application that is directed or targeted, in whole or in
      part, to children under age 13 and subject to the Children's Online
      Privacy Protection Act (COPPA) by specifying <code>true</code> or
        <code>false</code> in the <code>childDirected</code> field. By
      specifying <code>true</code> in the <code>childDirected</code> field, you
      confirm that your use of Amazon Lex <b>is</b> related
      to a website, program, or other application that is directed or targeted,
      in whole or in part, to children under age 13 and subject to COPPA. By
      specifying <code>false</code> in the <code>childDirected</code> field, you
      confirm that your use of Amazon Lex <b>is not</b>
      related to a website, program, or other application that is directed or
      targeted, in whole or in part, to children under age 13 and subject to
      COPPA. You may not specify a default value for the
        <code>childDirected</code> field that does not accurately reflect
      whether your use of Amazon Lex is related to a website, program, or other
      application that is directed or targeted, in whole or in part, to children
      under age 13 and subject to COPPA.</p>
         <p>If your use of Amazon Lex relates to a website, program, or other
      application that is directed in whole or in part, to children under age
      13, you must obtain any required verifiable parental consent under COPPA.
      For information regarding the use of Amazon Lex in connection with websites,
      programs, or other applications that are directed or targeted, in whole or
      in part, to children under age 13, see the <a href="https://aws.amazon.com/lex/faqs#data-security">Amazon Lex FAQ.</a>
         </p> |
| `detect_sentiment` | bool |  | <p>When set to <code>true</code> user utterances are sent to Amazon
      Comprehend for sentiment analysis. If you don't specify
        <code>detectSentiment</code>, the default is <code>false</code>.</p> |
| `voice_id` | String |  | <p>The Amazon Polly voice ID that you want Amazon Lex to use for voice
      interactions with the user. The locale configured for the voice must match
      the locale of the bot. For more information, see <a href="https://docs.aws.amazon.com/polly/latest/dg/voicelist.html">Voices
        in Amazon Polly</a> in the <i>Amazon Polly Developer
        Guide</i>.</p> |
| `process_behavior` | String |  | <p>If you set the <code>processBehavior</code> element to
        <code>BUILD</code>, Amazon Lex builds the bot so that it can be run. If you
      set the element to <code>SAVE</code> Amazon Lex saves the bot, but doesn't
      build it. </p>
         <p>If you don't specify this value, the default value is
        <code>BUILD</code>.</p> |
| `create_version` | bool |  | <p>When set to <code>true</code> a new numbered version of the bot is
      created. This is the same as calling the <code>CreateBotVersion</code>
      operation. If you don't specify <code>createVersion</code>, the default is
        <code>false</code>.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to add to the bot. You can only add tags when you
      create a bot, you can't use the <code>PutBot</code> operation to update
      the tags on a bot. To update tags, use the <code>TagResource</code>
      operation.</p> |
| `description` | String |  | <p>A description of the bot.</p> |
| `checksum` | String |  | <p>Identifies a specific revision of the <code>$LATEST</code>
      version.</p>
         <p>When you create a new bot, leave the <code>checksum</code> field
      blank. If you specify a checksum you get a
        <code>BadRequestException</code> exception.</p>
         <p>When you want to update a bot, set the <code>checksum</code> field
      to the checksum of the most recent revision of the <code>$LATEST</code>
      version. If you don't specify the <code> checksum</code> field, or if the
      checksum does not match the <code>$LATEST</code> version, you get a
        <code>PreconditionFailedException</code> exception.</p> |
| `locale` | String | ✅ | <p> Specifies the target locale for the bot. Any intent used in the
      bot must be compatible with the locale of the bot. </p>
         <p>The default is <code>en-US</code>.</p> |
| `idle_session_ttl_in_seconds` | i64 |  | <p>The maximum time in seconds that Amazon Lex retains the data gathered in
      a conversation.</p>
         <p>A user interaction session remains active for the amount of time
      specified. If no conversation occurs during this time, the session expires
      and Amazon Lex deletes any data provided before the timeout.</p>
         <p>For example, suppose that a user chooses the OrderPizza intent, but
      gets sidetracked halfway through placing an order. If the user doesn't
      complete the order within the specified time, Amazon Lex discards the slot
      information that it gathered, and the user must start over.</p>
         <p>If you don't include the <code>idleSessionTTLInSeconds</code>
      element in a <code>PutBot</code> operation request, Amazon Lex uses the default
      value. This is also true if the request replaces an existing
      bot.</p>
         <p>The default is 300 seconds (5 minutes).</p> |
| `enable_model_improvements` | bool |  | <p>Set to <code>true</code> to enable access to natural language
      understanding improvements. </p>
         <p>When you set the <code>enableModelImprovements</code> parameter to
        <code>true</code> you can use the
        <code>nluIntentConfidenceThreshold</code> parameter to configure
      confidence scores. For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/confidence-scores.html">Confidence Scores</a>.</p>
         <p>You can only set the <code>enableModelImprovements</code> parameter in
      certain Regions. If you set the parameter to <code>true</code>, your bot
      has access to accuracy improvements.</p>
         <p>The Regions where you can set the <code>enableModelImprovements</code>
      parameter to <code>true</code> are:</p>
         <ul>
            <li>
               <p>US East (N. Virginia) (us-east-1)</p>
            </li>
            <li>
               <p>US West (Oregon) (us-west-2)</p>
            </li>
            <li>
               <p>Asia Pacific (Sydney) (ap-southeast-2)</p>
            </li>
            <li>
               <p>EU (Ireland) (eu-west-1)</p>
            </li>
         </ul>
         <p>In other Regions, the <code>enableModelImprovements</code> parameter
      is set to <code>true</code> by default. In these Regions setting the
      parameter to <code>false</code> throws a <code>ValidationException</code>
      exception.</p> |
| `abort_statement` | String |  | <p>When Amazon Lex can't understand the user's input in context, it tries
      to elicit the information a few times. After that, Amazon Lex sends the message
      defined in <code>abortStatement</code> to the user, and then cancels the
      conversation. To set the number of retries, use the
        <code>valueElicitationPrompt</code> field for the slot type. </p>
         <p>For example, in a pizza ordering bot, Amazon Lex might ask a user "What
      type of crust would you like?" If the user's response is not one of the
      expected responses (for example, "thin crust, "deep dish," etc.), Amazon Lex
      tries to elicit a correct response a few more times. </p>
         <p>For example, in a pizza ordering application,
        <code>OrderPizza</code> might be one of the intents. This intent might
      require the <code>CrustType</code> slot. You specify the
        <code>valueElicitationPrompt</code> field when you create the
        <code>CrustType</code> slot.</p>
         <p>If you have defined a fallback intent the cancel statement will not be
      sent to the user, the fallback intent is used instead. For more
      information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/built-in-intent-fallback.html">
        AMAZON.FallbackIntent</a>.</p> |
| `nlu_intent_confidence_threshold` | f64 |  | <p>Determines the threshold where Amazon Lex will insert the
        <code>AMAZON.FallbackIntent</code>,
        <code>AMAZON.KendraSearchIntent</code>, or both when returning
      alternative intents in a <a href="https://docs.aws.amazon.com/lex/latest/dg/API_runtime_PostContent.html">PostContent</a> or
        <a href="https://docs.aws.amazon.com/lex/latest/dg/API_runtime_PostText.html">PostText</a> response.
        <code>AMAZON.FallbackIntent</code> and
        <code>AMAZON.KendraSearchIntent</code> are only inserted if they are
      configured for the bot.</p>
         <p>You must set the <code>enableModelImprovements</code> parameter to
        <code>true</code> to use confidence scores in the following
      regions.</p>
         <ul>
            <li>
               <p>US East (N. Virginia) (us-east-1)</p>
            </li>
            <li>
               <p>US West (Oregon) (us-west-2)</p>
            </li>
            <li>
               <p>Asia Pacific (Sydney) (ap-southeast-2)</p>
            </li>
            <li>
               <p>EU (Ireland) (eu-west-1)</p>
            </li>
         </ul>
         <p>In other Regions, the <code>enableModelImprovements</code> parameter
      is set to <code>true</code> by default.</p>
         <p>For example, suppose a bot is configured with the confidence threshold
      of 0.80 and the <code>AMAZON.FallbackIntent</code>. Amazon Lex returns three
      alternative intents with the following confidence scores: IntentA (0.70),
      IntentB (0.60), IntentC (0.50). The response from the
        <code>PostText</code> operation would be:</p>
         <ul>
            <li>
               <p>AMAZON.FallbackIntent</p>
            </li>
            <li>
               <p>IntentA</p>
            </li>
            <li>
               <p>IntentB</p>
            </li>
            <li>
               <p>IntentC</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version` | String | <p>The version of the bot. For a new bot, the version is always
        <code>$LATEST</code>.</p> |
| `child_directed` | bool | <p>For each Amazon Lex bot created with the Amazon Lex Model Building Service,
      you must specify whether your use of Amazon Lex is related to a website,
      program, or other application that is directed or targeted, in whole or in
      part, to children under age 13 and subject to the Children's Online
      Privacy Protection Act (COPPA) by specifying <code>true</code> or
        <code>false</code> in the <code>childDirected</code> field. By
      specifying <code>true</code> in the <code>childDirected</code> field, you
      confirm that your use of Amazon Lex <b>is</b> related
      to a website, program, or other application that is directed or targeted,
      in whole or in part, to children under age 13 and subject to COPPA. By
      specifying <code>false</code> in the <code>childDirected</code> field, you
      confirm that your use of Amazon Lex <b>is not</b>
      related to a website, program, or other application that is directed or
      targeted, in whole or in part, to children under age 13 and subject to
      COPPA. You may not specify a default value for the
        <code>childDirected</code> field that does not accurately reflect
      whether your use of Amazon Lex is related to a website, program, or other
      application that is directed or targeted, in whole or in part, to children
      under age 13 and subject to COPPA.</p>
         <p>If your use of Amazon Lex relates to a website, program, or other
      application that is directed in whole or in part, to children under age
      13, you must obtain any required verifiable parental consent under COPPA.
      For information regarding the use of Amazon Lex in connection with websites,
      programs, or other applications that are directed or targeted, in whole or
      in part, to children under age 13, see the <a href="https://aws.amazon.com/lex/faqs#data-security">Amazon Lex FAQ.</a>
         </p> |
| `detect_sentiment` | bool | <p>Indicates whether user utterances should be sent to Amazon Comprehend
      for sentiment analysis.</p> |
| `description` | String | <p>A description of the bot.</p> |
| `intents` | Vec<String> | <p>An array of <code>intent</code> objects. For more information, see
        <a>PutBot</a>.</p> |
| `last_updated_date` | String | <p>The date that the bot was updated. When you create a resource, the
      creation date and last updated date are the same. </p> |
| `created_date` | String | <p>The date that the bot was created.</p> |
| `locale` | String | <p> The target locale for the bot. </p> |
| `failure_reason` | String | <p>If <code>status</code> is <code>FAILED</code>, Amazon Lex explains why
      it failed to build the bot.</p> |
| `clarification_prompt` | String | <p>The message Amazon Lex uses when it doesn't understand the user's
      request. For more information, see <a>PutBot</a>. </p> |
| `abort_statement` | String | <p>The message that Amazon Lex returns when the user elects to end the
      conversation without completing it. For more information, see <a>PutBot</a>.</p> |
| `enable_model_improvements` | bool | <p>Indicates whether the bot uses accuracy improvements.
        <code>true</code> indicates that the bot is using the improvements,
      otherwise, <code>false</code>.</p> |
| `name` | String | <p>The name of the bot.</p> |
| `checksum` | String | <p>Checksum of the bot used to identify a specific revision of the
      bot's <code>$LATEST</code> version.</p> |
| `idle_session_ttl_in_seconds` | i64 | <p>The maximum time in seconds that Amazon Lex retains the data gathered in
      a conversation. For more information, see <a>PutBot</a>.</p> |
| `status` | String | <p>The status of the bot. </p>
         <p>When the status is <code>BUILDING</code> Amazon Lex is building the bot
      for testing and use.</p>
         <p>If the status of the bot is <code>READY_BASIC_TESTING</code>, you
      can test the bot using the exact utterances specified in the bot's
      intents. When the bot is ready for full testing or to run, the status is
        <code>READY</code>.</p>
         <p>If there was a problem with building the bot, the status is
        <code>FAILED</code> and the <code>failureReason</code> field explains
      why the bot did not build.</p>
         <p>If the bot was saved but not built, the status is
        <code>NOT_BUILT</code>.</p> |
| `nlu_intent_confidence_threshold` | f64 | <p>The score that determines where Amazon Lex inserts the
        <code>AMAZON.FallbackIntent</code>,
        <code>AMAZON.KendraSearchIntent</code>, or both when returning
      alternative intents in a <a href="https://docs.aws.amazon.com/lex/latest/dg/API_runtime_PostContent.html">PostContent</a> or
        <a href="https://docs.aws.amazon.com/lex/latest/dg/API_runtime_PostText.html">PostText</a> response.
        <code>AMAZON.FallbackIntent</code> is inserted if the confidence score
      for all intents is below this value.
        <code>AMAZON.KendraSearchIntent</code> is only inserted if it is
      configured for the bot.</p> |
| `voice_id` | String | <p>The Amazon Polly voice ID that Amazon Lex uses for voice interaction
      with the user. For more information, see <a>PutBot</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bot
bot = provider.lex_model_building_service.Bot {
    name = "value"  # <p>The name of the bot. The name is <i>not</i> case
      sensitive. </p>
    child_directed = "value"  # <p>For each Amazon Lex bot created with the Amazon Lex Model Building Service,
      you must specify whether your use of Amazon Lex is related to a website,
      program, or other application that is directed or targeted, in whole or in
      part, to children under age 13 and subject to the Children's Online
      Privacy Protection Act (COPPA) by specifying <code>true</code> or
        <code>false</code> in the <code>childDirected</code> field. By
      specifying <code>true</code> in the <code>childDirected</code> field, you
      confirm that your use of Amazon Lex <b>is</b> related
      to a website, program, or other application that is directed or targeted,
      in whole or in part, to children under age 13 and subject to COPPA. By
      specifying <code>false</code> in the <code>childDirected</code> field, you
      confirm that your use of Amazon Lex <b>is not</b>
      related to a website, program, or other application that is directed or
      targeted, in whole or in part, to children under age 13 and subject to
      COPPA. You may not specify a default value for the
        <code>childDirected</code> field that does not accurately reflect
      whether your use of Amazon Lex is related to a website, program, or other
      application that is directed or targeted, in whole or in part, to children
      under age 13 and subject to COPPA.</p>
         <p>If your use of Amazon Lex relates to a website, program, or other
      application that is directed in whole or in part, to children under age
      13, you must obtain any required verifiable parental consent under COPPA.
      For information regarding the use of Amazon Lex in connection with websites,
      programs, or other applications that are directed or targeted, in whole or
      in part, to children under age 13, see the <a href="https://aws.amazon.com/lex/faqs#data-security">Amazon Lex FAQ.</a>
         </p>
    locale = "value"  # <p> Specifies the target locale for the bot. Any intent used in the
      bot must be compatible with the locale of the bot. </p>
         <p>The default is <code>en-US</code>.</p>
}

# Access bot outputs
bot_id = bot.id
bot_version = bot.version
bot_child_directed = bot.child_directed
bot_detect_sentiment = bot.detect_sentiment
bot_description = bot.description
bot_intents = bot.intents
bot_last_updated_date = bot.last_updated_date
bot_created_date = bot.created_date
bot_locale = bot.locale
bot_failure_reason = bot.failure_reason
bot_clarification_prompt = bot.clarification_prompt
bot_abort_statement = bot.abort_statement
bot_enable_model_improvements = bot.enable_model_improvements
bot_name = bot.name
bot_checksum = bot.checksum
bot_idle_session_ttl_in_seconds = bot.idle_session_ttl_in_seconds
bot_status = bot.status
bot_nlu_intent_confidence_threshold = bot.nlu_intent_confidence_threshold
bot_voice_id = bot.voice_id
```

---


### Builtin_slot_types

BuiltinSlotTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If the response is truncated, the response includes a pagination
      token that you can use in your next request to fetch the next page of slot
      types.</p> |
| `slot_types` | Vec<String> | <p>An array of <code>BuiltInSlotTypeMetadata</code> objects, one entry
      for each slot type returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access builtin_slot_types outputs
builtin_slot_types_id = builtin_slot_types.id
builtin_slot_types_next_token = builtin_slot_types.next_token
builtin_slot_types_slot_types = builtin_slot_types.slot_types
```

---


### Intent_versions

IntentVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A pagination token for fetching the next page of intent versions.
      If the response to this call is truncated, Amazon Lex returns a pagination
      token in the response. To fetch the next page of versions, specify the
      pagination token in the next request. </p> |
| `intents` | Vec<String> | <p>An array of <code>IntentMetadata</code> objects, one for each
      numbered version of the intent plus one for the <code>$LATEST</code>
      version.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access intent_versions outputs
intent_versions_id = intent_versions.id
intent_versions_next_token = intent_versions.next_token
intent_versions_intents = intent_versions.intents
```

---


### Bot_versions

BotVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bots` | Vec<String> | <p>An array of <code>BotMetadata</code> objects, one for each numbered
      version of the bot plus one for the <code>$LATEST</code>
      version.</p> |
| `next_token` | String | <p>A pagination token for fetching the next page of bot versions. If
      the response to this call is truncated, Amazon Lex returns a pagination token
      in the response. To fetch the next page of versions, specify the
      pagination token in the next request. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access bot_versions outputs
bot_versions_id = bot_versions.id
bot_versions_bots = bot_versions.bots
bot_versions_next_token = bot_versions.next_token
```

---


### Migration

Migration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `v1_bot_locale` | String | <p>The locale of the Amazon Lex V1 bot migrated to Amazon Lex V2.</p> |
| `v2_bot_id` | String | <p>The unique identifier of the Amazon Lex V2 bot that the Amazon Lex V1 is being
      migrated to.</p> |
| `migration_strategy` | String | <p>The strategy used to conduct the migration.</p>
         <ul>
            <li>
               <p>
                  <code>CREATE_NEW</code> - Creates a new Amazon Lex V2 bot and migrates
          the Amazon Lex V1 bot to the new bot.</p>
            </li>
            <li>
               <p>
                  <code>UPDATE_EXISTING</code> - Overwrites the existing Amazon Lex V2 bot
        metadata and the locale being migrated. It doesn't change any other
        locales in the Amazon Lex V2 bot. If the locale doesn't exist, a new locale
        is created in the Amazon Lex V2 bot.</p>
            </li>
         </ul> |
| `v2_bot_role` | String | <p>The IAM role that Amazon Lex uses to run the Amazon Lex V2 bot.</p> |
| `alerts` | Vec<String> | <p>A list of alerts and warnings that indicate issues with the migration
      for the Amazon Lex V1 bot to Amazon Lex V2. You receive a warning when an Amazon Lex V1
      feature has a different implementation if Amazon Lex V2.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/migrate.html">Migrating a bot</a> in the <i>Amazon Lex V2
        developer guide</i>.</p> |
| `migration_timestamp` | String | <p>The date and time that the migration started.</p> |
| `migration_id` | String | <p>The unique identifier of the migration. This is the same as the
      identifier used when calling the <code>GetMigration</code>
      operation.</p> |
| `v1_bot_name` | String | <p>The name of the Amazon Lex V1 bot migrated to Amazon Lex V2.</p> |
| `v1_bot_version` | String | <p>The version of the Amazon Lex V1 bot migrated to Amazon Lex V2.</p> |
| `migration_status` | String | <p>Indicates the status of the migration. When the status is
        <code>COMPLETE</code> the migration is finished and the bot is available
      in Amazon Lex V2. There may be alerts and warnings that need to be resolved to
      complete the migration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access migration outputs
migration_id = migration.id
migration_v1_bot_locale = migration.v1_bot_locale
migration_v2_bot_id = migration.v2_bot_id
migration_migration_strategy = migration.migration_strategy
migration_v2_bot_role = migration.v2_bot_role
migration_alerts = migration.alerts
migration_migration_timestamp = migration.migration_timestamp
migration_migration_id = migration.migration_id
migration_v1_bot_name = migration.v1_bot_name
migration_v1_bot_version = migration.v1_bot_version
migration_migration_status = migration.migration_status
```

---


### Slot_type_version

SlotTypeVersion resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `checksum` | String |  | <p>Checksum for the <code>$LATEST</code> version of the slot type that
      you want to publish. If you specify a checksum and the
        <code>$LATEST</code> version of the slot type has a different checksum,
      Amazon Lex returns a <code>PreconditionFailedException</code> exception and
      doesn't publish the new version. If you don't specify a checksum, Amazon Lex
      publishes the <code>$LATEST</code> version.</p> |
| `name` | String | ✅ | <p>The name of the slot type that you want to create a new version
      for. The name is case sensitive. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create slot_type_version
slot_type_version = provider.lex_model_building_service.Slot_type_version {
    name = "value"  # <p>The name of the slot type that you want to create a new version
      for. The name is case sensitive. </p>
}

```

---


### Utterances

Utterances resource

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


### Bot_aliases

BotAliases resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bot_aliases` | Vec<String> | <p>An array of <code>BotAliasMetadata</code> objects, each describing
      a bot alias.</p> |
| `next_token` | String | <p>A pagination token for fetching next page of aliases. If the
      response to this call is truncated, Amazon Lex returns a pagination token in
      the response. To fetch the next page of aliases, specify the pagination
      token in the next request. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access bot_aliases outputs
bot_aliases_id = bot_aliases.id
bot_aliases_bot_aliases = bot_aliases.bot_aliases
bot_aliases_next_token = bot_aliases.next_token
```

---


### Migrations

Migrations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `migration_summaries` | Vec<String> | <p>An array of summaries for migrations from Amazon Lex V1 to Amazon Lex V2. To see
      details of the migration, use the <code>migrationId</code> from the
      summary in a call to the 
      operation.</p> |
| `next_token` | String | <p>If the response is truncated, it includes a pagination token that you
      can specify in your next request to fetch the next page of
      migrations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access migrations outputs
migrations_id = migrations.id
migrations_migration_summaries = migrations.migration_summaries
migrations_next_token = migrations.next_token
```

---


### Slot_type_versions

SlotTypeVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `slot_types` | Vec<String> | <p>An array of <code>SlotTypeMetadata</code> objects, one for each
      numbered version of the slot type plus one for the <code>$LATEST</code>
      version.</p> |
| `next_token` | String | <p>A pagination token for fetching the next page of slot type
      versions. If the response to this call is truncated, Amazon Lex returns a
      pagination token in the response. To fetch the next page of versions,
      specify the pagination token in the next request. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access slot_type_versions outputs
slot_type_versions_id = slot_type_versions.id
slot_type_versions_slot_types = slot_type_versions.slot_types
slot_type_versions_next_token = slot_type_versions.next_token
```

---


### Utterances_view

UtterancesView resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bot_name` | String | <p>The name of the bot for which utterance information was
      returned.</p> |
| `utterances` | Vec<String> | <p>An array of <a>UtteranceList</a> objects, each
      containing a list of <a>UtteranceData</a> objects describing
      the utterances that were processed by your bot. The response contains a
      maximum of 100 <code>UtteranceData</code> objects for each version. Amazon Lex
      returns the most frequent utterances received by the bot in the last 15
      days.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access utterances_view outputs
utterances_view_id = utterances_view.id
utterances_view_bot_name = utterances_view.bot_name
utterances_view_utterances = utterances_view.utterances
```

---


### Intent_version

IntentVersion resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the intent that you want to create a new version of.
      The name is case sensitive. </p> |
| `checksum` | String |  | <p>Checksum of the <code>$LATEST</code> version of the intent that
      should be used to create the new version. If you specify a checksum and
      the <code>$LATEST</code> version of the intent has a different checksum,
      Amazon Lex returns a <code>PreconditionFailedException</code> exception and
      doesn't publish a new version. If you don't specify a checksum, Amazon Lex
      publishes the <code>$LATEST</code> version.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create intent_version
intent_version = provider.lex_model_building_service.Intent_version {
    name = "value"  # <p>The name of the intent that you want to create a new version of.
      The name is case sensitive. </p>
}

```

---


### Slot_type

SlotType resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_version` | bool |  | <p>When set to <code>true</code> a new numbered version of the slot
      type is created. This is the same as calling the
        <code>CreateSlotTypeVersion</code> operation. If you do not specify
        <code>createVersion</code>, the default is <code>false</code>.</p> |
| `parent_slot_type_signature` | String |  | <p>The built-in slot type used as the parent of the slot type. When you
      define a parent slot type, the new slot type has all of the same
      configuration as the parent.</p>
         <p>Only <code>AMAZON.AlphaNumeric</code> is supported.</p> |
| `value_selection_strategy` | String |  | <p>Determines the slot resolution strategy that Amazon Lex uses to return
      slot type values. The field can be set to one of the following
      values:</p>
         <ul>
            <li>
               <p>
                  <code>ORIGINAL_VALUE</code> - Returns the value entered by the
          user, if the user value is similar to the slot value.</p>
            </li>
            <li>
               <p>
                  <code>TOP_RESOLUTION</code> - If there is a resolution list for
          the slot, return the first value in the resolution list as the slot
          type value. If there is no resolution list, null is
          returned.</p>
            </li>
         </ul>
         <p>If you don't specify the <code>valueSelectionStrategy</code>, the
      default is <code>ORIGINAL_VALUE</code>.</p> |
| `checksum` | String |  | <p>Identifies a specific revision of the <code>$LATEST</code>
      version.</p>
         <p>When you create a new slot type, leave the <code>checksum</code>
      field blank. If you specify a checksum you get a
        <code>BadRequestException</code> exception.</p>
         <p>When you want to update a slot type, set the <code>checksum</code>
      field to the checksum of the most recent revision of the
        <code>$LATEST</code> version. If you don't specify the <code>
        checksum</code> field, or if the checksum does not match the
        <code>$LATEST</code> version, you get a
        <code>PreconditionFailedException</code> exception.</p> |
| `slot_type_configurations` | Vec<String> |  | <p>Configuration information that extends the parent built-in slot type.
      The configuration is added to the settings for the parent slot
      type.</p> |
| `enumeration_values` | Vec<String> |  | <p>A list of <code>EnumerationValue</code> objects that defines the
      values that the slot type can take. Each value can have a list of
        <code>synonyms</code>, which are additional values that help train the
      machine learning model about the values that it resolves for a slot. </p>
         <p>A regular expression slot type doesn't require enumeration values.
      All other slot types require a list of enumeration values.</p>
         <p>When Amazon Lex resolves a slot value, it generates a resolution list
      that contains up to five possible values for the slot. If you are using a
      Lambda function, this resolution list is passed to the function. If you
      are not using a Lambda function you can choose to return the value that
      the user entered or the first value in the resolution list as the slot
      value. The <code>valueSelectionStrategy</code> field indicates the option
      to use. </p> |
| `description` | String |  | <p>A description of the slot type.</p> |
| `name` | String | ✅ | <p>The name of the slot type. The name is <i>not</i>
      case sensitive. </p>
         <p>The name can't match a built-in slot type name, or a built-in slot
      type name with "AMAZON." removed. For example, because there is a built-in
      slot type called <code>AMAZON.DATE</code>, you can't create a custom slot
      type called <code>DATE</code>.</p>
         <p>For a list of built-in slot types, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/slot-type-reference">Slot Type Reference</a> in the <i>Alexa Skills
        Kit</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_updated_date` | String | <p>The date that the slot type was updated. When you create a
      resource, the creation date and last update date are the same.</p> |
| `enumeration_values` | Vec<String> | <p>A list of <code>EnumerationValue</code> objects that defines the
      values that the slot type can take.</p> |
| `description` | String | <p>A description of the slot type.</p> |
| `checksum` | String | <p>Checksum of the <code>$LATEST</code> version of the slot
      type.</p> |
| `parent_slot_type_signature` | String | <p>The built-in slot type used as a parent for the slot type.</p> |
| `slot_type_configurations` | Vec<String> | <p>Configuration information that extends the parent built-in slot
      type.</p> |
| `version` | String | <p>The version of the slot type.</p> |
| `value_selection_strategy` | String | <p>The strategy that Amazon Lex uses to determine the value of the slot.
      For more information, see <a>PutSlotType</a>.</p> |
| `created_date` | String | <p>The date that the slot type was created.</p> |
| `name` | String | <p>The name of the slot type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create slot_type
slot_type = provider.lex_model_building_service.Slot_type {
    name = "value"  # <p>The name of the slot type. The name is <i>not</i>
      case sensitive. </p>
         <p>The name can't match a built-in slot type name, or a built-in slot
      type name with "AMAZON." removed. For example, because there is a built-in
      slot type called <code>AMAZON.DATE</code>, you can't create a custom slot
      type called <code>DATE</code>.</p>
         <p>For a list of built-in slot types, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/slot-type-reference">Slot Type Reference</a> in the <i>Alexa Skills
        Kit</i>.</p>
}

# Access slot_type outputs
slot_type_id = slot_type.id
slot_type_last_updated_date = slot_type.last_updated_date
slot_type_enumeration_values = slot_type.enumeration_values
slot_type_description = slot_type.description
slot_type_checksum = slot_type.checksum
slot_type_parent_slot_type_signature = slot_type.parent_slot_type_signature
slot_type_slot_type_configurations = slot_type.slot_type_configurations
slot_type_version = slot_type.version
slot_type_value_selection_strategy = slot_type.value_selection_strategy
slot_type_created_date = slot_type.created_date
slot_type_name = slot_type.name
```

---


### Builtin_intent

BuiltinIntent resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `signature` | String | <p>The unique identifier for a built-in intent.</p> |
| `supported_locales` | Vec<String> | <p>A list of locales that the intent supports.</p> |
| `slots` | Vec<String> | <p>An array of <code>BuiltinIntentSlot</code> objects, one entry for
      each slot type in the intent.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access builtin_intent outputs
builtin_intent_id = builtin_intent.id
builtin_intent_signature = builtin_intent.signature
builtin_intent_supported_locales = builtin_intent.supported_locales
builtin_intent_slots = builtin_intent.slots
```

---


### Bot_channel_associations

BotChannelAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A pagination token that fetches the next page of associations. If
      the response to this call is truncated, Amazon Lex returns a pagination token
      in the response. To fetch the next page of associations, specify the
      pagination token in the next request. </p> |
| `bot_channel_associations` | Vec<String> | <p>An array of objects, one for each association, that provides
      information about the Amazon Lex bot and its association with the channel.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access bot_channel_associations outputs
bot_channel_associations_id = bot_channel_associations.id
bot_channel_associations_next_token = bot_channel_associations.next_token
bot_channel_associations_bot_channel_associations = bot_channel_associations.bot_channel_associations
```

---


### Bot_version

BotVersion resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the bot that you want to create a new version of. The
      name is case sensitive. </p> |
| `checksum` | String |  | <p>Identifies a specific revision of the <code>$LATEST</code> version
      of the bot. If you specify a checksum and the <code>$LATEST</code> version
      of the bot has a different checksum, a
        <code>PreconditionFailedException</code> exception is returned and Amazon Lex
      doesn't publish a new version. If you don't specify a checksum, Amazon Lex
      publishes the <code>$LATEST</code> version.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bot_version
bot_version = provider.lex_model_building_service.Bot_version {
    name = "value"  # <p>The name of the bot that you want to create a new version of. The
      name is case sensitive. </p>
}

```

---


### Bots

Bots resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If the response is truncated, it includes a pagination token that
      you can specify in your next request to fetch the next page of bots.
    </p> |
| `bots` | Vec<String> | <p>An array of <code>botMetadata</code> objects, with one entry for
      each bot. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access bots outputs
bots_id = bots.id
bots_next_token = bots.next_token
bots_bots = bots.bots
```

---


### Export

Export resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `export_type` | String | <p>The format of the exported data.</p> |
| `url` | String | <p>An S3 pre-signed URL that provides the location of the exported
      resource. The exported resource is a ZIP archive that contains the
      exported resource in JSON format. The structure of the archive may change.
      Your code should not rely on the archive structure.</p> |
| `version` | String | <p>The version of the bot being exported.</p> |
| `failure_reason` | String | <p>If <code>status</code> is <code>FAILED</code>, Amazon Lex provides the
      reason that it failed to export the resource.</p> |
| `name` | String | <p>The name of the bot being exported.</p> |
| `resource_type` | String | <p>The type of the exported resource.</p> |
| `export_status` | String | <p>The status of the export. </p>
         <ul>
            <li>
               <p>
                  <code>IN_PROGRESS</code> - The export is in progress.</p>
            </li>
            <li>
               <p>
                  <code>READY</code> - The export is complete.</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code> - The export could not be
          completed.</p>
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

# Access export outputs
export_id = export.id
export_export_type = export.export_type
export_url = export.url
export_version = export.version
export_failure_reason = export.failure_reason
export_name = export.name
export_resource_type = export.resource_type
export_export_status = export.export_status
```

---


### Intents

Intents resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `intents` | Vec<String> | <p>An array of <code>Intent</code> objects. For more information, see
        <a>PutBot</a>.</p> |
| `next_token` | String | <p>If the response is truncated, the response includes a pagination
      token that you can specify in your next request to fetch the next page of
      intents. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access intents outputs
intents_id = intents.id
intents_intents = intents.intents
intents_next_token = intents.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple slot_types resources
slot_types_0 = provider.lex_model_building_service.Slot_types {
}
slot_types_1 = provider.lex_model_building_service.Slot_types {
}
slot_types_2 = provider.lex_model_building_service.Slot_types {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    slot_types = provider.lex_model_building_service.Slot_types {
    }
```

---

## Related Documentation

- [AWS Lex_model_building_service Documentation](https://docs.aws.amazon.com/lex_model_building_service/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
