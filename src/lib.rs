mod converter;
mod layer;
mod visitor;

pub use layer::SentryBreadcrumbLayer;

pub fn layer() -> SentryBreadcrumbLayer {
    SentryBreadcrumbLayer::new()
}
