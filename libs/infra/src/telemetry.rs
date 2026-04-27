use serde::{Deserialize, Serialize};
use std::sync::Once;
use tracing::{Subscriber, dispatcher::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{
    EnvFilter, Registry,
    fmt::{MakeWriter, layer},
    layer::SubscriberExt,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    #[default]
    Pretty,
    Json,
}

static TRACING: Once = Once::new();

pub fn get_subscriber<Sink>(
    name: impl Into<String>,
    filter: &str,
    sink: Sink,
    format: LogFormat,
) -> Box<dyn Subscriber + Send + Sync + 'static>
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter));

    match format {
        LogFormat::Json => Box::new(
            Registry::default()
                .with(env_filter)
                .with(JsonStorageLayer)
                .with(BunyanFormattingLayer::new(name.into(), sink)),
        ),
        LogFormat::Pretty => {
            let layer = layer().pretty().compact();
            Box::new(Registry::default().with(env_filter).with(layer))
        }
    }
}

pub fn init_subscriber(subscriber: Box<dyn Subscriber + Send + Sync + 'static>) {
    TRACING.call_once(|| {
        LogTracer::init().expect("Failed to set logger");
        set_global_default(subscriber.into()).expect("Failed to set subscriber");
    });
}
