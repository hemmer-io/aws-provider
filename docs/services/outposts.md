# Outposts Service



**Resources**: 11

---

## Overview

The outposts service provides access to 11 resource types:

- [Outpost_supported_instance_types](#outpost_supported_instance_types) [R]
- [Capacity_task](#capacity_task) [R]
- [Site](#site) [CRUD]
- [Outpost_billing_information](#outpost_billing_information) [R]
- [Outpost](#outpost) [CRUD]
- [Order](#order) [CR]
- [Catalog_item](#catalog_item) [R]
- [Connection](#connection) [R]
- [Site_rack_physical_properties](#site_rack_physical_properties) [U]
- [Site_address](#site_address) [RU]
- [Outpost_instance_types](#outpost_instance_types) [R]

---

## Resources


### Outpost_supported_instance_types

OutpostSupportedInstanceTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String |  |
| `instance_types` | Vec<String> |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access outpost_supported_instance_types outputs
outpost_supported_instance_types_id = outpost_supported_instance_types.id
outpost_supported_instance_types_next_token = outpost_supported_instance_types.next_token
outpost_supported_instance_types_instance_types = outpost_supported_instance_types.instance_types
```

---


### Capacity_task

CapacityTask resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `capacity_task_id` | String | <p>ID of the capacity task.</p> |
| `instances_to_exclude` | String | <p>Instances that the user specified they cannot stop in order to free up the capacity needed
      to run the capacity task.</p> |
| `failed` | String | <p>Reason why the capacity task failed.</p> |
| `dry_run` | bool | <p>Performs a dry run to determine if you are above or below instance capacity.</p> |
| `creation_date` | String | <p>The date the capacity task was created.</p> |
| `last_modified_date` | String | <p>The date the capacity task was last modified.</p> |
| `order_id` | String | <p>ID of the Amazon Web Services Outposts order associated with the specified capacity task.</p> |
| `asset_id` | String | <p>The ID of the Outpost asset. An Outpost asset can be a single server within an Outposts
      rack or an Outposts server configuration.</p> |
| `task_action_on_blocking_instances` | String | <p>User-specified option in case an instance is blocking the capacity task from running.
      Shows one of the following options:</p>
         <ul>
            <li>
               <p>
                  <code>WAIT_FOR_EVACUATION</code> - Checks every 10 minutes over 48 hours to determine
          if instances have stopped and capacity is available to complete the task.</p>
            </li>
            <li>
               <p>
                  <code>FAIL_TASK</code> - The capacity task fails.</p>
            </li>
         </ul> |
| `capacity_task_status` | String | <p>Status of the capacity task.</p>
         <p>A capacity task can have one of the following statuses:</p>
         <ul>
            <li>
               <p>
                  <code>REQUESTED</code> - The capacity task was created and is awaiting the next step
          by Amazon Web Services Outposts.</p>
            </li>
            <li>
               <p>
                  <code>IN_PROGRESS</code> - The capacity task is running and cannot be
          cancelled.</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code> - The capacity task could not be completed.</p>
            </li>
            <li>
               <p>
                  <code>COMPLETED</code> - The capacity task has completed successfully.</p>
            </li>
            <li>
               <p>
                  <code>WAITING_FOR_EVACUATION</code> - The capacity task requires capacity to run. You
          must stop the recommended EC2 running instances to free up capacity for the task to
          run.</p>
            </li>
            <li>
               <p>
                  <code>CANCELLATION_IN_PROGRESS</code> - The capacity task has been cancelled and is in
          the process of cleaning up resources.</p>
            </li>
            <li>
               <p>
                  <code>CANCELLED</code> - The capacity task is cancelled.</p>
            </li>
         </ul> |
| `outpost_id` | String | <p>ID of the Outpost associated with the specified capacity task.</p> |
| `requested_instance_pools` | Vec<String> | <p>List of instance pools requested in the capacity task.</p> |
| `completion_date` | String | <p>The date the capacity task ran successfully.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access capacity_task outputs
capacity_task_id = capacity_task.id
capacity_task_capacity_task_id = capacity_task.capacity_task_id
capacity_task_instances_to_exclude = capacity_task.instances_to_exclude
capacity_task_failed = capacity_task.failed
capacity_task_dry_run = capacity_task.dry_run
capacity_task_creation_date = capacity_task.creation_date
capacity_task_last_modified_date = capacity_task.last_modified_date
capacity_task_order_id = capacity_task.order_id
capacity_task_asset_id = capacity_task.asset_id
capacity_task_task_action_on_blocking_instances = capacity_task.task_action_on_blocking_instances
capacity_task_capacity_task_status = capacity_task.capacity_task_status
capacity_task_outpost_id = capacity_task.outpost_id
capacity_task_requested_instance_pools = capacity_task.requested_instance_pools
capacity_task_completion_date = capacity_task.completion_date
```

---


### Site

Site resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  |  |
| `name` | String | ✅ |  |
| `notes` | String |  | <p>Additional information that you provide about site access requirements, electrician
      scheduling, personal protective equipment, or regulation of equipment materials that could
      affect your installation process. </p> |
| `tags` | HashMap<String, String> |  | <p> The tags to apply to a site. </p> |
| `operating_address` | String |  | <p> The location to install and power on the hardware. This address might be different from
      the shipping address. </p> |
| `shipping_address` | String |  | <p> The location to ship the hardware. This address might be different from the operating
      address. </p> |
| `rack_physical_properties` | String |  | <p> Information about the physical and logistical details for the rack at this site.
      For more information
      about hardware requirements for racks, see <a href="https://docs.aws.amazon.com/outposts/latest/userguide/outposts-requirements.html#checklist">Network 
        readiness checklist</a> in the Amazon Web Services Outposts User Guide.
        </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `site` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create site
site = provider.outposts.Site {
    name = "value"  # Required field
}

# Access site outputs
site_id = site.id
site_site = site.site
```

---


### Outpost_billing_information

OutpostBillingInformation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subscriptions` | Vec<String> | <p>The subscription details for the specified Outpost.</p> |
| `next_token` | String |  |
| `contract_end_date` | String | <p>The date the current contract term ends for the specified Outpost. You must start the renewal or
      decommission process at least 5 business days before the current term for your
      Amazon Web Services Outposts ends. Failing to complete these steps at least 5 business days before the
      current term ends might result in unanticipated charges.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access outpost_billing_information outputs
outpost_billing_information_id = outpost_billing_information.id
outpost_billing_information_subscriptions = outpost_billing_information.subscriptions
outpost_billing_information_next_token = outpost_billing_information.next_token
outpost_billing_information_contract_end_date = outpost_billing_information.contract_end_date
```

---


### Outpost

Outpost resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `supported_hardware_type` | String |  | <p> The type of hardware for this Outpost. </p> |
| `description` | String |  |  |
| `availability_zone` | String |  |  |
| `name` | String | ✅ |  |
| `site_id` | String | ✅ | <p> The ID or the Amazon Resource Name (ARN) of the site. </p> |
| `availability_zone_id` | String |  |  |
| `tags` | HashMap<String, String> |  | <p>The tags to apply to the Outpost.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `outpost` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create outpost
outpost = provider.outposts.Outpost {
    name = "value"  # Required field
    site_id = "value"  # <p> The ID or the Amazon Resource Name (ARN) of the site. </p>
}

# Access outpost outputs
outpost_id = outpost.id
outpost_outpost = outpost.outpost
```

---


### Order

Order resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `outpost_identifier` | String | ✅ | <p> The ID or the Amazon Resource Name (ARN) of the Outpost. </p> |
| `payment_option` | String | ✅ | <p>The payment option.</p> |
| `line_items` | Vec<String> |  | <p>The line items that make up the order.</p> |
| `payment_term` | String |  | <p>The payment terms.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `order` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create order
order = provider.outposts.Order {
    outpost_identifier = "value"  # <p> The ID or the Amazon Resource Name (ARN) of the Outpost. </p>
    payment_option = "value"  # <p>The payment option.</p>
}

# Access order outputs
order_id = order.id
order_order = order.order
```

---


### Catalog_item

CatalogItem resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `catalog_item` | String | <p>Information about this catalog item.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access catalog_item outputs
catalog_item_id = catalog_item.id
catalog_item_catalog_item = catalog_item.catalog_item
```

---


### Connection

Connection resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connection_details` | String | <p> Information about the connection. </p> |
| `connection_id` | String | <p> The ID of the connection. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connection outputs
connection_id = connection.id
connection_connection_details = connection.connection_details
connection_connection_id = connection.connection_id
```

---


### Site_rack_physical_properties

SiteRackPhysicalProperties resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uplink_count` | String |  | <p>Racks come with two Outpost network devices. Depending on the supported uplink speed at
      the site, the Outpost network devices provide a variable number of uplinks. Specify the number
      of uplinks for each Outpost network device that you intend to use to connect the rack to your
      network. Note the correlation between <code>UplinkGbps</code> and <code>UplinkCount</code>. </p>
         <ul>
            <li>
               <p>1Gbps - Uplinks available: 1, 2, 4, 6, 8</p>
            </li>
            <li>
               <p>10Gbps - Uplinks available: 1, 2, 4, 8, 12, 16</p>
            </li>
            <li>
               <p>40 and 100 Gbps- Uplinks available: 1, 2, 4</p>
            </li>
         </ul> |
| `uplink_gbps` | String |  | <p>The uplink speed the rack should support for the connection to the Region. </p> |
| `maximum_supported_weight_lbs` | String |  | <p>The maximum rack weight that this site can support. <code>NO_LIMIT</code> is over 2000lbs.
    </p> |
| `power_draw_kva` | String |  | <p>The power draw, in kVA, available at the hardware placement position for the rack.</p> |
| `site_id` | String | ✅ | <p> The ID or the Amazon Resource Name (ARN) of the site. </p> |
| `power_connector` | String |  | <p>The power connector that Amazon Web Services should plan to provide for connections to the hardware.
      Note the correlation between <code>PowerPhase</code> and <code>PowerConnector</code>. </p>
         <ul>
            <li>
               <p>Single-phase AC feed</p>
               <ul>
                  <li>
                     <p>
                        <b>L6-30P</b> – (common in US); 30A; single phase</p>
                  </li>
                  <li>
                     <p>
                        <b>IEC309 (blue)</b> – P+N+E, 6hr; 32 A; single
              phase</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>Three-phase AC feed</p>
               <ul>
                  <li>
                     <p>
                        <b>AH530P7W (red)</b> – 3P+N+E, 7hr; 30A; three
              phase</p>
                  </li>
                  <li>
                     <p>
                        <b>AH532P6W (red)</b> – 3P+N+E, 6hr; 32A; three
              phase</p>
                  </li>
                  <li>
                     <p>
                        <b>CS8365C</b> – (common in US); 3P+E, 50A; three
              phase</p>
                  </li>
               </ul>
            </li>
         </ul> |
| `fiber_optic_cable_type` | String |  | <p>The type of fiber that you will use to attach the Outpost to your network. </p> |
| `optical_standard` | String |  | <p>The type of optical standard that you will use to attach the Outpost to your network. This
      field is dependent on uplink speed, fiber type, and distance to the upstream device.
      For more information
      about networking requirements for racks, see <a href="https://docs.aws.amazon.com/outposts/latest/userguide/outposts-requirements.html#facility-networking">Network</a> 
        in the Amazon Web Services Outposts User Guide.
        </p>
         <ul>
            <li>
               <p>
                  <code>OPTIC_10GBASE_SR</code>: 10GBASE-SR</p>
            </li>
            <li>
               <p>
                  <code>OPTIC_10GBASE_IR</code>: 10GBASE-IR</p>
            </li>
            <li>
               <p>
                  <code>OPTIC_10GBASE_LR</code>: 10GBASE-LR</p>
            </li>
            <li>
               <p>
                  <code>OPTIC_40GBASE_SR</code>: 40GBASE-SR</p>
            </li>
            <li>
               <p>
                  <code>OPTIC_40GBASE_ESR</code>: 40GBASE-ESR</p>
            </li>
            <li>
               <p>
                  <code>OPTIC_40GBASE_IR4_LR4L</code>: 40GBASE-IR (LR4L)</p>
            </li>
            <li>
               <p>
                  <code>OPTIC_40GBASE_LR4</code>: 40GBASE-LR4</p>
            </li>
            <li>
               <p>
                  <code>OPTIC_100GBASE_SR4</code>: 100GBASE-SR4</p>
            </li>
            <li>
               <p>
                  <code>OPTIC_100GBASE_CWDM4</code>: 100GBASE-CWDM4</p>
            </li>
            <li>
               <p>
                  <code>OPTIC_100GBASE_LR4</code>: 100GBASE-LR4</p>
            </li>
            <li>
               <p>
                  <code>OPTIC_100G_PSM4_MSA</code>: 100G PSM4 MSA</p>
            </li>
            <li>
               <p>
                  <code>OPTIC_1000BASE_LX</code>: 1000Base-LX</p>
            </li>
            <li>
               <p>
                  <code>OPTIC_1000BASE_SX</code> : 1000Base-SX</p>
            </li>
         </ul> |
| `power_phase` | String |  | <p>The power option that you can provide for hardware. </p>
         <ul>
            <li>
               <p>Single-phase AC feed: 200 V to 277 V, 50 Hz or 60 Hz</p>
            </li>
            <li>
               <p>Three-phase AC feed: 346 V to 480 V, 50 Hz or 60 Hz</p>
            </li>
         </ul> |
| `power_feed_drop` | String |  | <p>Indicates whether the power feed comes above or below the rack. </p> |



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


### Site_address

SiteAddress resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `address` | String | ✅ | <p> The address for the site. </p> |
| `address_type` | String | ✅ | <p> The type of the address. </p> |
| `site_id` | String | ✅ | <p> The ID or the Amazon Resource Name (ARN) of the site. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `address` | String | <p> Information about the address. </p> |
| `site_id` | String |  |
| `address_type` | String | <p>The type of the address you receive. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access site_address outputs
site_address_id = site_address.id
site_address_address = site_address.address
site_address_site_id = site_address.site_id
site_address_address_type = site_address.address_type
```

---


### Outpost_instance_types

OutpostInstanceTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `outpost_id` | String | <p> The ID of the Outpost. </p> |
| `next_token` | String |  |
| `outpost_arn` | String |  |
| `instance_types` | Vec<String> |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access outpost_instance_types outputs
outpost_instance_types_id = outpost_instance_types.id
outpost_instance_types_outpost_id = outpost_instance_types.outpost_id
outpost_instance_types_next_token = outpost_instance_types.next_token
outpost_instance_types_outpost_arn = outpost_instance_types.outpost_arn
outpost_instance_types_instance_types = outpost_instance_types.instance_types
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple outpost_supported_instance_types resources
outpost_supported_instance_types_0 = provider.outposts.Outpost_supported_instance_types {
}
outpost_supported_instance_types_1 = provider.outposts.Outpost_supported_instance_types {
}
outpost_supported_instance_types_2 = provider.outposts.Outpost_supported_instance_types {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    outpost_supported_instance_types = provider.outposts.Outpost_supported_instance_types {
    }
```

---

## Related Documentation

- [AWS Outposts Documentation](https://docs.aws.amazon.com/outposts/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
