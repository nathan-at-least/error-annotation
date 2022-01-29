use std::fmt;

pub fn annotate<S, I>(info: I) -> impl FnOnce(S) -> ErrorAnnotation<S, I> {
    ErrorAnnotation::annotate(info)
}

pub struct ErrorAnnotation<S, I> {
    pub source: S,
    pub info: I,
}

impl<S, I> fmt::Display for ErrorAnnotation<S, I>
where
    I: fmt::Display,
    S: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\nInfo: {}", self.source, self.info)
    }
}

impl<S, I> ErrorAnnotation<S, I> {
    pub fn annotate(info: I) -> impl FnOnce(S) -> Self {
        |source| ErrorAnnotation { source, info }
    }
}

#[cfg(test)]
mod tests;
