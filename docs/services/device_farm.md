# Device_farm Service



**Resources**: 19

---

## Overview

The device_farm service provides access to 19 resource types:

- [Test_grid_url](#test_grid_url) [C]
- [Device_pool_compatibility](#device_pool_compatibility) [R]
- [Test](#test) [R]
- [Job](#job) [R]
- [Device_pool](#device_pool) [CRUD]
- [Run](#run) [RD]
- [Vpce_configuration](#vpce_configuration) [CRUD]
- [Suite](#suite) [R]
- [Remote_access_session](#remote_access_session) [CRD]
- [Instance_profile](#instance_profile) [CRUD]
- [Test_grid_project](#test_grid_project) [CRUD]
- [Device](#device) [R]
- [Device_instance](#device_instance) [RU]
- [Project](#project) [CRUD]
- [Upload](#upload) [CRUD]
- [Offering_status](#offering_status) [R]
- [Account_settings](#account_settings) [R]
- [Network_profile](#network_profile) [CRUD]
- [Test_grid_session](#test_grid_session) [R]

---

## Resources


### Test_grid_url

TestGridUrl resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project_arn` | String | ✅ | <p>ARN (from <a>CreateTestGridProject</a> or <a>ListTestGridProjects</a>) to associate
         with the short-term URL. </p> |
| `expires_in_seconds` | i64 | ✅ | <p>Lifetime, in seconds, of the URL.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create test_grid_url
test_grid_url = provider.device_farm.Test_grid_url {
    project_arn = "value"  # <p>ARN (from <a>CreateTestGridProject</a> or <a>ListTestGridProjects</a>) to associate
         with the short-term URL. </p>
    expires_in_seconds = "value"  # <p>Lifetime, in seconds, of the URL.</p>
}

```

---


### Device_pool_compatibility

DevicePoolCompatibility resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `compatible_devices` | Vec<String> | <p>Information about compatible devices.</p> |
| `incompatible_devices` | Vec<String> | <p>Information about incompatible devices.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access device_pool_compatibility outputs
device_pool_compatibility_id = device_pool_compatibility.id
device_pool_compatibility_compatible_devices = device_pool_compatibility.compatible_devices
device_pool_compatibility_incompatible_devices = device_pool_compatibility.incompatible_devices
```

---


### Test

Test resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `test` | String | <p>A test condition that is evaluated.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access test outputs
test_id = test.id
test_test = test.test
```

---


### Job

Job resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job` | String | <p>An object that contains information about the requested job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access job outputs
job_id = job.id
job_job = job.job
```

---


### Device_pool

DevicePool resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `max_devices` | i64 |  | <p>The number of devices that Device Farm can add to your device pool. Device Farm adds devices that are
            available and meet the criteria that you assign for the <code>rules</code> parameter. Depending on how many
            devices meet these constraints, your device pool might contain fewer devices than the value for this
            parameter.</p>
        <p>By specifying the maximum number of devices, you can control the costs that you incur
            by running tests.</p> |
| `description` | String |  | <p>The device pool's description.</p> |
| `project_arn` | String | ✅ | <p>The ARN of the project for the device pool.</p> |
| `name` | String | ✅ | <p>The device pool's name.</p> |
| `rules` | Vec<String> | ✅ | <p>The device pool's rules.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `device_pool` | String | <p>An object that contains information about the requested device pool.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create device_pool
device_pool = provider.device_farm.Device_pool {
    project_arn = "value"  # <p>The ARN of the project for the device pool.</p>
    name = "value"  # <p>The device pool's name.</p>
    rules = "value"  # <p>The device pool's rules.</p>
}

# Access device_pool outputs
device_pool_id = device_pool.id
device_pool_device_pool = device_pool.device_pool
```

---


### Run

Run resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `run` | String | <p>The run to get results from.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access run outputs
run_id = run.id
run_run = run.run
```

---


### Vpce_configuration

VPCEConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vpce_configuration_name` | String | ✅ | <p>The friendly name you give to your VPC endpoint configuration, to manage your
            configurations more easily.</p> |
| `service_dns_name` | String | ✅ | <p>The DNS name of the service running in your VPC that you want Device Farm to
            test.</p> |
| `vpce_service_name` | String | ✅ | <p>The name of the VPC endpoint service running in your AWS account that you want Device Farm to test.</p> |
| `vpce_configuration_description` | String |  | <p>An optional description that provides details about your VPC endpoint configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpce_configuration` | String | <p>An object that contains information about your VPC endpoint configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpce_configuration
vpce_configuration = provider.device_farm.Vpce_configuration {
    vpce_configuration_name = "value"  # <p>The friendly name you give to your VPC endpoint configuration, to manage your
            configurations more easily.</p>
    service_dns_name = "value"  # <p>The DNS name of the service running in your VPC that you want Device Farm to
            test.</p>
    vpce_service_name = "value"  # <p>The name of the VPC endpoint service running in your AWS account that you want Device Farm to test.</p>
}

# Access vpce_configuration outputs
vpce_configuration_id = vpce_configuration.id
vpce_configuration_vpce_configuration = vpce_configuration.vpce_configuration
```

---


### Suite

Suite resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `suite` | String | <p>A collection of one or more tests.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access suite outputs
suite_id = suite.id
suite_suite = suite.suite
```

---


### Remote_access_session

RemoteAccessSession resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `device_arn` | String | ✅ | <p>The ARN of the device for which you want to create a remote access session.</p> |
| `name` | String |  | <p>The name of the remote access session to create.</p> |
| `remote_record_enabled` | bool |  | <p>Set to <code>true</code> to enable remote recording for the remote access
            session.</p> |
| `app_arn` | String |  | <p>The Amazon Resource Name (ARN) of the app to create the remote access session.</p> |
| `project_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the project for which you want to create a remote
            access session.</p> |
| `instance_arn` | String |  | <p>The Amazon Resource Name (ARN) of the device instance for which you want to create a
            remote access session.</p> |
| `remote_record_app_arn` | String |  | <p>The Amazon Resource Name (ARN) for the app to be recorded in the remote access
            session.</p> |
| `ssh_public_key` | String |  | <p>Ignored. The public key of the <code>ssh</code> key pair you want to use for connecting to remote
            devices in your remote debugging session. This key is required only if <code>remoteDebugEnabled</code> is
            set to <code>true</code>.</p>
        <p>Remote debugging is <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/history.html">no
            longer supported</a>.</p> |
| `remote_debug_enabled` | bool |  | <p>Set to <code>true</code> if you want to access devices remotely for debugging in
            your remote access session.</p>
        <p>Remote debugging is <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/history.html">no
            longer supported</a>.</p> |
| `client_id` | String |  | <p>Unique identifier for the client. If you want access to multiple devices on the same client, you should
            pass the same <code>clientId</code> value in each call to <code>CreateRemoteAccessSession</code>. This
            identifier is required only if <code>remoteDebugEnabled</code> is set to <code>true</code>.</p>
        <p>Remote debugging is <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/history.html">no
            longer supported</a>.</p> |
| `configuration` | String |  | <p>The configuration information for the remote access session request.</p> |
| `interaction_mode` | String |  | <p>The interaction mode of the remote access session. Valid values are:</p>
        <ul>
            <li>
                <p>INTERACTIVE: You can interact with the iOS device by viewing, touching, and
                    rotating the screen. You cannot run XCUITest framework-based tests in this
                    mode.</p>
            </li>
            <li>
                <p>NO_VIDEO: You are connected to the device, but cannot interact with it or view the screen. This
                    mode has the fastest test execution speed. You can run XCUITest framework-based tests in this
                    mode.</p>
            </li>
            <li>
                <p>VIDEO_ONLY: You can view the screen, but cannot touch or rotate it. You can run XCUITest
                    framework-based tests and watch the screen in this mode.</p>
            </li>
         </ul> |
| `skip_app_resign` | bool |  | <p>When set to <code>true</code>, for private devices, Device Farm does not sign your app again. For public
            devices, Device Farm always signs your apps again.</p>
        <p>For more information on how Device Farm modifies your uploads during tests, see <a href="http://aws.amazon.com/device-farm/faqs/">Do you modify my app?</a>
         </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `remote_access_session` | String | <p>A container that lists detailed information about the remote access
            session.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create remote_access_session
remote_access_session = provider.device_farm.Remote_access_session {
    device_arn = "value"  # <p>The ARN of the device for which you want to create a remote access session.</p>
    project_arn = "value"  # <p>The Amazon Resource Name (ARN) of the project for which you want to create a remote
            access session.</p>
}

# Access remote_access_session outputs
remote_access_session_id = remote_access_session.id
remote_access_session_remote_access_session = remote_access_session.remote_access_session
```

---


### Instance_profile

InstanceProfile resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reboot_after_use` | bool |  | <p>When set to <code>true</code>, Device Farm reboots the instance after a test run. The default value is
                <code>true</code>.</p> |
| `name` | String | ✅ | <p>The name of your instance profile.</p> |
| `description` | String |  | <p>The description of your instance profile.</p> |
| `package_cleanup` | bool |  | <p>When set to <code>true</code>, Device Farm removes app packages after a test run. The default value is
                <code>false</code> for private devices.</p> |
| `exclude_app_packages_from_cleanup` | Vec<String> |  | <p>An array of strings that specifies the list of app packages that should not be cleaned up from the device
            after a test run.</p>
        <p>The list of packages is considered only if you set <code>packageCleanup</code> to
            <code>true</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_profile` | String | <p>An object that contains information about an instance profile.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create instance_profile
instance_profile = provider.device_farm.Instance_profile {
    name = "value"  # <p>The name of your instance profile.</p>
}

# Access instance_profile outputs
instance_profile_id = instance_profile.id
instance_profile_instance_profile = instance_profile.instance_profile
```

---


### Test_grid_project

TestGridProject resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vpc_config` | String |  | <p>The VPC security groups and subnets that are attached to a project.</p> |
| `name` | String | ✅ | <p>Human-readable name of the Selenium testing project.</p> |
| `description` | String |  | <p>Human-readable description of the project.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `test_grid_project` | String | <p>A <a>TestGridProject</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create test_grid_project
test_grid_project = provider.device_farm.Test_grid_project {
    name = "value"  # <p>Human-readable name of the Selenium testing project.</p>
}

# Access test_grid_project outputs
test_grid_project_id = test_grid_project.id
test_grid_project_test_grid_project = test_grid_project.test_grid_project
```

---


### Device

Device resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `device` | String | <p>An object that contains information about the requested device.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access device outputs
device_id = device.id
device_device = device.device
```

---


### Device_instance

DeviceInstance resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the device instance.</p> |
| `labels` | Vec<String> |  | <p>An array of strings that you want to associate with the device instance.</p> |
| `profile_arn` | String |  | <p>The ARN of the profile that you want to associate with the device instance.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `device_instance` | String | <p>An object that contains information about your device instance.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access device_instance outputs
device_instance_id = device_instance.id
device_instance_device_instance = device_instance.device_instance
```

---


### Project

Project resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The project's name.</p> |
| `default_job_timeout_minutes` | i64 |  | <p>Sets the execution timeout value (in minutes) for a project. All test runs in this project use the
            specified execution timeout value unless overridden when scheduling a run.</p> |
| `vpc_config` | String |  | <p>The VPC security groups and subnets that are attached to a project.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `project` | String | <p>The project to get information about.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create project
project = provider.device_farm.Project {
    name = "value"  # <p>The project's name.</p>
}

# Access project outputs
project_id = project.id
project_project = project.project
```

---


### Upload

Upload resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project_arn` | String | ✅ | <p>The ARN of the project for the upload.</p> |
| `type` | String | ✅ | <p>The upload's upload type.</p>
        <p>Must be one of the following values:</p>
        <ul>
            <li>
                <p>ANDROID_APP</p>
            </li>
            <li>
                <p>IOS_APP</p>
            </li>
            <li>
                <p>WEB_APP</p>
            </li>
            <li>
                <p>EXTERNAL_DATA</p>
            </li>
            <li>
                <p>APPIUM_JAVA_JUNIT_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_JAVA_TESTNG_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_PYTHON_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_NODE_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_RUBY_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_WEB_JAVA_JUNIT_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_WEB_JAVA_TESTNG_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_WEB_PYTHON_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_WEB_NODE_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_WEB_RUBY_TEST_PACKAGE</p>
            </li>
            <li>
                <p>INSTRUMENTATION_TEST_PACKAGE</p>
            </li>
            <li>
                <p>XCTEST_TEST_PACKAGE</p>
            </li>
            <li>
                <p>XCTEST_UI_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_JAVA_JUNIT_TEST_SPEC</p>
            </li>
            <li>
                <p>APPIUM_JAVA_TESTNG_TEST_SPEC</p>
            </li>
            <li>
                <p>APPIUM_PYTHON_TEST_SPEC</p>
            </li>
            <li>
                <p>APPIUM_NODE_TEST_SPEC</p>
            </li>
            <li>
                <p>APPIUM_RUBY_TEST_SPEC</p>
            </li>
            <li>
                <p>APPIUM_WEB_JAVA_JUNIT_TEST_SPEC</p>
            </li>
            <li>
                <p>APPIUM_WEB_JAVA_TESTNG_TEST_SPEC</p>
            </li>
            <li>
                <p>APPIUM_WEB_PYTHON_TEST_SPEC</p>
            </li>
            <li>
                <p>APPIUM_WEB_NODE_TEST_SPEC</p>
            </li>
            <li>
                <p>APPIUM_WEB_RUBY_TEST_SPEC</p>
            </li>
            <li>
                <p>INSTRUMENTATION_TEST_SPEC</p>
            </li>
            <li>
                <p>XCTEST_UI_TEST_SPEC</p>
            </li>
         </ul>
        <p> If you call <code>CreateUpload</code> with <code>WEB_APP</code> specified, AWS
            Device Farm throws an <code>ArgumentException</code> error.</p> |
| `name` | String | ✅ | <p>The upload's file name. The name should not contain any forward slashes (<code>/</code>). If you are
            uploading an iOS app, the file name must end with the <code>.ipa</code> extension. If you are uploading an
            Android app, the file name must end with the <code>.apk</code> extension. For all others, the file name must
            end with the <code>.zip</code> file extension.</p> |
| `content_type` | String |  | <p>The upload's content type (for example, <code>application/octet-stream</code>).</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `upload` | String | <p>An app or a set of one or more tests to upload or that have been
            uploaded.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create upload
upload = provider.device_farm.Upload {
    project_arn = "value"  # <p>The ARN of the project for the upload.</p>
    type = "value"  # <p>The upload's upload type.</p>
        <p>Must be one of the following values:</p>
        <ul>
            <li>
                <p>ANDROID_APP</p>
            </li>
            <li>
                <p>IOS_APP</p>
            </li>
            <li>
                <p>WEB_APP</p>
            </li>
            <li>
                <p>EXTERNAL_DATA</p>
            </li>
            <li>
                <p>APPIUM_JAVA_JUNIT_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_JAVA_TESTNG_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_PYTHON_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_NODE_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_RUBY_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_WEB_JAVA_JUNIT_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_WEB_JAVA_TESTNG_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_WEB_PYTHON_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_WEB_NODE_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_WEB_RUBY_TEST_PACKAGE</p>
            </li>
            <li>
                <p>INSTRUMENTATION_TEST_PACKAGE</p>
            </li>
            <li>
                <p>XCTEST_TEST_PACKAGE</p>
            </li>
            <li>
                <p>XCTEST_UI_TEST_PACKAGE</p>
            </li>
            <li>
                <p>APPIUM_JAVA_JUNIT_TEST_SPEC</p>
            </li>
            <li>
                <p>APPIUM_JAVA_TESTNG_TEST_SPEC</p>
            </li>
            <li>
                <p>APPIUM_PYTHON_TEST_SPEC</p>
            </li>
            <li>
                <p>APPIUM_NODE_TEST_SPEC</p>
            </li>
            <li>
                <p>APPIUM_RUBY_TEST_SPEC</p>
            </li>
            <li>
                <p>APPIUM_WEB_JAVA_JUNIT_TEST_SPEC</p>
            </li>
            <li>
                <p>APPIUM_WEB_JAVA_TESTNG_TEST_SPEC</p>
            </li>
            <li>
                <p>APPIUM_WEB_PYTHON_TEST_SPEC</p>
            </li>
            <li>
                <p>APPIUM_WEB_NODE_TEST_SPEC</p>
            </li>
            <li>
                <p>APPIUM_WEB_RUBY_TEST_SPEC</p>
            </li>
            <li>
                <p>INSTRUMENTATION_TEST_SPEC</p>
            </li>
            <li>
                <p>XCTEST_UI_TEST_SPEC</p>
            </li>
         </ul>
        <p> If you call <code>CreateUpload</code> with <code>WEB_APP</code> specified, AWS
            Device Farm throws an <code>ArgumentException</code> error.</p>
    name = "value"  # <p>The upload's file name. The name should not contain any forward slashes (<code>/</code>). If you are
            uploading an iOS app, the file name must end with the <code>.ipa</code> extension. If you are uploading an
            Android app, the file name must end with the <code>.apk</code> extension. For all others, the file name must
            end with the <code>.zip</code> file extension.</p>
}

# Access upload outputs
upload_id = upload.id
upload_upload = upload.upload
```

---


### Offering_status

OfferingStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_period` | HashMap<String, String> | <p>When specified, gets the offering status for the next period.</p> |
| `current` | HashMap<String, String> | <p>When specified, gets the offering status for the current period.</p> |
| `next_token` | String | <p>An identifier that was returned from the previous call to this operation, which can
            be used to return the next set of items in the list.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access offering_status outputs
offering_status_id = offering_status.id
offering_status_next_period = offering_status.next_period
offering_status_current = offering_status.current
offering_status_next_token = offering_status.next_token
```

---


### Account_settings

AccountSettings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_settings` | String | <p>The account settings.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_settings outputs
account_settings_id = account_settings.id
account_settings_account_settings = account_settings.account_settings
```

---


### Network_profile

NetworkProfile resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uplink_jitter_ms` | i64 |  | <p>Time variation in the delay of received packets in milliseconds as an integer from
            0 to 2000.</p> |
| `downlink_loss_percent` | i64 |  | <p>Proportion of received packets that fail to arrive from 0 to 100 percent.</p> |
| `name` | String | ✅ | <p>The name for the new network profile.</p> |
| `uplink_loss_percent` | i64 |  | <p>Proportion of transmitted packets that fail to arrive from 0 to 100
            percent.</p> |
| `downlink_delay_ms` | i64 |  | <p>Delay time for all packets to destination in milliseconds as an integer from 0 to
            2000.</p> |
| `uplink_delay_ms` | i64 |  | <p>Delay time for all packets to destination in milliseconds as an integer from 0 to
            2000.</p> |
| `downlink_bandwidth_bits` | i64 |  | <p>The data throughput rate in bits per second, as an integer from 0 to
            104857600.</p> |
| `type` | String |  | <p>The type of network profile to create. Valid values are listed here.</p> |
| `project_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the project for which you want to create a
            network profile.</p> |
| `description` | String |  | <p>The description of the network profile.</p> |
| `uplink_bandwidth_bits` | i64 |  | <p>The data throughput rate in bits per second, as an integer from 0 to
            104857600.</p> |
| `downlink_jitter_ms` | i64 |  | <p>Time variation in the delay of received packets in milliseconds as an integer from
            0 to 2000.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `network_profile` | String | <p>The network profile.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create network_profile
network_profile = provider.device_farm.Network_profile {
    name = "value"  # <p>The name for the new network profile.</p>
    project_arn = "value"  # <p>The Amazon Resource Name (ARN) of the project for which you want to create a
            network profile.</p>
}

# Access network_profile outputs
network_profile_id = network_profile.id
network_profile_network_profile = network_profile.network_profile
```

---


### Test_grid_session

TestGridSession resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `test_grid_session` | String | <p>The <a>TestGridSession</a> that was requested.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access test_grid_session outputs
test_grid_session_id = test_grid_session.id
test_grid_session_test_grid_session = test_grid_session.test_grid_session
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple test_grid_url resources
test_grid_url_0 = provider.device_farm.Test_grid_url {
    project_arn = "value-0"
    expires_in_seconds = "value-0"
}
test_grid_url_1 = provider.device_farm.Test_grid_url {
    project_arn = "value-1"
    expires_in_seconds = "value-1"
}
test_grid_url_2 = provider.device_farm.Test_grid_url {
    project_arn = "value-2"
    expires_in_seconds = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    test_grid_url = provider.device_farm.Test_grid_url {
        project_arn = "production-value"
        expires_in_seconds = "production-value"
    }
```

---

## Related Documentation

- [AWS Device_farm Documentation](https://docs.aws.amazon.com/device_farm/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
