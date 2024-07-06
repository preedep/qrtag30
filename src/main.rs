#![allow(dead_code)]

use actix_files::NamedFile;
use actix_web::{web, App, Error, HttpServer};
use actix_web_opentelemetry::RequestTracing;
use log::Level;
use opentelemetry::{global, KeyValue};
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_sdk::logs::{BatchLogProcessor, LoggerProvider};
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions as semcov;

mod apis;
mod emvo_qrcode;
mod emvo_types;
mod prompt_pay;

//use opentelemetry::sdk::metrics::{controllers, processors, selectors};

pub async fn index() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("./static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = reqwest::Client::new();

    let _trace = opentelemetry_application_insights::new_pipeline_from_env()
        .expect("env var APPLICATIONINSIGHTS_CONNECTION_STRING is valid connection string")
        .with_client(client.clone())
        .with_live_metrics(true)
        .install_batch(Tokio);

    let connection_string = std::env::var("APPLICATIONINSIGHTS_CONNECTION_STRING").unwrap();
    let exporter = opentelemetry_application_insights::Exporter::new_from_connection_string(
        connection_string,
        client,
    )
    .expect("connection string is valid");
    let logger_provider = LoggerProvider::builder()
        .with_log_processor(BatchLogProcessor::builder(exporter, Tokio).build())
        .with_config(
            opentelemetry_sdk::logs::config().with_resource(Resource::new(vec![
                KeyValue::new(semcov::resource::SERVICE_NAMESPACE, "PromptPay"),
                KeyValue::new(semcov::resource::SERVICE_NAME, "PromptPay-QR-TAG30-Service"),
            ])),
        )
        .build();

    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    log::set_boxed_logger(Box::new(otel_log_appender)).expect("Could not set logger");
    log::set_max_level(Level::Info.to_level_filter());

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Compress::default())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(RequestTracing::new())
            .service(apis::qr_code_tag30)
            .route("/", web::get().to(index))
    })
    .workers(10)
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    global::shutdown_tracer_provider();
    logger_provider
        .shutdown()
        .expect("Failed to shutdown logger provider");

    Ok(())
}
