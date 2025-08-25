mod rule;
mod stream;
mod subslice;

use crate::Target;

pub trait Convert {
    type Output;

    fn convert(self, target: Target) -> Self::Output;
}
