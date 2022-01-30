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
///
/// # Multiple Annotations
///
/// Multiple annotations can be tracked by using an `ErrorAnnotation` type as a source, ie:
/// `ErrorAnnotation<ErrorAnnotation<MyError, String>, usize>`.
///
/// Annotating errors with a nested type comes from simply chaining the annotation methods.
///
/// If the outermost `ErrorAnnotation` needs to be converted into a `std::io::Error`, the
/// intermediate `ErrorAnnotation` values must be explicitly converted into `std::io::Error` when
/// chaining in this way.
///
/// ## Example
///
/// ```
/// use std::path::{Path, PathBuf};
/// use std::fs::Metadata;
/// use error_annotation::{AnnotateResult, ErrorAnnotation};
///
/// fn annotated_copy(src: &Path, dst: &Path) -> std::io::Result<u64> {
///   let bytes = std::fs::copy(src, dst)
///     .annotate_err("source", || src.display())
///     .map_err(std::io::Error::from)
///     .annotate_err("destination", || dst.display())?;
///   Ok(bytes)
/// }
///
/// let badsource = PathBuf::from("/this/path/does/not/exist");
/// let dest = PathBuf::from("/tmp/woah-dude");
/// let res = annotated_copy(&badsource, &dest);
/// let err = res.err().unwrap();
///
/// assert_eq!(&err.to_string(), "
///
/// No such file or directory (os error 2)
/// -with source: /this/path/does/not/exist
/// -with destination: /tmp/woah-dude
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
