use crate::ErrorAnnotation;

/// Capture diagnostic information in a closure that extends a source error type into a wrapping
/// [`ErrorAnnotation`].
///
/// `annotate` captures a `label` and a diagnostic `info` construction closure, `mkinfo`, returning
/// a closure that creates an [`ErrorAnnotation`] given a source error of type `S`. This API design
/// makes it ergonomic to use as the parameter to `Result::map_err` to transform a source error
/// by extending it with diagnostic information.
///
/// `mkinfo` is a closure which allows deferring the construction of the diagnostic `info` until
/// the error case is encountered, similar to the design of `Option::ok_or_else`.
///
/// # Example
///
/// ```
/// use std::path::Path;
/// use std::fs::Metadata;
/// use error_annotation::{ErrorAnnotation, annotate};
///
/// type IoErrorWithPath<'a> = ErrorAnnotation<std::io::Error, std::path::Display<'a>>;
///
/// fn metadata(p: &Path) -> Result<Metadata, IoErrorWithPath> {
///   std::fs::metadata(p).map_err(annotate("path", || p.display()))
/// }
/// ```
pub fn annotate<S, F, I>(label: &'static str, mkinfo: F) -> impl FnOnce(S) -> ErrorAnnotation<S, I>
where
    F: FnOnce() -> I,
{
    move |source| ErrorAnnotation {
        source,
        label,
        info: mkinfo(),
    }
}
