use std::fmt;

/// Combine a source error with labeled diagnostic information.
///
/// `ErrorAnnotation` combines a `source` error of type `S` with diagnostic `info` of type `I`
/// which will be labeled with `label` when displayed.
///
/// # `std::io::Error` ergonomics
///
/// A `ErrorAnnotation<std::io::Error, I>` can be converted into a `std::io::Error` (via `From` /
/// `Into` traits) provided that `I` implements `Display`. The `ErrorKind` is propagated while the
/// string description includes the formatted annotations.
///
/// ## Example
///
/// ```
/// use std::path::{Path, PathBuf};
/// use std::fs::Metadata;
/// use error_annotation::{AnnotateResult, ErrorAnnotation};
///
/// fn annotated_metadata(p: &Path) -> std::io::Result<Metadata> {
///   let m = std::fs::metadata(p).annotate_err("path", || p.display())?;
///   Ok(m)
/// }
///
/// let badpath = PathBuf::from("/this/path/does/not/exist");
/// let res = annotated_metadata(&badpath);
/// let err = res.err().unwrap();
///
/// assert_eq!(&err.to_string(), "
///
/// No such file or directory (os error 2)
/// -with path: /this/path/does/not/exist
///
/// ".trim());
/// ```
pub struct ErrorAnnotation<S, I> {
    pub source: S,
    pub label: &'static str,
    pub info: I,
}

impl<S, I> fmt::Display for ErrorAnnotation<S, I>
where
    I: fmt::Display,
    S: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n-with {}: {}", self.source, self.label, self.info)
    }
}

impl<I> From<ErrorAnnotation<std::io::Error, I>> for std::io::Error
where
    I: fmt::Display,
{
    fn from(ea: ErrorAnnotation<std::io::Error, I>) -> std::io::Error {
        std::io::Error::new(ea.source.kind(), ea.to_string())
    }
}
