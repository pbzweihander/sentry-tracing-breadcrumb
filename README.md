# sentry-tracing-breadcrumb

A [tracing layer](https://docs.rs/tracing-subscriber/0.1.28/tracing_subscriber/layer/trait.Layer.html) that
adds all [tracing events](https://docs.rs/tracing/0.1.26/tracing/struct.Event.html) to
[sentry breadcrumb](https://docs.rs/sentry/0.22.0/sentry/struct.Breadcrumb.html)
with all fields and spans.

## Usage

```rust
tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(sentry_tracing_breadcrumb::layer())
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        .init();
```
