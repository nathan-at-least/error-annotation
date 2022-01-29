use std::fmt;

pub fn annotate<I, S>(info: I) -> impl FnOnce(S) -> ErrorAnnotation<I, S> {
    ErrorAnnotation::annotate(info)
}

pub struct ErrorAnnotation<I, S> {
    pub info: I,
    pub source: S,
}

impl<I, S> From<(I, S)> for ErrorAnnotation<I, S> {
    fn from((info, source): (I, S)) -> Self {
        ErrorAnnotation::new(info, source)
    }
}

impl<I, S> fmt::Display for ErrorAnnotation<I, S>
where
    I: fmt::Display,
    S: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\nInfo: {}", self.source, self.info)
    }
}

impl<I, S> ErrorAnnotation<I, S> {
    pub fn new(info: I, source: S) -> Self {
        ErrorAnnotation { info, source }
    }

    pub fn annotate(info: I) -> impl FnOnce(S) -> Self {
        |source| ErrorAnnotation { info, source }
    }
}

#[cfg(test)]
mod tests;
