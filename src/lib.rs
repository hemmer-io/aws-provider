//! Aws Provider for Hemmer
//!
//! Auto-generated unified provider from aws SDK version v1
//!
//! This provider includes multiple services:
//! - oam
//! - forecast
//! - bedrock
//! - codebuild
//! - iotdeviceadvisor
//! - auto
//! - database
//! - codeguruprofiler
//! - kinesis
//! - pinpoint
//! - chime
//! - iottwinmaker
//! - organizations
//! - networkflowmonitor
//! - shield
//! - ssm
//! - arc
//! - workspaces
//! - partnercentral
//! - geo
//! - signer
//! - marketplace
//! - lakeformation
//! - pcs
//! - kinesis
//! - mediaconnect
//! - secrets
//! - mwaa
//! - kms
//! - quicksight
//! - workmail
//! - eventbridge
//! - frauddetector
//! - cloudwatch
//! - cloudhsm
//! - cloudsearch
//! - lookoutequipment
//! - iot
//! - securitylake
//! - cloudwatch
//! - glue
//! - application
//! - personalize
//! - voice
//! - s3
//! - appintegrations
//! - lex
//! - ssm
//! - chime
//! - sesv2
//! - bedrock
//! - emr
//! - controltower
//! - resource
//! - s3
//! - rtbfabric
//! - personalize
//! - marketplace
//! - directory
//! - ssm
//! - outposts
//! - workdocs
//! - networkmanager
//! - kinesis
//! - socialmessaging
//! - bedrock
//! - omics
//! - bcm
//! - mediapackage
//! - sagemaker
//! - config
//! - cloudwatch
//! - medialive
//! - backup
//! - connect
//! - mediaconvert
//! - sns
//! - evs
//! - datasync
//! - greengrassv2
//! - cleanroomsml
//! - neptunedata
//! - acm
//! - service
//! - b2bi
//! - iotanalytics
//! - lex
//! - inspector2
//! - groundstation
//! - ecr
//! - fis
//! - proton
//! - api
//! - cloudhsm
//! - mpa
//! - osis
//! - memorydb
//! - inspector
//! - translate
//! - mailmanager
//! - neptune
//! - chatbot
//! - fms
//! - qapps
//! - customer
//! - geo
//! - route
//! - service
//! - qbusiness
//! - synthetics
//! - codecatalyst
//! - keyspacesstreams
//! - storage
//! - elastic
//! - bcm
//! - iotsecuretunneling
//! - cloudfront
//! - bedrock
//! - location
//! - wafv2
//! - opensearch
//! - iotthingsgraph
//! - security
//! - repostspace
//! - health
//! - workmailmessageflow
//! - mediastore
//! - ec2
//! - comprehendmedical
//! - iotfleetwise
//! - route53profiles
//! - application
//! - resource
//! - accessanalyzer
//! - glacier
//! - lightsail
//! - rum
//! - direct
//! - elastic
//! - imagebuilder
//! - simspaceweaver
//! - iot
//! - freetier
//! - bedrock
//! - neptune
//! - transfer
//! - deadline
//! - braket
//! - verifiedpermissions
//! - scheduler
//! - waf
//! - marketplace
//! - ecr
//! - cost
//! - kinesis
//! - dynamodb
//! - resiliencehub
//! - macie2
//! - entityresolution
//! - s3outposts
//! - grafana
//! - kinesis
//! - eks
//! - rbin
//! - service
//! - ecs
//! - marketplace
//! - timestream
//! - qconnect
//! - cloudsearch
//! - bcm
//! - appfabric
//! - route53resolver
//! - marketplace
//! - workspaces
//! - gameliftstreams
//! - taxsettings
//! - pinpoint
//! - fsx
//! - codepipeline
//! - schemas
//! - emr
//! - sqs
//! - license
//! - route53
//! - migrationhuborchestrator
//! - iot
//! - sso
//! - codestar
//! - ebs
//! - aiops
//! - amplify
//! - cloudcontrol
//! - wellarchitected
//! - route
//! - ssm
//! - bedrock
//! - ivs
//! - migrationhub
//! - redshift
//! - inspector
//! - connectcases
//! - kinesis
//! - pca
//! - appflow
//! - gamelift
//! - cloudtrail
//! - resource
//! - supplychain
//! - timestream
//! - pipes
//! - evidently
//! - codeguru
//! - cost
//! - amplifyuibuilder
//! - route53
//! - vpc
//! - managedblockchain
//! - redshift
//! - mediatailor
//! - mediapackagev2
//! - pi
//! - appconfig
//! - networkmonitor
//! - network
//! - connectparticipant
//! - mgn
//! - sagemaker
//! - applicationcostprofiler
//! - keyspaces
//! - iam
//! - data
//! - odb
//! - chime
//! - mediastore
//! - iot
//! - cloud9
//! - wisdom
//! - sagemaker
//! - sso
//! - auditmanager
//! - snowball
//! - migration
//! - identitystore
//! - elastic
//! - connectcampaigns
//! - textract
//! - compute
//! - s3tables
//! - eks
//! - support
//! - mturk
//! - apigatewayv2
//! - cognito
//! - bedrock
//! - pinpoint
//! - amp
//! - drs
//! - payment
//! - kafkaconnect
//! - kafka
//! - databrew
//! - support
//! - codedeploy
//! - batch
//! - savingsplans
//! - bedrock
//! - directory
//! - workspaces
//! - chime
//! - migrationhubstrategy
//! - timestream
//! - codeguru
//! - appsync
//! - dlm
//! - iot
//! - global
//! - amplifybackend
//! - datazone
//! - connectcampaignsv2
//! - billingconductor
//! - budgets
//! - cloudtrail
//! - geo
//! - m2
//! - pinpoint
//! - lex
//! - finspace
//! - detective
//! - lambda
//! - kinesis
//! - panorama
//! - iot
//! - app
//! - managedblockchain
//! - waf
//! - ivs
//! - devops
//! - cost
//! - mq
//! - route53
//! - internetmonitor
//! - license
//! - codestar
//! - artifact
//! - ssm
//! - iotsitewise
//! - serverlessapplicationrepository
//! - ssm
//! - docdb
//! - sagemaker
//! - license
//! - sagemaker
//! - personalize
//! - clouddirectory
//! - chime
//! - notificationscontacts
//! - backupsearch
//! - dynamodb
//! - iot
//! - cognito
//! - codeartifact
//! - arc
//! - elastic
//! - guardduty
//! - cleanrooms
//! - trustedadvisor
//! - sagemaker
//! - dax
//! - docdb
//! - firehose
//! - ivschat
//! - ses
//! - bcm
//! - application
//! - device
//! - account
//! - launch
//! - finspace
//! - appconfigdata
//! - controlcatalog
//! - greengrass
//! - kendra
//! - snow
//! - securityhub
//! - s3vectors
//! - workspaces
//! - backup
//! - opensearchserverless
//! - cloudformation
//! - kendra
//! - connect
//! - machine
//! - elasticache
//! - sfn
//! - sso
//! - auto
//! - comprehend
//! - rds
//! - chime
//! - rekognition
//! - appstream
//! - polly
//! - invoicing
//! - rds
//! - pricing
//! - swf
//! - cloudfront
//! - marketplace
//! - medical
//! - transcribe
//! - observabilityadmin
//! - notifications
//! - codecommit
//! - lex
//! - pca
//! - forecastquery
//! - healthlake
//! - rolesanywhere
//! - marketplace
//! - billing
//! - emr
//! - apigatewaymanagementapi
//! - xray
//! - transcribe
//! - ram
//! - codeconnections
//! - efs
//! - migration
//! - elasticsearch
//! - cognito
//! - payment
//! - dataexchange
//! - sts
//! - sagemaker
//! - kinesis
//! - acm
//! - athena
//! - dsql
//! - mediapackage
//! - tnb
//! - ec2
//! - apprunner
//! - redshift


pub mod oam;
pub mod forecast;
pub mod bedrock;
pub mod codebuild;
pub mod iotdeviceadvisor;
pub mod auto;
pub mod database;
pub mod codeguruprofiler;
pub mod kinesis;
pub mod pinpoint;
pub mod chime;
pub mod iottwinmaker;
pub mod organizations;
pub mod networkflowmonitor;
pub mod shield;
pub mod ssm;
pub mod arc;
pub mod workspaces;
pub mod partnercentral;
pub mod geo;
pub mod signer;
pub mod marketplace;
pub mod lakeformation;
pub mod pcs;
pub mod kinesis;
pub mod mediaconnect;
pub mod secrets;
pub mod mwaa;
pub mod kms;
pub mod quicksight;
pub mod workmail;
pub mod eventbridge;
pub mod frauddetector;
pub mod cloudwatch;
pub mod cloudhsm;
pub mod cloudsearch;
pub mod lookoutequipment;
pub mod iot;
pub mod securitylake;
pub mod cloudwatch;
pub mod glue;
pub mod application;
pub mod personalize;
pub mod voice;
pub mod s3;
pub mod appintegrations;
pub mod lex;
pub mod ssm;
pub mod chime;
pub mod sesv2;
pub mod bedrock;
pub mod emr;
pub mod controltower;
pub mod resource;
pub mod s3;
pub mod rtbfabric;
pub mod personalize;
pub mod marketplace;
pub mod directory;
pub mod ssm;
pub mod outposts;
pub mod workdocs;
pub mod networkmanager;
pub mod kinesis;
pub mod socialmessaging;
pub mod bedrock;
pub mod omics;
pub mod bcm;
pub mod mediapackage;
pub mod sagemaker;
pub mod config;
pub mod cloudwatch;
pub mod medialive;
pub mod backup;
pub mod connect;
pub mod mediaconvert;
pub mod sns;
pub mod evs;
pub mod datasync;
pub mod greengrassv2;
pub mod cleanroomsml;
pub mod neptunedata;
pub mod acm;
pub mod service;
pub mod b2bi;
pub mod iotanalytics;
pub mod lex;
pub mod inspector2;
pub mod groundstation;
pub mod ecr;
pub mod fis;
pub mod proton;
pub mod api;
pub mod cloudhsm;
pub mod mpa;
pub mod osis;
pub mod memorydb;
pub mod inspector;
pub mod translate;
pub mod mailmanager;
pub mod neptune;
pub mod chatbot;
pub mod fms;
pub mod qapps;
pub mod customer;
pub mod geo;
pub mod route;
pub mod service;
pub mod qbusiness;
pub mod synthetics;
pub mod codecatalyst;
pub mod keyspacesstreams;
pub mod storage;
pub mod elastic;
pub mod bcm;
pub mod iotsecuretunneling;
pub mod cloudfront;
pub mod bedrock;
pub mod location;
pub mod wafv2;
pub mod opensearch;
pub mod iotthingsgraph;
pub mod security;
pub mod repostspace;
pub mod health;
pub mod workmailmessageflow;
pub mod mediastore;
pub mod ec2;
pub mod comprehendmedical;
pub mod iotfleetwise;
pub mod route53profiles;
pub mod application;
pub mod resource;
pub mod accessanalyzer;
pub mod glacier;
pub mod lightsail;
pub mod rum;
pub mod direct;
pub mod elastic;
pub mod imagebuilder;
pub mod simspaceweaver;
pub mod iot;
pub mod freetier;
pub mod bedrock;
pub mod neptune;
pub mod transfer;
pub mod deadline;
pub mod braket;
pub mod verifiedpermissions;
pub mod scheduler;
pub mod waf;
pub mod marketplace;
pub mod ecr;
pub mod cost;
pub mod kinesis;
pub mod dynamodb;
pub mod resiliencehub;
pub mod macie2;
pub mod entityresolution;
pub mod s3outposts;
pub mod grafana;
pub mod kinesis;
pub mod eks;
pub mod rbin;
pub mod service;
pub mod ecs;
pub mod marketplace;
pub mod timestream;
pub mod qconnect;
pub mod cloudsearch;
pub mod bcm;
pub mod appfabric;
pub mod route53resolver;
pub mod marketplace;
pub mod workspaces;
pub mod gameliftstreams;
pub mod taxsettings;
pub mod pinpoint;
pub mod fsx;
pub mod codepipeline;
pub mod schemas;
pub mod emr;
pub mod sqs;
pub mod license;
pub mod route53;
pub mod migrationhuborchestrator;
pub mod iot;
pub mod sso;
pub mod codestar;
pub mod ebs;
pub mod aiops;
pub mod amplify;
pub mod cloudcontrol;
pub mod wellarchitected;
pub mod route;
pub mod ssm;
pub mod bedrock;
pub mod ivs;
pub mod migrationhub;
pub mod redshift;
pub mod inspector;
pub mod connectcases;
pub mod kinesis;
pub mod pca;
pub mod appflow;
pub mod gamelift;
pub mod cloudtrail;
pub mod resource;
pub mod supplychain;
pub mod timestream;
pub mod pipes;
pub mod evidently;
pub mod codeguru;
pub mod cost;
pub mod amplifyuibuilder;
pub mod route53;
pub mod vpc;
pub mod managedblockchain;
pub mod redshift;
pub mod mediatailor;
pub mod mediapackagev2;
pub mod pi;
pub mod appconfig;
pub mod networkmonitor;
pub mod network;
pub mod connectparticipant;
pub mod mgn;
pub mod sagemaker;
pub mod applicationcostprofiler;
pub mod keyspaces;
pub mod iam;
pub mod data;
pub mod odb;
pub mod chime;
pub mod mediastore;
pub mod iot;
pub mod cloud9;
pub mod wisdom;
pub mod sagemaker;
pub mod sso;
pub mod auditmanager;
pub mod snowball;
pub mod migration;
pub mod identitystore;
pub mod elastic;
pub mod connectcampaigns;
pub mod textract;
pub mod compute;
pub mod s3tables;
pub mod eks;
pub mod support;
pub mod mturk;
pub mod apigatewayv2;
pub mod cognito;
pub mod bedrock;
pub mod pinpoint;
pub mod amp;
pub mod drs;
pub mod payment;
pub mod kafkaconnect;
pub mod kafka;
pub mod databrew;
pub mod support;
pub mod codedeploy;
pub mod batch;
pub mod savingsplans;
pub mod bedrock;
pub mod directory;
pub mod workspaces;
pub mod chime;
pub mod migrationhubstrategy;
pub mod timestream;
pub mod codeguru;
pub mod appsync;
pub mod dlm;
pub mod iot;
pub mod global;
pub mod amplifybackend;
pub mod datazone;
pub mod connectcampaignsv2;
pub mod billingconductor;
pub mod budgets;
pub mod cloudtrail;
pub mod geo;
pub mod m2;
pub mod pinpoint;
pub mod lex;
pub mod finspace;
pub mod detective;
pub mod lambda;
pub mod kinesis;
pub mod panorama;
pub mod iot;
pub mod app;
pub mod managedblockchain;
pub mod waf;
pub mod ivs;
pub mod devops;
pub mod cost;
pub mod mq;
pub mod route53;
pub mod internetmonitor;
pub mod license;
pub mod codestar;
pub mod artifact;
pub mod ssm;
pub mod iotsitewise;
pub mod serverlessapplicationrepository;
pub mod ssm;
pub mod docdb;
pub mod sagemaker;
pub mod license;
pub mod sagemaker;
pub mod personalize;
pub mod clouddirectory;
pub mod chime;
pub mod notificationscontacts;
pub mod backupsearch;
pub mod dynamodb;
pub mod iot;
pub mod cognito;
pub mod codeartifact;
pub mod arc;
pub mod elastic;
pub mod guardduty;
pub mod cleanrooms;
pub mod trustedadvisor;
pub mod sagemaker;
pub mod dax;
pub mod docdb;
pub mod firehose;
pub mod ivschat;
pub mod ses;
pub mod bcm;
pub mod application;
pub mod device;
pub mod account;
pub mod launch;
pub mod finspace;
pub mod appconfigdata;
pub mod controlcatalog;
pub mod greengrass;
pub mod kendra;
pub mod snow;
pub mod securityhub;
pub mod s3vectors;
pub mod workspaces;
pub mod backup;
pub mod opensearchserverless;
pub mod cloudformation;
pub mod kendra;
pub mod connect;
pub mod machine;
pub mod elasticache;
pub mod sfn;
pub mod sso;
pub mod auto;
pub mod comprehend;
pub mod rds;
pub mod chime;
pub mod rekognition;
pub mod appstream;
pub mod polly;
pub mod invoicing;
pub mod rds;
pub mod pricing;
pub mod swf;
pub mod cloudfront;
pub mod marketplace;
pub mod medical;
pub mod transcribe;
pub mod observabilityadmin;
pub mod notifications;
pub mod codecommit;
pub mod lex;
pub mod pca;
pub mod forecastquery;
pub mod healthlake;
pub mod rolesanywhere;
pub mod marketplace;
pub mod billing;
pub mod emr;
pub mod apigatewaymanagementapi;
pub mod xray;
pub mod transcribe;
pub mod ram;
pub mod codeconnections;
pub mod efs;
pub mod migration;
pub mod elasticsearch;
pub mod cognito;
pub mod payment;
pub mod dataexchange;
pub mod sts;
pub mod sagemaker;
pub mod kinesis;
pub mod acm;
pub mod athena;
pub mod dsql;
pub mod mediapackage;
pub mod tnb;
pub mod ec2;
pub mod apprunner;
pub mod redshift;


use thiserror::Error;

/// Provider error types
#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("SDK error: {0}")]
    SdkError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for provider operations
pub type Result<T> = std::result::Result<T, ProviderError>;

/// Unified provider client for Aws
pub struct AwsProvider {
    oam_client: aws_sdk_oam::Client,
    forecast_client: aws_sdk_forecast::Client,
    bedrock_client: aws_sdk_bedrock::Client,
    codebuild_client: aws_sdk_codebuild::Client,
    iotdeviceadvisor_client: aws_sdk_iotdeviceadvisor::Client,
    auto_client: aws_sdk_auto::Client,
    database_client: aws_sdk_database::Client,
    codeguruprofiler_client: aws_sdk_codeguruprofiler::Client,
    kinesis_client: aws_sdk_kinesis::Client,
    pinpoint_client: aws_sdk_pinpoint::Client,
    chime_client: aws_sdk_chime::Client,
    iottwinmaker_client: aws_sdk_iottwinmaker::Client,
    organizations_client: aws_sdk_organizations::Client,
    networkflowmonitor_client: aws_sdk_networkflowmonitor::Client,
    shield_client: aws_sdk_shield::Client,
    ssm_client: aws_sdk_ssm::Client,
    arc_client: aws_sdk_arc::Client,
    workspaces_client: aws_sdk_workspaces::Client,
    partnercentral_client: aws_sdk_partnercentral::Client,
    geo_client: aws_sdk_geo::Client,
    signer_client: aws_sdk_signer::Client,
    marketplace_client: aws_sdk_marketplace::Client,
    lakeformation_client: aws_sdk_lakeformation::Client,
    pcs_client: aws_sdk_pcs::Client,
    kinesis_client: aws_sdk_kinesis::Client,
    mediaconnect_client: aws_sdk_mediaconnect::Client,
    secrets_client: aws_sdk_secrets::Client,
    mwaa_client: aws_sdk_mwaa::Client,
    kms_client: aws_sdk_kms::Client,
    quicksight_client: aws_sdk_quicksight::Client,
    workmail_client: aws_sdk_workmail::Client,
    eventbridge_client: aws_sdk_eventbridge::Client,
    frauddetector_client: aws_sdk_frauddetector::Client,
    cloudwatch_client: aws_sdk_cloudwatch::Client,
    cloudhsm_client: aws_sdk_cloudhsm::Client,
    cloudsearch_client: aws_sdk_cloudsearch::Client,
    lookoutequipment_client: aws_sdk_lookoutequipment::Client,
    iot_client: aws_sdk_iot::Client,
    securitylake_client: aws_sdk_securitylake::Client,
    cloudwatch_client: aws_sdk_cloudwatch::Client,
    glue_client: aws_sdk_glue::Client,
    application_client: aws_sdk_application::Client,
    personalize_client: aws_sdk_personalize::Client,
    voice_client: aws_sdk_voice::Client,
    s3_client: aws_sdk_s3::Client,
    appintegrations_client: aws_sdk_appintegrations::Client,
    lex_client: aws_sdk_lex::Client,
    ssm_client: aws_sdk_ssm::Client,
    chime_client: aws_sdk_chime::Client,
    sesv2_client: aws_sdk_sesv2::Client,
    bedrock_client: aws_sdk_bedrock::Client,
    emr_client: aws_sdk_emr::Client,
    controltower_client: aws_sdk_controltower::Client,
    resource_client: aws_sdk_resource::Client,
    s3_client: aws_sdk_s3::Client,
    rtbfabric_client: aws_sdk_rtbfabric::Client,
    personalize_client: aws_sdk_personalize::Client,
    marketplace_client: aws_sdk_marketplace::Client,
    directory_client: aws_sdk_directory::Client,
    ssm_client: aws_sdk_ssm::Client,
    outposts_client: aws_sdk_outposts::Client,
    workdocs_client: aws_sdk_workdocs::Client,
    networkmanager_client: aws_sdk_networkmanager::Client,
    kinesis_client: aws_sdk_kinesis::Client,
    socialmessaging_client: aws_sdk_socialmessaging::Client,
    bedrock_client: aws_sdk_bedrock::Client,
    omics_client: aws_sdk_omics::Client,
    bcm_client: aws_sdk_bcm::Client,
    mediapackage_client: aws_sdk_mediapackage::Client,
    sagemaker_client: aws_sdk_sagemaker::Client,
    config_client: aws_sdk_config::Client,
    cloudwatch_client: aws_sdk_cloudwatch::Client,
    medialive_client: aws_sdk_medialive::Client,
    backup_client: aws_sdk_backup::Client,
    connect_client: aws_sdk_connect::Client,
    mediaconvert_client: aws_sdk_mediaconvert::Client,
    sns_client: aws_sdk_sns::Client,
    evs_client: aws_sdk_evs::Client,
    datasync_client: aws_sdk_datasync::Client,
    greengrassv2_client: aws_sdk_greengrassv2::Client,
    cleanroomsml_client: aws_sdk_cleanroomsml::Client,
    neptunedata_client: aws_sdk_neptunedata::Client,
    acm_client: aws_sdk_acm::Client,
    service_client: aws_sdk_service::Client,
    b2bi_client: aws_sdk_b2bi::Client,
    iotanalytics_client: aws_sdk_iotanalytics::Client,
    lex_client: aws_sdk_lex::Client,
    inspector2_client: aws_sdk_inspector2::Client,
    groundstation_client: aws_sdk_groundstation::Client,
    ecr_client: aws_sdk_ecr::Client,
    fis_client: aws_sdk_fis::Client,
    proton_client: aws_sdk_proton::Client,
    api_client: aws_sdk_api::Client,
    cloudhsm_client: aws_sdk_cloudhsm::Client,
    mpa_client: aws_sdk_mpa::Client,
    osis_client: aws_sdk_osis::Client,
    memorydb_client: aws_sdk_memorydb::Client,
    inspector_client: aws_sdk_inspector::Client,
    translate_client: aws_sdk_translate::Client,
    mailmanager_client: aws_sdk_mailmanager::Client,
    neptune_client: aws_sdk_neptune::Client,
    chatbot_client: aws_sdk_chatbot::Client,
    fms_client: aws_sdk_fms::Client,
    qapps_client: aws_sdk_qapps::Client,
    customer_client: aws_sdk_customer::Client,
    geo_client: aws_sdk_geo::Client,
    route_client: aws_sdk_route::Client,
    service_client: aws_sdk_service::Client,
    qbusiness_client: aws_sdk_qbusiness::Client,
    synthetics_client: aws_sdk_synthetics::Client,
    codecatalyst_client: aws_sdk_codecatalyst::Client,
    keyspacesstreams_client: aws_sdk_keyspacesstreams::Client,
    storage_client: aws_sdk_storage::Client,
    elastic_client: aws_sdk_elastic::Client,
    bcm_client: aws_sdk_bcm::Client,
    iotsecuretunneling_client: aws_sdk_iotsecuretunneling::Client,
    cloudfront_client: aws_sdk_cloudfront::Client,
    bedrock_client: aws_sdk_bedrock::Client,
    location_client: aws_sdk_location::Client,
    wafv2_client: aws_sdk_wafv2::Client,
    opensearch_client: aws_sdk_opensearch::Client,
    iotthingsgraph_client: aws_sdk_iotthingsgraph::Client,
    security_client: aws_sdk_security::Client,
    repostspace_client: aws_sdk_repostspace::Client,
    health_client: aws_sdk_health::Client,
    workmailmessageflow_client: aws_sdk_workmailmessageflow::Client,
    mediastore_client: aws_sdk_mediastore::Client,
    ec2_client: aws_sdk_ec2::Client,
    comprehendmedical_client: aws_sdk_comprehendmedical::Client,
    iotfleetwise_client: aws_sdk_iotfleetwise::Client,
    route53profiles_client: aws_sdk_route53profiles::Client,
    application_client: aws_sdk_application::Client,
    resource_client: aws_sdk_resource::Client,
    accessanalyzer_client: aws_sdk_accessanalyzer::Client,
    glacier_client: aws_sdk_glacier::Client,
    lightsail_client: aws_sdk_lightsail::Client,
    rum_client: aws_sdk_rum::Client,
    direct_client: aws_sdk_direct::Client,
    elastic_client: aws_sdk_elastic::Client,
    imagebuilder_client: aws_sdk_imagebuilder::Client,
    simspaceweaver_client: aws_sdk_simspaceweaver::Client,
    iot_client: aws_sdk_iot::Client,
    freetier_client: aws_sdk_freetier::Client,
    bedrock_client: aws_sdk_bedrock::Client,
    neptune_client: aws_sdk_neptune::Client,
    transfer_client: aws_sdk_transfer::Client,
    deadline_client: aws_sdk_deadline::Client,
    braket_client: aws_sdk_braket::Client,
    verifiedpermissions_client: aws_sdk_verifiedpermissions::Client,
    scheduler_client: aws_sdk_scheduler::Client,
    waf_client: aws_sdk_waf::Client,
    marketplace_client: aws_sdk_marketplace::Client,
    ecr_client: aws_sdk_ecr::Client,
    cost_client: aws_sdk_cost::Client,
    kinesis_client: aws_sdk_kinesis::Client,
    dynamodb_client: aws_sdk_dynamodb::Client,
    resiliencehub_client: aws_sdk_resiliencehub::Client,
    macie2_client: aws_sdk_macie2::Client,
    entityresolution_client: aws_sdk_entityresolution::Client,
    s3outposts_client: aws_sdk_s3outposts::Client,
    grafana_client: aws_sdk_grafana::Client,
    kinesis_client: aws_sdk_kinesis::Client,
    eks_client: aws_sdk_eks::Client,
    rbin_client: aws_sdk_rbin::Client,
    service_client: aws_sdk_service::Client,
    ecs_client: aws_sdk_ecs::Client,
    marketplace_client: aws_sdk_marketplace::Client,
    timestream_client: aws_sdk_timestream::Client,
    qconnect_client: aws_sdk_qconnect::Client,
    cloudsearch_client: aws_sdk_cloudsearch::Client,
    bcm_client: aws_sdk_bcm::Client,
    appfabric_client: aws_sdk_appfabric::Client,
    route53resolver_client: aws_sdk_route53resolver::Client,
    marketplace_client: aws_sdk_marketplace::Client,
    workspaces_client: aws_sdk_workspaces::Client,
    gameliftstreams_client: aws_sdk_gameliftstreams::Client,
    taxsettings_client: aws_sdk_taxsettings::Client,
    pinpoint_client: aws_sdk_pinpoint::Client,
    fsx_client: aws_sdk_fsx::Client,
    codepipeline_client: aws_sdk_codepipeline::Client,
    schemas_client: aws_sdk_schemas::Client,
    emr_client: aws_sdk_emr::Client,
    sqs_client: aws_sdk_sqs::Client,
    license_client: aws_sdk_license::Client,
    route53_client: aws_sdk_route53::Client,
    migrationhuborchestrator_client: aws_sdk_migrationhuborchestrator::Client,
    iot_client: aws_sdk_iot::Client,
    sso_client: aws_sdk_sso::Client,
    codestar_client: aws_sdk_codestar::Client,
    ebs_client: aws_sdk_ebs::Client,
    aiops_client: aws_sdk_aiops::Client,
    amplify_client: aws_sdk_amplify::Client,
    cloudcontrol_client: aws_sdk_cloudcontrol::Client,
    wellarchitected_client: aws_sdk_wellarchitected::Client,
    route_client: aws_sdk_route::Client,
    ssm_client: aws_sdk_ssm::Client,
    bedrock_client: aws_sdk_bedrock::Client,
    ivs_client: aws_sdk_ivs::Client,
    migrationhub_client: aws_sdk_migrationhub::Client,
    redshift_client: aws_sdk_redshift::Client,
    inspector_client: aws_sdk_inspector::Client,
    connectcases_client: aws_sdk_connectcases::Client,
    kinesis_client: aws_sdk_kinesis::Client,
    pca_client: aws_sdk_pca::Client,
    appflow_client: aws_sdk_appflow::Client,
    gamelift_client: aws_sdk_gamelift::Client,
    cloudtrail_client: aws_sdk_cloudtrail::Client,
    resource_client: aws_sdk_resource::Client,
    supplychain_client: aws_sdk_supplychain::Client,
    timestream_client: aws_sdk_timestream::Client,
    pipes_client: aws_sdk_pipes::Client,
    evidently_client: aws_sdk_evidently::Client,
    codeguru_client: aws_sdk_codeguru::Client,
    cost_client: aws_sdk_cost::Client,
    amplifyuibuilder_client: aws_sdk_amplifyuibuilder::Client,
    route53_client: aws_sdk_route53::Client,
    vpc_client: aws_sdk_vpc::Client,
    managedblockchain_client: aws_sdk_managedblockchain::Client,
    redshift_client: aws_sdk_redshift::Client,
    mediatailor_client: aws_sdk_mediatailor::Client,
    mediapackagev2_client: aws_sdk_mediapackagev2::Client,
    pi_client: aws_sdk_pi::Client,
    appconfig_client: aws_sdk_appconfig::Client,
    networkmonitor_client: aws_sdk_networkmonitor::Client,
    network_client: aws_sdk_network::Client,
    connectparticipant_client: aws_sdk_connectparticipant::Client,
    mgn_client: aws_sdk_mgn::Client,
    sagemaker_client: aws_sdk_sagemaker::Client,
    applicationcostprofiler_client: aws_sdk_applicationcostprofiler::Client,
    keyspaces_client: aws_sdk_keyspaces::Client,
    iam_client: aws_sdk_iam::Client,
    data_client: aws_sdk_data::Client,
    odb_client: aws_sdk_odb::Client,
    chime_client: aws_sdk_chime::Client,
    mediastore_client: aws_sdk_mediastore::Client,
    iot_client: aws_sdk_iot::Client,
    cloud9_client: aws_sdk_cloud9::Client,
    wisdom_client: aws_sdk_wisdom::Client,
    sagemaker_client: aws_sdk_sagemaker::Client,
    sso_client: aws_sdk_sso::Client,
    auditmanager_client: aws_sdk_auditmanager::Client,
    snowball_client: aws_sdk_snowball::Client,
    migration_client: aws_sdk_migration::Client,
    identitystore_client: aws_sdk_identitystore::Client,
    elastic_client: aws_sdk_elastic::Client,
    connectcampaigns_client: aws_sdk_connectcampaigns::Client,
    textract_client: aws_sdk_textract::Client,
    compute_client: aws_sdk_compute::Client,
    s3tables_client: aws_sdk_s3tables::Client,
    eks_client: aws_sdk_eks::Client,
    support_client: aws_sdk_support::Client,
    mturk_client: aws_sdk_mturk::Client,
    apigatewayv2_client: aws_sdk_apigatewayv2::Client,
    cognito_client: aws_sdk_cognito::Client,
    bedrock_client: aws_sdk_bedrock::Client,
    pinpoint_client: aws_sdk_pinpoint::Client,
    amp_client: aws_sdk_amp::Client,
    drs_client: aws_sdk_drs::Client,
    payment_client: aws_sdk_payment::Client,
    kafkaconnect_client: aws_sdk_kafkaconnect::Client,
    kafka_client: aws_sdk_kafka::Client,
    databrew_client: aws_sdk_databrew::Client,
    support_client: aws_sdk_support::Client,
    codedeploy_client: aws_sdk_codedeploy::Client,
    batch_client: aws_sdk_batch::Client,
    savingsplans_client: aws_sdk_savingsplans::Client,
    bedrock_client: aws_sdk_bedrock::Client,
    directory_client: aws_sdk_directory::Client,
    workspaces_client: aws_sdk_workspaces::Client,
    chime_client: aws_sdk_chime::Client,
    migrationhubstrategy_client: aws_sdk_migrationhubstrategy::Client,
    timestream_client: aws_sdk_timestream::Client,
    codeguru_client: aws_sdk_codeguru::Client,
    appsync_client: aws_sdk_appsync::Client,
    dlm_client: aws_sdk_dlm::Client,
    iot_client: aws_sdk_iot::Client,
    global_client: aws_sdk_global::Client,
    amplifybackend_client: aws_sdk_amplifybackend::Client,
    datazone_client: aws_sdk_datazone::Client,
    connectcampaignsv2_client: aws_sdk_connectcampaignsv2::Client,
    billingconductor_client: aws_sdk_billingconductor::Client,
    budgets_client: aws_sdk_budgets::Client,
    cloudtrail_client: aws_sdk_cloudtrail::Client,
    geo_client: aws_sdk_geo::Client,
    m2_client: aws_sdk_m2::Client,
    pinpoint_client: aws_sdk_pinpoint::Client,
    lex_client: aws_sdk_lex::Client,
    finspace_client: aws_sdk_finspace::Client,
    detective_client: aws_sdk_detective::Client,
    lambda_client: aws_sdk_lambda::Client,
    kinesis_client: aws_sdk_kinesis::Client,
    panorama_client: aws_sdk_panorama::Client,
    iot_client: aws_sdk_iot::Client,
    app_client: aws_sdk_app::Client,
    managedblockchain_client: aws_sdk_managedblockchain::Client,
    waf_client: aws_sdk_waf::Client,
    ivs_client: aws_sdk_ivs::Client,
    devops_client: aws_sdk_devops::Client,
    cost_client: aws_sdk_cost::Client,
    mq_client: aws_sdk_mq::Client,
    route53_client: aws_sdk_route53::Client,
    internetmonitor_client: aws_sdk_internetmonitor::Client,
    license_client: aws_sdk_license::Client,
    codestar_client: aws_sdk_codestar::Client,
    artifact_client: aws_sdk_artifact::Client,
    ssm_client: aws_sdk_ssm::Client,
    iotsitewise_client: aws_sdk_iotsitewise::Client,
    serverlessapplicationrepository_client: aws_sdk_serverlessapplicationrepository::Client,
    ssm_client: aws_sdk_ssm::Client,
    docdb_client: aws_sdk_docdb::Client,
    sagemaker_client: aws_sdk_sagemaker::Client,
    license_client: aws_sdk_license::Client,
    sagemaker_client: aws_sdk_sagemaker::Client,
    personalize_client: aws_sdk_personalize::Client,
    clouddirectory_client: aws_sdk_clouddirectory::Client,
    chime_client: aws_sdk_chime::Client,
    notificationscontacts_client: aws_sdk_notificationscontacts::Client,
    backupsearch_client: aws_sdk_backupsearch::Client,
    dynamodb_client: aws_sdk_dynamodb::Client,
    iot_client: aws_sdk_iot::Client,
    cognito_client: aws_sdk_cognito::Client,
    codeartifact_client: aws_sdk_codeartifact::Client,
    arc_client: aws_sdk_arc::Client,
    elastic_client: aws_sdk_elastic::Client,
    guardduty_client: aws_sdk_guardduty::Client,
    cleanrooms_client: aws_sdk_cleanrooms::Client,
    trustedadvisor_client: aws_sdk_trustedadvisor::Client,
    sagemaker_client: aws_sdk_sagemaker::Client,
    dax_client: aws_sdk_dax::Client,
    docdb_client: aws_sdk_docdb::Client,
    firehose_client: aws_sdk_firehose::Client,
    ivschat_client: aws_sdk_ivschat::Client,
    ses_client: aws_sdk_ses::Client,
    bcm_client: aws_sdk_bcm::Client,
    application_client: aws_sdk_application::Client,
    device_client: aws_sdk_device::Client,
    account_client: aws_sdk_account::Client,
    launch_client: aws_sdk_launch::Client,
    finspace_client: aws_sdk_finspace::Client,
    appconfigdata_client: aws_sdk_appconfigdata::Client,
    controlcatalog_client: aws_sdk_controlcatalog::Client,
    greengrass_client: aws_sdk_greengrass::Client,
    kendra_client: aws_sdk_kendra::Client,
    snow_client: aws_sdk_snow::Client,
    securityhub_client: aws_sdk_securityhub::Client,
    s3vectors_client: aws_sdk_s3vectors::Client,
    workspaces_client: aws_sdk_workspaces::Client,
    backup_client: aws_sdk_backup::Client,
    opensearchserverless_client: aws_sdk_opensearchserverless::Client,
    cloudformation_client: aws_sdk_cloudformation::Client,
    kendra_client: aws_sdk_kendra::Client,
    connect_client: aws_sdk_connect::Client,
    machine_client: aws_sdk_machine::Client,
    elasticache_client: aws_sdk_elasticache::Client,
    sfn_client: aws_sdk_sfn::Client,
    sso_client: aws_sdk_sso::Client,
    auto_client: aws_sdk_auto::Client,
    comprehend_client: aws_sdk_comprehend::Client,
    rds_client: aws_sdk_rds::Client,
    chime_client: aws_sdk_chime::Client,
    rekognition_client: aws_sdk_rekognition::Client,
    appstream_client: aws_sdk_appstream::Client,
    polly_client: aws_sdk_polly::Client,
    invoicing_client: aws_sdk_invoicing::Client,
    rds_client: aws_sdk_rds::Client,
    pricing_client: aws_sdk_pricing::Client,
    swf_client: aws_sdk_swf::Client,
    cloudfront_client: aws_sdk_cloudfront::Client,
    marketplace_client: aws_sdk_marketplace::Client,
    medical_client: aws_sdk_medical::Client,
    transcribe_client: aws_sdk_transcribe::Client,
    observabilityadmin_client: aws_sdk_observabilityadmin::Client,
    notifications_client: aws_sdk_notifications::Client,
    codecommit_client: aws_sdk_codecommit::Client,
    lex_client: aws_sdk_lex::Client,
    pca_client: aws_sdk_pca::Client,
    forecastquery_client: aws_sdk_forecastquery::Client,
    healthlake_client: aws_sdk_healthlake::Client,
    rolesanywhere_client: aws_sdk_rolesanywhere::Client,
    marketplace_client: aws_sdk_marketplace::Client,
    billing_client: aws_sdk_billing::Client,
    emr_client: aws_sdk_emr::Client,
    apigatewaymanagementapi_client: aws_sdk_apigatewaymanagementapi::Client,
    xray_client: aws_sdk_xray::Client,
    transcribe_client: aws_sdk_transcribe::Client,
    ram_client: aws_sdk_ram::Client,
    codeconnections_client: aws_sdk_codeconnections::Client,
    efs_client: aws_sdk_efs::Client,
    migration_client: aws_sdk_migration::Client,
    elasticsearch_client: aws_sdk_elasticsearch::Client,
    cognito_client: aws_sdk_cognito::Client,
    payment_client: aws_sdk_payment::Client,
    dataexchange_client: aws_sdk_dataexchange::Client,
    sts_client: aws_sdk_sts::Client,
    sagemaker_client: aws_sdk_sagemaker::Client,
    kinesis_client: aws_sdk_kinesis::Client,
    acm_client: aws_sdk_acm::Client,
    athena_client: aws_sdk_athena::Client,
    dsql_client: aws_sdk_dsql::Client,
    mediapackage_client: aws_sdk_mediapackage::Client,
    tnb_client: aws_sdk_tnb::Client,
    ec2_client: aws_sdk_ec2::Client,
    apprunner_client: aws_sdk_apprunner::Client,
    redshift_client: aws_sdk_redshift::Client,

}

impl AwsProvider {
    /// Create a new unified provider instance
    pub async fn new() -> Result<Self> {
        let config = aws_config::load_from_env().await;
        let oam_client = aws_sdk_oam::Client::new(&config);
        let forecast_client = aws_sdk_forecast::Client::new(&config);
        let bedrock_client = aws_sdk_bedrock::Client::new(&config);
        let codebuild_client = aws_sdk_codebuild::Client::new(&config);
        let iotdeviceadvisor_client = aws_sdk_iotdeviceadvisor::Client::new(&config);
        let auto_client = aws_sdk_auto::Client::new(&config);
        let database_client = aws_sdk_database::Client::new(&config);
        let codeguruprofiler_client = aws_sdk_codeguruprofiler::Client::new(&config);
        let kinesis_client = aws_sdk_kinesis::Client::new(&config);
        let pinpoint_client = aws_sdk_pinpoint::Client::new(&config);
        let chime_client = aws_sdk_chime::Client::new(&config);
        let iottwinmaker_client = aws_sdk_iottwinmaker::Client::new(&config);
        let organizations_client = aws_sdk_organizations::Client::new(&config);
        let networkflowmonitor_client = aws_sdk_networkflowmonitor::Client::new(&config);
        let shield_client = aws_sdk_shield::Client::new(&config);
        let ssm_client = aws_sdk_ssm::Client::new(&config);
        let arc_client = aws_sdk_arc::Client::new(&config);
        let workspaces_client = aws_sdk_workspaces::Client::new(&config);
        let partnercentral_client = aws_sdk_partnercentral::Client::new(&config);
        let geo_client = aws_sdk_geo::Client::new(&config);
        let signer_client = aws_sdk_signer::Client::new(&config);
        let marketplace_client = aws_sdk_marketplace::Client::new(&config);
        let lakeformation_client = aws_sdk_lakeformation::Client::new(&config);
        let pcs_client = aws_sdk_pcs::Client::new(&config);
        let kinesis_client = aws_sdk_kinesis::Client::new(&config);
        let mediaconnect_client = aws_sdk_mediaconnect::Client::new(&config);
        let secrets_client = aws_sdk_secrets::Client::new(&config);
        let mwaa_client = aws_sdk_mwaa::Client::new(&config);
        let kms_client = aws_sdk_kms::Client::new(&config);
        let quicksight_client = aws_sdk_quicksight::Client::new(&config);
        let workmail_client = aws_sdk_workmail::Client::new(&config);
        let eventbridge_client = aws_sdk_eventbridge::Client::new(&config);
        let frauddetector_client = aws_sdk_frauddetector::Client::new(&config);
        let cloudwatch_client = aws_sdk_cloudwatch::Client::new(&config);
        let cloudhsm_client = aws_sdk_cloudhsm::Client::new(&config);
        let cloudsearch_client = aws_sdk_cloudsearch::Client::new(&config);
        let lookoutequipment_client = aws_sdk_lookoutequipment::Client::new(&config);
        let iot_client = aws_sdk_iot::Client::new(&config);
        let securitylake_client = aws_sdk_securitylake::Client::new(&config);
        let cloudwatch_client = aws_sdk_cloudwatch::Client::new(&config);
        let glue_client = aws_sdk_glue::Client::new(&config);
        let application_client = aws_sdk_application::Client::new(&config);
        let personalize_client = aws_sdk_personalize::Client::new(&config);
        let voice_client = aws_sdk_voice::Client::new(&config);
        let s3_client = aws_sdk_s3::Client::new(&config);
        let appintegrations_client = aws_sdk_appintegrations::Client::new(&config);
        let lex_client = aws_sdk_lex::Client::new(&config);
        let ssm_client = aws_sdk_ssm::Client::new(&config);
        let chime_client = aws_sdk_chime::Client::new(&config);
        let sesv2_client = aws_sdk_sesv2::Client::new(&config);
        let bedrock_client = aws_sdk_bedrock::Client::new(&config);
        let emr_client = aws_sdk_emr::Client::new(&config);
        let controltower_client = aws_sdk_controltower::Client::new(&config);
        let resource_client = aws_sdk_resource::Client::new(&config);
        let s3_client = aws_sdk_s3::Client::new(&config);
        let rtbfabric_client = aws_sdk_rtbfabric::Client::new(&config);
        let personalize_client = aws_sdk_personalize::Client::new(&config);
        let marketplace_client = aws_sdk_marketplace::Client::new(&config);
        let directory_client = aws_sdk_directory::Client::new(&config);
        let ssm_client = aws_sdk_ssm::Client::new(&config);
        let outposts_client = aws_sdk_outposts::Client::new(&config);
        let workdocs_client = aws_sdk_workdocs::Client::new(&config);
        let networkmanager_client = aws_sdk_networkmanager::Client::new(&config);
        let kinesis_client = aws_sdk_kinesis::Client::new(&config);
        let socialmessaging_client = aws_sdk_socialmessaging::Client::new(&config);
        let bedrock_client = aws_sdk_bedrock::Client::new(&config);
        let omics_client = aws_sdk_omics::Client::new(&config);
        let bcm_client = aws_sdk_bcm::Client::new(&config);
        let mediapackage_client = aws_sdk_mediapackage::Client::new(&config);
        let sagemaker_client = aws_sdk_sagemaker::Client::new(&config);
        let config_client = aws_sdk_config::Client::new(&config);
        let cloudwatch_client = aws_sdk_cloudwatch::Client::new(&config);
        let medialive_client = aws_sdk_medialive::Client::new(&config);
        let backup_client = aws_sdk_backup::Client::new(&config);
        let connect_client = aws_sdk_connect::Client::new(&config);
        let mediaconvert_client = aws_sdk_mediaconvert::Client::new(&config);
        let sns_client = aws_sdk_sns::Client::new(&config);
        let evs_client = aws_sdk_evs::Client::new(&config);
        let datasync_client = aws_sdk_datasync::Client::new(&config);
        let greengrassv2_client = aws_sdk_greengrassv2::Client::new(&config);
        let cleanroomsml_client = aws_sdk_cleanroomsml::Client::new(&config);
        let neptunedata_client = aws_sdk_neptunedata::Client::new(&config);
        let acm_client = aws_sdk_acm::Client::new(&config);
        let service_client = aws_sdk_service::Client::new(&config);
        let b2bi_client = aws_sdk_b2bi::Client::new(&config);
        let iotanalytics_client = aws_sdk_iotanalytics::Client::new(&config);
        let lex_client = aws_sdk_lex::Client::new(&config);
        let inspector2_client = aws_sdk_inspector2::Client::new(&config);
        let groundstation_client = aws_sdk_groundstation::Client::new(&config);
        let ecr_client = aws_sdk_ecr::Client::new(&config);
        let fis_client = aws_sdk_fis::Client::new(&config);
        let proton_client = aws_sdk_proton::Client::new(&config);
        let api_client = aws_sdk_api::Client::new(&config);
        let cloudhsm_client = aws_sdk_cloudhsm::Client::new(&config);
        let mpa_client = aws_sdk_mpa::Client::new(&config);
        let osis_client = aws_sdk_osis::Client::new(&config);
        let memorydb_client = aws_sdk_memorydb::Client::new(&config);
        let inspector_client = aws_sdk_inspector::Client::new(&config);
        let translate_client = aws_sdk_translate::Client::new(&config);
        let mailmanager_client = aws_sdk_mailmanager::Client::new(&config);
        let neptune_client = aws_sdk_neptune::Client::new(&config);
        let chatbot_client = aws_sdk_chatbot::Client::new(&config);
        let fms_client = aws_sdk_fms::Client::new(&config);
        let qapps_client = aws_sdk_qapps::Client::new(&config);
        let customer_client = aws_sdk_customer::Client::new(&config);
        let geo_client = aws_sdk_geo::Client::new(&config);
        let route_client = aws_sdk_route::Client::new(&config);
        let service_client = aws_sdk_service::Client::new(&config);
        let qbusiness_client = aws_sdk_qbusiness::Client::new(&config);
        let synthetics_client = aws_sdk_synthetics::Client::new(&config);
        let codecatalyst_client = aws_sdk_codecatalyst::Client::new(&config);
        let keyspacesstreams_client = aws_sdk_keyspacesstreams::Client::new(&config);
        let storage_client = aws_sdk_storage::Client::new(&config);
        let elastic_client = aws_sdk_elastic::Client::new(&config);
        let bcm_client = aws_sdk_bcm::Client::new(&config);
        let iotsecuretunneling_client = aws_sdk_iotsecuretunneling::Client::new(&config);
        let cloudfront_client = aws_sdk_cloudfront::Client::new(&config);
        let bedrock_client = aws_sdk_bedrock::Client::new(&config);
        let location_client = aws_sdk_location::Client::new(&config);
        let wafv2_client = aws_sdk_wafv2::Client::new(&config);
        let opensearch_client = aws_sdk_opensearch::Client::new(&config);
        let iotthingsgraph_client = aws_sdk_iotthingsgraph::Client::new(&config);
        let security_client = aws_sdk_security::Client::new(&config);
        let repostspace_client = aws_sdk_repostspace::Client::new(&config);
        let health_client = aws_sdk_health::Client::new(&config);
        let workmailmessageflow_client = aws_sdk_workmailmessageflow::Client::new(&config);
        let mediastore_client = aws_sdk_mediastore::Client::new(&config);
        let ec2_client = aws_sdk_ec2::Client::new(&config);
        let comprehendmedical_client = aws_sdk_comprehendmedical::Client::new(&config);
        let iotfleetwise_client = aws_sdk_iotfleetwise::Client::new(&config);
        let route53profiles_client = aws_sdk_route53profiles::Client::new(&config);
        let application_client = aws_sdk_application::Client::new(&config);
        let resource_client = aws_sdk_resource::Client::new(&config);
        let accessanalyzer_client = aws_sdk_accessanalyzer::Client::new(&config);
        let glacier_client = aws_sdk_glacier::Client::new(&config);
        let lightsail_client = aws_sdk_lightsail::Client::new(&config);
        let rum_client = aws_sdk_rum::Client::new(&config);
        let direct_client = aws_sdk_direct::Client::new(&config);
        let elastic_client = aws_sdk_elastic::Client::new(&config);
        let imagebuilder_client = aws_sdk_imagebuilder::Client::new(&config);
        let simspaceweaver_client = aws_sdk_simspaceweaver::Client::new(&config);
        let iot_client = aws_sdk_iot::Client::new(&config);
        let freetier_client = aws_sdk_freetier::Client::new(&config);
        let bedrock_client = aws_sdk_bedrock::Client::new(&config);
        let neptune_client = aws_sdk_neptune::Client::new(&config);
        let transfer_client = aws_sdk_transfer::Client::new(&config);
        let deadline_client = aws_sdk_deadline::Client::new(&config);
        let braket_client = aws_sdk_braket::Client::new(&config);
        let verifiedpermissions_client = aws_sdk_verifiedpermissions::Client::new(&config);
        let scheduler_client = aws_sdk_scheduler::Client::new(&config);
        let waf_client = aws_sdk_waf::Client::new(&config);
        let marketplace_client = aws_sdk_marketplace::Client::new(&config);
        let ecr_client = aws_sdk_ecr::Client::new(&config);
        let cost_client = aws_sdk_cost::Client::new(&config);
        let kinesis_client = aws_sdk_kinesis::Client::new(&config);
        let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);
        let resiliencehub_client = aws_sdk_resiliencehub::Client::new(&config);
        let macie2_client = aws_sdk_macie2::Client::new(&config);
        let entityresolution_client = aws_sdk_entityresolution::Client::new(&config);
        let s3outposts_client = aws_sdk_s3outposts::Client::new(&config);
        let grafana_client = aws_sdk_grafana::Client::new(&config);
        let kinesis_client = aws_sdk_kinesis::Client::new(&config);
        let eks_client = aws_sdk_eks::Client::new(&config);
        let rbin_client = aws_sdk_rbin::Client::new(&config);
        let service_client = aws_sdk_service::Client::new(&config);
        let ecs_client = aws_sdk_ecs::Client::new(&config);
        let marketplace_client = aws_sdk_marketplace::Client::new(&config);
        let timestream_client = aws_sdk_timestream::Client::new(&config);
        let qconnect_client = aws_sdk_qconnect::Client::new(&config);
        let cloudsearch_client = aws_sdk_cloudsearch::Client::new(&config);
        let bcm_client = aws_sdk_bcm::Client::new(&config);
        let appfabric_client = aws_sdk_appfabric::Client::new(&config);
        let route53resolver_client = aws_sdk_route53resolver::Client::new(&config);
        let marketplace_client = aws_sdk_marketplace::Client::new(&config);
        let workspaces_client = aws_sdk_workspaces::Client::new(&config);
        let gameliftstreams_client = aws_sdk_gameliftstreams::Client::new(&config);
        let taxsettings_client = aws_sdk_taxsettings::Client::new(&config);
        let pinpoint_client = aws_sdk_pinpoint::Client::new(&config);
        let fsx_client = aws_sdk_fsx::Client::new(&config);
        let codepipeline_client = aws_sdk_codepipeline::Client::new(&config);
        let schemas_client = aws_sdk_schemas::Client::new(&config);
        let emr_client = aws_sdk_emr::Client::new(&config);
        let sqs_client = aws_sdk_sqs::Client::new(&config);
        let license_client = aws_sdk_license::Client::new(&config);
        let route53_client = aws_sdk_route53::Client::new(&config);
        let migrationhuborchestrator_client = aws_sdk_migrationhuborchestrator::Client::new(&config);
        let iot_client = aws_sdk_iot::Client::new(&config);
        let sso_client = aws_sdk_sso::Client::new(&config);
        let codestar_client = aws_sdk_codestar::Client::new(&config);
        let ebs_client = aws_sdk_ebs::Client::new(&config);
        let aiops_client = aws_sdk_aiops::Client::new(&config);
        let amplify_client = aws_sdk_amplify::Client::new(&config);
        let cloudcontrol_client = aws_sdk_cloudcontrol::Client::new(&config);
        let wellarchitected_client = aws_sdk_wellarchitected::Client::new(&config);
        let route_client = aws_sdk_route::Client::new(&config);
        let ssm_client = aws_sdk_ssm::Client::new(&config);
        let bedrock_client = aws_sdk_bedrock::Client::new(&config);
        let ivs_client = aws_sdk_ivs::Client::new(&config);
        let migrationhub_client = aws_sdk_migrationhub::Client::new(&config);
        let redshift_client = aws_sdk_redshift::Client::new(&config);
        let inspector_client = aws_sdk_inspector::Client::new(&config);
        let connectcases_client = aws_sdk_connectcases::Client::new(&config);
        let kinesis_client = aws_sdk_kinesis::Client::new(&config);
        let pca_client = aws_sdk_pca::Client::new(&config);
        let appflow_client = aws_sdk_appflow::Client::new(&config);
        let gamelift_client = aws_sdk_gamelift::Client::new(&config);
        let cloudtrail_client = aws_sdk_cloudtrail::Client::new(&config);
        let resource_client = aws_sdk_resource::Client::new(&config);
        let supplychain_client = aws_sdk_supplychain::Client::new(&config);
        let timestream_client = aws_sdk_timestream::Client::new(&config);
        let pipes_client = aws_sdk_pipes::Client::new(&config);
        let evidently_client = aws_sdk_evidently::Client::new(&config);
        let codeguru_client = aws_sdk_codeguru::Client::new(&config);
        let cost_client = aws_sdk_cost::Client::new(&config);
        let amplifyuibuilder_client = aws_sdk_amplifyuibuilder::Client::new(&config);
        let route53_client = aws_sdk_route53::Client::new(&config);
        let vpc_client = aws_sdk_vpc::Client::new(&config);
        let managedblockchain_client = aws_sdk_managedblockchain::Client::new(&config);
        let redshift_client = aws_sdk_redshift::Client::new(&config);
        let mediatailor_client = aws_sdk_mediatailor::Client::new(&config);
        let mediapackagev2_client = aws_sdk_mediapackagev2::Client::new(&config);
        let pi_client = aws_sdk_pi::Client::new(&config);
        let appconfig_client = aws_sdk_appconfig::Client::new(&config);
        let networkmonitor_client = aws_sdk_networkmonitor::Client::new(&config);
        let network_client = aws_sdk_network::Client::new(&config);
        let connectparticipant_client = aws_sdk_connectparticipant::Client::new(&config);
        let mgn_client = aws_sdk_mgn::Client::new(&config);
        let sagemaker_client = aws_sdk_sagemaker::Client::new(&config);
        let applicationcostprofiler_client = aws_sdk_applicationcostprofiler::Client::new(&config);
        let keyspaces_client = aws_sdk_keyspaces::Client::new(&config);
        let iam_client = aws_sdk_iam::Client::new(&config);
        let data_client = aws_sdk_data::Client::new(&config);
        let odb_client = aws_sdk_odb::Client::new(&config);
        let chime_client = aws_sdk_chime::Client::new(&config);
        let mediastore_client = aws_sdk_mediastore::Client::new(&config);
        let iot_client = aws_sdk_iot::Client::new(&config);
        let cloud9_client = aws_sdk_cloud9::Client::new(&config);
        let wisdom_client = aws_sdk_wisdom::Client::new(&config);
        let sagemaker_client = aws_sdk_sagemaker::Client::new(&config);
        let sso_client = aws_sdk_sso::Client::new(&config);
        let auditmanager_client = aws_sdk_auditmanager::Client::new(&config);
        let snowball_client = aws_sdk_snowball::Client::new(&config);
        let migration_client = aws_sdk_migration::Client::new(&config);
        let identitystore_client = aws_sdk_identitystore::Client::new(&config);
        let elastic_client = aws_sdk_elastic::Client::new(&config);
        let connectcampaigns_client = aws_sdk_connectcampaigns::Client::new(&config);
        let textract_client = aws_sdk_textract::Client::new(&config);
        let compute_client = aws_sdk_compute::Client::new(&config);
        let s3tables_client = aws_sdk_s3tables::Client::new(&config);
        let eks_client = aws_sdk_eks::Client::new(&config);
        let support_client = aws_sdk_support::Client::new(&config);
        let mturk_client = aws_sdk_mturk::Client::new(&config);
        let apigatewayv2_client = aws_sdk_apigatewayv2::Client::new(&config);
        let cognito_client = aws_sdk_cognito::Client::new(&config);
        let bedrock_client = aws_sdk_bedrock::Client::new(&config);
        let pinpoint_client = aws_sdk_pinpoint::Client::new(&config);
        let amp_client = aws_sdk_amp::Client::new(&config);
        let drs_client = aws_sdk_drs::Client::new(&config);
        let payment_client = aws_sdk_payment::Client::new(&config);
        let kafkaconnect_client = aws_sdk_kafkaconnect::Client::new(&config);
        let kafka_client = aws_sdk_kafka::Client::new(&config);
        let databrew_client = aws_sdk_databrew::Client::new(&config);
        let support_client = aws_sdk_support::Client::new(&config);
        let codedeploy_client = aws_sdk_codedeploy::Client::new(&config);
        let batch_client = aws_sdk_batch::Client::new(&config);
        let savingsplans_client = aws_sdk_savingsplans::Client::new(&config);
        let bedrock_client = aws_sdk_bedrock::Client::new(&config);
        let directory_client = aws_sdk_directory::Client::new(&config);
        let workspaces_client = aws_sdk_workspaces::Client::new(&config);
        let chime_client = aws_sdk_chime::Client::new(&config);
        let migrationhubstrategy_client = aws_sdk_migrationhubstrategy::Client::new(&config);
        let timestream_client = aws_sdk_timestream::Client::new(&config);
        let codeguru_client = aws_sdk_codeguru::Client::new(&config);
        let appsync_client = aws_sdk_appsync::Client::new(&config);
        let dlm_client = aws_sdk_dlm::Client::new(&config);
        let iot_client = aws_sdk_iot::Client::new(&config);
        let global_client = aws_sdk_global::Client::new(&config);
        let amplifybackend_client = aws_sdk_amplifybackend::Client::new(&config);
        let datazone_client = aws_sdk_datazone::Client::new(&config);
        let connectcampaignsv2_client = aws_sdk_connectcampaignsv2::Client::new(&config);
        let billingconductor_client = aws_sdk_billingconductor::Client::new(&config);
        let budgets_client = aws_sdk_budgets::Client::new(&config);
        let cloudtrail_client = aws_sdk_cloudtrail::Client::new(&config);
        let geo_client = aws_sdk_geo::Client::new(&config);
        let m2_client = aws_sdk_m2::Client::new(&config);
        let pinpoint_client = aws_sdk_pinpoint::Client::new(&config);
        let lex_client = aws_sdk_lex::Client::new(&config);
        let finspace_client = aws_sdk_finspace::Client::new(&config);
        let detective_client = aws_sdk_detective::Client::new(&config);
        let lambda_client = aws_sdk_lambda::Client::new(&config);
        let kinesis_client = aws_sdk_kinesis::Client::new(&config);
        let panorama_client = aws_sdk_panorama::Client::new(&config);
        let iot_client = aws_sdk_iot::Client::new(&config);
        let app_client = aws_sdk_app::Client::new(&config);
        let managedblockchain_client = aws_sdk_managedblockchain::Client::new(&config);
        let waf_client = aws_sdk_waf::Client::new(&config);
        let ivs_client = aws_sdk_ivs::Client::new(&config);
        let devops_client = aws_sdk_devops::Client::new(&config);
        let cost_client = aws_sdk_cost::Client::new(&config);
        let mq_client = aws_sdk_mq::Client::new(&config);
        let route53_client = aws_sdk_route53::Client::new(&config);
        let internetmonitor_client = aws_sdk_internetmonitor::Client::new(&config);
        let license_client = aws_sdk_license::Client::new(&config);
        let codestar_client = aws_sdk_codestar::Client::new(&config);
        let artifact_client = aws_sdk_artifact::Client::new(&config);
        let ssm_client = aws_sdk_ssm::Client::new(&config);
        let iotsitewise_client = aws_sdk_iotsitewise::Client::new(&config);
        let serverlessapplicationrepository_client = aws_sdk_serverlessapplicationrepository::Client::new(&config);
        let ssm_client = aws_sdk_ssm::Client::new(&config);
        let docdb_client = aws_sdk_docdb::Client::new(&config);
        let sagemaker_client = aws_sdk_sagemaker::Client::new(&config);
        let license_client = aws_sdk_license::Client::new(&config);
        let sagemaker_client = aws_sdk_sagemaker::Client::new(&config);
        let personalize_client = aws_sdk_personalize::Client::new(&config);
        let clouddirectory_client = aws_sdk_clouddirectory::Client::new(&config);
        let chime_client = aws_sdk_chime::Client::new(&config);
        let notificationscontacts_client = aws_sdk_notificationscontacts::Client::new(&config);
        let backupsearch_client = aws_sdk_backupsearch::Client::new(&config);
        let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);
        let iot_client = aws_sdk_iot::Client::new(&config);
        let cognito_client = aws_sdk_cognito::Client::new(&config);
        let codeartifact_client = aws_sdk_codeartifact::Client::new(&config);
        let arc_client = aws_sdk_arc::Client::new(&config);
        let elastic_client = aws_sdk_elastic::Client::new(&config);
        let guardduty_client = aws_sdk_guardduty::Client::new(&config);
        let cleanrooms_client = aws_sdk_cleanrooms::Client::new(&config);
        let trustedadvisor_client = aws_sdk_trustedadvisor::Client::new(&config);
        let sagemaker_client = aws_sdk_sagemaker::Client::new(&config);
        let dax_client = aws_sdk_dax::Client::new(&config);
        let docdb_client = aws_sdk_docdb::Client::new(&config);
        let firehose_client = aws_sdk_firehose::Client::new(&config);
        let ivschat_client = aws_sdk_ivschat::Client::new(&config);
        let ses_client = aws_sdk_ses::Client::new(&config);
        let bcm_client = aws_sdk_bcm::Client::new(&config);
        let application_client = aws_sdk_application::Client::new(&config);
        let device_client = aws_sdk_device::Client::new(&config);
        let account_client = aws_sdk_account::Client::new(&config);
        let launch_client = aws_sdk_launch::Client::new(&config);
        let finspace_client = aws_sdk_finspace::Client::new(&config);
        let appconfigdata_client = aws_sdk_appconfigdata::Client::new(&config);
        let controlcatalog_client = aws_sdk_controlcatalog::Client::new(&config);
        let greengrass_client = aws_sdk_greengrass::Client::new(&config);
        let kendra_client = aws_sdk_kendra::Client::new(&config);
        let snow_client = aws_sdk_snow::Client::new(&config);
        let securityhub_client = aws_sdk_securityhub::Client::new(&config);
        let s3vectors_client = aws_sdk_s3vectors::Client::new(&config);
        let workspaces_client = aws_sdk_workspaces::Client::new(&config);
        let backup_client = aws_sdk_backup::Client::new(&config);
        let opensearchserverless_client = aws_sdk_opensearchserverless::Client::new(&config);
        let cloudformation_client = aws_sdk_cloudformation::Client::new(&config);
        let kendra_client = aws_sdk_kendra::Client::new(&config);
        let connect_client = aws_sdk_connect::Client::new(&config);
        let machine_client = aws_sdk_machine::Client::new(&config);
        let elasticache_client = aws_sdk_elasticache::Client::new(&config);
        let sfn_client = aws_sdk_sfn::Client::new(&config);
        let sso_client = aws_sdk_sso::Client::new(&config);
        let auto_client = aws_sdk_auto::Client::new(&config);
        let comprehend_client = aws_sdk_comprehend::Client::new(&config);
        let rds_client = aws_sdk_rds::Client::new(&config);
        let chime_client = aws_sdk_chime::Client::new(&config);
        let rekognition_client = aws_sdk_rekognition::Client::new(&config);
        let appstream_client = aws_sdk_appstream::Client::new(&config);
        let polly_client = aws_sdk_polly::Client::new(&config);
        let invoicing_client = aws_sdk_invoicing::Client::new(&config);
        let rds_client = aws_sdk_rds::Client::new(&config);
        let pricing_client = aws_sdk_pricing::Client::new(&config);
        let swf_client = aws_sdk_swf::Client::new(&config);
        let cloudfront_client = aws_sdk_cloudfront::Client::new(&config);
        let marketplace_client = aws_sdk_marketplace::Client::new(&config);
        let medical_client = aws_sdk_medical::Client::new(&config);
        let transcribe_client = aws_sdk_transcribe::Client::new(&config);
        let observabilityadmin_client = aws_sdk_observabilityadmin::Client::new(&config);
        let notifications_client = aws_sdk_notifications::Client::new(&config);
        let codecommit_client = aws_sdk_codecommit::Client::new(&config);
        let lex_client = aws_sdk_lex::Client::new(&config);
        let pca_client = aws_sdk_pca::Client::new(&config);
        let forecastquery_client = aws_sdk_forecastquery::Client::new(&config);
        let healthlake_client = aws_sdk_healthlake::Client::new(&config);
        let rolesanywhere_client = aws_sdk_rolesanywhere::Client::new(&config);
        let marketplace_client = aws_sdk_marketplace::Client::new(&config);
        let billing_client = aws_sdk_billing::Client::new(&config);
        let emr_client = aws_sdk_emr::Client::new(&config);
        let apigatewaymanagementapi_client = aws_sdk_apigatewaymanagementapi::Client::new(&config);
        let xray_client = aws_sdk_xray::Client::new(&config);
        let transcribe_client = aws_sdk_transcribe::Client::new(&config);
        let ram_client = aws_sdk_ram::Client::new(&config);
        let codeconnections_client = aws_sdk_codeconnections::Client::new(&config);
        let efs_client = aws_sdk_efs::Client::new(&config);
        let migration_client = aws_sdk_migration::Client::new(&config);
        let elasticsearch_client = aws_sdk_elasticsearch::Client::new(&config);
        let cognito_client = aws_sdk_cognito::Client::new(&config);
        let payment_client = aws_sdk_payment::Client::new(&config);
        let dataexchange_client = aws_sdk_dataexchange::Client::new(&config);
        let sts_client = aws_sdk_sts::Client::new(&config);
        let sagemaker_client = aws_sdk_sagemaker::Client::new(&config);
        let kinesis_client = aws_sdk_kinesis::Client::new(&config);
        let acm_client = aws_sdk_acm::Client::new(&config);
        let athena_client = aws_sdk_athena::Client::new(&config);
        let dsql_client = aws_sdk_dsql::Client::new(&config);
        let mediapackage_client = aws_sdk_mediapackage::Client::new(&config);
        let tnb_client = aws_sdk_tnb::Client::new(&config);
        let ec2_client = aws_sdk_ec2::Client::new(&config);
        let apprunner_client = aws_sdk_apprunner::Client::new(&config);
        let redshift_client = aws_sdk_redshift::Client::new(&config);

        Ok(Self {
            oam_client,
            forecast_client,
            bedrock_client,
            codebuild_client,
            iotdeviceadvisor_client,
            auto_client,
            database_client,
            codeguruprofiler_client,
            kinesis_client,
            pinpoint_client,
            chime_client,
            iottwinmaker_client,
            organizations_client,
            networkflowmonitor_client,
            shield_client,
            ssm_client,
            arc_client,
            workspaces_client,
            partnercentral_client,
            geo_client,
            signer_client,
            marketplace_client,
            lakeformation_client,
            pcs_client,
            kinesis_client,
            mediaconnect_client,
            secrets_client,
            mwaa_client,
            kms_client,
            quicksight_client,
            workmail_client,
            eventbridge_client,
            frauddetector_client,
            cloudwatch_client,
            cloudhsm_client,
            cloudsearch_client,
            lookoutequipment_client,
            iot_client,
            securitylake_client,
            cloudwatch_client,
            glue_client,
            application_client,
            personalize_client,
            voice_client,
            s3_client,
            appintegrations_client,
            lex_client,
            ssm_client,
            chime_client,
            sesv2_client,
            bedrock_client,
            emr_client,
            controltower_client,
            resource_client,
            s3_client,
            rtbfabric_client,
            personalize_client,
            marketplace_client,
            directory_client,
            ssm_client,
            outposts_client,
            workdocs_client,
            networkmanager_client,
            kinesis_client,
            socialmessaging_client,
            bedrock_client,
            omics_client,
            bcm_client,
            mediapackage_client,
            sagemaker_client,
            config_client,
            cloudwatch_client,
            medialive_client,
            backup_client,
            connect_client,
            mediaconvert_client,
            sns_client,
            evs_client,
            datasync_client,
            greengrassv2_client,
            cleanroomsml_client,
            neptunedata_client,
            acm_client,
            service_client,
            b2bi_client,
            iotanalytics_client,
            lex_client,
            inspector2_client,
            groundstation_client,
            ecr_client,
            fis_client,
            proton_client,
            api_client,
            cloudhsm_client,
            mpa_client,
            osis_client,
            memorydb_client,
            inspector_client,
            translate_client,
            mailmanager_client,
            neptune_client,
            chatbot_client,
            fms_client,
            qapps_client,
            customer_client,
            geo_client,
            route_client,
            service_client,
            qbusiness_client,
            synthetics_client,
            codecatalyst_client,
            keyspacesstreams_client,
            storage_client,
            elastic_client,
            bcm_client,
            iotsecuretunneling_client,
            cloudfront_client,
            bedrock_client,
            location_client,
            wafv2_client,
            opensearch_client,
            iotthingsgraph_client,
            security_client,
            repostspace_client,
            health_client,
            workmailmessageflow_client,
            mediastore_client,
            ec2_client,
            comprehendmedical_client,
            iotfleetwise_client,
            route53profiles_client,
            application_client,
            resource_client,
            accessanalyzer_client,
            glacier_client,
            lightsail_client,
            rum_client,
            direct_client,
            elastic_client,
            imagebuilder_client,
            simspaceweaver_client,
            iot_client,
            freetier_client,
            bedrock_client,
            neptune_client,
            transfer_client,
            deadline_client,
            braket_client,
            verifiedpermissions_client,
            scheduler_client,
            waf_client,
            marketplace_client,
            ecr_client,
            cost_client,
            kinesis_client,
            dynamodb_client,
            resiliencehub_client,
            macie2_client,
            entityresolution_client,
            s3outposts_client,
            grafana_client,
            kinesis_client,
            eks_client,
            rbin_client,
            service_client,
            ecs_client,
            marketplace_client,
            timestream_client,
            qconnect_client,
            cloudsearch_client,
            bcm_client,
            appfabric_client,
            route53resolver_client,
            marketplace_client,
            workspaces_client,
            gameliftstreams_client,
            taxsettings_client,
            pinpoint_client,
            fsx_client,
            codepipeline_client,
            schemas_client,
            emr_client,
            sqs_client,
            license_client,
            route53_client,
            migrationhuborchestrator_client,
            iot_client,
            sso_client,
            codestar_client,
            ebs_client,
            aiops_client,
            amplify_client,
            cloudcontrol_client,
            wellarchitected_client,
            route_client,
            ssm_client,
            bedrock_client,
            ivs_client,
            migrationhub_client,
            redshift_client,
            inspector_client,
            connectcases_client,
            kinesis_client,
            pca_client,
            appflow_client,
            gamelift_client,
            cloudtrail_client,
            resource_client,
            supplychain_client,
            timestream_client,
            pipes_client,
            evidently_client,
            codeguru_client,
            cost_client,
            amplifyuibuilder_client,
            route53_client,
            vpc_client,
            managedblockchain_client,
            redshift_client,
            mediatailor_client,
            mediapackagev2_client,
            pi_client,
            appconfig_client,
            networkmonitor_client,
            network_client,
            connectparticipant_client,
            mgn_client,
            sagemaker_client,
            applicationcostprofiler_client,
            keyspaces_client,
            iam_client,
            data_client,
            odb_client,
            chime_client,
            mediastore_client,
            iot_client,
            cloud9_client,
            wisdom_client,
            sagemaker_client,
            sso_client,
            auditmanager_client,
            snowball_client,
            migration_client,
            identitystore_client,
            elastic_client,
            connectcampaigns_client,
            textract_client,
            compute_client,
            s3tables_client,
            eks_client,
            support_client,
            mturk_client,
            apigatewayv2_client,
            cognito_client,
            bedrock_client,
            pinpoint_client,
            amp_client,
            drs_client,
            payment_client,
            kafkaconnect_client,
            kafka_client,
            databrew_client,
            support_client,
            codedeploy_client,
            batch_client,
            savingsplans_client,
            bedrock_client,
            directory_client,
            workspaces_client,
            chime_client,
            migrationhubstrategy_client,
            timestream_client,
            codeguru_client,
            appsync_client,
            dlm_client,
            iot_client,
            global_client,
            amplifybackend_client,
            datazone_client,
            connectcampaignsv2_client,
            billingconductor_client,
            budgets_client,
            cloudtrail_client,
            geo_client,
            m2_client,
            pinpoint_client,
            lex_client,
            finspace_client,
            detective_client,
            lambda_client,
            kinesis_client,
            panorama_client,
            iot_client,
            app_client,
            managedblockchain_client,
            waf_client,
            ivs_client,
            devops_client,
            cost_client,
            mq_client,
            route53_client,
            internetmonitor_client,
            license_client,
            codestar_client,
            artifact_client,
            ssm_client,
            iotsitewise_client,
            serverlessapplicationrepository_client,
            ssm_client,
            docdb_client,
            sagemaker_client,
            license_client,
            sagemaker_client,
            personalize_client,
            clouddirectory_client,
            chime_client,
            notificationscontacts_client,
            backupsearch_client,
            dynamodb_client,
            iot_client,
            cognito_client,
            codeartifact_client,
            arc_client,
            elastic_client,
            guardduty_client,
            cleanrooms_client,
            trustedadvisor_client,
            sagemaker_client,
            dax_client,
            docdb_client,
            firehose_client,
            ivschat_client,
            ses_client,
            bcm_client,
            application_client,
            device_client,
            account_client,
            launch_client,
            finspace_client,
            appconfigdata_client,
            controlcatalog_client,
            greengrass_client,
            kendra_client,
            snow_client,
            securityhub_client,
            s3vectors_client,
            workspaces_client,
            backup_client,
            opensearchserverless_client,
            cloudformation_client,
            kendra_client,
            connect_client,
            machine_client,
            elasticache_client,
            sfn_client,
            sso_client,
            auto_client,
            comprehend_client,
            rds_client,
            chime_client,
            rekognition_client,
            appstream_client,
            polly_client,
            invoicing_client,
            rds_client,
            pricing_client,
            swf_client,
            cloudfront_client,
            marketplace_client,
            medical_client,
            transcribe_client,
            observabilityadmin_client,
            notifications_client,
            codecommit_client,
            lex_client,
            pca_client,
            forecastquery_client,
            healthlake_client,
            rolesanywhere_client,
            marketplace_client,
            billing_client,
            emr_client,
            apigatewaymanagementapi_client,
            xray_client,
            transcribe_client,
            ram_client,
            codeconnections_client,
            efs_client,
            migration_client,
            elasticsearch_client,
            cognito_client,
            payment_client,
            dataexchange_client,
            sts_client,
            sagemaker_client,
            kinesis_client,
            acm_client,
            athena_client,
            dsql_client,
            mediapackage_client,
            tnb_client,
            ec2_client,
            apprunner_client,
            redshift_client,

        })
    }

    /// Get oam service handler
    pub fn oam(&self) -> oam::OamService<'_> {
        oam::OamService::new(self)
    }
    /// Get forecast service handler
    pub fn forecast(&self) -> forecast::ForecastService<'_> {
        forecast::ForecastService::new(self)
    }
    /// Get bedrock service handler
    pub fn bedrock(&self) -> bedrock::BedrockService<'_> {
        bedrock::BedrockService::new(self)
    }
    /// Get codebuild service handler
    pub fn codebuild(&self) -> codebuild::CodebuildService<'_> {
        codebuild::CodebuildService::new(self)
    }
    /// Get iotdeviceadvisor service handler
    pub fn iotdeviceadvisor(&self) -> iotdeviceadvisor::IotdeviceadvisorService<'_> {
        iotdeviceadvisor::IotdeviceadvisorService::new(self)
    }
    /// Get auto service handler
    pub fn auto(&self) -> auto::AutoService<'_> {
        auto::AutoService::new(self)
    }
    /// Get database service handler
    pub fn database(&self) -> database::DatabaseService<'_> {
        database::DatabaseService::new(self)
    }
    /// Get codeguruprofiler service handler
    pub fn codeguruprofiler(&self) -> codeguruprofiler::CodeguruprofilerService<'_> {
        codeguruprofiler::CodeguruprofilerService::new(self)
    }
    /// Get kinesis service handler
    pub fn kinesis(&self) -> kinesis::KinesisService<'_> {
        kinesis::KinesisService::new(self)
    }
    /// Get pinpoint service handler
    pub fn pinpoint(&self) -> pinpoint::PinpointService<'_> {
        pinpoint::PinpointService::new(self)
    }
    /// Get chime service handler
    pub fn chime(&self) -> chime::ChimeService<'_> {
        chime::ChimeService::new(self)
    }
    /// Get iottwinmaker service handler
    pub fn iottwinmaker(&self) -> iottwinmaker::IottwinmakerService<'_> {
        iottwinmaker::IottwinmakerService::new(self)
    }
    /// Get organizations service handler
    pub fn organizations(&self) -> organizations::OrganizationsService<'_> {
        organizations::OrganizationsService::new(self)
    }
    /// Get networkflowmonitor service handler
    pub fn networkflowmonitor(&self) -> networkflowmonitor::NetworkflowmonitorService<'_> {
        networkflowmonitor::NetworkflowmonitorService::new(self)
    }
    /// Get shield service handler
    pub fn shield(&self) -> shield::ShieldService<'_> {
        shield::ShieldService::new(self)
    }
    /// Get ssm service handler
    pub fn ssm(&self) -> ssm::SsmService<'_> {
        ssm::SsmService::new(self)
    }
    /// Get arc service handler
    pub fn arc(&self) -> arc::ArcService<'_> {
        arc::ArcService::new(self)
    }
    /// Get workspaces service handler
    pub fn workspaces(&self) -> workspaces::WorkspacesService<'_> {
        workspaces::WorkspacesService::new(self)
    }
    /// Get partnercentral service handler
    pub fn partnercentral(&self) -> partnercentral::PartnercentralService<'_> {
        partnercentral::PartnercentralService::new(self)
    }
    /// Get geo service handler
    pub fn geo(&self) -> geo::GeoService<'_> {
        geo::GeoService::new(self)
    }
    /// Get signer service handler
    pub fn signer(&self) -> signer::SignerService<'_> {
        signer::SignerService::new(self)
    }
    /// Get marketplace service handler
    pub fn marketplace(&self) -> marketplace::MarketplaceService<'_> {
        marketplace::MarketplaceService::new(self)
    }
    /// Get lakeformation service handler
    pub fn lakeformation(&self) -> lakeformation::LakeformationService<'_> {
        lakeformation::LakeformationService::new(self)
    }
    /// Get pcs service handler
    pub fn pcs(&self) -> pcs::PcsService<'_> {
        pcs::PcsService::new(self)
    }
    /// Get kinesis service handler
    pub fn kinesis(&self) -> kinesis::KinesisService<'_> {
        kinesis::KinesisService::new(self)
    }
    /// Get mediaconnect service handler
    pub fn mediaconnect(&self) -> mediaconnect::MediaconnectService<'_> {
        mediaconnect::MediaconnectService::new(self)
    }
    /// Get secrets service handler
    pub fn secrets(&self) -> secrets::SecretsService<'_> {
        secrets::SecretsService::new(self)
    }
    /// Get mwaa service handler
    pub fn mwaa(&self) -> mwaa::MwaaService<'_> {
        mwaa::MwaaService::new(self)
    }
    /// Get kms service handler
    pub fn kms(&self) -> kms::KmsService<'_> {
        kms::KmsService::new(self)
    }
    /// Get quicksight service handler
    pub fn quicksight(&self) -> quicksight::QuicksightService<'_> {
        quicksight::QuicksightService::new(self)
    }
    /// Get workmail service handler
    pub fn workmail(&self) -> workmail::WorkmailService<'_> {
        workmail::WorkmailService::new(self)
    }
    /// Get eventbridge service handler
    pub fn eventbridge(&self) -> eventbridge::EventbridgeService<'_> {
        eventbridge::EventbridgeService::new(self)
    }
    /// Get frauddetector service handler
    pub fn frauddetector(&self) -> frauddetector::FrauddetectorService<'_> {
        frauddetector::FrauddetectorService::new(self)
    }
    /// Get cloudwatch service handler
    pub fn cloudwatch(&self) -> cloudwatch::CloudwatchService<'_> {
        cloudwatch::CloudwatchService::new(self)
    }
    /// Get cloudhsm service handler
    pub fn cloudhsm(&self) -> cloudhsm::CloudhsmService<'_> {
        cloudhsm::CloudhsmService::new(self)
    }
    /// Get cloudsearch service handler
    pub fn cloudsearch(&self) -> cloudsearch::CloudsearchService<'_> {
        cloudsearch::CloudsearchService::new(self)
    }
    /// Get lookoutequipment service handler
    pub fn lookoutequipment(&self) -> lookoutequipment::LookoutequipmentService<'_> {
        lookoutequipment::LookoutequipmentService::new(self)
    }
    /// Get iot service handler
    pub fn iot(&self) -> iot::IotService<'_> {
        iot::IotService::new(self)
    }
    /// Get securitylake service handler
    pub fn securitylake(&self) -> securitylake::SecuritylakeService<'_> {
        securitylake::SecuritylakeService::new(self)
    }
    /// Get cloudwatch service handler
    pub fn cloudwatch(&self) -> cloudwatch::CloudwatchService<'_> {
        cloudwatch::CloudwatchService::new(self)
    }
    /// Get glue service handler
    pub fn glue(&self) -> glue::GlueService<'_> {
        glue::GlueService::new(self)
    }
    /// Get application service handler
    pub fn application(&self) -> application::ApplicationService<'_> {
        application::ApplicationService::new(self)
    }
    /// Get personalize service handler
    pub fn personalize(&self) -> personalize::PersonalizeService<'_> {
        personalize::PersonalizeService::new(self)
    }
    /// Get voice service handler
    pub fn voice(&self) -> voice::VoiceService<'_> {
        voice::VoiceService::new(self)
    }
    /// Get s3 service handler
    pub fn s3(&self) -> s3::S3Service<'_> {
        s3::S3Service::new(self)
    }
    /// Get appintegrations service handler
    pub fn appintegrations(&self) -> appintegrations::AppintegrationsService<'_> {
        appintegrations::AppintegrationsService::new(self)
    }
    /// Get lex service handler
    pub fn lex(&self) -> lex::LexService<'_> {
        lex::LexService::new(self)
    }
    /// Get ssm service handler
    pub fn ssm(&self) -> ssm::SsmService<'_> {
        ssm::SsmService::new(self)
    }
    /// Get chime service handler
    pub fn chime(&self) -> chime::ChimeService<'_> {
        chime::ChimeService::new(self)
    }
    /// Get sesv2 service handler
    pub fn sesv2(&self) -> sesv2::Sesv2Service<'_> {
        sesv2::Sesv2Service::new(self)
    }
    /// Get bedrock service handler
    pub fn bedrock(&self) -> bedrock::BedrockService<'_> {
        bedrock::BedrockService::new(self)
    }
    /// Get emr service handler
    pub fn emr(&self) -> emr::EmrService<'_> {
        emr::EmrService::new(self)
    }
    /// Get controltower service handler
    pub fn controltower(&self) -> controltower::ControltowerService<'_> {
        controltower::ControltowerService::new(self)
    }
    /// Get resource service handler
    pub fn resource(&self) -> resource::ResourceService<'_> {
        resource::ResourceService::new(self)
    }
    /// Get s3 service handler
    pub fn s3(&self) -> s3::S3Service<'_> {
        s3::S3Service::new(self)
    }
    /// Get rtbfabric service handler
    pub fn rtbfabric(&self) -> rtbfabric::RtbfabricService<'_> {
        rtbfabric::RtbfabricService::new(self)
    }
    /// Get personalize service handler
    pub fn personalize(&self) -> personalize::PersonalizeService<'_> {
        personalize::PersonalizeService::new(self)
    }
    /// Get marketplace service handler
    pub fn marketplace(&self) -> marketplace::MarketplaceService<'_> {
        marketplace::MarketplaceService::new(self)
    }
    /// Get directory service handler
    pub fn directory(&self) -> directory::DirectoryService<'_> {
        directory::DirectoryService::new(self)
    }
    /// Get ssm service handler
    pub fn ssm(&self) -> ssm::SsmService<'_> {
        ssm::SsmService::new(self)
    }
    /// Get outposts service handler
    pub fn outposts(&self) -> outposts::OutpostsService<'_> {
        outposts::OutpostsService::new(self)
    }
    /// Get workdocs service handler
    pub fn workdocs(&self) -> workdocs::WorkdocsService<'_> {
        workdocs::WorkdocsService::new(self)
    }
    /// Get networkmanager service handler
    pub fn networkmanager(&self) -> networkmanager::NetworkmanagerService<'_> {
        networkmanager::NetworkmanagerService::new(self)
    }
    /// Get kinesis service handler
    pub fn kinesis(&self) -> kinesis::KinesisService<'_> {
        kinesis::KinesisService::new(self)
    }
    /// Get socialmessaging service handler
    pub fn socialmessaging(&self) -> socialmessaging::SocialmessagingService<'_> {
        socialmessaging::SocialmessagingService::new(self)
    }
    /// Get bedrock service handler
    pub fn bedrock(&self) -> bedrock::BedrockService<'_> {
        bedrock::BedrockService::new(self)
    }
    /// Get omics service handler
    pub fn omics(&self) -> omics::OmicsService<'_> {
        omics::OmicsService::new(self)
    }
    /// Get bcm service handler
    pub fn bcm(&self) -> bcm::BcmService<'_> {
        bcm::BcmService::new(self)
    }
    /// Get mediapackage service handler
    pub fn mediapackage(&self) -> mediapackage::MediapackageService<'_> {
        mediapackage::MediapackageService::new(self)
    }
    /// Get sagemaker service handler
    pub fn sagemaker(&self) -> sagemaker::SagemakerService<'_> {
        sagemaker::SagemakerService::new(self)
    }
    /// Get config service handler
    pub fn config(&self) -> config::ConfigService<'_> {
        config::ConfigService::new(self)
    }
    /// Get cloudwatch service handler
    pub fn cloudwatch(&self) -> cloudwatch::CloudwatchService<'_> {
        cloudwatch::CloudwatchService::new(self)
    }
    /// Get medialive service handler
    pub fn medialive(&self) -> medialive::MedialiveService<'_> {
        medialive::MedialiveService::new(self)
    }
    /// Get backup service handler
    pub fn backup(&self) -> backup::BackupService<'_> {
        backup::BackupService::new(self)
    }
    /// Get connect service handler
    pub fn connect(&self) -> connect::ConnectService<'_> {
        connect::ConnectService::new(self)
    }
    /// Get mediaconvert service handler
    pub fn mediaconvert(&self) -> mediaconvert::MediaconvertService<'_> {
        mediaconvert::MediaconvertService::new(self)
    }
    /// Get sns service handler
    pub fn sns(&self) -> sns::SnsService<'_> {
        sns::SnsService::new(self)
    }
    /// Get evs service handler
    pub fn evs(&self) -> evs::EvsService<'_> {
        evs::EvsService::new(self)
    }
    /// Get datasync service handler
    pub fn datasync(&self) -> datasync::DatasyncService<'_> {
        datasync::DatasyncService::new(self)
    }
    /// Get greengrassv2 service handler
    pub fn greengrassv2(&self) -> greengrassv2::Greengrassv2Service<'_> {
        greengrassv2::Greengrassv2Service::new(self)
    }
    /// Get cleanroomsml service handler
    pub fn cleanroomsml(&self) -> cleanroomsml::CleanroomsmlService<'_> {
        cleanroomsml::CleanroomsmlService::new(self)
    }
    /// Get neptunedata service handler
    pub fn neptunedata(&self) -> neptunedata::NeptunedataService<'_> {
        neptunedata::NeptunedataService::new(self)
    }
    /// Get acm service handler
    pub fn acm(&self) -> acm::AcmService<'_> {
        acm::AcmService::new(self)
    }
    /// Get service service handler
    pub fn service(&self) -> service::ServiceService<'_> {
        service::ServiceService::new(self)
    }
    /// Get b2bi service handler
    pub fn b2bi(&self) -> b2bi::B2biService<'_> {
        b2bi::B2biService::new(self)
    }
    /// Get iotanalytics service handler
    pub fn iotanalytics(&self) -> iotanalytics::IotanalyticsService<'_> {
        iotanalytics::IotanalyticsService::new(self)
    }
    /// Get lex service handler
    pub fn lex(&self) -> lex::LexService<'_> {
        lex::LexService::new(self)
    }
    /// Get inspector2 service handler
    pub fn inspector2(&self) -> inspector2::Inspector2Service<'_> {
        inspector2::Inspector2Service::new(self)
    }
    /// Get groundstation service handler
    pub fn groundstation(&self) -> groundstation::GroundstationService<'_> {
        groundstation::GroundstationService::new(self)
    }
    /// Get ecr service handler
    pub fn ecr(&self) -> ecr::EcrService<'_> {
        ecr::EcrService::new(self)
    }
    /// Get fis service handler
    pub fn fis(&self) -> fis::FisService<'_> {
        fis::FisService::new(self)
    }
    /// Get proton service handler
    pub fn proton(&self) -> proton::ProtonService<'_> {
        proton::ProtonService::new(self)
    }
    /// Get api service handler
    pub fn api(&self) -> api::ApiService<'_> {
        api::ApiService::new(self)
    }
    /// Get cloudhsm service handler
    pub fn cloudhsm(&self) -> cloudhsm::CloudhsmService<'_> {
        cloudhsm::CloudhsmService::new(self)
    }
    /// Get mpa service handler
    pub fn mpa(&self) -> mpa::MpaService<'_> {
        mpa::MpaService::new(self)
    }
    /// Get osis service handler
    pub fn osis(&self) -> osis::OsisService<'_> {
        osis::OsisService::new(self)
    }
    /// Get memorydb service handler
    pub fn memorydb(&self) -> memorydb::MemorydbService<'_> {
        memorydb::MemorydbService::new(self)
    }
    /// Get inspector service handler
    pub fn inspector(&self) -> inspector::InspectorService<'_> {
        inspector::InspectorService::new(self)
    }
    /// Get translate service handler
    pub fn translate(&self) -> translate::TranslateService<'_> {
        translate::TranslateService::new(self)
    }
    /// Get mailmanager service handler
    pub fn mailmanager(&self) -> mailmanager::MailmanagerService<'_> {
        mailmanager::MailmanagerService::new(self)
    }
    /// Get neptune service handler
    pub fn neptune(&self) -> neptune::NeptuneService<'_> {
        neptune::NeptuneService::new(self)
    }
    /// Get chatbot service handler
    pub fn chatbot(&self) -> chatbot::ChatbotService<'_> {
        chatbot::ChatbotService::new(self)
    }
    /// Get fms service handler
    pub fn fms(&self) -> fms::FmsService<'_> {
        fms::FmsService::new(self)
    }
    /// Get qapps service handler
    pub fn qapps(&self) -> qapps::QappsService<'_> {
        qapps::QappsService::new(self)
    }
    /// Get customer service handler
    pub fn customer(&self) -> customer::CustomerService<'_> {
        customer::CustomerService::new(self)
    }
    /// Get geo service handler
    pub fn geo(&self) -> geo::GeoService<'_> {
        geo::GeoService::new(self)
    }
    /// Get route service handler
    pub fn route(&self) -> route::RouteService<'_> {
        route::RouteService::new(self)
    }
    /// Get service service handler
    pub fn service(&self) -> service::ServiceService<'_> {
        service::ServiceService::new(self)
    }
    /// Get qbusiness service handler
    pub fn qbusiness(&self) -> qbusiness::QbusinessService<'_> {
        qbusiness::QbusinessService::new(self)
    }
    /// Get synthetics service handler
    pub fn synthetics(&self) -> synthetics::SyntheticsService<'_> {
        synthetics::SyntheticsService::new(self)
    }
    /// Get codecatalyst service handler
    pub fn codecatalyst(&self) -> codecatalyst::CodecatalystService<'_> {
        codecatalyst::CodecatalystService::new(self)
    }
    /// Get keyspacesstreams service handler
    pub fn keyspacesstreams(&self) -> keyspacesstreams::KeyspacesstreamsService<'_> {
        keyspacesstreams::KeyspacesstreamsService::new(self)
    }
    /// Get storage service handler
    pub fn storage(&self) -> storage::StorageService<'_> {
        storage::StorageService::new(self)
    }
    /// Get elastic service handler
    pub fn elastic(&self) -> elastic::ElasticService<'_> {
        elastic::ElasticService::new(self)
    }
    /// Get bcm service handler
    pub fn bcm(&self) -> bcm::BcmService<'_> {
        bcm::BcmService::new(self)
    }
    /// Get iotsecuretunneling service handler
    pub fn iotsecuretunneling(&self) -> iotsecuretunneling::IotsecuretunnelingService<'_> {
        iotsecuretunneling::IotsecuretunnelingService::new(self)
    }
    /// Get cloudfront service handler
    pub fn cloudfront(&self) -> cloudfront::CloudfrontService<'_> {
        cloudfront::CloudfrontService::new(self)
    }
    /// Get bedrock service handler
    pub fn bedrock(&self) -> bedrock::BedrockService<'_> {
        bedrock::BedrockService::new(self)
    }
    /// Get location service handler
    pub fn location(&self) -> location::LocationService<'_> {
        location::LocationService::new(self)
    }
    /// Get wafv2 service handler
    pub fn wafv2(&self) -> wafv2::Wafv2Service<'_> {
        wafv2::Wafv2Service::new(self)
    }
    /// Get opensearch service handler
    pub fn opensearch(&self) -> opensearch::OpensearchService<'_> {
        opensearch::OpensearchService::new(self)
    }
    /// Get iotthingsgraph service handler
    pub fn iotthingsgraph(&self) -> iotthingsgraph::IotthingsgraphService<'_> {
        iotthingsgraph::IotthingsgraphService::new(self)
    }
    /// Get security service handler
    pub fn security(&self) -> security::SecurityService<'_> {
        security::SecurityService::new(self)
    }
    /// Get repostspace service handler
    pub fn repostspace(&self) -> repostspace::RepostspaceService<'_> {
        repostspace::RepostspaceService::new(self)
    }
    /// Get health service handler
    pub fn health(&self) -> health::HealthService<'_> {
        health::HealthService::new(self)
    }
    /// Get workmailmessageflow service handler
    pub fn workmailmessageflow(&self) -> workmailmessageflow::WorkmailmessageflowService<'_> {
        workmailmessageflow::WorkmailmessageflowService::new(self)
    }
    /// Get mediastore service handler
    pub fn mediastore(&self) -> mediastore::MediastoreService<'_> {
        mediastore::MediastoreService::new(self)
    }
    /// Get ec2 service handler
    pub fn ec2(&self) -> ec2::Ec2Service<'_> {
        ec2::Ec2Service::new(self)
    }
    /// Get comprehendmedical service handler
    pub fn comprehendmedical(&self) -> comprehendmedical::ComprehendmedicalService<'_> {
        comprehendmedical::ComprehendmedicalService::new(self)
    }
    /// Get iotfleetwise service handler
    pub fn iotfleetwise(&self) -> iotfleetwise::IotfleetwiseService<'_> {
        iotfleetwise::IotfleetwiseService::new(self)
    }
    /// Get route53profiles service handler
    pub fn route53profiles(&self) -> route53profiles::Route53profilesService<'_> {
        route53profiles::Route53profilesService::new(self)
    }
    /// Get application service handler
    pub fn application(&self) -> application::ApplicationService<'_> {
        application::ApplicationService::new(self)
    }
    /// Get resource service handler
    pub fn resource(&self) -> resource::ResourceService<'_> {
        resource::ResourceService::new(self)
    }
    /// Get accessanalyzer service handler
    pub fn accessanalyzer(&self) -> accessanalyzer::AccessanalyzerService<'_> {
        accessanalyzer::AccessanalyzerService::new(self)
    }
    /// Get glacier service handler
    pub fn glacier(&self) -> glacier::GlacierService<'_> {
        glacier::GlacierService::new(self)
    }
    /// Get lightsail service handler
    pub fn lightsail(&self) -> lightsail::LightsailService<'_> {
        lightsail::LightsailService::new(self)
    }
    /// Get rum service handler
    pub fn rum(&self) -> rum::RumService<'_> {
        rum::RumService::new(self)
    }
    /// Get direct service handler
    pub fn direct(&self) -> direct::DirectService<'_> {
        direct::DirectService::new(self)
    }
    /// Get elastic service handler
    pub fn elastic(&self) -> elastic::ElasticService<'_> {
        elastic::ElasticService::new(self)
    }
    /// Get imagebuilder service handler
    pub fn imagebuilder(&self) -> imagebuilder::ImagebuilderService<'_> {
        imagebuilder::ImagebuilderService::new(self)
    }
    /// Get simspaceweaver service handler
    pub fn simspaceweaver(&self) -> simspaceweaver::SimspaceweaverService<'_> {
        simspaceweaver::SimspaceweaverService::new(self)
    }
    /// Get iot service handler
    pub fn iot(&self) -> iot::IotService<'_> {
        iot::IotService::new(self)
    }
    /// Get freetier service handler
    pub fn freetier(&self) -> freetier::FreetierService<'_> {
        freetier::FreetierService::new(self)
    }
    /// Get bedrock service handler
    pub fn bedrock(&self) -> bedrock::BedrockService<'_> {
        bedrock::BedrockService::new(self)
    }
    /// Get neptune service handler
    pub fn neptune(&self) -> neptune::NeptuneService<'_> {
        neptune::NeptuneService::new(self)
    }
    /// Get transfer service handler
    pub fn transfer(&self) -> transfer::TransferService<'_> {
        transfer::TransferService::new(self)
    }
    /// Get deadline service handler
    pub fn deadline(&self) -> deadline::DeadlineService<'_> {
        deadline::DeadlineService::new(self)
    }
    /// Get braket service handler
    pub fn braket(&self) -> braket::BraketService<'_> {
        braket::BraketService::new(self)
    }
    /// Get verifiedpermissions service handler
    pub fn verifiedpermissions(&self) -> verifiedpermissions::VerifiedpermissionsService<'_> {
        verifiedpermissions::VerifiedpermissionsService::new(self)
    }
    /// Get scheduler service handler
    pub fn scheduler(&self) -> scheduler::SchedulerService<'_> {
        scheduler::SchedulerService::new(self)
    }
    /// Get waf service handler
    pub fn waf(&self) -> waf::WafService<'_> {
        waf::WafService::new(self)
    }
    /// Get marketplace service handler
    pub fn marketplace(&self) -> marketplace::MarketplaceService<'_> {
        marketplace::MarketplaceService::new(self)
    }
    /// Get ecr service handler
    pub fn ecr(&self) -> ecr::EcrService<'_> {
        ecr::EcrService::new(self)
    }
    /// Get cost service handler
    pub fn cost(&self) -> cost::CostService<'_> {
        cost::CostService::new(self)
    }
    /// Get kinesis service handler
    pub fn kinesis(&self) -> kinesis::KinesisService<'_> {
        kinesis::KinesisService::new(self)
    }
    /// Get dynamodb service handler
    pub fn dynamodb(&self) -> dynamodb::DynamodbService<'_> {
        dynamodb::DynamodbService::new(self)
    }
    /// Get resiliencehub service handler
    pub fn resiliencehub(&self) -> resiliencehub::ResiliencehubService<'_> {
        resiliencehub::ResiliencehubService::new(self)
    }
    /// Get macie2 service handler
    pub fn macie2(&self) -> macie2::Macie2Service<'_> {
        macie2::Macie2Service::new(self)
    }
    /// Get entityresolution service handler
    pub fn entityresolution(&self) -> entityresolution::EntityresolutionService<'_> {
        entityresolution::EntityresolutionService::new(self)
    }
    /// Get s3outposts service handler
    pub fn s3outposts(&self) -> s3outposts::S3outpostsService<'_> {
        s3outposts::S3outpostsService::new(self)
    }
    /// Get grafana service handler
    pub fn grafana(&self) -> grafana::GrafanaService<'_> {
        grafana::GrafanaService::new(self)
    }
    /// Get kinesis service handler
    pub fn kinesis(&self) -> kinesis::KinesisService<'_> {
        kinesis::KinesisService::new(self)
    }
    /// Get eks service handler
    pub fn eks(&self) -> eks::EksService<'_> {
        eks::EksService::new(self)
    }
    /// Get rbin service handler
    pub fn rbin(&self) -> rbin::RbinService<'_> {
        rbin::RbinService::new(self)
    }
    /// Get service service handler
    pub fn service(&self) -> service::ServiceService<'_> {
        service::ServiceService::new(self)
    }
    /// Get ecs service handler
    pub fn ecs(&self) -> ecs::EcsService<'_> {
        ecs::EcsService::new(self)
    }
    /// Get marketplace service handler
    pub fn marketplace(&self) -> marketplace::MarketplaceService<'_> {
        marketplace::MarketplaceService::new(self)
    }
    /// Get timestream service handler
    pub fn timestream(&self) -> timestream::TimestreamService<'_> {
        timestream::TimestreamService::new(self)
    }
    /// Get qconnect service handler
    pub fn qconnect(&self) -> qconnect::QconnectService<'_> {
        qconnect::QconnectService::new(self)
    }
    /// Get cloudsearch service handler
    pub fn cloudsearch(&self) -> cloudsearch::CloudsearchService<'_> {
        cloudsearch::CloudsearchService::new(self)
    }
    /// Get bcm service handler
    pub fn bcm(&self) -> bcm::BcmService<'_> {
        bcm::BcmService::new(self)
    }
    /// Get appfabric service handler
    pub fn appfabric(&self) -> appfabric::AppfabricService<'_> {
        appfabric::AppfabricService::new(self)
    }
    /// Get route53resolver service handler
    pub fn route53resolver(&self) -> route53resolver::Route53resolverService<'_> {
        route53resolver::Route53resolverService::new(self)
    }
    /// Get marketplace service handler
    pub fn marketplace(&self) -> marketplace::MarketplaceService<'_> {
        marketplace::MarketplaceService::new(self)
    }
    /// Get workspaces service handler
    pub fn workspaces(&self) -> workspaces::WorkspacesService<'_> {
        workspaces::WorkspacesService::new(self)
    }
    /// Get gameliftstreams service handler
    pub fn gameliftstreams(&self) -> gameliftstreams::GameliftstreamsService<'_> {
        gameliftstreams::GameliftstreamsService::new(self)
    }
    /// Get taxsettings service handler
    pub fn taxsettings(&self) -> taxsettings::TaxsettingsService<'_> {
        taxsettings::TaxsettingsService::new(self)
    }
    /// Get pinpoint service handler
    pub fn pinpoint(&self) -> pinpoint::PinpointService<'_> {
        pinpoint::PinpointService::new(self)
    }
    /// Get fsx service handler
    pub fn fsx(&self) -> fsx::FsxService<'_> {
        fsx::FsxService::new(self)
    }
    /// Get codepipeline service handler
    pub fn codepipeline(&self) -> codepipeline::CodepipelineService<'_> {
        codepipeline::CodepipelineService::new(self)
    }
    /// Get schemas service handler
    pub fn schemas(&self) -> schemas::SchemasService<'_> {
        schemas::SchemasService::new(self)
    }
    /// Get emr service handler
    pub fn emr(&self) -> emr::EmrService<'_> {
        emr::EmrService::new(self)
    }
    /// Get sqs service handler
    pub fn sqs(&self) -> sqs::SqsService<'_> {
        sqs::SqsService::new(self)
    }
    /// Get license service handler
    pub fn license(&self) -> license::LicenseService<'_> {
        license::LicenseService::new(self)
    }
    /// Get route53 service handler
    pub fn route53(&self) -> route53::Route53Service<'_> {
        route53::Route53Service::new(self)
    }
    /// Get migrationhuborchestrator service handler
    pub fn migrationhuborchestrator(&self) -> migrationhuborchestrator::MigrationhuborchestratorService<'_> {
        migrationhuborchestrator::MigrationhuborchestratorService::new(self)
    }
    /// Get iot service handler
    pub fn iot(&self) -> iot::IotService<'_> {
        iot::IotService::new(self)
    }
    /// Get sso service handler
    pub fn sso(&self) -> sso::SsoService<'_> {
        sso::SsoService::new(self)
    }
    /// Get codestar service handler
    pub fn codestar(&self) -> codestar::CodestarService<'_> {
        codestar::CodestarService::new(self)
    }
    /// Get ebs service handler
    pub fn ebs(&self) -> ebs::EbsService<'_> {
        ebs::EbsService::new(self)
    }
    /// Get aiops service handler
    pub fn aiops(&self) -> aiops::AiopsService<'_> {
        aiops::AiopsService::new(self)
    }
    /// Get amplify service handler
    pub fn amplify(&self) -> amplify::AmplifyService<'_> {
        amplify::AmplifyService::new(self)
    }
    /// Get cloudcontrol service handler
    pub fn cloudcontrol(&self) -> cloudcontrol::CloudcontrolService<'_> {
        cloudcontrol::CloudcontrolService::new(self)
    }
    /// Get wellarchitected service handler
    pub fn wellarchitected(&self) -> wellarchitected::WellarchitectedService<'_> {
        wellarchitected::WellarchitectedService::new(self)
    }
    /// Get route service handler
    pub fn route(&self) -> route::RouteService<'_> {
        route::RouteService::new(self)
    }
    /// Get ssm service handler
    pub fn ssm(&self) -> ssm::SsmService<'_> {
        ssm::SsmService::new(self)
    }
    /// Get bedrock service handler
    pub fn bedrock(&self) -> bedrock::BedrockService<'_> {
        bedrock::BedrockService::new(self)
    }
    /// Get ivs service handler
    pub fn ivs(&self) -> ivs::IvsService<'_> {
        ivs::IvsService::new(self)
    }
    /// Get migrationhub service handler
    pub fn migrationhub(&self) -> migrationhub::MigrationhubService<'_> {
        migrationhub::MigrationhubService::new(self)
    }
    /// Get redshift service handler
    pub fn redshift(&self) -> redshift::RedshiftService<'_> {
        redshift::RedshiftService::new(self)
    }
    /// Get inspector service handler
    pub fn inspector(&self) -> inspector::InspectorService<'_> {
        inspector::InspectorService::new(self)
    }
    /// Get connectcases service handler
    pub fn connectcases(&self) -> connectcases::ConnectcasesService<'_> {
        connectcases::ConnectcasesService::new(self)
    }
    /// Get kinesis service handler
    pub fn kinesis(&self) -> kinesis::KinesisService<'_> {
        kinesis::KinesisService::new(self)
    }
    /// Get pca service handler
    pub fn pca(&self) -> pca::PcaService<'_> {
        pca::PcaService::new(self)
    }
    /// Get appflow service handler
    pub fn appflow(&self) -> appflow::AppflowService<'_> {
        appflow::AppflowService::new(self)
    }
    /// Get gamelift service handler
    pub fn gamelift(&self) -> gamelift::GameliftService<'_> {
        gamelift::GameliftService::new(self)
    }
    /// Get cloudtrail service handler
    pub fn cloudtrail(&self) -> cloudtrail::CloudtrailService<'_> {
        cloudtrail::CloudtrailService::new(self)
    }
    /// Get resource service handler
    pub fn resource(&self) -> resource::ResourceService<'_> {
        resource::ResourceService::new(self)
    }
    /// Get supplychain service handler
    pub fn supplychain(&self) -> supplychain::SupplychainService<'_> {
        supplychain::SupplychainService::new(self)
    }
    /// Get timestream service handler
    pub fn timestream(&self) -> timestream::TimestreamService<'_> {
        timestream::TimestreamService::new(self)
    }
    /// Get pipes service handler
    pub fn pipes(&self) -> pipes::PipesService<'_> {
        pipes::PipesService::new(self)
    }
    /// Get evidently service handler
    pub fn evidently(&self) -> evidently::EvidentlyService<'_> {
        evidently::EvidentlyService::new(self)
    }
    /// Get codeguru service handler
    pub fn codeguru(&self) -> codeguru::CodeguruService<'_> {
        codeguru::CodeguruService::new(self)
    }
    /// Get cost service handler
    pub fn cost(&self) -> cost::CostService<'_> {
        cost::CostService::new(self)
    }
    /// Get amplifyuibuilder service handler
    pub fn amplifyuibuilder(&self) -> amplifyuibuilder::AmplifyuibuilderService<'_> {
        amplifyuibuilder::AmplifyuibuilderService::new(self)
    }
    /// Get route53 service handler
    pub fn route53(&self) -> route53::Route53Service<'_> {
        route53::Route53Service::new(self)
    }
    /// Get vpc service handler
    pub fn vpc(&self) -> vpc::VpcService<'_> {
        vpc::VpcService::new(self)
    }
    /// Get managedblockchain service handler
    pub fn managedblockchain(&self) -> managedblockchain::ManagedblockchainService<'_> {
        managedblockchain::ManagedblockchainService::new(self)
    }
    /// Get redshift service handler
    pub fn redshift(&self) -> redshift::RedshiftService<'_> {
        redshift::RedshiftService::new(self)
    }
    /// Get mediatailor service handler
    pub fn mediatailor(&self) -> mediatailor::MediatailorService<'_> {
        mediatailor::MediatailorService::new(self)
    }
    /// Get mediapackagev2 service handler
    pub fn mediapackagev2(&self) -> mediapackagev2::Mediapackagev2Service<'_> {
        mediapackagev2::Mediapackagev2Service::new(self)
    }
    /// Get pi service handler
    pub fn pi(&self) -> pi::PiService<'_> {
        pi::PiService::new(self)
    }
    /// Get appconfig service handler
    pub fn appconfig(&self) -> appconfig::AppconfigService<'_> {
        appconfig::AppconfigService::new(self)
    }
    /// Get networkmonitor service handler
    pub fn networkmonitor(&self) -> networkmonitor::NetworkmonitorService<'_> {
        networkmonitor::NetworkmonitorService::new(self)
    }
    /// Get network service handler
    pub fn network(&self) -> network::NetworkService<'_> {
        network::NetworkService::new(self)
    }
    /// Get connectparticipant service handler
    pub fn connectparticipant(&self) -> connectparticipant::ConnectparticipantService<'_> {
        connectparticipant::ConnectparticipantService::new(self)
    }
    /// Get mgn service handler
    pub fn mgn(&self) -> mgn::MgnService<'_> {
        mgn::MgnService::new(self)
    }
    /// Get sagemaker service handler
    pub fn sagemaker(&self) -> sagemaker::SagemakerService<'_> {
        sagemaker::SagemakerService::new(self)
    }
    /// Get applicationcostprofiler service handler
    pub fn applicationcostprofiler(&self) -> applicationcostprofiler::ApplicationcostprofilerService<'_> {
        applicationcostprofiler::ApplicationcostprofilerService::new(self)
    }
    /// Get keyspaces service handler
    pub fn keyspaces(&self) -> keyspaces::KeyspacesService<'_> {
        keyspaces::KeyspacesService::new(self)
    }
    /// Get iam service handler
    pub fn iam(&self) -> iam::IamService<'_> {
        iam::IamService::new(self)
    }
    /// Get data service handler
    pub fn data(&self) -> data::DataService<'_> {
        data::DataService::new(self)
    }
    /// Get odb service handler
    pub fn odb(&self) -> odb::OdbService<'_> {
        odb::OdbService::new(self)
    }
    /// Get chime service handler
    pub fn chime(&self) -> chime::ChimeService<'_> {
        chime::ChimeService::new(self)
    }
    /// Get mediastore service handler
    pub fn mediastore(&self) -> mediastore::MediastoreService<'_> {
        mediastore::MediastoreService::new(self)
    }
    /// Get iot service handler
    pub fn iot(&self) -> iot::IotService<'_> {
        iot::IotService::new(self)
    }
    /// Get cloud9 service handler
    pub fn cloud9(&self) -> cloud9::Cloud9Service<'_> {
        cloud9::Cloud9Service::new(self)
    }
    /// Get wisdom service handler
    pub fn wisdom(&self) -> wisdom::WisdomService<'_> {
        wisdom::WisdomService::new(self)
    }
    /// Get sagemaker service handler
    pub fn sagemaker(&self) -> sagemaker::SagemakerService<'_> {
        sagemaker::SagemakerService::new(self)
    }
    /// Get sso service handler
    pub fn sso(&self) -> sso::SsoService<'_> {
        sso::SsoService::new(self)
    }
    /// Get auditmanager service handler
    pub fn auditmanager(&self) -> auditmanager::AuditmanagerService<'_> {
        auditmanager::AuditmanagerService::new(self)
    }
    /// Get snowball service handler
    pub fn snowball(&self) -> snowball::SnowballService<'_> {
        snowball::SnowballService::new(self)
    }
    /// Get migration service handler
    pub fn migration(&self) -> migration::MigrationService<'_> {
        migration::MigrationService::new(self)
    }
    /// Get identitystore service handler
    pub fn identitystore(&self) -> identitystore::IdentitystoreService<'_> {
        identitystore::IdentitystoreService::new(self)
    }
    /// Get elastic service handler
    pub fn elastic(&self) -> elastic::ElasticService<'_> {
        elastic::ElasticService::new(self)
    }
    /// Get connectcampaigns service handler
    pub fn connectcampaigns(&self) -> connectcampaigns::ConnectcampaignsService<'_> {
        connectcampaigns::ConnectcampaignsService::new(self)
    }
    /// Get textract service handler
    pub fn textract(&self) -> textract::TextractService<'_> {
        textract::TextractService::new(self)
    }
    /// Get compute service handler
    pub fn compute(&self) -> compute::ComputeService<'_> {
        compute::ComputeService::new(self)
    }
    /// Get s3tables service handler
    pub fn s3tables(&self) -> s3tables::S3tablesService<'_> {
        s3tables::S3tablesService::new(self)
    }
    /// Get eks service handler
    pub fn eks(&self) -> eks::EksService<'_> {
        eks::EksService::new(self)
    }
    /// Get support service handler
    pub fn support(&self) -> support::SupportService<'_> {
        support::SupportService::new(self)
    }
    /// Get mturk service handler
    pub fn mturk(&self) -> mturk::MturkService<'_> {
        mturk::MturkService::new(self)
    }
    /// Get apigatewayv2 service handler
    pub fn apigatewayv2(&self) -> apigatewayv2::Apigatewayv2Service<'_> {
        apigatewayv2::Apigatewayv2Service::new(self)
    }
    /// Get cognito service handler
    pub fn cognito(&self) -> cognito::CognitoService<'_> {
        cognito::CognitoService::new(self)
    }
    /// Get bedrock service handler
    pub fn bedrock(&self) -> bedrock::BedrockService<'_> {
        bedrock::BedrockService::new(self)
    }
    /// Get pinpoint service handler
    pub fn pinpoint(&self) -> pinpoint::PinpointService<'_> {
        pinpoint::PinpointService::new(self)
    }
    /// Get amp service handler
    pub fn amp(&self) -> amp::AmpService<'_> {
        amp::AmpService::new(self)
    }
    /// Get drs service handler
    pub fn drs(&self) -> drs::DrsService<'_> {
        drs::DrsService::new(self)
    }
    /// Get payment service handler
    pub fn payment(&self) -> payment::PaymentService<'_> {
        payment::PaymentService::new(self)
    }
    /// Get kafkaconnect service handler
    pub fn kafkaconnect(&self) -> kafkaconnect::KafkaconnectService<'_> {
        kafkaconnect::KafkaconnectService::new(self)
    }
    /// Get kafka service handler
    pub fn kafka(&self) -> kafka::KafkaService<'_> {
        kafka::KafkaService::new(self)
    }
    /// Get databrew service handler
    pub fn databrew(&self) -> databrew::DatabrewService<'_> {
        databrew::DatabrewService::new(self)
    }
    /// Get support service handler
    pub fn support(&self) -> support::SupportService<'_> {
        support::SupportService::new(self)
    }
    /// Get codedeploy service handler
    pub fn codedeploy(&self) -> codedeploy::CodedeployService<'_> {
        codedeploy::CodedeployService::new(self)
    }
    /// Get batch service handler
    pub fn batch(&self) -> batch::BatchService<'_> {
        batch::BatchService::new(self)
    }
    /// Get savingsplans service handler
    pub fn savingsplans(&self) -> savingsplans::SavingsplansService<'_> {
        savingsplans::SavingsplansService::new(self)
    }
    /// Get bedrock service handler
    pub fn bedrock(&self) -> bedrock::BedrockService<'_> {
        bedrock::BedrockService::new(self)
    }
    /// Get directory service handler
    pub fn directory(&self) -> directory::DirectoryService<'_> {
        directory::DirectoryService::new(self)
    }
    /// Get workspaces service handler
    pub fn workspaces(&self) -> workspaces::WorkspacesService<'_> {
        workspaces::WorkspacesService::new(self)
    }
    /// Get chime service handler
    pub fn chime(&self) -> chime::ChimeService<'_> {
        chime::ChimeService::new(self)
    }
    /// Get migrationhubstrategy service handler
    pub fn migrationhubstrategy(&self) -> migrationhubstrategy::MigrationhubstrategyService<'_> {
        migrationhubstrategy::MigrationhubstrategyService::new(self)
    }
    /// Get timestream service handler
    pub fn timestream(&self) -> timestream::TimestreamService<'_> {
        timestream::TimestreamService::new(self)
    }
    /// Get codeguru service handler
    pub fn codeguru(&self) -> codeguru::CodeguruService<'_> {
        codeguru::CodeguruService::new(self)
    }
    /// Get appsync service handler
    pub fn appsync(&self) -> appsync::AppsyncService<'_> {
        appsync::AppsyncService::new(self)
    }
    /// Get dlm service handler
    pub fn dlm(&self) -> dlm::DlmService<'_> {
        dlm::DlmService::new(self)
    }
    /// Get iot service handler
    pub fn iot(&self) -> iot::IotService<'_> {
        iot::IotService::new(self)
    }
    /// Get global service handler
    pub fn global(&self) -> global::GlobalService<'_> {
        global::GlobalService::new(self)
    }
    /// Get amplifybackend service handler
    pub fn amplifybackend(&self) -> amplifybackend::AmplifybackendService<'_> {
        amplifybackend::AmplifybackendService::new(self)
    }
    /// Get datazone service handler
    pub fn datazone(&self) -> datazone::DatazoneService<'_> {
        datazone::DatazoneService::new(self)
    }
    /// Get connectcampaignsv2 service handler
    pub fn connectcampaignsv2(&self) -> connectcampaignsv2::Connectcampaignsv2Service<'_> {
        connectcampaignsv2::Connectcampaignsv2Service::new(self)
    }
    /// Get billingconductor service handler
    pub fn billingconductor(&self) -> billingconductor::BillingconductorService<'_> {
        billingconductor::BillingconductorService::new(self)
    }
    /// Get budgets service handler
    pub fn budgets(&self) -> budgets::BudgetsService<'_> {
        budgets::BudgetsService::new(self)
    }
    /// Get cloudtrail service handler
    pub fn cloudtrail(&self) -> cloudtrail::CloudtrailService<'_> {
        cloudtrail::CloudtrailService::new(self)
    }
    /// Get geo service handler
    pub fn geo(&self) -> geo::GeoService<'_> {
        geo::GeoService::new(self)
    }
    /// Get m2 service handler
    pub fn m2(&self) -> m2::M2Service<'_> {
        m2::M2Service::new(self)
    }
    /// Get pinpoint service handler
    pub fn pinpoint(&self) -> pinpoint::PinpointService<'_> {
        pinpoint::PinpointService::new(self)
    }
    /// Get lex service handler
    pub fn lex(&self) -> lex::LexService<'_> {
        lex::LexService::new(self)
    }
    /// Get finspace service handler
    pub fn finspace(&self) -> finspace::FinspaceService<'_> {
        finspace::FinspaceService::new(self)
    }
    /// Get detective service handler
    pub fn detective(&self) -> detective::DetectiveService<'_> {
        detective::DetectiveService::new(self)
    }
    /// Get lambda service handler
    pub fn lambda(&self) -> lambda::LambdaService<'_> {
        lambda::LambdaService::new(self)
    }
    /// Get kinesis service handler
    pub fn kinesis(&self) -> kinesis::KinesisService<'_> {
        kinesis::KinesisService::new(self)
    }
    /// Get panorama service handler
    pub fn panorama(&self) -> panorama::PanoramaService<'_> {
        panorama::PanoramaService::new(self)
    }
    /// Get iot service handler
    pub fn iot(&self) -> iot::IotService<'_> {
        iot::IotService::new(self)
    }
    /// Get app service handler
    pub fn app(&self) -> app::AppService<'_> {
        app::AppService::new(self)
    }
    /// Get managedblockchain service handler
    pub fn managedblockchain(&self) -> managedblockchain::ManagedblockchainService<'_> {
        managedblockchain::ManagedblockchainService::new(self)
    }
    /// Get waf service handler
    pub fn waf(&self) -> waf::WafService<'_> {
        waf::WafService::new(self)
    }
    /// Get ivs service handler
    pub fn ivs(&self) -> ivs::IvsService<'_> {
        ivs::IvsService::new(self)
    }
    /// Get devops service handler
    pub fn devops(&self) -> devops::DevopsService<'_> {
        devops::DevopsService::new(self)
    }
    /// Get cost service handler
    pub fn cost(&self) -> cost::CostService<'_> {
        cost::CostService::new(self)
    }
    /// Get mq service handler
    pub fn mq(&self) -> mq::MqService<'_> {
        mq::MqService::new(self)
    }
    /// Get route53 service handler
    pub fn route53(&self) -> route53::Route53Service<'_> {
        route53::Route53Service::new(self)
    }
    /// Get internetmonitor service handler
    pub fn internetmonitor(&self) -> internetmonitor::InternetmonitorService<'_> {
        internetmonitor::InternetmonitorService::new(self)
    }
    /// Get license service handler
    pub fn license(&self) -> license::LicenseService<'_> {
        license::LicenseService::new(self)
    }
    /// Get codestar service handler
    pub fn codestar(&self) -> codestar::CodestarService<'_> {
        codestar::CodestarService::new(self)
    }
    /// Get artifact service handler
    pub fn artifact(&self) -> artifact::ArtifactService<'_> {
        artifact::ArtifactService::new(self)
    }
    /// Get ssm service handler
    pub fn ssm(&self) -> ssm::SsmService<'_> {
        ssm::SsmService::new(self)
    }
    /// Get iotsitewise service handler
    pub fn iotsitewise(&self) -> iotsitewise::IotsitewiseService<'_> {
        iotsitewise::IotsitewiseService::new(self)
    }
    /// Get serverlessapplicationrepository service handler
    pub fn serverlessapplicationrepository(&self) -> serverlessapplicationrepository::ServerlessapplicationrepositoryService<'_> {
        serverlessapplicationrepository::ServerlessapplicationrepositoryService::new(self)
    }
    /// Get ssm service handler
    pub fn ssm(&self) -> ssm::SsmService<'_> {
        ssm::SsmService::new(self)
    }
    /// Get docdb service handler
    pub fn docdb(&self) -> docdb::DocdbService<'_> {
        docdb::DocdbService::new(self)
    }
    /// Get sagemaker service handler
    pub fn sagemaker(&self) -> sagemaker::SagemakerService<'_> {
        sagemaker::SagemakerService::new(self)
    }
    /// Get license service handler
    pub fn license(&self) -> license::LicenseService<'_> {
        license::LicenseService::new(self)
    }
    /// Get sagemaker service handler
    pub fn sagemaker(&self) -> sagemaker::SagemakerService<'_> {
        sagemaker::SagemakerService::new(self)
    }
    /// Get personalize service handler
    pub fn personalize(&self) -> personalize::PersonalizeService<'_> {
        personalize::PersonalizeService::new(self)
    }
    /// Get clouddirectory service handler
    pub fn clouddirectory(&self) -> clouddirectory::ClouddirectoryService<'_> {
        clouddirectory::ClouddirectoryService::new(self)
    }
    /// Get chime service handler
    pub fn chime(&self) -> chime::ChimeService<'_> {
        chime::ChimeService::new(self)
    }
    /// Get notificationscontacts service handler
    pub fn notificationscontacts(&self) -> notificationscontacts::NotificationscontactsService<'_> {
        notificationscontacts::NotificationscontactsService::new(self)
    }
    /// Get backupsearch service handler
    pub fn backupsearch(&self) -> backupsearch::BackupsearchService<'_> {
        backupsearch::BackupsearchService::new(self)
    }
    /// Get dynamodb service handler
    pub fn dynamodb(&self) -> dynamodb::DynamodbService<'_> {
        dynamodb::DynamodbService::new(self)
    }
    /// Get iot service handler
    pub fn iot(&self) -> iot::IotService<'_> {
        iot::IotService::new(self)
    }
    /// Get cognito service handler
    pub fn cognito(&self) -> cognito::CognitoService<'_> {
        cognito::CognitoService::new(self)
    }
    /// Get codeartifact service handler
    pub fn codeartifact(&self) -> codeartifact::CodeartifactService<'_> {
        codeartifact::CodeartifactService::new(self)
    }
    /// Get arc service handler
    pub fn arc(&self) -> arc::ArcService<'_> {
        arc::ArcService::new(self)
    }
    /// Get elastic service handler
    pub fn elastic(&self) -> elastic::ElasticService<'_> {
        elastic::ElasticService::new(self)
    }
    /// Get guardduty service handler
    pub fn guardduty(&self) -> guardduty::GuarddutyService<'_> {
        guardduty::GuarddutyService::new(self)
    }
    /// Get cleanrooms service handler
    pub fn cleanrooms(&self) -> cleanrooms::CleanroomsService<'_> {
        cleanrooms::CleanroomsService::new(self)
    }
    /// Get trustedadvisor service handler
    pub fn trustedadvisor(&self) -> trustedadvisor::TrustedadvisorService<'_> {
        trustedadvisor::TrustedadvisorService::new(self)
    }
    /// Get sagemaker service handler
    pub fn sagemaker(&self) -> sagemaker::SagemakerService<'_> {
        sagemaker::SagemakerService::new(self)
    }
    /// Get dax service handler
    pub fn dax(&self) -> dax::DaxService<'_> {
        dax::DaxService::new(self)
    }
    /// Get docdb service handler
    pub fn docdb(&self) -> docdb::DocdbService<'_> {
        docdb::DocdbService::new(self)
    }
    /// Get firehose service handler
    pub fn firehose(&self) -> firehose::FirehoseService<'_> {
        firehose::FirehoseService::new(self)
    }
    /// Get ivschat service handler
    pub fn ivschat(&self) -> ivschat::IvschatService<'_> {
        ivschat::IvschatService::new(self)
    }
    /// Get ses service handler
    pub fn ses(&self) -> ses::SesService<'_> {
        ses::SesService::new(self)
    }
    /// Get bcm service handler
    pub fn bcm(&self) -> bcm::BcmService<'_> {
        bcm::BcmService::new(self)
    }
    /// Get application service handler
    pub fn application(&self) -> application::ApplicationService<'_> {
        application::ApplicationService::new(self)
    }
    /// Get device service handler
    pub fn device(&self) -> device::DeviceService<'_> {
        device::DeviceService::new(self)
    }
    /// Get account service handler
    pub fn account(&self) -> account::AccountService<'_> {
        account::AccountService::new(self)
    }
    /// Get launch service handler
    pub fn launch(&self) -> launch::LaunchService<'_> {
        launch::LaunchService::new(self)
    }
    /// Get finspace service handler
    pub fn finspace(&self) -> finspace::FinspaceService<'_> {
        finspace::FinspaceService::new(self)
    }
    /// Get appconfigdata service handler
    pub fn appconfigdata(&self) -> appconfigdata::AppconfigdataService<'_> {
        appconfigdata::AppconfigdataService::new(self)
    }
    /// Get controlcatalog service handler
    pub fn controlcatalog(&self) -> controlcatalog::ControlcatalogService<'_> {
        controlcatalog::ControlcatalogService::new(self)
    }
    /// Get greengrass service handler
    pub fn greengrass(&self) -> greengrass::GreengrassService<'_> {
        greengrass::GreengrassService::new(self)
    }
    /// Get kendra service handler
    pub fn kendra(&self) -> kendra::KendraService<'_> {
        kendra::KendraService::new(self)
    }
    /// Get snow service handler
    pub fn snow(&self) -> snow::SnowService<'_> {
        snow::SnowService::new(self)
    }
    /// Get securityhub service handler
    pub fn securityhub(&self) -> securityhub::SecurityhubService<'_> {
        securityhub::SecurityhubService::new(self)
    }
    /// Get s3vectors service handler
    pub fn s3vectors(&self) -> s3vectors::S3vectorsService<'_> {
        s3vectors::S3vectorsService::new(self)
    }
    /// Get workspaces service handler
    pub fn workspaces(&self) -> workspaces::WorkspacesService<'_> {
        workspaces::WorkspacesService::new(self)
    }
    /// Get backup service handler
    pub fn backup(&self) -> backup::BackupService<'_> {
        backup::BackupService::new(self)
    }
    /// Get opensearchserverless service handler
    pub fn opensearchserverless(&self) -> opensearchserverless::OpensearchserverlessService<'_> {
        opensearchserverless::OpensearchserverlessService::new(self)
    }
    /// Get cloudformation service handler
    pub fn cloudformation(&self) -> cloudformation::CloudformationService<'_> {
        cloudformation::CloudformationService::new(self)
    }
    /// Get kendra service handler
    pub fn kendra(&self) -> kendra::KendraService<'_> {
        kendra::KendraService::new(self)
    }
    /// Get connect service handler
    pub fn connect(&self) -> connect::ConnectService<'_> {
        connect::ConnectService::new(self)
    }
    /// Get machine service handler
    pub fn machine(&self) -> machine::MachineService<'_> {
        machine::MachineService::new(self)
    }
    /// Get elasticache service handler
    pub fn elasticache(&self) -> elasticache::ElasticacheService<'_> {
        elasticache::ElasticacheService::new(self)
    }
    /// Get sfn service handler
    pub fn sfn(&self) -> sfn::SfnService<'_> {
        sfn::SfnService::new(self)
    }
    /// Get sso service handler
    pub fn sso(&self) -> sso::SsoService<'_> {
        sso::SsoService::new(self)
    }
    /// Get auto service handler
    pub fn auto(&self) -> auto::AutoService<'_> {
        auto::AutoService::new(self)
    }
    /// Get comprehend service handler
    pub fn comprehend(&self) -> comprehend::ComprehendService<'_> {
        comprehend::ComprehendService::new(self)
    }
    /// Get rds service handler
    pub fn rds(&self) -> rds::RdsService<'_> {
        rds::RdsService::new(self)
    }
    /// Get chime service handler
    pub fn chime(&self) -> chime::ChimeService<'_> {
        chime::ChimeService::new(self)
    }
    /// Get rekognition service handler
    pub fn rekognition(&self) -> rekognition::RekognitionService<'_> {
        rekognition::RekognitionService::new(self)
    }
    /// Get appstream service handler
    pub fn appstream(&self) -> appstream::AppstreamService<'_> {
        appstream::AppstreamService::new(self)
    }
    /// Get polly service handler
    pub fn polly(&self) -> polly::PollyService<'_> {
        polly::PollyService::new(self)
    }
    /// Get invoicing service handler
    pub fn invoicing(&self) -> invoicing::InvoicingService<'_> {
        invoicing::InvoicingService::new(self)
    }
    /// Get rds service handler
    pub fn rds(&self) -> rds::RdsService<'_> {
        rds::RdsService::new(self)
    }
    /// Get pricing service handler
    pub fn pricing(&self) -> pricing::PricingService<'_> {
        pricing::PricingService::new(self)
    }
    /// Get swf service handler
    pub fn swf(&self) -> swf::SwfService<'_> {
        swf::SwfService::new(self)
    }
    /// Get cloudfront service handler
    pub fn cloudfront(&self) -> cloudfront::CloudfrontService<'_> {
        cloudfront::CloudfrontService::new(self)
    }
    /// Get marketplace service handler
    pub fn marketplace(&self) -> marketplace::MarketplaceService<'_> {
        marketplace::MarketplaceService::new(self)
    }
    /// Get medical service handler
    pub fn medical(&self) -> medical::MedicalService<'_> {
        medical::MedicalService::new(self)
    }
    /// Get transcribe service handler
    pub fn transcribe(&self) -> transcribe::TranscribeService<'_> {
        transcribe::TranscribeService::new(self)
    }
    /// Get observabilityadmin service handler
    pub fn observabilityadmin(&self) -> observabilityadmin::ObservabilityadminService<'_> {
        observabilityadmin::ObservabilityadminService::new(self)
    }
    /// Get notifications service handler
    pub fn notifications(&self) -> notifications::NotificationsService<'_> {
        notifications::NotificationsService::new(self)
    }
    /// Get codecommit service handler
    pub fn codecommit(&self) -> codecommit::CodecommitService<'_> {
        codecommit::CodecommitService::new(self)
    }
    /// Get lex service handler
    pub fn lex(&self) -> lex::LexService<'_> {
        lex::LexService::new(self)
    }
    /// Get pca service handler
    pub fn pca(&self) -> pca::PcaService<'_> {
        pca::PcaService::new(self)
    }
    /// Get forecastquery service handler
    pub fn forecastquery(&self) -> forecastquery::ForecastqueryService<'_> {
        forecastquery::ForecastqueryService::new(self)
    }
    /// Get healthlake service handler
    pub fn healthlake(&self) -> healthlake::HealthlakeService<'_> {
        healthlake::HealthlakeService::new(self)
    }
    /// Get rolesanywhere service handler
    pub fn rolesanywhere(&self) -> rolesanywhere::RolesanywhereService<'_> {
        rolesanywhere::RolesanywhereService::new(self)
    }
    /// Get marketplace service handler
    pub fn marketplace(&self) -> marketplace::MarketplaceService<'_> {
        marketplace::MarketplaceService::new(self)
    }
    /// Get billing service handler
    pub fn billing(&self) -> billing::BillingService<'_> {
        billing::BillingService::new(self)
    }
    /// Get emr service handler
    pub fn emr(&self) -> emr::EmrService<'_> {
        emr::EmrService::new(self)
    }
    /// Get apigatewaymanagementapi service handler
    pub fn apigatewaymanagementapi(&self) -> apigatewaymanagementapi::ApigatewaymanagementapiService<'_> {
        apigatewaymanagementapi::ApigatewaymanagementapiService::new(self)
    }
    /// Get xray service handler
    pub fn xray(&self) -> xray::XrayService<'_> {
        xray::XrayService::new(self)
    }
    /// Get transcribe service handler
    pub fn transcribe(&self) -> transcribe::TranscribeService<'_> {
        transcribe::TranscribeService::new(self)
    }
    /// Get ram service handler
    pub fn ram(&self) -> ram::RamService<'_> {
        ram::RamService::new(self)
    }
    /// Get codeconnections service handler
    pub fn codeconnections(&self) -> codeconnections::CodeconnectionsService<'_> {
        codeconnections::CodeconnectionsService::new(self)
    }
    /// Get efs service handler
    pub fn efs(&self) -> efs::EfsService<'_> {
        efs::EfsService::new(self)
    }
    /// Get migration service handler
    pub fn migration(&self) -> migration::MigrationService<'_> {
        migration::MigrationService::new(self)
    }
    /// Get elasticsearch service handler
    pub fn elasticsearch(&self) -> elasticsearch::ElasticsearchService<'_> {
        elasticsearch::ElasticsearchService::new(self)
    }
    /// Get cognito service handler
    pub fn cognito(&self) -> cognito::CognitoService<'_> {
        cognito::CognitoService::new(self)
    }
    /// Get payment service handler
    pub fn payment(&self) -> payment::PaymentService<'_> {
        payment::PaymentService::new(self)
    }
    /// Get dataexchange service handler
    pub fn dataexchange(&self) -> dataexchange::DataexchangeService<'_> {
        dataexchange::DataexchangeService::new(self)
    }
    /// Get sts service handler
    pub fn sts(&self) -> sts::StsService<'_> {
        sts::StsService::new(self)
    }
    /// Get sagemaker service handler
    pub fn sagemaker(&self) -> sagemaker::SagemakerService<'_> {
        sagemaker::SagemakerService::new(self)
    }
    /// Get kinesis service handler
    pub fn kinesis(&self) -> kinesis::KinesisService<'_> {
        kinesis::KinesisService::new(self)
    }
    /// Get acm service handler
    pub fn acm(&self) -> acm::AcmService<'_> {
        acm::AcmService::new(self)
    }
    /// Get athena service handler
    pub fn athena(&self) -> athena::AthenaService<'_> {
        athena::AthenaService::new(self)
    }
    /// Get dsql service handler
    pub fn dsql(&self) -> dsql::DsqlService<'_> {
        dsql::DsqlService::new(self)
    }
    /// Get mediapackage service handler
    pub fn mediapackage(&self) -> mediapackage::MediapackageService<'_> {
        mediapackage::MediapackageService::new(self)
    }
    /// Get tnb service handler
    pub fn tnb(&self) -> tnb::TnbService<'_> {
        tnb::TnbService::new(self)
    }
    /// Get ec2 service handler
    pub fn ec2(&self) -> ec2::Ec2Service<'_> {
        ec2::Ec2Service::new(self)
    }
    /// Get apprunner service handler
    pub fn apprunner(&self) -> apprunner::ApprunnerService<'_> {
        apprunner::ApprunnerService::new(self)
    }
    /// Get redshift service handler
    pub fn redshift(&self) -> redshift::RedshiftService<'_> {
        redshift::RedshiftService::new(self)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_provider_creation() {
        // Provider creation test
        // Note: This will fail without proper credentials
    }
}
