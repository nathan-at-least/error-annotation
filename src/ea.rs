use std::fmt;

pub fn annotate<S, F, I>(label: &'static str, mkinfo: F) -> impl FnOnce(S) -> ErrorAnnotation<S, I>
where
    F: FnOnce() -> I,
{
    ErrorAnnotation::annotate(label, mkinfo)
}

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

impl<S, I> ErrorAnnotation<S, I> {
    pub fn annotate<F>(label: &'static str, mkinfo: F) -> impl FnOnce(S) -> Self
    where
        F: FnOnce() -> I,
    {
        move |source| ErrorAnnotation {
            source,
            label,
            info: mkinfo(),
        }
    }
}
