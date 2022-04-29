use std::borrow::Borrow;
use std::fmt::Debug;

use sentry_core::protocol::value::{Map, Value};
use tracing_core::field::{Field, Visit};
use tracing_subscriber::field::RecordFields;
use tracing_subscriber::registry::{LookupSpan, SpanRef};

pub fn record_span<S>(recordable: impl RecordFields, span: &SpanRef<'_, S>)
where
    S: for<'a> LookupSpan<'a>,
{
    let mut extensions = span.extensions_mut();
    if let Some(JsonValuesRecorder { ref mut fields }) = extensions.get_mut() {
        record_to(recordable, fields);
    } else {
        let fields = record_new(recordable);
        extensions.insert(JsonValuesRecorder { fields });
    }
}

pub fn get_fields_from_span<'a, S>(span: impl Borrow<SpanRef<'a, S>>) -> Map<String, Value>
where
    S: LookupSpan<'a> + 'a,
{
    span.borrow()
        .extensions()
        .get::<JsonValuesRecorder>()
        .map(|ext| ext.fields.clone())
        .unwrap_or_default()
}

pub fn record_new(recordable: impl RecordFields) -> Map<String, Value> {
    let mut fields = Map::new();
    record_to(recordable, &mut fields);
    fields
}

pub fn record_to(recordable: impl RecordFields, fields: &mut Map<String, Value>) {
    let mut visitor = JsonValueVisitor { fields };
    recordable.record(&mut visitor);
}

struct JsonValuesRecorder {
    fields: Map<String, Value>,
}

struct JsonValueVisitor<'a> {
    fields: &'a mut Map<String, Value>,
}

impl<'a> JsonValueVisitor<'a> {
    fn record(&mut self, field: &Field, value: impl Into<Value>) {
        self.fields.insert(field.name().to_string(), value.into());
    }
}

impl<'a> Visit for JsonValueVisitor<'a> {
    fn record_i64(&mut self, field: &Field, value: i64) {
        self.record(field, value);
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.record(field, value);
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.record(field, value);
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.record(field, value);
    }

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        self.record(field, format!("{:?}", value));
    }
}
