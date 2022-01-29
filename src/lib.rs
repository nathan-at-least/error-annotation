mod annres;
mod ea;

pub use self::annres::AnnotateResult;
pub use self::ea::{annotate, ErrorAnnotation};

#[cfg(test)]
mod tests;
