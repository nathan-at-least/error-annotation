use std::fmt;

/// Combine a source error with labeled diagnostic information.
///
/// `ErrorAnnotation` combines a `source` error of type `S` with diagnostic `info` of type `I`
/// which will be labeled with `label` when displayed.
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
