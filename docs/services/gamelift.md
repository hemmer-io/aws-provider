# Gamelift Service



**Resources**: 47

---

## Overview

The gamelift service provides access to 47 resource types:

- [Fleet_events](#fleet_events) [R]
- [Game_server_instances](#game_server_instances) [R]
- [Game_session_placement](#game_session_placement) [R]
- [Game_session](#game_session) [CU]
- [Script](#script) [CRUD]
- [Fleet_utilization](#fleet_utilization) [R]
- [Fleet_attributes](#fleet_attributes) [RU]
- [Matchmaking_configurations](#matchmaking_configurations) [R]
- [Runtime_configuration](#runtime_configuration) [RU]
- [Fleet_location_utilization](#fleet_location_utilization) [R]
- [Game_session_details](#game_session_details) [R]
- [Game_session_queue](#game_session_queue) [CUD]
- [Fleet_location_attributes](#fleet_location_attributes) [R]
- [Instances](#instances) [R]
- [Alias](#alias) [CRUD]
- [Fleet_location_capacity](#fleet_location_capacity) [R]
- [Vpc_peering_connection](#vpc_peering_connection) [CD]
- [Container_group_definition](#container_group_definition) [CRUD]
- [Compute_auth_token](#compute_auth_token) [R]
- [Compute](#compute) [R]
- [Player_sessions](#player_sessions) [CR]
- [Instance_access](#instance_access) [R]
- [Fleet_locations](#fleet_locations) [CD]
- [Matchmaking](#matchmaking) [R]
- [Game_session_queues](#game_session_queues) [R]
- [Game_session_log_url](#game_session_log_url) [R]
- [Location](#location) [CD]
- [Matchmaking_rule_set](#matchmaking_rule_set) [CD]
- [Game_server_group](#game_server_group) [CRUD]
- [Game_server](#game_server) [RU]
- [Matchmaking_configuration](#matchmaking_configuration) [CUD]
- [Fleet_deployment](#fleet_deployment) [R]
- [Matchmaking_rule_sets](#matchmaking_rule_sets) [R]
- [Scaling_policies](#scaling_policies) [R]
- [Scaling_policy](#scaling_policy) [CD]
- [Fleet_capacity](#fleet_capacity) [RU]
- [Game_sessions](#game_sessions) [R]
- [Vpc_peering_connections](#vpc_peering_connections) [R]
- [Vpc_peering_authorizations](#vpc_peering_authorizations) [R]
- [Build](#build) [CRUD]
- [Container_fleet](#container_fleet) [CRUD]
- [Ec2_instance_limits](#ec2_instance_limits) [R]
- [Vpc_peering_authorization](#vpc_peering_authorization) [CD]
- [Fleet_port_settings](#fleet_port_settings) [RU]
- [Compute_access](#compute_access) [R]
- [Player_session](#player_session) [C]
- [Fleet](#fleet) [CD]

---

## Resources


### Fleet_events

FleetEvents resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `events` | Vec<String> | <p>A collection of objects containing event log entries for the specified fleet.</p> |
| `next_token` | String | <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fleet_events outputs
fleet_events_id = fleet_events.id
fleet_events_events = fleet_events.events
fleet_events_next_token = fleet_events.next_token
```

---


### Game_server_instances

GameServerInstances resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p> |
| `game_server_instances` | Vec<String> | <p>The collection of requested game server instances.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access game_server_instances outputs
game_server_instances_id = game_server_instances.id
game_server_instances_next_token = game_server_instances.next_token
game_server_instances_game_server_instances = game_server_instances.game_server_instances
```

---


### Game_session_placement

GameSessionPlacement resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `game_session_placement` | String | <p>Object that describes the requested game session placement.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access game_session_placement outputs
game_session_placement_id = game_session_placement.id
game_session_placement_game_session_placement = game_session_placement.game_session_placement
```

---


### Game_session

GameSession resource

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `creator_id` | String |  | <p>A unique identifier for a player or entity creating the game session. </p>
         <p>If you add a resource creation limit policy to a fleet, the
                <code>CreateGameSession</code> operation requires a <code>CreatorId</code>. Amazon GameLift Servers
            limits the number of game session creation requests with the same <code>CreatorId</code>
            in a specified time period.</p>
         <p>If you your fleet doesn't have a resource creation limit policy and you provide a
                <code>CreatorId</code> in your <code>CreateGameSession</code> requests, Amazon GameLift Servers
            limits requests to one request per <code>CreatorId</code> per second.</p>
         <p>To not limit <code>CreateGameSession</code> requests with the same
                <code>CreatorId</code>, don't provide a <code>CreatorId</code> in your
                <code>CreateGameSession</code> request.</p> |
| `game_session_data` | String |  | <p>A set of custom game session properties, formatted as a single string value. This data is passed to a game server process with a request to start a new game session. For more information, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a game session</a>.</p> |
| `game_session_id` | String |  | <p>
            <i>This parameter is deprecated. Use <code>IdempotencyToken</code>
                instead.</i>
         </p>
         <p>Custom string that uniquely identifies a request for a new game session. Maximum token
            length is 48 characters. If provided, this string is included in the new game session's
            ID.</p> |
| `idempotency_token` | String |  | <p>Custom string that uniquely identifies the new game session request. This is useful
            for ensuring that game session requests with the same idempotency token are processed
            only once. Subsequent requests with the same string return the original
                <code>GameSession</code> object, with an updated status. Maximum token length is 48
            characters. If provided, this string is included in the new game session's ID.
            A game session ARN has the following format: 
    <code>arn:aws:gamelift:<location>::gamesession/<fleet ID>/<custom ID string or idempotency token></code>. Idempotency tokens remain in use for 30 days after a game session has ended;
            game session objects are retained for this time period and then deleted.</p> |
| `maximum_player_session_count` | i64 | ✅ | <p>The maximum number of players that can be connected simultaneously to the game session.</p> |
| `name` | String |  | <p>A descriptive label that is associated with a game session. Session names do not need to be unique.</p> |
| `alias_id` | String |  | <p>A unique identifier for the alias associated with the fleet to create a game session in. You can use either the
            alias ID or ARN value. Each request must reference either a fleet ID or alias ID, but
            not both.</p> |
| `location` | String |  | <p>A fleet's remote location to place the new game session in. If this parameter is not
            set, the new game session is placed in the fleet's home Region. Specify a remote
            location with an Amazon Web Services Region code such as <code>us-west-2</code>. When using an
            Anywhere fleet, this parameter is required and must be set to the Anywhere fleet's
            custom location.</p> |
| `fleet_id` | String |  | <p>A unique identifier for the fleet to create a game session in. You can use either the fleet ID or ARN value. Each
            request must reference either a fleet ID or alias ID, but not both.</p> |
| `game_properties` | Vec<String> |  | <p>A set of key-value pairs that can store custom data in a game session.
  For example: <code>{"Key": "difficulty", "Value": "novice"}</code>.
          For an example, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-client-api.html#game-properties-create">Create a game session with custom properties</a>.                    
        </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create game_session
game_session = provider.gamelift.Game_session {
    maximum_player_session_count = "value"  # <p>The maximum number of players that can be connected simultaneously to the game session.</p>
}

```

---


### Script

Script resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `storage_location` | String |  | <p>The location of the Amazon S3 bucket where a zipped file containing your Realtime scripts is
            stored. The storage location must specify the Amazon S3 bucket name, the zip file name (the
            "key"), and a role ARN that allows Amazon GameLift Servers to access the Amazon S3 storage location. The S3
            bucket must be in the same Region where you want to create a new script. By default,
            Amazon GameLift Servers uploads the latest version of the zip file; if you have S3 object versioning
            turned on, you can use the <code>ObjectVersion</code> parameter to specify an earlier
            version. </p> |
| `version` | String |  | <p>Version information that is associated with a build or script. Version strings do not need to be unique. You can use <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_UpdateScript.html">UpdateScript</a> to
            change this value later. </p> |
| `name` | String |  | <p>A descriptive label that is associated with a script. Script names do not need to be unique. You can use <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_UpdateScript.html">UpdateScript</a> to
            change this value later. </p> |
| `zip_file` | String |  | <p>A data object containing your Realtime scripts and dependencies as a zip file. The zip
            file can have one or multiple files. Maximum size of a zip file is 5 MB.</p>
         <p>When using the Amazon Web Services CLI tool to create a script, this parameter is set to the zip
            file name. It must be prepended with the string "fileb://" to indicate that the file
            data is a binary object. For example: <code>--zip-file
                fileb://myRealtimeScript.zip</code>.</p> |
| `tags` | Vec<String> |  | <p>A list of labels to assign to the new script resource. Tags are developer-defined
            key-value pairs. Tagging Amazon Web Services resources are useful for resource management, access
            management and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging Amazon Web Services Resources</a> in the
                <i>Amazon Web Services General Reference</i>. Once the resource is created, you can
            use <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_TagResource.html">TagResource</a>, <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_UntagResource.html">UntagResource</a>, and
                <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_ListTagsForResource.html">ListTagsForResource</a> to add, remove, and view tags. The maximum tag limit
            may be lower than stated. See the Amazon Web Services General Reference for actual tagging
            limits.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `script` | String | <p>A set of properties describing the requested script.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create script
script = provider.gamelift.Script {
}

# Access script outputs
script_id = script.id
script_script = script.script
```

---


### Fleet_utilization

FleetUtilization resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p> |
| `fleet_utilization` | Vec<String> | <p>A collection of objects containing utilization information for each requested fleet
            ID. Utilization objects are returned only for fleets that currently exist.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fleet_utilization outputs
fleet_utilization_id = fleet_utilization.id
fleet_utilization_next_token = fleet_utilization.next_token
fleet_utilization_fleet_utilization = fleet_utilization.fleet_utilization
```

---


### Fleet_attributes

FleetAttributes resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_creation_limit_policy` | String |  | <p>Policy settings that limit the number of game sessions an individual player can create
            over a span of time. </p> |
| `new_game_session_protection_policy` | String |  | <p>The game session protection policy to apply to all new game sessions created in this
            fleet. Game sessions that already exist are not affected. You can set protection for
            individual game sessions using <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_UpdateGameSession.html">UpdateGameSession</a> .</p>
         <ul>
            <li>
               <p>
                  <b>NoProtection</b> -- The game session can be
                    terminated during a scale-down event.</p>
            </li>
            <li>
               <p>
                  <b>FullProtection</b> -- If the game session is in an
                        <code>ACTIVE</code> status, it cannot be terminated during a scale-down
                    event.</p>
            </li>
         </ul> |
| `fleet_id` | String | ✅ | <p>A unique identifier for the fleet to update attribute metadata for. You can use either the fleet ID or ARN
            value.</p> |
| `name` | String |  | <p>A descriptive label that is associated with a fleet. Fleet names do not need to be unique.</p> |
| `description` | String |  | <p>A human-readable description of a fleet.</p> |
| `metric_groups` | Vec<String> |  | <p>The name of a metric group to add this fleet to. Use a metric group in Amazon
            CloudWatch to aggregate the metrics from multiple fleets. Provide an existing metric
            group name, or create a new metric group by providing a new name. A fleet can only be in
            one metric group at a time.</p> |
| `anywhere_configuration` | String |  | <p>Amazon GameLift Servers Anywhere configuration options.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fleet_attributes` | Vec<String> | <p>A collection of objects containing attribute metadata for each requested fleet ID.
            Attribute objects are returned only for fleets that currently exist.</p> |
| `next_token` | String | <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fleet_attributes outputs
fleet_attributes_id = fleet_attributes.id
fleet_attributes_fleet_attributes = fleet_attributes.fleet_attributes
fleet_attributes_next_token = fleet_attributes.next_token
```

---


### Matchmaking_configurations

MatchmakingConfigurations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configurations` | Vec<String> | <p>A collection of requested matchmaking configurations.</p> |
| `next_token` | String | <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access matchmaking_configurations outputs
matchmaking_configurations_id = matchmaking_configurations.id
matchmaking_configurations_configurations = matchmaking_configurations.configurations
matchmaking_configurations_next_token = matchmaking_configurations.next_token
```

---


### Runtime_configuration

RuntimeConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `runtime_configuration` | String | ✅ | <p>Instructions for launching server processes on fleet computes. Server processes run
            either a custom game build executable or a Amazon GameLift Servers Realtime script. The runtime configuration lists
            the types of server processes to run, how to launch them, and the number of processes to
            run concurrently.</p> |
| `fleet_id` | String | ✅ | <p>A unique identifier for the fleet to update runtime configuration for. You can use either the fleet ID or ARN
            value.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `runtime_configuration` | String | <p>Instructions that describe how server processes are launched and maintained on
            computes in the fleet.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access runtime_configuration outputs
runtime_configuration_id = runtime_configuration.id
runtime_configuration_runtime_configuration = runtime_configuration.runtime_configuration
```

---


### Fleet_location_utilization

FleetLocationUtilization resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fleet_utilization` | String | <p>Utilization information for the requested fleet location. Utilization objects are
            returned only for fleets and locations that currently exist.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fleet_location_utilization outputs
fleet_location_utilization_id = fleet_location_utilization.id
fleet_location_utilization_fleet_utilization = fleet_location_utilization.fleet_utilization
```

---


### Game_session_details

GameSessionDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p> |
| `game_session_details` | Vec<String> | <p>A collection of properties for each game session that matches the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access game_session_details outputs
game_session_details_id = game_session_details.id
game_session_details_next_token = game_session_details.next_token
game_session_details_game_session_details = game_session_details.game_session_details
```

---


### Game_session_queue

GameSessionQueue resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destinations` | Vec<String> |  | <p>A list of fleets and/or fleet aliases that can be used to fulfill game session placement requests in the queue. 
    Destinations are identified by either a fleet ARN or a fleet alias ARN, and are listed in order of placement preference.</p> |
| `name` | String | ✅ | <p>A descriptive label that is associated with game session queue. Queue names must be unique within each Region.</p> |
| `timeout_in_seconds` | i64 |  | <p>The maximum time, in seconds, that a new game session placement request remains in the queue. When a request exceeds this time, the game session placement changes to a <code>TIMED_OUT</code> status. If you don't specify a request timeout, the queue uses a default value.</p> |
| `custom_event_data` | String |  | <p>Information to be added to all events that are related to this game session
            queue.</p> |
| `notification_target` | String |  | <p>An SNS topic ARN that is set up to receive game session placement notifications. See
                <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/queue-notification.html"> Setting up
                notifications for game session placement</a>.</p> |
| `player_latency_policies` | Vec<String> |  | <p>A set of policies that enforce a sliding cap on player latency when processing game sessions placement requests. 
	Use multiple policies to gradually relax the cap over time if Amazon GameLift Servers can't make a placement.
	    Policies are evaluated in order starting with the lowest maximum latency value.</p> |
| `filter_configuration` | String |  | <p>A list of locations where a queue is allowed to place new game sessions. Locations 
            are specified in the form of Amazon Web Services Region codes, such as <code>us-west-2</code>. If this parameter is 
            not set, game sessions can be placed in any queue location. </p> |
| `priority_configuration` | String |  | <p>Custom settings to use when prioritizing destinations and locations for game session placements. This 
            configuration replaces the FleetIQ default prioritization process. Priority types that are not explicitly 
            named will be automatically applied at the end of the prioritization process. </p> |
| `tags` | Vec<String> |  | <p>A list of labels to assign to the new game session queue resource. Tags are
            developer-defined key-value pairs. Tagging Amazon Web Services resources are useful for resource
            management, access management and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging Amazon Web Services
                Resources</a> in the <i>Amazon Web Services General Reference</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create game_session_queue
game_session_queue = provider.gamelift.Game_session_queue {
    name = "value"  # <p>A descriptive label that is associated with game session queue. Queue names must be unique within each Region.</p>
}

```

---


### Fleet_location_attributes

FleetLocationAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fleet_id` | String | <p>A unique identifier for the fleet that location attributes were requested for.</p> |
| `next_token` | String | <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p> |
| `location_attributes` | Vec<String> | <p> Location-specific information on the requested fleet's remote locations.</p> |
| `fleet_arn` | String | <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) that is assigned to a Amazon GameLift Servers fleet resource and uniquely identifies it. ARNs are unique across all Regions. Format is <code>arn:aws:gamelift:<region>::fleet/fleet-a1234567-b8c9-0d1e-2fa3-b45c6d7e8912</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fleet_location_attributes outputs
fleet_location_attributes_id = fleet_location_attributes.id
fleet_location_attributes_fleet_id = fleet_location_attributes.fleet_id
fleet_location_attributes_next_token = fleet_location_attributes.next_token
fleet_location_attributes_location_attributes = fleet_location_attributes.location_attributes
fleet_location_attributes_fleet_arn = fleet_location_attributes.fleet_arn
```

---


### Instances

Instances resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instances` | Vec<String> | <p>A collection of objects containing properties for each instance returned.</p> |
| `next_token` | String | <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instances outputs
instances_id = instances.id
instances_instances = instances.instances
instances_next_token = instances.next_token
```

---


### Alias

Alias resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `routing_strategy` | String | ✅ | <p>The routing configuration, including routing type and fleet target, for the alias.
        </p> |
| `description` | String |  | <p>A human-readable description of the alias.</p> |
| `name` | String | ✅ | <p>A descriptive label that is associated with an alias. Alias names do not need to be unique.</p> |
| `tags` | Vec<String> |  | <p>A list of labels to assign to the new alias resource. Tags are developer-defined
            key-value pairs. Tagging Amazon Web Services resources are useful for resource management, access
            management and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging Amazon Web Services Resources</a> in the
                <i>Amazon Web Services General Reference</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `alias` | String | <p>The requested alias resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create alias
alias = provider.gamelift.Alias {
    routing_strategy = "value"  # <p>The routing configuration, including routing type and fleet target, for the alias.
        </p>
    name = "value"  # <p>A descriptive label that is associated with an alias. Alias names do not need to be unique.</p>
}

# Access alias outputs
alias_id = alias.id
alias_alias = alias.alias
```

---


### Fleet_location_capacity

FleetLocationCapacity resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fleet_capacity` | String | <p>Resource capacity information for the requested fleet location. Capacity objects are
            returned only for fleets and locations that currently exist. Changes in desired instance
            value can take up to 1 minute to be reflected.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fleet_location_capacity outputs
fleet_location_capacity_id = fleet_location_capacity.id
fleet_location_capacity_fleet_capacity = fleet_location_capacity.fleet_capacity
```

---


### Vpc_peering_connection

VpcPeeringConnection resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fleet_id` | String | ✅ | <p>A unique identifier for the fleet. You can use either the fleet ID or ARN value. This tells Amazon GameLift Servers which GameLift
            VPC to peer with. </p> |
| `peer_vpc_aws_account_id` | String | ✅ | <p>A unique identifier for the Amazon Web Services account with the VPC that you want to peer your
            Amazon GameLift Servers fleet with. You can find your Account ID in the Amazon Web Services Management Console under account
            settings.</p> |
| `peer_vpc_id` | String | ✅ | <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift Servers fleet. The
            VPC must be in the same Region as your fleet. To look up a VPC ID, use the 
            <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the Amazon Web Services Management Console. 
            Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Servers Fleets</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_peering_connection
vpc_peering_connection = provider.gamelift.Vpc_peering_connection {
    fleet_id = "value"  # <p>A unique identifier for the fleet. You can use either the fleet ID or ARN value. This tells Amazon GameLift Servers which GameLift
            VPC to peer with. </p>
    peer_vpc_aws_account_id = "value"  # <p>A unique identifier for the Amazon Web Services account with the VPC that you want to peer your
            Amazon GameLift Servers fleet with. You can find your Account ID in the Amazon Web Services Management Console under account
            settings.</p>
    peer_vpc_id = "value"  # <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift Servers fleet. The
            VPC must be in the same Region as your fleet. To look up a VPC ID, use the 
            <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the Amazon Web Services Management Console. 
            Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Servers Fleets</a>.</p>
}

```

---


### Container_group_definition

ContainerGroupDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `operating_system` | String | ✅ | <p>The platform that all containers in the group use. Containers in a group must run on the
      same operating system.</p>
         <p>Default value: <code>AMAZON_LINUX_2023</code>
         </p>
         <note>
            <p>Amazon Linux 2 (AL2) will reach end of support on 6/30/2025. See more details in the <a href="http://aws.amazon.com/amazon-linux-2/faqs/">Amazon Linux 2 FAQs</a>. For game
    servers that are hosted on AL2 and use server SDK version 4.x for Amazon GameLift Servers, first update the game
        server build to server SDK 5.x, and then deploy to AL2023 instances. See <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/reference-serversdk5-migration.html"> Migrate to
          server SDK version 5.</a>
            </p>
         </note> |
| `tags` | Vec<String> |  | <p>A list of labels to assign to the container group definition resource. Tags are
      developer-defined key-value pairs. Tagging Amazon Web Services resources are useful for resource management,
      access management and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging Amazon Web Services Resources</a> in the
        <i>Amazon Web Services General Reference</i>. </p> |
| `version_description` | String |  | <p>A description for the initial version of this container group definition. </p> |
| `container_group_type` | String |  | <p>The type of container group being defined. Container group type determines how Amazon GameLift Servers 
      deploys the container group on each fleet instance.</p>
         <p>Default value: <code>GAME_SERVER</code>
         </p> |
| `name` | String | ✅ | <p>A descriptive identifier for the container group definition. The name value must be unique in an Amazon Web Services Region.</p> |
| `total_memory_limit_mebibytes` | i64 | ✅ | <p>The maximum amount of memory (in MiB) to allocate to the container group. All containers in
      the group share this memory. If you specify memory limits for an individual container, the
      total value must be greater than any individual container's memory limit.</p>
         <p>Default value: 1024</p> |
| `total_vcpu_limit` | f64 | ✅ | <p>The maximum amount of vCPU units to allocate to the container group (1 vCPU is equal to 1024
      CPU units). All containers in the group share this memory. If you specify vCPU limits for
      individual containers, the total value must be equal to or greater than the sum of the CPU
      limits for all containers in the group.</p>
         <p>Default value: 1</p> |
| `support_container_definitions` | Vec<String> |  | <p>One or more definition for support containers in this group. You can define a support
      container in any type of container group. You can pass in your container definitions as a JSON
      file.</p> |
| `game_server_container_definition` | String |  | <p>The definition for the game server container in this group. Define a game server container
      only when the container group type is <code>GAME_SERVER</code>. Game server containers specify
      a container image with your game server build. You can pass in your container definitions as a
      JSON file.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `container_group_definition` | String | <p>The properties of the requested container group definition resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create container_group_definition
container_group_definition = provider.gamelift.Container_group_definition {
    operating_system = "value"  # <p>The platform that all containers in the group use. Containers in a group must run on the
      same operating system.</p>
         <p>Default value: <code>AMAZON_LINUX_2023</code>
         </p>
         <note>
            <p>Amazon Linux 2 (AL2) will reach end of support on 6/30/2025. See more details in the <a href="http://aws.amazon.com/amazon-linux-2/faqs/">Amazon Linux 2 FAQs</a>. For game
    servers that are hosted on AL2 and use server SDK version 4.x for Amazon GameLift Servers, first update the game
        server build to server SDK 5.x, and then deploy to AL2023 instances. See <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/reference-serversdk5-migration.html"> Migrate to
          server SDK version 5.</a>
            </p>
         </note>
    name = "value"  # <p>A descriptive identifier for the container group definition. The name value must be unique in an Amazon Web Services Region.</p>
    total_memory_limit_mebibytes = "value"  # <p>The maximum amount of memory (in MiB) to allocate to the container group. All containers in
      the group share this memory. If you specify memory limits for an individual container, the
      total value must be greater than any individual container's memory limit.</p>
         <p>Default value: 1024</p>
    total_vcpu_limit = "value"  # <p>The maximum amount of vCPU units to allocate to the container group (1 vCPU is equal to 1024
      CPU units). All containers in the group share this memory. If you specify vCPU limits for
      individual containers, the total value must be equal to or greater than the sum of the CPU
      limits for all containers in the group.</p>
         <p>Default value: 1</p>
}

# Access container_group_definition outputs
container_group_definition_id = container_group_definition.id
container_group_definition_container_group_definition = container_group_definition.container_group_definition
```

---


### Compute_auth_token

ComputeAuthToken resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expiration_timestamp` | String | <p>The amount of time until the authentication token is no longer valid.</p> |
| `auth_token` | String | <p>A valid temporary authentication token.</p> |
| `compute_name` | String | <p>The name of the compute resource that the authentication token is issued to.</p> |
| `compute_arn` | String | <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) that is assigned to an Amazon GameLift Servers compute resource and uniquely identifies it.
            ARNs are unique across all Regions. Format is
                <code>arn:aws:gamelift:<region>::compute/compute-a1234567-b8c9-0d1e-2fa3-b45c6d7e8912</code>.</p> |
| `fleet_arn` | String | <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) that is assigned to a Amazon GameLift Servers fleet resource and uniquely identifies it. ARNs are unique across all Regions. Format is <code>arn:aws:gamelift:<region>::fleet/fleet-a1234567-b8c9-0d1e-2fa3-b45c6d7e8912</code>.</p> |
| `fleet_id` | String | <p>A unique identifier for the fleet that the compute is registered to.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access compute_auth_token outputs
compute_auth_token_id = compute_auth_token.id
compute_auth_token_expiration_timestamp = compute_auth_token.expiration_timestamp
compute_auth_token_auth_token = compute_auth_token.auth_token
compute_auth_token_compute_name = compute_auth_token.compute_name
compute_auth_token_compute_arn = compute_auth_token.compute_arn
compute_auth_token_fleet_arn = compute_auth_token.fleet_arn
compute_auth_token_fleet_id = compute_auth_token.fleet_id
```

---


### Compute

Compute resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `compute` | String | <p>The set of properties for the requested compute resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access compute outputs
compute_id = compute.id
compute_compute = compute.compute
```

---


### Player_sessions

PlayerSessions resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `player_ids` | Vec<String> | ✅ | <p>List of unique identifiers for the players to be added.</p> |
| `game_session_id` | String | ✅ | <p>A unique identifier for the game session to add players to.</p> |
| `player_data_map` | HashMap<String, String> |  | <p>Map of string pairs, each specifying a player ID and a set of developer-defined
            information related to the player. Amazon GameLift Servers does not use this data, so it can be formatted
            as needed for use in the game. Any player data strings for player IDs that are not
            included in the <code>PlayerIds</code> parameter are ignored. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `player_sessions` | Vec<String> | <p>A collection of objects containing properties for each player session that matches the
            request.</p> |
| `next_token` | String | <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create player_sessions
player_sessions = provider.gamelift.Player_sessions {
    player_ids = "value"  # <p>List of unique identifiers for the players to be added.</p>
    game_session_id = "value"  # <p>A unique identifier for the game session to add players to.</p>
}

# Access player_sessions outputs
player_sessions_id = player_sessions.id
player_sessions_player_sessions = player_sessions.player_sessions
player_sessions_next_token = player_sessions.next_token
```

---


### Instance_access

InstanceAccess resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_access` | String | <p>The connection information for a fleet instance, including IP address and access
            credentials.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_access outputs
instance_access_id = instance_access.id
instance_access_instance_access = instance_access.instance_access
```

---


### Fleet_locations

FleetLocations resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fleet_id` | String | ✅ | <p>A unique identifier for the fleet to add locations to. You can use either the fleet ID or ARN value.</p> |
| `locations` | Vec<String> | ✅ | <p>A list of locations to deploy additional instances to and manage as part of the fleet.
            You can add any Amazon GameLift Servers-supported Amazon Web Services Region as a remote location, in the form of an
            Amazon Web Services Region code such as <code>us-west-2</code>. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create fleet_locations
fleet_locations = provider.gamelift.Fleet_locations {
    fleet_id = "value"  # <p>A unique identifier for the fleet to add locations to. You can use either the fleet ID or ARN value.</p>
    locations = "value"  # <p>A list of locations to deploy additional instances to and manage as part of the fleet.
            You can add any Amazon GameLift Servers-supported Amazon Web Services Region as a remote location, in the form of an
            Amazon Web Services Region code such as <code>us-west-2</code>. </p>
}

```

---


### Matchmaking

Matchmaking resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ticket_list` | Vec<String> | <p>A collection of existing matchmaking ticket objects matching the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access matchmaking outputs
matchmaking_id = matchmaking.id
matchmaking_ticket_list = matchmaking.ticket_list
```

---


### Game_session_queues

GameSessionQueues resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `game_session_queues` | Vec<String> | <p>A collection of objects that describe the requested game session queues.</p> |
| `next_token` | String | <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access game_session_queues outputs
game_session_queues_id = game_session_queues.id
game_session_queues_game_session_queues = game_session_queues.game_session_queues
game_session_queues_next_token = game_session_queues.next_token
```

---


### Game_session_log_url

GameSessionLogUrl resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pre_signed_url` | String | <p>Location of the requested game session logs, available for download. This URL is valid
            for 15 minutes, after which S3 will reject any download request using this URL. You can
            request a new URL any time within the 14-day period that the logs are retained.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access game_session_log_url outputs
game_session_log_url_id = game_session_log_url.id
game_session_log_url_pre_signed_url = game_session_log_url.pre_signed_url
```

---


### Location

Location resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of labels to assign to the new resource. Tags are developer-defined key-value
            pairs. Tagging Amazon Web Services resources are useful for resource management, access management,
            and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging Amazon Web Services Resources</a> in the
                <i>Amazon Web Services General Rareference</i>.</p> |
| `location_name` | String | ✅ | <p>A descriptive name for the custom location.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create location
location = provider.gamelift.Location {
    location_name = "value"  # <p>A descriptive name for the custom location.</p>
}

```

---


### Matchmaking_rule_set

MatchmakingRuleSet resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of labels to assign to the new matchmaking rule set resource. Tags are
            developer-defined key-value pairs. Tagging Amazon Web Services resources are useful for resource
            management, access management and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging Amazon Web Services
                Resources</a> in the <i>Amazon Web Services General Reference</i>.</p> |
| `rule_set_body` | String | ✅ | <p>A collection of matchmaking rules, formatted as a JSON string. Comments are not
            allowed in JSON, but most elements support a description field.</p> |
| `name` | String | ✅ | <p>A unique identifier for the matchmaking rule set. A matchmaking configuration identifies the rule set it uses by this name
            value. Note that the rule set name is different from the optional <code>name</code>
            field in the rule set body.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create matchmaking_rule_set
matchmaking_rule_set = provider.gamelift.Matchmaking_rule_set {
    rule_set_body = "value"  # <p>A collection of matchmaking rules, formatted as a JSON string. Comments are not
            allowed in JSON, but most elements support a description field.</p>
    name = "value"  # <p>A unique identifier for the matchmaking rule set. A matchmaking configuration identifies the rule set it uses by this name
            value. Note that the rule set name is different from the optional <code>name</code>
            field in the rule set body.</p>
}

```

---


### Game_server_group

GameServerGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) for an IAM role that
            allows Amazon GameLift Servers to access your Amazon EC2 Auto Scaling groups.</p> |
| `launch_template` | String | ✅ | <p>The Amazon EC2 launch template that contains configuration settings and game server code to
            be deployed to all instances in the game server group. You can specify the template
            using either the template name or ID. For help with creating a launch template, see
                <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-launch-template.html">Creating a Launch
                Template for an Auto Scaling Group</a> in the <i>Amazon Elastic Compute Cloud Auto Scaling
                User Guide</i>. After the Auto Scaling group is created, update this value
            directly in the Auto Scaling group using the Amazon Web Services console or APIs.</p>
         <note>
            <p>If you specify network interfaces in your launch template, you must explicitly set
                the property <code>AssociatePublicIpAddress</code> to "true". If no network
                interface is specified in the launch template, Amazon GameLift Servers FleetIQ uses your account's default
                VPC.</p>
         </note> |
| `game_server_protection_policy` | String |  | <p>A flag that indicates whether instances in the game server group are protected 
            from early termination. Unprotected instances that have active game servers running might 
            be terminated during a scale-down event, causing players to be dropped from the game. 
            Protected instances cannot be terminated while there are active game servers running except 
            in the event of a forced game server group deletion (see ). An exception to this is with Spot 
            Instances, which can be terminated by Amazon Web Services regardless of protection status. This property is set to <code>NO_PROTECTION</code> by default.</p> |
| `balancing_strategy` | String |  | <p>Indicates how Amazon GameLift Servers FleetIQ balances the use of Spot Instances and On-Demand Instances in the
            game server group. Method options include the following:</p>
         <ul>
            <li>
               <p>
                  <code>SPOT_ONLY</code> - Only Spot Instances are used in the game server group. If Spot
                    Instances are unavailable or not viable for game hosting, the game server group
                    provides no hosting capacity until Spot Instances can again be used. Until then,
                    no new instances are started, and the existing nonviable Spot Instances are
                    terminated (after current gameplay ends) and are not replaced.</p>
            </li>
            <li>
               <p>
                  <code>SPOT_PREFERRED</code> - (default value) Spot Instances are used whenever available in
                    the game server group. If Spot Instances are unavailable, the game server group
                    continues to provide hosting capacity by falling back to On-Demand Instances.
                    Existing nonviable Spot Instances are terminated (after current gameplay ends)
                    and are replaced with new On-Demand Instances.</p>
            </li>
            <li>
               <p>
                  <code>ON_DEMAND_ONLY</code> - Only On-Demand Instances are used in the game
                    server group. No Spot Instances are used, even when available, while this
                    balancing strategy is in force.</p>
            </li>
         </ul> |
| `min_size` | i64 | ✅ | <p>The minimum number of instances allowed in the Amazon EC2 Auto Scaling group. During
            automatic scaling events, Amazon GameLift Servers FleetIQ and Amazon EC2 do not scale down the group below this
            minimum. In production, this value should be set to at least 1. After the Auto Scaling
            group is created, update this value directly in the Auto Scaling group using the Amazon Web Services
            console or APIs.</p> |
| `max_size` | i64 | ✅ | <p>The maximum number of instances allowed in the Amazon EC2 Auto Scaling group. During
            automatic scaling events, Amazon GameLift Servers FleetIQ and EC2 do not scale up the group above this maximum.
            After the Auto Scaling group is created, update this value directly in the Auto Scaling
            group using the Amazon Web Services console or APIs.</p> |
| `instance_definitions` | Vec<String> | ✅ | <p>The Amazon EC2 instance types and sizes to use in the Auto Scaling group. The instance
            definitions must specify at least two different instance types that are supported by
            Amazon GameLift Servers FleetIQ. For more information on instance types, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">EC2 Instance Types</a> in the
                <i>Amazon Elastic Compute Cloud User Guide</i>. You can optionally specify capacity
            weighting for each instance type. If no weight value is specified for an instance type,
            it is set to the default value "1". For more information about capacity weighting, see
                <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-instance-weighting.html"> Instance Weighting for
                Amazon EC2 Auto Scaling</a> in the Amazon EC2 Auto Scaling User Guide.</p> |
| `vpc_subnets` | Vec<String> |  | <p>A list of virtual private cloud (VPC) subnets to use with instances in the game server
            group. By default, all Amazon GameLift Servers FleetIQ-supported Availability Zones are used. You can use this
            parameter to specify VPCs that you've set up. This property cannot be updated after the
            game server group is created, and the corresponding Auto Scaling group will always use
            the property value that is set with this request, even if the Auto Scaling group is
            updated directly.</p> |
| `tags` | Vec<String> |  | <p>A list of labels to assign to the new game server group resource. Tags are
            developer-defined key-value pairs. Tagging Amazon Web Services resources is useful for resource
            management, access management, and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging Amazon Web Services
                Resources</a> in the <i>Amazon Web Services General Reference</i>.</p> |
| `game_server_group_name` | String | ✅ | <p>An identifier for the new game server group. This value is used to generate unique ARN
            identifiers for the Amazon EC2 Auto Scaling group and the Amazon GameLift Servers FleetIQ game server group. The name
            must be unique per Region per Amazon Web Services account.</p> |
| `auto_scaling_policy` | String |  | <p>Configuration settings to define a scaling policy for the Auto Scaling group that is
            optimized for game hosting. The scaling policy uses the metric
                <code>"PercentUtilizedGameServers"</code> to maintain a buffer of idle game servers
            that can immediately accommodate new games and players. After the Auto Scaling group is
            created, update this value directly in the Auto Scaling group using the Amazon Web Services console or
            APIs.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `game_server_group` | String | <p>An object with the property settings for the requested game server group resource.
        </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create game_server_group
game_server_group = provider.gamelift.Game_server_group {
    role_arn = "value"  # <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) for an IAM role that
            allows Amazon GameLift Servers to access your Amazon EC2 Auto Scaling groups.</p>
    launch_template = "value"  # <p>The Amazon EC2 launch template that contains configuration settings and game server code to
            be deployed to all instances in the game server group. You can specify the template
            using either the template name or ID. For help with creating a launch template, see
                <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-launch-template.html">Creating a Launch
                Template for an Auto Scaling Group</a> in the <i>Amazon Elastic Compute Cloud Auto Scaling
                User Guide</i>. After the Auto Scaling group is created, update this value
            directly in the Auto Scaling group using the Amazon Web Services console or APIs.</p>
         <note>
            <p>If you specify network interfaces in your launch template, you must explicitly set
                the property <code>AssociatePublicIpAddress</code> to "true". If no network
                interface is specified in the launch template, Amazon GameLift Servers FleetIQ uses your account's default
                VPC.</p>
         </note>
    min_size = "value"  # <p>The minimum number of instances allowed in the Amazon EC2 Auto Scaling group. During
            automatic scaling events, Amazon GameLift Servers FleetIQ and Amazon EC2 do not scale down the group below this
            minimum. In production, this value should be set to at least 1. After the Auto Scaling
            group is created, update this value directly in the Auto Scaling group using the Amazon Web Services
            console or APIs.</p>
    max_size = "value"  # <p>The maximum number of instances allowed in the Amazon EC2 Auto Scaling group. During
            automatic scaling events, Amazon GameLift Servers FleetIQ and EC2 do not scale up the group above this maximum.
            After the Auto Scaling group is created, update this value directly in the Auto Scaling
            group using the Amazon Web Services console or APIs.</p>
    instance_definitions = "value"  # <p>The Amazon EC2 instance types and sizes to use in the Auto Scaling group. The instance
            definitions must specify at least two different instance types that are supported by
            Amazon GameLift Servers FleetIQ. For more information on instance types, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">EC2 Instance Types</a> in the
                <i>Amazon Elastic Compute Cloud User Guide</i>. You can optionally specify capacity
            weighting for each instance type. If no weight value is specified for an instance type,
            it is set to the default value "1". For more information about capacity weighting, see
                <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-instance-weighting.html"> Instance Weighting for
                Amazon EC2 Auto Scaling</a> in the Amazon EC2 Auto Scaling User Guide.</p>
    game_server_group_name = "value"  # <p>An identifier for the new game server group. This value is used to generate unique ARN
            identifiers for the Amazon EC2 Auto Scaling group and the Amazon GameLift Servers FleetIQ game server group. The name
            must be unique per Region per Amazon Web Services account.</p>
}

# Access game_server_group outputs
game_server_group_id = game_server_group.id
game_server_group_game_server_group = game_server_group.game_server_group
```

---


### Game_server

GameServer resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `game_server_id` | String | ✅ | <p>A custom string that uniquely identifies the game server to update.</p> |
| `game_server_group_name` | String | ✅ | <p>A unique identifier for the game server group where the game server is running.</p> |
| `utilization_status` | String |  | <p>Indicates if the game server is available or is currently hosting gameplay. You can
            update a game server status from <code>AVAILABLE</code> to <code>UTILIZED</code>, but
            you can't change a the status from <code>UTILIZED</code> to
            <code>AVAILABLE</code>.</p> |
| `health_check` | String |  | <p>Indicates health status of the game server. A request that includes this parameter
            updates the game server's <i>LastHealthCheckTime</i> timestamp. </p> |
| `game_server_data` | String |  | <p>A set of custom game server properties, formatted as a single string value. This data 
            is passed to a game client or service when it requests information on game servers. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `game_server` | String | <p>Object that describes the requested game server.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access game_server outputs
game_server_id = game_server.id
game_server_game_server = game_server.game_server
```

---


### Matchmaking_configuration

MatchmakingConfiguration resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `acceptance_required` | bool | ✅ | <p>A flag that determines whether a match that was created with this configuration must
            be accepted by the matched players. To require acceptance, set to <code>TRUE</code>.
            With this option enabled, matchmaking tickets use the status
                <code>REQUIRES_ACCEPTANCE</code> to indicate when a completed potential match is
            waiting for player acceptance. </p> |
| `rule_set_name` | String | ✅ | <p>A unique identifier for the matchmaking rule set to use with this configuration. You can use either the rule set name or ARN
            value. A matchmaking configuration can only use rule sets that are defined in the same
            Region.</p> |
| `notification_target` | String |  | <p>An SNS topic ARN that is set up to receive matchmaking notifications. See <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-notification.html">
                Setting up notifications for matchmaking</a> for more information.</p> |
| `name` | String | ✅ | <p>A unique identifier for the matchmaking configuration. This name is used to identify the configuration associated with a matchmaking
            request or ticket.</p> |
| `game_session_queue_arns` | Vec<String> |  | <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) that is assigned to a Amazon GameLift Servers game session queue resource and uniquely identifies it. ARNs are unique across all Regions. Format is <code>arn:aws:gamelift:<region>::gamesessionqueue/<queue name></code>. Queues can be located in any Region. Queues are used to start new
            Amazon GameLift Servers-hosted game sessions for matches that are created with this matchmaking
            configuration. If <code>FlexMatchMode</code> is set to <code>STANDALONE</code>, do not
            set this parameter. </p> |
| `flex_match_mode` | String |  | <p>Indicates whether this matchmaking configuration is being used with Amazon GameLift Servers hosting or
            as a standalone matchmaking solution. </p>
         <ul>
            <li>
               <p>
                  <b>STANDALONE</b> - FlexMatch forms matches and
                    returns match information, including players and team assignments, in a <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-events.html#match-events-matchmakingsucceeded"> MatchmakingSucceeded</a> event.</p>
            </li>
            <li>
               <p>
                  <b>WITH_QUEUE</b> - FlexMatch forms matches and uses
                    the specified Amazon GameLift Servers queue to start a game session for the match. </p>
            </li>
         </ul> |
| `description` | String |  | <p>A human-readable description of the matchmaking configuration. </p> |
| `game_properties` | Vec<String> |  | <p>A set of key-value pairs that can store custom data in a game session.
  For example: <code>{"Key": "difficulty", "Value": "novice"}</code>. This information is added to the new <code>GameSession</code> object that is
            created for a successful match. This parameter is not used if <code>FlexMatchMode</code>
            is set to <code>STANDALONE</code>.</p> |
| `request_timeout_seconds` | i64 | ✅ | <p>The maximum duration, in seconds, that a matchmaking ticket can remain in process
            before timing out. Requests that fail due to timing out can be resubmitted as
            needed.</p> |
| `tags` | Vec<String> |  | <p>A list of labels to assign to the new matchmaking configuration resource. Tags are
            developer-defined key-value pairs. Tagging Amazon Web Services resources are useful for resource
            management, access management and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging Amazon Web Services
                Resources</a> in the <i>Amazon Web Services General Reference</i>.</p> |
| `backfill_mode` | String |  | <p>The method used to backfill game sessions that are created with this matchmaking
            configuration. Specify <code>MANUAL</code> when your game manages backfill requests
            manually or does not use the match backfill feature. Specify <code>AUTOMATIC</code> to
            have Amazon GameLift Servers create a backfill request whenever a game session has one or more open
            slots. Learn more about manual and automatic backfill in <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-backfill.html"> Backfill Existing Games
                with FlexMatch</a>. Automatic backfill is not available when
                <code>FlexMatchMode</code> is set to <code>STANDALONE</code>.</p> |
| `custom_event_data` | String |  | <p>Information to be added to all events related to this matchmaking configuration.
        </p> |
| `additional_player_count` | i64 |  | <p>The number of player slots in a match to keep open for future players. For example, if the configuration's rule set specifies 
            a match for a single 12-person team, and the additional player count is set to 2, only 10 players are selected for the match. This parameter is not used if <code>FlexMatchMode</code> is set to
                <code>STANDALONE</code>.</p> |
| `acceptance_timeout_seconds` | i64 |  | <p>The length of time (in seconds) to wait for players to accept a proposed match, if
            acceptance is required. </p> |
| `game_session_data` | String |  | <p>A set of custom game session properties, formatted as a single string value. This data is passed to a game server process with a request to start a new game session. For more information, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a game session</a>. This information is added to the new <code>GameSession</code> object that is
            created for a successful match. This parameter is not used if <code>FlexMatchMode</code>
            is set to <code>STANDALONE</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create matchmaking_configuration
matchmaking_configuration = provider.gamelift.Matchmaking_configuration {
    acceptance_required = "value"  # <p>A flag that determines whether a match that was created with this configuration must
            be accepted by the matched players. To require acceptance, set to <code>TRUE</code>.
            With this option enabled, matchmaking tickets use the status
                <code>REQUIRES_ACCEPTANCE</code> to indicate when a completed potential match is
            waiting for player acceptance. </p>
    rule_set_name = "value"  # <p>A unique identifier for the matchmaking rule set to use with this configuration. You can use either the rule set name or ARN
            value. A matchmaking configuration can only use rule sets that are defined in the same
            Region.</p>
    name = "value"  # <p>A unique identifier for the matchmaking configuration. This name is used to identify the configuration associated with a matchmaking
            request or ticket.</p>
    request_timeout_seconds = "value"  # <p>The maximum duration, in seconds, that a matchmaking ticket can remain in process
            before timing out. Requests that fail due to timing out can be resubmitted as
            needed.</p>
}

```

---


### Fleet_deployment

FleetDeployment resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `locational_deployments` | HashMap<String, String> | <p>If the deployment is for a multi-location fleet, the requests returns the deployment
            status in each fleet location.</p> |
| `fleet_deployment` | String | <p>The requested deployment information.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fleet_deployment outputs
fleet_deployment_id = fleet_deployment.id
fleet_deployment_locational_deployments = fleet_deployment.locational_deployments
fleet_deployment_fleet_deployment = fleet_deployment.fleet_deployment
```

---


### Matchmaking_rule_sets

MatchmakingRuleSets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p> |
| `rule_sets` | Vec<String> | <p>A collection of requested matchmaking rule set objects. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access matchmaking_rule_sets outputs
matchmaking_rule_sets_id = matchmaking_rule_sets.id
matchmaking_rule_sets_next_token = matchmaking_rule_sets.next_token
matchmaking_rule_sets_rule_sets = matchmaking_rule_sets.rule_sets
```

---


### Scaling_policies

ScalingPolicies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p> |
| `scaling_policies` | Vec<String> | <p>A collection of objects containing the scaling policies matching the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access scaling_policies outputs
scaling_policies_id = scaling_policies.id
scaling_policies_next_token = scaling_policies.next_token
scaling_policies_scaling_policies = scaling_policies.scaling_policies
```

---


### Scaling_policy

ScalingPolicy resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scaling_adjustment_type` | String |  | <p>The type of adjustment to make to a fleet's instance count:</p>
         <ul>
            <li>
               <p>
                  <b>ChangeInCapacity</b> -- add (or subtract) the
                    scaling adjustment value from the current instance count. Positive values scale
                    up while negative values scale down.</p>
            </li>
            <li>
               <p>
                  <b>ExactCapacity</b> -- set the instance count to the
                    scaling adjustment value.</p>
            </li>
            <li>
               <p>
                  <b>PercentChangeInCapacity</b> -- increase or reduce
                    the current instance count by the scaling adjustment, read as a percentage.
                    Positive values scale up while negative values scale down; for example, a value
                    of "-10" scales the fleet down by 10%.</p>
            </li>
         </ul> |
| `metric_name` | String | ✅ | <p>Name of the Amazon GameLift Servers-defined metric that is used to trigger a scaling adjustment. For
            detailed descriptions of fleet metrics, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/monitoring-cloudwatch.html">Monitor Amazon GameLift Servers
                with Amazon CloudWatch</a>. </p>
         <ul>
            <li>
               <p>
                  <b>ActivatingGameSessions</b> -- Game sessions in
                    the process of being created.</p>
            </li>
            <li>
               <p>
                  <b>ActiveGameSessions</b> -- Game sessions that
                    are currently running.</p>
            </li>
            <li>
               <p>
                  <b>ActiveInstances</b> -- Fleet instances that
                    are currently running at least one game session.</p>
            </li>
            <li>
               <p>
                  <b>AvailableGameSessions</b> -- Additional game
                    sessions that fleet could host simultaneously, given current capacity.</p>
            </li>
            <li>
               <p>
                  <b>AvailablePlayerSessions</b> -- Empty player
                    slots in currently active game sessions. This includes game sessions that are
                    not currently accepting players. Reserved player slots are not
                    included.</p>
            </li>
            <li>
               <p>
                  <b>CurrentPlayerSessions</b> -- Player slots in
                    active game sessions that are being used by a player or are reserved for a
                    player. </p>
            </li>
            <li>
               <p>
                  <b>IdleInstances</b> -- Active instances that are
                    currently hosting zero game sessions. </p>
            </li>
            <li>
               <p>
                  <b>PercentAvailableGameSessions</b> -- Unused
                    percentage of the total number of game sessions that a fleet could host
                    simultaneously, given current capacity. Use this metric for a target-based
                    scaling policy.</p>
            </li>
            <li>
               <p>
                  <b>PercentIdleInstances</b> -- Percentage of the
                    total number of active instances that are hosting zero game sessions.</p>
            </li>
            <li>
               <p>
                  <b>QueueDepth</b> -- Pending game session
                    placement requests, in any queue, where the current fleet is the top-priority
                    destination.</p>
            </li>
            <li>
               <p>
                  <b>WaitTime</b> -- Current wait time for pending
                    game session placement requests, in any queue, where the current fleet is the
                    top-priority destination. </p>
            </li>
         </ul> |
| `target_configuration` | String |  | <p>An object that contains settings for a target-based scaling policy.</p> |
| `comparison_operator` | String |  | <p>Comparison operator to use when measuring the metric against the threshold
            value.</p> |
| `name` | String | ✅ | <p>A descriptive label that is associated with a fleet's scaling policy. Policy names do not need to be unique. A fleet can have only one scaling policy with the same name.</p> |
| `scaling_adjustment` | i64 |  | <p>Amount of adjustment to make, based on the scaling adjustment type.</p> |
| `threshold` | f64 |  | <p>Metric value used to trigger a scaling event.</p> |
| `fleet_id` | String | ✅ | <p>A unique identifier for the fleet to apply this policy to. You can use either the fleet ID or ARN value. The fleet
            cannot be in any of the following statuses: ERROR or DELETING.</p> |
| `evaluation_periods` | i64 |  | <p>Length of time (in minutes) the metric must be at or beyond the threshold before a
            scaling event is triggered.</p> |
| `policy_type` | String |  | <p>The type of scaling policy to create. For a target-based policy, set the parameter
                <i>MetricName</i> to 'PercentAvailableGameSessions' and specify a
                <i>TargetConfiguration</i>. For a rule-based policy set the following
            parameters: <i>MetricName</i>, <i>ComparisonOperator</i>,
                <i>Threshold</i>, <i>EvaluationPeriods</i>,
                <i>ScalingAdjustmentType</i>, and
                <i>ScalingAdjustment</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create scaling_policy
scaling_policy = provider.gamelift.Scaling_policy {
    metric_name = "value"  # <p>Name of the Amazon GameLift Servers-defined metric that is used to trigger a scaling adjustment. For
            detailed descriptions of fleet metrics, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/monitoring-cloudwatch.html">Monitor Amazon GameLift Servers
                with Amazon CloudWatch</a>. </p>
         <ul>
            <li>
               <p>
                  <b>ActivatingGameSessions</b> -- Game sessions in
                    the process of being created.</p>
            </li>
            <li>
               <p>
                  <b>ActiveGameSessions</b> -- Game sessions that
                    are currently running.</p>
            </li>
            <li>
               <p>
                  <b>ActiveInstances</b> -- Fleet instances that
                    are currently running at least one game session.</p>
            </li>
            <li>
               <p>
                  <b>AvailableGameSessions</b> -- Additional game
                    sessions that fleet could host simultaneously, given current capacity.</p>
            </li>
            <li>
               <p>
                  <b>AvailablePlayerSessions</b> -- Empty player
                    slots in currently active game sessions. This includes game sessions that are
                    not currently accepting players. Reserved player slots are not
                    included.</p>
            </li>
            <li>
               <p>
                  <b>CurrentPlayerSessions</b> -- Player slots in
                    active game sessions that are being used by a player or are reserved for a
                    player. </p>
            </li>
            <li>
               <p>
                  <b>IdleInstances</b> -- Active instances that are
                    currently hosting zero game sessions. </p>
            </li>
            <li>
               <p>
                  <b>PercentAvailableGameSessions</b> -- Unused
                    percentage of the total number of game sessions that a fleet could host
                    simultaneously, given current capacity. Use this metric for a target-based
                    scaling policy.</p>
            </li>
            <li>
               <p>
                  <b>PercentIdleInstances</b> -- Percentage of the
                    total number of active instances that are hosting zero game sessions.</p>
            </li>
            <li>
               <p>
                  <b>QueueDepth</b> -- Pending game session
                    placement requests, in any queue, where the current fleet is the top-priority
                    destination.</p>
            </li>
            <li>
               <p>
                  <b>WaitTime</b> -- Current wait time for pending
                    game session placement requests, in any queue, where the current fleet is the
                    top-priority destination. </p>
            </li>
         </ul>
    name = "value"  # <p>A descriptive label that is associated with a fleet's scaling policy. Policy names do not need to be unique. A fleet can have only one scaling policy with the same name.</p>
    fleet_id = "value"  # <p>A unique identifier for the fleet to apply this policy to. You can use either the fleet ID or ARN value. The fleet
            cannot be in any of the following statuses: ERROR or DELETING.</p>
}

```

---


### Fleet_capacity

FleetCapacity resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `desired_instances` | i64 |  | <p>The number of Amazon EC2 instances you want to maintain in the specified fleet location.
            This value must fall between the minimum and maximum size limits. Changes in desired
            instance value can take up to 1 minute to be reflected when viewing the fleet's capacity
            settings.</p> |
| `max_size` | i64 |  | <p>The maximum number of instances that are allowed in the specified fleet location. If
            this parameter is not set, the default is 1.</p> |
| `location` | String |  | <p>The name of a remote location to update fleet capacity settings for, in the form of an
            Amazon Web Services Region code such as <code>us-west-2</code>.</p> |
| `min_size` | i64 |  | <p>The minimum number of instances that are allowed in the specified fleet location. If
            this parameter is not set, the default is 0.</p> |
| `fleet_id` | String | ✅ | <p>A unique identifier for the fleet to update capacity settings for. You can use either the fleet ID or ARN
            value.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fleet_capacity` | Vec<String> | <p>A collection of objects that contains capacity information for each requested fleet
            ID. Capacity objects are returned only for fleets that currently exist. Changes in
            desired instance value can take up to 1 minute to be reflected.</p> |
| `next_token` | String | <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fleet_capacity outputs
fleet_capacity_id = fleet_capacity.id
fleet_capacity_fleet_capacity = fleet_capacity.fleet_capacity
fleet_capacity_next_token = fleet_capacity.next_token
```

---


### Game_sessions

GameSessions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p> |
| `game_sessions` | Vec<String> | <p>A collection of properties for each game session that matches the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access game_sessions outputs
game_sessions_id = game_sessions.id
game_sessions_next_token = game_sessions.next_token
game_sessions_game_sessions = game_sessions.game_sessions
```

---


### Vpc_peering_connections

VpcPeeringConnections resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpc_peering_connections` | Vec<String> | <p>A collection of VPC peering connection records that match the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpc_peering_connections outputs
vpc_peering_connections_id = vpc_peering_connections.id
vpc_peering_connections_vpc_peering_connections = vpc_peering_connections.vpc_peering_connections
```

---


### Vpc_peering_authorizations

VpcPeeringAuthorizations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpc_peering_authorizations` | Vec<String> | <p>A collection of objects that describe all valid VPC peering operations for the current
            Amazon Web Services account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpc_peering_authorizations outputs
vpc_peering_authorizations_id = vpc_peering_authorizations.id
vpc_peering_authorizations_vpc_peering_authorizations = vpc_peering_authorizations.vpc_peering_authorizations
```

---


### Build

Build resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `operating_system` | String |  | <p>The operating system that your game server binaries run on. This value determines the
            type of fleet resources that you use for this build. If your game build contains
            multiple executables, they all must run on the same operating system. You must specify a
            valid operating system in this request. There is no default value. You can't change a
            build's operating system later.</p>
         <note>
            <p>Amazon Linux 2 (AL2) will reach end of support on 6/30/2025. See more details in 
            the <a href="http://aws.amazon.com/amazon-linux-2/faqs/">Amazon Linux 2 FAQs</a>. 
            For game servers
                that are hosted on AL2 and use server SDK version 4.x for Amazon GameLift Servers, first update the
                game server build to server SDK 5.x, and then deploy to AL2023 instances. See
            <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/reference-serversdk5-migration.html">
                Migrate to server SDK version 5.</a>
            </p>
         </note> |
| `server_sdk_version` | String |  | <p>A server SDK version you used when integrating your game server build with Amazon GameLift Servers. For more information see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/integration-custom-intro.html">Integrate games
                with custom game servers</a>. By default Amazon GameLift Servers sets this value to
                <code>4.0.2</code>.</p> |
| `storage_location` | String |  | <p>Information indicating where your game build files are stored. Use this parameter only
            when creating a build with files stored in an Amazon S3 bucket that you own. The storage
            location must specify an Amazon S3 bucket name and key. The location must also specify a role
            ARN that you set up to allow Amazon GameLift Servers to access your Amazon S3 bucket. The S3 bucket and your
            new build must be in the same Region.</p>
         <p>If a <code>StorageLocation</code> is specified, the size of your file can be found in
            your Amazon S3 bucket. Amazon GameLift Servers will report a <code>SizeOnDisk</code> of 0. </p> |
| `name` | String |  | <p>A descriptive label that is associated with a build. Build names do not need to be unique. You can change this value later. </p> |
| `tags` | Vec<String> |  | <p>A list of labels to assign to the new build resource. Tags are developer defined
            key-value pairs. Tagging Amazon Web Services resources are useful for resource management, access
            management and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging Amazon Web Services Resources</a> in the
                <i>Amazon Web Services General Reference</i>. Once the resource is created, you can
            use <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_TagResource.html">TagResource</a>, <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_UntagResource.html">UntagResource</a>, and
                <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_ListTagsForResource.html">ListTagsForResource</a> to add, remove, and view tags. The maximum tag limit
            may be lower than stated. See the Amazon Web Services General Reference for actual tagging
            limits.</p> |
| `version` | String |  | <p>Version information that is associated with a build or script. Version strings do not need to be unique. You can change this value later. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `build` | String | <p>Set of properties describing the requested build.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create build
build = provider.gamelift.Build {
}

# Access build outputs
build_id = build.id
build_build = build.build
```

---


### Container_fleet

ContainerFleet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_type` | String |  | <p>The Amazon EC2 instance type to use for all instances in the fleet. For multi-location
            fleets, the instance type must be available in the home region and all remote locations.
            Instance type determines the computing resources and processing power that's available
            to host your game servers. This includes including CPU, memory, storage, and networking
            capacity. </p>
         <p>By default, Amazon GameLift Servers selects an instance type that fits the needs of your container
            groups and is available in all selected fleet locations. You can also choose to manually
            set this parameter. See <a href="http://aws.amazon.com/ec2/instance-types/">Amazon Elastic Compute Cloud
                Instance Types</a> for detailed descriptions of Amazon EC2 instance types.</p>
         <p>You can't update this fleet property later.</p> |
| `locations` | Vec<String> |  | <p>A set of locations to deploy container fleet instances to. You can add any Amazon Web Services
            Region or Local Zone that's supported by Amazon GameLift Servers. Provide a list of one or more Amazon Web Services
            Region codes, such as <code>us-west-2</code>, or Local Zone names. Also include the
            fleet's home Region, which is the Amazon Web Services Region where the fleet is created. For a list of
            supported Regions and Local Zones, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-regions.html"> Amazon GameLift Servers service
                locations</a> for managed hosting.</p> |
| `log_configuration` | String |  | <p>A method for collecting container logs for the fleet. Amazon GameLift Servers saves all standard
            output for each container in logs, including game session logs. You can select from the
            following methods: </p>
         <ul>
            <li>
               <p>
                  <code>CLOUDWATCH</code> -- Send logs to an Amazon CloudWatch log group that you define. Each container
                    emits a log stream, which is organized in the log group. </p>
            </li>
            <li>
               <p>
                  <code>S3</code> -- Store logs in an Amazon S3 bucket that you define.</p>
            </li>
            <li>
               <p>
                  <code>NONE</code> -- Don't collect container logs.</p>
            </li>
         </ul>
         <p>By default, this property is set to <code>CLOUDWATCH</code>. </p>
         <p>Amazon GameLift Servers requires permissions to send logs other Amazon Web Services services in your account. These permissions are included in the IAM fleet role for this container fleet (see <code>FleetRoleArn)</code>.</p> |
| `per_instance_container_group_definition_name` | String |  | <p>The name of a container group definition resource that describes a set of axillary
            software. A fleet instance has one process for executables in this container group. A
            per-instance container group is optional. You can update the fleet to add or remove a
            per-instance container group at any time. You can specify the container group
            definition's name to use the latest version. Alternatively, provide an ARN value with a
            specific version number. </p>
         <p>Create a container group definition by calling 
            <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_CreateContainerGroupDefinition.html">https://docs.aws.amazon.com/gamelift/latest/apireference/API_CreateContainerGroupDefinition.html</a>. 
            This operation creates a 
            <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_ContainerGroupDefinition.html">https://docs.aws.amazon.com/gamelift/latest/apireference/API_ContainerGroupDefinition.html</a> resource.</p> |
| `game_session_creation_limit_policy` | String |  | <p>A policy that limits the number of game sessions that each individual player can create
            on instances in this fleet. The limit applies for a specified span of time.</p> |
| `description` | String |  | <p>A meaningful description of the container fleet.</p> |
| `game_server_container_group_definition_name` | String |  | <p>A container group definition resource that describes how to deploy containers with
            your game server build and support software onto each fleet instance. You can specify
            the container group definition's name to use the latest version. Alternatively, provide
            an ARN value with a specific version number.</p>
         <p>Create a container group definition by calling 
            <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_CreateContainerGroupDefinition.html">CreateContainerGroupDefinition</a>. 
            This operation creates a 
            <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_ContainerGroupDefinition.html">ContainerGroupDefinition</a> resource. </p> |
| `game_server_container_groups_per_instance` | i64 |  | <p>The number of times to replicate the game server container group on each fleet
            instance. </p>
         <p>By default, Amazon GameLift Servers calculates the maximum number of game server container groups that
            can fit on each instance. This calculation is based on the CPU and memory resources of
            the fleet's instance type). To use the calculated maximum, don't set this parameter. If
            you set this number manually, Amazon GameLift Servers uses your value as long as it's less than the
            calculated maximum.</p> |
| `instance_connection_port_range` | String |  | <p>The set of port numbers to open on each fleet instance. A fleet's connection ports map
            to container ports that are configured in the fleet's container group definitions. </p>
         <p>By default, Amazon GameLift Servers calculates an optimal port range based on your fleet
            configuration. To use the calculated range, don't set this parameter. The values
            are:</p>
         <ul>
            <li>
               <p>Port range: 4192 to a number calculated based on your fleet configuration.
                    Amazon GameLift Servers uses the following formula: <code>4192 + [# of game server container
                        groups per fleet instance] * [# of container ports in the game server
                        container group definition] + [# of container ports in the game server
                        container group definition]</code>
               </p>
            </li>
         </ul>
         <p>You can also choose to manually set this parameter. When manually setting this
            parameter, you must use port numbers that match the fleet's inbound permissions port
            range.</p>
         <note>
            <p>If you set values manually, Amazon GameLift Servers no longer calculates a port range for you,
                even if you later remove the manual settings. </p>
         </note> |
| `tags` | Vec<String> |  | <p>A list of labels to assign to the new fleet resource. Tags are developer-defined
            key-value pairs. Tagging Amazon Web Services resources are useful for resource management, access
            management and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging Amazon Web Services Resources</a> in the
            <i>Amazon Web Services General Reference</i>.</p> |
| `instance_inbound_permissions` | Vec<String> |  | <p>The IP address ranges and port settings that allow inbound traffic to access game
            server processes and other processes on this fleet. As a best practice, when remotely
            accessing a fleet instance, we recommend opening ports only when you need them and
            closing them when you're finished.</p>
         <p>By default, Amazon GameLift Servers calculates an optimal port range based on your fleet
            configuration. To use the calculated range, don't set this parameter. The values
            are:</p>
         <ul>
            <li>
               <p>Protocol: UDP</p>
            </li>
            <li>
               <p>Port range: 4192 to a number calculated based on your fleet configuration. Amazon GameLift Servers uses the
                    following formula: <code>4192 + [# of game server container groups per fleet
                        instance] * [# of container ports in the game server container group
                        definition] + [# of container ports in the game server container group
                        definition]</code>
               </p>
            </li>
         </ul>
         <p>You can also choose to manually set this parameter. When manually setting this
            parameter, you must use port numbers that match the fleet's connection port
            range.</p>
         <note>
            <p>If you set values manually, Amazon GameLift Servers no longer calculates a port range for you,
                even if you later remove the manual settings. </p>
         </note> |
| `metric_groups` | Vec<String> |  | <p>The name of an Amazon Web Services CloudWatch metric group to add this fleet to. You can use a metric group 
            to aggregate metrics for multiple fleets. You can specify an existing metric
            group name or use a new name to create a new metric group. Each fleet can have only one metric group, 
            but you can change this value at any time. </p> |
| `fleet_role_arn` | String | ✅ | <p>The unique identifier for an Identity and Access Management (IAM) role with permissions to run your
            containers on resources that are managed by Amazon GameLift Servers. Use an IAM service role with the
                <code>GameLiftContainerFleetPolicy</code> managed policy attached. For more
            information, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/setting-up-role.html">Set up an IAM service
                role</a>. You can't change this fleet property after the fleet is
            created.</p>
         <p>IAM role ARN values use the following pattern: <code>arn:aws:iam::[Amazon Web Services account]:role/[role name]</code>.</p> |
| `billing_type` | String |  | <p>Indicates whether to use On-Demand or Spot instances for this fleet. Learn more about
            when to use <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-ec2-instances.html#gamelift-ec2-instances-spot"> On-Demand versus Spot Instances</a>. This fleet property can't be changed after the fleet is created.</p>
         <p>By default, this property is set to <code>ON_DEMAND</code>.</p>
         <p>You can't update this fleet property later.</p> |
| `new_game_session_protection_policy` | String |  | <p>Determines whether Amazon GameLift Servers can shut down game sessions on the fleet that are actively
            running and hosting players. Amazon GameLift Servers might prompt an instance shutdown when scaling down
            fleet capacity or when retiring unhealthy instances. You can also set game session
            protection for individual game sessions using <a href="gamelift/latest/apireference/API_UpdateGameSession.html">UpdateGameSession</a>.</p>
         <ul>
            <li>
               <p>
                  <b>NoProtection</b> -- Game sessions can be shut down
                    during active gameplay. </p>
            </li>
            <li>
               <p>
                  <b>FullProtection</b> -- Game sessions in
                        <code>ACTIVE</code> status can't be shut down.</p>
            </li>
         </ul>
         <p>By default, this property is set to <code>NoProtection</code>. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `container_fleet` | String | <p>The properties for the requested container fleet, including current status.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create container_fleet
container_fleet = provider.gamelift.Container_fleet {
    fleet_role_arn = "value"  # <p>The unique identifier for an Identity and Access Management (IAM) role with permissions to run your
            containers on resources that are managed by Amazon GameLift Servers. Use an IAM service role with the
                <code>GameLiftContainerFleetPolicy</code> managed policy attached. For more
            information, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/setting-up-role.html">Set up an IAM service
                role</a>. You can't change this fleet property after the fleet is
            created.</p>
         <p>IAM role ARN values use the following pattern: <code>arn:aws:iam::[Amazon Web Services account]:role/[role name]</code>.</p>
}

# Access container_fleet outputs
container_fleet_id = container_fleet.id
container_fleet_container_fleet = container_fleet.container_fleet
```

---


### Ec2_instance_limits

EC2InstanceLimits resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ec2_instance_limits` | Vec<String> | <p>The maximum number of instances for the specified instance type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ec2_instance_limits outputs
ec2_instance_limits_id = ec2_instance_limits.id
ec2_instance_limits_ec2_instance_limits = ec2_instance_limits.ec2_instance_limits
```

---


### Vpc_peering_authorization

VpcPeeringAuthorization resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `game_lift_aws_account_id` | String | ✅ | <p>A unique identifier for the Amazon Web Services account that you use to manage your Amazon GameLift Servers fleet. 
            You can find your Account ID in the Amazon Web Services Management Console under account settings.</p> |
| `peer_vpc_id` | String | ✅ | <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift Servers fleet. The
            VPC must be in the same Region as your fleet. To look up a VPC ID, use the 
            <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the Amazon Web Services Management Console. 
            Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Servers Fleets</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_peering_authorization
vpc_peering_authorization = provider.gamelift.Vpc_peering_authorization {
    game_lift_aws_account_id = "value"  # <p>A unique identifier for the Amazon Web Services account that you use to manage your Amazon GameLift Servers fleet. 
            You can find your Account ID in the Amazon Web Services Management Console under account settings.</p>
    peer_vpc_id = "value"  # <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift Servers fleet. The
            VPC must be in the same Region as your fleet. To look up a VPC ID, use the 
            <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the Amazon Web Services Management Console. 
            Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Servers Fleets</a>.</p>
}

```

---


### Fleet_port_settings

FleetPortSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `inbound_permission_revocations` | Vec<String> |  | <p>A collection of port settings to be removed from the fleet resource.</p> |
| `inbound_permission_authorizations` | Vec<String> |  | <p>A collection of port settings to be added to the fleet resource.</p> |
| `fleet_id` | String | ✅ | <p>A unique identifier for the fleet to update port settings for. You can use either the fleet ID or ARN
            value.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location` | String | <p>The requested fleet location, expressed as an Amazon Web Services Region code, such as
                <code>us-west-2</code>. </p> |
| `inbound_permissions` | Vec<String> | <p>The port settings for the requested fleet ID.</p> |
| `fleet_id` | String | <p>A unique identifier for the fleet that was requested. </p> |
| `update_status` | String | <p>The current status of updates to the fleet's port settings in the requested fleet
            location. A status of <code>PENDING_UPDATE</code> indicates that an update was requested
            for the fleet but has not yet been completed for the location.</p> |
| `fleet_arn` | String | <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) that is assigned to a Amazon GameLift Servers fleet resource and uniquely identifies it. ARNs are unique across all Regions. Format is <code>arn:aws:gamelift:<region>::fleet/fleet-a1234567-b8c9-0d1e-2fa3-b45c6d7e8912</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fleet_port_settings outputs
fleet_port_settings_id = fleet_port_settings.id
fleet_port_settings_location = fleet_port_settings.location
fleet_port_settings_inbound_permissions = fleet_port_settings.inbound_permissions
fleet_port_settings_fleet_id = fleet_port_settings.fleet_id
fleet_port_settings_update_status = fleet_port_settings.update_status
fleet_port_settings_fleet_arn = fleet_port_settings.fleet_arn
```

---


### Compute_access

ComputeAccess resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fleet_id` | String | <p>The ID of the fleet that holds the compute resource to be accessed.</p> |
| `credentials` | String | <p>A set of temporary Amazon Web Services credentials for use when connecting to the
            compute resource with Amazon EC2 Systems Manager (SSM).</p> |
| `compute_name` | String | <p>The identifier of the compute resource to be accessed.  This value might be either a
            compute name or an instance ID.</p> |
| `target` | String | <p>The instance ID where the compute resource is running.</p> |
| `compute_arn` | String | <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) that is assigned to an Amazon GameLift Servers compute resource and uniquely identifies it.
            ARNs are unique across all Regions. Format is
                <code>arn:aws:gamelift:<region>::compute/compute-a1234567-b8c9-0d1e-2fa3-b45c6d7e8912</code>.</p> |
| `fleet_arn` | String | <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) that is assigned to a Amazon GameLift Servers fleet resource and uniquely identifies it. ARNs are unique across all Regions. Format is <code>arn:aws:gamelift:<region>::fleet/fleet-a1234567-b8c9-0d1e-2fa3-b45c6d7e8912</code>.</p> |
| `container_identifiers` | Vec<String> | <p>For a managed container fleet, a list of containers on the compute. Use the container
            runtime ID with Docker commands to connect to a specific container. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access compute_access outputs
compute_access_id = compute_access.id
compute_access_fleet_id = compute_access.fleet_id
compute_access_credentials = compute_access.credentials
compute_access_compute_name = compute_access.compute_name
compute_access_target = compute_access.target
compute_access_compute_arn = compute_access.compute_arn
compute_access_fleet_arn = compute_access.fleet_arn
compute_access_container_identifiers = compute_access.container_identifiers
```

---


### Player_session

PlayerSession resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `player_data` | String |  | <p>Developer-defined information related to a player. Amazon GameLift Servers does not use this data, so it can be formatted as needed for use in the game.</p> |
| `game_session_id` | String | ✅ | <p>A unique identifier for the game session to add a player to.</p> |
| `player_id` | String | ✅ | <p>A unique identifier for a player. Player IDs are developer-defined.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create player_session
player_session = provider.gamelift.Player_session {
    game_session_id = "value"  # <p>A unique identifier for the game session to add a player to.</p>
    player_id = "value"  # <p>A unique identifier for a player. Player IDs are developer-defined.</p>
}

```

---


### Fleet

Fleet resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of labels to assign to the new fleet resource. Tags are developer-defined
            key-value pairs. Tagging Amazon Web Services resources are useful for resource management, access
            management and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging Amazon Web Services Resources</a> in the
                <i>Amazon Web Services General Reference</i>.</p> |
| `build_id` | String |  | <p>The unique identifier for a custom game server build to be deployed to a fleet with
            compute type <code>EC2</code>. You can use either the build ID or ARN. The build must be
            uploaded to Amazon GameLift Servers and in <code>READY</code> status. This fleet property can't be changed after the fleet is created.</p> |
| `script_id` | String |  | <p>The unique identifier for a Realtime configuration script to be deployed to a fleet with
            compute type <code>EC2</code>. You can use either the script ID or ARN. Scripts must be
            uploaded to Amazon GameLift Servers prior to creating the fleet. This fleet property can't be changed after the fleet is created.</p> |
| `server_launch_parameters` | String |  | <p>
            <b>This parameter is no longer used.</b> Specify server
            launch parameters using the <code>RuntimeConfiguration</code> parameter. Requests that
            use this parameter instead continue to be valid.</p> |
| `peer_vpc_id` | String |  | <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift Servers fleet. The
            VPC must be in the same Region as your fleet. To look up a VPC ID, use the 
            <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the Amazon Web Services Management Console. 
            Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Servers Fleets</a>.</p> |
| `certificate_configuration` | String |  | <p>Prompts Amazon GameLift Servers to generate a TLS/SSL certificate for the fleet. Amazon GameLift Servers uses the
            certificates to encrypt traffic between game clients and the game servers running on
            Amazon GameLift Servers. By default, the <code>CertificateConfiguration</code> is <code>DISABLED</code>.
            You can't change this property after you create the fleet. </p>
         <p>Certificate Manager (ACM) certificates expire after 13 months.
            Certificate expiration can cause fleets to fail, preventing players from connecting to
            instances in the fleet. We recommend you replace fleets before 13 months, consider using
            fleet aliases for a smooth transition.</p>
         <note>
            <p>ACM isn't available in all Amazon Web Services regions. A fleet creation request
                with certificate generation enabled in an unsupported Region, fails with a 4xx
                error. For more information about the supported Regions, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/acm-regions.html">Supported
                    Regions</a> in the <i>Certificate Manager User
                Guide</i>.</p>
         </note> |
| `name` | String | ✅ | <p>A descriptive label that is associated with a fleet. Fleet names do not need to be unique.</p> |
| `metric_groups` | Vec<String> |  | <p>The name of an Amazon Web Services CloudWatch metric group to add this fleet to. A metric group is
            used to aggregate the metrics for multiple fleets. You can specify an existing metric
            group name or set a new name to create a new metric group. A fleet can be included in
            only one metric group at a time. </p> |
| `compute_type` | String |  | <p>The type of compute resource used to host your game servers. </p>
         <ul>
            <li>
               <p>
                  <code>EC2</code> – The game server build is deployed to Amazon EC2 instances for
                    cloud hosting. This is the default setting.</p>
            </li>
            <li>
               <p>
                  <code>ANYWHERE</code> – Game servers 
                    and supporting software are deployed to compute resources that you provide and
                    manage. With this compute type, you can also set the
                        <code>AnywhereConfiguration</code> parameter.</p>
            </li>
         </ul> |
| `new_game_session_protection_policy` | String |  | <p>The status of termination protection for active game sessions on the fleet. By
            default, this property is set to <code>NoProtection</code>. You can also set game
            session protection for an individual game session by calling <a href="gamelift/latest/apireference/API_UpdateGameSession.html">UpdateGameSession</a>.</p>
         <ul>
            <li>
               <p>
                  <b>NoProtection</b> - Game sessions can be terminated
                    during active gameplay as a result of a scale-down event. </p>
            </li>
            <li>
               <p>
                  <b>FullProtection</b> - Game sessions in
                        <code>ACTIVE</code> status cannot be terminated during a scale-down
                    event.</p>
            </li>
         </ul> |
| `runtime_configuration` | String |  | <p>Instructions for how to launch and run server processes on the fleet. Set runtime
            configuration for managed EC2 fleets. For an Anywhere fleets, set this
            parameter only if the fleet is running the Amazon GameLift Servers Agent. The runtime configuration
            defines one or more server process configurations. Each server process identifies a game
            executable or Realtime script file and the number of processes to run concurrently. </p>
         <note>
            <p>This parameter replaces the parameters <code>ServerLaunchPath</code> and
                    <code>ServerLaunchParameters</code>, which are still supported for backward
                compatibility.</p>
         </note> |
| `anywhere_configuration` | String |  | <p>Amazon GameLift Servers Anywhere configuration options.</p> |
| `fleet_type` | String |  | <p>Indicates whether to use On-Demand or Spot instances for this fleet. By default, this
            property is set to <code>ON_DEMAND</code>. Learn more about when to use <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-ec2-instances.html#gamelift-ec2-instances-spot"> On-Demand versus Spot Instances</a>. This fleet property can't be changed after the fleet is created.</p> |
| `server_launch_path` | String |  | <p>
            <b>This parameter is no longer used.</b> Specify a server
            launch path using the <code>RuntimeConfiguration</code> parameter. Requests that use
            this parameter instead continue to be valid.</p> |
| `log_paths` | String |  | <p>
            <b>This parameter is no longer used.</b> To specify where
            Amazon GameLift Servers should store log files once a server process shuts down, use the Amazon GameLift Servers server
            API <code>ProcessReady()</code> and specify one or more directory paths in
                <code>logParameters</code>. For more information, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-initialize">Initialize the server process</a> in the <i>Amazon GameLift Servers Developer
                Guide</i>. </p> |
| `ec2_inbound_permissions` | Vec<String> |  | <p>The IP address ranges and port settings that allow inbound traffic to access game
            server processes and other processes on this fleet. Set this parameter for managed EC2 fleets. You can leave this parameter empty when creating the fleet, but you must call 
            <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_UpdateFleetPortSettings">https://docs.aws.amazon.com/gamelift/latest/apireference/API_UpdateFleetPortSettings</a> to set it before players can connect to game sessions. 
            As a best practice, we recommend 
            opening ports for remote access only when you need them and closing them when you're finished. 
            For Amazon GameLift Servers Realtime fleets, Amazon GameLift Servers automatically sets TCP and UDP ranges.</p> |
| `ec2_instance_type` | String |  | <p>The Amazon GameLift Servers-supported Amazon EC2 instance type to use with managed EC2 fleets.
            Instance type determines the computing resources that will be used to host your game
            servers, including CPU, memory, storage, and networking capacity. See <a href="http://aws.amazon.com/ec2/instance-types/">Amazon Elastic Compute Cloud Instance Types</a> for
            detailed descriptions of Amazon EC2 instance types.</p> |
| `instance_role_credentials_provider` | String |  | <p>Prompts Amazon GameLift Servers to generate a shared credentials file for the IAM role
            that's defined in <code>InstanceRoleArn</code>. The shared credentials file is stored on
            each fleet instance and refreshed as needed. Use shared credentials for applications
            that are deployed along with the game server executable, if the game server is
            integrated with server SDK version 5.x. For more information about using shared
            credentials, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-resources.html"> Communicate
                with other Amazon Web Services resources from your fleets</a>.</p> |
| `description` | String |  | <p>A description for the fleet.</p> |
| `locations` | Vec<String> |  | <p>A set of remote locations to deploy additional instances to and manage as a
            multi-location fleet. Use this parameter when creating a fleet in Amazon Web Services Regions that
            support multiple locations. You can add any Amazon Web Services Region or Local Zone that's supported
            by Amazon GameLift Servers. Provide a list of one or more Amazon Web Services Region codes, such as
                <code>us-west-2</code>, or Local Zone names. When using this parameter, Amazon GameLift Servers
            requires you to include your home location in the request. For a list of supported
            Regions and Local Zones, see 
            <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-regions.html">
                Amazon GameLift Servers service locations</a> for managed hosting.</p> |
| `resource_creation_limit_policy` | String |  | <p>A policy that limits the number of game sessions that an individual player can create
            on instances in this fleet within a specified span of time.</p> |
| `peer_vpc_aws_account_id` | String |  | <p>Used when peering your Amazon GameLift Servers fleet with a VPC, the unique identifier for the Amazon Web Services
            account that owns the VPC. You can find your account ID in the Amazon Web Services Management Console under account
            settings. </p> |
| `instance_role_arn` | String |  | <p>A unique identifier for an IAM role that manages access to your Amazon Web Services services. 
        With an instance role ARN set, any application that runs on an instance in this fleet can assume the role, 
        including install scripts, server processes, and daemons (background processes). Create a role or look up a role's 
        ARN by using the <a href="https://console.aws.amazon.com/iam/">IAM dashboard</a> in the Amazon Web Services Management Console.
        Learn more about using on-box credentials for your game servers at 
        <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-resources.html">
        Access external resources from a game server</a>. This fleet property can't be changed after the fleet is created.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create fleet
fleet = provider.gamelift.Fleet {
    name = "value"  # <p>A descriptive label that is associated with a fleet. Fleet names do not need to be unique.</p>
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

# Create multiple fleet_events resources
fleet_events_0 = provider.gamelift.Fleet_events {
}
fleet_events_1 = provider.gamelift.Fleet_events {
}
fleet_events_2 = provider.gamelift.Fleet_events {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    fleet_events = provider.gamelift.Fleet_events {
    }
```

---

## Related Documentation

- [AWS Gamelift Documentation](https://docs.aws.amazon.com/gamelift/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
