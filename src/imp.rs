use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::io::{self, LineWriter, Write};

use sentry_core::types::{DateTime, Utc};
use sentry_core::Breadcrumb;
use serde::Deserialize;
use tracing_subscriber::fmt::format::{Format, Json, JsonFields};
use tracing_subscriber::fmt::time::ChronoUtc;
use tracing_subscriber::fmt::{Layer, MakeWriter};

pub type InnerLayer<S> = Layer<S, JsonFields, Format<Json, ChronoUtc>, SentryBreadcrumbWriter>;

pub fn inner_layer<S>() -> InnerLayer<S> {
    tracing_subscriber::fmt::layer()
        .json()
        .with_timer(ChronoUtc::rfc3339())
        .with_writer(SentryBreadcrumbWriter)
}

#[derive(Debug, Deserialize)]
struct TracingJsonFields {
    message: String,
    #[serde(flatten, default)]
    other: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct TracingJson {
    timestamp: DateTime<Utc>,
    level: String,
    target: String,
    fields: TracingJsonFields,
    #[serde(flatten, default)]
    other: serde_json::Map<String, serde_json::Value>,
}

impl TryFrom<TracingJson> for Breadcrumb {
    type Error = ();

    fn try_from(log: TracingJson) -> Result<Self, Self::Error> {
        let mut data: BTreeMap<_, _> = log.other.into_iter().collect();
        data.insert("fields".to_string(), log.fields.other.into());
        Ok(Breadcrumb {
            timestamp: log.timestamp,
            ty: "log".to_string(),
            category: Some(log.target),
            level: log.level.to_lowercase().parse().map_err(|_| ())?,
            message: Some(log.fields.message),
            data,
        })
    }
}

#[derive(Debug)]
pub struct SentryBreadcrumbWriterInner;

impl Write for SentryBreadcrumbWriterInner {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Ok(breadcrumb) = serde_json::from_slice::<TracingJson>(buf)
            .map_err(|_| ())
            .and_then(Breadcrumb::try_from)
        {
            sentry_core::add_breadcrumb(breadcrumb);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct SentryBreadcrumbWriter;

impl MakeWriter for SentryBreadcrumbWriter {
    type Writer = LineWriter<SentryBreadcrumbWriterInner>;

    fn make_writer(&self) -> Self::Writer {
        LineWriter::new(SentryBreadcrumbWriterInner)
    }
}
