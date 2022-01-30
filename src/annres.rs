use crate::{annotate, ErrorAnnotation};

/// A trait to extend `Result` with a convenient `annotate_err` method. This is the recommended
/// interface for annotating errors directly on `Result` values.
///
/// # Example
///
/// ```
/// use std::path::{Path, PathBuf};
/// use std::fs::Metadata;
/// use error_annotation::{AnnotateResult, ErrorAnnotation};
///
/// type IoErrorWithPath<'a> = ErrorAnnotation<std::io::Error, std::path::Display<'a>>;
///
/// fn metadata(p: &Path) -> Result<Metadata, IoErrorWithPath> {
///   std::fs::metadata(p).annotate_err("path", || p.display())
/// }
///
/// let badpath = PathBuf::from("/this/path/does/not/exist");
/// let res = metadata(&badpath);
/// let err = res.err().unwrap();
///
/// assert_eq!(&err.to_string(), "
///
/// No such file or directory (os error 2)
/// -with path: /this/path/does/not/exist
///
/// ".trim());
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
