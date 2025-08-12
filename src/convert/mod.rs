mod group;
mod ident;
mod stream;
mod tt;

use crate::Target;

pub trait Convert {
    type Output;

    fn convert(self, target: Target) -> Self::Output;
}
