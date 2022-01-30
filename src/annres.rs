use crate::{annotate, ErrorAnnotation};

/// A trait to extend `Result` with a convenient `annotate_err` method. This is the recommended
/// interface for annotating errors directly on `Result` values.
///
/// # Example
///
/// ```
/// use std::path::Path;
/// use error_annotation::{AnnotateResult, ErrorAnnotation};
///
/// type IoErrorWithPath<'a> = ErrorAnnotation<std::io::Error, std::path::Display<'a>>;
///
/// fn remove_dir_all(p: &Path) -> Result<(), IoErrorWithPath> {
///   std::fs::remove_dir_all(p).annotate_err("path", || p.display())
/// }
/// ```
pub trait AnnotateResult<T, E> {
    fn annotate_err<F, I>(self, label: &'static str, mkinfo: F) -> Result<T, ErrorAnnotation<E, I>>
    where
        F: FnOnce() -> I;
}

impl<T, E> AnnotateResult<T, E> for Result<T, E> {
    fn annotate_err<F, I>(self, label: &'static str, mkinfo: F) -> Result<T, ErrorAnnotation<E, I>>
    where
        F: FnOnce() -> I,
    {
        self.map_err(annotate(label, mkinfo))
    }
}
