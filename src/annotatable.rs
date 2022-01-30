use crate::ErrorAnnotation;
use std::fmt::Display;

/// An `Annotatable` error type is can convert an `ErrorAnnotation` with `Self` as the source type
/// back into `Self`, provided the annotated `info` implements `Display`.
///
/// A prime example is the `std::io::Error` impl, which can be used to convert any annotation on a
/// `std::io::Error` back into a `std::io::Error`.
///
/// Errors which implement `Annotatable` allow code to introduce annotations without all of the
/// various diagnostic information resulting in a plethora of uniquely parameterized
/// `ErrorAnnotation` types. This can be convenient, for example, to prevent `ErrorAnnotation`
/// types from leaking out into a crate's interface, so that consumers need only handle common base
/// error types, such as `std::io::Error`.
///
/// The conversion provided by implementations of `Annotatable` is typically conveniently
/// accomplished via `AnnotateResult::annotate_err_into`.
///
/// # Example
///
/// ```
/// use std::path::{Path, PathBuf};
/// use std::fs::Metadata;
/// use error_annotation::{AnnotateResult, ErrorAnnotation};
///
/// fn annotated_copy(src: &Path, dst: &Path) -> std::io::Result<u64> {
///   std::fs::copy(src, dst)
///     .annotate_err_into("source", || src.display())
///     .annotate_err_into("destination", || dst.display())
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
pub trait Annotatable: Sized {
    fn merge_annotation<I>(ea: ErrorAnnotation<Self, I>) -> Self
    where
        I: Display;
}

impl Annotatable for std::io::Error {
    fn merge_annotation<I>(ea: ErrorAnnotation<Self, I>) -> Self
    where
        I: Display,
    {
        Self::new(ea.source.kind(), ea.to_string())
    }
}
