#![allow(dead_code)]

mod apis;
mod emvo_qrcode;
mod emvo_types;
mod prompt_pay;

use crate::apis::*;
use crate::emvo_qrcode::*;
use crate::emvo_types::*;
use crate::prompt_pay::*;

use actix_files as fs;
use actix_files::NamedFile;
use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpServer};

use actix_web_opentelemetry::{RequestTracing,RequestMetricsBuilder};


use log::info;
use std::env;
use std::time::Duration;

use rand::{thread_rng, Rng};

//use opentelemetry::sdk::metrics::{controllers, processors, selectors};

use opentelemetry::sdk::export::metrics::aggregation::stateless_temporality_selector;
use opentelemetry::{global, Context, sdk::{
    export::metrics::aggregation,
    metrics::{controllers, processors, selectors},
    propagation::TraceContextPropagator,
}, KeyValue};
use opentelemetry::metrics::Unit;


pub async fn index() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("./static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "actix_web=info");

    let tag_namespace = env::var("TAG_NAME");
    let tag_namespace = tag_namespace.unwrap_or("QR_CODE_TAG30_SERVICE".to_string());
    env_logger::init();

    let instrumentation_key = std::env::var("INSTRUMENTATION_KEY")
        .unwrap_or("cf9fa72a-f172-4184-b55d-97bd77e6d4a7".to_string());



        let instrumentation_endpoint = std::env::var("INSTRUMENTATION_ENDPOINT")
            .unwrap_or("https://southeastasia-1.in.applicationinsights.azure.com".to_string());

        let _tracer = opentelemetry_application_insights::new_pipeline(instrumentation_key.clone())
            .with_client(reqwest::Client::new())
            .with_endpoint(instrumentation_endpoint.as_str())
            .unwrap()
            .install_batch(opentelemetry::runtime::Tokio);


        let temporality_selector = stateless_temporality_selector();
        let exporter = opentelemetry_application_insights::Exporter::new(instrumentation_key, ())
            .with_temporality_selector(temporality_selector.clone());
        let controller = controllers::basic(processors::factory(
            selectors::simple::inexpensive(),
            temporality_selector,
        ))
            .with_exporter(exporter)
            .with_collect_period(Duration::from_secs(1))
            .build();


        let cx = Context::new();
        controller.start(&cx, opentelemetry::runtime::Tokio).expect("Metrics controller start error");
        global::set_meter_provider(controller.clone());

    let meter = global::meter("custom.instrumentation");
    let cpu_utilization_gauge = meter
        .f64_observable_gauge("system.cpu.utilization")
        .with_unit(Unit::new("1"))
        .init();

    meter.register_callback(move |cx| {
        let mut rng = thread_rng();
        cpu_utilization_gauge.observe(
            cx,
            rng.gen_range(0.1..0.2),
            &[KeyValue::new("state", "idle"), KeyValue::new("cpu", 0)],
        )
    }).expect("Can't register call back metrics");

    let request_metrics = RequestMetricsBuilder::new().build(meter);


    HttpServer::new(move || {
        App::new()
            // enable logger
            //.wrap(middleware::Logger::default())
            .wrap(Logger::default().log_target(tag_namespace.clone()))
            .wrap(RequestTracing::new())
            .wrap(request_metrics.clone())
            .service(qr_code_tag30)
            //.service(fs::Files::new("/static", ".").prefer_utf8(true).index_file("index.html"))
            .route("/", web::get().to(index))
    })
        .workers(10)
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    controller.stop(&cx).expect("Metrics controller stop failed");

    global::shutdown_tracer_provider();


    Ok(())
}
