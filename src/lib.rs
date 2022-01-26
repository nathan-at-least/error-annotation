use std::borrow::Borrow;

pub struct Annotate<I, S> {
    pub info: I,
    pub source: S,
}

impl<I, S> From<(I, S)> for Annotate<I, S> {
    fn from((info, source): (I, S)) -> Self {
        Annotate { info, source }
    }
}

impl<I, S> Annotate<I, S> {
    pub fn within<B, F, T>(iref: &B, f: F) -> Result<T, Self>
    where
        B: ToOwned<Owned = I> + ?Sized,
        I: Borrow<B>,
        F: FnOnce(&B) -> Result<T, S>,
    {
        let info = iref.to_owned();
        f(info.borrow()).map_err(|source| Annotate { info, source })
    }
}

#[cfg(test)]
mod tests;
