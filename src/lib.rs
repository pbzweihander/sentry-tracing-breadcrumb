mod imp;

use tracing_core::span::{Attributes, Id, Record};
use tracing_core::{Event, Subscriber};
use tracing_subscriber::layer::{Context, Layer};
use tracing_subscriber::registry::LookupSpan;

pub fn layer<S>() -> SentryBreadcrumbLayer<S> {
    SentryBreadcrumbLayer::new()
}

#[derive(Debug)]
pub struct SentryBreadcrumbLayer<S> {
    inner_layer: imp::InnerLayer<S>,
}

impl<S> SentryBreadcrumbLayer<S> {
    pub fn new() -> Self {
        let inner_layer = imp::inner_layer();
        Self { inner_layer }
    }
}

impl<S> Default for SentryBreadcrumbLayer<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Layer<S> for SentryBreadcrumbLayer<S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        self.inner_layer.new_span(attrs, id, ctx)
    }

    fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        self.inner_layer.on_record(id, values, ctx)
    }

    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
        self.inner_layer.on_enter(id, ctx)
    }

    fn on_exit(&self, id: &Id, ctx: Context<'_, S>) {
        self.inner_layer.on_exit(id, ctx)
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        self.inner_layer.on_close(id, ctx)
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        self.inner_layer.on_event(event, ctx)
    }
}
