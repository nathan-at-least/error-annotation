use crate::{annotate, ErrorAnnotation};

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
