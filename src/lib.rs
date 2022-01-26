use std::borrow::Borrow;

pub struct ErrorAnnotation<I, S> {
    pub info: I,
    pub source: S,
}

impl<I, S> From<(I, S)> for ErrorAnnotation<I, S> {
    fn from((info, source): (I, S)) -> Self {
        ErrorAnnotation { info, source }
    }
}

impl<I, S> ErrorAnnotation<I, S> {
    pub fn within<B, F, T>(info: I, f: F) -> Result<T, Self>
    where
        B: ?Sized,
        I: Borrow<B>,
        F: FnOnce(&B) -> Result<T, S>,
    {
        let iref = info.borrow();
        f(iref).map_err(|source| ErrorAnnotation { info, source })
    }
}

#[cfg(test)]
mod tests;
