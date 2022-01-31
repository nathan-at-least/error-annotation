use std::fmt;

/// Combine a source error with labeled diagnostic information.
///
/// `ErrorAnnotation` combines a `source` error of type `S` with diagnostic `info` of type `I`
/// which will be labeled with `label` when displayed.
#[derive(Debug)]
pub struct ErrorAnnotation<S, I> {
    pub source: S,
    pub label: &'static str,
    pub info: I,
}

impl<S, I> ErrorAnnotation<S, I> {
    /// Transform the source while leaving the annotated info.
    pub fn map_source<F, T>(self, f: F) -> ErrorAnnotation<T, I>
    where
        F: FnOnce(S) -> T,
    {
        ErrorAnnotation {
            source: f(self.source),
            label: self.label,
            info: self.info,
        }
    }

    /// Transform the info with a new label, leaving the original source.
    pub fn map_info<F, J>(self, f: F) -> ErrorAnnotation<S, J>
    where
        F: FnOnce(I) -> (&'static str, J),
    {
        let (label, info) = f(self.info);
        ErrorAnnotation {
            source: self.source,
            label,
            info,
        }
    }
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
