# Aws Provider for Hemmer

**Auto-generated AWS provider with 394 services and 4616 resources**

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

---

## Installation

### Using Hemmer CLI (Recommended)

```bash
hemmer provider install aws
```

### Manual Installation

Download the latest release for your platform from the [Releases](../../releases) page.

📖 **[Detailed installation instructions](docs/installation.md)**

---

## Quick Start

```rust
use hemmer_aws_provider::AwsProvider;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let provider = AwsProvider::new().await?;

    // Access service
    let emr_serverless_service = provider.emr_serverless();

    Ok(())
}
```

📖 **[Getting started guide](docs/getting-started.md)**

---

## Services

This provider includes the following services:

| Service | Resources | Documentation |
|---------|-----------|---------------|
| emr_serverless | 0 | [docs/services/emr_serverless.md](docs/services/emr_serverless.md) |
| cloudformation | 25 | [docs/services/cloudformation.md](docs/services/cloudformation.md) |
| application_auto_scaling | 7 | [docs/services/application_auto_scaling.md](docs/services/application_auto_scaling.md) |
| personalize_events | 5 | [docs/services/personalize_events.md](docs/services/personalize_events.md) |
| tnb | 9 | [docs/services/tnb.md](docs/services/tnb.md) |
| rolesanywhere | 1 | [docs/services/rolesanywhere.md](docs/services/rolesanywhere.md) |
| kms | 12 | [docs/services/kms.md](docs/services/kms.md) |
| datasync | 15 | [docs/services/datasync.md](docs/services/datasync.md) |
| bedrock_agent_runtime | 0 | [docs/services/bedrock_agent_runtime.md](docs/services/bedrock_agent_runtime.md) |
| pinpoint_email | 25 | [docs/services/pinpoint_email.md](docs/services/pinpoint_email.md) |
| connect_contact_lens | 0 | [docs/services/connect_contact_lens.md](docs/services/connect_contact_lens.md) |
| athena | 19 | [docs/services/athena.md](docs/services/athena.md) |
| iotfleetwise | 4 | [docs/services/iotfleetwise.md](docs/services/iotfleetwise.md) |
| iot_data_plane | 3 | [docs/services/iot_data_plane.md](docs/services/iot_data_plane.md) |
| bedrock_data_automation_runtime | 0 | [docs/services/bedrock_data_automation_runtime.md](docs/services/bedrock_data_automation_runtime.md) |
| entityresolution | 10 | [docs/services/entityresolution.md](docs/services/entityresolution.md) |
| forecastquery | 0 | [docs/services/forecastquery.md](docs/services/forecastquery.md) |
| detective | 6 | [docs/services/detective.md](docs/services/detective.md) |
| panorama | 11 | [docs/services/panorama.md](docs/services/panorama.md) |
| backup | 31 | [docs/services/backup.md](docs/services/backup.md) |
| mwaa | 3 | [docs/services/mwaa.md](docs/services/mwaa.md) |
| iot_jobs_data_plane | 2 | [docs/services/iot_jobs_data_plane.md](docs/services/iot_jobs_data_plane.md) |
| transcribe | 9 | [docs/services/transcribe.md](docs/services/transcribe.md) |
| cloudwatch | 18 | [docs/services/cloudwatch.md](docs/services/cloudwatch.md) |
| snow_device_management | 0 | [docs/services/snow_device_management.md](docs/services/snow_device_management.md) |
| workmailmessageflow | 1 | [docs/services/workmailmessageflow.md](docs/services/workmailmessageflow.md) |
| appconfig | 10 | [docs/services/appconfig.md](docs/services/appconfig.md) |
| lightsail | 89 | [docs/services/lightsail.md](docs/services/lightsail.md) |
| guardduty | 25 | [docs/services/guardduty.md](docs/services/guardduty.md) |
| apigatewayv2 | 31 | [docs/services/apigatewayv2.md](docs/services/apigatewayv2.md) |
| wafv2 | 19 | [docs/services/wafv2.md](docs/services/wafv2.md) |
| iotsitewise | 26 | [docs/services/iotsitewise.md](docs/services/iotsitewise.md) |
| iotthingsgraph | 9 | [docs/services/iotthingsgraph.md](docs/services/iotthingsgraph.md) |
| batch | 13 | [docs/services/batch.md](docs/services/batch.md) |
| mailmanager | 7 | [docs/services/mailmanager.md](docs/services/mailmanager.md) |
| marketplace_reporting | 0 | [docs/services/marketplace_reporting.md](docs/services/marketplace_reporting.md) |
| m2 | 1 | [docs/services/m2.md](docs/services/m2.md) |
| codedeploy | 11 | [docs/services/codedeploy.md](docs/services/codedeploy.md) |
| route53_recovery_control_config | 5 | [docs/services/route53_recovery_control_config.md](docs/services/route53_recovery_control_config.md) |
| simspaceweaver | 0 | [docs/services/simspaceweaver.md](docs/services/simspaceweaver.md) |
| resiliencehub | 14 | [docs/services/resiliencehub.md](docs/services/resiliencehub.md) |
| oam | 3 | [docs/services/oam.md](docs/services/oam.md) |
| license_manager_linux_subscriptions | 2 | [docs/services/license_manager_linux_subscriptions.md](docs/services/license_manager_linux_subscriptions.md) |
| voice_id | 5 | [docs/services/voice_id.md](docs/services/voice_id.md) |
| chime | 14 | [docs/services/chime.md](docs/services/chime.md) |
| efs | 15 | [docs/services/efs.md](docs/services/efs.md) |
| freetier | 3 | [docs/services/freetier.md](docs/services/freetier.md) |
| storage_gateway | 41 | [docs/services/storage_gateway.md](docs/services/storage_gateway.md) |
| dynamodb_streams | 3 | [docs/services/dynamodb_streams.md](docs/services/dynamodb_streams.md) |
| gamelift | 47 | [docs/services/gamelift.md](docs/services/gamelift.md) |
| inspector2 | 18 | [docs/services/inspector2.md](docs/services/inspector2.md) |
| keyspaces | 4 | [docs/services/keyspaces.md](docs/services/keyspaces.md) |
| sqs | 5 | [docs/services/sqs.md](docs/services/sqs.md) |
| ram | 7 | [docs/services/ram.md](docs/services/ram.md) |
| ssm_sap | 7 | [docs/services/ssm_sap.md](docs/services/ssm_sap.md) |
| directory_service_data | 2 | [docs/services/directory_service_data.md](docs/services/directory_service_data.md) |
| route_53_domains | 9 | [docs/services/route_53_domains.md](docs/services/route_53_domains.md) |
| bedrock_agentcore | 5 | [docs/services/bedrock_agentcore.md](docs/services/bedrock_agentcore.md) |
| trustedadvisor | 4 | [docs/services/trustedadvisor.md](docs/services/trustedadvisor.md) |
| migrationhubstrategy | 12 | [docs/services/migrationhubstrategy.md](docs/services/migrationhubstrategy.md) |
| dataexchange | 7 | [docs/services/dataexchange.md](docs/services/dataexchange.md) |
| braket | 0 | [docs/services/braket.md](docs/services/braket.md) |
| codebuild | 12 | [docs/services/codebuild.md](docs/services/codebuild.md) |
| acm | 3 | [docs/services/acm.md](docs/services/acm.md) |
| route53_recovery_cluster | 2 | [docs/services/route53_recovery_cluster.md](docs/services/route53_recovery_cluster.md) |
| lookoutequipment | 10 | [docs/services/lookoutequipment.md](docs/services/lookoutequipment.md) |
| marketplace_catalog | 3 | [docs/services/marketplace_catalog.md](docs/services/marketplace_catalog.md) |
| payment_cryptography_data | 0 | [docs/services/payment_cryptography_data.md](docs/services/payment_cryptography_data.md) |
| cloud9 | 6 | [docs/services/cloud9.md](docs/services/cloud9.md) |
| workdocs | 21 | [docs/services/workdocs.md](docs/services/workdocs.md) |
| license_manager | 13 | [docs/services/license_manager.md](docs/services/license_manager.md) |
| sts | 4 | [docs/services/sts.md](docs/services/sts.md) |
| s3vectors | 0 | [docs/services/s3vectors.md](docs/services/s3vectors.md) |
| chime_sdk_media_pipelines | 11 | [docs/services/chime_sdk_media_pipelines.md](docs/services/chime_sdk_media_pipelines.md) |
| machine_learning | 13 | [docs/services/machine_learning.md](docs/services/machine_learning.md) |
| timestream_query | 3 | [docs/services/timestream_query.md](docs/services/timestream_query.md) |
| codeguru_reviewer | 3 | [docs/services/codeguru_reviewer.md](docs/services/codeguru_reviewer.md) |
| mgn | 0 | [docs/services/mgn.md](docs/services/mgn.md) |
| evidently | 0 | [docs/services/evidently.md](docs/services/evidently.md) |
| qbusiness | 12 | [docs/services/qbusiness.md](docs/services/qbusiness.md) |
| connectcases | 0 | [docs/services/connectcases.md](docs/services/connectcases.md) |
| fsx | 22 | [docs/services/fsx.md](docs/services/fsx.md) |
| ecr | 22 | [docs/services/ecr.md](docs/services/ecr.md) |
| connectcampaignsv2 | 17 | [docs/services/connectcampaignsv2.md](docs/services/connectcampaignsv2.md) |
| rds | 68 | [docs/services/rds.md](docs/services/rds.md) |
| qapps | 7 | [docs/services/qapps.md](docs/services/qapps.md) |
| qconnect | 0 | [docs/services/qconnect.md](docs/services/qconnect.md) |
| omics | 1 | [docs/services/omics.md](docs/services/omics.md) |
| bcm_dashboards | 2 | [docs/services/bcm_dashboards.md](docs/services/bcm_dashboards.md) |
| geo_routes | 0 | [docs/services/geo_routes.md](docs/services/geo_routes.md) |
| quicksight | 68 | [docs/services/quicksight.md](docs/services/quicksight.md) |
| amp | 1 | [docs/services/amp.md](docs/services/amp.md) |
| opensearchserverless | 5 | [docs/services/opensearchserverless.md](docs/services/opensearchserverless.md) |
| emr | 16 | [docs/services/emr.md](docs/services/emr.md) |
| service_quotas | 9 | [docs/services/service_quotas.md](docs/services/service_quotas.md) |
| service_catalog_appregistry | 4 | [docs/services/service_catalog_appregistry.md](docs/services/service_catalog_appregistry.md) |
| migrationhub_config | 3 | [docs/services/migrationhub_config.md](docs/services/migrationhub_config.md) |
| iam | 37 | [docs/services/iam.md](docs/services/iam.md) |
| accessanalyzer | 8 | [docs/services/accessanalyzer.md](docs/services/accessanalyzer.md) |
| appconfigdata | 1 | [docs/services/appconfigdata.md](docs/services/appconfigdata.md) |
| route53resolver | 17 | [docs/services/route53resolver.md](docs/services/route53resolver.md) |
| s3outposts | 1 | [docs/services/s3outposts.md](docs/services/s3outposts.md) |
| kendra_ranking | 1 | [docs/services/kendra_ranking.md](docs/services/kendra_ranking.md) |
| controltower | 0 | [docs/services/controltower.md](docs/services/controltower.md) |
| arc_region_switch | 4 | [docs/services/arc_region_switch.md](docs/services/arc_region_switch.md) |
| neptune_graph | 2 | [docs/services/neptune_graph.md](docs/services/neptune_graph.md) |
| route53_recovery_readiness | 10 | [docs/services/route53_recovery_readiness.md](docs/services/route53_recovery_readiness.md) |
| greengrassv2 | 7 | [docs/services/greengrassv2.md](docs/services/greengrassv2.md) |
| migration_hub_refactor_spaces | 5 | [docs/services/migration_hub_refactor_spaces.md](docs/services/migration_hub_refactor_spaces.md) |
| cost_and_usage_report_service | 2 | [docs/services/cost_and_usage_report_service.md](docs/services/cost_and_usage_report_service.md) |
| ebs | 1 | [docs/services/ebs.md](docs/services/ebs.md) |
| appflow | 8 | [docs/services/appflow.md](docs/services/appflow.md) |
| migrationhuborchestrator | 0 | [docs/services/migrationhuborchestrator.md](docs/services/migrationhuborchestrator.md) |
| chime_sdk_identity | 7 | [docs/services/chime_sdk_identity.md](docs/services/chime_sdk_identity.md) |
| cloudfront_keyvaluestore | 3 | [docs/services/cloudfront_keyvaluestore.md](docs/services/cloudfront_keyvaluestore.md) |
| waf | 19 | [docs/services/waf.md](docs/services/waf.md) |
| greengrass | 26 | [docs/services/greengrass.md](docs/services/greengrass.md) |
| sagemaker_featurestore_runtime | 1 | [docs/services/sagemaker_featurestore_runtime.md](docs/services/sagemaker_featurestore_runtime.md) |
| inspector | 15 | [docs/services/inspector.md](docs/services/inspector.md) |
| appfabric | 4 | [docs/services/appfabric.md](docs/services/appfabric.md) |
| lex_model_building_service | 25 | [docs/services/lex_model_building_service.md](docs/services/lex_model_building_service.md) |
| serverlessapplicationrepository | 5 | [docs/services/serverlessapplicationrepository.md](docs/services/serverlessapplicationrepository.md) |
| cloudsearch_domain | 0 | [docs/services/cloudsearch_domain.md](docs/services/cloudsearch_domain.md) |
| codeguru_security | 5 | [docs/services/codeguru_security.md](docs/services/codeguru_security.md) |
| socialmessaging | 3 | [docs/services/socialmessaging.md](docs/services/socialmessaging.md) |
| geo_maps | 0 | [docs/services/geo_maps.md](docs/services/geo_maps.md) |
| kinesis | 28 | [docs/services/kinesis.md](docs/services/kinesis.md) |
| dsql | 0 | [docs/services/dsql.md](docs/services/dsql.md) |
| appintegrations | 4 | [docs/services/appintegrations.md](docs/services/appintegrations.md) |
| personalize | 19 | [docs/services/personalize.md](docs/services/personalize.md) |
| proton | 4 | [docs/services/proton.md](docs/services/proton.md) |
| cloudcontrol | 2 | [docs/services/cloudcontrol.md](docs/services/cloudcontrol.md) |
| redshift | 64 | [docs/services/redshift.md](docs/services/redshift.md) |
| geo_places | 0 | [docs/services/geo_places.md](docs/services/geo_places.md) |
| elasticsearch_service | 21 | [docs/services/elasticsearch_service.md](docs/services/elasticsearch_service.md) |
| bcm_recommended_actions | 0 | [docs/services/bcm_recommended_actions.md](docs/services/bcm_recommended_actions.md) |
| invoicing | 1 | [docs/services/invoicing.md](docs/services/invoicing.md) |
| apprunner | 8 | [docs/services/apprunner.md](docs/services/apprunner.md) |
| sns | 12 | [docs/services/sns.md](docs/services/sns.md) |
| textract | 7 | [docs/services/textract.md](docs/services/textract.md) |
| workmail | 29 | [docs/services/workmail.md](docs/services/workmail.md) |
| datazone | 26 | [docs/services/datazone.md](docs/services/datazone.md) |
| rekognition | 23 | [docs/services/rekognition.md](docs/services/rekognition.md) |
| ssm | 68 | [docs/services/ssm.md](docs/services/ssm.md) |
| medical_imaging | 4 | [docs/services/medical_imaging.md](docs/services/medical_imaging.md) |
| lex_models | 23 | [docs/services/lex_models.md](docs/services/lex_models.md) |
| support | 12 | [docs/services/support.md](docs/services/support.md) |
| signer | 4 | [docs/services/signer.md](docs/services/signer.md) |
| partnercentral_selling | 1 | [docs/services/partnercentral_selling.md](docs/services/partnercentral_selling.md) |
| comprehendmedical | 5 | [docs/services/comprehendmedical.md](docs/services/comprehendmedical.md) |
| macie2 | 29 | [docs/services/macie2.md](docs/services/macie2.md) |
| redshift_data | 4 | [docs/services/redshift_data.md](docs/services/redshift_data.md) |
| marketplace_agreement | 2 | [docs/services/marketplace_agreement.md](docs/services/marketplace_agreement.md) |
| health | 12 | [docs/services/health.md](docs/services/health.md) |
| odb | 1 | [docs/services/odb.md](docs/services/odb.md) |
| resource_groups_tagging_api | 5 | [docs/services/resource_groups_tagging_api.md](docs/services/resource_groups_tagging_api.md) |
| application_insights | 9 | [docs/services/application_insights.md](docs/services/application_insights.md) |
| timestream_write | 4 | [docs/services/timestream_write.md](docs/services/timestream_write.md) |
| pinpoint_sms | 47 | [docs/services/pinpoint_sms.md](docs/services/pinpoint_sms.md) |
| mediapackagev2 | 0 | [docs/services/mediapackagev2.md](docs/services/mediapackagev2.md) |
| ec2 | 335 | [docs/services/ec2.md](docs/services/ec2.md) |
| cleanrooms | 0 | [docs/services/cleanrooms.md](docs/services/cleanrooms.md) |
| healthlake | 3 | [docs/services/healthlake.md](docs/services/healthlake.md) |
| sfn | 9 | [docs/services/sfn.md](docs/services/sfn.md) |
| iottwinmaker | 9 | [docs/services/iottwinmaker.md](docs/services/iottwinmaker.md) |
| cloudtrail | 13 | [docs/services/cloudtrail.md](docs/services/cloudtrail.md) |
| iotdeviceadvisor | 4 | [docs/services/iotdeviceadvisor.md](docs/services/iotdeviceadvisor.md) |
| ssm_incidents | 8 | [docs/services/ssm_incidents.md](docs/services/ssm_incidents.md) |
| pcs | 0 | [docs/services/pcs.md](docs/services/pcs.md) |
| support_app | 3 | [docs/services/support_app.md](docs/services/support_app.md) |
| managedblockchain_query | 3 | [docs/services/managedblockchain_query.md](docs/services/managedblockchain_query.md) |
| iot_events_data | 2 | [docs/services/iot_events_data.md](docs/services/iot_events_data.md) |
| lex_runtime | 1 | [docs/services/lex_runtime.md](docs/services/lex_runtime.md) |
| observabilityadmin | 6 | [docs/services/observabilityadmin.md](docs/services/observabilityadmin.md) |
| applicationcostprofiler | 1 | [docs/services/applicationcostprofiler.md](docs/services/applicationcostprofiler.md) |
| billingconductor | 1 | [docs/services/billingconductor.md](docs/services/billingconductor.md) |
| artifact | 0 | [docs/services/artifact.md](docs/services/artifact.md) |
| ecr_public | 10 | [docs/services/ecr_public.md](docs/services/ecr_public.md) |
| connectparticipant | 5 | [docs/services/connectparticipant.md](docs/services/connectparticipant.md) |
| rds_data | 0 | [docs/services/rds_data.md](docs/services/rds_data.md) |
| internetmonitor | 0 | [docs/services/internetmonitor.md](docs/services/internetmonitor.md) |
| route_53 | 24 | [docs/services/route_53.md](docs/services/route_53.md) |
| bedrock_runtime | 0 | [docs/services/bedrock_runtime.md](docs/services/bedrock_runtime.md) |
| amplifybackend | 8 | [docs/services/amplifybackend.md](docs/services/amplifybackend.md) |
| marketplace_deployment | 0 | [docs/services/marketplace_deployment.md](docs/services/marketplace_deployment.md) |
| account | 0 | [docs/services/account.md](docs/services/account.md) |
| snowball | 11 | [docs/services/snowball.md](docs/services/snowball.md) |
| eventbridge | 13 | [docs/services/eventbridge.md](docs/services/eventbridge.md) |
| auto_scaling_plans | 4 | [docs/services/auto_scaling_plans.md](docs/services/auto_scaling_plans.md) |
| directory_service | 31 | [docs/services/directory_service.md](docs/services/directory_service.md) |
| mediapackage | 6 | [docs/services/mediapackage.md](docs/services/mediapackage.md) |
| ssm_quicksetup | 4 | [docs/services/ssm_quicksetup.md](docs/services/ssm_quicksetup.md) |
| s3_control | 33 | [docs/services/s3_control.md](docs/services/s3_control.md) |
| codecatalyst | 1 | [docs/services/codecatalyst.md](docs/services/codecatalyst.md) |
| notificationscontacts | 0 | [docs/services/notificationscontacts.md](docs/services/notificationscontacts.md) |
| mpa | 2 | [docs/services/mpa.md](docs/services/mpa.md) |
| ec2_instance_connect | 0 | [docs/services/ec2_instance_connect.md](docs/services/ec2_instance_connect.md) |
| sagemaker_geospatial | 0 | [docs/services/sagemaker_geospatial.md](docs/services/sagemaker_geospatial.md) |
| notifications | 0 | [docs/services/notifications.md](docs/services/notifications.md) |
| securitylake | 1 | [docs/services/securitylake.md](docs/services/securitylake.md) |
| networkmonitor | 0 | [docs/services/networkmonitor.md](docs/services/networkmonitor.md) |
| codeconnections | 8 | [docs/services/codeconnections.md](docs/services/codeconnections.md) |
| app_mesh | 0 | [docs/services/app_mesh.md](docs/services/app_mesh.md) |
| workspaces_thin_client | 3 | [docs/services/workspaces_thin_client.md](docs/services/workspaces_thin_client.md) |
| finspace_data | 8 | [docs/services/finspace_data.md](docs/services/finspace_data.md) |
| compute_optimizer | 17 | [docs/services/compute_optimizer.md](docs/services/compute_optimizer.md) |
| secrets_manager | 5 | [docs/services/secrets_manager.md](docs/services/secrets_manager.md) |
| mediastore | 5 | [docs/services/mediastore.md](docs/services/mediastore.md) |
| ecs | 23 | [docs/services/ecs.md](docs/services/ecs.md) |
| vpc_lattice | 2 | [docs/services/vpc_lattice.md](docs/services/vpc_lattice.md) |
| auto_scaling | 31 | [docs/services/auto_scaling.md](docs/services/auto_scaling.md) |
| resource_groups | 6 | [docs/services/resource_groups.md](docs/services/resource_groups.md) |
| eks | 18 | [docs/services/eks.md](docs/services/eks.md) |
| marketplace_entitlement_service | 1 | [docs/services/marketplace_entitlement_service.md](docs/services/marketplace_entitlement_service.md) |
| database_migration_service | 59 | [docs/services/database_migration_service.md](docs/services/database_migration_service.md) |
| security_ir | 0 | [docs/services/security_ir.md](docs/services/security_ir.md) |
| inspector_scan | 0 | [docs/services/inspector_scan.md](docs/services/inspector_scan.md) |
| global_accelerator | 9 | [docs/services/global_accelerator.md](docs/services/global_accelerator.md) |
| kinesis_analytics | 16 | [docs/services/kinesis_analytics.md](docs/services/kinesis_analytics.md) |
| neptunedata | 14 | [docs/services/neptunedata.md](docs/services/neptunedata.md) |
| swf | 5 | [docs/services/swf.md](docs/services/swf.md) |
| cloudwatch_logs | 43 | [docs/services/cloudwatch_logs.md](docs/services/cloudwatch_logs.md) |
| connect | 81 | [docs/services/connect.md](docs/services/connect.md) |
| glue | 91 | [docs/services/glue.md](docs/services/glue.md) |
| cognito_identity_provider | 26 | [docs/services/cognito_identity_provider.md](docs/services/cognito_identity_provider.md) |
| cloudwatch_events | 12 | [docs/services/cloudwatch_events.md](docs/services/cloudwatch_events.md) |
| cost_explorer | 27 | [docs/services/cost_explorer.md](docs/services/cost_explorer.md) |
| network_firewall | 20 | [docs/services/network_firewall.md](docs/services/network_firewall.md) |
| firehose | 4 | [docs/services/firehose.md](docs/services/firehose.md) |
| transfer | 5 | [docs/services/transfer.md](docs/services/transfer.md) |
| marketplace_metering | 0 | [docs/services/marketplace_metering.md](docs/services/marketplace_metering.md) |
| rbin | 1 | [docs/services/rbin.md](docs/services/rbin.md) |
| timestream_influxdb | 0 | [docs/services/timestream_influxdb.md](docs/services/timestream_influxdb.md) |
| iotanalytics | 6 | [docs/services/iotanalytics.md](docs/services/iotanalytics.md) |
| ivs | 8 | [docs/services/ivs.md](docs/services/ivs.md) |
| kafka | 21 | [docs/services/kafka.md](docs/services/kafka.md) |
| sesv2 | 50 | [docs/services/sesv2.md](docs/services/sesv2.md) |
| kendra | 12 | [docs/services/kendra.md](docs/services/kendra.md) |
| sagemaker_edge | 2 | [docs/services/sagemaker_edge.md](docs/services/sagemaker_edge.md) |
| launch_wizard | 0 | [docs/services/launch_wizard.md](docs/services/launch_wizard.md) |
| securityhub | 36 | [docs/services/securityhub.md](docs/services/securityhub.md) |
| finspace | 14 | [docs/services/finspace.md](docs/services/finspace.md) |
| keyspacesstreams | 3 | [docs/services/keyspacesstreams.md](docs/services/keyspacesstreams.md) |
| cleanroomsml | 0 | [docs/services/cleanroomsml.md](docs/services/cleanroomsml.md) |
| transcribe_streaming | 1 | [docs/services/transcribe_streaming.md](docs/services/transcribe_streaming.md) |
| aiops | 0 | [docs/services/aiops.md](docs/services/aiops.md) |
| service_catalog | 20 | [docs/services/service_catalog.md](docs/services/service_catalog.md) |
| databrew | 10 | [docs/services/databrew.md](docs/services/databrew.md) |
| codecommit | 36 | [docs/services/codecommit.md](docs/services/codecommit.md) |
| resource_explorer_2 | 7 | [docs/services/resource_explorer_2.md](docs/services/resource_explorer_2.md) |
| acm_pca | 7 | [docs/services/acm_pca.md](docs/services/acm_pca.md) |
| payment_cryptography | 5 | [docs/services/payment_cryptography.md](docs/services/payment_cryptography.md) |
| mq | 7 | [docs/services/mq.md](docs/services/mq.md) |
| api_gateway | 48 | [docs/services/api_gateway.md](docs/services/api_gateway.md) |
| grafana | 0 | [docs/services/grafana.md](docs/services/grafana.md) |
| glacier | 8 | [docs/services/glacier.md](docs/services/glacier.md) |
| bedrock | 0 | [docs/services/bedrock.md](docs/services/bedrock.md) |
| s3tables | 0 | [docs/services/s3tables.md](docs/services/s3tables.md) |
| ivs_realtime | 9 | [docs/services/ivs_realtime.md](docs/services/ivs_realtime.md) |
| medialive | 27 | [docs/services/medialive.md](docs/services/medialive.md) |
| backupsearch | 0 | [docs/services/backupsearch.md](docs/services/backupsearch.md) |
| networkflowmonitor | 0 | [docs/services/networkflowmonitor.md](docs/services/networkflowmonitor.md) |
| elasticache | 30 | [docs/services/elasticache.md](docs/services/elasticache.md) |
| fis | 8 | [docs/services/fis.md](docs/services/fis.md) |
| cloudhsm | 10 | [docs/services/cloudhsm.md](docs/services/cloudhsm.md) |
| cost_optimization_hub | 3 | [docs/services/cost_optimization_hub.md](docs/services/cost_optimization_hub.md) |
| synthetics | 6 | [docs/services/synthetics.md](docs/services/synthetics.md) |
| rum | 1 | [docs/services/rum.md](docs/services/rum.md) |
| emr_containers | 6 | [docs/services/emr_containers.md](docs/services/emr_containers.md) |
| sagemaker_a2i_runtime | 1 | [docs/services/sagemaker_a2i_runtime.md](docs/services/sagemaker_a2i_runtime.md) |
| ssm_contacts | 7 | [docs/services/ssm_contacts.md](docs/services/ssm_contacts.md) |
| bcm_data_exports | 3 | [docs/services/bcm_data_exports.md](docs/services/bcm_data_exports.md) |
| opensearch | 29 | [docs/services/opensearch.md](docs/services/opensearch.md) |
| dax | 9 | [docs/services/dax.md](docs/services/dax.md) |
| neptune | 29 | [docs/services/neptune.md](docs/services/neptune.md) |
| pricing | 4 | [docs/services/pricing.md](docs/services/pricing.md) |
| location | 0 | [docs/services/location.md](docs/services/location.md) |
| route53profiles | 3 | [docs/services/route53profiles.md](docs/services/route53profiles.md) |
| lambda | 1 | [docs/services/lambda.md](docs/services/lambda.md) |
| ivschat | 4 | [docs/services/ivschat.md](docs/services/ivschat.md) |
| billing | 2 | [docs/services/billing.md](docs/services/billing.md) |
| wisdom | 0 | [docs/services/wisdom.md](docs/services/wisdom.md) |
| schemas | 8 | [docs/services/schemas.md](docs/services/schemas.md) |
| bedrock_agentcore_control | 1 | [docs/services/bedrock_agentcore_control.md](docs/services/bedrock_agentcore_control.md) |
| controlcatalog | 0 | [docs/services/controlcatalog.md](docs/services/controlcatalog.md) |
| cloudsearch | 14 | [docs/services/cloudsearch.md](docs/services/cloudsearch.md) |
| deadline | 3 | [docs/services/deadline.md](docs/services/deadline.md) |
| managedblockchain | 5 | [docs/services/managedblockchain.md](docs/services/managedblockchain.md) |
| amplify | 8 | [docs/services/amplify.md](docs/services/amplify.md) |
| iotsecuretunneling | 1 | [docs/services/iotsecuretunneling.md](docs/services/iotsecuretunneling.md) |
| connectcampaigns | 10 | [docs/services/connectcampaigns.md](docs/services/connectcampaigns.md) |
| kafkaconnect | 4 | [docs/services/kafkaconnect.md](docs/services/kafkaconnect.md) |
| mediaconvert | 8 | [docs/services/mediaconvert.md](docs/services/mediaconvert.md) |
| data_pipeline | 4 | [docs/services/data_pipeline.md](docs/services/data_pipeline.md) |
| codepipeline | 14 | [docs/services/codepipeline.md](docs/services/codepipeline.md) |
| clouddirectory | 13 | [docs/services/clouddirectory.md](docs/services/clouddirectory.md) |
| amplifyuibuilder | 2 | [docs/services/amplifyuibuilder.md](docs/services/amplifyuibuilder.md) |
| rtbfabric | 0 | [docs/services/rtbfabric.md](docs/services/rtbfabric.md) |
| memorydb | 22 | [docs/services/memorydb.md](docs/services/memorydb.md) |
| iot | 64 | [docs/services/iot.md](docs/services/iot.md) |
| marketplace_commerce_analytics | 0 | [docs/services/marketplace_commerce_analytics.md](docs/services/marketplace_commerce_analytics.md) |
| frauddetector | 40 | [docs/services/frauddetector.md](docs/services/frauddetector.md) |
| bedrock_data_automation | 1 | [docs/services/bedrock_data_automation.md](docs/services/bedrock_data_automation.md) |
| elastic_load_balancing | 37 | [docs/services/elastic_load_balancing.md](docs/services/elastic_load_balancing.md) |
| verifiedpermissions | 0 | [docs/services/verifiedpermissions.md](docs/services/verifiedpermissions.md) |
| networkmanager | 37 | [docs/services/networkmanager.md](docs/services/networkmanager.md) |
| devops_guru | 13 | [docs/services/devops_guru.md](docs/services/devops_guru.md) |
| taxsettings | 6 | [docs/services/taxsettings.md](docs/services/taxsettings.md) |
| workspaces_instances | 2 | [docs/services/workspaces_instances.md](docs/services/workspaces_instances.md) |
| arc_zonal_shift | 0 | [docs/services/arc_zonal_shift.md](docs/services/arc_zonal_shift.md) |
| elastic_transcoder | 5 | [docs/services/elastic_transcoder.md](docs/services/elastic_transcoder.md) |
| fms | 11 | [docs/services/fms.md](docs/services/fms.md) |
| imagebuilder | 17 | [docs/services/imagebuilder.md](docs/services/imagebuilder.md) |
| chime_sdk | 25 | [docs/services/chime_sdk.md](docs/services/chime_sdk.md) |
| groundstation | 1 | [docs/services/groundstation.md](docs/services/groundstation.md) |
| forecast | 16 | [docs/services/forecast.md](docs/services/forecast.md) |
| appstream | 34 | [docs/services/appstream.md](docs/services/appstream.md) |
| chime_sdk_meetings | 4 | [docs/services/chime_sdk_meetings.md](docs/services/chime_sdk_meetings.md) |
| comprehend | 16 | [docs/services/comprehend.md](docs/services/comprehend.md) |
| redshift_serverless | 4 | [docs/services/redshift_serverless.md](docs/services/redshift_serverless.md) |
| pinpoint | 53 | [docs/services/pinpoint.md](docs/services/pinpoint.md) |
| pi | 5 | [docs/services/pi.md](docs/services/pi.md) |
| gameliftstreams | 2 | [docs/services/gameliftstreams.md](docs/services/gameliftstreams.md) |
| customer_profiles | 26 | [docs/services/customer_profiles.md](docs/services/customer_profiles.md) |
| workspaces | 37 | [docs/services/workspaces.md](docs/services/workspaces.md) |
| auditmanager | 23 | [docs/services/auditmanager.md](docs/services/auditmanager.md) |
| docdb | 23 | [docs/services/docdb.md](docs/services/docdb.md) |
| mturk | 14 | [docs/services/mturk.md](docs/services/mturk.md) |
| cognito_identity | 9 | [docs/services/cognito_identity.md](docs/services/cognito_identity.md) |
| dynamodb | 15 | [docs/services/dynamodb.md](docs/services/dynamodb.md) |
| codeartifact | 16 | [docs/services/codeartifact.md](docs/services/codeartifact.md) |
| organizations | 9 | [docs/services/organizations.md](docs/services/organizations.md) |
| dlm | 2 | [docs/services/dlm.md](docs/services/dlm.md) |
| sso | 1 | [docs/services/sso.md](docs/services/sso.md) |
| osis | 5 | [docs/services/osis.md](docs/services/osis.md) |
| migration_hub | 4 | [docs/services/migration_hub.md](docs/services/migration_hub.md) |
| chatbot | 12 | [docs/services/chatbot.md](docs/services/chatbot.md) |
| docdb_elastic | 3 | [docs/services/docdb_elastic.md](docs/services/docdb_elastic.md) |
| supplychain | 2 | [docs/services/supplychain.md](docs/services/supplychain.md) |
| ses | 23 | [docs/services/ses.md](docs/services/ses.md) |
| repostspace | 2 | [docs/services/repostspace.md](docs/services/repostspace.md) |
| mediastore_data | 1 | [docs/services/mediastore_data.md](docs/services/mediastore_data.md) |
| bedrock_agent | 0 | [docs/services/bedrock_agent.md](docs/services/bedrock_agent.md) |
| wellarchitected | 21 | [docs/services/wellarchitected.md](docs/services/wellarchitected.md) |
| budgets | 12 | [docs/services/budgets.md](docs/services/budgets.md) |
| mediatailor | 0 | [docs/services/mediatailor.md](docs/services/mediatailor.md) |
| appsync | 16 | [docs/services/appsync.md](docs/services/appsync.md) |
| ssm_guiconnect | 0 | [docs/services/ssm_guiconnect.md](docs/services/ssm_guiconnect.md) |
| evs | 0 | [docs/services/evs.md](docs/services/evs.md) |
| eks_auth | 0 | [docs/services/eks_auth.md](docs/services/eks_auth.md) |
| chime_sdk_messaging | 14 | [docs/services/chime_sdk_messaging.md](docs/services/chime_sdk_messaging.md) |
| mediaconnect | 0 | [docs/services/mediaconnect.md](docs/services/mediaconnect.md) |
| identitystore | 3 | [docs/services/identitystore.md](docs/services/identitystore.md) |
| bcm_pricing_calculator | 1 | [docs/services/bcm_pricing_calculator.md](docs/services/bcm_pricing_calculator.md) |
| lakeformation | 20 | [docs/services/lakeformation.md](docs/services/lakeformation.md) |
| xray | 22 | [docs/services/xray.md](docs/services/xray.md) |
| cloudfront | 41 | [docs/services/cloudfront.md](docs/services/cloudfront.md) |
| sagemaker_metrics | 0 | [docs/services/sagemaker_metrics.md](docs/services/sagemaker_metrics.md) |
| sso_oidc | 2 | [docs/services/sso_oidc.md](docs/services/sso_oidc.md) |
| sagemaker | 90 | [docs/services/sagemaker.md](docs/services/sagemaker.md) |
| codestar_connections | 8 | [docs/services/codestar_connections.md](docs/services/codestar_connections.md) |
| device_farm | 19 | [docs/services/device_farm.md](docs/services/device_farm.md) |
| translate | 3 | [docs/services/translate.md](docs/services/translate.md) |
| sagemaker_runtime | 0 | [docs/services/sagemaker_runtime.md](docs/services/sagemaker_runtime.md) |
| b2bi | 2 | [docs/services/b2bi.md](docs/services/b2bi.md) |
| savingsplans | 6 | [docs/services/savingsplans.md](docs/services/savingsplans.md) |
| pipes | 0 | [docs/services/pipes.md](docs/services/pipes.md) |
| config_service | 60 | [docs/services/config_service.md](docs/services/config_service.md) |
| codeguruprofiler | 1 | [docs/services/codeguruprofiler.md](docs/services/codeguruprofiler.md) |
| s3 | 38 | [docs/services/s3.md](docs/services/s3.md) |
| polly | 3 | [docs/services/polly.md](docs/services/polly.md) |
| cognito_sync | 7 | [docs/services/cognito_sync.md](docs/services/cognito_sync.md) |
| scheduler | 0 | [docs/services/scheduler.md](docs/services/scheduler.md) |
| pca_connector_ad | 0 | [docs/services/pca_connector_ad.md](docs/services/pca_connector_ad.md) |
| waf_regional | 20 | [docs/services/waf_regional.md](docs/services/waf_regional.md) |
| apigatewaymanagementapi | 1 | [docs/services/apigatewaymanagementapi.md](docs/services/apigatewaymanagementapi.md) |
| workspaces_web | 1 | [docs/services/workspaces_web.md](docs/services/workspaces_web.md) |
| pca_connector_scep | 0 | [docs/services/pca_connector_scep.md](docs/services/pca_connector_scep.md) |
| codestar_notifications | 2 | [docs/services/codestar_notifications.md](docs/services/codestar_notifications.md) |
| direct_connect | 30 | [docs/services/direct_connect.md](docs/services/direct_connect.md) |
| shield | 9 | [docs/services/shield.md](docs/services/shield.md) |
| application_signals | 2 | [docs/services/application_signals.md](docs/services/application_signals.md) |
| iot_managed_integrations | 1 | [docs/services/iot_managed_integrations.md](docs/services/iot_managed_integrations.md) |
| iot_wireless | 29 | [docs/services/iot_wireless.md](docs/services/iot_wireless.md) |
| iot_events | 6 | [docs/services/iot_events.md](docs/services/iot_events.md) |
| backup_gateway | 0 | [docs/services/backup_gateway.md](docs/services/backup_gateway.md) |
| sso_admin | 19 | [docs/services/sso_admin.md](docs/services/sso_admin.md) |
| elastic_beanstalk | 21 | [docs/services/elastic_beanstalk.md](docs/services/elastic_beanstalk.md) |
| drs | 2 | [docs/services/drs.md](docs/services/drs.md) |
| personalize_runtime | 3 | [docs/services/personalize_runtime.md](docs/services/personalize_runtime.md) |
| outposts | 11 | [docs/services/outposts.md](docs/services/outposts.md) |
| license_manager_user_subscriptions | 2 | [docs/services/license_manager_user_subscriptions.md](docs/services/license_manager_user_subscriptions.md) |
| cloudtrail_data | 1 | [docs/services/cloudtrail_data.md](docs/services/cloudtrail_data.md) |
| lex_runtime_service | 1 | [docs/services/lex_runtime_service.md](docs/services/lex_runtime_service.md) |

---

## Documentation

- 📖 [Installation Guide](docs/installation.md)
- 🚀 [Getting Started](docs/getting-started.md)
- 📚 [Service Documentation](docs/services/)

---

## Building from Source

```bash
git clone https://github.com/YOUR_ORG/hemmer-provider-aws.git
cd hemmer-provider-aws
cargo build --release
```

The binary will be at: `target/release/libhemmer_aws_provider.{so,dylib,dll}`

---

## Creating a Release

This provider includes automated release workflows.

1. Update version in `Cargo.toml`
2. Commit and push changes
3. Create and push a tag: `git tag v0.2.0 && git push origin v0.2.0`
4. GitHub Actions will automatically build and publish the release

📖 See [Release Workflow](.github/workflows/release.yml) for details

---

## Generated Code

This provider was automatically generated using the Hemmer Provider Generator.

- **Generator**: [hemmer-provider-generator](https://github.com/hemmer-io/hemmer-provider-generator) v0.3.3
- **Provider**: AWS
- **SDK Version**: v1
- **Services**: 394
- **Total Resources**: 4616
- **Generated**: 2025-11-02

To regenerate this provider:

```bash
hemmer-provider-generator generate-unified \
  --provider aws \
  --spec-dir /path/to/aws-sdk \
  --output .
```

---

## License

Apache-2.0
