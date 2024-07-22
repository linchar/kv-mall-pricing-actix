// Set up OpenTelemetry tracer

use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::{TonicExporterBuilder, WithExportConfig};
use opentelemetry_sdk::{
    propagation::TraceContextPropagator, runtime::TokioCurrentThread, trace, Resource,
};
use std::env;

pub fn init_tracer() {
    // Start a new OTLP trace pipeline
    global::set_text_map_propagator(TraceContextPropagator::new());

    let service_name_resource = Resource::new(vec![KeyValue::new(
        opentelemetry_semantic_conventions::resource::SERVICE_NAME,
        "pricing",
    )]);

    let _tracer: trace::Tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(TonicExporterBuilder::default().with_endpoint(format!(
            "{}",
            env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:4317".to_string())
        )))
        .with_trace_config(trace::Config::default().with_resource(service_name_resource))
        .install_batch(TokioCurrentThread)
        .expect("pipeline install error");
}
