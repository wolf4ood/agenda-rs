use std::collections::HashMap;

use opentelemetry::sdk::propagation::TraceContextPropagator;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use tracing_subscriber::prelude::*;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

pub fn tracing_headers() -> HeaderMap {
    use opentelemetry::propagation::text_map_propagator::TextMapPropagator;
    use tracing_opentelemetry::OpenTelemetrySpanExt;

    let context = tracing::Span::current().context();
    let mut tracing_headers: HashMap<String, String> = HashMap::new();
    let extractor = opentelemetry::sdk::propagation::TraceContextPropagator::new();
    extractor.inject_context(&context, &mut tracing_headers);

    tracing_headers
        .iter()
        .flat_map(|(name, value)| {
            let header_name = HeaderName::from_bytes(name.as_bytes());
            let header_value = HeaderValue::from_bytes(value.as_bytes());
            match (header_name, header_value) {
                (Ok(valid_header_name), Ok(valid_header_value)) => {
                    vec![(valid_header_name, valid_header_value)]
                }
                _ => vec![],
            }
        })
        .collect()
}

pub fn install_telemetry(service: &str) -> Result<(), Box<dyn std::error::Error>> {
    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(service)
        .install_simple()?;
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .with(opentelemetry)
        .try_init()?;
    Ok(())
}
