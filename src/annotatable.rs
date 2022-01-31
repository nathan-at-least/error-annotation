use crate::ErrorAnnotation;
use std::fmt::Display;

/// An `Annotatable` error type is can convert an [ErrorAnnotation] with `Self` as the source type
/// back into `Self`, provided the annotated `info` implements [std::fmt::Display].
///
/// A prime example is the [std::io::Error] impl, which can be used to convert any annotation on a
/// [std::io::Error] back into a [std::io::Error], as the [`crate`]-level example demonstrates.
///
/// The conversion provided by implementations of `Annotatable` is typically conveniently
/// accomplished via [AnnotateResult::annotate_err_into](crate::AnnotateResult::annotate_err_into).
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
