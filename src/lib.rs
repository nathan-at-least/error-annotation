//! Add useful diagnostic information to error values as they propagate.
//!
//! # Annotatable Error Types
//!
//! The most ergonomic usage is available for types that implement [`Annotatable`] via the
//! [`AnnotateResult::annotate_err_into`] method, which is implemented for `Result`. In this case,
//! diagnostic annotations can be built up without altering the resulting error types. An example
//! using the `std::io::Error` impl of `Annotatable` clarifies these ergonomics:
//!
//! ## Example: Chaining Diagnostic Annotations on `std::io::Error`
//!
//! ```
//! use std::path::{Path, PathBuf};
//! use std::fs::Metadata;
//! use error_annotation::{AnnotateResult, ErrorAnnotation};
//!
//! fn annotated_copy(src: &Path, dst: &Path) -> std::io::Result<u64> {
//!   std::fs::copy(src, dst)
//!     .annotate_err_into("source", || src.display())
//!     .annotate_err_into("destination", || dst.display())
//! }
//!
//! let badsource = PathBuf::from("/this/path/does/not/exist");
//! let dest = PathBuf::from("/tmp/woah-dude");
//! let res = annotated_copy(&badsource, &dest);
//! let err = res.err().unwrap();
//!
//! assert_eq!(&err.to_string(), "
//!
//! No such file or directory (os error 2)
//! -with source: /this/path/does/not/exist
//! -with destination: /tmp/woah-dude
//!
//! ".trim());
//! ```
//!
//! # Annotating Other Error Types
//!
//! It is still possible to annotate an arbitrary error type `T` which does not implement [`Annotatable`]
//! with annotation info type `I` by way of the [`ErrorAnnotation`]
//! parameterized type. The downside being each annotation corresponds to a different
//! parameterization, ie `ErrorAnnotation<T, I>`, which propagates out of interfaces and must be
//! explicitly handled by consuming code.

mod annotatable;
mod annotate;
mod annres;
mod ea;

pub use self::annotatable::Annotatable;
pub use self::annotate::annotate;
pub use self::annres::AnnotateResult;
pub use self::ea::ErrorAnnotation;

#[cfg(test)]
mod tests;
