# Getting Started

This guide will help you get started using the Aws provider for Hemmer.

---

## Prerequisites

- Hemmer CLI installed
- Aws provider installed ([Installation Guide](installation.md))
- AWS credentials configured (AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY)

---

## Basic Usage

### 1. Initialize Provider

Create a new Rust project and add the provider dependency:

```bash
cargo new my-aws-app
cd my-aws-app
```

Add to `Cargo.toml`:

```toml
[dependencies]
hemmer-aws-provider = "*"
hemmer-core = "*"
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
```

### 2. Basic Example

```kcl
# main.k
import aws

# Create provider instance
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---

## Common Patterns

### Conditional Resource Creation

```kcl
# Only create resource if condition is met
if environment == "production":
    resource = provider.service.Resource {
        # configuration
    }
```

### Referencing Resource Outputs

```kcl
# Create a resource
bucket = provider.s3.Bucket {
    # configuration
}

# Reference its outputs
output_value = bucket.id
```

---

## Available Services

This provider includes 394 services:

### 1. Emr_serverless

**Resources**: 0


📖 [Full emr_serverless documentation](services/emr_serverless.md)

### 2. Cloudformation

**Resources**: 25

- Change_set [CRD]
- Account_limits [R]
- Stack_resource [R]
- Stack_resources [R]
- Stacks [R]
- Stack_set [CRUD]
- Organizations_access [R]
- Stack_set_operation [R]
- Change_set_hooks [R]
- Type_registration [R]
- Stack_refactor [CR]
- Stack_drift_detection_status [R]
- Generated_template [CRUD]
- Type [R]
- Stack_resource_drifts [R]
- Stack_policy [R]
- Template_summary [R]
- Stack_instances [CUD]
- Termination_protection [U]
- Publisher [R]
- Template [R]
- Stack_events [R]
- Resource_scan [R]
- Stack [CUD]
- Stack_instance [R]

📖 [Full cloudformation documentation](services/cloudformation.md)

### 3. Application_auto_scaling

**Resources**: 7

- Scaling_policy [CD]
- Scalable_targets [R]
- Scaling_activities [R]
- Scheduled_action [CD]
- Predictive_scaling_forecast [R]
- Scaling_policies [R]
- Scheduled_actions [R]

📖 [Full application_auto_scaling documentation](services/application_auto_scaling.md)

### 4. Personalize_events

**Resources**: 5

- Actions [C]
- Items [C]
- Action_interactions [C]
- Users [C]
- Events [C]

📖 [Full personalize_events documentation](services/personalize_events.md)

### 5. Tnb

**Resources**: 9

- Sol_function_instance [R]
- Sol_network_package_descriptor [R]
- Sol_network_operation [R]
- Sol_function_package [CRUD]
- Sol_function_package_content [CR]
- Sol_network_instance [CRUD]
- Sol_network_package_content [CR]
- Sol_function_package_descriptor [R]
- Sol_network_package [CRUD]

📖 [Full tnb documentation](services/tnb.md)

### 6. Rolesanywhere

**Resources**: 1

- Notification_settings [C]

📖 [Full rolesanywhere documentation](services/rolesanywhere.md)

### 7. Kms

**Resources**: 12

- Key_policy [CR]
- Alias [CUD]
- Key [CR]
- Custom_key_store [CUD]
- Imported_key_material [D]
- Key_description [U]
- Primary_region [U]
- Grant [C]
- Custom_key_stores [R]
- Key_rotation_status [R]
- Parameters_for_import [R]
- Public_key [R]

📖 [Full kms documentation](services/kms.md)

### 8. Datasync

**Resources**: 15

- Location_fsx_windows [CRU]
- Location_azure_blob [CRU]
- Location_fsx_ontap [CRU]
- Location_hdfs [CRU]
- Location_s3 [CRU]
- Location_smb [CRU]
- Task_execution [RU]
- Location_efs [CRU]
- Location_fsx_lustre [CRU]
- Agent [CRUD]
- Task [CRUD]
- Location [D]
- Location_nfs [CRU]
- Location_object_storage [CRU]
- Location_fsx_open_zfs [CRU]

📖 [Full datasync documentation](services/datasync.md)

### 9. Bedrock_agent_runtime

**Resources**: 0


📖 [Full bedrock_agent_runtime documentation](services/bedrock_agent_runtime.md)

### 10. Pinpoint_email

**Resources**: 25

- Configuration_set_reputation_options [C]
- Email_identity_feedback_attributes [C]
- Dedicated_ip_pool [CD]
- Account [R]
- Email_identity_dkim_attributes [C]
- Domain_deliverability_campaign [R]
- Configuration_set_event_destinations [R]
- Configuration_set [CRD]
- Account_sending_attributes [C]
- Configuration_set_event_destination [CUD]
- Dedicated_ip [R]
- Domain_statistics_report [R]
- Deliverability_test_report [CR]
- Dedicated_ip_in_pool [C]
- Email_identity_mail_from_attributes [C]
- Configuration_set_tracking_options [C]
- Deliverability_dashboard_options [R]
- Configuration_set_delivery_options [C]
- Dedicated_ips [R]
- Configuration_set_sending_options [C]
- Account_dedicated_ip_warmup_attributes [C]
- Email_identity [CRD]
- Dedicated_ip_warmup_attributes [C]
- Deliverability_dashboard_option [C]
- Blacklist_reports [R]

📖 [Full pinpoint_email documentation](services/pinpoint_email.md)

### 11. Connect_contact_lens

**Resources**: 0


📖 [Full connect_contact_lens documentation](services/connect_contact_lens.md)

### 12. Athena

**Resources**: 19

- Data_catalog [CRUD]
- Notebook [CUD]
- Query_runtime_statistics [R]
- Session_status [R]
- Calculation_execution [R]
- Presigned_notebook_url [C]
- Notebook_metadata [RU]
- Session [R]
- Database [R]
- Calculation_execution_code [R]
- Query_execution [R]
- Named_query [CRUD]
- Table_metadata [R]
- Calculation_execution_status [R]
- Prepared_statement [CRUD]
- Work_group [CRUD]
- Capacity_reservation [CRUD]
- Query_results [R]
- Capacity_assignment_configuration [CR]

📖 [Full athena documentation](services/athena.md)

### 13. Iotfleetwise

**Resources**: 4

- Logging_options [CR]
- Register_account_status [R]
- Encryption_configuration [CR]
- Vehicle_status [R]

📖 [Full iotfleetwise documentation](services/iotfleetwise.md)

### 14. Iot_data_plane

**Resources**: 3

- Connection [D]
- Thing_shadow [RUD]
- Retained_message [R]

📖 [Full iot_data_plane documentation](services/iot_data_plane.md)

### 15. Bedrock_data_automation_runtime

**Resources**: 0


📖 [Full bedrock_data_automation_runtime documentation](services/bedrock_data_automation_runtime.md)

### 16. Entityresolution

**Resources**: 10

- Provider_service [R]
- Matching_workflow [CRUD]
- Id_mapping_job [R]
- Id_mapping_workflow [CRUD]
- Schema_mapping [CRUD]
- Policy_statement [D]
- Id_namespace [CRUD]
- Match_id [R]
- Matching_job [R]
- Policy [CR]

📖 [Full entityresolution documentation](services/entityresolution.md)

### 17. Forecastquery

**Resources**: 0


📖 [Full forecastquery documentation](services/forecastquery.md)

### 18. Detective

**Resources**: 6

- Graph [CD]
- Members [CRD]
- Organization_configuration [RU]
- Investigation [R]
- Datasource_packages [U]
- Investigation_state [U]

📖 [Full detective documentation](services/detective.md)

### 19. Panorama

**Resources**: 11

- Device_job [R]
- Application_instance_details [R]
- Application_instance [CR]
- Device_metadata [U]
- Package_version [R]
- Package_import_job [CR]
- Package [CRD]
- Device [RD]
- Node_from_template_job [CR]
- Job_for_devices [C]
- Node [R]

📖 [Full panorama documentation](services/panorama.md)

### 20. Backup

**Resources**: 31

- Backup_plan_from_json [R]
- Restore_access_backup_vault [C]
- Recovery_point_index_settings [U]
- Recovery_point_lifecycle [U]
- Restore_job [R]
- Backup_vault_access_policy [CRD]
- Region_settings [RU]
- Report_job [R]
- Backup_plan [CRUD]
- Recovery_point_index_details [R]
- Backup_plan_from_template [R]
- Framework [CRUD]
- Global_settings [RU]
- Restore_testing_selection [CRUD]
- Legal_hold [CR]
- Logically_air_gapped_backup_vault [C]
- Backup_vault_lock_configuration [CD]
- Backup_selection [CRD]
- Recovery_point [RD]
- Restore_testing_inferred_metadata [R]
- Backup_vault [CRD]
- Backup_job [R]
- Restore_testing_plan [CRUD]
- Supported_resource_types [R]
- Backup_vault_notifications [CRD]
- Restore_job_metadata [R]
- Recovery_point_restore_metadata [R]
- Report_plan [CRUD]
- Copy_job [R]
- Protected_resource [R]
- Restore_validation_result [C]

📖 [Full backup documentation](services/backup.md)

### 21. Mwaa

**Resources**: 3

- Web_login_token [C]
- Environment [CRUD]
- Cli_token [C]

📖 [Full mwaa documentation](services/mwaa.md)

### 22. Iot_jobs_data_plane

**Resources**: 2

- Pending_job_executions [R]
- Job_execution [RU]

📖 [Full iot_jobs_data_plane documentation](services/iot_jobs_data_plane.md)

### 23. Transcribe

**Resources**: 9

- Call_analytics_job [RD]
- Language_model [CRD]
- Call_analytics_category [CRUD]
- Vocabulary_filter [CRUD]
- Medical_vocabulary [CRUD]
- Medical_transcription_job [RD]
- Transcription_job [RD]
- Medical_scribe_job [RD]
- Vocabulary [CRUD]

📖 [Full transcribe documentation](services/transcribe.md)

### 24. Cloudwatch

**Resources**: 18

- Insight_rule [C]
- Anomaly_detector [CD]
- Metric_stream [CRD]
- Alarm_history [R]
- Alarms [RD]
- Metric_data [CR]
- Composite_alarm [C]
- Metric_statistics [R]
- Insight_rules [RD]
- Alarm_contributors [R]
- Managed_insight_rules [C]
- Dashboard [CR]
- Alarms_for_metric [R]
- Insight_rule_report [R]
- Anomaly_detectors [R]
- Metric_alarm [C]
- Metric_widget_image [R]
- Dashboards [D]

📖 [Full cloudwatch documentation](services/cloudwatch.md)

### 25. Snow_device_management

**Resources**: 0


📖 [Full snow_device_management documentation](services/snow_device_management.md)

### 26. Workmailmessageflow

**Resources**: 1

- Raw_message_content [CR]

📖 [Full workmailmessageflow documentation](services/workmailmessageflow.md)

### 27. Appconfig

**Resources**: 10

- Hosted_configuration_version [CRD]
- Deployment [R]
- Application [CRUD]
- Extension [CRUD]
- Configuration [R]
- Environment [CRUD]
- Deployment_strategy [CRUD]
- Extension_association [CRUD]
- Configuration_profile [CRUD]
- Account_settings [RU]

📖 [Full appconfig documentation](services/appconfig.md)

### 28. Lightsail

**Resources**: 89

- Instances_from_snapshot [C]
- Export_snapshot_records [R]
- Distribution_latest_cache_reset [R]
- Cloud_formation_stack_records [R]
- Relational_database_bundles [R]
- Container_service_powers [R]
- Relational_database_master_user_password [R]
- Static_ip [R]
- Alarms [R]
- Load_balancer_tls_policies [R]
- Key_pair [CRD]
- Operations [R]
- Distribution_bundles [R]
- Static_ips [R]
- Disk_from_snapshot [C]
- Contact_method [CD]
- Relational_database_snapshots [R]
- Load_balancers [R]
- Known_host_keys [D]
- Distribution [CUD]
- Container_service_deployment [C]
- Relational_database_snapshot [CRD]
- Auto_snapshot [D]
- Container_services [R]
- Active_names [R]
- Container_image [D]
- Instance_state [R]
- Relational_database_log_streams [R]
- Bundles [R]
- Relational_database_blueprints [R]
- Distribution_metric_data [R]
- Cloud_formation_stack [C]
- Bucket_metric_data [R]
- Bucket_access_key [CD]
- Instance_port_states [R]
- Certificate [CD]
- Container_service [CUD]
- Disks [R]
- Disk [CRD]
- Relational_database_from_snapshot [C]
- Container_service_deployments [R]
- Key_pairs [R]
- Gui_session_access_details [C]
- Instance_snapshot [CRD]
- Alarm [CD]
- Instance [RD]
- Bucket_access_keys [R]
- Container_log [R]
- Disk_snapshots [R]
- Load_balancer_metric_data [R]
- Setup_history [R]
- Domains [R]
- Container_api_metadata [R]
- Domain [CRD]
- Cost_estimate [R]
- Buckets [R]
- Blueprints [R]
- Instance_metric_data [R]
- Regions [R]
- Relational_database_events [R]
- Relational_databases [R]
- Load_balancer_tls_certificate [CD]
- Instance_access_details [R]
- Instance_public_ports [C]
- Distribution_bundle [U]
- Instance_metadata_options [U]
- Load_balancer_tls_certificates [R]
- Domain_entry [CUD]
- Instances [CR]
- Container_images [R]
- Load_balancer_attribute [U]
- Instance_snapshots [R]
- Relational_database_log_events [R]
- Disk_snapshot [CRD]
- Relational_database_metric_data [R]
- Distributions [R]
- Bucket_bundles [R]
- Container_service_metric_data [R]
- Operation [R]
- Bucket_bundle [U]
- Operations_for_resource [R]
- Load_balancer [CRD]
- Contact_methods [R]
- Bucket [CUD]
- Container_service_registry_login [C]
- Relational_database [CRUD]
- Auto_snapshots [R]
- Relational_database_parameters [RU]
- Certificates [R]

📖 [Full lightsail documentation](services/lightsail.md)

### 29. Guardduty

**Resources**: 25

- Usage_statistics [R]
- Findings_feedback [U]
- Invitations [D]
- Publishing_destination [CRUD]
- Filter [CRUD]
- Detector [CRUD]
- Trusted_entity_set [CRUD]
- Malware_scans [R]
- Invitations_count [R]
- Organization_configuration [RU]
- Ip_set [CRUD]
- Master_account [R]
- Findings_statistics [R]
- Members [CRD]
- Sample_findings [C]
- Organization_statistics [R]
- Threat_intel_set [CRUD]
- Coverage_statistics [R]
- Malware_scan_settings [RU]
- Threat_entity_set [CRUD]
- Administrator_account [R]
- Remaining_free_trial_days [R]
- Member_detectors [RU]
- Findings [R]
- Malware_protection_plan [CRUD]

📖 [Full guardduty documentation](services/guardduty.md)

### 30. Apigatewayv2

**Resources**: 31

- Vpc_links [R]
- Stage [CRUD]
- Routing_rule [CRD]
- Apis [R]
- Model_template [R]
- Deployments [R]
- Tags [R]
- Routes [R]
- Access_log_settings [D]
- Api [CRUD]
- Integration_response [CRUD]
- Route_request_parameter [D]
- Api_mapping [CRUD]
- Integration_responses [R]
- Model [CRUD]
- Route [CRUD]
- Cors_configuration [D]
- Stages [R]
- Models [R]
- Route_response [CRUD]
- Domain_names [R]
- Authorizers [R]
- Authorizer [CRUD]
- Deployment [CRUD]
- Vpc_link [CRUD]
- Route_settings [D]
- Route_responses [R]
- Domain_name [CRUD]
- Integration [CRUD]
- Api_mappings [R]
- Integrations [R]

📖 [Full apigatewayv2 documentation](services/apigatewayv2.md)

### 31. Wafv2

**Resources**: 19

- Firewall_manager_rule_groups [D]
- Permission_policy [CRD]
- Managed_rule_group [R]
- Logging_configuration [CRD]
- Mobile_sdk_release [R]
- Decrypted_api_key [R]
- Api_key [CD]
- Sampled_requests [R]
- All_managed_products [R]
- Regex_pattern_set [CRUD]
- Rate_based_statement_managed_keys [R]
- Ip_set [CRUD]
- Managed_products_by_vendor [R]
- Web_acl_for_resource [R]
- Rule_group [CRUD]
- Managed_rule_set_version_expiry_date [U]
- Managed_rule_set [R]
- Managed_rule_set_versions [C]
- Web_acl [CRUD]

📖 [Full wafv2 documentation](services/wafv2.md)

### 32. Iotsitewise

**Resources**: 26

- Storage_configuration [CR]
- Dashboard [CRUD]
- Asset_composite_model [R]
- Action [R]
- Access_policy [CRUD]
- Gateway [CRUD]
- Default_encryption_configuration [CR]
- Project [CRUD]
- Asset_property [RU]
- Execution [R]
- Asset_property_value [R]
- Gateway_capability_configuration [RU]
- Asset [CRUD]
- Asset_property_value_history [R]
- Asset_model_composite_model [CRUD]
- Time_series [RD]
- Portal [CRUD]
- Logging_options [CR]
- Computation_model_execution_summary [R]
- Asset_model_interface_relationship [CRD]
- Interpolated_asset_property_values [R]
- Dataset [CRUD]
- Asset_property_aggregates [R]
- Bulk_import_job [CR]
- Asset_model [CRUD]
- Computation_model [CRUD]

📖 [Full iotsitewise documentation](services/iotsitewise.md)

### 33. Iotthingsgraph

**Resources**: 9

- System_template [CRUD]
- Namespace [RD]
- System_instance [CRD]
- Flow_template_revisions [R]
- Namespace_deletion_status [R]
- Flow_template [CRUD]
- Entities [R]
- Upload_status [R]
- System_template_revisions [R]

📖 [Full iotthingsgraph documentation](services/iotthingsgraph.md)

### 34. Batch

**Resources**: 13

- Jobs [R]
- Job_queue [CUD]
- Compute_environment [CUD]
- Job_queues [R]
- Service_environments [R]
- Job_queue_snapshot [R]
- Job_definitions [R]
- Scheduling_policy [CUD]
- Service_environment [CUD]
- Service_job [R]
- Compute_environments [R]
- Scheduling_policies [R]
- Consumable_resource [CRUD]

📖 [Full batch documentation](services/batch.md)

### 35. Mailmanager

**Resources**: 7

- Archive_message_content [R]
- Address_list_import_job [CR]
- Archive_search_results [R]
- Archive_export [R]
- Archive_message [R]
- Member_of_address_list [R]
- Archive_search [R]

📖 [Full mailmanager documentation](services/mailmanager.md)

### 36. Marketplace_reporting

**Resources**: 0


📖 [Full marketplace_reporting documentation](services/marketplace_reporting.md)

### 37. M2

**Resources**: 1

- Signed_bluinsights_url [R]

📖 [Full m2 documentation](services/m2.md)

### 38. Codedeploy

**Resources**: 11

- Application [CRUD]
- Deployment_config [CRD]
- Deployment_instance [R]
- Deployment_group [CRUD]
- Git_hub_account_token [D]
- Lifecycle_event_hook_execution_status [C]
- Resources_by_external_id [D]
- Deployment [CR]
- On_premises_instance [R]
- Deployment_target [R]
- Application_revision [R]

📖 [Full codedeploy documentation](services/codedeploy.md)

### 39. Route53_recovery_control_config

**Resources**: 5

- Cluster [CRUD]
- Control_panel [CRUD]
- Resource_policy [R]
- Routing_control [CRUD]
- Safety_rule [CRUD]

📖 [Full route53_recovery_control_config documentation](services/route53_recovery_control_config.md)

### 40. Simspaceweaver

**Resources**: 0


📖 [Full simspaceweaver documentation](services/simspaceweaver.md)

### 41. Resiliencehub

**Resources**: 14

- App_version_template [R]
- Draft_app_version_template [C]
- App_version_resources_resolution_status [R]
- Resource_grouping_recommendation_task [R]
- App_assessment [RD]
- App_version_resource [CRUD]
- Resiliency_policy [CRUD]
- App_version_app_component [CRUD]
- App [CRUD]
- App_input_source [D]
- Metrics_export [R]
- App_version [RU]
- Recommendation_template [CD]
- Draft_app_version_resources_import_status [R]

📖 [Full resiliencehub documentation](services/resiliencehub.md)

### 42. Oam

**Resources**: 3

- Sink_policy [CR]
- Link [CRUD]
- Sink [CRD]

📖 [Full oam documentation](services/oam.md)

### 43. License_manager_linux_subscriptions

**Resources**: 2

- Service_settings [RU]
- Registered_subscription_provider [R]

📖 [Full license_manager_linux_subscriptions documentation](services/license_manager_linux_subscriptions.md)

### 44. Voice_id

**Resources**: 5

- Fraudster_registration_job [R]
- Fraudster [RD]
- Watchlist [CRUD]
- Speaker_enrollment_job [R]
- Speaker [RD]

📖 [Full voice_id documentation](services/voice_id.md)

### 45. Chime

**Resources**: 14

- Retention_settings [CR]
- User_settings [RU]
- Phone_number_settings [RU]
- Phone_number_order [CR]
- Account_settings [RU]
- Room [CRUD]
- Room_membership [CUD]
- Events_configuration [CRD]
- Phone_number [RUD]
- Account [CRUD]
- Global_settings [RU]
- Meeting_dial_out [C]
- User [CRU]
- Bot [CRU]

📖 [Full chime documentation](services/chime.md)

### 46. Efs

**Resources**: 15

- Account_preferences [CR]
- Backup_policy [CR]
- File_system [CUD]
- File_systems [R]
- Access_point [CD]
- Access_points [R]
- Lifecycle_configuration [CR]
- Replication_configuration [CD]
- Mount_target_security_groups [R]
- Replication_configurations [R]
- Tags [CRD]
- Mount_target [CD]
- File_system_policy [CRD]
- Mount_targets [R]
- File_system_protection [U]

📖 [Full efs documentation](services/efs.md)

### 47. Freetier

**Resources**: 3

- Free_tier_usage [R]
- Account_activity [R]
- Account_plan_state [R]

📖 [Full freetier documentation](services/freetier.md)

### 48. Storage_gateway

**Resources**: 41

- Nfs_file_share [CU]
- Cache_report [RD]
- Snapshot_from_volume_recovery_point [C]
- Tape_with_barcode [C]
- Working_storage [R]
- Nfs_file_shares [R]
- Smb_local_groups [U]
- Cachedi_scsi_volumes [R]
- Vtl_devices [R]
- Availability_monitor_test [R]
- Tape_archive [D]
- Bandwidth_rate_limit_schedule [RU]
- Tapes [CR]
- Maintenance_start_time [RU]
- File_system_association [U]
- Tape_archives [R]
- Tape [D]
- Tape_recovery_points [R]
- Automatic_tape_creation_policy [UD]
- Smb_file_share [CU]
- Smb_settings [R]
- Upload_buffer [R]
- Storedi_scsi_volumes [R]
- Storedi_scsi_volume [C]
- Chap_credentials [RUD]
- Gateway_information [RU]
- File_system_associations [R]
- Cachedi_scsi_volume [C]
- Cache [R]
- File_share [D]
- Smb_file_share_visibility [U]
- Volume [D]
- Smb_security_strategy [U]
- Vtl_device_type [U]
- Tape_pool [CD]
- Smb_file_shares [R]
- Gateway_software_now [U]
- Gateway [D]
- Snapshot [C]
- Bandwidth_rate_limit [RUD]
- Snapshot_schedule [RUD]

📖 [Full storage_gateway documentation](services/storage_gateway.md)

### 49. Dynamodb_streams

**Resources**: 3

- Records [R]
- Stream [R]
- Shard_iterator [R]

📖 [Full dynamodb_streams documentation](services/dynamodb_streams.md)

### 50. Gamelift

**Resources**: 47

- Game_session_details [R]
- Compute_auth_token [R]
- Game_server_group [CRUD]
- Player_sessions [CR]
- Vpc_peering_connections [R]
- Instances [R]
- Container_group_definition [CRUD]
- Vpc_peering_authorization [CD]
- Matchmaking_rule_set [CD]
- Player_session [C]
- Game_session_queue [CUD]
- Vpc_peering_connection [CD]
- Build [CRUD]
- Fleet_attributes [RU]
- Game_sessions [R]
- Fleet_deployment [R]
- Fleet_capacity [RU]
- Game_server_instances [R]
- Matchmaking [R]
- Game_server [RU]
- Matchmaking_configurations [R]
- Instance_access [R]
- Container_fleet [CRUD]
- Fleet_location_utilization [R]
- Matchmaking_configuration [CUD]
- Fleet_utilization [R]
- Fleet [CD]
- Fleet_locations [CD]
- Game_session [CU]
- Fleet_events [R]
- Fleet_port_settings [RU]
- Script [CRUD]
- Game_session_placement [R]
- Location [CD]
- Runtime_configuration [RU]
- Game_session_log_url [R]
- Scaling_policy [CD]
- Scaling_policies [R]
- Vpc_peering_authorizations [R]
- Compute [R]
- Ec2_instance_limits [R]
- Fleet_location_capacity [R]
- Fleet_location_attributes [R]
- Compute_access [R]
- Game_session_queues [R]
- Matchmaking_rule_sets [R]
- Alias [CRUD]

📖 [Full gamelift documentation](services/gamelift.md)

### 51. Inspector2

**Resources**: 18

- Member [R]
- Filter [CUD]
- Findings_report [C]
- Cis_scan_report [R]
- Delegated_admin_account [R]
- Code_security_scan [R]
- Findings_report_status [R]
- Org_ec2_deep_inspection_configuration [U]
- Cis_scan_configuration [CUD]
- Sbom_export [CR]
- Encryption_key [RU]
- Code_security_integration [CRUD]
- Organization_configuration [RU]
- Cis_scan_result_details [R]
- Clusters_for_image [R]
- Configuration [RU]
- Ec2_deep_inspection_configuration [RU]
- Code_security_scan_configuration [CRUD]

📖 [Full inspector2 documentation](services/inspector2.md)

### 52. Keyspaces

**Resources**: 4

- Keyspace [CRUD]
- Table_auto_scaling_settings [R]
- Table [CRUD]
- Type [CRD]

📖 [Full keyspaces documentation](services/keyspaces.md)

### 53. Sqs

**Resources**: 5

- Queue_url [R]
- Message [D]
- Queue_attributes [R]
- Queue [CD]
- Message_batch [D]

📖 [Full sqs documentation](services/sqs.md)

### 54. Ram

**Resources**: 7

- Permission_version [CD]
- Resource_policies [R]
- Resource_share_invitations [R]
- Permission [CRD]
- Resource_shares [R]
- Resource_share [CUD]
- Resource_share_associations [R]

📖 [Full ram documentation](services/ram.md)

### 55. Ssm_sap

**Resources**: 7

- Application_settings [U]
- Application [R]
- Configuration_check_operation [R]
- Operation [R]
- Database [R]
- Resource_permission [CRD]
- Component [R]

📖 [Full ssm_sap documentation](services/ssm_sap.md)

### 56. Directory_service_data

**Resources**: 2

- User [CRUD]
- Group [CRUD]

📖 [Full directory_service_data documentation](services/directory_service_data.md)

### 57. Route_53_domains

**Resources**: 9

- Contact_reachability_status [R]
- Domain [D]
- Tags_for_domain [UD]
- Domain_suggestions [R]
- Domain_detail [R]
- Domain_contact [U]
- Operation_detail [R]
- Domain_contact_privacy [U]
- Domain_nameservers [U]

📖 [Full route_53_domains documentation](services/route_53_domains.md)

### 58. Bedrock_agentcore

**Resources**: 5

- Workload_access_token_for_user_id [R]
- Resource_api_key [R]
- Workload_access_token [R]
- Resource_oauth2_token [R]
- Workload_access_token_for_jwt [R]

📖 [Full bedrock_agentcore documentation](services/bedrock_agentcore.md)

### 59. Trustedadvisor

**Resources**: 4

- Organization_recommendation_lifecycle [U]
- Organization_recommendation [R]
- Recommendation [R]
- Recommendation_lifecycle [U]

📖 [Full trustedadvisor documentation](services/trustedadvisor.md)

### 60. Migrationhubstrategy

**Resources**: 12

- Recommendation_report_details [R]
- Application_component_details [R]
- Server_strategies [R]
- Portfolio_preferences [CR]
- Import_file_task [R]
- Portfolio_summary [R]
- Latest_assessment_id [R]
- Application_component_config [U]
- Assessment [R]
- Server_config [U]
- Server_details [R]
- Application_component_strategies [R]

📖 [Full migrationhubstrategy documentation](services/migrationhubstrategy.md)

### 61. Dataexchange

**Resources**: 7

- Data_set [CRUD]
- Revision [CRUD]
- Event_action [CRUD]
- Asset [RUD]
- Job [CR]
- Data_grant [CRD]
- Received_data_grant [R]

📖 [Full dataexchange documentation](services/dataexchange.md)

### 62. Braket

**Resources**: 0


📖 [Full braket documentation](services/braket.md)

### 63. Codebuild

**Resources**: 12

- Source_credentials [D]
- Fleet [CUD]
- Project_visibility [U]
- Report_group [CUD]
- Webhook [CUD]
- Resource_policy [CRD]
- Report [D]
- Report_group_trend [R]
- Code_coverages [R]
- Test_cases [R]
- Project [CUD]
- Build_batch [D]

📖 [Full codebuild documentation](services/codebuild.md)

### 64. Acm

**Resources**: 3

- Certificate_options [U]
- Certificate [RD]
- Account_configuration [CR]

📖 [Full acm documentation](services/acm.md)

### 65. Route53_recovery_cluster

**Resources**: 2

- Routing_control_state [RU]
- Routing_control_states [U]

📖 [Full route53_recovery_cluster documentation](services/route53_recovery_cluster.md)

### 66. Lookoutequipment

**Resources**: 10

- Active_model_version [U]
- Model [CRUD]
- Data_ingestion_job [R]
- Inference_scheduler [CRUD]
- Model_version [R]
- Label [CRD]
- Resource_policy [CRD]
- Label_group [CRUD]
- Retraining_scheduler [CRUD]
- Dataset [CRD]

📖 [Full lookoutequipment documentation](services/lookoutequipment.md)

### 67. Marketplace_catalog

**Resources**: 3

- Entity [R]
- Resource_policy [CRD]
- Change_set [R]

📖 [Full marketplace_catalog documentation](services/marketplace_catalog.md)

### 68. Payment_cryptography_data

**Resources**: 0


📖 [Full payment_cryptography_data documentation](services/payment_cryptography_data.md)

### 69. Cloud9

**Resources**: 6

- Environments [R]
- Environment_status [R]
- Environment [UD]
- Environment_memberships [R]
- Environment_ec2 [C]
- Environment_membership [CUD]

📖 [Full cloud9 documentation](services/cloud9.md)

### 70. Workdocs

**Resources**: 21

- Current_user [R]
- Resources [R]
- Document_version [RUD]
- Groups [R]
- Labels [CD]
- Document_versions [R]
- Document [RUD]
- Users [R]
- User [CUD]
- Comments [R]
- Activities [R]
- Folder [CRUD]
- Comment [CD]
- Notification_subscription [CD]
- Folder_contents [RD]
- Notification_subscriptions [R]
- Resource_permissions [R]
- Document_path [R]
- Folder_path [R]
- Custom_metadata [CD]
- Root_folders [R]

📖 [Full workdocs documentation](services/workdocs.md)

### 71. License_manager

**Resources**: 13

- License_configuration [CRUD]
- License_version [C]
- Token [CD]
- License_conversion_task [R]
- License [CRD]
- License_conversion_task_for_resource [C]
- License_manager_report_generator [CRUD]
- Grant_version [C]
- Grant [CRD]
- License_usage [R]
- License_specifications_for_resource [U]
- Access_token [R]
- Service_settings [RU]

📖 [Full license_manager documentation](services/license_manager.md)

### 72. Sts

**Resources**: 4

- Caller_identity [R]
- Session_token [R]
- Access_key_info [R]
- Federation_token [R]

📖 [Full sts documentation](services/sts.md)

### 73. S3vectors

**Resources**: 0


📖 [Full s3vectors documentation](services/s3vectors.md)

### 74. Chime_sdk_media_pipelines

**Resources**: 11

- Media_capture_pipeline [CRD]
- Media_insights_pipeline [C]
- Voice_tone_analysis_task [R]
- Media_pipeline [RD]
- Media_insights_pipeline_status [U]
- Media_pipeline_kinesis_video_stream_pool [CRUD]
- Media_insights_pipeline_configuration [CRUD]
- Media_live_connector_pipeline [C]
- Media_stream_pipeline [C]
- Speaker_search_task [R]
- Media_concatenation_pipeline [C]

📖 [Full chime_sdk_media_pipelines documentation](services/chime_sdk_media_pipelines.md)

### 75. Machine_learning

**Resources**: 13

- Data_source [RUD]
- Data_sources [R]
- Evaluations [R]
- Data_source_from_s3 [C]
- Batch_predictions [R]
- Evaluation [CRUD]
- Realtime_endpoint [CD]
- Ml_models [R]
- Data_source_from_redshift [C]
- Data_source_from_rds [C]
- Tags [RD]
- Ml_model [CRUD]
- Batch_prediction [CRUD]

📖 [Full machine_learning documentation](services/machine_learning.md)

### 76. Timestream_query

**Resources**: 3

- Scheduled_query [CRUD]
- Endpoints [R]
- Account_settings [RU]

📖 [Full timestream_query documentation](services/timestream_query.md)

### 77. Codeguru_reviewer

**Resources**: 3

- Code_review [CR]
- Recommendation_feedback [CR]
- Repository_association [R]

📖 [Full codeguru_reviewer documentation](services/codeguru_reviewer.md)

### 78. Mgn

**Resources**: 0


📖 [Full mgn documentation](services/mgn.md)

### 79. Evidently

**Resources**: 0


📖 [Full evidently documentation](services/evidently.md)

### 80. Qbusiness

**Resources**: 12

- Conversation [D]
- Group [CRD]
- Attachment [D]
- User [CRUD]
- Chat_controls_configuration [RUD]
- Subscription [CU]
- Media [R]
- Policy [R]
- Feedback [C]
- Chat_response_configuration [CRUD]
- Document_content [R]
- Anonymous_web_experience_url [C]

📖 [Full qbusiness documentation](services/qbusiness.md)

### 81. Connectcases

**Resources**: 0


📖 [Full connectcases documentation](services/connectcases.md)

### 82. Fsx

**Resources**: 22

- Storage_virtual_machines [R]
- Volumes [R]
- Volume [CUD]
- Data_repository_tasks [R]
- File_system [CUD]
- Data_repository_association [CUD]
- Storage_virtual_machine [CUD]
- Volume_from_backup [C]
- S3_access_point_attachments [R]
- Snapshots [R]
- File_cache [CUD]
- Backup [CD]
- Data_repository_associations [R]
- File_systems [R]
- Backups [R]
- File_system_aliases [R]
- File_caches [R]
- Snapshot [CUD]
- Data_repository_task [C]
- And_attach_s3_access_point [C]
- File_system_from_backup [C]
- Shared_vpc_configuration [RU]

📖 [Full fsx documentation](services/fsx.md)

### 83. Ecr

**Resources**: 22

- Pull_through_cache_rule [CUD]
- Lifecycle_policy [CRD]
- Repository_creation_template [CUD]
- Images [R]
- Download_url_for_layer [R]
- Lifecycle_policy_preview [R]
- Registry_scanning_configuration [CR]
- Replication_configuration [C]
- Account_setting [CR]
- Registry [R]
- Image [C]
- Authorization_token [R]
- Image_scanning_configuration [C]
- Image_tag_mutability [C]
- Pull_through_cache_rules [R]
- Repositories [R]
- Image_replication_status [R]
- Registry_policy [CRD]
- Repository_policy [RD]
- Repository [CD]
- Image_scan_findings [R]
- Repository_creation_templates [R]

📖 [Full ecr documentation](services/ecr.md)

### 84. Connectcampaignsv2

**Resources**: 17

- Instance_onboarding_job [D]
- Campaign_state [R]
- Instance_onboarding_job_status [R]
- Campaign_state_batch [R]
- Campaign_communication_limits [UD]
- Campaign_schedule [U]
- Instance_communication_limits [CR]
- Campaign_communication_time [UD]
- Campaign [CRD]
- Campaign_source [U]
- Outbound_request_batch [C]
- Profile_outbound_request_batch [C]
- Connect_instance_config [RD]
- Campaign_flow_association [U]
- Campaign_name [U]
- Campaign_channel_subtype_config [UD]
- Connect_instance_integration [CD]

📖 [Full connectcampaignsv2 documentation](services/connectcampaignsv2.md)

### 85. Rds

**Resources**: 68

- Db_subnet_groups [R]
- Db_shard_groups [R]
- Db_instance_read_replica [C]
- Db_proxies [R]
- Db_proxy_endpoints [R]
- Blue_green_deployment [CD]
- Event_categories [R]
- Integration [CD]
- Engine_default_cluster_parameters [R]
- Db_cluster_snapshots [R]
- Orderable_db_instance_options [R]
- Blue_green_deployments [R]
- Db_cluster_backtracks [R]
- Db_cluster_parameter_groups [R]
- Custom_db_engine_version [CD]
- Db_subnet_group [CD]
- Db_proxy_target_groups [R]
- Tenant_database [CD]
- Db_shard_group [CD]
- Db_snapshots [R]
- Events [R]
- Reserved_db_instances_offerings [R]
- Engine_default_parameters [R]
- Export_tasks [R]
- Db_cluster_endpoint [CD]
- Db_security_groups [R]
- Db_security_group [CD]
- Db_snapshot [CD]
- Account_attributes [R]
- Source_regions [R]
- Certificates [R]
- Db_cluster_snapshot_attributes [R]
- Db_instance_automated_backup [D]
- Db_cluster_endpoints [R]
- Db_engine_versions [R]
- Tenant_databases [R]
- Db_proxy_endpoint [CD]
- Db_instance_automated_backups [R]
- Db_cluster_automated_backups [R]
- Db_recommendations [R]
- Db_cluster_automated_backup [D]
- Db_cluster_snapshot [CD]
- Db_log_files [R]
- Db_proxy [CD]
- Event_subscription [CD]
- Db_cluster [CD]
- Pending_maintenance_actions [R]
- Global_cluster [CD]
- Db_cluster_parameters [R]
- Db_parameters [R]
- Option_group [CD]
- Event_subscriptions [R]
- Option_group_options [R]
- Db_snapshot_tenant_databases [R]
- Db_major_engine_versions [R]
- Db_instance [CD]
- Db_cluster_parameter_group [CD]
- Db_parameter_group [CD]
- Db_clusters [R]
- Db_proxy_targets [R]
- Db_parameter_groups [R]
- Integrations [R]
- Reserved_db_instances [R]
- Db_instances [R]
- Option_groups [R]
- Valid_db_instance_modifications [R]
- Global_clusters [R]
- Db_snapshot_attributes [R]

📖 [Full rds documentation](services/rds.md)

### 86. Qapps

**Resources**: 7

- Library_item_metadata [U]
- Q_app_session_metadata [RU]
- Library_item [CRUD]
- Q_app_permissions [RU]
- Q_app [CRUD]
- Q_app_session [RU]
- Presigned_url [C]

📖 [Full qapps documentation](services/qapps.md)

### 87. Qconnect

**Resources**: 0


📖 [Full qconnect documentation](services/qconnect.md)

### 88. Omics

**Resources**: 1

- S3_access_policy [CRD]

📖 [Full omics documentation](services/omics.md)

### 89. Bcm_dashboards

**Resources**: 2

- Dashboard [CRUD]
- Resource_policy [R]

📖 [Full bcm_dashboards documentation](services/bcm_dashboards.md)

### 90. Geo_routes

**Resources**: 0


📖 [Full geo_routes documentation](services/geo_routes.md)

### 91. Quicksight

**Resources**: 68

- Account_subscription [CRD]
- Dashboard_published_version [U]
- User [RUD]
- Brand_assignment [RUD]
- Quick_sight_q_search_configuration [RU]
- Action_connector_permissions [RU]
- Iam_policy_assignment [CRUD]
- Dashboard_definition [R]
- Application_with_token_exchange_grant [U]
- Account_customization [CRUD]
- Dashboard_permissions [RU]
- Analysis_permissions [RU]
- Topic_permissions [RU]
- Analysis [CRUD]
- Flow_permissions [RU]
- Folder [CRUD]
- Dashboard_snapshot_job_result [R]
- Ip_restriction [RU]
- Session_embed_url [R]
- Group_membership [CRD]
- Template_definition [R]
- Dashboard_links [U]
- Asset_bundle_export_job [R]
- Folder_resolved_permissions [R]
- Dashboard [CRUD]
- Role_membership [CD]
- Data_set_permissions [RU]
- Folder_permissions [RU]
- Topic_refresh_schedule [CRUD]
- Template_alias [CRUD]
- Theme [CRUD]
- Ingestion [CR]
- Spice_capacity_configuration [U]
- Action_connector [CRUD]
- Key_registration [RU]
- Custom_permissions [CRUD]
- Identity_propagation_config [UD]
- Q_personalization_configuration [RU]
- Public_sharing_settings [U]
- Refresh_schedule [CRUD]
- Default_q_business_application [RUD]
- Dashboard_snapshot_job [R]
- Folder_membership [CD]
- Template [CRUD]
- Asset_bundle_import_job [R]
- Brand [CRUD]
- User_by_principal_id [D]
- Vpc_connection [CRUD]
- Data_source [CRUD]
- Theme_alias [CRUD]
- Dashboards_qa_configuration [RU]
- Data_set_refresh_properties [CRD]
- Group [CRUD]
- Account_custom_permission [RUD]
- Role_custom_permission [RUD]
- Brand_published_version [RU]
- Template_permissions [RU]
- Dashboard_embed_url [R]
- Topic_refresh [R]
- Account_settings [RU]
- Data_source_permissions [RU]
- Theme_permissions [RU]
- Analysis_definition [R]
- Data_set [CRUD]
- Topic [CRUD]
- Namespace [CRD]
- Flow_metadata [R]
- User_custom_permission [UD]

📖 [Full quicksight documentation](services/quicksight.md)

### 92. Amp

**Resources**: 1

- Default_scraper_configuration [R]

📖 [Full amp documentation](services/amp.md)

### 93. Opensearchserverless

**Resources**: 5

- Lifecycle_policy [C]
- Vpc_endpoint [U]
- Policies_stats [R]
- Account_settings [RU]
- Security_policy [C]

📖 [Full opensearchserverless documentation](services/opensearchserverless.md)

### 94. Emr

**Resources**: 16

- Security_configuration [CRD]
- Auto_termination_policy [CR]
- Cluster [R]
- Persistent_app_ui [CR]
- Notebook_execution [R]
- Studio_session_mapping [CRUD]
- Persistent_app_ui_presigned_url [R]
- Managed_scaling_policy [CR]
- Step [R]
- On_cluster_app_ui_presigned_url [R]
- Studio [CRUD]
- Block_public_access_configuration [CR]
- Cluster_session_credentials [R]
- Auto_scaling_policy [C]
- Release_label [R]
- Job_flows [R]

📖 [Full emr documentation](services/emr.md)

### 95. Service_quotas

**Resources**: 9

- Service_quota [R]
- Support_case [C]
- Auto_management_configuration [R]
- Association_for_service_quota_template [R]
- Auto_management [U]
- Aws_default_service_quota [R]
- Service_quota_increase_request_into_template [C]
- Requested_service_quota_change [R]
- Service_quota_increase_request_from_template [RD]

📖 [Full service_quotas documentation](services/service_quotas.md)

### 96. Service_catalog_appregistry

**Resources**: 4

- Attribute_group [CRUD]
- Configuration [CR]
- Application [CRUD]
- Associated_resource [R]

📖 [Full service_catalog_appregistry documentation](services/service_catalog_appregistry.md)

### 97. Migrationhub_config

**Resources**: 3

- Home_region [R]
- Home_region_controls [R]
- Home_region_control [CD]

📖 [Full migrationhub_config documentation](services/migrationhub_config.md)

### 98. Iam

**Resources**: 37

- Account_password_policy [RUD]
- Context_keys_for_principal_policy [R]
- Service_linked_role_deletion_status [R]
- Role_description [U]
- Credential_report [R]
- Policy [CRD]
- User [CRUD]
- Group [CRUD]
- Role_policy [CRD]
- Context_keys_for_custom_policy [R]
- Login_profile [CRUD]
- Virtual_mfa_device [CD]
- Mfa_device [R]
- Account_alias [CD]
- Open_id_connect_provider [CRD]
- Access_key_last_used [R]
- User_policy [CRD]
- Group_policy [CRD]
- Signing_certificate [UD]
- Ssh_public_key [RUD]
- Access_key [CUD]
- Service_specific_credential [CUD]
- Instance_profile [CRD]
- Server_certificate [RUD]
- Service_last_accessed_details [R]
- Organizations_access_report [R]
- User_permissions_boundary [CD]
- Account_summary [R]
- Account_authorization_details [R]
- Policy_version [CRD]
- Assume_role_policy [U]
- Service_last_accessed_details_with_entities [R]
- Role_permissions_boundary [CD]
- Saml_provider [CRUD]
- Open_id_connect_provider_thumbprint [U]
- Role [CRUD]
- Service_linked_role [CD]

📖 [Full iam documentation](services/iam.md)

### 99. Accessanalyzer

**Resources**: 8

- Finding_recommendation [R]
- Generated_policy [R]
- Access_preview [CR]
- Findings_statistics [R]
- Analyzed_resource [R]
- Finding [R]
- Findings [U]
- Finding_v2 [R]

📖 [Full accessanalyzer documentation](services/accessanalyzer.md)

### 100. Appconfigdata

**Resources**: 1

- Latest_configuration [R]

📖 [Full appconfigdata documentation](services/appconfigdata.md)

### 101. Route53resolver

**Resources**: 17

- Firewall_rule_group [CRD]
- Resolver_rule_association [R]
- Resolver_dnssec_config [RU]
- Firewall_rule_group_policy [CR]
- Resolver_query_log_config [CRD]
- Firewall_config [RU]
- Firewall_rule_group_association [RU]
- Firewall_domains [U]
- Firewall_rule [CUD]
- Resolver_config [RU]
- Resolver_rule [CRUD]
- Resolver_endpoint [CRUD]
- Firewall_domain_list [CRD]
- Resolver_rule_policy [CR]
- Outpost_resolver [CRUD]
- Resolver_query_log_config_policy [CR]
- Resolver_query_log_config_association [R]

📖 [Full route53resolver documentation](services/route53resolver.md)

### 102. S3outposts

**Resources**: 1

- Endpoint [CD]

📖 [Full s3outposts documentation](services/s3outposts.md)

### 103. Kendra_ranking

**Resources**: 1

- Rescore_execution_plan [CRUD]

📖 [Full kendra_ranking documentation](services/kendra_ranking.md)

### 104. Controltower

**Resources**: 0


📖 [Full controltower documentation](services/controltower.md)

### 105. Arc_region_switch

**Resources**: 4

- Plan_execution_step [U]
- Plan_execution [RU]
- Plan_in_region [R]
- Plan_evaluation_status [R]

📖 [Full arc_region_switch documentation](services/arc_region_switch.md)

### 106. Neptune_graph

**Resources**: 2

- Query [R]
- Graph_summary [R]

📖 [Full neptune_graph documentation](services/neptune_graph.md)

### 107. Route53_recovery_readiness

**Resources**: 10

- Cell [CRUD]
- Cell_readiness_summary [R]
- Readiness_check_status [R]
- Recovery_group [CRUD]
- Architecture_recommendations [R]
- Cross_account_authorization [CD]
- Readiness_check [CRUD]
- Resource_set [CRUD]
- Readiness_check_resource_status [R]
- Recovery_group_readiness_summary [R]

📖 [Full route53_recovery_readiness documentation](services/route53_recovery_readiness.md)

### 108. Greengrassv2

**Resources**: 7

- Deployment [CRD]
- Core_device [RD]
- Connectivity_info [RU]
- Component [RD]
- Component_version_artifact [R]
- Component_version [C]
- Service_role_for_account [R]

📖 [Full greengrassv2 documentation](services/greengrassv2.md)

### 109. Migration_hub_refactor_spaces

**Resources**: 5

- Environment [CRD]
- Service [CRD]
- Route [CRUD]
- Application [CRD]
- Resource_policy [CRD]

📖 [Full migration_hub_refactor_spaces documentation](services/migration_hub_refactor_spaces.md)

### 110. Cost_and_usage_report_service

**Resources**: 2

- Report_definition [CD]
- Report_definitions [R]

📖 [Full cost_and_usage_report_service documentation](services/cost_and_usage_report_service.md)

### 111. Ebs

**Resources**: 1

- Snapshot_block [CR]

📖 [Full ebs documentation](services/ebs.md)

### 112. Appflow

**Resources**: 8

- Connectors [R]
- Connector_entity [R]
- Connector_registration [U]
- Flow_execution_records [R]
- Connector [R]
- Connector_profile [CUD]
- Connector_profiles [R]
- Flow [CRUD]

📖 [Full appflow documentation](services/appflow.md)

### 113. Migrationhuborchestrator

**Resources**: 0


📖 [Full migrationhuborchestrator documentation](services/migrationhuborchestrator.md)

### 114. Chime_sdk_identity

**Resources**: 7

- App_instance_user_endpoint [RU]
- App_instance_retention_settings [CR]
- App_instance [CRUD]
- App_instance_admin [CRD]
- App_instance_bot [CRUD]
- App_instance_user [CRUD]
- App_instance_user_expiration_settings [C]

📖 [Full chime_sdk_identity documentation](services/chime_sdk_identity.md)

### 115. Cloudfront_keyvaluestore

**Resources**: 3

- Key_value_store [R]
- Keys [U]
- Key [CRD]

📖 [Full cloudfront_keyvaluestore documentation](services/cloudfront_keyvaluestore.md)

### 116. Waf

**Resources**: 19

- Rule_group [CRUD]
- Web_acl_migration_stack [C]
- Geo_match_set [CRUD]
- Web_acl [CRUD]
- Rate_based_rule [CRUD]
- Permission_policy [CRD]
- Regex_match_set [CRUD]
- Rule [CRUD]
- Logging_configuration [CRD]
- Size_constraint_set [CRUD]
- Xss_match_set [CRUD]
- Change_token [R]
- Rate_based_rule_managed_keys [R]
- Regex_pattern_set [CRUD]
- Byte_match_set [CRUD]
- Sql_injection_match_set [CRUD]
- Change_token_status [R]
- Sampled_requests [R]
- Ip_set [CRUD]

📖 [Full waf documentation](services/waf.md)

### 117. Greengrass

**Resources**: 26

- Connectivity_info [RU]
- Bulk_deployment_status [R]
- Logger_definition [CRUD]
- Connector_definition [CRUD]
- Subscription_definition_version [CR]
- Associated_role [R]
- Thing_runtime_configuration [RU]
- Resource_definition [CRUD]
- Device_definition [CRUD]
- Function_definition_version [CR]
- Group_certificate_configuration [RU]
- Software_update_job [C]
- Subscription_definition [CRUD]
- Deployment_status [R]
- Core_definition [CRUD]
- Resource_definition_version [CR]
- Function_definition [CRUD]
- Logger_definition_version [CR]
- Group_certificate_authority [CR]
- Deployment [C]
- Core_definition_version [CR]
- Connector_definition_version [CR]
- Device_definition_version [CR]
- Group [CRUD]
- Group_version [CR]
- Service_role_for_account [R]

📖 [Full greengrass documentation](services/greengrass.md)

### 118. Sagemaker_featurestore_runtime

**Resources**: 1

- Record [CRD]

📖 [Full sagemaker_featurestore_runtime documentation](services/sagemaker_featurestore_runtime.md)

### 119. Inspector

**Resources**: 15

- Assessment_target [CUD]
- Exclusions_preview [CR]
- Findings [R]
- Resource_groups [R]
- Assessment_run [D]
- Exclusions [R]
- Rules_packages [R]
- Assessment_targets [R]
- Assessment_report [R]
- Assessment_runs [R]
- Assessment_templates [R]
- Assessment_template [CD]
- Cross_account_access_role [R]
- Telemetry_metadata [R]
- Resource_group [C]

📖 [Full inspector documentation](services/inspector.md)

### 120. Appfabric

**Resources**: 4

- Ingestion_destination [CRUD]
- App_bundle [CRD]
- App_authorization [CRUD]
- Ingestion [CRD]

📖 [Full appfabric documentation](services/appfabric.md)

### 121. Lex_model_building_service

**Resources**: 25

- Import [R]
- Intent_versions [R]
- Slot_types [R]
- Bot [CRD]
- Intent [CRD]
- Bot_alias [CRD]
- Slot_type [CRD]
- Utterances [D]
- Bot_channel_associations [R]
- Builtin_intent [R]
- Builtin_slot_types [R]
- Export [R]
- Intents [R]
- Bot_channel_association [RD]
- Migration [R]
- Migrations [R]
- Bot_versions [R]
- Bots [R]
- Bot_version [CD]
- Intent_version [CD]
- Slot_type_versions [R]
- Utterances_view [R]
- Bot_aliases [R]
- Slot_type_version [CD]
- Builtin_intents [R]

📖 [Full lex_model_building_service documentation](services/lex_model_building_service.md)

### 122. Serverlessapplicationrepository

**Resources**: 5

- Cloud_formation_template [CR]
- Application_policy [CR]
- Application [CRUD]
- Cloud_formation_change_set [C]
- Application_version [C]

📖 [Full serverlessapplicationrepository documentation](services/serverlessapplicationrepository.md)

### 123. Cloudsearch_domain

**Resources**: 0


📖 [Full cloudsearch_domain documentation](services/cloudsearch_domain.md)

### 124. Codeguru_security

**Resources**: 5

- Scan [CR]
- Findings [R]
- Metrics_summary [R]
- Upload_url [C]
- Account_configuration [RU]

📖 [Full codeguru_security documentation](services/codeguru_security.md)

### 125. Socialmessaging

**Resources**: 3

- Whats_app_message_template_from_library [C]
- Whats_app_message_template_media [C]
- Whats_app_message_template [CRUD]

📖 [Full socialmessaging documentation](services/socialmessaging.md)

### 126. Geo_maps

**Resources**: 0


📖 [Full geo_maps documentation](services/geo_maps.md)

### 127. Kinesis

**Resources**: 28

- Hls_streaming_session_url [R]
- Clip [R]
- Images [R]
- Media_for_fragment_list [R]
- Dash_streaming_session_url [R]
- Media [R]
- Notification_configuration [RU]
- Signaling_channel_endpoint [R]
- Signaling_channel [CRUD]
- Edge_configuration [RD]
- Data_endpoint [R]
- Image_generation_configuration [RU]
- Media_storage_configuration [RU]
- Mapped_resource_configuration [R]
- Stream [CRUD]
- Data_retention [U]
- Shard_iterator [R]
- Limits [R]
- Record [C]
- Resource_policy [CRD]
- Stream_consumer [R]
- Records [CR]
- Stream_mode [U]
- Shard_count [U]
- Stream [CRD]
- Max_record_size [U]
- Stream_summary [R]
- Ice_server_config [R]

📖 [Full kinesis documentation](services/kinesis.md)

### 128. Dsql

**Resources**: 0


📖 [Full dsql documentation](services/dsql.md)

### 129. Appintegrations

**Resources**: 4

- Application [CRUD]
- Event_integration [CRUD]
- Data_integration [CRUD]
- Data_integration_association [CU]

📖 [Full appintegrations documentation](services/appintegrations.md)

### 130. Personalize

**Resources**: 19

- Solution_metrics [R]
- Dataset_import_job [CR]
- Solution [CRUD]
- Dataset [CRUD]
- Batch_segment_job [CR]
- Recipe [R]
- Filter [CRD]
- Algorithm [R]
- Batch_inference_job [CR]
- Data_deletion_job [CR]
- Dataset_group [CRD]
- Metric_attribution [CRUD]
- Recommender [CRUD]
- Campaign [CRUD]
- Schema [CRD]
- Solution_version [CR]
- Feature_transformation [R]
- Dataset_export_job [CR]
- Event_tracker [CRD]

📖 [Full personalize documentation](services/personalize.md)

### 131. Proton

**Resources**: 4

- Service_instance_sync_status [R]
- Repository_sync_status [R]
- Template_sync_status [R]
- Resources_summary [R]

📖 [Full proton documentation](services/proton.md)

### 132. Cloudcontrol

**Resources**: 2

- Resource_request_status [R]
- Resource [CRUD]

📖 [Full cloudcontrol documentation](services/cloudcontrol.md)

### 133. Redshift

**Resources**: 64

- Reserved_node_offerings [R]
- Inbound_integrations [R]
- Cluster_parameter_group [CD]
- Tags [CRD]
- Cluster [CD]
- Resource_policy [CRD]
- Cluster_snapshot [CD]
- Cluster_security_group [CD]
- Account_attributes [R]
- Integrations [R]
- Snapshot_schedules [R]
- Cluster_credentials_with_iam [R]
- Data_shares_for_producer [R]
- Cluster_db_revisions [R]
- Endpoint_authorization [R]
- Scheduled_actions [R]
- Node_configuration_options [R]
- Snapshot_copy_grant [CD]
- Hsm_client_certificates [R]
- Clusters [R]
- Authentication_profile [CD]
- Redshift_idc_application [CD]
- Event_categories [R]
- Hsm_configuration [CD]
- Resize [R]
- Cluster_subnet_group [CD]
- Orderable_cluster_options [R]
- Usage_limit [CD]
- Event_subscription [CD]
- Events [R]
- Cluster_security_groups [R]
- Authentication_profiles [R]
- Reserved_node_exchange_offerings [R]
- Snapshot_schedule [CD]
- Cluster_snapshots [R]
- Cluster_tracks [R]
- Scheduled_action [CD]
- Hsm_client_certificate [CD]
- Partner [D]
- Cluster_parameter_groups [R]
- Event_subscriptions [R]
- Cluster_parameters [R]
- Hsm_configurations [R]
- Reserved_node_exchange_status [R]
- Integration [CD]
- Table_restore_status [R]
- Endpoint_access [CRD]
- Logging_status [R]
- Cluster_versions [R]
- Default_cluster_parameters [R]
- Partner_status [U]
- Storage [R]
- Cluster_subnet_groups [R]
- Reserved_nodes [R]
- Data_shares [R]
- Snapshot_copy_grants [R]
- Cluster_credentials [R]
- Redshift_idc_applications [R]
- Reserved_node_exchange_configuration_options [R]
- Usage_limits [R]
- Partners [R]
- Custom_domain_associations [R]
- Custom_domain_association [CD]
- Data_shares_for_consumer [R]

📖 [Full redshift documentation](services/redshift.md)

### 134. Geo_places

**Resources**: 0


📖 [Full geo_places documentation](services/geo_places.md)

### 135. Elasticsearch_service

**Resources**: 21

- Elasticsearch_domains [R]
- Elasticsearch_domain [CRD]
- Elasticsearch_domain_config [RU]
- Domain_change_progress [R]
- Elasticsearch_instance_type_limits [R]
- Package_version_history [R]
- Vpc_endpoint [CUD]
- Outbound_cross_cluster_search_connection [CD]
- Vpc_endpoints [R]
- Reserved_elasticsearch_instance_offerings [R]
- Upgrade_history [R]
- Packages [R]
- Elasticsearch_service_role [D]
- Reserved_elasticsearch_instances [R]
- Inbound_cross_cluster_search_connections [R]
- Package [CUD]
- Upgrade_status [R]
- Inbound_cross_cluster_search_connection [D]
- Domain_auto_tunes [R]
- Outbound_cross_cluster_search_connections [R]
- Compatible_elasticsearch_versions [R]

📖 [Full elasticsearch_service documentation](services/elasticsearch_service.md)

### 136. Bcm_recommended_actions

**Resources**: 0


📖 [Full bcm_recommended_actions documentation](services/bcm_recommended_actions.md)

### 137. Invoicing

**Resources**: 1

- Invoice_unit [CRUD]

📖 [Full invoicing documentation](services/invoicing.md)

### 138. Apprunner

**Resources**: 8

- Default_auto_scaling_configuration [U]
- Vpc_ingress_connection [CRUD]
- Observability_configuration [CRD]
- Service [CRUD]
- Connection [CD]
- Custom_domains [R]
- Vpc_connector [CRD]
- Auto_scaling_configuration [CRD]

📖 [Full apprunner documentation](services/apprunner.md)

### 139. Sns

**Resources**: 12

- Sms_sandbox_phone_number [CD]
- Sms_attributes [R]
- Endpoint_attributes [R]
- Topic [CD]
- Topic_attributes [R]
- Data_protection_policy [CR]
- Subscription_attributes [R]
- Platform_endpoint [C]
- Platform_application_attributes [R]
- Endpoint [D]
- Sms_sandbox_account_status [R]
- Platform_application [CD]

📖 [Full sns documentation](services/sns.md)

### 140. Textract

**Resources**: 7

- Adapter_version [CRD]
- Lending_analysis_summary [R]
- Lending_analysis [R]
- Document_text_detection [R]
- Adapter [CRUD]
- Document_analysis [R]
- Expense_analysis [R]

📖 [Full textract documentation](services/textract.md)

### 141. Workmail

**Resources**: 29

- Primary_email_address [U]
- Mobile_device_access_rule [CUD]
- Access_control_rule [CD]
- Organization [CRD]
- Impersonation_role [CRUD]
- Resource [CRUD]
- Identity_center_application [CD]
- Access_control_effect [R]
- Email_monitoring_configuration [CRD]
- Default_mail_domain [U]
- Mailbox_quota [U]
- Retention_policy [CD]
- Personal_access_token [D]
- Default_retention_policy [R]
- Availability_configuration [CUD]
- Mail_domain [R]
- Mailbox_details [R]
- Mobile_device_access_effect [R]
- User [CRUD]
- Identity_provider_configuration [CRD]
- Inbound_dmarc_settings [CR]
- Personal_access_token_metadata [R]
- Group [CRUD]
- Alias [CD]
- Impersonation_role_effect [R]
- Entity [R]
- Mobile_device_access_override [CRD]
- Mailbox_permissions [CD]
- Mailbox_export_job [R]

📖 [Full workmail documentation](services/workmail.md)

### 142. Datazone

**Resources**: 26

- Environment [CRUD]
- Environment_credentials [R]
- Account_pool [CRUD]
- Group_profile [CRU]
- Connection [CRUD]
- Subscription [R]
- Environment_blueprint [CRUD]
- Time_series_data_points [D]
- Subscription_grant [CRD]
- Listing_change_set [C]
- Subscription_request_details [R]
- Asset_filter [CRUD]
- Iam_portal_login_url [R]
- Project_profile [CRUD]
- Time_series_data_point [R]
- Lineage_event [R]
- Subscription_grant_status [U]
- Subscription_request [CUD]
- Subscription_target [CRUD]
- Project_membership [CD]
- Job_run [R]
- Project [CRUD]
- Lineage_node [R]
- Environment_action [CRUD]
- Environment_profile [CRUD]
- User_profile [CRU]

📖 [Full datazone documentation](services/datazone.md)

### 143. Rekognition

**Resources**: 23

- Dataset_entries [U]
- Collection [CRD]
- Project [CD]
- Celebrity_recognition [R]
- Stream_processor [CRUD]
- Content_moderation [R]
- Media_analysis_job [R]
- Person_tracking [R]
- Face_liveness_session_results [R]
- Face_search [R]
- User [CD]
- Project_versions [R]
- Project_policy [CD]
- Dataset [CRD]
- Projects [R]
- Celebrity_info [R]
- Face_liveness_session [C]
- Face_detection [R]
- Label_detection [R]
- Project_version [CD]
- Segment_detection [R]
- Faces [D]
- Text_detection [R]

📖 [Full rekognition documentation](services/rekognition.md)

### 144. Ssm

**Resources**: 68

- Document_default_version [U]
- Maintenance_window_targets [R]
- Parameter_history [R]
- Effective_patches_for_patch_baseline [R]
- Maintenance_window_target [U]
- Association [CRUD]
- Association_execution_targets [R]
- Calendar_state [R]
- Ops_item [CRUD]
- Maintenance_window_executions [R]
- Activation [CD]
- Effective_instance_associations [R]
- Execution_preview [R]
- Resource_policies [R]
- Association_batch [C]
- Instance_associations_status [R]
- Document [CRUD]
- Ops_summary [R]
- Command_invocation [R]
- Compliance_items [C]
- Patch_baselines [R]
- Access_token [R]
- Maintenance_window_tasks [R]
- Maintenance_window_execution_tasks [R]
- Deployable_patch_snapshot_for_instance [R]
- Maintenance_window_schedule [R]
- Document_metadata [U]
- Association_status [U]
- Maintenance_window_execution_task_invocations [R]
- Instance_patch_states_for_patch_group [R]
- Patch_groups [R]
- Automation_step_executions [R]
- Patch_baseline_for_patch_group [R]
- Ops_metadata [CRUD]
- Inventory [CRD]
- Patch_properties [R]
- Instance_patch_states [R]
- Instance_information [R]
- Resource_policy [CD]
- Activations [R]
- Maintenance_window_execution_task_invocation [R]
- Maintenance_windows_for_target [R]
- Service_setting [RU]
- Automation_execution [R]
- Patch_group_state [R]
- Maintenance_window [CRUD]
- Parameter [CRD]
- Sessions [R]
- Resource_data_sync [CUD]
- Managed_instance_role [U]
- Default_patch_baseline [R]
- Automation_executions [R]
- Connection_status [R]
- Ops_items [R]
- Maintenance_window_execution [R]
- Maintenance_window_task [RU]
- Document_permission [R]
- Instance_properties [R]
- Inventory_schema [R]
- Association_executions [R]
- Parameters_by_path [R]
- Patch_baseline [CRUD]
- Parameters [RD]
- Available_patches [R]
- Maintenance_window_execution_task [R]
- Instance_patches [R]
- Inventory_deletions [R]
- Maintenance_windows [R]

📖 [Full ssm documentation](services/ssm.md)

### 145. Medical_imaging

**Resources**: 4

- Dicom_import_job [R]
- Image_set [RD]
- Image_set_metadata [RU]
- Image_frame [R]

📖 [Full medical_imaging documentation](services/medical_imaging.md)

### 146. Lex_models

**Resources**: 23

- Bot_resource_generation [R]
- Slot_type [CRUD]
- Custom_vocabulary [D]
- Slot [CRUD]
- Utterances [D]
- Custom_vocabulary_metadata [R]
- Resource_policy [CRUD]
- Bot_version [CRD]
- Test_set_generation [R]
- Bot_locale [CRUD]
- Export [CRUD]
- Test_set [RUD]
- Bot_alias [CRUD]
- Intent [CRUD]
- Bot [CRUD]
- Bot_replica [CRD]
- Import [RD]
- Upload_url [C]
- Resource_policy_statement [CD]
- Test_set_discrepancy_report [CR]
- Test_execution [R]
- Test_execution_artifacts_url [R]
- Bot_recommendation [RU]

📖 [Full lex_models documentation](services/lex_models.md)

### 147. Support

**Resources**: 12

- Cases [R]
- Case [C]
- Attachment [R]
- Services [R]
- Severity_levels [R]
- Create_case_options [R]
- Trusted_advisor_check_refresh_statuses [R]
- Communications [R]
- Trusted_advisor_check_summaries [R]
- Trusted_advisor_checks [R]
- Trusted_advisor_check_result [R]
- Supported_languages [R]

📖 [Full support documentation](services/support.md)

### 148. Signer

**Resources**: 4

- Signing_job [R]
- Signing_profile [CR]
- Revocation_status [R]
- Signing_platform [R]

📖 [Full signer documentation](services/signer.md)

### 149. Partnercentral_selling

**Resources**: 1

- Selling_system_settings [CR]

📖 [Full partnercentral_selling documentation](services/partnercentral_selling.md)

### 150. Comprehendmedical

**Resources**: 5

- Icd10_cm_inference_job [R]
- Snomedct_inference_job [R]
- Phi_detection_job [R]
- Rx_norm_inference_job [R]
- Entities_detection_v2_job [R]

📖 [Full comprehendmedical documentation](services/comprehendmedical.md)

### 151. Macie2

**Resources**: 29

- Sample_findings [C]
- Custom_data_identifier [CRD]
- Macie_session [RU]
- Findings [R]
- Sensitive_data_occurrences_availability [R]
- Member_session [U]
- Invitations [CD]
- Sensitive_data_occurrences [R]
- Master_account [R]
- Member [CRD]
- Resource_profile [RU]
- Classification_job [CRU]
- Automated_discovery_configuration [RU]
- Organization_configuration [RU]
- Findings_publication_configuration [CR]
- Findings_filter [CRUD]
- Resource_profile_detections [U]
- Usage_statistics [R]
- Sensitivity_inspection_template [RU]
- Bucket_statistics [R]
- Buckets [R]
- Classification_scope [RU]
- Classification_export_configuration [CR]
- Allow_list [CRUD]
- Reveal_configuration [RU]
- Administrator_account [R]
- Finding_statistics [R]
- Invitations_count [R]
- Usage_totals [R]

📖 [Full macie2 documentation](services/macie2.md)

### 152. Redshift_data

**Resources**: 4

- Statement [R]
- Statement_result [R]
- Table [R]
- Statement_result_v2 [R]

📖 [Full redshift_data documentation](services/redshift_data.md)

### 153. Marketplace_agreement

**Resources**: 2

- Agreement [R]
- Agreement_terms [R]

📖 [Full marketplace_agreement documentation](services/marketplace_agreement.md)

### 154. Health

**Resources**: 12

- Event_aggregates [R]
- Event_details_for_organization [R]
- Affected_accounts_for_organization [R]
- Event_details [R]
- Entity_aggregates [R]
- Affected_entities [R]
- Affected_entities_for_organization [R]
- Events [R]
- Events_for_organization [R]
- Health_service_status_for_organization [R]
- Entity_aggregates_for_organization [R]
- Event_types [R]

📖 [Full health documentation](services/health.md)

### 155. Odb

**Resources**: 1

- Oci_onboarding_status [R]

📖 [Full odb documentation](services/odb.md)

### 156. Resource_groups_tagging_api

**Resources**: 5

- Resources [R]
- Tag_keys [R]
- Report_creation [R]
- Compliance_summary [R]
- Tag_values [R]

📖 [Full resource_groups_tagging_api documentation](services/resource_groups_tagging_api.md)

### 157. Application_insights

**Resources**: 9

- Component_configuration_recommendation [R]
- Component [CRUD]
- Problem [RU]
- Component_configuration [RU]
- Log_pattern [CRUD]
- Observation [R]
- Problem_observations [R]
- Workload [RU]
- Application [CRUD]

📖 [Full application_insights documentation](services/application_insights.md)

### 158. Timestream_write

**Resources**: 4

- Database [CRUD]
- Endpoints [R]
- Batch_load_task [CR]
- Table [CRUD]

📖 [Full timestream_write documentation](services/timestream_write.md)

### 159. Pinpoint_sms

**Resources**: 47

- Configuration_set [CD]
- Configuration_set_event_destination [CUD]
- Configuration_set_event_destinations [R]
- Keywords [R]
- Default_message_type [D]
- Protect_configuration_country_rule_set [RU]
- Opted_out_number [CD]
- Registration_field_values [R]
- Default_sender_id [D]
- Configuration_sets [R]
- Text_message_spend_limit_override [D]
- Account_limits [R]
- Protect_configurations [R]
- Spend_limits [R]
- Opt_out_lists [R]
- Registration_section_definitions [R]
- Protect_configuration_rule_set_number_override [CD]
- Opt_out_list [CD]
- Pool [CUD]
- Media_message_spend_limit_override [D]
- Sender_ids [R]
- Account_default_protect_configuration [D]
- Verified_destination_number [CD]
- Keyword [CD]
- Registration_version [C]
- Event_destination [CUD]
- Registration_attachment [CD]
- Protect_configuration [CUD]
- Voice_message_spend_limit_override [D]
- Account_attributes [R]
- Opted_out_numbers [R]
- Registration_field_definitions [R]
- Registration [CD]
- Phone_numbers [R]
- Resource_policy [CRD]
- Registration_versions [R]
- Registration_attachments [R]
- Registrations [R]
- Registration_type_definitions [R]
- Message_feedback [C]
- Sender_id [U]
- Verified_destination_numbers [R]
- Registration_association [C]
- Configuration_set [CD]
- Pools [R]
- Phone_number [U]
- Registration_field_value [CD]

📖 [Full pinpoint_sms documentation](services/pinpoint_sms.md)

### 160. Mediapackagev2

**Resources**: 0


📖 [Full mediapackagev2 documentation](services/mediapackagev2.md)

### 161. Ec2

**Resources**: 335

- Vpcs [R]
- Spot_datafeed_subscription [CRD]
- Aws_network_performance_data [R]
- Transit_gateway_connects [R]
- Transit_gateway_attachments [R]
- Instances [R]
- Local_gateway_route_table_virtual_interface_group_associations [R]
- Export_tasks [R]
- Capacity_reservation_billing_requests [R]
- Capacity_reservations [R]
- Id_format [R]
- Groups_for_capacity_reservation [R]
- Regions [R]
- Delegate_mac_volume_ownership_task [C]
- Subnet_cidr_reservations [R]
- Verified_access_group_policy [R]
- Managed_prefix_lists [R]
- Capacity_reservation_usage [R]
- Launch_template_versions [RD]
- Subnet [CD]
- Vpc_endpoint_service_configuration [C]
- Transit_gateway_connect_peers [R]
- Network_insights_access_scope_content [R]
- Subnet_cidr_reservation [CD]
- Vpc [CD]
- Import_snapshot_tasks [R]
- Ipam_prefix_list_resolver_target [CD]
- Instance_credit_specifications [R]
- Fleet [C]
- Fleets [RD]
- Instance_attribute [R]
- Launch_templates [R]
- Reserved_instances [R]
- Reserved_instances_listings [R]
- Image_usage_report_entries [R]
- Vpc_peering_connection [CD]
- Store_image_task [C]
- Iam_instance_profile_associations [R]
- Route_server_associations [R]
- Snapshot_block_public_access_state [R]
- Local_gateway_route_table_vpc_association [CD]
- Principal_id_format [R]
- Vpc_block_public_access_exclusion [CD]
- Scheduled_instances [R]
- Fleet_history [R]
- Key_pairs [R]
- Ipam_resource_discovery [CD]
- Capacity_manager_organizations_access [U]
- Local_gateway_route [CD]
- Traffic_mirror_target [CD]
- Client_vpn_endpoint [CD]
- Transit_gateway_policy_table [CD]
- Customer_gateways [R]
- Ipam_prefix_list_resolver_targets [R]
- Ipam_resource_discoveries [R]
- Reserved_instances_exchange_quote [R]
- Vpc_peering_connections [R]
- Route_table [CD]
- Prefix_lists [R]
- Network_insights_access_scope_analysis [D]
- Console_output [R]
- Security_group_rule_descriptions_egress [U]
- Volume_status [R]
- Vpc_classic_link [R]
- Queued_reserved_instances [D]
- Import_image_tasks [R]
- Instance_event_notification_attributes [R]
- Spot_fleet_requests [R]
- Capacity_blocks [R]
- Client_vpn_target_networks [R]
- Mac_system_integrity_protection_modification_task [C]
- Capacity_reservation_fleets [R]
- Transit_gateway_multicast_domain [CD]
- Fpga_image_attribute [R]
- Mac_modification_tasks [R]
- Vpc_endpoints [RD]
- Network_insights_access_scope_analyses [R]
- Fpga_images [R]
- Traffic_mirror_sessions [R]
- Vpc_endpoint_connections [R]
- Coip_pools [R]
- Vpc_endpoint_service_permissions [R]
- Fast_launch_images [R]
- Instance_event_window [CD]
- Ebs_encryption_by_default [R]
- Launch_template_version [C]
- Transit_gateway_peering_attachment [CD]
- Instance_event_windows [R]
- Vpc_endpoint_associations [R]
- Declarative_policies_reports [R]
- Traffic_mirror_filter_rules [R]
- Tags [CRD]
- Image_attribute [R]
- Verified_access_instances [R]
- Allowed_images_settings [R]
- Declarative_policies_report_summary [R]
- Transit_gateway_attachment_propagations [R]
- Instance_status [R]
- Vpn_gateway [CD]
- Network_interface_permission [CD]
- Volumes_modifications [R]
- Instance_uefi_data [R]
- Carrier_gateway [CD]
- Launch_template [CD]
- Associated_enclave_certificate_iam_roles [R]
- Verified_access_endpoint_policy [R]
- Key_pair [CD]
- Conversion_tasks [R]
- Local_gateway_route_tables [R]
- Capacity_block_extension_history [R]
- Outpost_lags [R]
- Instance_tpm_ek_pub [R]
- Moving_addresses [R]
- Volume_attribute [R]
- Ipam_discovered_public_addresses [R]
- Capacity_manager_metric_data [R]
- Transit_gateway_peering_attachments [R]
- Security_group [CD]
- Capacity_manager_data_export [CD]
- Transit_gateway_connect_peer [CD]
- Verified_access_endpoint [CD]
- Network_interfaces [R]
- Ipam_resource_cidrs [R]
- Egress_only_internet_gateways [R]
- Aws_network_performance_metric_subscriptions [R]
- Mac_hosts [R]
- Vpc_block_public_access_options [R]
- Associated_ipv6_pool_cidrs [R]
- Local_gateway_route_table_vpc_associations [R]
- Network_acl_entry [CD]
- Host_reservation_offerings [R]
- Flow_logs_integration_template [R]
- Spot_fleet_instances [R]
- Transit_gateway_policy_tables [R]
- Ipam_pool_allocations [R]
- Public_ipv4_pools [R]
- Network_insights_access_scope_analysis_findings [R]
- Security_group_rule_descriptions_ingress [U]
- Internet_gateway [CD]
- Coip_pool_usage [R]
- Reserved_instances_listing [C]
- Transit_gateway_route [CD]
- Capacity_block_offerings [R]
- Instance_connect_endpoints [R]
- Transit_gateways [R]
- Launch_template_data [R]
- Console_screenshot [R]
- Traffic_mirror_targets [R]
- Volume [CD]
- Ipam [CD]
- Image_usage_reports [R]
- Default_credit_specification [R]
- Customer_gateway [CD]
- Local_gateway_route_table_virtual_interface_group_association [CD]
- Ipam_discovered_accounts [R]
- Transit_gateway_route_tables [R]
- Security_groups_for_vpc [R]
- Instance_topology [R]
- Managed_prefix_list_associations [R]
- Vpn_gateways [R]
- Ipam_external_resource_verification_token [CD]
- Vpc_endpoint_connection_notifications [RD]
- Vpc_block_public_access_exclusions [R]
- Vpn_connection_device_sample_configuration [R]
- Image_references [R]
- Local_gateway_virtual_interface_groups [R]
- Ipam_pools [R]
- Route_servers [R]
- Traffic_mirror_filters [R]
- Transit_gateway [CD]
- Instance_image_metadata [R]
- Route_server_propagations [R]
- Transit_gateway_policy_table_associations [R]
- Replace_root_volume_task [C]
- Verified_access_groups [R]
- Verified_access_instance_logging_configurations [R]
- Fast_snapshot_restores [R]
- Ipam_pool [CD]
- Capacity_block_extension_offerings [R]
- Ipam_prefix_list_resolver_versions [R]
- Account_attributes [R]
- Default_vpc [C]
- Fleet_instances [R]
- Spot_fleet_request_history [R]
- Route_tables [R]
- Image_usage_report [CD]
- Snapshot [CD]
- Availability_zones [R]
- Network_insights_access_scopes [R]
- Route_server_peers [R]
- Security_group_vpc_associations [R]
- Ebs_default_kms_key_id [R]
- Coip_pool [CD]
- Network_insights_access_scope [CD]
- Ipam_discovered_resource_cidrs [R]
- Route_server_peer [CD]
- Managed_prefix_list [CD]
- Capacity_manager_attributes [R]
- Vpn_connection [CD]
- Verified_access_instance [CD]
- Dhcp_options [CRD]
- Client_vpn_endpoints [R]
- Client_vpn_connections [R]
- Stale_security_groups [R]
- Ipam_byoasn [R]
- Traffic_mirror_filter_rule [CD]
- Ipam_prefix_list_resolver_rules [R]
- Vpc_classic_link_dns_support [R]
- Network_acl [CD]
- Transit_gateway_route_table [CD]
- Export_image_tasks [R]
- Internet_gateways [R]
- Ipam_external_resource_verification_tokens [R]
- Default_subnet [C]
- Vpc_endpoint_connection_notification [C]
- Elastic_gpus [R]
- Scheduled_instance_availability [R]
- Transit_gateway_multicast_domains [R]
- Instance_type_offerings [R]
- Addresses [R]
- Verified_access_trust_providers [R]
- Route_server_endpoint [CD]
- Service_link_virtual_interfaces [R]
- Snapshot_attribute [R]
- Serial_console_access_status [R]
- Capacity_reservation_fleet [C]
- Vpc_endpoint_services [R]
- Bundle_tasks [R]
- Snapshot_tier_status [R]
- Reserved_instances_modifications [R]
- Transit_gateway_connect [CD]
- Network_acls [R]
- Vpc_endpoint_service_configurations [RD]
- Network_insights_analyses [R]
- Vpn_connection_route [CD]
- Subnets [R]
- Security_group_rules [R]
- Address_transfers [R]
- Traffic_mirror_session [CD]
- Security_groups [R]
- Placement_groups [R]
- Transit_gateway_prefix_list_reference [CD]
- Capacity_block_status [R]
- Restore_image_task [C]
- Placement_group [CD]
- Local_gateway_route_table [CD]
- Security_group_references [R]
- Volumes [R]
- Local_gateway_virtual_interface [CD]
- Vpn_tunnel_replacement_status [R]
- Active_vpn_tunnel_status [R]
- Ipam_scopes [R]
- Verified_access_group [CD]
- Client_vpn_authorization_rules [R]
- Transit_gateway_route_table_propagations [R]
- Network_insights_analysis [D]
- Password_data [R]
- Host_reservations [R]
- Hosts [R]
- Vpn_connection_device_types [R]
- Network_interface_permissions [R]
- Ipam_address_history [R]
- Vpc_endpoint [C]
- Ipam_prefix_list_resolver [CD]
- Aggregate_id_format [R]
- Transit_gateway_route_table_announcements [R]
- Flow_logs [CRD]
- Network_insights_paths [R]
- Image_block_public_access_state [R]
- Transit_gateway_policy_table_entries [R]
- Reserved_instances_offerings [R]
- Transit_gateway_route_table_announcement [CD]
- Ipam_prefix_list_resolvers [R]
- Verified_access_endpoint_targets [R]
- Instance_types_from_instance_requirements [R]
- Carrier_gateways [R]
- Transit_gateway_vpc_attachments [R]
- Snapshots [CR]
- Local_gateway_virtual_interfaces [R]
- Ipam_resource_discovery_associations [R]
- Nat_gateways [R]
- Images [R]
- Image [C]
- Nat_gateway [CD]
- Route_server_endpoints [R]
- Route_server [CD]
- Addresses_attribute [R]
- Host_reservation_purchase_preview [R]
- Instance_types [R]
- Route_server_routing_database [R]
- Trunk_interface_associations [R]
- Verified_access_endpoints [R]
- Verified_access_trust_provider [CD]
- Spot_price_history [R]
- Vpc_attribute [R]
- Spot_instance_requests [R]
- Instance_metadata_defaults [R]
- Ipam_prefix_list_resolver_version_entries [R]
- Network_insights_path [CD]
- Ipam_scope [CD]
- Identity_id_format [R]
- Managed_prefix_list_entries [R]
- Transit_gateway_multicast_domain_associations [R]
- Spot_placement_scores [R]
- Transit_gateway_prefix_list_references [R]
- Client_vpn_routes [R]
- Ipv6_pools [R]
- Transit_gateway_route_table_associations [R]
- Instance_export_task [C]
- Traffic_mirror_filter [CD]
- Transit_gateway_vpc_attachment [CD]
- Instance_connect_endpoint [CD]
- Capacity_reservation_by_splitting [C]
- Local_gateways [R]
- Locked_snapshots [R]
- Vpn_connections [R]
- Fpga_image [CD]
- Network_interface_attribute [R]
- Ipam_pool_cidrs [R]
- Ipams [R]
- Public_ipv4_pool [CD]
- Byoip_cidrs [R]
- Capacity_reservation_topology [R]
- Classic_link_instances [R]
- Local_gateway_virtual_interface_group [CD]
- Route [CD]
- Capacity_manager_metric_dimensions [R]
- Coip_cidr [CD]
- Egress_only_internet_gateway [CD]
- Capacity_reservation [C]
- Client_vpn_route [CD]
- Network_interface [CD]
- Capacity_manager_data_exports [R]
- Store_image_tasks [R]
- Replace_root_volume_tasks [R]

📖 [Full ec2 documentation](services/ec2.md)

### 162. Cleanrooms

**Resources**: 0


📖 [Full cleanrooms documentation](services/cleanrooms.md)

### 163. Healthlake

**Resources**: 3

- Fhir_datastore [CRD]
- Fhir_export_job [R]
- Fhir_import_job [R]

📖 [Full healthlake documentation](services/healthlake.md)

### 164. Sfn

**Resources**: 9

- State_machine_alias [CRUD]
- Execution_history [R]
- Map_run [RU]
- State_machine [CRUD]
- Activity_task [R]
- Execution [R]
- Activity [CRD]
- State_machine_for_execution [R]
- State_machine_version [D]

📖 [Full sfn documentation](services/sfn.md)

### 165. Iottwinmaker

**Resources**: 9

- Property_value [R]
- Workspace [CRUD]
- Entity [CRUD]
- Scene [CRUD]
- Component_type [CRUD]
- Property_value_history [R]
- Pricing_plan [RU]
- Sync_job [CRD]
- Metadata_transfer_job [CR]

📖 [Full iottwinmaker documentation](services/iottwinmaker.md)

### 166. Cloudtrail

**Resources**: 13

- Trail_status [R]
- Channel [CRUD]
- Resource_policy [CRD]
- Dashboard [CRUD]
- Query [R]
- Event_selectors [CR]
- Query_results [R]
- Insight_selectors [CR]
- Trails [R]
- Event_configuration [CR]
- Import [R]
- Trail [CRUD]
- Event_data_store [CRUD]

📖 [Full cloudtrail documentation](services/cloudtrail.md)

### 167. Iotdeviceadvisor

**Resources**: 4

- Endpoint [R]
- Suite_run [R]
- Suite_run_report [R]
- Suite_definition [CRUD]

📖 [Full iotdeviceadvisor documentation](services/iotdeviceadvisor.md)

### 168. Ssm_incidents

**Resources**: 8

- Deletion_protection [U]
- Replication_set [CRUD]
- Timeline_event [CRUD]
- Related_items [U]
- Resource_policy [CD]
- Incident_record [RUD]
- Response_plan [CRUD]
- Resource_policies [R]

📖 [Full ssm_incidents documentation](services/ssm_incidents.md)

### 169. Pcs

**Resources**: 0


📖 [Full pcs documentation](services/pcs.md)

### 170. Support_app

**Resources**: 3

- Slack_channel_configuration [CUD]
- Account_alias [CRD]
- Slack_workspace_configuration [D]

📖 [Full support_app documentation](services/support_app.md)

### 171. Managedblockchain_query

**Resources**: 3

- Asset_contract [R]
- Transaction [R]
- Token_balance [R]

📖 [Full managedblockchain_query documentation](services/managedblockchain_query.md)

### 172. Iot_events_data

**Resources**: 2

- Alarm [R]
- Detector [R]

📖 [Full iot_events_data documentation](services/iot_events_data.md)

### 173. Lex_runtime

**Resources**: 1

- Session [CRD]

📖 [Full lex_runtime documentation](services/lex_runtime.md)

### 174. Observabilityadmin

**Resources**: 6

- Telemetry_enrichment_status [R]
- Telemetry_rule_for_organization [CRUD]
- Centralization_rule_for_organization [CRUD]
- Telemetry_evaluation_status_for_organization [R]
- Telemetry_evaluation_status [R]
- Telemetry_rule [CRUD]

📖 [Full observabilityadmin documentation](services/observabilityadmin.md)

### 175. Applicationcostprofiler

**Resources**: 1

- Report_definition [CRUD]

📖 [Full applicationcostprofiler documentation](services/applicationcostprofiler.md)

### 176. Billingconductor

**Resources**: 1

- Billing_group_cost_report [R]

📖 [Full billingconductor documentation](services/billingconductor.md)

### 177. Artifact

**Resources**: 0


📖 [Full artifact documentation](services/artifact.md)

### 178. Ecr_public

**Resources**: 10

- Repository_policy [RD]
- Repository [CD]
- Images [R]
- Repositories [R]
- Registry_catalog_data [CR]
- Repository_catalog_data [CR]
- Authorization_token [R]
- Image_tags [R]
- Registries [R]
- Image [C]

📖 [Full ecr_public documentation](services/ecr_public.md)

### 179. Connectparticipant

**Resources**: 5

- Authentication_url [R]
- Transcript [R]
- View [R]
- Participant_connection [C]
- Attachment [R]

📖 [Full connectparticipant documentation](services/connectparticipant.md)

### 180. Rds_data

**Resources**: 0


📖 [Full rds_data documentation](services/rds_data.md)

### 181. Internetmonitor

**Resources**: 0


📖 [Full internetmonitor documentation](services/internetmonitor.md)

### 182. Route_53

**Resources**: 24

- Reusable_delegation_set [CRD]
- Health_check [CRUD]
- Traffic_policy_instance_count [R]
- Checker_ip_ranges [R]
- Health_check_last_failure_reason [R]
- Hosted_zone_count [R]
- Query_logging_config [CRD]
- Health_check_status [R]
- Traffic_policy [CRD]
- Key_signing_key [CD]
- Account_limit [R]
- Health_check_count [R]
- Change [R]
- Hosted_zone_comment [U]
- Vpc_association_authorization [CD]
- Reusable_delegation_set_limit [R]
- Traffic_policy_comment [U]
- Hosted_zone [CRD]
- Traffic_policy_instance [CRUD]
- Cidr_collection [CD]
- Dnssec [R]
- Traffic_policy_version [C]
- Geo_location [R]
- Hosted_zone_limit [R]

📖 [Full route_53 documentation](services/route_53.md)

### 183. Bedrock_runtime

**Resources**: 0


📖 [Full bedrock_runtime documentation](services/bedrock_runtime.md)

### 184. Amplifybackend

**Resources**: 8

- Backend [CRD]
- Backend_api [CRUD]
- Backend_config [CU]
- Token [CRD]
- Backend_auth [CRUD]
- Backend_job [RU]
- Backend_api_models [R]
- Backend_storage [CRUD]

📖 [Full amplifybackend documentation](services/amplifybackend.md)

### 185. Marketplace_deployment

**Resources**: 0


📖 [Full marketplace_deployment documentation](services/marketplace_deployment.md)

### 186. Account

**Resources**: 0


📖 [Full account documentation](services/account.md)

### 187. Snowball

**Resources**: 11

- Snowball_usage [R]
- Software_updates [R]
- Addresses [R]
- Job_shipment_state [U]
- Job [CRU]
- Cluster [CRU]
- Long_term_pricing [CU]
- Job_unlock_code [R]
- Address [CR]
- Job_manifest [R]
- Return_shipping_label [CR]

📖 [Full snowball documentation](services/snowball.md)

### 188. Eventbridge

**Resources**: 13

- Replay [R]
- Connection [CRUD]
- Events [C]
- Partner_events [C]
- Rule [CRD]
- Permission [C]
- Archive [CRUD]
- Api_destination [CRUD]
- Event_bus [CRUD]
- Partner_event_source [CRD]
- Endpoint [CRUD]
- Targets [C]
- Event_source [R]

📖 [Full eventbridge documentation](services/eventbridge.md)

### 189. Auto_scaling_plans

**Resources**: 4

- Scaling_plans [R]
- Scaling_plan_resources [R]
- Scaling_plan_resource_forecast_data [R]
- Scaling_plan [CUD]

📖 [Full auto_scaling_plans documentation](services/auto_scaling_plans.md)

### 190. Directory_service

**Resources**: 31

- Settings [RU]
- Directory [CD]
- Directory_setup [U]
- Hybrid_ad_update [R]
- Microsoft_ad [C]
- Alias [C]
- Trusts [R]
- Ad_assessment [RD]
- Client_authentication_settings [R]
- Domain_controllers [R]
- Ca_enrollment_policy [R]
- Hybrid_ad [CU]
- Computer [C]
- Snapshot [CD]
- Event_topics [R]
- Directories [R]
- Update_directory [R]
- Radius [U]
- Log_subscription [CD]
- Number_of_domain_controllers [U]
- Directory_data_access [R]
- Trust [CUD]
- Conditional_forwarders [R]
- Ldaps_settings [R]
- Regions [R]
- Snapshot_limits [R]
- Directory_limits [R]
- Shared_directories [R]
- Snapshots [R]
- Conditional_forwarder [CUD]
- Certificate [R]

📖 [Full directory_service documentation](services/directory_service.md)

### 191. Mediapackage

**Resources**: 6

- Origin_endpoint [CRUD]
- Channel [CRUD]
- Harvest_job [CR]
- Packaging_configuration [CRD]
- Packaging_group [CRUD]
- Asset [CRD]

📖 [Full mediapackage documentation](services/mediapackage.md)

### 192. Ssm_quicksetup

**Resources**: 4

- Configuration_definition [U]
- Configuration_manager [CRUD]
- Service_settings [RU]
- Configuration [R]

📖 [Full ssm_quicksetup documentation](services/ssm_quicksetup.md)

### 193. S3_control

**Resources**: 33

- Access_grants_location [CRUD]
- Access_point_policy_for_object_lambda [CRD]
- Access_grants_instance [CRD]
- Multi_region_access_point_policy [CR]
- Storage_lens_group [CRUD]
- Bucket_policy [CRD]
- Access_point_policy [CRD]
- Bucket_replication [CRD]
- Job_tagging [CRD]
- Access_grant [CRD]
- Bucket_tagging [CRD]
- Access_point_for_object_lambda [CRD]
- Access_point [CRD]
- Multi_region_access_point_policy_status [R]
- Data_access [R]
- Bucket_lifecycle_configuration [CRD]
- Access_grants_instance_resource_policy [CRD]
- Bucket [CRD]
- Public_access_block [CRD]
- Storage_lens_configuration_tagging [CRD]
- Multi_region_access_point_routes [R]
- Access_point_policy_status [R]
- Bucket_versioning [CR]
- Storage_lens_configuration [CRD]
- Access_point_configuration_for_object_lambda [CR]
- Multi_region_access_point [CRD]
- Multi_region_access_point_operation [R]
- Job_status [U]
- Access_point_scope [CRD]
- Access_point_policy_status_for_object_lambda [R]
- Job_priority [U]
- Access_grants_instance_for_prefix [R]
- Job [CR]

📖 [Full s3_control documentation](services/s3_control.md)

### 194. Codecatalyst

**Resources**: 1

- User_details [R]

📖 [Full codecatalyst documentation](services/codecatalyst.md)

### 195. Notificationscontacts

**Resources**: 0


📖 [Full notificationscontacts documentation](services/notificationscontacts.md)

### 196. Mpa

**Resources**: 2

- Resource_policy [R]
- Policy_version [R]

📖 [Full mpa documentation](services/mpa.md)

### 197. Ec2_instance_connect

**Resources**: 0


📖 [Full ec2_instance_connect documentation](services/ec2_instance_connect.md)

### 198. Sagemaker_geospatial

**Resources**: 0


📖 [Full sagemaker_geospatial documentation](services/sagemaker_geospatial.md)

### 199. Notifications

**Resources**: 0


📖 [Full notifications documentation](services/notifications.md)

### 200. Securitylake

**Resources**: 1

- Data_lake_exception_subscription [CRUD]

📖 [Full securitylake documentation](services/securitylake.md)

### 201. Networkmonitor

**Resources**: 0


📖 [Full networkmonitor documentation](services/networkmonitor.md)

### 202. Codeconnections

**Resources**: 8

- Sync_blocker_summary [R]
- Resource_sync_status [R]
- Sync_configuration [CRUD]
- Repository_link [CRUD]
- Sync_blocker [U]
- Connection [CRD]
- Repository_sync_status [R]
- Host [CRUD]

📖 [Full codeconnections documentation](services/codeconnections.md)

### 203. App_mesh

**Resources**: 0


📖 [Full app_mesh documentation](services/app_mesh.md)

### 204. Workspaces_thin_client

**Resources**: 3

- Software_set [RU]
- Device [RUD]
- Environment [CRUD]

📖 [Full workspaces_thin_client documentation](services/workspaces_thin_client.md)

### 205. Finspace_data

**Resources**: 8

- Working_location [R]
- Dataset [CRUD]
- Permission_group [CRUD]
- Data_view [CR]
- External_data_view_access_details [R]
- User [CRU]
- Changeset [CRU]
- Programmatic_access_credentials [R]

📖 [Full finspace_data documentation](services/finspace_data.md)

### 206. Compute_optimizer

**Resources**: 17

- Recommendation_preferences [CRD]
- Ecs_service_recommendation_projected_metrics [R]
- Enrollment_status [RU]
- License_recommendations [R]
- Lambda_function_recommendations [R]
- Effective_recommendation_preferences [R]
- Ec2_recommendation_projected_metrics [R]
- Idle_recommendations [R]
- Rds_database_recommendation_projected_metrics [R]
- Rds_database_recommendations [R]
- Enrollment_statuses_for_organization [R]
- Auto_scaling_group_recommendations [R]
- Ec2_instance_recommendations [R]
- Ebs_volume_recommendations [R]
- Recommendation_export_jobs [R]
- Ecs_service_recommendations [R]
- Recommendation_summaries [R]

📖 [Full compute_optimizer documentation](services/compute_optimizer.md)

### 207. Secrets_manager

**Resources**: 5

- Random_password [R]
- Secret_value [CR]
- Secret [CRUD]
- Resource_policy [CRD]
- Secret_version_stage [U]

📖 [Full secrets_manager documentation](services/secrets_manager.md)

### 208. Mediastore

**Resources**: 5

- Metric_policy [CRD]
- Container [CRD]
- Cors_policy [CRD]
- Lifecycle_policy [CRD]
- Container_policy [CRD]

📖 [Full mediastore documentation](services/mediastore.md)

### 209. Ecs

**Resources**: 23

- Cluster_settings [U]
- Tasks [R]
- Task_set [CUD]
- Capacity_providers [R]
- Service_deployments [R]
- Clusters [R]
- Cluster [CUD]
- Task_protection [RU]
- Task_definition [R]
- Task_sets [R]
- Capacity_provider [CUD]
- Task_definitions [D]
- Account_setting [CD]
- Service_revisions [R]
- Service_primary_task_set [U]
- Container_agent [U]
- Attributes [CD]
- Account_setting_default [C]
- Service [CUD]
- Container_instances_state [U]
- Container_instances [R]
- Cluster_capacity_providers [C]
- Services [R]

📖 [Full ecs documentation](services/ecs.md)

### 210. Vpc_lattice

**Resources**: 2

- Auth_policy [CRD]
- Resource_policy [CRD]

📖 [Full vpc_lattice documentation](services/vpc_lattice.md)

### 211. Auto_scaling

**Resources**: 31

- Load_balancer_target_groups [R]
- Auto_scaling_instances [R]
- Adjustment_types [R]
- Warm_pool [CRD]
- Scheduled_action [D]
- Tags [RD]
- Launch_configurations [R]
- Scaling_policy [C]
- Scaling_process_types [R]
- Load_balancers [R]
- Traffic_sources [R]
- Auto_scaling_group [CUD]
- Termination_policy_types [R]
- Scheduled_actions [R]
- Lifecycle_hook [CD]
- Notification_configuration [CD]
- Instance_refreshes [R]
- Auto_scaling_notification_types [R]
- Policy [D]
- Scheduled_update_group_action [C]
- Launch_configuration [CD]
- Lifecycle_hook_types [R]
- Notification_configurations [R]
- Lifecycle_hooks [R]
- Account_limits [R]
- Auto_scaling_groups [R]
- Metric_collection_types [R]
- Policies [R]
- Scaling_activities [R]
- Predictive_scaling_forecast [R]
- Or_update_tags [C]

📖 [Full auto_scaling documentation](services/auto_scaling.md)

### 212. Resource_groups

**Resources**: 6

- Account_settings [RU]
- Group_query [RU]
- Tag_sync_task [R]
- Group_configuration [CR]
- Group [CRUD]
- Tags [R]

📖 [Full resource_groups documentation](services/resource_groups.md)

### 213. Eks

**Resources**: 18

- Insight [R]
- Cluster_version [U]
- Nodegroup_version [U]
- Addon [CRUD]
- Cluster [CRD]
- Pod_identity_association [CRUD]
- Insights_refresh [R]
- Eks_anywhere_subscription [CRUD]
- Identity_provider_config [R]
- Addon_configuration [R]
- Cluster_versions [R]
- Update [R]
- Cluster_config [U]
- Fargate_profile [CRD]
- Access_entry [CRUD]
- Nodegroup [CRD]
- Addon_versions [R]
- Nodegroup_config [U]

📖 [Full eks documentation](services/eks.md)

### 214. Marketplace_entitlement_service

**Resources**: 1

- Entitlements [R]

📖 [Full marketplace_entitlement_service documentation](services/marketplace_entitlement_service.md)

### 215. Database_migration_service

**Resources**: 59

- Certificate [D]
- Fleet_advisor_databases [RD]
- Account_attributes [R]
- Fleet_advisor_lsa_analysis [R]
- Replication_instance_task_logs [R]
- Recommendations [R]
- Replication_subnet_groups [R]
- Replication_subnet_group [CD]
- Refresh_schemas_status [R]
- Replication_config [CD]
- Replication_task_assessment_run [D]
- Data_migrations [R]
- Recommendation_limitations [R]
- Data_providers [R]
- Metadata_model_imports [R]
- Replications [R]
- Fleet_advisor_collectors [R]
- Metadata_model_exports_as_script [R]
- Extension_pack_associations [R]
- Endpoint_settings [R]
- Endpoints [R]
- Metadata_model_conversions [R]
- Orderable_replication_instances [R]
- Pending_maintenance_actions [R]
- Metadata_model_assessments [R]
- Connections [R]
- Schemas [R]
- Events [R]
- Instance_profiles [R]
- Endpoint [CD]
- Endpoint_types [R]
- Instance_profile [CD]
- Connection [D]
- Replication_task_assessment_results [R]
- Fleet_advisor_schemas [R]
- Replication_tasks [R]
- Event_subscription [CD]
- Replication_instances [R]
- Migration_projects [R]
- Replication_table_statistics [R]
- Replication_task_assessment_runs [R]
- Certificates [R]
- Replication_task [CD]
- Replication_configs [R]
- Migration_project [CD]
- Subscriptions_to_event_bridge [U]
- Conversion_configuration [R]
- Event_categories [R]
- Engine_versions [R]
- Fleet_advisor_schema_object_summary [R]
- Replication_task_individual_assessments [R]
- Metadata_model_exports_to_target [R]
- Data_provider [CD]
- Fleet_advisor_collector [CD]
- Applicable_individual_assessments [R]
- Data_migration [CD]
- Table_statistics [R]
- Replication_instance [CD]
- Event_subscriptions [R]

📖 [Full database_migration_service documentation](services/database_migration_service.md)

### 216. Security_ir

**Resources**: 0


📖 [Full security_ir documentation](services/security_ir.md)

### 217. Inspector_scan

**Resources**: 0


📖 [Full inspector_scan documentation](services/inspector_scan.md)

### 218. Global_accelerator

**Resources**: 9

- Accelerator_attributes [RU]
- Listener [CRUD]
- Accelerator [CRUD]
- Custom_routing_accelerator_attributes [RU]
- Custom_routing_endpoint_group [CRD]
- Custom_routing_accelerator [CRUD]
- Endpoint_group [CRUD]
- Cross_account_attachment [CRUD]
- Custom_routing_listener [CRUD]

📖 [Full global_accelerator documentation](services/global_accelerator.md)

### 219. Kinesis_analytics

**Resources**: 16

- Application_vpc_configuration [D]
- Application_reference_data_source [D]
- Application_operation [R]
- Application_cloud_watch_logging_option [D]
- Application_maintenance_configuration [U]
- Application_output [D]
- Application_snapshot [CRD]
- Application_presigned_url [C]
- Application_input_processing_configuration [D]
- Application_version [R]
- Application [CRUD]
- Application_cloud_watch_logging_option [D]
- Application_output [D]
- Application_reference_data_source [D]
- Application_input_processing_configuration [D]
- Application [CRUD]

📖 [Full kinesis_analytics documentation](services/kinesis_analytics.md)

### 220. Neptunedata

**Resources**: 14

- Loader_job_status [R]
- Ml_data_processing_job [R]
- Gremlin_query_status [R]
- Sparql_statistics [RD]
- Propertygraph_summary [R]
- Propertygraph_statistics [RD]
- Ml_model_training_job [R]
- Propertygraph_stream [R]
- Open_cypher_query_status [R]
- Ml_model_transform_job [R]
- Sparql_stream [R]
- Rdf_graph_summary [R]
- Ml_endpoint [CRD]
- Engine_status [R]

📖 [Full neptunedata documentation](services/neptunedata.md)

### 221. Swf

**Resources**: 5

- Activity_type [RD]
- Workflow_execution_history [R]
- Workflow_execution [R]
- Workflow_type [RD]
- Domain [R]

📖 [Full swf documentation](services/swf.md)

### 222. Cloudwatch_logs

**Resources**: 43

- Metric_filter [CD]
- Log_stream [CD]
- Index_policies [R]
- Log_group_fields [R]
- Deliveries [R]
- Query_definitions [R]
- Resource_policies [R]
- Delivery_configuration [U]
- Delivery_destinations [R]
- Log_streams [R]
- Transformer [CRD]
- Log_anomaly_detector [CRUD]
- Account_policy [CD]
- Export_tasks [R]
- Field_indexes [R]
- Integration [CRD]
- Metric_filters [R]
- Delivery [CRD]
- Delivery_destination [CRD]
- Queries [R]
- Log_events [CR]
- Export_task [C]
- Configuration_templates [R]
- Log_group [CD]
- Retention_policy [CD]
- Destination_policy [C]
- Subscription_filters [R]
- Subscription_filter [CD]
- Log_object [R]
- Index_policy [CD]
- Log_record [R]
- Delivery_sources [R]
- Delivery_source [CRD]
- Anomaly [U]
- Resource_policy [CD]
- Destinations [R]
- Query_results [R]
- Data_protection_policy [CRD]
- Account_policies [R]
- Query_definition [CD]
- Log_groups [R]
- Destination [CD]
- Delivery_destination_policy [CRD]

📖 [Full cloudwatch_logs documentation](services/cloudwatch_logs.md)

### 223. Connect

**Resources**: 81

- Routing_profile_name [U]
- User_phone_config [U]
- Queue_outbound_caller_config [U]
- Routing_profile_default_outbound_queue [U]
- Contact_flow_version [CD]
- User_hierarchy_group [CRD]
- Contact_evaluation [RUD]
- Contact_flow [CRD]
- User_security_profiles [U]
- Attached_file [RD]
- Evaluation_form [CRUD]
- Queue_name [U]
- Quick_connect_config [U]
- Agent_status [CRU]
- Contact_flow_name [U]
- User_proficiencies [U]
- Hours_of_operation_override [CRUD]
- Rule [CRUD]
- Use_case [CD]
- Metric_data_v2 [R]
- Predefined_attribute [CRUD]
- User_hierarchy [U]
- View_content [U]
- Current_metric_data [R]
- Hours_of_operation [CRUD]
- Participant_authentication [U]
- Contact_flow_module [CRD]
- Routing_profile [CRD]
- Contact_flow_metadata [U]
- Routing_profile_agent_availability_timer [U]
- Contact_flow_module_content [U]
- Participant_role_config [U]
- Quick_connect [CRD]
- User_routing_profile [U]
- Federation_token [R]
- View_version [CD]
- Contact_attributes [RU]
- Phone_number_metadata [U]
- Contact_routing_data [U]
- Contact_schedule [U]
- Queue_outbound_email_config [U]
- Metric_data [R]
- User_hierarchy_group_name [U]
- Phone_number [RU]
- Instance [CRD]
- Prompt_file [R]
- Push_notification_registration [CD]
- Task_template [CRUD]
- View [CRD]
- Authentication_profile [RU]
- Flow_association [R]
- User_identity_info [U]
- Email_address [CRD]
- Contact [CRU]
- Contact_flow_content [U]
- Quick_connect_name [U]
- Routing_profile_concurrency [U]
- Participant [C]
- Queue_status [U]
- Prompt [CRUD]
- Current_user_data [R]
- View_metadata [U]
- Contact_metrics [R]
- Queue [CRD]
- Security_profile [CRUD]
- Traffic_distribution [RU]
- Contact_flow_module_metadata [U]
- Routing_profile_queues [U]
- Persistent_contact_association [C]
- User [CRD]
- Vocabulary [CRD]
- User_status [C]
- Instance_attribute [RU]
- Instance_storage_config [RU]
- Traffic_distribution_group [CRD]
- Effective_hours_of_operations [R]
- Integration_association [CD]
- Email_address_metadata [U]
- Queue_hours_of_operation [U]
- User_hierarchy_structure [RU]
- Queue_max_contacts [U]

📖 [Full connect documentation](services/connect.md)

### 224. Glue

**Resources**: 91

- Schema [CRUD]
- Mapping [R]
- Ml_transforms [R]
- Databases [R]
- Resource_policy [CRD]
- Table_optimizer [CRUD]
- Entity [R]
- Unfiltered_partition_metadata [R]
- Job_run [R]
- Data_catalog_encryption_settings [CR]
- Integration_resource_property [CRU]
- Classifier [CRUD]
- Crawler [CRUD]
- Ml_transform [CRUD]
- Dataflow_graph [R]
- Table_version [RD]
- Partition_indexes [R]
- Column_statistics_for_partition [RUD]
- Script [C]
- Security_configurations [R]
- Partition [CRUD]
- Column_statistics_for_table [RUD]
- Data_quality_model_result [R]
- Unfiltered_table_metadata [R]
- Connection [CRUD]
- Session [CRD]
- Triggers [R]
- Crawler_schedule [U]
- Database [CRUD]
- Integration [CD]
- Partition_index [CD]
- Partitions [R]
- Crawlers [R]
- Schema_by_definition [R]
- Connection_type [R]
- Blueprint_runs [R]
- Job_bookmark [R]
- Job_from_source_control [U]
- Security_configuration [CRD]
- Glue_identity_center_configuration [CRUD]
- Catalog [CRUD]
- Dev_endpoints [R]
- Ml_task_runs [R]
- Dev_endpoint [CRUD]
- Workflow_run_properties [CR]
- Column_statistics_task_runs [R]
- Job [CRUD]
- Jobs [R]
- Table_versions [R]
- Workflow [CRUD]
- Data_quality_ruleset [CRUD]
- Tags [R]
- Schema_versions [D]
- User_defined_functions [R]
- Workflow_run [R]
- Column_statistics_task_settings [CRUD]
- Schema_versions_diff [R]
- Data_quality_result [R]
- Data_quality_ruleset_evaluation_run [R]
- Classifiers [R]
- Workflow_runs [R]
- Job_runs [R]
- Blueprint [CRUD]
- Data_quality_profile_annotation [C]
- Crawler_metrics [R]
- Source_control_from_job [U]
- Plan [R]
- Connections [R]
- Inbound_integrations [R]
- Column_statistics_task_run [R]
- Usage_profile [CRUD]
- Integrations [R]
- Blueprint_run [R]
- User_defined_function [CRUD]
- Registry [CRUD]
- Ml_task_run [R]
- Statement [R]
- Schema_version_metadata [C]
- Integration_table_properties [CRUD]
- Data_quality_model [R]
- Trigger [CRUD]
- Entity_records [R]
- Resource_policies [R]
- Schema_version [R]
- Table [CRUD]
- Catalogs [R]
- Data_quality_rule_recommendation_run [R]
- Unfiltered_partitions_metadata [R]
- Catalog_import_status [R]
- Custom_entity_type [CRD]
- Tables [R]

📖 [Full glue documentation](services/glue.md)

### 225. Cognito_identity_provider

**Resources**: 26

- User_import_job [CR]
- Log_delivery_configuration [R]
- User_attribute_verification_code [R]
- Identity_provider [CRUD]
- Csv_header [R]
- Risk_configuration [R]
- Ui_customization [R]
- Signing_certificate [R]
- Web_authn_credential [D]
- User_pool_client [CRUD]
- Terms [CRUD]
- User_attributes [UD]
- Device_status [U]
- Group [CRUD]
- User_pool [CRUD]
- User_pool_domain [CRUD]
- User_pool_mfa_config [R]
- Managed_login_branding [CRUD]
- Tokens_from_refresh_token [R]
- Managed_login_branding_by_client [R]
- User [RD]
- User_auth_factors [R]
- Resource_server [CRUD]
- Auth_event_feedback [U]
- Identity_provider_by_identifier [R]
- Device [R]

📖 [Full cognito_identity_provider documentation](services/cognito_identity_provider.md)

### 226. Cloudwatch_events

**Resources**: 12

- Connection [CRUD]
- Api_destination [CRUD]
- Rule [CRD]
- Partner_events [C]
- Replay [R]
- Event_bus [CRD]
- Targets [C]
- Partner_event_source [CRD]
- Event_source [R]
- Archive [CRUD]
- Permission [C]
- Events [C]

📖 [Full cloudwatch_events documentation](services/cloudwatch_events.md)

### 227. Cost_explorer

**Resources**: 27

- Anomaly_monitor [CUD]
- Cost_and_usage_comparisons [R]
- Savings_plan_purchase_recommendation_details [R]
- Commitment_purchase_analysis [R]
- Anomaly_monitors [R]
- Savings_plans_utilization [R]
- Tags [R]
- Cost_and_usage_with_resources [R]
- Rightsizing_recommendation [R]
- Cost_allocation_tags_status [U]
- Anomalies [R]
- Reservation_purchase_recommendation [R]
- Dimension_values [R]
- Cost_category_definition [CRUD]
- Savings_plans_purchase_recommendation [R]
- Anomaly_subscriptions [R]
- Approximate_usage_records [R]
- Cost_forecast [R]
- Cost_categories [R]
- Anomaly_subscription [CUD]
- Savings_plans_utilization_details [R]
- Cost_comparison_drivers [R]
- Usage_forecast [R]
- Reservation_coverage [R]
- Cost_and_usage [R]
- Savings_plans_coverage [R]
- Reservation_utilization [R]

📖 [Full cost_explorer documentation](services/cost_explorer.md)

### 228. Network_firewall

**Resources**: 20

- Availability_zone_change_protection [U]
- Firewall_description [U]
- Vpc_endpoint_association [CRD]
- Firewall [CRD]
- Resource_policy [CRD]
- Tls_inspection_configuration [CRUD]
- Rule_group_metadata [R]
- Network_firewall_transit_gateway_attachment [D]
- Rule_group_summary [R]
- Rule_group [CRUD]
- Analysis_report_results [R]
- Firewall_policy [CRUD]
- Logging_configuration [RU]
- Flow_operation [R]
- Firewall_encryption_configuration [U]
- Firewall_policy_change_protection [U]
- Subnet_change_protection [U]
- Firewall_analysis_settings [U]
- Firewall_metadata [R]
- Firewall_delete_protection [U]

📖 [Full network_firewall documentation](services/network_firewall.md)

### 229. Firehose

**Resources**: 4

- Destination [U]
- Record [C]
- Record_batch [C]
- Delivery_stream [CRD]

📖 [Full firehose documentation](services/firehose.md)

### 230. Transfer

**Resources**: 5

- Security_policy [R]
- Execution [R]
- Host_key [RUD]
- Access [CRUD]
- Ssh_public_key [D]

📖 [Full transfer documentation](services/transfer.md)

### 231. Marketplace_metering

**Resources**: 0


📖 [Full marketplace_metering documentation](services/marketplace_metering.md)

### 232. Rbin

**Resources**: 1

- Rule [CRUD]

📖 [Full rbin documentation](services/rbin.md)

### 233. Timestream_influxdb

**Resources**: 0


📖 [Full timestream_influxdb documentation](services/timestream_influxdb.md)

### 234. Iotanalytics

**Resources**: 6

- Datastore [CRUD]
- Dataset_content [CRD]
- Logging_options [CR]
- Pipeline [CRUD]
- Dataset [CRUD]
- Channel [CRUD]

📖 [Full iotanalytics documentation](services/iotanalytics.md)

### 235. Ivs

**Resources**: 8

- Metadata [C]
- Recording_configuration [CRD]
- Playback_key_pair [RD]
- Stream [R]
- Stream_session [R]
- Playback_restriction_policy [CRUD]
- Channel [CRUD]
- Stream_key [CRD]

📖 [Full ivs documentation](services/ivs.md)

### 236. Kafka

**Resources**: 21

- Cluster_v2 [CR]
- Security [U]
- Compatible_kafka_versions [R]
- Monitoring [U]
- Configuration [CRUD]
- Configuration_revision [R]
- Replicator [CRD]
- Bootstrap_brokers [R]
- Cluster [CRD]
- Cluster_operation [R]
- Broker_count [U]
- Cluster_configuration [U]
- Cluster_kafka_version [U]
- Connectivity [U]
- Replication_info [U]
- Cluster_policy [CRD]
- Vpc_connection [CRD]
- Cluster_operation_v2 [R]
- Broker_type [U]
- Broker_storage [U]
- Storage [U]

📖 [Full kafka documentation](services/kafka.md)

### 237. Sesv2

**Resources**: 50

- Blacklist_reports [R]
- Account_dedicated_ip_warmup_attributes [C]
- Dedicated_ip_in_pool [C]
- Import_job [CR]
- Export_job [CR]
- Suppressed_destination [CRD]
- Account_vdm_attributes [C]
- Configuration_set_sending_options [C]
- Configuration_set_suppression_options [C]
- Reputation_entity [R]
- Deliverability_dashboard_options [R]
- Contact [CRUD]
- Configuration_set_event_destinations [R]
- Domain_statistics_report [R]
- Account_sending_attributes [C]
- Contact_list [CRUD]
- Multi_region_endpoint [CRD]
- Configuration_set_archiving_options [C]
- Dedicated_ip_pool_scaling_attributes [C]
- Email_identity_dkim_attributes [C]
- Email_identity_feedback_attributes [C]
- Configuration_set_reputation_options [C]
- Reputation_entity_customer_managed_status [U]
- Dedicated_ip [R]
- Email_template [CRUD]
- Email_identity_dkim_signing_attributes [C]
- Configuration_set_event_destination [CUD]
- Configuration_set_vdm_options [C]
- Reputation_entity_policy [U]
- Deliverability_dashboard_option [C]
- Dedicated_ip_warmup_attributes [C]
- Email_identity_policies [R]
- Domain_deliverability_campaign [R]
- Tenant_resource_association [CD]
- Deliverability_test_report [CR]
- Account [R]
- Dedicated_ips [R]
- Account_suppression_attributes [C]
- Configuration_set_tracking_options [C]
- Email_identity_configuration_set_attributes [C]
- Email_identity [CRD]
- Configuration_set [CRD]
- Tenant [CRD]
- Email_identity_mail_from_attributes [C]
- Configuration_set_delivery_options [C]
- Email_identity_policy [CUD]
- Account_details [C]
- Message_insights [R]
- Custom_verification_email_template [CRUD]
- Dedicated_ip_pool [CRD]

📖 [Full sesv2 documentation](services/sesv2.md)

### 238. Kendra

**Resources**: 12

- Index [CRUD]
- Experience [CRUD]
- Query_suggestions_block_list [CRUD]
- Query_suggestions_config [RU]
- Faq [CRD]
- Snapshots [R]
- Query_suggestions [R]
- Access_control_configuration [CRUD]
- Thesaurus [CRUD]
- Data_source [CRUD]
- Principal_mapping [CRD]
- Featured_results_set [CRU]

📖 [Full kendra documentation](services/kendra.md)

### 239. Sagemaker_edge

**Resources**: 2

- Deployments [R]
- Device_registration [R]

📖 [Full sagemaker_edge documentation](services/sagemaker_edge.md)

### 240. Launch_wizard

**Resources**: 0


📖 [Full launch_wizard documentation](services/launch_wizard.md)

### 241. Securityhub

**Resources**: 36

- Ticket_v2 [C]
- Products_v2 [R]
- Security_control [U]
- Invitations_count [R]
- Findings [RU]
- Resources_v2 [R]
- Action_target [CUD]
- Action_targets [R]
- Findings_v2 [R]
- Standards_control [U]
- Products [R]
- Members [CRD]
- Administrator_account [R]
- Insights [R]
- Aggregator_v2 [CRUD]
- Standards [R]
- Master_account [R]
- Enabled_standards [R]
- Connector_v2 [CRUD]
- Automation_rule [C]
- Standards_controls [R]
- Organization_configuration [RU]
- Finding_aggregator [CRUD]
- Invitations [D]
- Finding_statistics_v2 [R]
- Security_control_definition [R]
- Finding_history [R]
- Automation_rule_v2 [CRUD]
- Configuration_policy [CRUD]
- Insight_results [R]
- Configuration_policy_association [R]
- Security_hub_configuration [U]
- Resources_statistics_v2 [R]
- Insight [CUD]
- Security_hub_v2 [R]
- Hub [R]

📖 [Full securityhub documentation](services/securityhub.md)

### 242. Finspace

**Resources**: 14

- Kx_connection_string [R]
- Kx_database [CRUD]
- Kx_volume [CRUD]
- Kx_cluster [CRD]
- Kx_dataview [CRUD]
- Kx_cluster_code_configuration [U]
- Kx_changeset [CR]
- Kx_environment_network [U]
- Environment [CRUD]
- Kx_cluster_databases [U]
- Kx_scaling_group [CRD]
- Kx_cluster_node [D]
- Kx_user [CRUD]
- Kx_environment [CRUD]

📖 [Full finspace documentation](services/finspace.md)

### 243. Keyspacesstreams

**Resources**: 3

- Shard_iterator [R]
- Records [R]
- Stream [R]

📖 [Full keyspacesstreams documentation](services/keyspacesstreams.md)

### 244. Cleanroomsml

**Resources**: 0


📖 [Full cleanroomsml documentation](services/cleanroomsml.md)

### 245. Transcribe_streaming

**Resources**: 1

- Medical_scribe_stream [R]

📖 [Full transcribe_streaming documentation](services/transcribe_streaming.md)

### 246. Aiops

**Resources**: 0


📖 [Full aiops documentation](services/aiops.md)

### 247. Service_catalog

**Resources**: 20

- Product_as_admin [R]
- Provisioning_artifact [CRUD]
- Aws_organizations_access_status [R]
- Portfolio_share [CUD]
- Product_view [R]
- Portfolio_shares [R]
- Record [R]
- Product [CRUD]
- Provisioned_product_plan [CRD]
- Provisioned_product_outputs [R]
- Tag_option [CRUD]
- Portfolio [CRUD]
- Service_action [CRUD]
- Copy_product_status [R]
- Constraint [CRUD]
- Provisioned_product [RU]
- Service_action_execution_parameters [R]
- Portfolio_share_status [R]
- Provisioned_product_properties [U]
- Provisioning_parameters [R]

📖 [Full service_catalog documentation](services/service_catalog.md)

### 248. Databrew

**Resources**: 10

- Project [CRUD]
- Job_run [R]
- Job [RD]
- Recipe_version [D]
- Schedule [CRUD]
- Profile_job [CU]
- Dataset [CRUD]
- Recipe [CRU]
- Recipe_job [CU]
- Ruleset [CRUD]

📖 [Full databrew documentation](services/databrew.md)

### 249. Codecommit

**Resources**: 36

- Comments_for_compared_commit [R]
- Approval_rule_template_description [U]
- Comment_content [D]
- Pull_request [CR]
- Comments_for_pull_request [R]
- Pull_request_description [U]
- Repository_description [U]
- Blob [R]
- Merge_conflicts [R]
- Pull_request_events [R]
- Repository_encryption_key [U]
- Repository [CRD]
- Comment_reactions [R]
- Merge_options [R]
- Approval_rule_template_name [U]
- Pull_request_title [U]
- Pull_request_approval_rule_content [U]
- Repository_name [U]
- Comment [RU]
- Commit [CR]
- Differences [R]
- File [CRD]
- Pull_request_override_state [R]
- Pull_request_status [U]
- Branch [CRD]
- Approval_rule_template [CRD]
- Default_branch [U]
- Pull_request_approval_states [R]
- Repository_triggers [CR]
- Pull_request_approval_rule [CD]
- Approval_rule_template_content [U]
- Merge_commit [R]
- Pull_request_approval_state [U]
- Unreferenced_merge_commit [C]
- Comment_reaction [C]
- Folder [R]

📖 [Full codecommit documentation](services/codecommit.md)

### 250. Resource_explorer_2

**Resources**: 7

- Service_index [R]
- Index [R]
- Service_view [R]
- Resource_explorer_setup [CRD]
- Account_level_service_configuration [R]
- Default_view [R]
- Managed_view [R]

📖 [Full resource_explorer_2 documentation](services/resource_explorer_2.md)

### 251. Acm_pca

**Resources**: 7

- Certificate_authority_audit_report [CR]
- Certificate_authority_certificate [R]
- Certificate_authority_csr [R]
- Certificate_authority [CRUD]
- Permission [CD]
- Certificate [R]
- Policy [CRD]

📖 [Full acm_pca documentation](services/acm_pca.md)

### 252. Payment_cryptography

**Resources**: 5

- Certificate_signing_request [R]
- Parameters_for_export [R]
- Parameters_for_import [R]
- Public_key_certificate [R]
- Default_key_replication_regions [R]

📖 [Full payment_cryptography documentation](services/payment_cryptography.md)

### 253. Mq

**Resources**: 7

- Broker_engine_types [R]
- Broker [CRUD]
- Configuration [CRUD]
- User [CRUD]
- Configuration_revision [R]
- Tags [CD]
- Broker_instance_options [R]

📖 [Full mq documentation](services/mq.md)

### 254. Api_gateway

**Resources**: 48

- Integration_response [CRUD]
- Tags [R]
- Deployments [R]
- Model [CRUD]
- Method_response [CRUD]
- Gateway_responses [R]
- Domain_name [CRUD]
- Export [R]
- Request_validator [CRUD]
- Rest_apis [R]
- Sdk_type [R]
- Usage_plan_key [CRD]
- Sdk_types [R]
- Domain_name_access_association [CD]
- Stages [R]
- Usage_plans [R]
- Request_validators [R]
- Authorizers [R]
- Documentation_parts [R]
- Domain_names [R]
- Deployment [CRUD]
- Authorizer [CRUD]
- Api_keys [R]
- Gateway_response [CRUD]
- Client_certificate [RUD]
- Documentation_part [CRUD]
- Usage_plan [CRUD]
- Integration [CRUD]
- Method [CRUD]
- Base_path_mappings [R]
- Rest_api [CRUD]
- Vpc_link [CRUD]
- Model_template [R]
- Models [R]
- Vpc_links [R]
- Resources [R]
- Api_key [CRUD]
- Stage [CRUD]
- Account [RU]
- Base_path_mapping [CRUD]
- Domain_name_access_associations [R]
- Documentation_version [CRUD]
- Usage_plan_keys [R]
- Resource [CRUD]
- Usage [RU]
- Documentation_versions [R]
- Client_certificates [R]
- Sdk [R]

📖 [Full api_gateway documentation](services/api_gateway.md)

### 255. Grafana

**Resources**: 0


📖 [Full grafana documentation](services/grafana.md)

### 256. Glacier

**Resources**: 8

- Vault [CRD]
- Vault_access_policy [RD]
- Archive [D]
- Data_retrieval_policy [R]
- Job [R]
- Job_output [R]
- Vault_notifications [RD]
- Vault_lock [R]

📖 [Full glacier documentation](services/glacier.md)

### 257. Bedrock

**Resources**: 0


📖 [Full bedrock documentation](services/bedrock.md)

### 258. S3tables

**Resources**: 0


📖 [Full s3tables documentation](services/s3tables.md)

### 259. Ivs_realtime

**Resources**: 9

- Participant [R]
- Ingest_configuration [CRUD]
- Public_key [RD]
- Participant_token [C]
- Stage_session [R]
- Stage [CRUD]
- Storage_configuration [CRD]
- Composition [R]
- Encoder_configuration [CRD]

📖 [Full ivs_realtime documentation](services/ivs_realtime.md)

### 260. Medialive

**Resources**: 27

- Partner_input [C]
- Event_bridge_rule_template_group [CRUD]
- Multiplex [CRUD]
- Input_device [RU]
- Input [CRUD]
- Input_security_group [CRUD]
- Node_registration_script [C]
- Network [CRUD]
- Event_bridge_rule_template [CRUD]
- Schedule [RD]
- Cloud_watch_alarm_template [CRUD]
- Node_state [U]
- Thumbnails [R]
- Cloud_watch_alarm_template_group [CRUD]
- Account_configuration [RU]
- Offering [R]
- Signal_map [CRD]
- Cluster [CRUD]
- Reservation [RUD]
- Channel_placement_group [CRUD]
- Node [CRUD]
- Channel [CRUD]
- Tags [CD]
- Sdi_source [CRUD]
- Multiplex_program [CRUD]
- Channel_class [U]
- Input_device_thumbnail [R]

📖 [Full medialive documentation](services/medialive.md)

### 261. Backupsearch

**Resources**: 0


📖 [Full backupsearch documentation](services/backupsearch.md)

### 262. Networkflowmonitor

**Resources**: 0


📖 [Full networkflowmonitor documentation](services/networkflowmonitor.md)

### 263. Elasticache

**Resources**: 30

- Serverless_caches [R]
- Serverless_cache_snapshots [R]
- Snapshot [CD]
- User_group [CD]
- Replication_group [CD]
- Cache_security_group [CD]
- Cache_security_groups [R]
- Cache_engine_versions [R]
- Cache_clusters [R]
- Serverless_cache [CD]
- User_groups [R]
- Serverless_cache_snapshot [CD]
- Global_replication_group [CD]
- Cache_cluster [CD]
- Cache_subnet_groups [R]
- Cache_parameters [R]
- User [CD]
- Reserved_cache_nodes [R]
- Service_updates [R]
- Replication_groups [R]
- Engine_default_parameters [R]
- Events [R]
- Global_replication_groups [R]
- Reserved_cache_nodes_offerings [R]
- Cache_subnet_group [CD]
- Users [R]
- Cache_parameter_group [CD]
- Cache_parameter_groups [R]
- Update_actions [R]
- Snapshots [R]

📖 [Full elasticache documentation](services/elasticache.md)

### 264. Fis

**Resources**: 8

- Safety_lever [R]
- Experiment [R]
- Experiment_target_account_configuration [R]
- Action [R]
- Experiment_template [CRUD]
- Safety_lever_state [U]
- Target_resource_type [R]
- Target_account_configuration [CRUD]

📖 [Full fis documentation](services/fis.md)

### 265. Cloudhsm

**Resources**: 10

- Hsm [CRD]
- Config [R]
- Luna_client [CRD]
- Hapg [CRD]
- Resource_policy [CRD]
- Hsm [CD]
- Clusters [R]
- Backup [D]
- Backups [R]
- Cluster [CD]

📖 [Full cloudhsm documentation](services/cloudhsm.md)

### 266. Cost_optimization_hub

**Resources**: 3

- Preferences [RU]
- Recommendation [R]
- Enrollment_status [U]

📖 [Full cost_optimization_hub documentation](services/cost_optimization_hub.md)

### 267. Synthetics

**Resources**: 6

- Canary [CRUD]
- Canaries_last_run [R]
- Runtime_versions [R]
- Canaries [R]
- Group [CRD]
- Canary_runs [R]

📖 [Full synthetics documentation](services/synthetics.md)

### 268. Rum

**Resources**: 1

- Rum_events [C]

📖 [Full rum documentation](services/rum.md)

### 269. Emr_containers

**Resources**: 6

- Managed_endpoint [CRD]
- Virtual_cluster [CRD]
- Managed_endpoint_session_credentials [R]
- Security_configuration [CR]
- Job_template [CRD]
- Job_run [R]

📖 [Full emr_containers documentation](services/emr_containers.md)

### 270. Sagemaker_a2i_runtime

**Resources**: 1

- Human_loop [RD]

📖 [Full sagemaker_a2i_runtime documentation](services/sagemaker_a2i_runtime.md)

### 271. Ssm_contacts

**Resources**: 7

- Contact_policy [CR]
- Contact_channel [CRUD]
- Rotation [CRUD]
- Rotation_override [CRD]
- Engagement [R]
- Page [R]
- Contact [CRUD]

📖 [Full ssm_contacts documentation](services/ssm_contacts.md)

### 272. Bcm_data_exports

**Resources**: 3

- Table [R]
- Execution [R]
- Export [CRUD]

📖 [Full bcm_data_exports documentation](services/bcm_data_exports.md)

### 273. Opensearch

**Resources**: 29

- Instance_type_limits [R]
- Packages [R]
- Direct_query_data_source [RUD]
- Outbound_connection [CD]
- Upgrade_history [R]
- Domains [R]
- Vpc_endpoints [R]
- Package_scope [U]
- Application [CRUD]
- Upgrade_status [R]
- Outbound_connections [R]
- Domain [CRD]
- Inbound_connections [R]
- Domain_maintenance_status [R]
- Package [CUD]
- Vpc_endpoint [CUD]
- Compatible_versions [R]
- Domain_auto_tunes [R]
- Reserved_instance_offerings [R]
- Domain_health [R]
- Inbound_connection [D]
- Domain_change_progress [R]
- Domain_nodes [R]
- Scheduled_action [U]
- Package_version_history [R]
- Dry_run_progress [R]
- Reserved_instances [R]
- Domain_config [RU]
- Data_source [RUD]

📖 [Full opensearch documentation](services/opensearch.md)

### 274. Dax

**Resources**: 9

- Default_parameters [R]
- Cluster [CUD]
- Parameters [R]
- Subnet_group [CUD]
- Events [R]
- Parameter_groups [R]
- Parameter_group [CUD]
- Clusters [R]
- Subnet_groups [R]

📖 [Full dax documentation](services/dax.md)

### 275. Neptune

**Resources**: 29

- Db_cluster_parameter_groups [R]
- Db_cluster_snapshot [CD]
- Db_cluster [CD]
- Event_categories [R]
- Db_cluster_snapshots [R]
- Db_parameter_group [CD]
- Db_cluster_parameter_group [CD]
- Db_cluster_endpoints [R]
- Db_subnet_group [CD]
- Engine_default_parameters [R]
- Orderable_db_instance_options [R]
- Db_cluster_snapshot_attributes [R]
- Db_cluster_endpoint [CD]
- Db_parameters [R]
- Db_instances [R]
- Engine_default_cluster_parameters [R]
- Db_engine_versions [R]
- Event_subscriptions [R]
- Global_cluster [CD]
- Events [R]
- Db_cluster_parameters [R]
- Valid_db_instance_modifications [R]
- Db_parameter_groups [R]
- Event_subscription [CD]
- Db_instance [CD]
- Global_clusters [R]
- Pending_maintenance_actions [R]
- Db_clusters [R]
- Db_subnet_groups [R]

📖 [Full neptune documentation](services/neptune.md)

### 276. Pricing

**Resources**: 4

- Products [R]
- Price_list_file_url [R]
- Services [R]
- Attribute_values [R]

📖 [Full pricing documentation](services/pricing.md)

### 277. Location

**Resources**: 0


📖 [Full location documentation](services/location.md)

### 278. Route53profiles

**Resources**: 3

- Profile_association [R]
- Profile_resource_association [RU]
- Profile [CRD]

📖 [Full route53profiles documentation](services/route53profiles.md)

### 279. Lambda

**Resources**: 1

- Account_settings [R]

📖 [Full lambda documentation](services/lambda.md)

### 280. Ivschat

**Resources**: 4

- Room [CRUD]
- Logging_configuration [CRUD]
- Message [D]
- Chat_token [C]

📖 [Full ivschat documentation](services/ivschat.md)

### 281. Billing

**Resources**: 2

- Resource_policy [R]
- Billing_view [CRUD]

📖 [Full billing documentation](services/billing.md)

### 282. Wisdom

**Resources**: 0


📖 [Full wisdom documentation](services/wisdom.md)

### 283. Schemas

**Resources**: 8

- Schema_version [D]
- Code_binding_source [R]
- Code_binding [CR]
- Registry [CRUD]
- Schema [CRUD]
- Discovered_schema [R]
- Resource_policy [CRD]
- Discoverer [CRUD]

📖 [Full schemas documentation](services/schemas.md)

### 284. Bedrock_agentcore_control

**Resources**: 1

- Token_vault [R]

📖 [Full bedrock_agentcore_control documentation](services/bedrock_agentcore_control.md)

### 285. Controlcatalog

**Resources**: 0


📖 [Full controlcatalog documentation](services/controlcatalog.md)

### 286. Cloudsearch

**Resources**: 14

- Expressions [R]
- Index_field [D]
- Scaling_parameters [RU]
- Suggester [D]
- Domains [R]
- Analysis_schemes [R]
- Index_fields [R]
- Service_access_policies [RU]
- Suggesters [R]
- Domain [CD]
- Expression [D]
- Analysis_scheme [D]
- Domain_endpoint_options [RU]
- Availability_options [RU]

📖 [Full cloudsearch documentation](services/cloudsearch.md)

### 287. Deadline

**Resources**: 3

- Queue_fleet_association [CRUD]
- Sessions_statistics_aggregation [R]
- Queue_limit_association [CRUD]

📖 [Full deadline documentation](services/deadline.md)

### 288. Managedblockchain

**Resources**: 5

- Node [CRUD]
- Network [CR]
- Accessor [CRD]
- Member [CRUD]
- Proposal [CR]

📖 [Full managedblockchain documentation](services/managedblockchain.md)

### 289. Amplify

**Resources**: 8

- Webhook [CRUD]
- Artifact_url [R]
- App [CRUD]
- Domain_association [CRUD]
- Backend_environment [CRD]
- Branch [CRUD]
- Deployment [C]
- Job [RD]

📖 [Full amplify documentation](services/amplify.md)

### 290. Iotsecuretunneling

**Resources**: 1

- Tunnel [R]

📖 [Full iotsecuretunneling documentation](services/iotsecuretunneling.md)

### 291. Connectcampaigns

**Resources**: 10

- Campaign_dialer_config [U]
- Campaign_outbound_call_config [U]
- Instance_onboarding_job [D]
- Connect_instance_config [RD]
- Dial_request_batch [C]
- Instance_onboarding_job_status [R]
- Campaign_name [U]
- Campaign [CRD]
- Campaign_state_batch [R]
- Campaign_state [R]

📖 [Full connectcampaigns documentation](services/connectcampaigns.md)

### 292. Kafkaconnect

**Resources**: 4

- Custom_plugin [CRD]
- Connector_operation [R]
- Connector [CRUD]
- Worker_configuration [CRD]

📖 [Full kafkaconnect documentation](services/kafkaconnect.md)

### 293. Mediaconvert

**Resources**: 8

- Jobs_query_results [R]
- Preset [CRUD]
- Endpoints [R]
- Policy [CRD]
- Job [CR]
- Queue [CRUD]
- Job_template [CRUD]
- Resource_share [C]

📖 [Full mediaconvert documentation](services/mediaconvert.md)

### 294. Data_pipeline

**Resources**: 4

- Objects [R]
- Pipeline [CD]
- Pipelines [R]
- Pipeline_definition [CR]

📖 [Full data_pipeline documentation](services/data_pipeline.md)

### 295. Codepipeline

**Resources**: 14

- Custom_action_type [CD]
- Webhook [CD]
- Action_type [RU]
- Pipeline_state [R]
- Third_party_job_details [R]
- Job_success_result [C]
- Job_failure_result [C]
- Third_party_job_success_result [C]
- Action_revision [C]
- Pipeline_execution [R]
- Third_party_job_failure_result [C]
- Pipeline [CRUD]
- Approval_result [C]
- Job_details [R]

📖 [Full codepipeline documentation](services/codepipeline.md)

### 296. Clouddirectory

**Resources**: 13

- Schema_as_json [R]
- Object_attributes [RU]
- Applied_schema_version [R]
- Index [C]
- Object_information [R]
- Schema_from_json [C]
- Directory [CRD]
- Schema [CUD]
- Typed_link_facet [CUD]
- Object [CD]
- Link_attributes [RU]
- Typed_link_facet_information [R]
- Facet [CRUD]

📖 [Full clouddirectory documentation](services/clouddirectory.md)

### 297. Amplifyuibuilder

**Resources**: 2

- Metadata_flag [C]
- Metadata [R]

📖 [Full amplifyuibuilder documentation](services/amplifyuibuilder.md)

### 298. Rtbfabric

**Resources**: 0


📖 [Full rtbfabric documentation](services/rtbfabric.md)

### 299. Memorydb

**Resources**: 22

- Multi_region_parameters [R]
- Reserved_nodes [R]
- Snapshot [CD]
- Events [R]
- Multi_region_parameter_groups [R]
- Subnet_group [CUD]
- Multi_region_cluster [CUD]
- Engine_versions [R]
- Cluster [CUD]
- Clusters [R]
- Reserved_nodes_offerings [R]
- Snapshots [R]
- Acl [CUD]
- Ac_ls [R]
- Service_updates [R]
- Parameter_groups [R]
- User [CUD]
- Users [R]
- Parameters [R]
- Subnet_groups [R]
- Parameter_group [CUD]
- Multi_region_clusters [R]

📖 [Full memorydb documentation](services/memorydb.md)

### 300. Iot

**Resources**: 64

- Verification_state_on_violation [C]
- Ota_update [CRD]
- Security_profile [CRUD]
- Behavior_model_training_summaries [R]
- Endpoint [R]
- Default_authorizer [R]
- Detect_mitigation_actions_task [R]
- Effective_policies [R]
- Mitigation_action [CRUD]
- Audit_task [R]
- Audit_finding [R]
- Billing_group [CRUD]
- Policy_version [CRD]
- Package [CRUD]
- Policy [CRD]
- Thing_group [CRUD]
- Topic_rule [CRD]
- Job_template [CRD]
- Stream [CRUD]
- Statistics [R]
- Dynamic_thing_group [CUD]
- Thing_groups_for_thing [U]
- Job_execution [RD]
- Keys_and_certificate [C]
- Cardinality [R]
- Indexing_configuration [RU]
- V2_logging_level [D]
- Audit_suppression [CRUD]
- Role_alias [CRUD]
- Certificate [RUD]
- Dimension [CRUD]
- V2_logging_options [R]
- Certificate_from_csr [C]
- Provisioning_template [CRUD]
- Command_execution [RD]
- Index [R]
- Registration_code [RD]
- Thing_registration_task [R]
- Logging_options [R]
- Command [CRUD]
- Event_configurations [RU]
- Fleet_metric [CRUD]
- Topic_rule_destination [CRUD]
- Thing_connectivity_data [R]
- Thing [CRUD]
- Encryption_configuration [RU]
- Ca_certificate [RUD]
- Job_document [R]
- Provisioning_claim [C]
- Thing_type [CRUD]
- Scheduled_audit [CRUD]
- Job [CRUD]
- Package_configuration [RU]
- Provisioning_template_version [CRD]
- Authorizer [CRUD]
- Certificate_provider [CRUD]
- Percentiles [R]
- Package_version [CRUD]
- Buckets_aggregation [R]
- Audit_mitigation_actions_task [R]
- Custom_metric [CRUD]
- Managed_job_template [R]
- Domain_configuration [CRUD]
- Account_audit_configuration [RUD]

📖 [Full iot documentation](services/iot.md)

### 301. Marketplace_commerce_analytics

**Resources**: 0


📖 [Full marketplace_commerce_analytics documentation](services/marketplace_commerce_analytics.md)

### 302. Frauddetector

**Resources**: 40

- Rule_metadata [U]
- Rule_version [U]
- Outcomes [R]
- Event [RD]
- Detector_version_status [U]
- Label [CD]
- Delete_events_by_event_type_status [R]
- List [CUD]
- Entity_types [R]
- Models [R]
- Rule [CD]
- Variable [CUD]
- Kms_encryption_key [CR]
- Event_prediction_metadata [R]
- Labels [R]
- Variables [R]
- Model_versions [R]
- Event_types [R]
- External_models [R]
- Entity_type [CD]
- External_model [CD]
- Batch_import_jobs [R]
- Model_version [CRUD]
- Event_prediction [R]
- Model [CUD]
- Events_by_event_type [D]
- Event_type [CD]
- Lists_metadata [R]
- Detector_version [CRUD]
- Batch_import_job [CD]
- Detector_version_metadata [U]
- Model_version_status [U]
- Event_label [U]
- Detectors [R]
- Batch_prediction_jobs [R]
- Outcome [CD]
- Batch_prediction_job [CD]
- List_elements [R]
- Detector [CRD]
- Rules [R]

📖 [Full frauddetector documentation](services/frauddetector.md)

### 303. Bedrock_data_automation

**Resources**: 1

- Blueprint_version [C]

📖 [Full bedrock_data_automation documentation](services/bedrock_data_automation.md)

### 304. Elastic_load_balancing

**Resources**: 37

- Trust_store [CD]
- Trust_store_associations [R]
- Trust_stores [R]
- Listener_attributes [R]
- Shared_trust_store_association [D]
- Capacity_reservation [R]
- Listener_certificates [R]
- Load_balancers [R]
- Ssl_policies [R]
- Load_balancer_attributes [R]
- Rules [R]
- Load_balancer [CD]
- Account_limits [R]
- Target_groups [R]
- Tags [R]
- Trust_store_revocations [R]
- Listeners [R]
- Target_health [R]
- Rule [CD]
- Target_group_attributes [R]
- Trust_store_ca_certificates_bundle [R]
- Target_group [CD]
- Trust_store_revocation_content [R]
- Listener [CD]
- Resource_policy [R]
- Load_balancer_policies [R]
- App_cookie_stickiness_policy [C]
- Load_balancer_policy_types [R]
- Account_limits [R]
- Load_balancers [R]
- Load_balancer [CD]
- Load_balancer_attributes [R]
- Instance_health [R]
- Load_balancer_listeners [CD]
- Tags [R]
- Lb_cookie_stickiness_policy [C]
- Load_balancer_policy [CD]

📖 [Full elastic_load_balancing documentation](services/elastic_load_balancing.md)

### 305. Verifiedpermissions

**Resources**: 0


📖 [Full verifiedpermissions documentation](services/verifiedpermissions.md)

### 306. Networkmanager

**Resources**: 37

- Sites [R]
- Core_network_policy [CR]
- Customer_gateway_associations [R]
- Link [CUD]
- Network_resource_metadata [U]
- Core_network_policy_version [D]
- Global_network [CUD]
- Transit_gateway_route_table_attachment [CR]
- Peering [D]
- Connect_peer_associations [R]
- Resource_policy [CRD]
- Network_resource_counts [R]
- Transit_gateway_connect_peer_associations [R]
- Connect_peer [CRD]
- Vpc_attachment [CRU]
- Global_networks [R]
- Devices [R]
- Link_associations [R]
- Device [CUD]
- Connect_attachment [CR]
- Site_to_site_vpn_attachment [CR]
- Core_network_change_events [R]
- Links [R]
- Network_routes [R]
- Connection [CUD]
- Direct_connect_gateway_attachment [CRU]
- Connections [R]
- Core_network_change_set [R]
- Attachment [D]
- Network_resources [R]
- Route_analysis [R]
- Core_network [CRUD]
- Network_telemetry [R]
- Site [CUD]
- Transit_gateway_peering [CR]
- Network_resource_relationships [R]
- Transit_gateway_registrations [R]

📖 [Full networkmanager documentation](services/networkmanager.md)

### 307. Devops_guru

**Resources**: 13

- Account_overview [R]
- Account_health [R]
- Feedback [CR]
- Resource_collection_health [R]
- Anomaly [R]
- Event_sources_config [RU]
- Organization_resource_collection_health [R]
- Organization_overview [R]
- Organization_health [R]
- Resource_collection [RU]
- Insight [RD]
- Service_integration [RU]
- Cost_estimation [R]

📖 [Full devops_guru documentation](services/devops_guru.md)

### 308. Taxsettings

**Resources**: 6

- Tax_registration [CRD]
- Tax_exemption_types [R]
- Tax_inheritance [CR]
- Tax_registration_document [R]
- Tax_exemption [C]
- Supplemental_tax_registration [CD]

📖 [Full taxsettings documentation](services/taxsettings.md)

### 309. Workspaces_instances

**Resources**: 2

- Volume [CD]
- Workspace_instance [CRD]

📖 [Full workspaces_instances documentation](services/workspaces_instances.md)

### 310. Arc_zonal_shift

**Resources**: 0


📖 [Full arc_zonal_shift documentation](services/arc_zonal_shift.md)

### 311. Elastic_transcoder

**Resources**: 5

- Pipeline [CUD]
- Pipeline_notifications [U]
- Preset [CD]
- Job [C]
- Pipeline_status [U]

📖 [Full elastic_transcoder documentation](services/elastic_transcoder.md)

### 312. Fms

**Resources**: 11

- Third_party_firewall_association_status [R]
- Protection_status [R]
- Admin_scope [R]
- Compliance_detail [R]
- Admin_account [CR]
- Notification_channel [CRD]
- Protocols_list [CRD]
- Apps_list [CRD]
- Policy [CRD]
- Violation_details [R]
- Resource_set [CRD]

📖 [Full fms documentation](services/fms.md)

### 313. Imagebuilder

**Resources**: 17

- Image_recipe_policy [CR]
- Container_recipe_policy [CR]
- Distribution_configuration [CRUD]
- Workflow_step_execution [R]
- Marketplace_resource [R]
- Image_recipe [CRD]
- Image_policy [CR]
- Image_pipeline [CRUD]
- Workflow [CRD]
- Infrastructure_configuration [CRUD]
- Lifecycle_policy [CRUD]
- Component_policy [CR]
- Workflow_execution [R]
- Image [CRD]
- Component [CRD]
- Container_recipe [CRD]
- Lifecycle_execution [R]

📖 [Full imagebuilder documentation](services/imagebuilder.md)

### 314. Chime_sdk

**Resources**: 25

- Sip_media_application_logging_configuration [CR]
- Voice_connector_group [CRUD]
- Voice_connector_external_systems_configuration [CRD]
- Speaker_search_task [R]
- Voice_connector_emergency_calling_configuration [CRD]
- Voice_profile_domain [CRUD]
- Phone_number_settings [RU]
- Global_settings [RU]
- Voice_connector_logging_configuration [CR]
- Sip_media_application_call [CU]
- Voice_connector [CRUD]
- Proxy_session [CRUD]
- Phone_number_order [CR]
- Voice_connector_proxy [CRD]
- Voice_profile [CRUD]
- Voice_connector_origination [CRD]
- Sip_rule [CRUD]
- Voice_tone_analysis_task [R]
- Voice_connector_streaming_configuration [CRD]
- Sip_media_application_alexa_skill_configuration [CR]
- Phone_number [RUD]
- Sip_media_application [CRUD]
- Voice_connector_termination [CRD]
- Voice_connector_termination_health [R]
- Voice_connector_termination_credentials [CD]

📖 [Full chime_sdk documentation](services/chime_sdk.md)

### 315. Groundstation

**Resources**: 1

- Minute_usage [R]

📖 [Full groundstation documentation](services/groundstation.md)

### 316. Forecast

**Resources**: 16

- Dataset [CRD]
- Explainability_export [CRD]
- Predictor [CRD]
- Explainability [CRD]
- Monitor [CRD]
- What_if_analysis [CRD]
- Forecast [CRD]
- What_if_forecast [CRD]
- Auto_predictor [CR]
- Predictor_backtest_export_job [CRD]
- What_if_forecast_export [CRD]
- Resource_tree [D]
- Dataset_import_job [CRD]
- Dataset_group [CRUD]
- Accuracy_metrics [R]
- Forecast_export_job [CRD]

📖 [Full forecast documentation](services/forecast.md)

### 317. Appstream

**Resources**: 34

- Application_fleet_associations [R]
- Application [CUD]
- Usage_report_subscription [CD]
- Fleet [CUD]
- Image [D]
- Fleets [R]
- Image_builder_streaming_url [C]
- Streaming_url [C]
- User [CD]
- App_blocks [R]
- User_stack_associations [R]
- Sessions [R]
- Image_builder [CD]
- Images [R]
- App_block_builder_app_block_associations [R]
- Stack [CUD]
- App_block_builder [CUD]
- App_block_builder_streaming_url [C]
- Software_associations [R]
- Applications [R]
- App_block_builders [R]
- App_license_usage [R]
- Directory_configs [R]
- Usage_report_subscriptions [R]
- Image_permissions [RUD]
- App_block [CD]
- Directory_config [CUD]
- Updated_image [C]
- Theme_for_stack [CRUD]
- Entitlements [R]
- Image_builders [R]
- Users [R]
- Entitlement [CUD]
- Stacks [R]

📖 [Full appstream documentation](services/appstream.md)

### 318. Chime_sdk_meetings

**Resources**: 4

- Meeting_with_attendees [C]
- Attendee [CRD]
- Meeting [CRD]
- Attendee_capabilities [U]

📖 [Full chime_sdk_meetings documentation](services/chime_sdk_meetings.md)

### 319. Comprehend

**Resources**: 16

- Entities_detection_job [R]
- Events_detection_job [R]
- Flywheel_iteration [R]
- Flywheel [CRUD]
- Entity_recognizer [CRD]
- Document_classifier [CRD]
- Document_classification_job [R]
- Key_phrases_detection_job [R]
- Sentiment_detection_job [R]
- Endpoint [CRUD]
- Dataset [CR]
- Dominant_language_detection_job [R]
- Topics_detection_job [R]
- Targeted_sentiment_detection_job [R]
- Pii_entities_detection_job [R]
- Resource_policy [CRD]

📖 [Full comprehend documentation](services/comprehend.md)

### 320. Redshift_serverless

**Resources**: 4

- Custom_domain_association [CRUD]
- Track [R]
- Credentials [R]
- Resource_policy [CRD]

📖 [Full redshift_serverless documentation](services/redshift_serverless.md)

### 321. Pinpoint

**Resources**: 53

- Adm_channel [RUD]
- Import_job [CR]
- Import_jobs [R]
- Template_active_version [U]
- App [CRD]
- Baidu_channel [RUD]
- Sms_channel [RUD]
- Apps [R]
- Email_channel [RUD]
- Recommender_configurations [R]
- Endpoint [RUD]
- Apns_sandbox_channel [RUD]
- Campaigns [R]
- Journey_run_execution_activity_metrics [R]
- Journey_state [U]
- Campaign [CRUD]
- Campaign_versions [R]
- Gcm_channel [RUD]
- In_app_messages [R]
- Voice_channel [RUD]
- Campaign_version [R]
- Apns_voip_channel [RUD]
- Channels [R]
- Journey_execution_metrics [R]
- Journey_run_execution_metrics [R]
- Journey_runs [R]
- Push_template [CRUD]
- User_endpoints [RD]
- Segment_import_jobs [R]
- Event_stream [CRD]
- Apns_voip_sandbox_channel [RUD]
- Recommender_configuration [CRUD]
- Campaign_activities [R]
- Sms_template [CRUD]
- Apns_channel [RUD]
- Email_template [CRUD]
- Journey_date_range_kpi [R]
- Segments [R]
- Segment_versions [R]
- Voice_template [CRUD]
- Segment_export_jobs [R]
- Journey_execution_activity_metrics [R]
- Export_jobs [R]
- Segment_version [R]
- Campaign_date_range_kpi [R]
- Events [C]
- Endpoints_batch [U]
- Application_date_range_kpi [R]
- Journey [CRUD]
- In_app_template [CRUD]
- Segment [CRUD]
- Export_job [CR]
- Application_settings [RU]

📖 [Full pinpoint documentation](services/pinpoint.md)

### 322. Pi

**Resources**: 5

- Dimension_keys [R]
- Performance_analysis_report [CRD]
- Resource_metrics [R]
- Resource_metadata [R]
- Dimension_key_details [R]

📖 [Full pi documentation](services/pi.md)

### 323. Gameliftstreams

**Resources**: 2

- Stream_session [R]
- Stream_session_connection [C]

📖 [Full gameliftstreams documentation](services/gameliftstreams.md)

### 324. Customer_profiles

**Resources**: 26

- Profile [CUD]
- Segment_snapshot [CR]
- Upload_job_path [R]
- Profile_object_type_template [R]
- Event_stream [CRD]
- Domain_layout [CRUD]
- Domain [CRUD]
- Segment_definition [CRD]
- Calculated_attribute_definition [CRUD]
- Integration_workflow [C]
- Calculated_attribute_for_profile [R]
- Matches [R]
- Workflow [RD]
- Integration [CRD]
- Workflow_steps [R]
- Profile_key [D]
- Profile_object [CD]
- Segment_membership [R]
- Similar_profiles [R]
- Auto_merging_preview [R]
- Upload_job [CR]
- Identity_resolution_job [R]
- Profile_object_type [CRD]
- Segment_estimate [CR]
- Event_trigger [CRUD]
- Profile_history_record [R]

📖 [Full customer_profiles documentation](services/customer_profiles.md)

### 325. Workspaces

**Resources**: 37

- Rules_of_ip_group [U]
- Workspace_image [CD]
- Ip_groups [R]
- Image_associations [R]
- Account [R]
- Workspace_images [R]
- Workspaces_connection_status [R]
- Workspace_snapshots [R]
- Workspace_image_permissions [R]
- Workspace_bundle [CUD]
- Connection_alias [CD]
- Bundle_associations [R]
- Client_branding [RD]
- Connect_client_add_in [CUD]
- Workspaces [CR]
- Account_link_invitation [CD]
- Workspace_directories [R]
- Standby_workspaces [C]
- Workspace_image_permission [U]
- Applications [R]
- Tags [CRD]
- Connection_alias_permissions [R]
- Custom_workspace_image_import [R]
- Connection_alias_permission [U]
- Workspace_bundles [R]
- Connect_client_add_ins [R]
- Workspaces_pool_sessions [R]
- Account_link [R]
- Ip_group [CD]
- Client_properties [R]
- Connection_aliases [R]
- Updated_workspace_image [C]
- Workspace_associations [R]
- Account_modifications [R]
- Workspaces_pool [CU]
- Application_associations [R]
- Workspaces_pools [R]

📖 [Full workspaces documentation](services/workspaces.md)

### 326. Auditmanager

**Resources**: 23

- Assessment_control [U]
- Assessment_report [CD]
- Assessment_control_set_status [U]
- Organization_admin_account [R]
- Assessment_status [U]
- Evidence_file_upload_url [R]
- Assessment [CRUD]
- Change_logs [R]
- Insights_by_assessment [R]
- Evidence_by_evidence_folder [R]
- Assessment_framework [CRUD]
- Insights [R]
- Control [CRUD]
- Services_in_scope [R]
- Evidence [R]
- Assessment_framework_share [UD]
- Evidence_folder [R]
- Settings [RU]
- Evidence_folders_by_assessment [R]
- Assessment_report_url [R]
- Account_status [R]
- Delegations [R]
- Evidence_folders_by_assessment_control [R]

📖 [Full auditmanager documentation](services/auditmanager.md)

### 327. Docdb

**Resources**: 23

- Db_cluster [CD]
- Certificates [R]
- Db_instance [CD]
- Event_subscription [CD]
- Db_clusters [R]
- Db_instances [R]
- Engine_default_cluster_parameters [R]
- Event_subscriptions [R]
- Orderable_db_instance_options [R]
- Pending_maintenance_actions [R]
- Db_cluster_parameter_groups [R]
- Db_subnet_group [CD]
- Db_subnet_groups [R]
- Events [R]
- Global_clusters [R]
- Db_cluster_snapshots [R]
- Db_cluster_parameters [R]
- Global_cluster [CD]
- Db_cluster_snapshot_attributes [R]
- Db_engine_versions [R]
- Event_categories [R]
- Db_cluster_parameter_group [CD]
- Db_cluster_snapshot [CD]

📖 [Full docdb documentation](services/docdb.md)

### 328. Mturk

**Resources**: 14

- Additional_assignments_for_hit [C]
- Hit_with_hit_type [C]
- Hit_type [C]
- Qualification_score [R]
- Qualification_type [CRUD]
- Assignment [R]
- Hit [CRD]
- Expiration_for_hit [U]
- Notification_settings [U]
- Worker_block [CD]
- Hit_review_status [U]
- Account_balance [R]
- File_upload_url [R]
- Hit_type_of_hit [U]

📖 [Full mturk documentation](services/mturk.md)

### 329. Cognito_identity

**Resources**: 9

- Principal_tag_attribute_map [R]
- Identities [D]
- Identity_pool [CRUD]
- Credentials_for_identity [R]
- Identity_pool_roles [R]
- Identity [R]
- Id [R]
- Open_id_token_for_developer_identity [R]
- Open_id_token [R]

📖 [Full cognito_identity documentation](services/cognito_identity.md)

### 330. Dynamodb

**Resources**: 15

- Endpoints [R]
- Export [R]
- Time_to_live [RU]
- Table [CRUD]
- Global_table_settings [RU]
- Resource_policy [CRD]
- Contributor_insights [RU]
- Limits [R]
- Continuous_backups [RU]
- Global_table [CRU]
- Backup [CRD]
- Import [R]
- Kinesis_streaming_destination [RU]
- Item [CRUD]
- Table_replica_auto_scaling [RU]

📖 [Full dynamodb documentation](services/dynamodb.md)

### 331. Codeartifact

**Resources**: 16

- Repository_permissions_policy [CRD]
- Package_version [R]
- Domain [CRD]
- Repository [CRUD]
- Authorization_token [R]
- Package_version_readme [R]
- Package_version_asset [R]
- Associated_package_group [R]
- Package_origin_configuration [C]
- Package_versions [D]
- Package_versions_status [U]
- Package_group [CRUD]
- Domain_permissions_policy [CRD]
- Package_group_origin_configuration [U]
- Package [RD]
- Repository_endpoint [R]

📖 [Full codeartifact documentation](services/codeartifact.md)

### 332. Organizations

**Resources**: 9

- Create_account_status [R]
- Handshake [R]
- Gov_cloud_account [C]
- Organizational_unit [CRUD]
- Effective_policy [R]
- Account [CR]
- Organization [CRD]
- Resource_policy [CRD]
- Policy [CRUD]

📖 [Full organizations documentation](services/organizations.md)

### 333. Dlm

**Resources**: 2

- Lifecycle_policy [CRUD]
- Lifecycle_policies [R]

📖 [Full dlm documentation](services/dlm.md)

### 334. Sso

**Resources**: 1

- Role_credentials [R]

📖 [Full sso documentation](services/sso.md)

### 335. Osis

**Resources**: 5

- Pipeline_endpoint [CD]
- Pipeline_change_progress [R]
- Pipeline_blueprint [R]
- Resource_policy [CRD]
- Pipeline [CRUD]

📖 [Full osis documentation](services/osis.md)

### 336. Migration_hub

**Resources**: 4

- Progress_update_stream [CD]
- Migration_task [R]
- Application_state [R]
- Resource_attributes [C]

📖 [Full migration_hub documentation](services/migration_hub.md)

### 337. Chatbot

**Resources**: 12

- Account_preferences [RU]
- Microsoft_teams_user_identity [D]
- Slack_user_identities [R]
- Slack_channel_configuration [CUD]
- Slack_workspace_authorization [D]
- Microsoft_teams_channel_configuration [CRUD]
- Chime_webhook_configurations [R]
- Microsoft_teams_configured_team [D]
- Chime_webhook_configuration [CUD]
- Slack_user_identity [D]
- Slack_channel_configurations [R]
- Slack_workspaces [R]

📖 [Full chatbot documentation](services/chatbot.md)

### 338. Docdb_elastic

**Resources**: 3

- Cluster [CRUD]
- Cluster_snapshot [CRD]
- Pending_maintenance_action [R]

📖 [Full docdb_elastic documentation](services/docdb_elastic.md)

### 339. Supplychain

**Resources**: 2

- Data_integration_flow_execution [R]
- Data_integration_event [R]

📖 [Full supplychain documentation](services/supplychain.md)

### 340. Ses

**Resources**: 23

- Receipt_rule [CRUD]
- Identity_dkim_attributes [R]
- Identity [D]
- Identity_mail_from_domain_attributes [R]
- Identity_notification_attributes [R]
- Identity_policy [CD]
- Receipt_filter [CD]
- Configuration_set_event_destination [CUD]
- Configuration_set_delivery_options [C]
- Receipt_rule_set [CRD]
- Custom_verification_email_template [CRUD]
- Verified_email_address [D]
- Configuration_set_tracking_options [CUD]
- Identity_policies [R]
- Send_statistics [R]
- Configuration_set [CRD]
- Active_receipt_rule_set [R]
- Account_sending_enabled [RU]
- Identity_verification_attributes [R]
- Send_quota [R]
- Configuration_set_reputation_metrics_enabled [U]
- Configuration_set_sending_enabled [U]
- Template [CRUD]

📖 [Full ses documentation](services/ses.md)

### 341. Repostspace

**Resources**: 2

- Channel [CRU]
- Space [CRUD]

📖 [Full repostspace documentation](services/repostspace.md)

### 342. Mediastore_data

**Resources**: 1

- Object [CRD]

📖 [Full mediastore_data documentation](services/mediastore_data.md)

### 343. Bedrock_agent

**Resources**: 0


📖 [Full bedrock_agent documentation](services/bedrock_agent.md)

### 344. Wellarchitected

**Resources**: 21

- Lens_share [CD]
- Lens [RD]
- Lens_review_report [R]
- Review_template [CRUD]
- Share_invitation [U]
- Review_template_lens_review [RU]
- Integration [U]
- Lens_version [C]
- Workload [CRUD]
- Consolidated_report [R]
- Review_template_answer [RU]
- Profile_share [CD]
- Profile [CRUD]
- Workload_share [CUD]
- Template_share [CD]
- Milestone [CR]
- Answer [RU]
- Lens_review [RU]
- Lens_version_difference [R]
- Profile_template [R]
- Global_settings [RU]

📖 [Full wellarchitected documentation](services/wellarchitected.md)

### 345. Budgets

**Resources**: 12

- Notification [CUD]
- Budgets [R]
- Budget_performance_history [R]
- Subscribers_for_notification [R]
- Budget_action_histories [R]
- Subscriber [CUD]
- Budget_actions_for_account [R]
- Notifications_for_budget [R]
- Budget [CRUD]
- Budget_actions_for_budget [R]
- Budget_action [CRUD]
- Budget_notifications_for_account [R]

📖 [Full budgets documentation](services/budgets.md)

### 346. Mediatailor

**Resources**: 0


📖 [Full mediatailor documentation](services/mediatailor.md)

### 347. Appsync

**Resources**: 16

- Resolver [CRUD]
- Type [CRUD]
- Domain_name [CRUD]
- Api_association [R]
- Api [CRUD]
- Schema_creation_status [R]
- Data_source [CRUD]
- Data_source_introspection [R]
- Function [CRUD]
- Graphql_api [CRUD]
- Source_api_association [RU]
- Graphql_api_environment_variables [CR]
- Introspection_schema [R]
- Channel_namespace [CRUD]
- Api_key [CUD]
- Api_cache [CRUD]

📖 [Full appsync documentation](services/appsync.md)

### 348. Ssm_guiconnect

**Resources**: 0


📖 [Full ssm_guiconnect documentation](services/ssm_guiconnect.md)

### 349. Evs

**Resources**: 0


📖 [Full evs documentation](services/evs.md)

### 350. Eks_auth

**Resources**: 0


📖 [Full eks_auth documentation](services/eks_auth.md)

### 351. Chime_sdk_messaging

**Resources**: 14

- Channel_message [RUD]
- Channel_message_status [R]
- Channel_expiration_settings [C]
- Channel_membership_preferences [CR]
- Messaging_session_endpoint [R]
- Channel_read_marker [U]
- Channel_moderated_by_app_instance_user [R]
- Messaging_streaming_configurations [CRD]
- Channel [CRUD]
- Channel_moderator [CRD]
- Channel_ban [CRD]
- Channel_flow [CRUD]
- Channel_membership [CRD]
- Channel_membership_for_app_instance_user [R]

📖 [Full chime_sdk_messaging documentation](services/chime_sdk_messaging.md)

### 352. Mediaconnect

**Resources**: 0


📖 [Full mediaconnect documentation](services/mediaconnect.md)

### 353. Identitystore

**Resources**: 3

- Group_id [R]
- User_id [R]
- Group_membership_id [R]

📖 [Full identitystore documentation](services/identitystore.md)

### 354. Bcm_pricing_calculator

**Resources**: 1

- Preferences [RU]

📖 [Full bcm_pricing_calculator documentation](services/bcm_pricing_calculator.md)

### 355. Lakeformation

**Resources**: 20

- Data_lake_settings [CR]
- Query_statistics [R]
- Lf_tag_expression [CRUD]
- Data_cells_filter [CRUD]
- Data_lake_principal [R]
- Lake_formation_identity_center_configuration [CRUD]
- Work_units [R]
- Table_storage_optimizer [U]
- Transaction [R]
- Temporary_glue_partition_credentials [R]
- Objects_on_cancel [D]
- Lake_formation_opt_in [CD]
- Temporary_glue_table_credentials [R]
- Work_unit_results [R]
- Table_objects [RU]
- Lf_tag [CRUD]
- Effective_permissions_for_path [R]
- Query_state [R]
- Resource_lf_tags [R]
- Resource [RU]

📖 [Full lakeformation documentation](services/lakeformation.md)

### 356. Xray

**Resources**: 22

- Trace_summaries [R]
- Sampling_targets [R]
- Indexing_rules [R]
- Insight_impact_graph [R]
- Trace_graph [R]
- Resource_policy [CD]
- Groups [R]
- Retrieved_traces_graph [R]
- Service_graph [R]
- Insight_summaries [R]
- Sampling_statistic_summaries [R]
- Insight_events [R]
- Indexing_rule [U]
- Group [CRUD]
- Telemetry_records [C]
- Trace_segments [C]
- Encryption_config [CR]
- Sampling_rules [R]
- Insight [R]
- Time_series_service_statistics [R]
- Sampling_rule [CUD]
- Trace_segment_destination [RU]

📖 [Full xray documentation](services/xray.md)

### 357. Cloudfront

**Resources**: 41

- Connection_group [CRUD]
- Streaming_distribution_config [R]
- Distribution_tenant [CRUD]
- Distribution_with_staging_config [U]
- Function [CRUD]
- Field_level_encryption_profile_config [R]
- Origin_request_policy_config [R]
- Field_level_encryption_config [CRUD]
- Origin_access_control [CRUD]
- Continuous_deployment_policy [CRUD]
- Response_headers_policy [CRUD]
- Streaming_distribution_with_tags [C]
- Cache_policy [CRUD]
- Origin_access_control_config [R]
- Cloud_front_origin_access_identity [CRUD]
- Domain_association [U]
- Public_key [CRUD]
- Field_level_encryption [R]
- Anycast_ip_list [CRD]
- Cache_policy_config [R]
- Monitoring_subscription [CRD]
- Invalidation_for_distribution_tenant [CR]
- Key_group_config [R]
- Key_group [CRUD]
- Response_headers_policy_config [R]
- Field_level_encryption_profile [CRUD]
- Distribution_with_tags [C]
- Distribution_tenant_by_domain [R]
- Distribution [CRUD]
- Invalidation [CR]
- Distribution_config [R]
- Connection_group_by_routing_endpoint [R]
- Public_key_config [R]
- Managed_certificate_details [R]
- Vpc_origin [CRUD]
- Streaming_distribution [CRUD]
- Continuous_deployment_policy_config [R]
- Realtime_log_config [CRUD]
- Origin_request_policy [CRUD]
- Cloud_front_origin_access_identity_config [R]
- Key_value_store [CRUD]

📖 [Full cloudfront documentation](services/cloudfront.md)

### 358. Sagemaker_metrics

**Resources**: 0


📖 [Full sagemaker_metrics documentation](services/sagemaker_metrics.md)

### 359. Sso_oidc

**Resources**: 2

- Token_with_iam [C]
- Token [C]

📖 [Full sso_oidc documentation](services/sso_oidc.md)

### 360. Sagemaker

**Resources**: 90

- Edge_packaging_job [CR]
- Reserved_capacity [R]
- Inference_recommendations_job [CR]
- Device_fleet_report [R]
- Hub_content [RUD]
- Trial_component [CRUD]
- Artifact [CRUD]
- Device_fleet [CRUD]
- Lineage_group [R]
- Subscribed_workteam [R]
- Inference_experiment [CRUD]
- Workforce [CRUD]
- Model_package_group_policy [CRD]
- Human_task_ui [CRD]
- Pipeline_version [U]
- Cluster_scheduler_config [CRUD]
- Hub_content_reference [CUD]
- Partner_app [CRUD]
- Feature_metadata [RU]
- Sagemaker_servicecatalog_portfolio_status [R]
- User_profile [CRUD]
- App [CRD]
- Labeling_job [CR]
- Model_bias_job_definition [CRD]
- Notebook_instance [CRUD]
- Data_quality_job_definition [CRD]
- Presigned_domain_url [C]
- Cluster_event [R]
- Compute_quota [CRUD]
- Hub [CRUD]
- Device [R]
- Endpoint [CRUD]
- Pipeline_execution [RU]
- Processing_job [CRD]
- Pipeline_definition_for_execution [R]
- Monitoring_alert [U]
- Compilation_job [CRD]
- Auto_ml_job [CR]
- Studio_lifecycle_config [CRD]
- Training_plan [CR]
- Optimization_job [CRD]
- Model_explainability_job_definition [CRD]
- Cluster [CRUD]
- Context [CRUD]
- Cluster_software [U]
- Algorithm [CRD]
- Inference_component_runtime_config [U]
- Experiment [CRUD]
- App_image_config [CRUD]
- Domain [CRUD]
- Flow_definition [CRD]
- Hub_content_presigned_urls [C]
- Edge_deployment_stage [CD]
- Inference_component [CRUD]
- Model_card_export_job [CR]
- Model_package_group [CRD]
- Monitoring_schedule [CRUD]
- Partner_app_presigned_url [C]
- Project [CRUD]
- Space [CRUD]
- Transform_job [CR]
- Workteam [CRUD]
- Tags [D]
- Lineage_group_policy [R]
- Pipeline [CRUD]
- Scaling_configuration_recommendation [R]
- Endpoint_config [CRD]
- Mlflow_tracking_server [CRUD]
- Training_job [CRUD]
- Model_package [CRUD]
- Auto_ml_job_v2 [CR]
- Trial [CRUD]
- Search_suggestions [R]
- Presigned_notebook_instance_url [C]
- Image [CRUD]
- Code_repository [CRUD]
- Feature_group [CRUD]
- Devices [U]
- Model_quality_job_definition [CRD]
- Model [CRD]
- Model_card [CRUD]
- Cluster_node [R]
- Endpoint_weights_and_capacities [U]
- Edge_deployment_plan [CRD]
- Hyper_parameter_tuning_job [CRD]
- Presigned_mlflow_tracking_server_url [C]
- Notebook_instance_lifecycle_config [CRUD]
- Action [CRUD]
- Image_version [CRUD]
- Association [D]

📖 [Full sagemaker documentation](services/sagemaker.md)

### 361. Codestar_connections

**Resources**: 8

- Sync_blocker [U]
- Repository_sync_status [R]
- Sync_configuration [CRUD]
- Host [CRUD]
- Resource_sync_status [R]
- Repository_link [CRUD]
- Sync_blocker_summary [R]
- Connection [CRD]

📖 [Full codestar_connections documentation](services/codestar_connections.md)

### 362. Device_farm

**Resources**: 19

- Test_grid_url [C]
- Device_pool_compatibility [R]
- Test [R]
- Job [R]
- Device_pool [CRUD]
- Run [RD]
- Vpce_configuration [CRUD]
- Suite [R]
- Remote_access_session [CRD]
- Instance_profile [CRUD]
- Test_grid_project [CRUD]
- Device [R]
- Device_instance [RU]
- Project [CRUD]
- Upload [CRUD]
- Offering_status [R]
- Account_settings [R]
- Network_profile [CRUD]
- Test_grid_session [R]

📖 [Full device_farm documentation](services/device_farm.md)

### 363. Translate

**Resources**: 3

- Parallel_data [CRUD]
- Terminology [RD]
- Text_translation_job [R]

📖 [Full translate documentation](services/translate.md)

### 364. Sagemaker_runtime

**Resources**: 0


📖 [Full sagemaker_runtime documentation](services/sagemaker_runtime.md)

### 365. B2bi

**Resources**: 2

- Starter_mapping_template [C]
- Transformer_job [R]

📖 [Full b2bi documentation](services/b2bi.md)

### 366. Savingsplans

**Resources**: 6

- Savings_plan [C]
- Queued_savings_plan [D]
- Savings_plan_rates [R]
- Savings_plans [R]
- Savings_plans_offering_rates [R]
- Savings_plans_offerings [R]

📖 [Full savingsplans documentation](services/savingsplans.md)

### 367. Pipes

**Resources**: 0


📖 [Full pipes documentation](services/pipes.md)

### 368. Config_service

**Resources**: 60

- Organization_custom_rule_policy [R]
- Remediation_configuration [D]
- Aggregate_config_rule_compliance_summary [R]
- Pending_aggregation_request [D]
- Organization_config_rule [CD]
- Config_rule [CD]
- Compliance_details_by_resource [R]
- Delivery_channel [CD]
- Aggregate_discovered_resource_counts [R]
- Custom_rule_policy [R]
- Resource_evaluation_summary [R]
- External_evaluation [C]
- Organization_config_rule_detailed_status [R]
- Discovered_resource_counts [R]
- Evaluation_results [D]
- Retention_configurations [R]
- Stored_query [CRD]
- Configuration_aggregators [R]
- Delivery_channel_status [R]
- Organization_conformance_packs [R]
- Compliance_by_config_rule [R]
- Configuration_aggregator_sources_status [R]
- Configuration_recorder [CD]
- Organization_conformance_pack [CD]
- Compliance_summary_by_config_rule [R]
- Aggregate_compliance_details_by_config_rule [R]
- Aggregate_conformance_pack_compliance_summary [R]
- Aggregation_authorization [CD]
- Conformance_pack [CD]
- Conformance_packs [R]
- Conformance_pack_compliance [R]
- Retention_configuration [CD]
- Configuration_recorder_status [R]
- Resource_config [CD]
- Remediation_exceptions [CRD]
- Aggregate_compliance_by_conformance_packs [R]
- Configuration_aggregator [CD]
- Aggregate_compliance_by_config_rules [R]
- Conformance_pack_status [R]
- Conformance_pack_compliance_details [R]
- Aggregation_authorizations [R]
- Aggregate_resource_config [R]
- Compliance_details_by_config_rule [R]
- Remediation_configurations [CR]
- Conformance_pack_compliance_summary [R]
- Config_rule_evaluation_status [R]
- Pending_aggregation_requests [R]
- Configuration_recorders [R]
- Organization_conformance_pack_statuses [R]
- Organization_config_rules [R]
- Organization_conformance_pack_detailed_status [R]
- Organization_config_rule_statuses [R]
- Config_rules [R]
- Evaluations [C]
- Delivery_channels [R]
- Remediation_execution_status [R]
- Compliance_by_resource [R]
- Resource_config_history [R]
- Service_linked_configuration_recorder [CD]
- Compliance_summary_by_resource_type [R]

📖 [Full config_service documentation](services/config_service.md)

### 369. Codeguruprofiler

**Resources**: 1

- Findings_report_account_summary [R]

📖 [Full codeguruprofiler documentation](services/codeguruprofiler.md)

### 370. S3

**Resources**: 38

- Object [CRD]
- Session [C]
- Bucket_cors [CRD]
- Objects [D]
- Bucket_analytics_configuration [CRD]
- Bucket_metrics_configuration [CRD]
- Public_access_block [CRD]
- Object_torrent [R]
- Object_legal_hold [CR]
- Bucket_metadata_configuration [CRD]
- Bucket_tagging [CRD]
- Bucket_versioning [CR]
- Bucket_notification_configuration [CR]
- Bucket_logging [CR]
- Object_acl [CR]
- Bucket_policy_status [R]
- Bucket_accelerate_configuration [CR]
- Bucket_encryption [CRD]
- Bucket_website [CRD]
- Object_lock_configuration [CR]
- Object_retention [CR]
- Bucket [CRD]
- Bucket_ownership_controls [CRD]
- Bucket_policy [CRD]
- Bucket_lifecycle [D]
- Bucket_metadata_journal_table_configuration [U]
- Bucket_acl [CR]
- Bucket_metadata_table_configuration [CRD]
- Bucket_lifecycle_configuration [CR]
- Bucket_inventory_configuration [CRD]
- Object_attributes [R]
- Bucket_location [R]
- Bucket_request_payment [CR]
- Object_tagging [CRD]
- Bucket_metadata_inventory_table_configuration [U]
- Multipart_upload [C]
- Bucket_replication [CRD]
- Bucket_intelligent_tiering_configuration [CRD]

📖 [Full s3 documentation](services/s3.md)

### 371. Polly

**Resources**: 3

- Lexicon [CRD]
- Voices [R]
- Speech_synthesis_task [R]

📖 [Full polly documentation](services/polly.md)

### 372. Cognito_sync

**Resources**: 7

- Identity_usage [R]
- Identity_pool_configuration [R]
- Dataset [RD]
- Records [U]
- Bulk_publish_details [R]
- Cognito_events [R]
- Identity_pool_usage [R]

📖 [Full cognito_sync documentation](services/cognito_sync.md)

### 373. Scheduler

**Resources**: 0


📖 [Full scheduler documentation](services/scheduler.md)

### 374. Pca_connector_ad

**Resources**: 0


📖 [Full pca_connector_ad documentation](services/pca_connector_ad.md)

### 375. Waf_regional

**Resources**: 20

- Geo_match_set [CRUD]
- Web_acl_migration_stack [C]
- Regex_pattern_set [CRUD]
- Web_acl_for_resource [R]
- Byte_match_set [CRUD]
- Rate_based_rule [CRUD]
- Rule [CRUD]
- Logging_configuration [CRD]
- Sql_injection_match_set [CRUD]
- Change_token [R]
- Permission_policy [CRD]
- Change_token_status [R]
- Rate_based_rule_managed_keys [R]
- Rule_group [CRUD]
- Ip_set [CRUD]
- Size_constraint_set [CRUD]
- Web_acl [CRUD]
- Regex_match_set [CRUD]
- Xss_match_set [CRUD]
- Sampled_requests [R]

📖 [Full waf_regional documentation](services/waf_regional.md)

### 376. Apigatewaymanagementapi

**Resources**: 1

- Connection [RD]

📖 [Full apigatewaymanagementapi documentation](services/apigatewaymanagementapi.md)

### 377. Workspaces_web

**Resources**: 1

- Session [R]

📖 [Full workspaces_web documentation](services/workspaces_web.md)

### 378. Pca_connector_scep

**Resources**: 0


📖 [Full pca_connector_scep documentation](services/pca_connector_scep.md)

### 379. Codestar_notifications

**Resources**: 2

- Notification_rule [CRUD]
- Target [D]

📖 [Full codestar_notifications documentation](services/codestar_notifications.md)

### 380. Direct_connect

**Resources**: 30

- Transit_virtual_interface [C]
- Direct_connect_gateway_associations [R]
- Router_configuration [R]
- Interconnect [CD]
- Lag [CUD]
- Connections [R]
- Direct_connect_gateway_association_proposals [R]
- Lags [R]
- Virtual_gateways [R]
- Connection [CUD]
- Virtual_interfaces [R]
- Hosted_connections [R]
- Tags [R]
- Direct_connect_gateway_attachments [R]
- Customer_metadata [R]
- Public_virtual_interface [C]
- Bgp_peer [CD]
- Virtual_interface_attributes [U]
- Connections_on_interconnect [R]
- Virtual_interface [D]
- Interconnect_loa [R]
- Loa [R]
- Private_virtual_interface [C]
- Locations [R]
- Direct_connect_gateway [CUD]
- Interconnects [R]
- Connection_loa [R]
- Direct_connect_gateways [R]
- Direct_connect_gateway_association_proposal [CD]
- Direct_connect_gateway_association [CUD]

📖 [Full direct_connect documentation](services/direct_connect.md)

### 381. Shield

**Resources**: 9

- Protection_group [CRUD]
- Subscription_state [R]
- Drt_access [R]
- Attack [R]
- Emergency_contact_settings [RU]
- Application_layer_automatic_response [U]
- Subscription [CRUD]
- Protection [CRD]
- Attack_statistics [R]

📖 [Full shield documentation](services/shield.md)

### 382. Application_signals

**Resources**: 2

- Grouping_configuration [CD]
- Service [R]

📖 [Full application_signals documentation](services/application_signals.md)

### 383. Iot_managed_integrations

**Resources**: 1

- Custom_endpoint [R]

📖 [Full iot_managed_integrations documentation](services/iot_managed_integrations.md)

### 384. Iot_wireless

**Resources**: 29

- Resource_log_level [CR]
- Metrics [R]
- Partner_account [RU]
- Fuota_task [CRUD]
- Wireless_gateway_statistics [R]
- Event_configuration_by_resource_types [RU]
- Metric_configuration [RU]
- Service_profile [CRD]
- Wireless_gateway_task [CRD]
- Multicast_group_session [R]
- Device_profile [CRD]
- Wireless_gateway_firmware_information [R]
- Wireless_gateway_task_definition [CRD]
- Destination [CRUD]
- Log_levels_by_resource_types [RU]
- Position [RU]
- Service_endpoint [R]
- Wireless_gateway_certificate [R]
- Resource_event_configuration [RU]
- Position_estimate [R]
- Resource_position [RU]
- Wireless_device_statistics [R]
- Position_configuration [CR]
- Network_analyzer_configuration [CRUD]
- Wireless_device [CRUD]
- Wireless_device_import_task [RUD]
- Multicast_group [CRUD]
- Wireless_gateway [CRUD]
- Queued_messages [D]

📖 [Full iot_wireless documentation](services/iot_wireless.md)

### 385. Iot_events

**Resources**: 6

- Detector_model [CRUD]
- Detector_model_analysis [R]
- Logging_options [CR]
- Detector_model_analysis_results [R]
- Alarm_model [CRUD]
- Input [CRUD]

📖 [Full iot_events documentation](services/iot_events.md)

### 386. Backup_gateway

**Resources**: 0


📖 [Full backup_gateway documentation](services/backup_gateway.md)

### 387. Sso_admin

**Resources**: 19

- Permissions_boundary_for_permission_set [R]
- Trusted_token_issuer [CRUD]
- Inline_policy_for_permission_set [R]
- Account_assignment_deletion_status [R]
- Permission_set_provisioning_status [R]
- Application_provider [R]
- Permissions_boundary_to_permission_set [C]
- Application [CRUD]
- Application_assignment_configuration [CR]
- Inline_policy_from_permission_set [D]
- Inline_policy_to_permission_set [C]
- Permissions_boundary_from_permission_set [D]
- Account_assignment [CD]
- Account_assignment_creation_status [R]
- Application_assignment [CRD]
- Permission_set [CRUD]
- Application_session_configuration [CR]
- Instance [CRUD]
- Instance_access_control_attribute_configuration [CRUD]

📖 [Full sso_admin documentation](services/sso_admin.md)

### 388. Elastic_beanstalk

**Resources**: 21

- Environment [CU]
- Environment_health [R]
- Environment_managed_actions [R]
- Instances_health [R]
- Environments [R]
- Application_resource_lifecycle [U]
- Configuration_settings [R]
- Application [CUD]
- Environment_managed_action_history [R]
- Environment_resources [R]
- Environment_configuration [D]
- Events [R]
- Applications [R]
- Configuration_options [R]
- Application_version [CUD]
- Account_attributes [R]
- Storage_location [C]
- Tags_for_resource [U]
- Platform_version [CRD]
- Application_versions [R]
- Configuration_template [CUD]

📖 [Full elastic_beanstalk documentation](services/elastic_beanstalk.md)

### 389. Drs

**Resources**: 2

- Extended_source_server [C]
- Launch_action [CD]

📖 [Full drs documentation](services/drs.md)

### 390. Personalize_runtime

**Resources**: 3

- Personalized_ranking [R]
- Recommendations [R]
- Action_recommendations [R]

📖 [Full personalize_runtime documentation](services/personalize_runtime.md)

### 391. Outposts

**Resources**: 11

- Outpost [CRUD]
- Capacity_task [R]
- Connection [R]
- Site_rack_physical_properties [U]
- Site [CRUD]
- Outpost_supported_instance_types [R]
- Outpost_billing_information [R]
- Catalog_item [R]
- Site_address [RU]
- Outpost_instance_types [R]
- Order [CR]

📖 [Full outposts documentation](services/outposts.md)

### 392. License_manager_user_subscriptions

**Resources**: 2

- License_server_endpoint [CD]
- Identity_provider_settings [U]

📖 [Full license_manager_user_subscriptions documentation](services/license_manager_user_subscriptions.md)

### 393. Cloudtrail_data

**Resources**: 1

- Audit_events [C]

📖 [Full cloudtrail_data documentation](services/cloudtrail_data.md)

### 394. Lex_runtime_service

**Resources**: 1

- Session [CRD]

📖 [Full lex_runtime_service documentation](services/lex_runtime_service.md)


---

## Example: Complete Workflow

Here's a complete example showing a typical workflow:

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---

## Configuration

### Environment Variables

Configure AWS credentials using environment variables:

```bash
export AWS_ACCESS_KEY_ID=your_access_key
export AWS_SECRET_ACCESS_KEY=your_secret_key
export AWS_REGION=us-east-1  # Optional, defaults to us-east-1
```

### KCL Configuration

```kcl
# Configure provider in your KCL code
provider = aws.AwsProvider {
    region = "us-west-2"
    # Credentials will be read from environment:
    # AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY
}
```

---

## Next Steps

- 📚 [Service Documentation](services/) - Detailed docs for each service
- 📖 [Installation Guide](installation.md) - Installation options
- ⬅️ [Back to README](../README.md)

---

## Need Help?

- 📖 Check service-specific documentation in `docs/services/`
- 🐛 [Report issues](https://github.com/YOUR_ORG/hemmer-provider-aws/issues)
- 💬 [Join discussions](https://github.com/YOUR_ORG/hemmer-provider-aws/discussions)
