//! Add useful diagnostic information to error values as they propagate.
//!
//! The recommended way to annotate errors directly from a `Result` value is with the
//! [`AnnotateResult::annotate_err`] method which is implemented on `Result`.

mod annotate;
mod annres;
mod ea;

pub use self::annotate::annotate;
pub use self::annres::AnnotateResult;
pub use self::ea::ErrorAnnotation;

#[cfg(test)]
mod tests;
