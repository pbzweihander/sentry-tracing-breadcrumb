use std::borrow::Borrow;

use sentry_core::protocol::{Map, Value};
use sentry_core::types::Utc;
use sentry_core::Breadcrumb;
use tracing_core::{Event, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::{LookupSpan, SpanRef};

use crate::visitor::{get_fields_from_span, record_new};

fn level_to_level(level: &tracing_core::Level) -> sentry_core::Level {
    match level {
        &tracing_core::Level::TRACE | &tracing_core::Level::DEBUG => sentry_core::Level::Debug,
        &tracing_core::Level::INFO => sentry_core::Level::Info,
        &tracing_core::Level::WARN => sentry_core::Level::Warning,
        &tracing_core::Level::ERROR => sentry_core::Level::Error,
    }
}

fn span_to_data<'a, S>(span: impl Borrow<SpanRef<'a, S>>) -> Value
where
    S: LookupSpan<'a> + 'a,
{
    let span = span.borrow();
    let mut fields = get_fields_from_span(span);
    fields.insert("name".to_string(), span.name().to_string().into());
    fields.into()
}

pub fn event_to_breadcrumb<S>(event: &Event<'_>, ctx: Context<'_, S>) -> Breadcrumb
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let metadata = event.metadata();
    let fields = record_new(event);

    let message = fields
        .get("message")
        .and_then(|v| v.as_str().map(str::to_string));

    let mut data = Map::new();
    data.insert("fields".to_string(), fields.into());

    if let Some(current_span) = ctx.lookup_current() {
        data.insert("span".to_string(), span_to_data(&current_span));

        let spans: Vec<_> = current_span.scope().from_root().map(span_to_data).collect();
        data.insert("spans".to_string(), spans.into());
    }

    Breadcrumb {
        timestamp: Utc::now(),
        ty: "log".to_string(),
        category: Some(metadata.target().to_string()),
        level: level_to_level(metadata.level()),
        message,
        data,
    }
}
