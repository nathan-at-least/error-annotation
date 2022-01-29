use crate::{annotate, ErrorAnnotation};

pub trait AnnotateResult<T, E> {
    fn annotate_err<I>(self, label: &'static str, info: I) -> Result<T, ErrorAnnotation<E, I>>;
}

impl<T, E> AnnotateResult<T, E> for Result<T, E> {
    fn annotate_err<I>(self, label: &'static str, info: I) -> Result<T, ErrorAnnotation<E, I>> {
        self.map_err(annotate(label, info))
    }
}
