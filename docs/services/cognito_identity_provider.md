# Cognito_identity_provider Service



**Resources**: 26

---

## Overview

The cognito_identity_provider service provides access to 26 resource types:

- [User_import_job](#user_import_job) [CR]
- [Log_delivery_configuration](#log_delivery_configuration) [R]
- [User_attribute_verification_code](#user_attribute_verification_code) [R]
- [Identity_provider](#identity_provider) [CRUD]
- [Csv_header](#csv_header) [R]
- [Risk_configuration](#risk_configuration) [R]
- [Ui_customization](#ui_customization) [R]
- [Signing_certificate](#signing_certificate) [R]
- [Web_authn_credential](#web_authn_credential) [D]
- [User_pool_client](#user_pool_client) [CRUD]
- [Terms](#terms) [CRUD]
- [User_attributes](#user_attributes) [UD]
- [Device_status](#device_status) [U]
- [Group](#group) [CRUD]
- [User_pool](#user_pool) [CRUD]
- [User_pool_domain](#user_pool_domain) [CRUD]
- [User_pool_mfa_config](#user_pool_mfa_config) [R]
- [Managed_login_branding](#managed_login_branding) [CRUD]
- [Tokens_from_refresh_token](#tokens_from_refresh_token) [R]
- [Managed_login_branding_by_client](#managed_login_branding_by_client) [R]
- [User](#user) [RD]
- [User_auth_factors](#user_auth_factors) [R]
- [Resource_server](#resource_server) [CRUD]
- [Auth_event_feedback](#auth_event_feedback) [U]
- [Identity_provider_by_identifier](#identity_provider_by_identifier) [R]
- [Device](#device) [R]

---

## Resources


### User_import_job

UserImportJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cloud_watch_logs_role_arn` | String | ✅ | <p>You must specify an IAM role that has permission to log import-job results to
            Amazon CloudWatch Logs. This parameter is the ARN of that role.</p> |
| `user_pool_id` | String | ✅ | <p>The ID of the user pool that you want to import users into.</p> |
| `job_name` | String | ✅ | <p>A friendly name for the user import job.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_import_job` | String | <p>The details of the user import job. Includes logging destination, status, and the Amazon S3
            pre-signed URL for CSV upload.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user_import_job
user_import_job = provider.cognito_identity_provider.User_import_job {
    cloud_watch_logs_role_arn = "value"  # <p>You must specify an IAM role that has permission to log import-job results to
            Amazon CloudWatch Logs. This parameter is the ARN of that role.</p>
    user_pool_id = "value"  # <p>The ID of the user pool that you want to import users into.</p>
    job_name = "value"  # <p>A friendly name for the user import job.</p>
}

# Access user_import_job outputs
user_import_job_id = user_import_job.id
user_import_job_user_import_job = user_import_job.user_import_job
```

---


### Log_delivery_configuration

LogDeliveryConfiguration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `log_delivery_configuration` | String | <p>The logging configuration of the requested user pool. Includes types of logs
            configured and their destinations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access log_delivery_configuration outputs
log_delivery_configuration_id = log_delivery_configuration.id
log_delivery_configuration_log_delivery_configuration = log_delivery_configuration.log_delivery_configuration
```

---


### User_attribute_verification_code

UserAttributeVerificationCode resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `code_delivery_details` | String | <p>Information about the delivery destination of the user attribute verification
            code.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access user_attribute_verification_code outputs
user_attribute_verification_code_id = user_attribute_verification_code.id
user_attribute_verification_code_code_delivery_details = user_attribute_verification_code.code_delivery_details
```

---


### Identity_provider

IdentityProvider resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_pool_id` | String | ✅ | <p>The Id of the user pool where you want to create an IdP.</p> |
| `provider_details` | HashMap<String, String> | ✅ | <p>The scopes, URLs, and identifiers for your external identity provider. The following
examples describe the provider detail keys for each IdP type. These values and their
schema are subject to change. Social IdP <code>authorize_scopes</code> values must match
the values listed here.</p>
         <dl>
            <dt>OpenID Connect (OIDC)</dt>
            <dd>
               <p>Amazon Cognito accepts the following elements when it can't discover endpoint
                URLs from <code>oidc_issuer</code>: <code>attributes_url</code>,
                    <code>authorize_url</code>, <code>jwks_uri</code>,
                    <code>token_url</code>.</p>
               <p>Create or update request: <code>"ProviderDetails": {
                    "attributes_request_method": "GET", "attributes_url":
                    "https://auth.example.com/userInfo", "authorize_scopes": "openid profile
                    email", "authorize_url": "https://auth.example.com/authorize",
                    "client_id": "1example23456789", "client_secret":
                    "provider-app-client-secret", "jwks_uri":
                    "https://auth.example.com/.well-known/jwks.json", "oidc_issuer":
                    "https://auth.example.com", "token_url": "https://example.com/token"
                    }</code>
               </p>
               <p>Describe response: <code>"ProviderDetails": { "attributes_request_method":
                    "GET", "attributes_url": "https://auth.example.com/userInfo",
                    "attributes_url_add_attributes": "false", "authorize_scopes": "openid
                    profile email", "authorize_url": "https://auth.example.com/authorize",
                    "client_id": "1example23456789", "client_secret":
                    "provider-app-client-secret", "jwks_uri":
                    "https://auth.example.com/.well-known/jwks.json", "oidc_issuer":
                    "https://auth.example.com", "token_url": "https://example.com/token"
                    }</code>
               </p>
            </dd>
            <dt>SAML</dt>
            <dd>
               <p>Create or update request with Metadata URL: <code>"ProviderDetails": { "IDPInit": "true",
                    "IDPSignout": "true", "EncryptedResponses" : "true", "MetadataURL":
                    "https://auth.example.com/sso/saml/metadata", "RequestSigningAlgorithm":
                    "rsa-sha256" }</code>
               </p>
               <p>Create or update request with Metadata file: <code>"ProviderDetails": { "IDPInit": "true",
                    "IDPSignout": "true", "EncryptedResponses" : "true",  
                    "MetadataFile": "[metadata XML]", "RequestSigningAlgorithm":
                    "rsa-sha256" }</code>
               </p>
               <p>The value of <code>MetadataFile</code> must be the plaintext metadata document with all 
                quote (") characters escaped by backslashes.</p>
               <p>Describe response: <code>"ProviderDetails": { "IDPInit": "true",
                    "IDPSignout": "true", "EncryptedResponses" : "true", "ActiveEncryptionCertificate": "[certificate]",
                    "MetadataURL": "https://auth.example.com/sso/saml/metadata", "RequestSigningAlgorithm":
                    "rsa-sha256", "SLORedirectBindingURI":
                    "https://auth.example.com/slo/saml", "SSORedirectBindingURI":
                    "https://auth.example.com/sso/saml" }</code>
               </p>
            </dd>
            <dt>LoginWithAmazon</dt>
            <dd>
               <p>Create or update request: <code>"ProviderDetails": { "authorize_scopes":
                    "profile postal_code", "client_id":
                    "amzn1.application-oa2-client.1example23456789", "client_secret":
                    "provider-app-client-secret"</code>
               </p>
               <p>Describe response: <code>"ProviderDetails": { "attributes_url":
                    "https://api.amazon.com/user/profile", "attributes_url_add_attributes":
                    "false", "authorize_scopes": "profile postal_code", "authorize_url":
                    "https://www.amazon.com/ap/oa", "client_id":
                    "amzn1.application-oa2-client.1example23456789", "client_secret":
                    "provider-app-client-secret", "token_request_method": "POST",
                    "token_url": "https://api.amazon.com/auth/o2/token" }</code>
               </p>
            </dd>
            <dt>Google</dt>
            <dd>
               <p>Create or update request: <code>"ProviderDetails": { "authorize_scopes":
                    "email profile openid", "client_id":
                    "1example23456789.apps.googleusercontent.com", "client_secret":
                    "provider-app-client-secret" }</code>
               </p>
               <p>Describe response: <code>"ProviderDetails": { "attributes_url":
                    "https://people.googleapis.com/v1/people/me?personFields=",
                    "attributes_url_add_attributes": "true", "authorize_scopes": "email
                    profile openid", "authorize_url":
                    "https://accounts.google.com/o/oauth2/v2/auth", "client_id":
                    "1example23456789.apps.googleusercontent.com", "client_secret":
                    "provider-app-client-secret", "oidc_issuer":
                    "https://accounts.google.com", "token_request_method": "POST",
                    "token_url": "https://www.googleapis.com/oauth2/v4/token"
                }</code>
               </p>
            </dd>
            <dt>SignInWithApple</dt>
            <dd>
               <p>Create or update request: <code>"ProviderDetails": { "authorize_scopes":
                    "email name", "client_id": "com.example.cognito", "private_key": "1EXAMPLE", 
                    "key_id": "2EXAMPLE", "team_id": "3EXAMPLE" }</code>
               </p>
               <p>Describe response: <code>"ProviderDetails": {
                    "attributes_url_add_attributes": "false", "authorize_scopes": "email
                    name", "authorize_url": "https://appleid.apple.com/auth/authorize",
                    "client_id": "com.example.cognito", "key_id": "1EXAMPLE", "oidc_issuer":
                    "https://appleid.apple.com", "team_id": "2EXAMPLE",
                    "token_request_method": "POST", "token_url":
                    "https://appleid.apple.com/auth/token" }</code>
               </p>
            </dd>
            <dt>Facebook</dt>
            <dd>
               <p>Create or update request: <code>"ProviderDetails": { "api_version": "v17.0", 
            "authorize_scopes": "public_profile, email", "client_id": "1example23456789", 
            "client_secret": "provider-app-client-secret" }</code>
               </p>
               <p>Describe response: <code>"ProviderDetails": 
            { "api_version": "v17.0", "attributes_url": "https://graph.facebook.com/v17.0/me?fields=", 
            "attributes_url_add_attributes": "true", "authorize_scopes": "public_profile, email", 
            "authorize_url": "https://www.facebook.com/v17.0/dialog/oauth", "client_id": 
            "1example23456789", "client_secret": "provider-app-client-secret", "token_request_method": 
            "GET", "token_url": "https://graph.facebook.com/v17.0/oauth/access_token" }</code>
               </p>
            </dd>
         </dl> |
| `provider_name` | String | ✅ | <p>The name that you want to assign to the IdP. You can pass the identity provider name
            in the <code>identity_provider</code> query parameter of requests to the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authorization-endpoint.html">Authorize endpoint</a> to silently redirect to sign-in with the associated
            IdP.</p> |
| `attribute_mapping` | HashMap<String, String> |  | <p>A mapping of IdP attributes to standard and custom user pool attributes. Specify a
            user pool attribute as the key of the key-value pair, and the IdP attribute claim name
            as the value.</p> |
| `idp_identifiers` | Vec<String> |  | <p>An array of IdP identifiers, for example <code>"IdPIdentifiers": [ "MyIdP", "MyIdP2"
                ]</code>. Identifiers are friendly names that you can pass in the
                <code>idp_identifier</code> query parameter of requests to the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authorization-endpoint.html">Authorize endpoint</a> to silently redirect to sign-in with the associated IdP.
            Identifiers in a domain format also enable the use of <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managing-saml-idp-naming.html">email-address matching with SAML providers</a>. </p> |
| `provider_type` | String | ✅ | <p>The type of IdP that you want to add. Amazon Cognito supports OIDC, SAML 2.0, Login With
            Amazon, Sign In With Apple, Google, and Facebook IdPs.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `identity_provider` | String | <p>The details of the requested IdP.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create identity_provider
identity_provider = provider.cognito_identity_provider.Identity_provider {
    user_pool_id = "value"  # <p>The Id of the user pool where you want to create an IdP.</p>
    provider_details = "value"  # <p>The scopes, URLs, and identifiers for your external identity provider. The following
examples describe the provider detail keys for each IdP type. These values and their
schema are subject to change. Social IdP <code>authorize_scopes</code> values must match
the values listed here.</p>
         <dl>
            <dt>OpenID Connect (OIDC)</dt>
            <dd>
               <p>Amazon Cognito accepts the following elements when it can't discover endpoint
                URLs from <code>oidc_issuer</code>: <code>attributes_url</code>,
                    <code>authorize_url</code>, <code>jwks_uri</code>,
                    <code>token_url</code>.</p>
               <p>Create or update request: <code>"ProviderDetails": {
                    "attributes_request_method": "GET", "attributes_url":
                    "https://auth.example.com/userInfo", "authorize_scopes": "openid profile
                    email", "authorize_url": "https://auth.example.com/authorize",
                    "client_id": "1example23456789", "client_secret":
                    "provider-app-client-secret", "jwks_uri":
                    "https://auth.example.com/.well-known/jwks.json", "oidc_issuer":
                    "https://auth.example.com", "token_url": "https://example.com/token"
                    }</code>
               </p>
               <p>Describe response: <code>"ProviderDetails": { "attributes_request_method":
                    "GET", "attributes_url": "https://auth.example.com/userInfo",
                    "attributes_url_add_attributes": "false", "authorize_scopes": "openid
                    profile email", "authorize_url": "https://auth.example.com/authorize",
                    "client_id": "1example23456789", "client_secret":
                    "provider-app-client-secret", "jwks_uri":
                    "https://auth.example.com/.well-known/jwks.json", "oidc_issuer":
                    "https://auth.example.com", "token_url": "https://example.com/token"
                    }</code>
               </p>
            </dd>
            <dt>SAML</dt>
            <dd>
               <p>Create or update request with Metadata URL: <code>"ProviderDetails": { "IDPInit": "true",
                    "IDPSignout": "true", "EncryptedResponses" : "true", "MetadataURL":
                    "https://auth.example.com/sso/saml/metadata", "RequestSigningAlgorithm":
                    "rsa-sha256" }</code>
               </p>
               <p>Create or update request with Metadata file: <code>"ProviderDetails": { "IDPInit": "true",
                    "IDPSignout": "true", "EncryptedResponses" : "true",  
                    "MetadataFile": "[metadata XML]", "RequestSigningAlgorithm":
                    "rsa-sha256" }</code>
               </p>
               <p>The value of <code>MetadataFile</code> must be the plaintext metadata document with all 
                quote (") characters escaped by backslashes.</p>
               <p>Describe response: <code>"ProviderDetails": { "IDPInit": "true",
                    "IDPSignout": "true", "EncryptedResponses" : "true", "ActiveEncryptionCertificate": "[certificate]",
                    "MetadataURL": "https://auth.example.com/sso/saml/metadata", "RequestSigningAlgorithm":
                    "rsa-sha256", "SLORedirectBindingURI":
                    "https://auth.example.com/slo/saml", "SSORedirectBindingURI":
                    "https://auth.example.com/sso/saml" }</code>
               </p>
            </dd>
            <dt>LoginWithAmazon</dt>
            <dd>
               <p>Create or update request: <code>"ProviderDetails": { "authorize_scopes":
                    "profile postal_code", "client_id":
                    "amzn1.application-oa2-client.1example23456789", "client_secret":
                    "provider-app-client-secret"</code>
               </p>
               <p>Describe response: <code>"ProviderDetails": { "attributes_url":
                    "https://api.amazon.com/user/profile", "attributes_url_add_attributes":
                    "false", "authorize_scopes": "profile postal_code", "authorize_url":
                    "https://www.amazon.com/ap/oa", "client_id":
                    "amzn1.application-oa2-client.1example23456789", "client_secret":
                    "provider-app-client-secret", "token_request_method": "POST",
                    "token_url": "https://api.amazon.com/auth/o2/token" }</code>
               </p>
            </dd>
            <dt>Google</dt>
            <dd>
               <p>Create or update request: <code>"ProviderDetails": { "authorize_scopes":
                    "email profile openid", "client_id":
                    "1example23456789.apps.googleusercontent.com", "client_secret":
                    "provider-app-client-secret" }</code>
               </p>
               <p>Describe response: <code>"ProviderDetails": { "attributes_url":
                    "https://people.googleapis.com/v1/people/me?personFields=",
                    "attributes_url_add_attributes": "true", "authorize_scopes": "email
                    profile openid", "authorize_url":
                    "https://accounts.google.com/o/oauth2/v2/auth", "client_id":
                    "1example23456789.apps.googleusercontent.com", "client_secret":
                    "provider-app-client-secret", "oidc_issuer":
                    "https://accounts.google.com", "token_request_method": "POST",
                    "token_url": "https://www.googleapis.com/oauth2/v4/token"
                }</code>
               </p>
            </dd>
            <dt>SignInWithApple</dt>
            <dd>
               <p>Create or update request: <code>"ProviderDetails": { "authorize_scopes":
                    "email name", "client_id": "com.example.cognito", "private_key": "1EXAMPLE", 
                    "key_id": "2EXAMPLE", "team_id": "3EXAMPLE" }</code>
               </p>
               <p>Describe response: <code>"ProviderDetails": {
                    "attributes_url_add_attributes": "false", "authorize_scopes": "email
                    name", "authorize_url": "https://appleid.apple.com/auth/authorize",
                    "client_id": "com.example.cognito", "key_id": "1EXAMPLE", "oidc_issuer":
                    "https://appleid.apple.com", "team_id": "2EXAMPLE",
                    "token_request_method": "POST", "token_url":
                    "https://appleid.apple.com/auth/token" }</code>
               </p>
            </dd>
            <dt>Facebook</dt>
            <dd>
               <p>Create or update request: <code>"ProviderDetails": { "api_version": "v17.0", 
            "authorize_scopes": "public_profile, email", "client_id": "1example23456789", 
            "client_secret": "provider-app-client-secret" }</code>
               </p>
               <p>Describe response: <code>"ProviderDetails": 
            { "api_version": "v17.0", "attributes_url": "https://graph.facebook.com/v17.0/me?fields=", 
            "attributes_url_add_attributes": "true", "authorize_scopes": "public_profile, email", 
            "authorize_url": "https://www.facebook.com/v17.0/dialog/oauth", "client_id": 
            "1example23456789", "client_secret": "provider-app-client-secret", "token_request_method": 
            "GET", "token_url": "https://graph.facebook.com/v17.0/oauth/access_token" }</code>
               </p>
            </dd>
         </dl>
    provider_name = "value"  # <p>The name that you want to assign to the IdP. You can pass the identity provider name
            in the <code>identity_provider</code> query parameter of requests to the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authorization-endpoint.html">Authorize endpoint</a> to silently redirect to sign-in with the associated
            IdP.</p>
    provider_type = "value"  # <p>The type of IdP that you want to add. Amazon Cognito supports OIDC, SAML 2.0, Login With
            Amazon, Sign In With Apple, Google, and Facebook IdPs.</p>
}

# Access identity_provider outputs
identity_provider_id = identity_provider.id
identity_provider_identity_provider = identity_provider.identity_provider
```

---


### Csv_header

CSVHeader resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `csv_header` | Vec<String> | <p>A comma-separated list of attributes from your user pool. Save this output to a
                <code>.csv</code> file and populate it with the attributes of the users that you
            want to import.</p> |
| `user_pool_id` | String | <p>The ID of the requested user pool.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access csv_header outputs
csv_header_id = csv_header.id
csv_header_csv_header = csv_header.csv_header
csv_header_user_pool_id = csv_header.user_pool_id
```

---


### Risk_configuration

RiskConfiguration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `risk_configuration` | String | <p>The details of the requested risk configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access risk_configuration outputs
risk_configuration_id = risk_configuration.id
risk_configuration_risk_configuration = risk_configuration.risk_configuration
```

---


### Ui_customization

UICustomization resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ui_customization` | String | <p>Information about the classic hosted UI custom CSS and logo-image branding that you
            applied to the user pool or app client.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ui_customization outputs
ui_customization_id = ui_customization.id
ui_customization_ui_customization = ui_customization.ui_customization
```

---


### Signing_certificate

SigningCertificate resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `certificate` | String | <p>The x.509 certificate that signs SAML 2.0 authentication requests for your user
            pool.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access signing_certificate outputs
signing_certificate_id = signing_certificate.id
signing_certificate_certificate = signing_certificate.certificate
```

---


### Web_authn_credential

WebAuthnCredential resource

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


### User_pool_client

UserPoolClient resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `generate_secret` | bool |  | <p>When <code>true</code>, generates a client secret for the app client. Client secrets
            are used with server-side and machine-to-machine applications. Client secrets are
            automatically generated; you can't specify a secret value. For more information,
            see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-client-apps.html#user-pool-settings-client-app-client-types">App client types</a>.</p> |
| `supported_identity_providers` | Vec<String> |  | <p>A list of provider names for the identity providers (IdPs) that are supported on this
            client. The following are supported: <code>COGNITO</code>, <code>Facebook</code>,
            <code>Google</code>, <code>SignInWithApple</code>, and <code>LoginWithAmazon</code>.
            You can also specify the names that you configured for the SAML and OIDC IdPs in your
            user pool, for example <code>MySAMLIdP</code> or <code>MyOIDCIdP</code>.</p>
         <p>This parameter sets the IdPs that <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html">managed 
            login</a> will display on the login page for your app client. The removal of 
            <code>COGNITO</code> from this list doesn't prevent authentication operations 
            for local users with the user pools API in an Amazon Web Services SDK. The only way to prevent 
            SDK-based authentication is to block access with a <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-waf.html">WAF rule</a>.
        </p> |
| `refresh_token_rotation` | String |  | <p>The configuration of your app client for refresh token rotation. When enabled, your
            app client issues new ID, access, and refresh tokens when users renew their sessions
            with refresh tokens. When disabled, token refresh issues only ID and access
            tokens.</p> |
| `auth_session_validity` | i64 |  | <p>Amazon Cognito creates a session token for each API request in an authentication flow. <code>AuthSessionValidity</code> is the duration, 
in minutes, of that session token. Your user pool native user must respond to each authentication challenge before the session expires.</p> |
| `callback_ur_ls` | Vec<String> |  | <p>A list of allowed redirect, or callback, URLs for managed login authentication. These
            URLs are the paths where you want to send your users' browsers after they complete
            authentication with managed login or a third-party IdP. Typically, callback URLs are the
            home of an application that uses OAuth or OIDC libraries to process authentication
            outcomes.</p>
         <p>A redirect URI must meet the following requirements:</p>
         <ul>
            <li>
               <p>Be an absolute URI.</p>
            </li>
            <li>
               <p>Be registered with the authorization server. Amazon Cognito doesn't accept
                    authorization requests with <code>redirect_uri</code> values that aren't in
                    the list of <code>CallbackURLs</code> that you provide in this parameter.</p>
            </li>
            <li>
               <p>Not include a fragment component.</p>
            </li>
         </ul>
         <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 -
                Redirection Endpoint</a>.</p>
         <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes
            only.</p>
         <p>App callback URLs such as myapp://example are also supported.</p> |
| `analytics_configuration` | String |  | <p>The user pool analytics configuration for collecting metrics and sending them to your
            Amazon Pinpoint campaign.</p>
         <p>In Amazon Web Services Regions where Amazon Pinpoint isn't available, user pools might not have access to
            analytics or might be configurable with campaigns in the US East (N. Virginia) Region.
            For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-pinpoint-integration.html">Using Amazon Pinpoint analytics</a>.</p> |
| `allowed_o_auth_flows_user_pool_client` | bool |  | <p>Set to <code>true</code> to use OAuth 2.0 authorization server features in your app client.</p>
         <p>This parameter must have a value of <code>true</code> before you can configure 
the following features in your app client.</p>
         <ul>
            <li>
               <p>
                  <code>CallBackURLs</code>: Callback URLs.</p>
            </li>
            <li>
               <p>
                  <code>LogoutURLs</code>: Sign-out redirect URLs.</p>
            </li>
            <li>
               <p>
                  <code>AllowedOAuthScopes</code>: OAuth 2.0 scopes.</p>
            </li>
            <li>
               <p>
                  <code>AllowedOAuthFlows</code>: Support for authorization code, implicit, and client credentials OAuth 2.0 grants.</p>
            </li>
         </ul>
         <p>To use authorization server features, configure one of these features in the Amazon Cognito console or set 
<code>AllowedOAuthFlowsUserPoolClient</code> to <code>true</code> in a <code>CreateUserPoolClient</code> or 
<code>UpdateUserPoolClient</code> API request. If you don't set a value for 
<code>AllowedOAuthFlowsUserPoolClient</code> in a request with the CLI or SDKs, it defaults 
to <code>false</code>. When <code>false</code>, only SDK-based API sign-in is permitted.</p> |
| `prevent_user_existence_errors` | String |  | <p>When <code>ENABLED</code>, suppresses messages that might indicate a valid user exists 
            when someone attempts sign-in. This parameters sets your preference for the errors and 
            responses that you want Amazon Cognito APIs to return during authentication, account
            confirmation, and password recovery when the user doesn't exist in the user pool. When
            set to <code>ENABLED</code> and the user doesn't exist, authentication returns an error
            indicating either the username or password was incorrect. Account confirmation and
            password recovery return a response indicating a code was sent to a simulated
            destination. When set to <code>LEGACY</code>, those APIs return a
            <code>UserNotFoundException</code> exception if the user doesn't exist in the user
            pool.</p>
         <p>Defaults to <code>LEGACY</code>.</p> |
| `enable_token_revocation` | bool |  | <p>Activates or deactivates <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/token-revocation.html">token
                revocation</a> in the target app client.</p>
         <p>If you don't include this parameter, token revocation is automatically activated for
            the new user pool client.</p> |
| `access_token_validity` | i64 |  | <p>The access token time limit. After this limit expires, your user can't use 
their access token. To specify the time unit for <code>AccessTokenValidity</code> as 
<code>seconds</code>, <code>minutes</code>, <code>hours</code>, or <code>days</code>, 
set a <code>TokenValidityUnits</code> value in your API request.</p>
         <p>For example, when you set <code>AccessTokenValidity</code> to <code>10</code> and
<code>TokenValidityUnits</code> to <code>hours</code>, your user can authorize access with
their access token for 10 hours.</p>
         <p>The default time unit for <code>AccessTokenValidity</code> in an API request is hours. 
<i>Valid range</i> is displayed below in seconds.</p>
         <p>If you don't specify otherwise in the configuration of your app client, your access
tokens are valid for one hour.</p> |
| `logout_ur_ls` | Vec<String> |  | <p>A list of allowed logout URLs for managed login authentication. When you pass
                <code>logout_uri</code> and <code>client_id</code> parameters to
                <code>/logout</code>, Amazon Cognito signs out your user and redirects them to the logout
            URL. This parameter describes the URLs that you want to be the permitted targets of
                <code>logout_uri</code>. A typical use of these URLs is when a user selects "Sign
            out" and you redirect them to your public homepage. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/logout-endpoint.html">Logout
                endpoint</a>.</p> |
| `read_attributes` | Vec<String> |  | <p>The list of user attributes that you want your app client to have read access to.
    After your user authenticates in your app, their access token authorizes them to read
    their own attribute value for any attribute in this list.</p>
         <p>When you don't specify the <code>ReadAttributes</code> for your app client, your
    app can read the values of <code>email_verified</code>,
    <code>phone_number_verified</code>, and the standard attributes of your user pool.
    When your user pool app client has read access to these default attributes,
    <code>ReadAttributes</code> doesn't return any information. Amazon Cognito only
    populates <code>ReadAttributes</code> in the API response if you have specified your own
    custom set of read attributes.</p> |
| `allowed_o_auth_scopes` | Vec<String> |  | <p>The OAuth, OpenID Connect (OIDC), and custom scopes that you want to permit your app
            client to authorize access with. Scopes govern access control to user pool self-service
            API operations, user data from the <code>userInfo</code> endpoint, and third-party APIs.
            Scope values include <code>phone</code>, <code>email</code>, <code>openid</code>, and
                <code>profile</code>. The <code>aws.cognito.signin.user.admin</code> scope
            authorizes user self-service operations. Custom scopes with resource servers authorize
            access to external APIs.</p> |
| `user_pool_id` | String | ✅ | <p>The ID of the user pool where you want to create an app client.</p> |
| `refresh_token_validity` | i64 |  | <p>The refresh token time limit. After this limit expires, your user can't use 
their refresh token. To specify the time unit for <code>RefreshTokenValidity</code> as 
<code>seconds</code>, <code>minutes</code>, <code>hours</code>, or <code>days</code>, 
set a <code>TokenValidityUnits</code> value in your API request.</p>
         <p>For example, when you set <code>RefreshTokenValidity</code> as <code>10</code> and
<code>TokenValidityUnits</code> as <code>days</code>, your user can refresh their session
and retrieve new access and ID tokens for 10 days.</p>
         <p>The default time unit for <code>RefreshTokenValidity</code> in an API request is days. 
You can't set <code>RefreshTokenValidity</code> to 0. If you do, Amazon Cognito overrides the 
value with the default value of 30 days. <i>Valid range</i> is displayed below 
in seconds.</p>
         <p>If you don't specify otherwise in the configuration of your app client, your refresh
tokens are valid for 30 days.</p> |
| `allowed_o_auth_flows` | Vec<String> |  | <p>The OAuth grant types that you want your app client to generate for clients in managed
            login authentication. To create an app client that generates client credentials grants,
            you must add <code>client_credentials</code> as the only allowed OAuth flow.</p>
         <dl>
            <dt>code</dt>
            <dd>
               <p>Use a code grant flow, which provides an authorization code as the
                        response. This code can be exchanged for access tokens with the
                            <code>/oauth2/token</code> endpoint.</p>
            </dd>
            <dt>implicit</dt>
            <dd>
               <p>Issue the access token, and the ID token when scopes like
                            <code>openid</code> and <code>profile</code> are requested, directly to
                        your user.</p>
            </dd>
            <dt>client_credentials</dt>
            <dd>
               <p>Issue the access token from the <code>/oauth2/token</code> endpoint
                        directly to a non-person user, authorized by a combination of the client ID
                        and client secret.</p>
            </dd>
         </dl> |
| `token_validity_units` | String |  | <p>The units that validity times are represented in. The default unit for refresh tokens
            is days, and the default for ID and access tokens are hours.</p> |
| `explicit_auth_flows` | Vec<String> |  | <p>The <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-authentication-flow-methods.html">authentication flows</a> that you want your user pool client to support. For each app 
client in your user pool, you can sign in your users with any combination of one or more flows, including with 
a user name and Secure Remote Password (SRP), a user name and password, or a custom authentication process that 
you define with Lambda functions.</p>
         <note>
            <p>If you don't specify a value for <code>ExplicitAuthFlows</code>, your app client supports 
<code>ALLOW_REFRESH_TOKEN_AUTH</code>, <code>ALLOW_USER_SRP_AUTH</code>, and <code>ALLOW_CUSTOM_AUTH</code>.
</p>
         </note>
         <p>The values for authentication flow options include the following.</p>
         <ul>
            <li>
               <p>
                  <code>ALLOW_USER_AUTH</code>: Enable selection-based sign-in
            with <code>USER_AUTH</code>. This setting covers username-password,
            secure remote password (SRP), passwordless, and passkey authentication.
            This authentiation flow can do username-password and SRP authentication
            without other <code>ExplicitAuthFlows</code> permitting them. For example
            users can complete an SRP challenge through <code>USER_AUTH</code> 
            without the flow <code>USER_SRP_AUTH</code> being active for the app
            client. This flow doesn't include <code>CUSTOM_AUTH</code>.
        </p>
               <p>To activate this setting, your user pool must be in the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/feature-plans-features-essentials.html">
                     Essentials tier</a> or higher.</p>
            </li>
            <li>
               <p>
                  <code>ALLOW_ADMIN_USER_PASSWORD_AUTH</code>: Enable admin based user password
            authentication flow <code>ADMIN_USER_PASSWORD_AUTH</code>. This setting replaces
            the <code>ADMIN_NO_SRP_AUTH</code> setting. With this authentication flow, your app
            passes a user name and password to Amazon Cognito in the request, instead of using the Secure 
            Remote Password (SRP) protocol to securely transmit the password.</p>
            </li>
            <li>
               <p>
                  <code>ALLOW_CUSTOM_AUTH</code>: Enable Lambda trigger based
            authentication.</p>
            </li>
            <li>
               <p>
                  <code>ALLOW_USER_PASSWORD_AUTH</code>: Enable user password-based
            authentication. In this flow, Amazon Cognito receives the password in the request instead
            of using the SRP protocol to verify passwords.</p>
            </li>
            <li>
               <p>
                  <code>ALLOW_USER_SRP_AUTH</code>: Enable SRP-based authentication.</p>
            </li>
            <li>
               <p>
                  <code>ALLOW_REFRESH_TOKEN_AUTH</code>: Enable authflow to refresh
            tokens.</p>
            </li>
         </ul>
         <p>In some environments, you will see the values <code>ADMIN_NO_SRP_AUTH</code>, <code>CUSTOM_AUTH_FLOW_ONLY</code>, or <code>USER_PASSWORD_AUTH</code>. 
You can't assign these legacy <code>ExplicitAuthFlows</code> values to user pool clients at the same time as values that begin with <code>ALLOW_</code>,
like <code>ALLOW_USER_SRP_AUTH</code>.</p> |
| `default_redirect_uri` | String |  | <p>The default redirect URI. In app clients with one assigned IdP, replaces
                <code>redirect_uri</code> in authentication requests. Must be in the
                <code>CallbackURLs</code> list.</p> |
| `enable_propagate_additional_user_context_data` | bool |  | <p>When <code>true</code>, your application can include additional
                <code>UserContextData</code> in authentication requests. This data includes the IP
            address, and contributes to analysis by threat protection features. For more information
            about propagation of user context data, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pool-settings-adaptive-authentication.html#user-pool-settings-adaptive-authentication-device-fingerprint">Adding session data to API requests</a>. If you don’t include this parameter,
            you can't send the source IP address to Amazon Cognito threat protection features. You can only
            activate <code>EnablePropagateAdditionalUserContextData</code> in an app client that has
            a client secret.</p> |
| `client_name` | String | ✅ | <p>A friendly name for the app client that you want to create.</p> |
| `id_token_validity` | i64 |  | <p>The ID token time limit. After this limit expires, your user can't use 
their ID token. To specify the time unit for <code>IdTokenValidity</code> as 
<code>seconds</code>, <code>minutes</code>, <code>hours</code>, or <code>days</code>, 
set a <code>TokenValidityUnits</code> value in your API request.</p>
         <p>For example, when you set <code>IdTokenValidity</code> as <code>10</code> and
<code>TokenValidityUnits</code> as <code>hours</code>, your user can authenticate their 
session with their ID token for 10 hours.</p>
         <p>The default time unit for <code>IdTokenValidity</code> in an API request is hours. 
<i>Valid range</i> is displayed below in seconds.</p>
         <p>If you don't specify otherwise in the configuration of your app client, your ID
tokens are valid for one hour.</p> |
| `write_attributes` | Vec<String> |  | <p>The list of user attributes that you want your app client to have write access to.
    After your user authenticates in your app, their access token authorizes them to set or
    modify their own attribute value for any attribute in this list.</p>
         <p>When you don't specify the <code>WriteAttributes</code> for your app client, your
    app can write the values of the Standard attributes of your user pool. When your user
    pool has write access to these default attributes, <code>WriteAttributes</code>
    doesn't return any information. Amazon Cognito only populates
        <code>WriteAttributes</code> in the API response if you have specified your own
    custom set of write attributes.</p>
         <p>If your app client allows users to sign in through an IdP, this array must include all
    attributes that you have mapped to IdP attributes. Amazon Cognito updates mapped attributes when
    users sign in to your application through an IdP. If your app client does not have write
    access to a mapped attribute, Amazon Cognito throws an error when it tries to update the
    attribute. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-specifying-attribute-mapping.html">Specifying IdP Attribute Mappings for Your user
    pool</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_pool_client` | String | <p>The details of the request app client.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user_pool_client
user_pool_client = provider.cognito_identity_provider.User_pool_client {
    user_pool_id = "value"  # <p>The ID of the user pool where you want to create an app client.</p>
    client_name = "value"  # <p>A friendly name for the app client that you want to create.</p>
}

# Access user_pool_client outputs
user_pool_client_id = user_pool_client.id
user_pool_client_user_pool_client = user_pool_client.user_pool_client
```

---


### Terms

Terms resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enforcement` | String | ✅ | <p>This parameter is reserved for future use and currently accepts only one value.</p> |
| `terms_source` | String | ✅ | <p>This parameter is reserved for future use and currently accepts only one value.</p> |
| `client_id` | String | ✅ | <p>The ID of the app client where you want to create terms documents. Must be an app
            client in the requested user pool.</p> |
| `links` | HashMap<String, String> |  | <p>A map of URLs to languages. For each localized language that will view the requested
                <code>TermsName</code>, assign a URL. A selection of <code>cognito:default</code>
            displays for all languages that don't have a language-specific URL.</p>
         <p>For example, <code>"cognito:default": "https://terms.example.com", "cognito:spanish":
                "https://terms.example.com/es"</code>.</p> |
| `user_pool_id` | String | ✅ | <p>The ID of the user pool where you want to create terms documents.</p> |
| `terms_name` | String | ✅ | <p>A friendly name for the document that you want to create in the current request. Must
            begin with <code>terms-of-use</code> or <code>privacy-policy</code> as identification of
            the document type. Provide URLs for both <code>terms-of-use</code> and
                <code>privacy-policy</code> in separate requests.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `terms` | String | <p>A summary of the requested terms documents. Includes a unique identifier for later
            changes to the terms documents.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create terms
terms = provider.cognito_identity_provider.Terms {
    enforcement = "value"  # <p>This parameter is reserved for future use and currently accepts only one value.</p>
    terms_source = "value"  # <p>This parameter is reserved for future use and currently accepts only one value.</p>
    client_id = "value"  # <p>The ID of the app client where you want to create terms documents. Must be an app
            client in the requested user pool.</p>
    user_pool_id = "value"  # <p>The ID of the user pool where you want to create terms documents.</p>
    terms_name = "value"  # <p>A friendly name for the document that you want to create in the current request. Must
            begin with <code>terms-of-use</code> or <code>privacy-policy</code> as identification of
            the document type. Provide URLs for both <code>terms-of-use</code> and
                <code>privacy-policy</code> in separate requests.</p>
}

# Access terms outputs
terms_id = terms.id
terms_terms = terms.terms
```

---


### User_attributes

UserAttributes resource

**Operations**: ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_metadata` | HashMap<String, String> |  | <p>A map of custom key-value pairs that you can provide as input for any custom workflows
            that this action initiates. </p>
         <p>You create custom workflows by assigning Lambda functions to user pool triggers. When
            you use the UpdateUserAttributes API action, Amazon Cognito invokes the function that is assigned
            to the <i>custom message</i> trigger. When Amazon Cognito invokes this function, it
            passes a JSON payload, which the function receives as input. This payload contains a
                <code>clientMetadata</code> attribute, which provides the data that you assigned to
            the ClientMetadata parameter in your UpdateUserAttributes request. In your function code
            in Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow
            for your specific needs.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html">
Using Lambda triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p>
         <note>
            <p>When you use the <code>ClientMetadata</code> parameter, note that Amazon Cognito won't do the
                following:</p>
            <ul>
               <li>
                  <p>Store the <code>ClientMetadata</code> value. This data is available only
                        to Lambda triggers that are assigned to a user pool to support custom
                        workflows. If your user pool configuration doesn't include triggers, the
                        <code>ClientMetadata</code> parameter serves no purpose.</p>
               </li>
               <li>
                  <p>Validate the <code>ClientMetadata</code> value.</p>
               </li>
               <li>
                  <p>Encrypt the <code>ClientMetadata</code> value. Don't send sensitive
                        information in this parameter.</p>
               </li>
            </ul>
         </note> |
| `user_attributes` | Vec<String> | ✅ | <p>An array of name-value pairs representing user attributes.</p>
         <p>For custom attributes, you must add a <code>custom:</code> prefix to the attribute
            name.</p>
         <p>If you have set an attribute to require verification before Amazon Cognito updates its value,
            this request doesn’t immediately update the value of that attribute. After your user
            receives and responds to a verification message to verify the new value, Amazon Cognito updates
            the attribute value. Your user can sign in and receive messages with the original
            attribute value until they verify the new value.</p> |
| `access_token` | String | ✅ | <p>A valid access token that Amazon Cognito issued to the currently signed-in user. Must include a scope claim for 
<code>aws.cognito.signin.user.admin</code>.</p> |



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


### Device_status

DeviceStatus resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `device_remembered_status` | String |  | <p>To enable device authentication with the specified device, set to
                <code>remembered</code>.To disable, set to <code>not_remembered</code>.</p> |
| `device_key` | String | ✅ | <p>The device key of the device you want to update, for example
                <code>us-west-2_a1b2c3d4-5678-90ab-cdef-EXAMPLE11111</code>.</p> |
| `access_token` | String | ✅ | <p>A valid access token that Amazon Cognito issued to the currently signed-in user. Must include a scope claim for 
<code>aws.cognito.signin.user.admin</code>.</p> |



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


### Group

Group resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_pool_id` | String | ✅ | <p>The ID of the user pool where you want to create a user group.</p> |
| `description` | String |  | <p>A description of the group that you're creating.</p> |
| `role_arn` | String |  | <p>The Amazon Resource Name (ARN) for the IAM role that you want to associate with the
            group. A group role primarily declares a preferred role for the credentials that you get
            from an identity pool. Amazon Cognito ID tokens have a <code>cognito:preferred_role</code> claim
            that presents the highest-precedence group that a user belongs to. Both ID and access
            tokens also contain a <code>cognito:groups</code> claim that list all the groups that a
            user is a member of.</p> |
| `precedence` | i64 |  | <p>A non-negative integer value that specifies the precedence of this group relative to
            the other groups that a user can belong to in the user pool. Zero is the highest
            precedence value. Groups with lower <code>Precedence</code> values take precedence over
            groups with higher or null <code>Precedence</code> values. If a user belongs to two or
            more groups, it is the group with the lowest precedence value whose role ARN is given in
            the user's tokens for the <code>cognito:roles</code> and
                <code>cognito:preferred_role</code> claims.</p>
         <p>Two groups can have the same <code>Precedence</code> value. If this happens, neither
            group takes precedence over the other. If two groups with the same
                <code>Precedence</code> have the same role ARN, that role is used in the
                <code>cognito:preferred_role</code> claim in tokens for users in each group. If the
            two groups have different role ARNs, the <code>cognito:preferred_role</code> claim isn't
            set in users' tokens.</p>
         <p>The default <code>Precedence</code> value is null. The maximum <code>Precedence</code>
            value is <code>2^31-1</code>.</p> |
| `group_name` | String | ✅ | <p>A name for the group. This name must be unique in your user pool.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `group` | String | <p>A container for the requested group. Includes description, precedence, and IAM role
            values.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create group
group = provider.cognito_identity_provider.Group {
    user_pool_id = "value"  # <p>The ID of the user pool where you want to create a user group.</p>
    group_name = "value"  # <p>A name for the group. This name must be unique in your user pool.</p>
}

# Access group outputs
group_id = group.id
group_group = group.group
```

---


### User_pool

UserPool resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `device_configuration` | String |  | <p>The device-remembering configuration for a user pool. Device remembering or device
            tracking is a "Remember me on this device" option for user pools that perform
            authentication with the device key of a trusted device in the back end, instead of a
            user-provided MFA code. For more information about device authentication, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with user devices in your user pool</a>. A null value indicates that
            you have deactivated device remembering in your user pool.</p>
         <note>
            <p>When you provide a value for any <code>DeviceConfiguration</code> field, you
                activate the Amazon Cognito device-remembering feature. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with devices</a>.</p>
         </note> |
| `email_configuration` | String |  | <p>The email configuration of your user pool. The email configuration type sets your
            preferred sending method, Amazon Web Services Region, and sender for messages from your user
            pool.</p> |
| `verification_message_template` | String |  | <p>The template for the verification message that your user pool delivers to users who
            set an email address or phone number attribute.</p>
         <p>Set the email message type that corresponds to your <code>DefaultEmailOption</code>
            selection. For <code>CONFIRM_WITH_LINK</code>, specify an
                <code>EmailMessageByLink</code> and leave <code>EmailMessage</code> blank. For
                <code>CONFIRM_WITH_CODE</code>, specify an <code>EmailMessage</code> and leave
                <code>EmailMessageByLink</code> blank. When you supply both parameters with either
            choice, Amazon Cognito returns an error.</p> |
| `email_verification_message` | String |  | <p>This parameter is no longer used.</p> |
| `alias_attributes` | Vec<String> |  | <p>Attributes supported as an alias for this user pool. For more information about alias
            attributes, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-attributes.html#user-pool-settings-aliases">Customizing sign-in attributes</a>.</p> |
| `user_attribute_update_settings` | String |  | <p>The settings for updates to user attributes. These settings include the property <code>AttributesRequireVerificationBeforeUpdate</code>,
a user-pool setting that tells Amazon Cognito how to handle changes to the value of your users' email address and phone number attributes. For
more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-email-phone-verification.html#user-pool-settings-verifications-verify-attribute-updates">
Verifying updates to email addresses and phone numbers</a>.</p> |
| `user_pool_tags` | HashMap<String, String> |  | <p>The tag keys and values to assign to the user pool. A tag is a label that you can use
            to categorize and manage user pools in different ways, such as by purpose, owner,
            environment, or other criteria.</p> |
| `admin_create_user_config` | String |  | <p>The configuration for administrative creation of users. Includes the template for the
            invitation message for new users, the duration of temporary passwords, and permitting
            self-service sign-up.</p> |
| `schema` | Vec<String> |  | <p>An array of attributes for the new user pool. You can add custom attributes and modify
            the properties of default attributes. The specifications in this parameter set the
            required attributes in your user pool. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-attributes.html">Working with user attributes</a>.</p> |
| `user_pool_tier` | String |  | <p>The user pool <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-sign-in-feature-plans.html">feature plan</a>, or tier. This parameter determines the
            eligibility of the user pool for features like managed login, access-token
            customization, and threat protection. Defaults to <code>ESSENTIALS</code>.</p> |
| `pool_name` | String | ✅ | <p>A friendly name for your user pool.</p> |
| `username_attributes` | Vec<String> |  | <p>Specifies whether a user can use an email address or phone number as a username when
            they sign up. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-attributes.html#user-pool-settings-aliases">Customizing sign-in attributes</a>.</p> |
| `sms_configuration` | String |  | <p>The settings for your Amazon Cognito user pool to send SMS messages with Amazon Simple Notification Service. To send SMS
            messages with Amazon SNS in the Amazon Web Services Region that you want, the Amazon Cognito user pool uses an
            Identity and Access Management (IAM) role in your Amazon Web Services account. For more information see
                <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-sms-settings.html">SMS message settings</a>.</p> |
| `username_configuration` | String |  | <p>Sets the case sensitivity option for sign-in usernames. When
                <code>CaseSensitive</code> is <code>false</code> (case insensitive), users can sign
            in with any combination of capital and lowercase letters. For example,
                <code>username</code>, <code>USERNAME</code>, or <code>UserName</code>, or for
            email, <code>email@example.com</code> or <code>EMaiL@eXamplE.Com</code>. For most use
            cases, set case sensitivity to <code>false</code> as a best practice. When usernames and
            email addresses are case insensitive, Amazon Cognito treats any variation in case as the same
            user, and prevents a case variation from being assigned to the same attribute for a
            different user.</p>
         <p>When <code>CaseSensitive</code> is <code>true</code> (case sensitive), Amazon Cognito
            interprets <code>USERNAME</code> and <code>UserName</code> as distinct users.</p>
         <p>This configuration is immutable after you set it.</p> |
| `sms_verification_message` | String |  | <p>This parameter is no longer used.</p> |
| `email_verification_subject` | String |  | <p>This parameter is no longer used.</p> |
| `deletion_protection` | String |  | <p>When active, <code>DeletionProtection</code> prevents accidental deletion of your user
pool. Before you can delete a user pool that you have protected against deletion, you
must deactivate this feature.</p>
         <p>When you try to delete a protected user pool in a <code>DeleteUserPool</code> API request, 
Amazon Cognito returns an <code>InvalidParameterException</code> error. To delete a protected user pool, 
send a new <code>DeleteUserPool</code> request after you deactivate deletion protection in an 
<code>UpdateUserPool</code> API request.</p> |
| `account_recovery_setting` | String |  | <p>The available verified method a user can use to recover their password when they call
                <code>ForgotPassword</code>. You can use this setting to define a preferred method
            when a user has more than one method available. With this setting, SMS doesn't qualify
            for a valid password recovery mechanism if the user also has SMS multi-factor
            authentication (MFA) activated. Email MFA is also disqualifying for account recovery
            with email. In the absence of this setting, Amazon Cognito uses the legacy behavior to determine
            the recovery method where SMS is preferred over email.</p>
         <p>As a best practice, configure both <code>verified_email</code> and
                <code>verified_phone_number</code>, with one having a higher priority than the
            other.</p> |
| `policies` | String |  | <p>The password policy and sign-in policy in the user pool. The password policy sets
            options like password complexity requirements and password history. The sign-in policy
            sets the options available to applications in <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flows-selection-sdk.html#authentication-flows-selection-choice">choice-based authentication</a>.</p> |
| `user_pool_add_ons` | String |  | <p>Contains settings for activation of threat protection, including the operating 
mode and additional authentication types. To log user security information but take 
no action, set to <code>AUDIT</code>. To configure automatic security responses to 
potentially unwanted traffic to your user pool, set to <code>ENFORCED</code>.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pool-settings-advanced-security.html">Adding advanced security to a user pool</a>. To activate this setting, your user pool must be on the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/feature-plans-features-plus.html">
                     Plus tier</a>.</p> |
| `auto_verified_attributes` | Vec<String> |  | <p>The attributes that you want your user pool to automatically verify. For more
            information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/signing-up-users-in-your-app.html#allowing-users-to-sign-up-and-confirm-themselves">Verifying contact information at sign-up</a>.</p> |
| `lambda_config` | String |  | <p>A collection of user pool Lambda triggers. Amazon Cognito invokes triggers at several possible
            stages of authentication operations. Triggers can modify the outcome of the operations
            that invoked them.</p> |
| `sms_authentication_message` | String |  | <p>The contents of the SMS message that your user pool sends to users in SMS OTP and MFA
            authentication.</p> |
| `mfa_configuration` | String |  | <p>Sets multi-factor authentication (MFA) to be on, off, or optional. When
                <code>ON</code>, all users must set up MFA before they can sign in. When
                <code>OPTIONAL</code>, your application must make a client-side determination of
            whether a user wants to register an MFA device. For user pools with adaptive
            authentication with threat protection, choose <code>OPTIONAL</code>.</p>
         <p>When <code>MfaConfiguration</code> is <code>OPTIONAL</code>, managed login
            doesn't automatically prompt users to set up MFA. Amazon Cognito generates MFA prompts in
            API responses and in managed login for users who have chosen and configured a preferred
            MFA factor.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_pool` | String | <p>The details of the requested user pool.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user_pool
user_pool = provider.cognito_identity_provider.User_pool {
    pool_name = "value"  # <p>A friendly name for your user pool.</p>
}

# Access user_pool outputs
user_pool_id = user_pool.id
user_pool_user_pool = user_pool.user_pool
```

---


### User_pool_domain

UserPoolDomain resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `managed_login_version` | i64 |  | <p>The version of managed login branding that you want to apply to your domain. A value
            of <code>1</code> indicates hosted UI (classic) and a version of <code>2</code>
            indicates managed login.</p>
         <p>Managed login requires that your user pool be configured for any <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-sign-in-feature-plans.html">feature plan</a> other than <code>Lite</code>.</p> |
| `user_pool_id` | String | ✅ | <p>The ID of the user pool where you want to add a domain.</p> |
| `domain` | String | ✅ | <p>The domain string. For custom domains, this is the fully-qualified domain name, such
            as <code>auth.example.com</code>. For prefix domains, this is the prefix alone, such as
                <code>myprefix</code>. A prefix value of <code>myprefix</code> for a user pool in
            the <code>us-east-1</code> Region results in a domain of
                <code>myprefix.auth.us-east-1.amazoncognito.com</code>.</p> |
| `custom_domain_config` | String |  | <p>The configuration for a custom domain. Configures your domain with an Certificate Manager
            certificate in the <code>us-east-1</code> Region.</p>
         <p>Provide this parameter only if you want to use a <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-add-custom-domain.html">custom domain</a> for your user pool. Otherwise, you can
            omit this parameter and use a <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-assign-domain-prefix.html">prefix domain</a> instead.</p>
         <p>When you create a custom domain, the passkey RP ID defaults to the custom domain. If
            you had a prefix domain active, this will cause passkey integration for your prefix
            domain to stop working due to a mismatch in RP ID. To keep the prefix domain passkey
            integration working, you can explicitly set RP ID to the prefix domain.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_description` | String | <p>The details of the requested user pool domain.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user_pool_domain
user_pool_domain = provider.cognito_identity_provider.User_pool_domain {
    user_pool_id = "value"  # <p>The ID of the user pool where you want to add a domain.</p>
    domain = "value"  # <p>The domain string. For custom domains, this is the fully-qualified domain name, such
            as <code>auth.example.com</code>. For prefix domains, this is the prefix alone, such as
                <code>myprefix</code>. A prefix value of <code>myprefix</code> for a user pool in
            the <code>us-east-1</code> Region results in a domain of
                <code>myprefix.auth.us-east-1.amazoncognito.com</code>.</p>
}

# Access user_pool_domain outputs
user_pool_domain_id = user_pool_domain.id
user_pool_domain_domain_description = user_pool_domain.domain_description
```

---


### User_pool_mfa_config

UserPoolMfaConfig resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mfa_configuration` | String | <p>Displays the state of multi-factor authentication (MFA) as on, off, or optional. When
                <code>ON</code>, all users must set up MFA before they can sign in. When
                <code>OPTIONAL</code>, your application must make a client-side determination of
            whether a user wants to register an MFA device. For user pools with adaptive
            authentication with threat protection, choose <code>OPTIONAL</code>.</p>
         <p>When <code>MfaConfiguration</code> is <code>OPTIONAL</code>, managed login
            doesn't automatically prompt users to set up MFA. Amazon Cognito generates MFA prompts in
            API responses and in managed login for users who have chosen and configured a preferred
            MFA factor.</p> |
| `web_authn_configuration` | String | <p>Shows user pool configuration for sign-in with passkey authenticators like biometric
            devices and security keys. Passkeys are not eligible MFA factors. They are instead an
            eligible primary sign-in factor for <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flows-selection-sdk.html#authentication-flows-selection-choice">choice-based authentication</a>, or the
                <code>USER_AUTH</code> flow.</p> |
| `sms_mfa_configuration` | String | <p>Shows user pool configuration for SMS message MFA. Includes the message template and
            the SMS message sending configuration for Amazon SNS.</p> |
| `software_token_mfa_configuration` | String | <p>Shows user pool configuration for time-based one-time password (TOTP) MFA. Includes
            TOTP enabled or disabled state.</p> |
| `email_mfa_configuration` | String | <p>Shows configuration for user pool email message MFA and sign-in with one-time
            passwords (OTPs). Includes the subject and body of the email message template for
            sign-in and MFA messages. To activate this setting, your user pool must be in the <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/feature-plans-features-essentials.html">
                     Essentials tier</a> or higher.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access user_pool_mfa_config outputs
user_pool_mfa_config_id = user_pool_mfa_config.id
user_pool_mfa_config_mfa_configuration = user_pool_mfa_config.mfa_configuration
user_pool_mfa_config_web_authn_configuration = user_pool_mfa_config.web_authn_configuration
user_pool_mfa_config_sms_mfa_configuration = user_pool_mfa_config.sms_mfa_configuration
user_pool_mfa_config_software_token_mfa_configuration = user_pool_mfa_config.software_token_mfa_configuration
user_pool_mfa_config_email_mfa_configuration = user_pool_mfa_config.email_mfa_configuration
```

---


### Managed_login_branding

ManagedLoginBranding resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `use_cognito_provided_values` | bool |  | <p>When true, applies the default branding style options. These default options are
            managed by Amazon Cognito. You can modify them later in the branding editor.</p>
         <p>When you specify <code>true</code> for this option, you must also omit values for
                <code>Settings</code> and <code>Assets</code> in the request.</p> |
| `settings` | String |  | <p>A JSON file, encoded as a <code>Document</code> type, with the the settings that you
            want to apply to your style.</p>
         <p>The following components are not currently implemented and reserved for future
            use:</p>
         <ul>
            <li>
               <p>
                  <code>signUp</code>
               </p>
            </li>
            <li>
               <p>
                  <code>instructions</code>
               </p>
            </li>
            <li>
               <p>
                  <code>sessionTimerDisplay</code>
               </p>
            </li>
            <li>
               <p>
                  <code>languageSelector</code> (for localization, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-managed-login.html#managed-login-localization">Managed login localization)</a>
               </p>
            </li>
         </ul> |
| `assets` | Vec<String> |  | <p>An array of image files that you want to apply to functions like backgrounds, logos,
            and icons. Each object must also indicate whether it is for dark mode, light mode, or
            browser-adaptive mode.</p> |
| `client_id` | String | ✅ | <p>The app client that you want to create the branding style for. Each style is linked to
            an app client until you delete it.</p> |
| `user_pool_id` | String | ✅ | <p>The ID of the user pool where you want to create a new branding style.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `managed_login_branding` | String | <p>The details of the requested branding style.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create managed_login_branding
managed_login_branding = provider.cognito_identity_provider.Managed_login_branding {
    client_id = "value"  # <p>The app client that you want to create the branding style for. Each style is linked to
            an app client until you delete it.</p>
    user_pool_id = "value"  # <p>The ID of the user pool where you want to create a new branding style.</p>
}

# Access managed_login_branding outputs
managed_login_branding_id = managed_login_branding.id
managed_login_branding_managed_login_branding = managed_login_branding.managed_login_branding
```

---


### Tokens_from_refresh_token

TokensFromRefreshToken resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `authentication_result` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access tokens_from_refresh_token outputs
tokens_from_refresh_token_id = tokens_from_refresh_token.id
tokens_from_refresh_token_authentication_result = tokens_from_refresh_token.authentication_result
```

---


### Managed_login_branding_by_client

ManagedLoginBrandingByClient resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `managed_login_branding` | String | <p>The details of the requested branding style.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access managed_login_branding_by_client outputs
managed_login_branding_by_client_id = managed_login_branding_by_client.id
managed_login_branding_by_client_managed_login_branding = managed_login_branding_by_client.managed_login_branding
```

---


### User

User resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mfa_options` | Vec<String> | <p>
            <i>This response parameter is no longer supported.</i> It provides
            information only about SMS MFA configurations. It doesn't provide information about
            time-based one-time password (TOTP) software token MFA configurations. To look up
            information about either type of MFA configuration, use UserMFASettingList
            instead.</p> |
| `username` | String | <p>The name of the user that you requested.</p> |
| `preferred_mfa_setting` | String | <p>The user's preferred MFA. Users can prefer SMS message, email message, or TOTP
            MFA.</p> |
| `user_attributes` | Vec<String> | <p>An array of name-value pairs representing user attributes.</p>
         <p>Custom attributes are prepended with the <code>custom:</code> prefix.</p> |
| `user_mfa_setting_list` | Vec<String> | <p>The MFA options that are activated for the user. The possible values in this list are
                <code>SMS_MFA</code>, <code>EMAIL_OTP</code>, and
            <code>SOFTWARE_TOKEN_MFA</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access user outputs
user_id = user.id
user_mfa_options = user.mfa_options
user_username = user.username
user_preferred_mfa_setting = user.preferred_mfa_setting
user_user_attributes = user.user_attributes
user_user_mfa_setting_list = user.user_mfa_setting_list
```

---


### User_auth_factors

UserAuthFactors resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `username` | String | <p>The name of the user who is eligible for the authentication factors in the
            response.</p> |
| `user_mfa_setting_list` | Vec<String> | <p>The MFA options that are activated for the user. The possible values in this list are
                <code>SMS_MFA</code>, <code>EMAIL_OTP</code>, and
            <code>SOFTWARE_TOKEN_MFA</code>.</p> |
| `configured_user_auth_factors` | Vec<String> | <p>The authentication types that are available to the user with <code>USER_AUTH</code>
            sign-in, for example <code>["PASSWORD", "WEB_AUTHN"]</code>.</p> |
| `preferred_mfa_setting` | String | <p>The challenge method that Amazon Cognito returns to the user in response to sign-in requests.
            Users can prefer SMS message, email message, or TOTP MFA.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access user_auth_factors outputs
user_auth_factors_id = user_auth_factors.id
user_auth_factors_username = user_auth_factors.username
user_auth_factors_user_mfa_setting_list = user_auth_factors.user_mfa_setting_list
user_auth_factors_configured_user_auth_factors = user_auth_factors.configured_user_auth_factors
user_auth_factors_preferred_mfa_setting = user_auth_factors.preferred_mfa_setting
```

---


### Resource_server

ResourceServer resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `identifier` | String | ✅ | <p>A unique resource server identifier for the resource server. The identifier can be an
            API friendly name like <code>solar-system-data</code>. You can also set an API URL like
                <code>https://solar-system-data-api.example.com</code> as your identifier.</p>
         <p>Amazon Cognito represents scopes in the access token in the format
                <code>$resource-server-identifier/$scope</code>. Longer scope-identifier strings
            increase the size of your access tokens.</p> |
| `user_pool_id` | String | ✅ | <p>The ID of the user pool where you want to create a resource server.</p> |
| `scopes` | Vec<String> |  | <p>A list of custom scopes. Each scope is a key-value map with the keys
                <code>ScopeName</code> and <code>ScopeDescription</code>. The name of a custom scope
            is a combination of <code>ScopeName</code> and the resource server <code>Name</code> in
            this request, for example <code>MyResourceServerName/MyScopeName</code>.</p> |
| `name` | String | ✅ | <p>A friendly name for the resource server.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_server` | String | <p>The details of the requested resource server.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_server
resource_server = provider.cognito_identity_provider.Resource_server {
    identifier = "value"  # <p>A unique resource server identifier for the resource server. The identifier can be an
            API friendly name like <code>solar-system-data</code>. You can also set an API URL like
                <code>https://solar-system-data-api.example.com</code> as your identifier.</p>
         <p>Amazon Cognito represents scopes in the access token in the format
                <code>$resource-server-identifier/$scope</code>. Longer scope-identifier strings
            increase the size of your access tokens.</p>
    user_pool_id = "value"  # <p>The ID of the user pool where you want to create a resource server.</p>
    name = "value"  # <p>A friendly name for the resource server.</p>
}

# Access resource_server outputs
resource_server_id = resource_server.id
resource_server_resource_server = resource_server.resource_server
```

---


### Auth_event_feedback

AuthEventFeedback resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_pool_id` | String | ✅ | <p>The ID of the user pool where you want to update auth event feedback.</p> |
| `event_id` | String | ✅ | <p>The ID of the authentication event that you want to submit feedback for.</p> |
| `username` | String | ✅ | <p>The name of the user that you want to query or modify. The value of this parameter
            is typically your user's username, but it can be any of their alias attributes. If
                <code>username</code> isn't an alias attribute in your user pool, this value
            must be the <code>sub</code> of a local user or the username of a user from a
            third-party IdP.</p> |
| `feedback_token` | String | ✅ | <p>The feedback token, an encrypted object generated by Amazon Cognito and passed to your user in
            the notification email message from the event.</p> |
| `feedback_value` | String | ✅ | <p>Your feedback to the authentication event. When you provide a <code>FeedbackValue</code>
value of <code>valid</code>, you tell Amazon Cognito that you trust a user session where Amazon Cognito
has evaluated some level of risk. When you provide a <code>FeedbackValue</code> value of
<code>invalid</code>, you tell Amazon Cognito that you don't trust a user session, or you 
don't believe that Amazon Cognito evaluated a high-enough risk level.</p> |



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


### Identity_provider_by_identifier

IdentityProviderByIdentifier resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `identity_provider` | String | <p>The configuration of the IdP in your user pool. Includes additional identifiers, the
            IdP name and type, and trust-relationship details like the issuer URL.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access identity_provider_by_identifier outputs
identity_provider_by_identifier_id = identity_provider_by_identifier.id
identity_provider_by_identifier_identity_provider = identity_provider_by_identifier.identity_provider
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
| `device` | String | <p>Details of the requested device. Includes device information, last-accessed and
            created dates, and the device key.</p> |


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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple user_import_job resources
user_import_job_0 = provider.cognito_identity_provider.User_import_job {
    cloud_watch_logs_role_arn = "value-0"
    user_pool_id = "value-0"
    job_name = "value-0"
}
user_import_job_1 = provider.cognito_identity_provider.User_import_job {
    cloud_watch_logs_role_arn = "value-1"
    user_pool_id = "value-1"
    job_name = "value-1"
}
user_import_job_2 = provider.cognito_identity_provider.User_import_job {
    cloud_watch_logs_role_arn = "value-2"
    user_pool_id = "value-2"
    job_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    user_import_job = provider.cognito_identity_provider.User_import_job {
        cloud_watch_logs_role_arn = "production-value"
        user_pool_id = "production-value"
        job_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Cognito_identity_provider Documentation](https://docs.aws.amazon.com/cognito_identity_provider/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
