mod rule;
mod stream;
mod subslice;

/// Target compilation environment
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Target {
    /// Stable Rust - remove const syntax
    #[cfg_attr(not(feature = "nightly"), default)]
    Stable,

    /// Nightly Rust - use modern const syntax
    #[cfg_attr(feature = "nightly", default)]
    Nightly,
}

pub trait Convert {
    type Output;

    fn convert(self, target: Target) -> Self::Output;
}
