use std::fmt;

pub fn annotate<S, I>(label: &'static str, info: I) -> impl FnOnce(S) -> ErrorAnnotation<S, I> {
    ErrorAnnotation::annotate(label, info)
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
    pub fn annotate(label: &'static str, info: I) -> impl FnOnce(S) -> Self {
        move |source| ErrorAnnotation {
            source,
            label,
            info,
        }
    }
}
