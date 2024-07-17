use actix_web_opentelemetry::ClientExt;
use awc::Client;
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::{TonicExporterBuilder, WithExportConfig};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::runtime::TokioCurrentThread;
use opentelemetry_sdk::{trace, Resource};
use std::error::Error;
use std::io;

async fn execute_request(client: Client) -> io::Result<String> {
    let mut response = client
        .get("http://127.0.0.1:8080/price?id=1")
        .trace_request()
        .send()
        .await
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;

    let bytes = response
        .body()
        .await
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;

    std::str::from_utf8(&bytes)
        .map(|s| s.to_owned())
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Start a new OTLP trace pipeline
    global::set_text_map_propagator(TraceContextPropagator::new());

    let service_name_resource = Resource::new(vec![KeyValue::new("service.name", "actix_client")]);

    let _tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(TonicExporterBuilder::default().with_endpoint("http://localhost:4317"))
        .with_trace_config(trace::Config::default().with_resource(service_name_resource))
        .install_batch(TokioCurrentThread)
        .expect("pipeline install error");

    let client = awc::Client::new();
    let response = execute_request(client).await?;

    println!("Response: {}", response);

    global::shutdown_tracer_provider();

    Ok(())
}
