use tracing_core::span::{Attributes, Id, Record};
use tracing_core::{Event, Subscriber};
use tracing_subscriber::field::RecordFields;
use tracing_subscriber::layer::{Context, Layer};
use tracing_subscriber::registry::LookupSpan;

use crate::converter::event_to_breadcrumb;
use crate::visitor::record_span;

#[derive(Debug)]
pub struct SentryBreadcrumbLayer;

impl SentryBreadcrumbLayer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SentryBreadcrumbLayer {
    fn default() -> Self {
        Self::new()
    }
}

fn record_fields<S>(recordable: impl RecordFields, id: &Id, ctx: Context<'_, S>)
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let span = ctx.span(id).expect("Span not found, this is a bug");
    record_span(recordable, &span);
}

impl<S> Layer<S> for SentryBreadcrumbLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        record_fields(attrs, id, ctx);
    }

    fn on_record(&self, id: &Id, record: &Record<'_>, ctx: Context<'_, S>) {
        record_fields(record, id, ctx);
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let breadcrumb = event_to_breadcrumb(event, ctx);
        sentry_core::add_breadcrumb(breadcrumb);
    }
}
